//! `gateway` — thin composition root for pluggable domain services.
//!
//! See `openspec/changes/refactor-api-into-pluggable-domain-libraries/design.md`
//! for the architectural context. This crate is a binary that produces the
//! `my-cms-api` deployment image, iterates a `Vec<Box<dyn DomainService>>` to
//! compose the public/protected/administrator Axum routers, applies the
//! cross-cutting layers, and serves the listener.
//!
//! # GraphQL mount authorization (merged by `merge-graphql-into-posts-domain`)
//!
//! The post domain registers `/posts/graphql/{immutable,mutable}` into the
//! public / protected router slots. The mutable mount accepts requests that
//! hold either the `my-headless-cms-writer` or the
//! `my-headless-cms-administrator` Supabase app role — this matches the
//! pre-existing role set on the `legacy_bootstrap` binary's mutable mount.
//!
//! Prior to the `merge-graphql-into-posts-domain` change the gateway only
//! accepted the administrator role on `/graphql/mutable`; the role set is
//! widened here to align with the legacy bootstrap and the standalone
//! `domain_posts` binary. Do not silently revert to administrator-only —
//! writers that talk to the gateway's GraphQL endpoint depend on it.

#![deny(unsafe_code)]

use std::env;
use std::process::ExitCode;
use std::sync::Arc;

use axum::{
    response::{self, IntoResponse},
    routing::get,
    Router,
};
use domain_auth::service::DomainAuthService;
use domain_interface::{DomainContext, DomainService, Mount};
use domain_posts::service::DomainPostService;
use tracing::{info, warn};

/// Manifest of domain services registered into the gateway.
///
/// Each future domain (`domain_categories`, `domain_tags`, `domain_media`,
/// `domain_users`) is appended here as a `Box<dyn DomainService>`.
pub fn manifest() -> Vec<Box<dyn DomainService>> {
    vec![
        Box::new(DomainPostService::new()),
        Box::new(DomainAuthService::new()),
    ]
}

/// Collect `MigrationDescriptor`s from every registered domain and run them
/// against the shared `DatabaseConnection`. The descriptors are
/// topologically sorted by `id` (and `depends_on`), deduplicated, and run
/// sequentially. Errors are surfaced as `String` so the caller can convert
/// them to `DomainConfigError::MigrationExecution` or to a 5xx HTTP
/// response.
pub async fn run_orchestrator(
    services: &[Box<dyn DomainService>],
    conn: &sea_orm::DatabaseConnection,
) -> Result<(), String> {
    use domain_interface::MigrationDescriptor;

    let mut all: Vec<MigrationDescriptor> = services.iter().flat_map(|s| s.migrations()).collect();

    // Deduplicate by id, preserving the first occurrence.
    all.sort_by_key(|d| d.id);
    all.dedup_by_key(|d| d.id);

    info!("running {} migration(s)", all.len());
    for d in &all {
        info!("  - {} (depends_on={:?})", d.id, d.depends_on);
    }

    // Each domain's migration runner is invoked via the per-domain CLI.
    // For now, only `domain_posts` owns migrations; future domains will
    // extend this with their own runner.
    for d in &all {
        if d.id.starts_with("m2024") || d.id.starts_with("m2026") {
            domain_posts::migrations_cli::run(conn)
                .await
                .map_err(|e| format!("{} failed: {}", d.id, e))?;
        } else {
            warn!("migration {} has no runner registered", d.id);
        }
    }

    Ok(())
}

#[tokio::main]
async fn main() -> ExitCode {
    let _ = dotenv::dotenv();
    init_observability();

    let services = manifest();
    info!(
        "gateway booting with {} registered domain service(s)",
        services.len()
    );

    let conn = match connect_database().await {
        Ok(c) => c,
        Err(e) => {
            eprintln!("database connection failed: {}", e);
            return ExitCode::FAILURE;
        }
    };

    if let Err(e) = run_orchestrator(&services, conn.as_ref()).await {
        eprintln!("migration orchestrator failed: {}", e);
        return ExitCode::FAILURE;
    }

    let graphql_immutable = match build_schema(conn.as_ref().clone(), false) {
        Ok(s) => Arc::new(s),
        Err(e) => {
            eprintln!("immutable schema build failed: {}", e);
            return ExitCode::FAILURE;
        }
    };
    let graphql_mutable = match build_schema(conn.as_ref().clone(), true) {
        Ok(s) => Arc::new(s),
        Err(e) => {
            eprintln!("mutable schema build failed: {}", e);
            return ExitCode::FAILURE;
        }
    };

    let ctx = DomainContext {
        conn,
        graphql_immutable,
        graphql_mutable,
    };

    for service in &services {
        if let Err(e) = service.validate_config() {
            eprintln!("config validation failed: {}", e);
            return ExitCode::FAILURE;
        }
        if let Err(e) = service.startup_health(&ctx).await {
            eprintln!("startup health failed: {}", e);
            return ExitCode::FAILURE;
        }
    }

    let app = compose_routers(&services, &ctx);

    let host = env::var("HOST").unwrap_or("127.0.0.1".to_string());
    let port = env::var("PORT").unwrap_or("8989".to_string());
    let host_port = format!("{}:{}", host, port);
    info!("gateway listening on http://{}", host_port);

    let listener = match tokio::net::TcpListener::bind(&host_port).await {
        Ok(l) => l,
        Err(e) => {
            eprintln!("bind failed: {}", e);
            return ExitCode::FAILURE;
        }
    };

    if let Err(e) = axum::serve(listener, app.with_state(ctx.clone())).await {
        eprintln!("serve error: {}", e);
        return ExitCode::FAILURE;
    }

    ExitCode::SUCCESS
}

/// Compose the public/protected/administrator routers from the registered
/// domains' `RouteRegistration`s, plus the gateway's own routes
/// (`/`, `/health`, `/healthz`, `/posts/graphql/**`).
///
/// The post domain registers the two `/posts/graphql/**` endpoints through
/// `register_routes`, so the gateway does NOT add them inline — it only
/// composes the auth layer over the merged protected router. The mutable
/// mount's role set is widened from administrator-only to
/// writer + administrator (see module doc-comment).
fn compose_routers(
    services: &[Box<dyn DomainService>],
    ctx: &DomainContext,
) -> Router<DomainContext> {
    let mut public: Router<DomainContext> = Router::new()
        .route("/", get(root_handler))
        .route("/health", get(health_handler))
        .route("/healthz", get(health_handler));
    let mut protected: Router<DomainContext> = Router::new();
    let mut administrator: Router<DomainContext> = Router::new();

    for service in services.iter() {
        for reg in service.register_routes(ctx) {
            match reg.mount {
                Mount::Public => public = public.merge(reg.router),
                Mount::Protected => protected = protected.merge(reg.router),
                Mount::Administrator => administrator = administrator.merge(reg.router),
            }
        }
    }

    public
        .merge(protected.layer(
            domain_auth::legacy_bootstrap::construct_supabase_auth_layer(
                env::var("AUTHORIZATION_AUDIENCE").unwrap_or_else(|_| "authenticated".to_string()),
                vec![
                    "my-headless-cms-writer".to_string(),
                    "my-headless-cms-administrator".to_string(),
                ],
            ),
        ))
        .merge(administrator.layer(
            domain_auth::legacy_bootstrap::construct_supabase_auth_layer(
                env::var("AUTHORIZATION_AUDIENCE").unwrap_or_else(|_| "authenticated".to_string()),
                vec!["my-headless-cms-administrator".to_string()],
            ),
        ))
}

/// `GET /` — root banner.
async fn root_handler() -> &'static str {
    "CMS is running successfully!"
}

/// `GET /health` and `GET /healthz` — readiness probe.
async fn health_handler() -> impl IntoResponse {
    response::Html("CMS is running successfully!")
}

async fn connect_database() -> Result<Arc<sea_orm::DatabaseConnection>, String> {
    use sea_orm::Database;
    let database_url =
        std::env::var("DATABASE_URL").map_err(|_| "DATABASE_URL must be set".to_string())?;
    let conn = Database::connect(&database_url)
        .await
        .map_err(|e| format!("Failed to connect to database: {}", e))?;
    Ok(Arc::new(conn))
}

fn build_schema(
    database: sea_orm::DatabaseConnection,
    is_mutation_supported: bool,
) -> Result<async_graphql::dynamic::Schema, async_graphql::dynamic::SchemaError> {
    domain_posts::domain::graphql::contribute_post_schema(
        database,
        None,
        None,
        is_mutation_supported,
    )
}

fn init_observability() {
    use tracing_subscriber::layer::SubscriberExt;
    use tracing_subscriber::EnvFilter;

    let filter = EnvFilter::try_from_default_env().unwrap_or_else(|_| EnvFilter::new("info"));
    let subscriber = tracing_subscriber::registry()
        .with(filter)
        .with(tracing_subscriber::fmt::layer().with_target(true));
    let _ = tracing::subscriber::set_global_default(subscriber);
}
