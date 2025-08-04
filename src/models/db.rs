use chrono::{self, DateTime, Utc, serde::ts_milliseconds};
use entity::t_users;
use serde::{Deserialize, Serialize};
use uuid::Uuid;

// Helper function để tạo current timestamp
#[allow(dead_code)]
fn get_current_time() -> DateTime<Utc> {
    Utc::now()
}

#[derive(Serialize, Deserialize, Clone, Debug)]
#[serde(from = "t_users::Model")]
pub struct User {
    pub id: Uuid,
    pub name: String,
    pub email: String,
    #[serde(
        rename = "createdAt",
        default = "get_current_time",
        with = "ts_milliseconds"
    )]
    pub created_at: DateTime<Utc>,
    #[serde(
        rename = "updatedAt",
        default = "get_current_time",
        with = "ts_milliseconds"
    )]
    pub updated_at: DateTime<Utc>,
}

// Cho phép gọi .into() trên t_users::Model để convert sang User
impl From<t_users::Model> for User {
    fn from(user: t_users::Model) -> Self {
        Self {
            id: user.id,
            name: user.name,
            email: user.email,
            created_at: user.created_at.with_timezone(&Utc),
            updated_at: user.updated_at.with_timezone(&Utc),
        }
    }
}
