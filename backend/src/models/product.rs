use serde::{Deserialize, Serialize};
use sqlx::FromRow;
use chrono::{DateTime, Utc};
use validator::Validate;

#[derive(Debug, Serialize, Deserialize, FromRow)]
pub struct Product {
    pub id: i32,
    pub name: String,
    pub slug: String,
    pub description: Option<String>,
    pub category_id: Option<i32>,
    pub brand: String,
    pub model_number: Option<String>,
    pub origin_country: String,
    pub warranty_period: i32,
    pub technical_sheet_url: Option<String>,
    pub registro_sanitario: String,
    pub specifications: serde_json::Value,
    pub image_url: Option<String>,
    pub additional_images: serde_json::Value,
    pub regulatory_info: serde_json::Value,
    pub is_active: bool,
    pub created_at: DateTime<Utc>,
    pub updated_at: DateTime<Utc>,
}

#[derive(Debug, Deserialize, Validate)]
pub struct CreateProductRequest {
    #[validate(length(min = 2, max = 255))]
    pub name: String,
    #[validate(length(min = 2, max = 255))]
    pub slug: String,
    #[validate(length(max = 5000))]
    pub description: Option<String>,
    pub category_id: Option<i32>,
    #[validate(length(min = 1, max = 200))]
    pub brand: String,
    #[validate(length(max = 200))]
    pub model_number: Option<String>,
    #[validate(length(min = 2, max = 100))]
    pub origin_country: String,
    #[validate(range(min = 0, max = 120))]
    pub warranty_period: Option<i32>,
    #[validate(url)]
    pub technical_sheet_url: Option<String>,
    #[validate(length(min = 1, max = 100))]
    pub registro_sanitario: String,
    pub specifications: Option<serde_json::Value>,
    pub regulatory_info: Option<serde_json::Value>,
}

#[derive(Debug, Deserialize)]
#[allow(dead_code)]
pub struct UpdateProductRequest {
    pub name: Option<String>,
    pub slug: Option<String>,
    pub description: Option<String>,
    pub category_id: Option<i32>,
    pub brand: Option<String>,
    pub model_number: Option<String>,
    pub origin_country: Option<String>,
    pub warranty_period: Option<i32>,
    pub technical_sheet_url: Option<String>,
    pub registro_sanitario: Option<String>,
    pub specifications: Option<serde_json::Value>,
    pub regulatory_info: Option<serde_json::Value>,
    pub is_active: Option<bool>,
}

#[derive(Debug, Serialize)]
pub struct ProductListResponse {
    pub products: Vec<Product>,
    pub total: i64,
    pub page: i32,
    pub limit: i32,
}