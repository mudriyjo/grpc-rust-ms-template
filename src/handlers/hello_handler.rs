use axum::response::Html;

use crate::services::hello_service::get_hello;

pub async fn index_handler() -> Html<String> {
    Html(get_hello().await)
}