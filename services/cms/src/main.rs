use crate::handlers::root_handler::handle_root;
use axum::{
    http::StatusCode,
    routing::{get, post},
    Json, Router,
};
use serde::{Deserialize, Serialize};

#[tokio::main]
async fn main() {
    tracing_subscriber::fmt::init();

    let app = Router::new().route("/", get(handle_root));

    let host_port = "0.0.0.0:8080";
    let listener = tokio::net::TcpListener::bind(host_port).await.unwrap();

    axum::serve(listener, app).await.unwrap();

    println!("Listening on port {}", host_port);
}
