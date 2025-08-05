use entity::t_todos;
use serde::{Deserialize, Serialize};

use crate::models::db::User;

#[derive(Serialize, Deserialize, Clone, Debug)]
pub struct CommonResponse<T> {
    pub message: T,
}

#[derive(Serialize, Deserialize, Clone, Debug)]
pub struct SignUpResponse {
    #[serde(rename = "accessToken")]
    pub access_token: String,
    #[serde(rename = "refreshToken")]
    pub refresh_token: String,
    pub user: User,
}

#[derive(Serialize, Deserialize, Clone, Debug)]
pub struct SignInResponse {
    #[serde(rename = "accessToken")]
    pub access_token: String,
    #[serde(rename = "refreshToken")]
    pub refresh_token: String,
    pub user: User,
}

#[derive(Serialize, Deserialize, Clone, Debug)]
pub struct MeResponse(pub User);

#[derive(Serialize, Deserialize, Clone, Debug)]
pub struct UpdateUserResponse(pub User);

#[derive(Serialize, Deserialize, Clone, Debug)]
pub struct RefreshTokenResponse {
    #[serde(rename = "accessToken")]
    pub access_token: String,
    #[serde(rename = "refreshToken")]
    pub refresh_token: String,
}

#[derive(Serialize, Deserialize, Clone, Debug)]
pub struct GetAllTodosResponse {
    pub total: usize,
    #[serde(rename = "totalPages")]
    pub total_pages: usize,
    pub data: Vec<t_todos::Model>,
}
