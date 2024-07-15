use std::env;

use axum::{
    routing::{get, post},
    Router,
};
use axum_keycloak_auth::{
    instance::{KeycloakAuthInstance, KeycloakConfig},
    layer::KeycloakAuthLayer,
    PassthroughMode,
};
use cms::{commands, public, AppState};
use dotenv::dotenv;
use reqwest::Url;
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
    let keycloak_auth_instance = KeycloakAuthInstance::new(
        KeycloakConfig::builder()
            .server(Url::parse("https://keycloak-admin.doitsu.tech").unwrap())
            .realm(String::from("master"))
            .build(),
    );

    let app = public_router().merge(protected_router(keycloak_auth_instance, app_state));

    info!("Starting server...");

    let host = env::var("HOST").expect("HOST must be set in .env file");
    let port = env::var("PORT").expect("PORT must be set in .env file");
    let host_port = format!("{}:{}", host, port);

    tracing::info!("try to call `curl -i http://{}/`", host_port); //Devskim: ignore DS137138
    tracing::info!("try to call `curl -i http://{}/healthz`", host_port); //Devskim: ignore DS137138

    let listener = tokio::net::TcpListener::bind(&host_port).await.unwrap();
    axum::serve(listener, app).await.unwrap();
}

pub fn public_router() -> Router {
    Router::new()
        .route("/", get(public::root::handler::handle))
        .route("/health", get(public::root::handler::check_health))
        .route("/healthz", get(public::root::handler::check_health))
        .layer(OtelInResponseLayer)
        .layer(OtelAxumLayer::default())
}

pub fn protected_router(instance: KeycloakAuthInstance, app_state: AppState) -> Router {
    Router::new()
        .route(
            "/administrator/database/migration",
            post(commands::administrator::migration::migration_handler::handle_api_database_migration),
        )
        .route(
            "/categories",
            get(commands::category::read::read_handler::handle_api_get_all_categories)
                .post(commands::category::create::create_handler::handle_api_create_category),
        )
        .route(
            "/posts",
            get(commands::post::read::read_handler::handle_api_get_all_posts)
                .post(commands::post::create::create_handler::handle_api_create_post),
        )
        .layer(
            KeycloakAuthLayer::<String>::builder()
                .instance(instance)
                .passthrough_mode(PassthroughMode::Block)
                .persist_raw_claims(false)
                .expected_audiences(vec![String::from("my-headless-cms-api")])
                .required_roles(vec![String::from("my-cms-headless-administrator")])
                .build(),
        )
        .layer(OtelInResponseLayer)
        .layer(OtelAxumLayer::default())
        .layer(CookieManagerLayer::new())
        .with_state(app_state)
}
