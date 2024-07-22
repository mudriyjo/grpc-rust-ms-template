use sqlx::{Pool, Postgres};
use tonic::{Request, Response, Status};

pub use user::user_server::{User, UserServer};
use user::{
    Empty, UserCreateRequest, UserIdRequest, UserListResponse, UserResponse, UserUpdateRequest,
};

use crate::{
    repositories::entity::user::{CreateUser, UpdateUser, User as RepositoryUser},
    services::user_service::{create_user, delete_user, get_user_by_id, get_users, update_user},
};

pub mod user {
    #![allow(clippy::large_enum_variant)]
    #![allow(clippy::derive_partial_eq_without_eq)]
    tonic::include_proto!("user");
}

impl From<RepositoryUser> for UserResponse {
    fn from(user: RepositoryUser) -> Self {
        UserResponse {
            id: user.id,
            user_name: user.user_name,
            user_second_name: user.user_second_name,
            user_address: user.user_address,
            phone: user.phone,
        }
    }
}

#[derive(Debug)]
pub struct UserHandelr {
    connection: Pool<Postgres>,
}

impl UserHandelr {
    pub fn new(connection: Pool<Postgres>) -> UserHandelr {
        UserHandelr { connection }
    }
}

#[tonic::async_trait]
impl User for UserHandelr {
    async fn get_user(
        &self,
        request: Request<UserIdRequest>,
    ) -> Result<Response<UserResponse>, Status> {
        tracing::info!("Got a request: {:?}", request);
        let user_id: UserIdRequest = request.into_inner();

        get_user_by_id(user_id.id, &self.connection)
            .await
            .map(|user| Response::new(user.into()))
            .map_err(|e| e.into())
    }

    async fn get_user_list(
        &self,
        _request: Request<Empty>,
    ) -> Result<Response<UserListResponse>, Status> {
        tracing::info!("Got a request: {:?}", _request);

        if let Ok(user_list) = get_users(&self.connection).await {
            let reply = UserListResponse {
                users: user_list.into_iter().map(|user| user.into()).collect(),
            };

            Ok(Response::new(reply))
        } else {
            Err(Status::aborted("Can't fetch users from DB"))
        }
    }

    async fn create_user(
        &self,
        request: Request<UserCreateRequest>,
    ) -> Result<Response<UserResponse>, Status> {
        tracing::info!("Got a request: {:?}", request);
        let user_request: UserCreateRequest = request.into_inner();

        let user = CreateUser {
            user_name: user_request.user_name,
            user_second_name: user_request.user_second_name,
            phone: user_request.phone,
            user_address: user_request.user_address,
        };

        create_user(user, &self.connection)
            .await
            .map(|el| Response::new(el.into()))
            .map_err(|e| e.into())
    }

    async fn delete_user(
        &self,
        request: Request<UserIdRequest>,
    ) -> Result<Response<Empty>, Status> {
        tracing::info!("Got a request: {:?}", request);
        let user_id: UserIdRequest = request.into_inner();

        delete_user(user_id.id, &self.connection)
            .await
            .map(|_| Response::new(Empty {}))
            .map_err(|e| e.into())
    }

    async fn update_user(
        &self,
        request: Request<UserUpdateRequest>,
    ) -> Result<Response<UserResponse>, Status> {
        tracing::info!("Got a request: {:?}", request);
        let user_request: UserUpdateRequest = request.into_inner();
        let user_for_update = UpdateUser {
            id: user_request.id,
            user_name: user_request.user_name,
            user_second_name: user_request.user_second_name,
            user_address: user_request.user_address,
            phone: user_request.phone,
        };

        update_user(user_for_update, &self.connection)
            .await
            .map(|user| Response::new(user.into()))
            .map_err(|e| e.into())
    }
}
