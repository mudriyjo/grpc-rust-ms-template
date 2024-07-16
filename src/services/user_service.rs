use sqlx::{Pool, Postgres};
use crate::repositories::user_repository::{create, get_user, get_user_lsit, CreateUser, User};

pub async fn get_user_by_id(id: i32, pool: &Pool<Postgres>) -> anyhow::Result<User> {
    let user = get_user(id, pool).await?;
    Ok(user)
}

pub async fn get_users(pool: &Pool<Postgres>) -> anyhow::Result<Vec<User>> {
    let user = get_user_lsit(pool).await?;
    Ok(user)
}

pub async fn create_user(user: CreateUser, pool: &Pool<Postgres>) -> anyhow::Result<User> {
    let user = create(user, pool).await?;
    Ok(user)
}