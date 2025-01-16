use axum::http::StatusCode;
use axum::{Json, Router};
use axum::routing::get;
use serde::Serialize;

#[tokio::main]

async fn main() {
    let app = Router::new()
        .route("/", get(hello_json));
    let listener = tokio::net::TcpListener::bind("0.0.0.0:3000").await.unwrap();
    axum::serve(listener, app).await.unwrap();
}

#[derive(Serialize)]
struct Response {
    message: &'static str,
}

async fn hello_json() -> (StatusCode, Json<Response>) {
    let response = Response {
        message: "Hello, axum! I'm the backend",
    };
    
    (StatusCode::OK, Json(response))
}
