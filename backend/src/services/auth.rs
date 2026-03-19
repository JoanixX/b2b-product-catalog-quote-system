#![allow(dead_code)]
use argon2::{
    Argon2,
    PasswordHash,
    PasswordHasher,
    PasswordVerifier,
    password_hash::{SaltString, rand_core::OsRng},
};
use jsonwebtoken::{decode, encode, DecodingKey, EncodingKey, Header, Validation};
use serde::{Deserialize, Serialize};
use chrono::{Duration, Utc};
use crate::error::{ApiError, ApiResult};

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct Claims {
    pub sub: String, // Sujeto (email del admin)
    pub exp: i64,    // Tiempo de expiración
    pub iat: i64,    // Emitido en
}

/// hashea contrasena usando argon2id (resistente a gpu y side-channel attacks)
pub fn hash_password(password: &str) -> ApiResult<String> {
    let salt = SaltString::generate(&mut OsRng);
    let argon2 = Argon2::default();
    
    argon2.hash_password(password.as_bytes(), &salt)
        .map(|hash| hash.to_string())
        .map_err(|e| ApiError::Internal(format!("Error al hashear contrasena: {}", e)))
}

/// verifica contrasena contra hash argon2id
pub fn verify_password(password: &str, hash: &str) -> ApiResult<bool> {
    let parsed_hash = PasswordHash::new(hash)
        .map_err(|e| ApiError::Internal(format!("Hash invalido: {}", e)))?;
    
    Ok(Argon2::default()
        .verify_password(password.as_bytes(), &parsed_hash)
        .is_ok())
}

/// genera jwt con expiracion de 2 horas
pub fn generate_jwt(email: &str, secret: &str) -> ApiResult<String> {
    let now = Utc::now();
    let expiration = now + Duration::hours(2);
    
    let claims = Claims {
        sub: email.to_string(),
        exp: expiration.timestamp(),
        iat: now.timestamp(),
    };
    
    encode(
        &Header::default(),
        &claims,
        &EncodingKey::from_secret(secret.as_bytes()),
    )
    .map_err(|e| ApiError::Internal(format!("Error al generar jwt: {}", e)))
}

/// verifica y decodifica jwt
pub fn verify_jwt(token: &str, secret: &str) -> ApiResult<Claims> {
    decode::<Claims>(
        token,
        &DecodingKey::from_secret(secret.as_bytes()),
        &Validation::default(),
    )
    .map(|data| data.claims)
    .map_err(|e| {
        tracing::warn!(error_type = "jwt", details = %e, "Token jwt invalido");
        ApiError::Auth("Token invalido o expirado".to_string())
    })
}