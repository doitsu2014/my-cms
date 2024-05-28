use std::env;

use axum::{routing::get, Router};
use cms::{post_handler, root_handler, tracing_initializer, AppState};
use dotenv::dotenv;
use sea_orm::Database;
use tower_cookies::CookieManagerLayer;
use tracing::info;

use axum_tracing_opentelemetry::middleware::{OtelAxumLayer, OtelInResponseLayer};

#[tokio::main]
async fn main() {
    dotenv().ok();
    init_tracing_opentelemetry::tracing_subscriber_ext::init_subscribers().unwrap();

    let database_url = env::var("DATABASE_URL").expect("DATABASE_URL must be set");
    let conn = Database::connect(&database_url).await.unwrap();

    let app_state = AppState { conn };
    let app = Router::new()
        .route("/", get(root_handler::handle))
        .route("/healthz", get(root_handler::check_health))
        .route(
            "/admin/database/migration",
            get(root_handler::admin_database_migration),
        )
        .route(
            "/posts",
            get(post_handler::handle_get_list).post(post_handler::handle_post),
        )
        .layer(OtelInResponseLayer::default())
        .layer(OtelAxumLayer::default())
        .layer(CookieManagerLayer::new())
        .with_state(app_state);

    info!("Starting server...");

    let host = env::var("HOST").expect("HOST must be set in .env file");
    let port = env::var("PORT").expect("PORT must be set in .env file");
    let host_port = format!("{}:{}", host, port);
    let listener = tokio::net::TcpListener::bind(&host_port).await.unwrap();
    axum::serve(listener, app).await.unwrap();
}
