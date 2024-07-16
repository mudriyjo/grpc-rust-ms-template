use axum::{response::Html, routing::get};
use sqlx::PgPool;
use tokio::net::TcpListener;

async fn index() -> Html<String> {
    Html("Hello world!".to_string())
}

#[tokio::main]
async fn main() -> anyhow::Result<()> {
    dotenv::dotenv().unwrap();

    let server_port_address = std::env::var("SERVER")?;
    let db_url = std::env::var("DATABASE_URL")?;
    let pool = PgPool::connect(&db_url).await?;

    sqlx::migrate!("./migrations").run(&pool).await?;

    color_eyre::install().expect("Error with starting color eyre hook...");

    tracing_subscriber::fmt::init();

    let connection = TcpListener::bind(server_port_address).await?;

    let router = axum::Router::new()
        .route("/", get(index));

    let server = axum::serve(connection, router);
    tracing::info!("Start server...");
    server.await?;

    Ok(())
}