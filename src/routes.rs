use axum::{response::Html, Router};

pub fn router() -> Router {
    Router::new().route("/", axum::routing::get(index))
}

pub async fn index() -> Html<String> {
    Html("Hello, World!".to_string())
}
