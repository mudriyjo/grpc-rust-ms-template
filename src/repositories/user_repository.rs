use serde::{Deserialize, Serialize};
use sqlx::{Row, prelude::FromRow, Pool, Postgres};

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

pub async fn get_user_lsit(pool: &Pool<Postgres>) -> anyhow::Result<Vec<User>> {
    let res: Vec<User> = sqlx::query_as(
        "SELECT id, user_name, user_second_name, phone, user_address FROM user_information",
    )
    .fetch_all(pool)
    .await
    .unwrap_or_else(|_| panic!("Can't fetch users"));

    Ok(res)
}

pub async fn create(user: CreateUser, pool: &Pool<Postgres>) -> anyhow::Result<User> {
    let res: User = sqlx::query_as(
            "INSERT INTO user_information (user_name, user_second_name, phone, user_address) VALUES ($1, $2, $3, $4) RETURNING id, user_name, user_second_name, phone, user_address;",
        )
        .bind(user.user_name)
        .bind(user.user_second_name)
        .bind(user.phone)
        .bind(user.user_address)
        .fetch_one(pool)
        .await
        .unwrap_or_else(|_| panic!("Can't fetch inserted user"));

    Ok(res)
}

pub async fn delete(user_id: i32, pool: &Pool<Postgres>) -> anyhow::Result<()> {
    sqlx::query("DELETE FROM user_information WHERE id = $1;")
        .bind(user_id)
        .execute(pool)
        .await
        .unwrap_or_else(|_| panic!("Can't delete user with user id: {}", user_id));

    Ok(())
}

pub async fn update(user: UpdateUser, pool: &Pool<Postgres>) -> anyhow::Result<User> {
    let mut query =
        sqlx::query_builder::QueryBuilder::new("UPDATE user_information SET ".to_string());
    let mut binding_result = vec![];

    if let Some(phone) = user.phone {
        binding_result.push(("phone = ", phone));
    }
    
    if let Some(user_address) = user.user_address {
        binding_result.push(("user_address = ", user_address));
    }

    if let Some(user_name) = user.user_name {
        binding_result.push(("user_name = ", user_name));
    }

    if let Some(user_second_name) = user.user_second_name {
        binding_result.push(("user_second_name = ", user_second_name));
    }

    let result_len = binding_result.len();
    for (i, rec) in binding_result.into_iter().enumerate() {
        query.push(rec.0);
        query.push_bind(rec.1);
        if i + 1 < result_len {
            query.push(",");
        }
    }

    query.push(" WHERE id = ");
    query.push_bind(user.id);

    query.push(" RETURNING id, user_name, user_second_name, phone, user_address;");

    let mut query_for_execution = query.build();

    query_for_execution = query_for_execution.bind(user.id);

    let res = query_for_execution
        .fetch_one(pool)
        .await
        .unwrap_or_else(|_| panic!("Can't fetch inserted user"));

    let user = User {
        id: res.get("id"),
        user_name: res.get("user_name"),
        user_second_name: res.get("user_second_name"),
        phone: res.get("phone"),
        user_address: res.get("user_address"),
    };

    Ok(user)
}
