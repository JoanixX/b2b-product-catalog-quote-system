use serde::{Deserialize, Serialize};
use sqlx::FromRow;
use validator::Validate;
use chrono::{DateTime, Utc};

#[derive(Debug, Serialize, Deserialize, FromRow)]
pub struct Quote {
    pub id: i32,
    pub company_name: String,
    pub company_tax_id: String,
    pub contact_name: String,
    pub email: String,
    pub phone: Option<String>,
    pub product_ids: Vec<i32>,
    pub estimated_quantity: Option<String>,
    pub message: Option<String>,
    pub status: String,
    pub created_at: DateTime<Utc>,
    pub contacted_at: Option<DateTime<Utc>>,
    pub notes: Option<String>,
}

#[derive(Debug, Deserialize, Validate)]
pub struct CreateQuoteRequest {
    #[validate(length(min = 2, max = 255))]
    pub company_name: String,
    
    // Para este ruc peruano se valida con algoritmo modulo 11
    #[validate(length(equal = 11))]
    pub company_tax_id: String,
    
    #[validate(length(min = 2, max = 255))]
    pub contact_name: String,
    
    #[validate(email)]
    pub email: String,
    
    #[validate(length(max = 50))]
    pub phone: Option<String>,
    
    #[validate(length(min = 1))]
    pub product_ids: Vec<i32>,
    
    #[validate(length(max = 1000))]
    pub estimated_quantity: Option<String>,
    
    #[validate(length(max = 2000))]
    pub message: Option<String>,
}

#[derive(Debug, Deserialize)]
pub struct UpdateQuoteStatusRequest {
    pub status: String,
    pub notes: Option<String>,
}

#[derive(Debug, Serialize)]
pub struct QuoteListResponse {
    pub quotes: Vec<Quote>,
    pub total: i64,
    pub page: i32,
    pub limit: i32,
}