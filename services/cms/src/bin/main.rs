use axum::{routing::get, Router};
use cms::{post_handler, root_handler, AppState};
use dotenv::dotenv;
use sea_orm::Database;
use std::env;
use tower_cookies::CookieManagerLayer;

#[tokio::main]
async fn main() {
    tracing_subscriber::fmt::init();
    dotenv().ok();
    let database_url = env::var("DATABASE_URL").expect("DATABASE_URL must be set");
    let conn = Database::connect(&database_url).await.unwrap();

    let app_state = AppState { conn: conn.clone() };

    let app = Router::new()
        .route("/", get(root_handler::handle))
        .route("/health", get(root_handler::check_health))
        .route("/healthz", get(root_handler::check_health))
        .route(
            "/admin/database/migration",
            get(root_handler::admin_database_migration),
        )
        .route("/posts", get(post_handler::handle_get_list))
        .layer(CookieManagerLayer::new())
        .with_state(app_state);

    let host = env::var("HOST").expect("HOST must be set in .env file");
    let port = env::var("PORT").expect("PORT must be set in .env file");
    let host_port = format!("{}:{}", host, port);

    let listener = tokio::net::TcpListener::bind(&host_port).await.unwrap();
    axum::serve(listener, app).await.unwrap();
    println!("Listening on port {}", host_port);
}
