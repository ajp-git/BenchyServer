use axum::{routing::post, Router, Json};
use serde::{Deserialize, Serialize};
use std::net::SocketAddr;
use chrono::Utc;

#[derive(Deserialize)]
struct Request {
    client_id: String,
    request: String,
    timestamp: String,
}

#[derive(Serialize)]
struct Response {
    success: bool,
    server_time: String,
}

async fn handle_request(Json(_payload): Json<Request>) -> Json<Response> {
    Json(Response {
        success: true,
        server_time: Utc::now().to_rfc3339(),
    })
}

#[tokio::main]
async fn main() {
    let app = Router::new()
        .route("/", post(handle_request));

    let listener = tokio::net::TcpListener::bind("0.0.0.0:8080")
    .await
    .expect("Unable to listen to port 8080");
    println!("--> listening on port 8080");
    axum::serve(listener, app).await.unwrap();

}