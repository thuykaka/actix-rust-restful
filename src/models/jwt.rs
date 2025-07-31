use chrono::{Duration, Utc};
use jsonwebtoken::{DecodingKey, EncodingKey, Header, Validation, decode, encode};
use serde::{Deserialize, Serialize};

use crate::models::auth::UserInfo;

#[derive(Debug, Serialize, Deserialize, Clone)]
pub struct Claims {
    pub sub: String,   // Subject (user ID)
    pub email: String, // User email
    pub name: String,  // User name
    pub exp: i64,      // Expiration time
    pub iat: i64,      // Issued at
}

impl Claims {
    pub fn new(user: UserInfo) -> Self {
        let now = Utc::now();
        let exp = now + Duration::hours(24); // Token expires in 24 hours

        Claims {
            sub: user.id.to_string(),
            email: user.email,
            name: user.name,
            exp: exp.timestamp(),
            iat: now.timestamp(),
        }
    }

    pub fn generate_token(&self, secret: &str) -> Result<String, jsonwebtoken::errors::Error> {
        let header = Header::default();
        let encoding_key = EncodingKey::from_secret(secret.as_ref());
        encode(&header, self, &encoding_key)
    }
}

pub fn verify_token(token: &str, secret: &str) -> Result<Claims, jsonwebtoken::errors::Error> {
    let decoding_key = DecodingKey::from_secret(secret.as_ref());
    let validation = Validation::default();
    let token_data = decode::<Claims>(token, &decoding_key, &validation)?;
    Ok(token_data.claims)
}
