use crate::utils::validator::validate_password;
use serde::{Deserialize, Serialize};
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
    pub password: String,
}
