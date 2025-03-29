use std::env;
use std::sync::Arc;

use application_core::{
    commands::media::{MediaConfig, S3MediaStorage},
    graphql::query_root::schema,
};
use async_graphql_axum::GraphQL;
use axum::{
    extract::DefaultBodyLimit,
    routing::{delete, get, post},
    Router,
};
use axum_keycloak_auth::{
    instance::{KeycloakAuthInstance, KeycloakConfig},
    layer::KeycloakAuthLayer,
    PassthroughMode,
};
use cms::{
    api, category::delete::delete_handler::api_delete_categories,
    post::delete::delete_handler::api_delete_posts, public,
    tag::delete::delete_handler::api_delete_tags, AppState,
};
use dotenv::{dotenv, from_filename};
use hyper::Method;
use init_tracing_opentelemetry::tracing_subscriber_ext::{
    build_level_filter_layer, build_logger_text, TracingGuard,
};
use reqwest::Url;
use s3::{creds::Credentials, Region};
use sea_orm::Database;
use tower_cookies::CookieManagerLayer;
use tower_http::cors::{Any, CorsLayer};
use tracing::info;

use axum_tracing_opentelemetry::middleware::{OtelAxumLayer, OtelInResponseLayer};
use tracing_subscriber::layer::SubscriberExt;

#[tokio::main]
async fn main() {
    from_filename(".env.local").ok();
    dotenv().ok();

    let _guard = setup_otel_tracing_and_logging();
    if _guard.is_none() {
        setup_logging();
    }

    let app = public_router()
        .await
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

pub fn setup_otel_tracing_and_logging() -> Option<TracingGuard> {
    let enabled_otlp_exporter_str =
        env::var("ENABLED_OTLP_EXPORTER").unwrap_or("false".to_string());
    let enabled_otlp_exporter = enabled_otlp_exporter_str.parse::<bool>().unwrap();

    if enabled_otlp_exporter {
        // very opinionated init of tracing, look as is source to compose your own
        let _guard = init_tracing_opentelemetry::tracing_subscriber_ext::init_subscribers();
        _guard.ok().or(None)
    } else {
        None
    }
}

pub fn setup_logging() {
    let subscriber = tracing_subscriber::registry()
        .with(build_level_filter_layer("").unwrap_or_default())
        .with(build_logger_text());
    tracing::subscriber::set_global_default(subscriber).unwrap();
}

pub async fn public_router() -> Router {
    let app_state = construct_app_state().await;

    Router::new()
        .route("/", get(public::root::handler::handle))
        .route("/health", get(public::root::handler::check_health))
        .route("/healthz", get(public::root::handler::check_health))
        .route(
            "/graphql/immutable",
            get(api::graphql::graphql_immutable).post_service(GraphQL::new(
                app_state.graphql_immutable_schema.as_ref().to_owned(),
            )),
        )
        .layer(OtelInResponseLayer)
        .layer(OtelAxumLayer::default())
        .layer(construct_cors_layer())
}

pub async fn protected_router() -> Router {
    let app_state = construct_app_state().await;

    Router::new()
        .route(
            "/categories",
            get(api::category::read::read_handler::api_get_categories_with_filtering)
                .post(api::category::create::create_handler::api_create_category_with_tags)
                .put(api::category::modify::modify_handler::api_modify_category)
                .delete(api_delete_categories),
        )
        .route(
            "/categories/{category_id}",
            get(api::category::read::read_handler::api_get_category),
        )
        .route(
            "/posts",
            get(api::post::read::read_handler::api_get_posts_with_filtering)
                .post(api::post::create::create_handler::api_create_post)
                .put(api::post::modify::modify_handler::api_modify_post)
                .delete(api_delete_posts),
        )
        .route(
            "/posts/{post_id}",
            get(api::post::read::read_handler::api_get_post),
        )
        .route("/tags", delete(api_delete_tags))
        .route(
            "/media/images",
            post(api::media::create::create_handler::api_create_media_image),
        )
        .route(
            "/graphql/mutable",
            get(api::graphql::graphql_mutable).post_service(GraphQL::new(
                app_state.graphql_mutable_schema.as_ref().to_owned(),
            )),
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
        .layer(DefaultBodyLimit::max(
            env::var("MAX_BODY_LENGTH")
                .unwrap_or((10 * 1024 * 1024).to_string())
                .parse()
                .unwrap(),
        ))
        .layer(construct_cors_layer())
        .with_state(app_state)
}

pub async fn protected_administrator_router() -> Router {
    let app_state = construct_app_state().await;

    Router::new()
        .route(
            "/administrator/database/migration",
            post(api::administrator::migration::migration_handler::handle_api_database_migration),
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
        .layer(construct_cors_layer())
        .with_state(app_state)
}

async fn construct_app_state() -> AppState {
    let database_url = env::var("DATABASE_URL").expect("DATABASE_URL must be set");
    let conn = Database::connect(&database_url).await.unwrap();
    let s3_region_str: String = env::var("S3_REGION").unwrap_or_default();
    let s3_region: Region = s3_region_str.parse().unwrap_or(Region::ApSoutheast1);
    let s3_bucket_name = env::var("S3_BUCKET_NAME").unwrap_or_default();
    let s3_credentials: Credentials =
        Credentials::from_env().unwrap_or(Credentials::default().unwrap());
    let media_imgproxy_server = env::var("MEDIA_IMG_PROXY_SERVER").unwrap_or_default();
    let graphql_immutable_schema = schema(conn.clone(), None, None, false).unwrap();
    let graphql_mutable_schema = schema(conn.clone(), None, None, true).unwrap();

    AppState {
        conn: Arc::new(conn),
        media_config: Arc::new(MediaConfig {
            s3_media_storage: S3MediaStorage {
                s3_region,
                s3_credentials,
                s3_bucket_name,
            },
            media_imgproxy_server,
        }),
        graphql_immutable_schema: Arc::new(graphql_immutable_schema),
        graphql_mutable_schema: Arc::new(graphql_mutable_schema),
    }
}

fn construct_keycloak_auth_instance() -> KeycloakAuthInstance {
    let issuer =
        env::var("KEYCLOAK_ISSUER").unwrap_or("https://my-ids-admin.ducth.dev".to_string());
    let realm = env::var("KEYCLOAK_REALM").unwrap_or("master".to_string());

    KeycloakAuthInstance::new(
        KeycloakConfig::builder()
            .server(Url::parse(&issuer).unwrap())
            .realm(realm)
            .build(),
    )
}

fn construct_cors_layer() -> CorsLayer {
    CorsLayer::new()
        .allow_methods(vec![
            Method::GET,
            Method::POST,
            Method::OPTIONS,
            Method::PUT,
            Method::DELETE,
        ])
        .allow_origin(Any)
        .allow_headers(Any)
}
