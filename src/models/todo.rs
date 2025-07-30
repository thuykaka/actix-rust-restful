use serde::{Deserialize, Serialize};

#[derive(Serialize, Deserialize, Clone, Debug)]
pub struct Todo {
    pub id: Option<String>,
    pub title: String,
    pub description: String,
}
