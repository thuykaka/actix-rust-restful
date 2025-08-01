use serde::{Deserialize, Serialize};

use crate::models::db::User;

#[derive(Serialize, Deserialize, Clone, Debug)]
pub struct SignUpResponse {
    pub token: String,
    pub user: User,
}

#[derive(Serialize, Deserialize, Clone, Debug)]
pub struct SignInResponse {
    pub token: String,
    pub user: User,
}

#[derive(Serialize, Deserialize, Clone, Debug)]
pub struct MeResponse(pub User);
