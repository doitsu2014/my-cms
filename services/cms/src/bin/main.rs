use axum::{
    http::StatusCode,
    routing::{get, post},
    Json, Router,
};
use cms::handlers::*;
use diesel::prelude::*;
use dotenv::dotenv;
use serde::{Deserialize, Serialize};
use std::env;

#[tokio::main]
async fn main() {
    tracing_subscriber::fmt::init();
    dotenv().ok();

    let database_url = env::var("DATABASE_URL").expect("DATABASE_URL must be set");
    PgConnection::establish(&database_url)
        .unwrap_or_else(|_| panic!("Error connecting to {}", database_url));

    let app = Router::new().route("/", get(root_handler::handle));
    let host_port = "0.0.0.0:8080";
    let listener = tokio::net::TcpListener::bind(host_port).await.unwrap();
    axum::serve(listener, app).await.unwrap();
    println!("Listening on port {}", host_port);
}
