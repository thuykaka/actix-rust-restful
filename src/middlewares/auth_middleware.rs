use actix_web::{
    Error, HttpMessage,
    body::MessageBody,
    dev::{ServiceRequest, ServiceResponse},
    error::ErrorUnauthorized,
    middleware::Next,
};

use crate::utils::jwt::{AuthenticatedUser, verify_token};

pub async fn auth_middleware(
    req: ServiceRequest,
    next: Next<impl MessageBody>,
) -> Result<ServiceResponse<impl MessageBody>, Error> {
    let jwt_token = req
        .headers()
        .get("Authorization")
        .and_then(|header| header.to_str().ok())
        .and_then(|header_string| header_string.strip_prefix("Bearer "))
        .ok_or_else(|| ErrorUnauthorized("Unauthorized"))?;

    let claims = verify_token(jwt_token).map_err(|_| ErrorUnauthorized("Unauthorized"))?;

    req.extensions_mut()
        .insert(AuthenticatedUser(claims.clone()));

    next.call(req).await
}
