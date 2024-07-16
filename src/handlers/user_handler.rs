use sqlx::{Pool, Postgres};
use tonic::{Request, Response, Status};

pub use user::user_server::{User, UserServer};
use user::{UserIdRequest, UserResponse};

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

        let reply = UserResponse {
            id: user_id.id,
            user_name: "Test".to_string(),
            user_second_name: "Testovich".to_string(),
            user_address: "Lol city".to_string(),
            phone: "8919999999".to_string(),
        };

        Ok(Response::new(reply))
    }
}