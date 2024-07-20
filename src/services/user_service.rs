use sqlx::{Pool, Postgres};
use crate::repositories::{entity::user::{CreateUser, UpdateUser, User}, user_repository::{create, delete, get_user, get_user_lsit, update}};

pub async fn get_user_by_id(user_id: i32, pool: &Pool<Postgres>) -> anyhow::Result<User> {
    get_user(user_id, pool).await
}

pub async fn get_users(pool: &Pool<Postgres>) -> anyhow::Result<Vec<User>> {
    get_user_lsit(pool).await
}

pub async fn create_user(user: CreateUser, pool: &Pool<Postgres>) -> anyhow::Result<User> {
    create(user, pool).await
}

pub async fn delete_user(user_id: i32, pool: &Pool<Postgres>) -> anyhow::Result<()> {
    delete(user_id, pool).await
}

pub async fn update_user(user: UpdateUser, pool: &Pool<Postgres>) -> anyhow::Result<User> {
    update(user, pool).await
}