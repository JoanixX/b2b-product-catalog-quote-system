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
    services::validation::{validate_ruc, sanitize_text},
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
    let page = params.page.unwrap_or(1).max(1);
    let limit = params.limit.unwrap_or(20).min(100);
    let offset = (page - 1) * limit;
    
    let mut query = String::from(
        "SELECT * FROM products WHERE is_active = true"
    );
    
    // filtro por categoria
    if let Some(category_slug) = &params.category {
        let clean = sanitize_text(category_slug);
        query.push_str(&format!(
            " AND category_id = (SELECT id FROM categories WHERE slug = '{}')",
            clean
        ));
    }
    
    // filtro por busqueda
    if let Some(search) = &params.search {
        let clean = sanitize_text(search);
        query.push_str(&format!(
            " AND (name ILIKE '%{}%' OR description ILIKE '%{}%')",
            clean, clean
        ));
    }
    
    query.push_str(" ORDER BY created_at DESC");
    query.push_str(&format!(" LIMIT {} OFFSET {}", limit, offset));
    
    let products: Vec<Product> = sqlx::query_as(&query)
        .fetch_all(&state.db)
        .await?;
    
    // obtener total de resultados
    let count_query = "SELECT COUNT(*) as count FROM products WHERE is_active = true";
    let total: (i64,) = sqlx::query_as(count_query)
        .fetch_one(&state.db)
        .await?;
    
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
    let product = sqlx::query_as::<_, Product>(
        "SELECT * FROM products WHERE slug = $1 AND is_active = true"
    )
    .bind(&slug)
    .fetch_optional(&state.db)
    .await?
    .ok_or_else(|| ApiError::NotFound("Producto no encontrado".to_string()))?;
    
    Ok(Json(product))
}

async fn get_categories(
    State(state): State<AppState>,
) -> ApiResult<Json<Vec<Category>>> {
    let categories = sqlx::query_as::<_, Category>(
        "SELECT * FROM categories ORDER BY name ASC"
    )
    .fetch_all(&state.db)
    .await?;
    
    Ok(Json(categories))
}

async fn create_quote(
    State(state): State<AppState>,
    Json(payload): Json<CreateQuoteRequest>,
) -> ApiResult<Json<serde_json::Value>> {
    // validar estructura basica
    payload.validate()
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
    let _quote = sqlx::query_as::<_, Quote>(
        r#"
        INSERT INTO quotes (
            company_name, company_tax_id, contact_name, email, phone,
            product_ids, estimated_quantity, message, status
        )
        VALUES ($1, $2, $3, $4, $5, $6, $7, $8, 'pending')
        RETURNING *
        "#
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
    
    // obtener nombres de productos para email
    let product_names: Vec<String> = sqlx::query_as::<_, (String,)>(
        "SELECT name FROM products WHERE id = ANY($1)"
    )
    .bind(&payload.product_ids)
    .fetch_all(&state.db)
    .await?
    .into_iter()
    .map(|(name,)| name)
    .collect();
    
    let products_text = product_names.join(", ");
    
    // enviar notificacion por email
    state.email.send_quote_notification(
        &company_name,
        &contact_name,
        &payload.email,
        payload.phone.as_deref(),
        &payload.company_tax_id,
        &products_text,
        message.as_deref(),
    ).await?;
    
    Ok(Json(serde_json::json!({
        "code": "OK",
        "message": "Solicitud de cotizacion enviada exitosamente"
    })))
}