use sqlx::{Pool, Postgres};
use crate::repositories::user_repository::{get_user, User};

pub async fn get_user_by_id(id: i32, pool: &Pool<Postgres>) -> anyhow::Result<User> {
    let user = get_user(id, pool).await?;
    Ok(user)
}