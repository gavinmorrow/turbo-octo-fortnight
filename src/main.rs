use axum::{routing::get, Router};

const ROOT_PATH: &str = "0.0.0.0:7878";

#[tokio::main]
async fn main() {
    // build our application with a single route
    let app = Router::new().route("/", get(|| async { "Hello, World!" }));

    // run it with hyper on localhost:3000
    axum::Server::bind(&ROOT_PATH.parse().unwrap())
        .serve(app.into_make_service())
        .await
        .unwrap();
}
