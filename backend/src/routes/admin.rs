#[cfg(feature = "legacy-uploads")]
use crate::services::s3;
#[cfg(feature = "legacy-uploads")]
use axum::extract::Multipart;
use axum::{
    extract::{Path, Query, State},
    routing::{get, patch, post, put},
    Json, Router,
};
use serde::Deserialize;
use validator::Validate;

use crate::{
    error::{ApiError, ApiResult},
    models::*,
    services::{
        auth::{generate_jwt, verify_password},
        validation::sanitize_text,
    },
    AppState,
};

pub fn routes(state: AppState) -> Router<AppState> {
    Router::new()
        .route("/products", get(get_admin_products).post(create_product))
        .route("/products/:id", put(update_product).delete(delete_product))
        .route("/products/:id/toggle", patch(toggle_product))
        .route(
            "/categories",
            get(get_admin_categories).post(create_category),
        )
        .route(
            "/categories/:id",
            put(update_category).delete(delete_category),
        )
        .route("/quotes", get(get_quotes))
        .route("/quotes/:id", get(get_quote_by_id))
        .route("/quotes/:id/status", patch(update_quote_status))
        .route("/upload", post(upload_file))
        .route_layer(axum::middleware::from_fn_with_state(
            state,
            crate::middleware::auth::auth_middleware,
        ))
        .route("/login", post(login))
}

async fn login(
    State(state): State<AppState>,
    Json(payload): Json<LoginRequest>,
) -> ApiResult<Json<LoginResponse>> {
    // Small single-operator pilot: cap login attempts across this process.
    static LOGIN_LIMIT: std::sync::OnceLock<governor::DefaultDirectRateLimiter> = std::sync::OnceLock::new();
    let limiter = LOGIN_LIMIT.get_or_init(|| governor::RateLimiter::direct(
        governor::Quota::per_minute(std::num::NonZeroU32::new(20).unwrap())
    ));
    if limiter.check().is_err() { return Err(ApiError::RateLimitExceeded); }
    payload
        .validate()
        .map_err(|e| ApiError::Validation(e.to_string()))?;

    let admin = sqlx::query_as::<_, Admin>("SELECT * FROM admins WHERE email = $1")
        .bind(&payload.email)
        .fetch_optional(&state.db)
        .await?
        .ok_or_else(|| {
            tracing::warn!(email = %payload.email, "Intento de login con email inexistente");
            ApiError::Auth("Credenciales invalidas".to_string())
        })?;

    // Verificar contraseña
    let is_valid = verify_password(&payload.password, &admin.password_hash)?;
    if !is_valid {
        tracing::warn!(email = %payload.email, "Intento de login con contrasena incorrecta");
        return Err(ApiError::Auth("Credenciales invalidas".to_string()));
    }

    // Actualizar último login
    sqlx::query("UPDATE admins SET last_login = NOW() WHERE id = $1")
        .bind(admin.id)
        .execute(&state.db)
        .await?;

    let token = generate_jwt(&admin.email, &state.config.jwt_secret)?;
    tracing::info!(email = %admin.email, "login exitoso");

    Ok(Json(LoginResponse {
        token,
        admin: AdminInfo {
            id: admin.id,
            email: admin.email,
            name: admin.name,
        },
    }))
}

#[derive(Debug, Deserialize)]
pub struct AdminProductQuery {
    pub page: Option<i32>,
    pub limit: Option<i32>,
    pub active: Option<bool>,
}

async fn get_admin_products(
    State(state): State<AppState>,
    Query(params): Query<AdminProductQuery>,
) -> ApiResult<Json<ProductListResponse>> {
    let page = params.page.unwrap_or(1).clamp(1, 1_000_000);
    let limit = params.limit.unwrap_or(50).clamp(1, 100);
    let offset = (page - 1) * limit;
    let products: Vec<Product> = sqlx::query_as("SELECT * FROM products WHERE ($1::boolean IS NULL OR is_active = $1) ORDER BY created_at DESC LIMIT $2 OFFSET $3")
        .bind(params.active).bind(limit).bind(offset).fetch_all(&state.db).await?;
    let total: (i64,) = sqlx::query_as(
        "SELECT COUNT(*) FROM products WHERE ($1::boolean IS NULL OR is_active = $1)",
    )
    .bind(params.active)
    .fetch_one(&state.db)
    .await?;
    Ok(Json(ProductListResponse {
        products,
        total: total.0,
        page,
        limit,
    }))
}

async fn create_product(
    State(state): State<AppState>,
    Json(payload): Json<CreateProductRequest>,
) -> ApiResult<Json<Product>> {
    payload
        .validate()
        .map_err(|e| ApiError::Validation(e.to_string()))?;

    let product = sqlx::query_as::<_, Product>(
        r#"
        INSERT INTO products (
            name, slug, description, category_id, brand, model_number,
            origin_country, warranty_period, technical_sheet_url,
            registro_sanitario, specifications, regulatory_info
        )
        VALUES ($1, $2, $3, $4, $5, $6, $7, $8, $9, $10, $11, $12)
        RETURNING *
        "#,
    )
    .bind(sanitize_text(&payload.name))
    .bind(sanitize_text(&payload.slug))
    .bind(payload.description.as_deref().map(sanitize_text))
    .bind(&payload.category_id)
    .bind(sanitize_text(&payload.brand))
    .bind(payload.model_number.as_deref().map(sanitize_text))
    .bind(sanitize_text(&payload.origin_country))
    .bind(payload.warranty_period.unwrap_or(12))
    .bind(&payload.technical_sheet_url)
    .bind(sanitize_text(&payload.registro_sanitario))
    .bind(
        payload
            .specifications
            .clone()
            .unwrap_or(serde_json::json!({})),
    )
    .bind(
        payload
            .regulatory_info
            .clone()
            .unwrap_or(serde_json::json!({})),
    )
    .fetch_one(&state.db)
    .await?;

    Ok(Json(product))
}

async fn update_product(
    State(state): State<AppState>,
    Path(id): Path<i32>,
    Json(payload): Json<UpdateProductRequest>,
) -> ApiResult<Json<Product>> {
    let product = sqlx::query_as::<_, Product>(
        r#"
        UPDATE products 
        SET name = COALESCE($1, name),
            slug = COALESCE($2, slug),
            description = COALESCE($3, description),
            is_active = COALESCE($4, is_active),
            brand = COALESCE($5, brand),
            origin_country = COALESCE($6, origin_country),
            warranty_period = COALESCE($7, warranty_period),
            registro_sanitario = COALESCE($8, registro_sanitario),
            updated_at = NOW()
        WHERE id = $9
        RETURNING *
        "#,
    )
    .bind(&payload.name)
    .bind(&payload.slug)
    .bind(&payload.description)
    .bind(&payload.is_active)
    .bind(&payload.brand)
    .bind(&payload.origin_country)
    .bind(&payload.warranty_period)
    .bind(&payload.registro_sanitario)
    .bind(id)
    .fetch_optional(&state.db)
    .await?
    .ok_or_else(|| ApiError::NotFound("Producto no encontrado".to_string()))?;

    Ok(Json(product))
}

async fn delete_product(
    State(state): State<AppState>,
    Path(id): Path<i32>,
) -> ApiResult<Json<serde_json::Value>> {
    let result = sqlx::query("DELETE FROM products WHERE id = $1")
        .bind(id)
        .execute(&state.db)
        .await?;

    if result.rows_affected() == 0 {
        return Err(ApiError::NotFound("Producto no encontrado".to_string()));
    }

    Ok(Json(serde_json::json!({
        "code": "OK",
        "message": "Producto eliminado exitosamente"
    })))
}

async fn toggle_product(
    State(state): State<AppState>,
    Path(id): Path<i32>,
) -> ApiResult<Json<Product>> {
    let product = sqlx::query_as::<_, Product>(
        "UPDATE products SET is_active = NOT is_active WHERE id = $1 RETURNING *",
    )
    .bind(id)
    .fetch_optional(&state.db)
    .await?
    .ok_or_else(|| ApiError::NotFound("Producto no encontrado".to_string()))?;

    Ok(Json(product))
}

async fn get_admin_categories(State(state): State<AppState>) -> ApiResult<Json<Vec<Category>>> {
    let categories = sqlx::query_as::<_, Category>("SELECT * FROM categories ORDER BY name ASC")
        .fetch_all(&state.db)
        .await?;

    Ok(Json(categories))
}

async fn create_category(
    State(state): State<AppState>,
    Json(payload): Json<CreateCategoryRequest>,
) -> ApiResult<Json<Category>> {
    let category = sqlx::query_as::<_, Category>(
        "INSERT INTO categories (name, slug, description) VALUES ($1, $2, $3) RETURNING *",
    )
    .bind(sanitize_text(&payload.name))
    .bind(sanitize_text(&payload.slug))
    .bind(payload.description.as_deref().map(sanitize_text))
    .fetch_one(&state.db)
    .await?;

    Ok(Json(category))
}

async fn update_category(
    State(state): State<AppState>,
    Path(id): Path<i32>,
    Json(payload): Json<UpdateCategoryRequest>,
) -> ApiResult<Json<Category>> {
    let category = sqlx::query_as::<_, Category>(
        r#"
        UPDATE categories 
        SET name = COALESCE($1, name),
            slug = COALESCE($2, slug),
            description = COALESCE($3, description)
        WHERE id = $4
        RETURNING *
        "#,
    )
    .bind(&payload.name)
    .bind(&payload.slug)
    .bind(&payload.description)
    .bind(id)
    .fetch_optional(&state.db)
    .await?
    .ok_or_else(|| ApiError::NotFound("Categoria no encontrada".to_string()))?;

    Ok(Json(category))
}

async fn delete_category(
    State(state): State<AppState>,
    Path(id): Path<i32>,
) -> ApiResult<Json<serde_json::Value>> {
    let result = sqlx::query("DELETE FROM categories WHERE id = $1")
        .bind(id)
        .execute(&state.db)
        .await?;

    if result.rows_affected() == 0 {
        return Err(ApiError::NotFound("Categoria no encontrada".to_string()));
    }

    Ok(Json(serde_json::json!({
        "code": "OK",
        "message": "Categoria eliminada exitosamente"
    })))
}

#[derive(Debug, Deserialize)]
pub struct QuoteQuery {
    pub status: Option<String>,
    pub page: Option<i32>,
    pub limit: Option<i32>,
}

async fn get_quotes(
    State(state): State<AppState>,
    Query(params): Query<QuoteQuery>,
) -> ApiResult<Json<QuoteListResponse>> {
    let page = params.page.unwrap_or(1).clamp(1, 1_000_000);
    let limit = params.limit.unwrap_or(50).clamp(1, 100);
    let offset = (page - 1) * limit;
    let quotes: Vec<Quote> = sqlx::query_as("SELECT * FROM quotes WHERE ($1::text IS NULL OR status = $1) ORDER BY created_at DESC LIMIT $2 OFFSET $3")
        .bind(&params.status).bind(limit).bind(offset).fetch_all(&state.db).await?;
    let total: (i64,) =
        sqlx::query_as("SELECT COUNT(*) FROM quotes WHERE ($1::text IS NULL OR status = $1)")
            .bind(&params.status)
            .fetch_one(&state.db)
            .await?;
    Ok(Json(QuoteListResponse {
        quotes,
        total: total.0,
        page,
        limit,
    }))
}

async fn get_quote_by_id(
    State(state): State<AppState>,
    Path(id): Path<i32>,
) -> ApiResult<Json<Quote>> {
    let quote = sqlx::query_as::<_, Quote>("SELECT * FROM quotes WHERE id = $1")
        .bind(id)
        .fetch_optional(&state.db)
        .await?
        .ok_or_else(|| ApiError::NotFound("Cotizacion no encontrada".to_string()))?;

    Ok(Json(quote))
}

async fn update_quote_status(
    State(state): State<AppState>,
    Path(id): Path<i32>,
    Json(payload): Json<UpdateQuoteStatusRequest>,
) -> ApiResult<Json<Quote>> {
    let quote = sqlx::query_as::<_, Quote>(
        r#"
        UPDATE quotes 
        SET status = $1, 
            notes = COALESCE($2, notes),
            contacted_at = CASE WHEN $1 = 'contacted' THEN NOW() ELSE contacted_at END
        WHERE id = $3
        RETURNING *
        "#,
    )
    .bind(sanitize_text(&payload.status))
    .bind(payload.notes.as_deref().map(sanitize_text))
    .bind(id)
    .fetch_optional(&state.db)
    .await?
    .ok_or_else(|| ApiError::NotFound("Cotizacion no encontrada".to_string()))?;

    Ok(Json(quote))
}

#[cfg(feature = "legacy-uploads")]
async fn upload_file(
    State(state): State<AppState>,
    mut multipart: Multipart,
) -> ApiResult<Json<serde_json::Value>> {
    while let Some(field) = multipart
        .next_field()
        .await
        .map_err(|e| ApiError::BadRequest(format!("Error al leer campo multipart: {}", e)))?
    {
        let name = field.name().unwrap_or("").to_string();

        if name == "file" {
            let content_type = field
                .content_type()
                .ok_or_else(|| ApiError::BadRequest("Falta el tipo de contenido".to_string()))?
                .to_string();

            let data = field.bytes().await.map_err(|e| {
                ApiError::BadRequest(format!("Error al leer datos del archivo: {}", e))
            })?;

            // validar tamano del archivo (max 10mb)
            if data.len() > 10 * 1024 * 1024 {
                return Err(ApiError::BadRequest(
                    "Archivo muy grande (max 10MB)".to_string(),
                ));
            }

            let url = s3::upload_file(
                &state.s3,
                &state.config.aws_s3_bucket,
                data.to_vec(),
                &content_type,
            )
            .await?;

            return Ok(Json(serde_json::json!({
                "code": "OK",
                "url": url
            })));
        }
    }

    Err(ApiError::BadRequest(
        "No se proporciono archivo".to_string(),
    ))
}
#[cfg(not(feature = "legacy-uploads"))]
async fn upload_file() -> ApiResult<Json<serde_json::Value>> {
    Err(ApiError::BadRequest(
        "Archivos deshabilitados en este piloto".into(),
    ))
}
