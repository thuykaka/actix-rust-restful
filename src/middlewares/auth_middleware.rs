use actix_web::{
    Error as ActixWebError, HttpMessage,
    body::MessageBody,
    dev::{ServiceRequest, ServiceResponse},
    http,
    middleware::Next,
};

use crate::models::errors::Error;
use crate::utils::jwt::{AuthenticatedUser, verify_token};

pub async fn auth_middleware(
    req: ServiceRequest,
    next: Next<impl MessageBody>,
) -> Result<ServiceResponse<impl MessageBody>, ActixWebError> {
    // Mặc định actix web không thể trả về custom error cho mình
    // phải implement ResponseError cho Error

    let jwt_token = req
        .headers()
        .get(http::header::AUTHORIZATION)
        .and_then(|h| h.to_str().ok())
        .and_then(|s| s.strip_prefix("Bearer "))
        .ok_or_else(|| Error::Unauthorized)?;

    let claims = verify_token(jwt_token).map_err(|_| Error::Unauthorized)?;

    req.extensions_mut()
        .insert(AuthenticatedUser(claims.clone()));

    next.call(req).await
}
