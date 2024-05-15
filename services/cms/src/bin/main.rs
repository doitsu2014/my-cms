use axum::{extract::Request, routing::get, Router};
use cms::{post_handler, root_handler, AppState};
use dotenv::dotenv;
use hyper::{body::Incoming, server};
use sea_orm::Database;
use std::env;
use tower::Service;
use tower_cookies::CookieManagerLayer;
use tracing_subscriber::layer::SubscriberExt;
use hyper-util::conn::TokioExecutor;

#[tokio::main]
async fn main() {
    tracing_subscriber::registry()
        .with(
            tracing_subscriber::EnvFilter::try_from_default_env()
                .unwrap_or_else(|_| "example_dependency_injection=debug".into()),
        )
        .with(tracing_subscriber::fmt::layer());

    dotenv().ok();
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
        .layer(CookieManagerLayer::new())
        .with_state(app_state);
    let tower_service = app.clone();

    let host = env::var("HOST").expect("HOST must be set in .env file");
    let port = env::var("PORT").expect("PORT must be set in .env file");
    let host_port = format!("{}:{}", host, port);
    let listener = tokio::net::TcpListener::bind(&host_port).await.unwrap();
    let (socket, _remote_addr) = listener.accept().await.unwrap();

    let hyper_service = hyper::service::service_fn(move |request: Request<Incoming>| {
        tower_service.clone().call(request)
    });

    let server = server::conn::http2::Builder::new(TokioExecutor::new()).serve_connection();

    // axum::serve(listener, app).await.unwrap();

    println!("Listening on port {}", host_port);
}
