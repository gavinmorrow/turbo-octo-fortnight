mod routes;

const ROOT_PATH: &str = "0.0.0.0:7878";

#[tokio::main]
async fn main() {
    let app = routes::router();

    // run it with hyper on localhost:3000
    axum::Server::bind(&ROOT_PATH.parse().unwrap())
        .serve(app.into_make_service())
        .await
        .unwrap();
}
