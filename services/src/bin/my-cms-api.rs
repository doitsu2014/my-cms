use std::env;
use std::sync::Arc;

use application_core::{
    commands::media::{read::read_handler::create_media_cache, MediaConfig, S3MediaStorage},
    graphql::query_root::schema,
};
use async_graphql_axum::GraphQL;
use axum::{
    extract::DefaultBodyLimit,
    routing::{delete, get, post},
    Router,
};
use cms::{
    common::supabase_auth::{SupabaseAuthConfig, SupabaseAuthLayer},

    api, category::delete::delete_handler::api_delete_categories,
    post::delete::delete_handler::api_delete_posts, public,
    tag::delete::delete_handler::api_delete_tags, AppState,
};
use dotenv::{dotenv, from_filename};
use hyper::Method;
use init_tracing_opentelemetry::{
    otlp::OtelGuard,
    tracing_subscriber_ext::{build_level_filter_layer, build_logger_text},
};
use s3::creds::Credentials;
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

pub fn setup_otel_tracing_and_logging() -> Option<OtelGuard> {
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
        // Image delivery with resize support
        .route(
            "/media/images/{*path}",
            get(api::media::read::read_handler::api_get_media_image),
        )
        // General media delivery (documents, etc.) - no resize
        .route(
            "/media/{*path}",
            get(api::media::read::read_handler::api_get_media),
        )
        .route(
            "/graphql/immutable",
            get(api::graphql::graphql_immutable).post_service(GraphQL::new(
                app_state.graphql_immutable_schema.as_ref().to_owned(),
            )),
        )
        .layer(OtelInResponseLayer)
        .layer(OtelAxumLayer::default())
        .layer(construct_cors_layer())
        .with_state(app_state)
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
        .route(
            "/posts/{post_id}/translate",
            post(api::post::translate::translate_handler::api_translate_post),
        )
        .route(
            "/posts/{post_id}/translate/background",
            post(api::post::translate::translate_handler::api_translate_post_background),
        )
        .route(
            "/posts/{post_id}/translate/jobs/{job_id}",
            get(api::post::translate::job_handler::api_get_job_status),
        )
        .route(
            "/posts/{post_id}/translate/jobs",
            get(api::post::translate::job_handler::api_get_active_jobs),
        )
        .route(
            "/ai/models",
            get(api::ai::models::models_handler::api_get_openai_models),
        )
        .route("/tags", delete(api_delete_tags))
        // Media management routes
        .route(
            "/media",
            get(api::media::list::list_handler::api_list_media)
                .post(api::media::create::create_handler::api_create_media)
                .delete(api::media::delete::delete_handler::api_delete_media_batch),
        )
        .route(
            "/media/info/{*path}",
            get(api::media::read::metadata_handler::api_get_media_metadata),
        )
        .route(
            "/media/delete/{*path}",
            delete(api::media::delete::delete_handler::api_delete_media),
        )
        .route(
            "/graphql/mutable",
            get(api::graphql::graphql_mutable).post_service(GraphQL::new(
                app_state.graphql_mutable_schema.as_ref().to_owned(),
            )),
        )
        .layer(construct_supabase_auth_layer(
            env::var("AUTHORIZATION_AUDIENCE").unwrap_or("authenticated".to_string()),
            vec![String::from("my-headless-cms-writer")],
        ))
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
        .layer(construct_supabase_auth_layer(
            env::var("AUTHORIZATION_AUDIENCE").unwrap_or("authenticated".to_string()),
            vec![String::from("my-headless-cms-administrator")],
        ))
        .layer(OtelInResponseLayer)
        .layer(OtelAxumLayer::default())
        .layer(CookieManagerLayer::new())
        .layer(construct_cors_layer())
        .with_state(app_state)
}

async fn construct_app_state() -> AppState {
    let database_url = env::var("DATABASE_URL").expect("DATABASE_URL must be set");
    let conn = Database::connect(&database_url).await.unwrap();
    let s3_endpoint = env::var("S3_ENDPOINT").unwrap_or_default();
    let s3_bucket_name = env::var("S3_BUCKET_NAME").unwrap_or_default();
    let s3_credentials: Credentials =
        Credentials::from_env().unwrap_or(Credentials::default().unwrap());

    let host = env::var("HOST").unwrap_or("127.0.0.1".to_string());
    let port = env::var("PORT").unwrap_or("8989".to_string());
    let media_base_url =
        env::var("MEDIA_BASE_URL").unwrap_or(format!("http://{}:{}", host, port));

    let graphql_immutable_schema = schema(conn.clone(), None, None, false).unwrap();
    let graphql_mutable_schema = schema(conn.clone(), None, None, true).unwrap();

    AppState {
        conn: Arc::new(conn),
        media_config: Arc::new(MediaConfig {
            s3_media_storage: S3MediaStorage {
                s3_endpoint,
                s3_credentials,
                s3_bucket_name,
            },
            media_base_url,
        }),
        media_cache: Arc::new(create_media_cache()),
        graphql_immutable_schema: Arc::new(graphql_immutable_schema),
        graphql_mutable_schema: Arc::new(graphql_mutable_schema),
    }
}

fn construct_supabase_auth_layer(expected_audience: String, required_roles: Vec<String>) -> SupabaseAuthLayer {
    let supabase_url = env::var("SUPABASE_URL").expect("SUPABASE_URL must be set");
    let jwt_secret = env::var("SUPABASE_JWT_SECRET").expect("SUPABASE_JWT_SECRET must be set");

    SupabaseAuthLayer::new(SupabaseAuthConfig {
        supabase_url,
        jwt_secret,
        expected_audience,
        required_roles,
    })
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
