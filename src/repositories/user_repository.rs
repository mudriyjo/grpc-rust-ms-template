use serde::{Deserialize, Serialize};
use sqlx::{prelude::FromRow, Pool, Postgres};

#[derive(Debug, Serialize, Deserialize, FromRow)]
pub struct User {
    pub id: i32,
    pub user_name: String,
    pub user_second_name: String,
    pub phone: String,
    pub user_address: String
}

pub async fn get_user(user_id: i32, pool: &Pool<Postgres>) -> anyhow::Result<User> {
    let res: User = sqlx::query_as(
            "SELECT id, user_name, user_second_name, phone, user_address FROM user_information WHERE id = $1",
        )
        .bind(user_id)
        .fetch_one(pool)
        .await
        .unwrap_or_else(|_| panic!("Can't find user with user id: {}", user_id));

    Ok(res)
}
