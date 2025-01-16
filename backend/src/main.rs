use axum::Router;
use axum::routing::get;

#[tokio::main]

async fn main() {
    let app = Router::new()
        .route("/", get(hello));
    let listener = tokio::net::TcpListener::bind("0.0.0.0:3000").await.unwrap();
    axum::serve(listener, app).await.unwrap();
}

async fn hello() -> &'static str {
    "Hello, axum! I'm the backend"
}