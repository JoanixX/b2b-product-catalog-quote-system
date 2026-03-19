use axum::{
    http::StatusCode,
    response::{IntoResponse, Response},
    Json,
};
use serde_json::json;
use thiserror::Error;

// codigos de error estandarizados, o sea nunca se exponen 
// detalles internos al cliente
#[derive(Error, Debug)]
pub enum ApiError {
    #[error("Error de base de datos")]
    Database(#[from] sqlx::Error),
    
    #[error("Error de autenticacion")]
    Auth(String),
    
    #[error("Error de validacion")]
    Validation(String),
    
    #[error("Recurso no encontrado")]
    NotFound(String),
    
    #[error("Error interno del servidor")]
    Internal(String),
    
    #[error("Solicitud invalida")]
    BadRequest(String),
    
    #[error("No autorizado")]
    Unauthorized,
    
    #[error("Lmite de solicitudes excedido")]
    RateLimitExceeded,

    #[error("RUC invalido")]
    InvalidRuc,
}

impl IntoResponse for ApiError {
    fn into_response(self) -> Response {
        let (status, code, message) = match self {
            ApiError::Database(ref e) => {
                // loguear error real internamente, nunca exponer al cliente
                tracing::error!(
                    error_type = "database",
                    details = %e,
                    "Error de base de datos"
                );
                (
                    StatusCode::INTERNAL_SERVER_ERROR,
                    "ERR_INTERNAL_SERVER",
                    "Error interno del servidor".to_string(),
                )
            }
            ApiError::Auth(ref msg) => {
                tracing::warn!(
                    error_type = "auth",
                    details = %msg,
                    "Intento de autenticación fallido"
                );
                (
                    StatusCode::UNAUTHORIZED,
                    "ERR_UNAUTHORIZED",
                    "Credenciales inválidas".to_string(),
                )
            }
            ApiError::Validation(ref msg) => (
                StatusCode::BAD_REQUEST,
                "ERR_VALIDATION",
                format!("Error de validación: {}", msg),
            ),
            ApiError::NotFound(_) => (
                StatusCode::NOT_FOUND,
                "ERR_NOT_FOUND",
                "Recurso no encontrado".to_string(),
            ),
            ApiError::Internal(ref msg) => {
                tracing::error!(
                    error_type = "internal",
                    details = %msg,
                    "Error interno"
                );
                (
                    StatusCode::INTERNAL_SERVER_ERROR,
                    "ERR_INTERNAL_SERVER",
                    "Error interno del servidor".to_string(),
                )
            }
            ApiError::BadRequest(ref msg) => (
                StatusCode::BAD_REQUEST,
                "ERR_BAD_REQUEST",
                msg.clone(),
            ),
            ApiError::Unauthorized => (
                StatusCode::UNAUTHORIZED,
                "ERR_UNAUTHORIZED",
                "No autorizado".to_string(),
            ),
            ApiError::RateLimitExceeded => (
                StatusCode::TOO_MANY_REQUESTS,
                "ERR_RATE_LIMIT",
                "Demasiadas solicitudes, intente en unos minutos".to_string(),
            ),
            ApiError::InvalidRuc => (
                StatusCode::BAD_REQUEST,
                "ERR_INVALID_RUC",
                "El RUC proporcionado no es valido".to_string(),
            ),
        };

        let body = Json(json!({
            "code": code,
            "message": message,
        }));

        (status, body).into_response()
    }
}

pub type ApiResult<T> = Result<T, ApiError>;