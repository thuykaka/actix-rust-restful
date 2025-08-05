use crate::{utils::hash::hash_password, validators::validate_password};
use chrono::Utc;
use entity::*;
use sea_orm::ActiveValue::Set;
use serde::{Deserialize, Serialize};
use uuid::Uuid;
use validator::Validate;

#[derive(Serialize, Deserialize, Clone, Debug, Validate)]
pub struct SignUpRequest {
    #[validate(length(min = 3, message = "Name must be at least 3 characters long"))]
    pub name: String,

    #[validate(custom(function = validate_password))]
    pub password: String,

    #[validate(email(message = "Email must be valid email address"))]
    pub email: String,
}

impl SignUpRequest {
    pub fn into_active_model(self) -> t_users::ActiveModel {
        t_users::ActiveModel {
            id: Set(Uuid::new_v4()),
            name: Set(self.name),
            email: Set(self.email),
            password: Set(hash_password(&self.password)),
            created_at: Set(Utc::now().into()),
            updated_at: Set(Utc::now().into()),
            ..Default::default()
        }
    }
}

#[derive(Serialize, Deserialize, Clone, Debug, Validate)]
pub struct SignInRequest {
    #[validate(email(message = "Email must be valid email address"))]
    pub email: String,
    #[validate(length(min = 8, message = "Password must be at least 8 characters long"))]
    pub password: String,
}

#[derive(Serialize, Deserialize, Clone, Debug, Validate)]
pub struct UpdateUserRequest {
    #[validate(length(min = 3, message = "Name must be at least 3 characters long"))]
    pub name: Option<String>,

    #[validate(custom(function = validate_password))]
    pub password: Option<String>,
}

#[derive(Serialize, Deserialize, Clone, Debug, Validate)]
#[serde(rename_all = "camelCase")]
pub struct RefreshTokenRequest {
    #[validate(length(min = 1, message = "Refresh token is required"))]
    pub refresh_token: String,
}
