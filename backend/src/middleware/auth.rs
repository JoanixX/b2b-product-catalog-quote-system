use crate::{error::ApiError, services::auth::verify_jwt, AppState};
use axum::{
    extract::{Request, State},
    middleware::Next,
    response::Response,
};

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

    let exists: (bool,) = sqlx::query_as("SELECT EXISTS (SELECT 1 FROM admins WHERE email=$1)")
        .bind(&claims.sub)
        .fetch_one(&state.db)
        .await?;
    if !exists.0 {
        return Err(ApiError::Unauthorized);
    }
    // se agregaron claims a las extensiones del request para uso en handlers
    request.extensions_mut().insert(claims);
    Ok(next.run(request).await)
}
