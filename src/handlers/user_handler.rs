use sqlx::{Pool, Postgres};
use tonic::{Request, Response, Status};

pub use user::user_server::{User, UserServer};
use user::{UserIdRequest, UserResponse};

use crate::services::user_service::get_user_by_id;

pub mod user {
    #![allow(clippy::large_enum_variant)]
    #![allow(clippy::derive_partial_eq_without_eq)]
    tonic::include_proto!("user");
}

#[derive(Debug)]
pub struct UserHandelr {
    connection: Pool<Postgres>
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

        let user = get_user_by_id(user_id.id, &self.connection).await.expect("Can't fetch user");
        let reply = UserResponse {
            id: user.id,
            user_name: user.user_name,
            user_second_name: user.user_second_name,
            user_address: user.user_address,
            phone: user.phone,
        };

        Ok(Response::new(reply))
    }
}