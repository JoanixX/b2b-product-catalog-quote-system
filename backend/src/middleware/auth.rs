use axum::{
    extract::{Request, State},
    middleware::Next,
    response::Response,
};
use crate::{error::ApiError, services::auth::verify_jwt, AppState};

#[allow(dead_code)]
pub async fn auth_middleware(
    State(state): State<AppState>,
    mut request: Request,
    next: Next,
) -> Result<Response, ApiError> {
    let auth_header = request
        .headers()
        .get("Authorization")
        .and_then(|h| h.to_str().ok())
        .ok_or(ApiError::Unauthorized)?;
    
    let token = auth_header
        .strip_prefix("Bearer ")
        .ok_or(ApiError::Unauthorized)?;
    
    let claims = verify_jwt(token, &state.config.jwt_secret)?;
    
    // se agregaron claims a las extensiones del request para uso en handlers
    request.extensions_mut().insert(claims);
    Ok(next.run(request).await)
}