use serde::{Deserialize, Serialize};

#[derive(Serialize, Deserialize, Clone, Debug)]
pub struct SignUpRequest {
    pub name: String,
    pub password: String,
    pub email: String,
}

#[derive(Serialize, Deserialize, Clone, Debug)]
pub struct SignInRequest {
    pub email: String,
    pub password: String,
}
