use axum::{
    routing::{get, post},
    Router,
};
use cms::{post_handler::RouterPostHandlerExt, root_handler::RouterRootHandlerExt, AppState};
use diesel::prelude::*;
use dotenv::dotenv;
use std::env;

#[tokio::main]
async fn main() {
    tracing_subscriber::fmt::init();
    dotenv().ok();
    let database_url = env::var("DATABASE_URL").expect("DATABASE_URL must be set");

    let app_state: AppState = AppState {
        db_connection: PgConnection::establish(&database_url)
            .unwrap_or_else(|_| panic!("Error connecting to {}", database_url)),
    };

    let app = Router::new().build_root_routes().build_post_routes();

    let host_port = "0.0.0.0:8080";
    let listener = tokio::net::TcpListener::bind(host_port).await.unwrap();
    axum::serve(listener, app).await.unwrap();
    println!("Listening on port {}", host_port);
}
