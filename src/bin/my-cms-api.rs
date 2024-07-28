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
use init_tracing_opentelemetry::{
    tracing_subscriber_ext::{build_logger_text, build_loglevel_filter_layer, build_otel_layer},
    Error,
};
use reqwest::Url;
use sea_orm::Database;
use tower_cookies::CookieManagerLayer;
use tracing::info;

use axum_tracing_opentelemetry::middleware::{OtelAxumLayer, OtelInResponseLayer};
use tracing_subscriber::layer::SubscriberExt;

#[tokio::main]
async fn main() {
    dotenv().ok();
    init_my_subscribers().unwrap();

    let app = public_router()
        .merge(protected_router().await)
        .merge(protected_administrator_router().await);
    info!("Starting server...");

    let host = env::var("HOST").unwrap_or("127.0.0.1".to_string());
    let port = env::var("PORT").unwrap_or("8989".to_string());
    let host_port = format!("{}:{}", host, port);
    tracing::info!("App will host on `http://{}`", host_port);

    let listener = tokio::net::TcpListener::bind(&host_port).await.unwrap();
    axum::serve(listener, app).await.unwrap();
    tracing::info!(
        "App is ready, please try to run `curl -i http://{}/healthz`",
        host_port
    );
}

pub fn init_my_subscribers() -> Result<(), Error> {
    let enabled_otlp_exporter_str =
        env::var("ENABLED_OTLP_EXPORTER").unwrap_or("false".to_string());
    let enabled_otlp_exporter = enabled_otlp_exporter_str.parse::<bool>().unwrap();

    //setup a temporary subscriber to log output during setup
    let subscriber = tracing_subscriber::registry()
        .with(build_loglevel_filter_layer())
        .with(build_logger_text());
    let _guard = tracing::subscriber::set_default(subscriber);
    info!("init logging & tracing");

    if enabled_otlp_exporter {
        let subscriber = tracing_subscriber::registry()
            .with(build_otel_layer()?)
            .with(build_loglevel_filter_layer())
            .with(build_logger_text());
        tracing::subscriber::set_global_default(subscriber)?;
    } else {
        let subscriber = tracing_subscriber::registry()
            .with(build_loglevel_filter_layer())
            .with(build_logger_text());
        tracing::subscriber::set_global_default(subscriber)?;
    }
    Ok(())
}

pub fn public_router() -> Router {
    Router::new()
        .route("/", get(public::root::handler::handle))
        .route("/health", get(public::root::handler::check_health))
        .route("/healthz", get(public::root::handler::check_health))
        .layer(OtelInResponseLayer)
        .layer(OtelAxumLayer::default())
}

pub async fn protected_router() -> Router {
    let app_state = construct_app_state().await;

    Router::new()
        .route(
            "/categories",
            get(commands::category::read::read_handler::api_get_all_categories)
                .post(commands::category::create::create_handler::api_create_category_with_tags)
                .put(commands::category::modify::modify_handler::api_modify_category),
        )
        .route(
            "/posts",
            get(commands::post::read::read_handler::api_get_all_posts)
                .post(commands::post::create::create_handler::api_create_post)
                .put(commands::post::modify::modify_handler::api_modify_post),
        )
        .layer(
            KeycloakAuthLayer::<String>::builder()
                .instance(construct_keycloak_auth_instance())
                .passthrough_mode(PassthroughMode::Block)
                .persist_raw_claims(false)
                .expected_audiences(vec![String::from("my-headless-cms-api")])
                .required_roles(vec![String::from("my-headless-cms-writer")])
                .build(),
        )
        .layer(OtelInResponseLayer)
        .layer(OtelAxumLayer::default())
        .layer(CookieManagerLayer::new())
        .with_state(app_state)
}

pub async fn protected_administrator_router() -> Router {
    let app_state = construct_app_state().await;

    Router::new()
        .route(
            "/administrator/database/migration",
            post(commands::administrator::migration::migration_handler::handle_api_database_migration),
        )
        .layer(
            KeycloakAuthLayer::<String>::builder()
                .instance(construct_keycloak_auth_instance())
                .passthrough_mode(PassthroughMode::Block)
                .persist_raw_claims(false)
                .expected_audiences(vec![String::from("my-headless-cms-api")])
                .required_roles(vec![String::from("my-headless-cms-administrator")])
                .build(),
        )
        .layer(OtelInResponseLayer)
        .layer(OtelAxumLayer::default())
        .layer(CookieManagerLayer::new())
        .with_state(app_state)
}

async fn construct_app_state() -> AppState {
    let database_url = env::var("DATABASE_URL").expect("DATABASE_URL must be set");
    let conn = Database::connect(&database_url).await.unwrap();
    AppState { conn }
}

fn construct_keycloak_auth_instance() -> KeycloakAuthInstance {
    let issuer =
        env::var("KEYCLOAK_ISSUER").unwrap_or("https://keycloak-admin.doitsu.tech".to_string());
    let realm = env::var("KEYCLOAK_REALM").unwrap_or("master".to_string());

    KeycloakAuthInstance::new(
        KeycloakConfig::builder()
            .server(Url::parse(&issuer).unwrap())
            .realm(realm)
            .build(),
    )
}
