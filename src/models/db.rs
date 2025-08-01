use entity::t_users;
use serde::{Deserialize, Serialize};

#[derive(Serialize, Deserialize, Clone, Debug)]
#[serde(from = "t_users::Model")]
pub struct User {
    pub id: i32,
    pub name: String,
    pub email: String,
}

impl From<t_users::Model> for User {
    fn from(user: t_users::Model) -> Self {
        Self {
            id: user.id,
            name: user.name,
            email: user.email,
        }
    }
}
