use serde::{Deserialize, Serialize};

use crate::models::db::User;

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
