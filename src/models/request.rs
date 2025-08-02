use serde::{Deserialize, Serialize};
use validator::Validate;

#[derive(Serialize, Deserialize, Clone, Debug)]
pub struct SignUpRequest {
    pub name: String,
    pub password: String,
    pub email: String,
}

#[derive(Serialize, Deserialize, Clone, Debug, Validate)]
pub struct SignInRequest {
    #[validate(email)]
    pub email: String,
    #[validate(length(min = 8))]
    pub password: String,
}

#[derive(Serialize, Deserialize, Clone, Debug)]
pub struct UpdateUserRequest {
    pub name: Option<String>,
    pub password: String,
}
