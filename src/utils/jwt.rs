use crate::{config, models::errors::Error};
use actix_web::{FromRequest, HttpMessage, HttpRequest, dev};
use chrono::{Duration, Utc};
use jsonwebtoken::{DecodingKey, EncodingKey, Header, Validation, decode, encode};
use serde::{Deserialize, Serialize};
use std::future::{Ready, ready};

#[derive(Debug, Serialize, Deserialize, Clone)]
pub struct JwtClaims {
    pub sub: String,   // Subject (user ID)
    pub email: String, // User email
    pub name: String,  // User name
    pub exp: i64,      // Expiration time
    pub iat: i64,      // Issued at
}

#[derive(Debug, Clone)]
pub struct AuthenticatedUser(pub JwtClaims);

impl std::ops::Deref for AuthenticatedUser {
    type Target = JwtClaims;

    fn deref(&self) -> &Self::Target {
        &self.0
    }
}

// extractor lấy AuthenticatedUser từ request extensions
impl FromRequest for AuthenticatedUser {
    type Error = actix_web::Error;
    type Future = Ready<Result<Self, actix_web::Error>>;

    fn from_request(req: &HttpRequest, _: &mut dev::Payload) -> Self::Future {
        match req.extensions().get::<AuthenticatedUser>() {
            Some(data) => ready(Ok(data.clone())),
            None => ready(Err(actix_web::error::ErrorUnauthorized("Unauthorized"))),
        }
    }
}

impl JwtClaims {
    pub fn new(sub: String, email: String, name: String) -> Self {
        let now = Utc::now();
        let exp = now + Duration::hours(24);

        JwtClaims {
            sub,
            email,
            name,
            exp: exp.timestamp(),
            iat: now.timestamp(),
        }
    }
    pub fn generate_token(&self) -> Result<String, Error> {
        let header = Header::default();
        let encoding_key = EncodingKey::from_secret(config::JWT_SECRET.to_string().as_ref());
        encode(&header, self, &encoding_key).map_err(|_| Error::FailedToGenerateToken)
    }
}

pub fn verify_token(token: &str) -> Result<JwtClaims, Error> {
    let decoding_key = DecodingKey::from_secret(config::JWT_SECRET.to_string().as_ref());
    let validation = Validation::default();
    let token_data = decode::<JwtClaims>(token, &decoding_key, &validation)
        .map_err(|_| Error::FailedToVerifyToken)?;
    Ok(token_data.claims)
}
