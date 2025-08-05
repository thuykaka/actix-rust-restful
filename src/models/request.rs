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

#[derive(Serialize, Deserialize, Clone, Debug, Validate)]
pub struct GetAllTodosRequest {
    #[validate(range(min = 0, message = "Page must be greater than or equal to 0"))]
    pub page: Option<u64>,

    #[validate(range(min = 1, message = "Page size must be greater than 0"))]
    #[serde(rename = "pageSize")]
    pub page_size: Option<u64>,

    #[validate(length(min = 3, message = "Search must be at least 3 characters long"))]
    pub search: Option<String>,
}

#[derive(Serialize, Deserialize, Clone, Debug, Validate)]
pub struct CreateTodoRequest {
    #[validate(length(min = 3, message = "Title must be at least 3 characters long"))]
    pub title: String,

    #[validate(length(min = 3, message = "Description must be at least 3 characters long"))]
    pub description: String,

    pub completed: Option<bool>,
}

impl CreateTodoRequest {
    pub fn into_active_model(self, user_id: Uuid) -> t_todos::ActiveModel {
        t_todos::ActiveModel {
            id: Set(Uuid::new_v4()),
            title: Set(self.title),
            description: Set(self.description),
            completed: Set(self.completed.unwrap_or(false)),
            created_at: Set(Utc::now().into()),
            updated_at: Set(Utc::now().into()),
            user_id: Set(user_id),
            ..Default::default()
        }
    }
}

#[derive(Serialize, Deserialize, Clone, Debug, Validate)]
pub struct UpdateTodoRequest {
    #[validate(length(min = 3, message = "Title must be at least 3 characters long"))]
    pub title: Option<String>,

    #[validate(length(min = 3, message = "Description must be at least 3 characters long"))]
    pub description: Option<String>,

    pub completed: Option<bool>,
}
