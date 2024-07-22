use crate::repositories::{
    entity::user::{CreateUser, UpdateUser, User},
    user_repository::{create, delete, get_user, get_user_lsit, update},
};
use sqlx::{Pool, Postgres};
use tonic::Status;

type ServiceResult<T> = Result<T, UserServiceError>;
pub enum UserServiceError {
    DBError(String),
}

impl From<UserServiceError> for Status {
    fn from(value: UserServiceError) -> Self {
        match value {
            UserServiceError::DBError(msg) => Status::internal(msg)
        }
    }
}

pub async fn get_user_by_id(user_id: i32, pool: &Pool<Postgres>) -> ServiceResult<User> {
    get_user(user_id, pool)
        .await
        .map_err(|e| UserServiceError::DBError(e.to_string()))
}

pub async fn get_users(pool: &Pool<Postgres>) -> ServiceResult<Vec<User>> {
    get_user_lsit(pool)
        .await
        .map_err(|e| UserServiceError::DBError(e.to_string()))
}

pub async fn create_user(user: CreateUser, pool: &Pool<Postgres>) -> ServiceResult<User> {
    create(user, pool)
        .await
        .map_err(|e| UserServiceError::DBError(e.to_string()))
}

pub async fn delete_user(user_id: i32, pool: &Pool<Postgres>) -> ServiceResult<()> {
    delete(user_id, pool)
        .await
        .map_err(|e| UserServiceError::DBError(e.to_string()))
}

pub async fn update_user(user: UpdateUser, pool: &Pool<Postgres>) -> ServiceResult<User> {
    update(user, pool)
        .await
        .map_err(|e| UserServiceError::DBError(e.to_string()))
}
