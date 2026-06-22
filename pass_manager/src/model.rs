use serde::{Deserialize, Serialize};

#[derive(Debug, Deserialize, Serialize, PartialEq, sqlx::FromRow)]
pub struct Credential {
    pub id: i64,
    pub website_name: String,
    pub user_name: String,
    pub password: String,
}

pub struct NewCredential {
    pub website_name: String,
    pub user_name: String,
    pub password: String,
}