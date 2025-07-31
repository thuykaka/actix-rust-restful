use std::future::{Ready, ready};

use actix_web::{
    Error, HttpMessage, HttpResponse,
    body::EitherBody,
    dev::{Service, ServiceRequest, ServiceResponse, Transform, forward_ready},
};
use futures::future::LocalBoxFuture;

use crate::{
    config::constants::JWT_SECRET,
    models::{
        auth::UserInfo,
        jwt::{Claims, verify_token},
    },
};

// Middleware factory
pub struct AuthMiddleware;

impl<S, B> Transform<S, ServiceRequest> for AuthMiddleware
where
    S: Service<ServiceRequest, Response = ServiceResponse<B>, Error = Error>,
    S::Future: 'static,
    B: 'static,
{
    type Response = ServiceResponse<EitherBody<B>>;
    type Error = Error;
    type InitError = ();
    type Transform = AuthMiddlewareService<S>;
    type Future = Ready<Result<Self::Transform, Self::InitError>>;

    fn new_transform(&self, service: S) -> Self::Future {
        ready(Ok(AuthMiddlewareService { service }))
    }
}

// Middleware service
pub struct AuthMiddlewareService<S> {
    service: S,
}

impl<S, B> Service<ServiceRequest> for AuthMiddlewareService<S>
where
    S: Service<ServiceRequest, Response = ServiceResponse<B>, Error = Error>,
    S::Future: 'static,
    B: 'static,
{
    type Response = ServiceResponse<EitherBody<B>>;
    type Error = Error;
    type Future = LocalBoxFuture<'static, Result<Self::Response, Self::Error>>;

    forward_ready!(service);

    fn call(&self, req: ServiceRequest) -> Self::Future {
        // Extract Authorization header
        let auth_header = req.headers().get("Authorization");

        if let Some(auth_header) = auth_header {
            if let Ok(auth_str) = auth_header.to_str() {
                if auth_str.starts_with("Bearer ") {
                    let token = &auth_str[7..];

                    // Verify token
                    match verify_token(token, &JWT_SECRET) {
                        Ok(claims) => {
                            // Create UserInfo from claims
                            let user_info = UserInfo {
                                id: claims.sub.parse().unwrap_or(0),
                                name: claims.name.clone(),
                                email: claims.email.clone(),
                            };

                            // Insert both Claims and UserInfo into request extensions
                            req.extensions_mut().insert(claims);
                            req.extensions_mut().insert(user_info);

                            let fut = self.service.call(req);
                            Box::pin(async move {
                                let res = fut.await?;
                                Ok(res.map_into_left_body())
                            })
                        }
                        Err(_) => {
                            let (req, _) = req.into_parts();
                            let response = HttpResponse::Unauthorized()
                                .json(serde_json::json!({
                                    "message": "Invalid or expired token"
                                }))
                                .map_into_right_body();

                            Box::pin(ready(Ok(ServiceResponse::new(req, response))))
                        }
                    }
                } else {
                    let (req, _) = req.into_parts();
                    let response = HttpResponse::Unauthorized()
                        .json(serde_json::json!({
                            "message": "Authorization header must be in 'Bearer <token>' format"
                        }))
                        .map_into_right_body();

                    Box::pin(ready(Ok(ServiceResponse::new(req, response))))
                }
            } else {
                let (req, _) = req.into_parts();
                let response = HttpResponse::Unauthorized()
                    .json(serde_json::json!({
                        "message": "Invalid authorization header format"
                    }))
                    .map_into_right_body();

                Box::pin(ready(Ok(ServiceResponse::new(req, response))))
            }
        } else {
            let (req, _) = req.into_parts();
            let response = HttpResponse::Unauthorized()
                .json(serde_json::json!({
                    "message": "Authorization header missing"
                }))
                .map_into_right_body();

            Box::pin(ready(Ok(ServiceResponse::new(req, response))))
        }
    }
}

// Extractor để lấy user info từ request extensions
pub struct AuthenticatedUser(pub UserInfo);

impl std::ops::Deref for AuthenticatedUser {
    type Target = UserInfo;

    fn deref(&self) -> &Self::Target {
        &self.0
    }
}

impl actix_web::FromRequest for AuthenticatedUser {
    type Error = Error;
    type Future = Ready<Result<Self, Self::Error>>;

    fn from_request(req: &actix_web::HttpRequest, _: &mut actix_web::dev::Payload) -> Self::Future {
        match req.extensions().get::<UserInfo>() {
            Some(user_info) => ready(Ok(AuthenticatedUser(user_info.clone()))),
            None => ready(Err(actix_web::error::ErrorUnauthorized(
                "User not authenticated",
            ))),
        }
    }
}

// Extractor để lấy JWT claims từ request extensions
pub struct AuthenticatedClaims(pub Claims);

impl actix_web::FromRequest for AuthenticatedClaims {
    type Error = Error;
    type Future = Ready<Result<Self, Self::Error>>;

    fn from_request(req: &actix_web::HttpRequest, _: &mut actix_web::dev::Payload) -> Self::Future {
        match req.extensions().get::<Claims>() {
            Some(claims) => ready(Ok(AuthenticatedClaims(claims.clone()))),
            None => ready(Err(actix_web::error::ErrorUnauthorized(
                "User not authenticated",
            ))),
        }
    }
}
