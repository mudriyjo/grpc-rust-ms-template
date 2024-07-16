use sqlx::PgPool;
use tonic::transport::Server;
use crate::handlers::user_handler::UserHandelr;
use handlers::user_handler::UserServer;

mod handlers;
mod services;

#[tokio::main]
async fn main() -> anyhow::Result<()> {
    dotenv::dotenv().unwrap();

    let server_port_address = std::env::var("SERVER")?;
    let db_url = std::env::var("DATABASE_URL")?;
    let pool = PgPool::connect(&db_url).await?;

    sqlx::migrate!("./migration").run(&pool).await?;

    color_eyre::install().expect("Error with starting color eyre hook...");

    tracing_subscriber::fmt::init();

    let user = UserHandelr::new(pool.clone());

    tracing::info!("Start server...");

    Server::builder()
        .add_service(UserServer::new(user))
        .serve(server_port_address.parse().expect("Can't parse server connection string"))
        .await?;

    Ok(())
}