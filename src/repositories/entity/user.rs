use serde::{Deserialize, Serialize};
use sqlx::FromRow;

#[derive(Debug, Serialize, Deserialize, FromRow)]
pub struct User {
    pub id: i32,
    pub user_name: String,
    pub user_second_name: String,
    pub phone: String,
    pub user_address: String,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct CreateUser {
    pub user_name: String,
    pub user_second_name: String,
    pub phone: String,
    pub user_address: String,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct UpdateUser {
    pub id: i32,
    pub user_name: Option<String>,
    pub user_second_name: Option<String>,
    pub phone: Option<String>,
    pub user_address: Option<String>,
}