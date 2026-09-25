use axum::{
    extract::{Path, Query, State},
    routing::{get, post},
    Json, Router,
};
use serde::Deserialize;
use validator::Validate;

use crate::{
    error::{ApiError, ApiResult},
    models::*,
    services::validation::{sanitize_text, validate_ruc},
    AppState,
};

pub fn routes() -> Router<AppState> {
    Router::new()
        .route("/products", get(get_products))
        .route("/products/:slug", get(get_product_by_slug))
        .route("/categories", get(get_categories))
        .route("/quotes", post(create_quote))
}

#[derive(Debug, Deserialize)]
pub struct ProductQuery {
    pub category: Option<String>,
    pub search: Option<String>,
    pub page: Option<i32>,
    pub limit: Option<i32>,
}

async fn get_products(
    State(state): State<AppState>,
    Query(params): Query<ProductQuery>,
) -> ApiResult<Json<ProductListResponse>> {
    let page = params.page.unwrap_or(1).clamp(1, 1_000_000);
    let limit = params.limit.unwrap_or(20).clamp(1, 100);
    let offset = (page - 1) * limit;
    let search = params.search.as_ref().map(|s| format!("%{}%", s));
    let products: Vec<Product> = sqlx::query_as(
        "SELECT * FROM products WHERE is_active = true AND ($1::text IS NULL OR category_id = (SELECT id FROM categories WHERE slug = $1)) AND ($2::text IS NULL OR name ILIKE $2 OR description ILIKE $2) ORDER BY created_at DESC LIMIT $3 OFFSET $4"
    ).bind(&params.category).bind(&search).bind(limit).bind(offset).fetch_all(&state.db).await?;
    let total: (i64,) = sqlx::query_as(
        "SELECT COUNT(*) FROM products WHERE is_active = true AND ($1::text IS NULL OR category_id = (SELECT id FROM categories WHERE slug = $1)) AND ($2::text IS NULL OR name ILIKE $2 OR description ILIKE $2)"
    ).bind(&params.category).bind(&search).fetch_one(&state.db).await?;

    Ok(Json(ProductListResponse {
        products,
        total: total.0,
        page,
        limit,
    }))
}

async fn get_product_by_slug(
    State(state): State<AppState>,
    Path(slug): Path<String>,
) -> ApiResult<Json<Product>> {
    let product =
        sqlx::query_as::<_, Product>("SELECT * FROM products WHERE slug = $1 AND is_active = true")
            .bind(&slug)
            .fetch_optional(&state.db)
            .await?
            .ok_or_else(|| ApiError::NotFound("Producto no encontrado".to_string()))?;

    Ok(Json(product))
}

async fn get_categories(State(state): State<AppState>) -> ApiResult<Json<Vec<Category>>> {
    let categories = sqlx::query_as::<_, Category>("SELECT * FROM categories ORDER BY name ASC")
        .fetch_all(&state.db)
        .await?;

    Ok(Json(categories))
}

async fn create_quote(
    State(state): State<AppState>,
    Json(payload): Json<CreateQuoteRequest>,
) -> ApiResult<Json<serde_json::Value>> {
    // validar estructura basica
    payload
        .validate()
        .map_err(|e| ApiError::Validation(e.to_string()))?;

    // validar ruc peruano con algoritmo modulo 11
    if !validate_ruc(&payload.company_tax_id) {
        return Err(ApiError::InvalidRuc);
    }

    // Sanitizar campos de texto
    let company_name = sanitize_text(&payload.company_name);
    let contact_name = sanitize_text(&payload.contact_name);
    let message = payload.message.as_deref().map(sanitize_text);
    let estimated_quantity = payload.estimated_quantity.as_deref().map(sanitize_text);

    // Insertar cotizacion en base de datos
    let quote = sqlx::query_as::<_, Quote>(
        r#"
        INSERT INTO quotes (
            company_name, company_tax_id, contact_name, email, phone,
            product_ids, estimated_quantity, message, status
        )
        VALUES ($1, $2, $3, $4, $5, $6, $7, $8, 'pending')
        RETURNING *
        "#,
    )
    .bind(&company_name)
    .bind(&payload.company_tax_id)
    .bind(&contact_name)
    .bind(&payload.email)
    .bind(&payload.phone)
    .bind(&payload.product_ids)
    .bind(&estimated_quantity)
    .bind(&message)
    .fetch_one(&state.db)
    .await?;

    // The database is authoritative. Notifications are explicitly disabled in the pilot.
    // Never return an insertion failure after a successful save because email failed.
    Ok(Json(serde_json::json!({
        "code": "OK", "id": quote.id, "saved": true,
        "notification_status": "disabled",
        "message": "Solicitud guardada. No se ha enviado correo."
    })))
}
