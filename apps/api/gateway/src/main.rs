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
//! pre-existing role set on the `domain_posts` standalone binary's mutable mount.
//!
//! Prior to the `merge-graphql-into-posts-domain` change the gateway only
//! accepted the administrator role on `/graphql/mutable`; the role set is
//! widened here to align with the standalone `domain_posts` binary. Do not
//! silently revert to administrator-only — writers that talk to the
//! gateway's GraphQL endpoint depend on it.

#![deny(unsafe_code)]

mod migrate_cli;

use std::env;
use std::process::ExitCode;
use std::sync::Arc;

use axum::{
    response::{self, IntoResponse},
    routing::get,
    Router,
};
use domain_auth::factory::auth_layer_from_env;
use domain_auth::service::DomainAuthService;
use domain_interface::{DomainContext, DomainService, Mount};
use domain_media::handlers::MediaConfig;
use domain_media::service::DomainMediaService;
use domain_posts::service::DomainPostService;
use domain_user::api::state::UserApiState;
use domain_user::handlers::supabase_admin_client::SupabaseAdminClient;
use domain_user::service::DomainUserService;
use tower_cookies::CookieManagerLayer;
use tracing::{error, info};

/// Manifest of domain services registered into the gateway.
///
/// Each future domain (`domain_categories`, `domain_tags`, ...) is appended
/// here as a `Box<dyn DomainService>`. Slice 1 of
/// `wire-all-domains-and-collapse-to-gateway-binary` adds `DomainMediaService`
/// and `DomainUserService` so the gateway exposes every route-owning domain.
pub fn manifest(
    media_config: Arc<MediaConfig>,
    user_state: UserApiState,
) -> Vec<Box<dyn DomainService>> {
    vec![
        Box::new(DomainPostService::new()),
        Box::new(DomainAuthService::new()),
        Box::new(DomainMediaService::new(media_config)),
        Box::new(DomainUserService::from_state(user_state)),
    ]
}

/// Collect `MigrationDescriptor`s from every registered domain and run them
/// against the shared `DatabaseConnection`. The descriptors are
/// deduplicated by `id` and dispatched per-domain via the
/// `DomainService::run_migrations` trait method — no domain name is
/// hard-coded here.
pub async fn run_orchestrator(
    services: &[Box<dyn DomainService>],
    conn: &sea_orm::DatabaseConnection,
) -> Result<(), domain_interface::DomainConfigError> {
    let mut all: Vec<domain_interface::MigrationDescriptor> =
        services.iter().flat_map(|s| s.migrations()).collect();

    // Deduplicate by id, preserving the first occurrence.
    all.sort_by_key(|d| d.id);
    all.dedup_by_key(|d| d.id);

    info!(
        "running {} migration(s) across {} service(s)",
        all.len(),
        services.len()
    );
    for d in &all {
        info!("  - {} (depends_on={:?})", d.id, d.depends_on);
    }

    for service in services {
        let descriptors = service.migrations();
        if descriptors.is_empty() {
            continue;
        }
        let name = service.health().name;
        info!(
            "dispatching {} migration(s) for {}",
            descriptors.len(),
            name
        );
        service
            .run_migrations(conn, &descriptors)
            .await
            .map_err(|e| {
                domain_interface::DomainConfigError::MigrationExecution(format!("{}: {}", name, e))
            })?;
    }

    Ok(())
}

#[tokio::main]
async fn main() -> ExitCode {
    let _ = dotenv::dotenv();

    // CLI dispatch: `my-cms-api migrate <verb>` runs migrations without
    // binding the HTTP listener. Must happen before observability init
    // so the migration output stays clean.
    let args: Vec<String> = std::env::args().skip(1).collect();
    if args.first().map(|s| s.as_str()) == Some("migrate") {
        return migrate_cli::handle_args(&args[1..]).await;
    }

    init_observability();

    let media_config = match MediaConfig::from_env() {
        Ok(c) => Arc::new(c),
        Err(e) => {
            eprintln!("media config failed: {}", e);
            return ExitCode::FAILURE;
        }
    };

    let user_state = match build_user_state() {
        Ok(s) => s,
        Err(e) => {
            eprintln!("user state construction failed: {}", e);
            return ExitCode::FAILURE;
        }
    };

    let services = manifest(media_config, user_state);
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

    let app = match compose_routers(&services, &ctx) {
        Ok(a) => a,
        Err(e) => {
            error!("auth layer construction failed: {}", e);
            eprintln!("auth layer construction failed: {}", e);
            return ExitCode::FAILURE;
        }
    };

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
///
/// Returns `Err(DomainConfigError)` if the auth-layer factory cannot
/// resolve the required env vars (`SUPABASE_URL`, `SUPABASE_JWT_SECRET`).
/// The caller (`main`) propagates the error to `ExitCode::FAILURE`.
fn compose_routers(
    services: &[Box<dyn DomainService>],
    ctx: &DomainContext,
) -> Result<Router<DomainContext>, domain_interface::DomainConfigError> {
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

    let audience =
        env::var("AUTHORIZATION_AUDIENCE").unwrap_or_else(|_| "authenticated".to_string());

    let protected_layer = auth_layer_from_env(
        audience.clone(),
        vec![
            "my-headless-cms-writer".to_string(),
            "my-headless-cms-administrator".to_string(),
        ],
    )?;
    let administrator_layer =
        auth_layer_from_env(audience, vec!["my-headless-cms-administrator".to_string()])?;

    Ok(with_cookie_manager(
        public
            .merge(protected.layer(protected_layer))
            .merge(administrator.layer(administrator_layer)),
    ))
}

/// Supply the request extension required by handlers that extract `Cookies`.
/// Domain routers remain bare, so this belongs at the gateway boundary.
fn with_cookie_manager<S>(router: Router<S>) -> Router<S>
where
    S: Clone + Send + Sync + 'static,
{
    router.layer(CookieManagerLayer::new())
}

/// `GET /` — root banner.
async fn root_handler() -> &'static str {
    "CMS is running successfully!"
}

/// `GET /health` and `GET /healthz` — readiness probe.
async fn health_handler() -> impl IntoResponse {
    response::Html("CMS is running successfully!")
}

pub(crate) async fn connect_database() -> Result<Arc<sea_orm::DatabaseConnection>, String> {
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

/// Build the `UserApiState` from `SUPABASE_URL` + `SUPABASE_SERVICE_ROLE_KEY`.
/// Fails fast with a clear message if either is missing.
pub(crate) fn build_user_state() -> Result<UserApiState, String> {
    let url = env::var("SUPABASE_URL").map_err(|_| "SUPABASE_URL must be set".to_string())?;
    let key = env::var("SUPABASE_SERVICE_ROLE_KEY")
        .map_err(|_| "SUPABASE_SERVICE_ROLE_KEY must be set".to_string())?;
    let client = SupabaseAdminClient::new(url, key);
    Ok(UserApiState::new(client))
}

/// Build the `MediaConfig` from process env vars. Re-exported so
/// `migrate_cli::run_up_orchestrator` can compose the same config the HTTP
/// listener uses.
pub(crate) fn build_media_config() -> Result<MediaConfig, String> {
    MediaConfig::from_env().map_err(|e| format!("{}", e))
}

#[cfg(test)]
mod tests {
    use super::*;
    use axum::{
        body::Body,
        http::{Request, StatusCode},
    };
    use domain_interface::{HealthDescriptor, RouteRegistration};
    use domain_user::handlers::supabase_admin_client::SupabaseAdminClient;
    use std::{ffi::OsString, sync::Mutex};
    use tower::ServiceExt;

    static ENV_LOCK: Mutex<()> = Mutex::new(());

    struct ScopedEnvVar {
        name: &'static str,
        previous: Option<OsString>,
    }

    impl ScopedEnvVar {
        fn set(name: &'static str, value: &str) -> Self {
            let previous = std::env::var_os(name);
            std::env::set_var(name, value);
            Self { name, previous }
        }
    }

    impl Drop for ScopedEnvVar {
        fn drop(&mut self) {
            if let Some(value) = &self.previous {
                std::env::set_var(self.name, value);
            } else {
                std::env::remove_var(self.name);
            }
        }
    }

    struct CookieProbeService;

    impl DomainService for CookieProbeService {
        fn health(&self) -> HealthDescriptor {
            HealthDescriptor {
                name: "cookie-probe",
                version: "test",
            }
        }

        fn required_env(&self) -> &'static [&'static str] {
            &[]
        }

        fn validate_config(&self) -> Result<(), domain_interface::DomainConfigError> {
            Ok(())
        }

        fn migrations(&self) -> Vec<domain_interface::MigrationDescriptor> {
            Vec::new()
        }

        fn register_routes(&self, _ctx: &DomainContext) -> Vec<RouteRegistration> {
            vec![RouteRegistration {
                mount: Mount::Public,
                router: Router::new().route("/cookie-probe", get(cookie_probe)),
                prefix: "/cookie-probe",
            }]
        }
    }

    async fn cookie_probe(_cookies: tower_cookies::Cookies) -> StatusCode {
        StatusCode::OK
    }

    fn stub_domain_context() -> DomainContext {
        use async_graphql::dynamic::{Field, FieldFuture, Object, Schema, TypeRef};
        use async_graphql::Value;

        let schema = || {
            Arc::new(
                Schema::build("Query", None, None)
                    .register(Object::new("Query").field(Field::new(
                        "placeholder",
                        TypeRef::named_nn("Boolean"),
                        |_| FieldFuture::new(async { Ok(Some(Value::from(true))) }),
                    )))
                    .finish()
                    .unwrap(),
            )
        };

        DomainContext {
            conn: Arc::new(sea_orm::DatabaseConnection::default()),
            graphql_immutable: schema(),
            graphql_mutable: schema(),
        }
    }

    fn stub_media_config() -> Arc<MediaConfig> {
        use domain_media::handlers::supabase_storage::SupabaseStorage;
        use domain_media::handlers::MediaConfig;
        Arc::new(MediaConfig {
            storage: SupabaseStorage::new(
                "http://localhost:9999".to_string(),
                "anon".to_string(),
                Some("service-role-test-key".to_string()),
            ),
            bucket: "test-bucket".to_string(),
            media_base_url: "http://localhost:9999".to_string(),
        })
    }

    fn stub_user_state() -> UserApiState {
        UserApiState::new(SupabaseAdminClient::new(
            "http://localhost:9999".to_string(),
            "service-role-test-key".to_string(),
        ))
    }

    #[tokio::test]
    async fn compose_routers_supplies_cookie_extractor_to_domain_routes() {
        let _env_lock = ENV_LOCK.lock().unwrap_or_else(|error| error.into_inner());
        let supabase_url = ScopedEnvVar::set("SUPABASE_URL", "http://localhost:8000");
        let jwt_secret = ScopedEnvVar::set("SUPABASE_JWT_SECRET", "test-secret");

        let ctx = stub_domain_context();
        let services: Vec<Box<dyn DomainService>> = vec![Box::new(CookieProbeService)];
        let app = compose_routers(&services, &ctx).unwrap().with_state(ctx);

        let response = app
            .oneshot(
                Request::builder()
                    .uri("/cookie-probe")
                    .body(Body::empty())
                    .unwrap(),
            )
            .await
            .unwrap();

        assert_eq!(response.status(), StatusCode::OK);

        drop(jwt_secret);
        drop(supabase_url);
    }

    #[test]
    fn manifest_with_four_services_returns_four_entries() {
        let services = manifest(stub_media_config(), stub_user_state());
        assert_eq!(services.len(), 4);
        let names: Vec<&str> = services.iter().map(|s| s.health().name).collect();
        assert!(names.contains(&"domain-posts"));
        assert!(names.contains(&"domain-auth"));
        assert!(names.contains(&"domain-media"));
        assert!(names.contains(&"domain-user"));
    }
}
