//! `gateway` — thin composition root for pluggable domain services.
//!
//! This crate is the binary that produces the `my-cms-api` deployment image.
//! It iterates a `Vec<Box<dyn DomainService>>` to compose the
//! public/protected/administrator Axum routers, applies the cross-cutting
//! layers, and serves the listener. See
//! `docs/architecture/software-architecture.md`.
//!
//! # GraphQL mount authorization
//!
//! The post domain registers `/posts/graphql/{immutable,mutable}` into the
//! public / protected router slots. The mutable mount accepts requests that
//! hold either the `my-headless-cms-writer` or the
//! `my-headless-cms-administrator` Supabase app role. Do not narrow the
//! mutable mount to administrator-only: writers that call the gateway
//! GraphQL endpoint depend on the writer role.

#![deny(unsafe_code)]

mod backup_cli;
mod migrate_cli;
#[cfg(test)]
mod migration_tests;

use std::env;
use std::process::ExitCode;
use std::sync::Arc;
use std::time::Duration;

use axum::{
    body::Body,
    extract::{Request, State},
    middleware::{self, Next},
    response::{self, IntoResponse, Response},
    routing::get,
    Router,
};
use domain_auth::factory::auth_layer_from_env;
use domain_auth::service::DomainAuthService;
use domain_backup::domain::config::{BackupConfig, BackupConfigState};
use domain_backup::service::DomainBackupService;
use domain_interface::{DomainContext, DomainService, Mount};
use domain_media::handlers::MediaConfig;
use domain_media::service::DomainMediaService;
use domain_posts::service::DomainPostService;
use domain_seo::service::DomainSeoService;
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
    backup: DomainBackupService,
) -> Vec<Box<dyn DomainService>> {
    vec![
        Box::new(DomainPostService::new()),
        Box::new(DomainAuthService::new()),
        Box::new(DomainMediaService::new(media_config)),
        Box::new(DomainUserService::from_state(user_state)),
        Box::new(DomainSeoService::new()),
        Box::new(backup),
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

    // Operator CLI commands run without binding the HTTP listener.
    let args: Vec<String> = std::env::args().skip(1).collect();
    if args.first().map(|s| s.as_str()) == Some("migrate") {
        return migrate_cli::handle_args(&args[1..]).await;
    }
    if args.first().map(|s| s.as_str()) == Some("backup") {
        return backup_cli::handle_args(&args[1..]).await;
    }

    // Keep the OTLP guard alive until the HTTP server finishes so its exporter
    // can continue receiving spans and flush on shutdown.
    let _otel_guard = init_observability();

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

    let bind_host = effective_bind_host(env_lookup);
    let backup = DomainBackupService::new(build_backup_config(&bind_host, env_lookup));
    let ingress_limits = IngressLimits::from_backup_state(backup.config());
    let services = manifest(media_config, user_state, backup.clone());
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

    // The runtime is started by the backup service's `startup_health` above, after the
    // migrations. A started runtime gets its periodic driver here.
    if backup.runtime().is_some() {
        spawn_backup_scheduler(backup.clone());
    }

    let app = match compose_routers_with_limits(&services, &ctx, ingress_limits) {
        Ok(a) => a,
        Err(e) => {
            error!("auth layer construction failed: {}", e);
            eprintln!("auth layer construction failed: {}", e);
            return ExitCode::FAILURE;
        }
    };

    let host = bind_host;
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

/// Owner-approved ingress limits for the backup router (CFG-17, CFG-18).
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub(crate) struct IngressLimits {
    pub request_timeout: Duration,
    pub max_header_bytes: usize,
}

impl IngressLimits {
    pub(crate) const OWNER_APPROVED: Self = Self {
        request_timeout: Duration::from_secs(30),
        max_header_bytes: 16384,
    };

    /// Read CFG-17 and CFG-18 from a validated backup configuration. A disabled backup still
    /// has its routes wrapped (they answer `BACKUP_DISABLED`), with the owner-approved values.
    pub(crate) fn from_backup_state(state: &BackupConfigState) -> Self {
        match state {
            BackupConfigState::Enabled(config) => Self {
                request_timeout: Duration::from_secs(config.request_timeout_secs()),
                max_header_bytes: usize::try_from(config.max_header_bytes()).unwrap_or(usize::MAX),
            },
            BackupConfigState::Disabled { .. } => Self::OWNER_APPROVED,
        }
    }
}

/// CFG-17: answer 408 with the backup error body once the inner service (authentication
/// and the handler) takes longer than the limit. The inner future is dropped, which cancels
/// its work. Slow header delivery is hyper's concern and happens before the router.
async fn backup_request_timeout(
    State(limits): State<IngressLimits>,
    request: Request<Body>,
    next: Next,
) -> Response {
    match tokio::time::timeout(limits.request_timeout, next.run(request)).await {
        Ok(response) => response,
        Err(_) => domain_backup::domain::response::ingress_request_timeout().into_response(),
    }
}

/// CFG-18: answer 431 with the backup error body when the request headers (name plus value
/// bytes of every header) exceed the limit. It sits outside the authentication layer, so an
/// oversized request is refused before any credential is looked at.
async fn backup_header_size_limit(
    State(limits): State<IngressLimits>,
    request: Request<Body>,
    next: Next,
) -> Response {
    let header_bytes: usize = request
        .headers()
        .iter()
        .map(|(name, value)| name.as_str().len() + value.len())
        .sum();
    if header_bytes > limits.max_header_bytes {
        return domain_backup::domain::response::ingress_header_too_large().into_response();
    }
    next.run(request).await
}

/// The bind host the listener uses (`HOST`, default loopback).
pub(crate) fn effective_bind_host<F>(lookup: F) -> String
where
    F: Fn(&str) -> Option<String>,
{
    lookup("HOST").unwrap_or_else(|| "127.0.0.1".to_string())
}

/// Backup configuration with the effective bind host (F-39).
///
/// The listener binds `bind_host`, so that value, not a separate read of `HOST`, decides the
/// loopback guard: a non-loopback bind forces backup disabled (`NON_LOOPBACK_BIND`).
pub(crate) fn build_backup_config<F>(bind_host: &str, lookup: F) -> BackupConfigState
where
    F: Fn(&str) -> Option<String>,
{
    BackupConfig::load(|key| {
        if key == "HOST" {
            Some(bind_host.to_string())
        } else {
            lookup(key)
        }
    })
}

/// `compose_routers_with_limits` with the owner-approved ingress limits (test convenience).
#[cfg(test)]
fn compose_routers(
    services: &[Box<dyn DomainService>],
    ctx: &DomainContext,
) -> Result<Router<DomainContext>, domain_interface::DomainConfigError> {
    compose_routers_with_limits(services, ctx, IngressLimits::OWNER_APPROVED)
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
///
/// `Mount::AdministratorIngress` registrations (the backup routes, F-38) are merged into a
/// separate router wrapped, outermost to innermost, by the request timeout (CFG-17), the
/// header-size limit (CFG-18) and the administrator auth layer, so oversized headers get 431
/// before authentication and a slow inner service gets 408. Every other route keeps its
/// layering.
fn compose_routers_with_limits(
    services: &[Box<dyn DomainService>],
    ctx: &DomainContext,
    limits: IngressLimits,
) -> Result<Router<DomainContext>, domain_interface::DomainConfigError> {
    let mut public: Router<DomainContext> = Router::new()
        .route("/", get(root_handler))
        .route("/health", get(health_handler))
        .route("/healthz", get(health_handler));
    let mut protected: Router<DomainContext> = Router::new();
    let mut administrator: Router<DomainContext> = Router::new();
    let mut backup_admin: Router<DomainContext> = Router::new();

    for service in services.iter() {
        for reg in service.register_routes(ctx) {
            match reg.mount {
                Mount::Public => public = public.merge(reg.router),
                Mount::Protected => protected = protected.merge(reg.router),
                Mount::Administrator => administrator = administrator.merge(reg.router),
                Mount::AdministratorIngress => backup_admin = backup_admin.merge(reg.router),
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

    // F-38: the backup routes get their own administrator router. Outermost to innermost:
    // request timeout (CFG-17), header-size limit (CFG-18), administrator auth, routes. Each
    // `.layer` wraps the previous ones, so the last call is the outermost.
    let backup_admin = backup_admin
        .layer(administrator_layer.clone())
        .layer(middleware::from_fn_with_state(
            limits,
            backup_header_size_limit,
        ))
        .layer(middleware::from_fn_with_state(
            limits,
            backup_request_timeout,
        ));

    let (otel_in_response, otel_axum) = domain_posts::domain::layers::otel_layers();
    Ok(with_cookie_manager(
        public
            .merge(protected.layer(protected_layer))
            .merge(administrator.layer(administrator_layer))
            .merge(backup_admin),
    )
    .layer(otel_in_response)
    .layer(otel_axum))
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

fn init_observability() -> Option<init_tracing_opentelemetry::otlp::OtelGuard> {
    init_observability_with(
        domain_posts::observability::init,
        domain_posts::observability::init_text_logging,
    )
}

fn init_observability_with<F, G>(
    init_otel: F,
    init_text_logging: G,
) -> Option<init_tracing_opentelemetry::otlp::OtelGuard>
where
    F: FnOnce() -> Option<init_tracing_opentelemetry::otlp::OtelGuard>,
    G: FnOnce(),
{
    let guard = init_otel();
    if guard.is_none() {
        init_text_logging();
    }
    guard
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

/// Process-environment lookup for configuration readers. A present but non-UTF-8 value is
/// an empty string, so it fails validation rather than counting as absent.
fn env_lookup(key: &str) -> Option<String> {
    env::var_os(key).map(|value| value.into_string().unwrap_or_default())
}

/// The backup domain service for the process environment, with the effective bind host.
pub(crate) fn backup_service() -> DomainBackupService {
    DomainBackupService::new(build_backup_config(
        &effective_bind_host(env_lookup),
        env_lookup,
    ))
}

/// Period of the backup scheduler driver. The runtime decides what is due (cleaner tick every
/// `BACKUP_CLEANUP_BACKOFF_SECS`, sweep every `BACKUP_SWEEP_INTERVAL_SECS`); this only wakes it.
const BACKUP_SCHEDULER_PERIOD: Duration = Duration::from_secs(60);

/// Period of the backup worker driver (F-16): how long a queued job waits at most before the
/// worker claims it. Well inside the queued-age bound (CFG-10, 600 s) after which the sweeper
/// fails an unclaimed job with `DISPATCH_FAILED`.
const BACKUP_WORKER_PERIOD: Duration = Duration::from_secs(2);

fn spawn_backup_scheduler(backup: DomainBackupService) {
    let worker = backup.clone();
    tokio::spawn(async move {
        let mut interval = tokio::time::interval(BACKUP_WORKER_PERIOD);
        loop {
            interval.tick().await;
            worker.worker_step().await;
        }
    });
    tokio::spawn(async move {
        let mut interval = tokio::time::interval(BACKUP_SCHEDULER_PERIOD);
        // The first tick completes at once; the runtime has just run its start-up cycle.
        interval.tick().await;
        loop {
            interval.tick().await;
            backup.scheduler_step().await;
        }
    });
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
    use std::{
        ffi::OsString,
        sync::{Arc, Mutex},
    };
    use tower::ServiceExt;
    use tracing_subscriber::{
        layer::{Context, SubscriberExt},
        registry::LookupSpan,
    };

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

        fn unset(name: &'static str) -> Self {
            let previous = std::env::var_os(name);
            std::env::remove_var(name);
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

    #[derive(Clone, Default)]
    struct CapturedHttpSpans {
        spans: Arc<Mutex<Vec<CapturedSpan>>>,
    }

    #[derive(Debug, Default)]
    struct CapturedSpan {
        name: String,
        fields: Vec<(String, String)>,
    }

    impl CapturedHttpSpans {
        fn has_request(&self, route: &str, status: StatusCode) -> bool {
            self.spans.lock().unwrap().iter().any(|span| {
                span.name == "HTTP request"
                    && span
                        .fields
                        .iter()
                        .any(|(field, value)| field == "http.route" && value == route)
                    && span.fields.iter().any(|(field, value)| {
                        field == "http.response.status_code"
                            && value == &status.as_u16().to_string()
                    })
            })
        }
    }

    impl<S> tracing_subscriber::Layer<S> for CapturedHttpSpans
    where
        S: tracing::Subscriber + for<'a> LookupSpan<'a>,
    {
        fn on_new_span(
            &self,
            attrs: &tracing::span::Attributes<'_>,
            id: &tracing::span::Id,
            ctx: Context<'_, S>,
        ) {
            let mut visitor = FieldVisitor::default();
            attrs.record(&mut visitor);
            let span = ctx.span(id).expect("span exists");
            span.extensions_mut().insert(CapturedSpan {
                name: attrs.metadata().name().to_string(),
                fields: visitor.0,
            });
        }

        fn on_record(
            &self,
            id: &tracing::span::Id,
            values: &tracing::span::Record<'_>,
            ctx: Context<'_, S>,
        ) {
            let mut visitor = FieldVisitor::default();
            values.record(&mut visitor);
            let span = ctx.span(id).expect("span exists");
            let mut extensions = span.extensions_mut();
            extensions
                .get_mut::<CapturedSpan>()
                .expect("captured span fields inserted")
                .fields
                .extend(visitor.0);
        }

        fn on_close(&self, id: tracing::span::Id, ctx: Context<'_, S>) {
            let span = ctx.span(&id).expect("span exists");
            let captured = span
                .extensions_mut()
                .remove::<CapturedSpan>()
                .expect("captured span fields inserted");
            self.spans.lock().unwrap().push(captured);
        }
    }

    #[derive(Default)]
    struct FieldVisitor(Vec<(String, String)>);

    impl tracing::field::Visit for FieldVisitor {
        fn record_str(&mut self, field: &tracing::field::Field, value: &str) {
            self.0.push((field.name().to_string(), value.to_string()));
        }

        fn record_debug(&mut self, field: &tracing::field::Field, value: &dyn std::fmt::Debug) {
            self.0
                .push((field.name().to_string(), format!("{value:?}")));
        }

        fn record_i64(&mut self, field: &tracing::field::Field, value: i64) {
            self.0.push((field.name().to_string(), value.to_string()));
        }

        fn record_u64(&mut self, field: &tracing::field::Field, value: u64) {
            self.0.push((field.name().to_string(), value.to_string()));
        }
    }

    struct ProtectedProbeService {
        handler_calls: Arc<std::sync::atomic::AtomicUsize>,
    }

    impl DomainService for ProtectedProbeService {
        fn health(&self) -> HealthDescriptor {
            HealthDescriptor {
                name: "protected-probe",
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
            let handler_calls = self.handler_calls.clone();
            vec![RouteRegistration {
                mount: Mount::Protected,
                router: Router::new().route(
                    "/protected-probe",
                    get(move || {
                        let handler_calls = handler_calls.clone();
                        async move {
                            handler_calls.fetch_add(1, std::sync::atomic::Ordering::SeqCst);
                            StatusCode::OK
                        }
                    }),
                ),
                prefix: "/protected-probe",
            }]
        }
    }

    fn stub_domain_context() -> DomainContext {
        stub_domain_context_with_conn(Arc::new(sea_orm::DatabaseConnection::default()))
    }

    fn stub_domain_context_with_conn(conn: Arc<sea_orm::DatabaseConnection>) -> DomainContext {
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
            conn,
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
        drop(jwt_secret);
        drop(supabase_url);
        drop(_env_lock);

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
    }

    #[test]
    fn compose_routers_traces_health_requests_without_changing_response() {
        let _env_lock = ENV_LOCK.lock().unwrap_or_else(|error| error.into_inner());
        let supabase_url = ScopedEnvVar::set("SUPABASE_URL", "http://localhost:8000");
        let jwt_secret = ScopedEnvVar::set("SUPABASE_JWT_SECRET", "test-secret");
        let captured = CapturedHttpSpans::default();
        let subscriber = tracing_subscriber::registry()
            .with(tracing::level_filters::LevelFilter::TRACE)
            .with(captured.clone());

        let ctx = stub_domain_context();
        let app = compose_routers(&[], &ctx).unwrap().with_state(ctx);
        drop(jwt_secret);
        drop(supabase_url);
        drop(_env_lock);
        let response = tracing::subscriber::with_default(subscriber, || {
            tokio::runtime::Builder::new_current_thread()
                .enable_all()
                .build()
                .unwrap()
                .block_on(
                    app.oneshot(
                        Request::builder()
                            .uri("/health")
                            .body(Body::empty())
                            .unwrap(),
                    ),
                )
                .unwrap()
        });

        assert_eq!(response.status(), StatusCode::OK);
        assert!(captured.has_request("/health", StatusCode::OK));
    }

    #[test]
    fn compose_routers_traces_rejected_protected_requests_without_calling_handler() {
        let _env_lock = ENV_LOCK.lock().unwrap_or_else(|error| error.into_inner());
        let supabase_url = ScopedEnvVar::set("SUPABASE_URL", "http://localhost:8000");
        let jwt_secret = ScopedEnvVar::set("SUPABASE_JWT_SECRET", "test-secret");
        let captured = CapturedHttpSpans::default();
        let subscriber = tracing_subscriber::registry()
            .with(tracing::level_filters::LevelFilter::TRACE)
            .with(captured.clone());
        let handler_calls = Arc::new(std::sync::atomic::AtomicUsize::new(0));

        let ctx = stub_domain_context();
        let services: Vec<Box<dyn DomainService>> = vec![Box::new(ProtectedProbeService {
            handler_calls: handler_calls.clone(),
        })];
        let app = compose_routers(&services, &ctx).unwrap().with_state(ctx);
        drop(jwt_secret);
        drop(supabase_url);
        drop(_env_lock);
        let response = tracing::subscriber::with_default(subscriber, || {
            tokio::runtime::Builder::new_current_thread()
                .enable_all()
                .build()
                .unwrap()
                .block_on(
                    app.oneshot(
                        Request::builder()
                            .uri("/protected-probe")
                            .body(Body::empty())
                            .unwrap(),
                    ),
                )
                .unwrap()
        });

        assert_eq!(response.status(), StatusCode::UNAUTHORIZED);
        assert_eq!(handler_calls.load(std::sync::atomic::Ordering::SeqCst), 0);
        assert!(captured.has_request("/protected-probe", StatusCode::UNAUTHORIZED));
    }

    #[test]
    fn disabled_or_invalid_otlp_exporter_uses_text_logging_fallback() {
        let _env_lock = ENV_LOCK.lock().unwrap_or_else(|error| error.into_inner());

        for enabled in [Some("false"), None, Some("not-a-boolean")] {
            let _enabled = match enabled {
                Some(value) => ScopedEnvVar::set("ENABLED_OTLP_EXPORTER", value),
                None => ScopedEnvVar::unset("ENABLED_OTLP_EXPORTER"),
            };
            let mut text_logging_initialized = false;

            let guard = init_observability_with(domain_posts::observability::init, || {
                text_logging_initialized = true;
            });

            assert!(guard.is_none());
            assert!(text_logging_initialized);
        }
    }

    #[test]
    // Name kept from the five-service era so the test keeps its history; WU-10 adds
    // `domain-backup`, so it now asserts six entries.
    fn manifest_with_five_services_returns_five_entries() {
        let services = manifest(
            stub_media_config(),
            stub_user_state(),
            stub_backup_service(),
        );
        assert_eq!(services.len(), 6);
        let names: Vec<&str> = services.iter().map(|s| s.health().name).collect();
        assert!(names.contains(&"domain-posts"));
        assert!(names.contains(&"domain-auth"));
        assert!(names.contains(&"domain-media"));
        assert!(names.contains(&"domain-user"));
        assert!(names.contains(&"domain-seo"));
        assert!(names.contains(&"domain-backup"));
    }

    // ---------------------------------------------------------------------
    // WU-10: composed-router security for the backup routes (TS-03, TS-07,
    // TS-08b, TS-55), runtime start, README and migration-identity checks.
    // ---------------------------------------------------------------------

    const TEST_JWT_SECRET: &str = "wu10-test-secret-at-least-32-characters-long";
    const ADMIN_ROLE: &str = "my-headless-cms-administrator";
    const WRITER_ROLE: &str = "my-headless-cms-writer";
    const LOCAL_HOST_HEADER: &str = "127.0.0.1:8989";

    fn stub_backup_service() -> DomainBackupService {
        DomainBackupService::new(BackupConfigState::Disabled {
            reason: domain_backup::domain::config::DisabledReason::NotConfigured,
        })
    }

    /// Link-free, unique storage root (macOS `/var` is a symlink, which CFG-04 rejects).
    fn link_free_root(tag: &str) -> std::path::PathBuf {
        std::fs::canonicalize(std::env::temp_dir())
            .expect("resolve temp dir")
            .join(format!(
                "wu10-{tag}-{}-{}",
                std::process::id(),
                std::time::SystemTime::now()
                    .duration_since(std::time::UNIX_EPOCH)
                    .unwrap()
                    .as_nanos()
            ))
    }

    /// A complete, valid enabled environment (no `HOST`: the bind host is passed separately).
    fn backup_env(root: &std::path::Path) -> impl Fn(&str) -> Option<String> {
        let root = root.to_string_lossy().into_owned();
        move |key: &str| {
            Some(match key {
                "BACKUP_ENABLED" => "true".to_string(),
                "BACKUP_INSTANCE_ID" => "local-1".to_string(),
                "BACKUP_INSTANCE_LABEL" => "Local instance".to_string(),
                "BACKUP_STORAGE_ROOT" => root.clone(),
                "BACKUP_ALLOWED_ORIGINS" => "http://localhost:3002".to_string(),
                "BACKUP_REQUEST_TIMEOUT_SECS" => "30".to_string(),
                "BACKUP_MAX_HEADER_BYTES" => "16384".to_string(),
                _ => return None,
            })
        }
    }

    fn mint_token(secret: &str, roles: &[&str], exp_offset_secs: i64) -> String {
        use jsonwebtoken::{encode, Algorithm, EncodingKey, Header};
        let now = std::time::SystemTime::now()
            .duration_since(std::time::UNIX_EPOCH)
            .unwrap()
            .as_secs() as i64;
        let claims = serde_json::json!({
            "sub": "wu10-user",
            "email": "wu10@example.com",
            "aud": "authenticated",
            "role": "authenticated",
            "exp": now + exp_offset_secs,
            "iat": now,
            "app_metadata": { "roles": roles },
            "user_metadata": {},
        });
        encode(
            &Header::new(Algorithm::HS256),
            &claims,
            &EncodingKey::from_secret(secret.as_bytes()),
        )
        .expect("encode test token")
    }

    fn admin_token() -> String {
        mint_token(TEST_JWT_SECRET, &[ADMIN_ROLE], 3600)
    }

    /// Compose with the auth env the layers read at construction, then release the env.
    fn compose_for_test(
        services: &[Box<dyn DomainService>],
        ctx: &DomainContext,
        limits: IngressLimits,
    ) -> Router {
        let _env_lock = ENV_LOCK.lock().unwrap_or_else(|error| error.into_inner());
        let _url = ScopedEnvVar::set("SUPABASE_URL", "http://localhost:8000");
        let _secret = ScopedEnvVar::set("SUPABASE_JWT_SECRET", TEST_JWT_SECRET);
        let _port = ScopedEnvVar::set("PORT", "8989");
        compose_routers_with_limits(services, ctx, limits)
            .unwrap()
            .with_state(ctx.clone())
    }

    fn request(method: &str, uri: &str, headers: &[(&str, String)]) -> Request<Body> {
        let mut builder = Request::builder()
            .method(method)
            .uri(uri)
            .header("host", LOCAL_HOST_HEADER);
        for (name, value) in headers {
            builder = builder.header(*name, value);
        }
        builder.body(Body::empty()).unwrap()
    }

    fn bearer(token: &str) -> (&'static str, String) {
        ("authorization", format!("Bearer {token}"))
    }

    async fn body_text(response: axum::response::Response) -> String {
        let bytes = axum::body::to_bytes(response.into_body(), usize::MAX)
            .await
            .unwrap();
        String::from_utf8_lossy(&bytes).into_owned()
    }

    fn short_limits(timeout_ms: u64, max_header_bytes: usize) -> IngressLimits {
        IngressLimits {
            request_timeout: Duration::from_millis(timeout_ms),
            max_header_bytes,
        }
    }

    type CallCounter = Arc<std::sync::atomic::AtomicUsize>;
    type TokenCase<'a> = (&'a str, Vec<(&'a str, String)>, StatusCode);

    /// Registers one counting GET/POST route per `(mount, path)`; the handler can be slow.
    struct ProbeService {
        routes: Vec<(Mount, &'static str)>,
        calls: CallCounter,
        delay: Duration,
    }

    impl ProbeService {
        fn boxed(
            routes: Vec<(Mount, &'static str)>,
            delay: Duration,
        ) -> (Box<dyn DomainService>, CallCounter) {
            let calls: CallCounter = Arc::new(std::sync::atomic::AtomicUsize::new(0));
            (
                Box::new(Self {
                    routes,
                    calls: calls.clone(),
                    delay,
                }),
                calls,
            )
        }
    }

    impl DomainService for ProbeService {
        fn health(&self) -> HealthDescriptor {
            HealthDescriptor {
                name: "wu10-probe",
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
            self.routes
                .iter()
                .map(|(mount, path)| {
                    let calls = self.calls.clone();
                    let delay = self.delay;
                    let handler = move || {
                        let calls = calls.clone();
                        async move {
                            calls.fetch_add(1, std::sync::atomic::Ordering::SeqCst);
                            if !delay.is_zero() {
                                tokio::time::sleep(delay).await;
                            }
                            (StatusCode::OK, r#"{"instances":[{"id":"local-1"}]}"#)
                        }
                    };
                    RouteRegistration {
                        mount: *mount,
                        router: Router::new().route(path, get(handler.clone()).post(handler)),
                        prefix: "/wu10-probe",
                    }
                })
                .collect()
        }
    }

    fn backup_probe(delay: Duration) -> (Vec<Box<dyn DomainService>>, CallCounter) {
        let (service, calls) = ProbeService::boxed(
            vec![
                (Mount::AdministratorIngress, "/backup/instances"),
                (Mount::AdministratorIngress, "/backup/jobs"),
                (Mount::AdministratorIngress, "/backup/jobs/{job_id}"),
            ],
            delay,
        );
        (vec![service], calls)
    }

    fn calls_of(calls: &CallCounter) -> usize {
        calls.load(std::sync::atomic::Ordering::SeqCst)
    }

    // ---- TS-03 ----------------------------------------------------------

    /// TS-03 (AC-001 S2, AC-005 S1, T-01): missing, expired, forged, writer and non-admin
    /// tokens are denied before any handler runs, on the instance list and the job endpoints.
    #[tokio::test]
    async fn ts03_backup_routes_deny_unauthorized_tokens_before_handler() {
        let (services, calls) = backup_probe(Duration::ZERO);
        let ctx = stub_domain_context();
        let app = compose_for_test(&services, &ctx, IngressLimits::OWNER_APPROVED);

        let expired = mint_token(TEST_JWT_SECRET, &[ADMIN_ROLE], -3600);
        let forged = mint_token(
            "a-different-secret-that-is-long-enough!!",
            &[ADMIN_ROLE],
            3600,
        );
        let writer = mint_token(TEST_JWT_SECRET, &[WRITER_ROLE], 3600);
        let no_roles = mint_token(TEST_JWT_SECRET, &[], 3600);
        let cases: Vec<TokenCase> = vec![
            ("missing", vec![], StatusCode::UNAUTHORIZED),
            ("expired", vec![bearer(&expired)], StatusCode::UNAUTHORIZED),
            ("forged", vec![bearer(&forged)], StatusCode::UNAUTHORIZED),
            ("writer", vec![bearer(&writer)], StatusCode::FORBIDDEN),
            ("non-admin", vec![bearer(&no_roles)], StatusCode::FORBIDDEN),
        ];
        let targets = [
            ("GET", "/backup/instances"),
            ("GET", "/backup/jobs"),
            ("POST", "/backup/jobs"),
            ("GET", "/backup/jobs/7b6f3a0e-5c1d-4f5e-9a39-0d6f6a7c1b11"),
        ];
        for (method, uri) in targets {
            for (label, headers, expected) in &cases {
                let response = app
                    .clone()
                    .oneshot(request(method, uri, headers))
                    .await
                    .unwrap();
                assert_eq!(
                    response.status(),
                    *expected,
                    "{method} {uri} with {label} token"
                );
                let body = body_text(response).await;
                assert!(
                    !body.contains("instances") && !body.contains("local-1"),
                    "{method} {uri} {label}: instance list must be absent, got {body}"
                );
            }
        }
        assert_eq!(
            calls_of(&calls),
            0,
            "no handler may run for a denied request"
        );

        // The administrator token reaches the handler (control for the matrix above).
        let response = app
            .clone()
            .oneshot(request(
                "GET",
                "/backup/instances",
                &[bearer(&admin_token())],
            ))
            .await
            .unwrap();
        assert_eq!(response.status(), StatusCode::OK);
        assert_eq!(calls_of(&calls), 1);
    }

    // ---- TS-07 ----------------------------------------------------------

    /// TS-07 (AC-005 S1, T-04): no CORS layer on backup routes, so a preflight and a
    /// cross-origin read get no `access-control-*` headers.
    #[tokio::test]
    async fn ts07_backup_routes_carry_no_cors_headers() {
        let (services, _calls) = backup_probe(Duration::ZERO);
        let ctx = stub_domain_context();
        let app = compose_for_test(&services, &ctx, IngressLimits::OWNER_APPROVED);
        let origin = ("origin", "http://localhost:3002".to_string());

        let preflight = app
            .clone()
            .oneshot(request(
                "OPTIONS",
                "/backup/jobs",
                &[
                    origin.clone(),
                    ("access-control-request-method", "POST".to_string()),
                    (
                        "access-control-request-headers",
                        "authorization,content-type,idempotency-key".to_string(),
                    ),
                ],
            ))
            .await
            .unwrap();
        let read = app
            .clone()
            .oneshot(request(
                "GET",
                "/backup/instances",
                &[origin, bearer(&admin_token())],
            ))
            .await
            .unwrap();
        for (label, response) in [("preflight", preflight), ("read", read)] {
            let cors: Vec<_> = response
                .headers()
                .keys()
                .filter(|name| name.as_str().starts_with("access-control-"))
                .collect();
            assert!(cors.is_empty(), "{label}: unexpected CORS headers {cors:?}");
        }
    }

    /// TS-07 / RK-07 (AC-005 S1, T-04): a valid administrator token carried only in a
    /// cookie never authorizes a mutation; auth is bearer-only.
    #[tokio::test]
    async fn ts07_cookie_only_mutation_is_not_authorized() {
        let (services, calls) = backup_probe(Duration::ZERO);
        let ctx = stub_domain_context();
        let app = compose_for_test(&services, &ctx, IngressLimits::OWNER_APPROVED);
        let token = admin_token();

        for cookie_name in ["sb-access-token", "access_token", "token", "jwt"] {
            let response = app
                .clone()
                .oneshot(request(
                    "POST",
                    "/backup/jobs",
                    &[
                        ("cookie", format!("{cookie_name}={token}")),
                        ("content-type", "application/json".to_string()),
                        (
                            "idempotency-key",
                            "11111111-1111-4111-8111-111111111111".to_string(),
                        ),
                    ],
                ))
                .await
                .unwrap();
            assert_eq!(
                response.status(),
                StatusCode::UNAUTHORIZED,
                "cookie {cookie_name} must not authorize POST /backup/jobs"
            );
        }
        assert_eq!(calls_of(&calls), 0);
    }

    // ---- TS-08b ---------------------------------------------------------

    /// TS-08b (AC-005 S1, T-04, F-39): the bind host the gateway passes decides the state.
    #[test]
    fn ts08b_non_loopback_bind_forces_backup_disabled() {
        let root = link_free_root("ts08b");
        for host in [
            "0.0.0.0",
            "::",
            "10.0.0.5",
            "192.168.1.20",
            "example.com",
            "localhost",
            "",
        ] {
            let state = build_backup_config(host, backup_env(&root));
            assert_eq!(
                state.disabled_reason(),
                Some("NON_LOOPBACK_BIND"),
                "bind host {host:?} must disable backup"
            );
        }
        for host in ["127.0.0.1", "127.1.2.3", "::1"] {
            let state = build_backup_config(host, backup_env(&root));
            assert!(
                matches!(state, BackupConfigState::Enabled(_)),
                "loopback bind host {host:?} must enable backup, got {state:?}"
            );
        }
    }

    /// TS-08b: the effective bind value wins over any `HOST` the lookup would report, and
    /// the effective value defaults to loopback exactly as the listener does.
    #[test]
    fn ts08b_effective_bind_host_is_what_the_config_sees() {
        let root = link_free_root("ts08b-effective");
        let env = backup_env(&root);
        let lookup_says_loopback = |key: &str| {
            if key == "HOST" {
                Some("127.0.0.1".to_string())
            } else {
                env(key)
            }
        };
        assert_eq!(
            build_backup_config("0.0.0.0", lookup_says_loopback).disabled_reason(),
            Some("NON_LOOPBACK_BIND")
        );
        assert_eq!(effective_bind_host(|_| None), "127.0.0.1");
        assert_eq!(
            effective_bind_host(|key| (key == "HOST").then(|| "10.1.2.3".to_string())),
            "10.1.2.3"
        );
        assert_eq!(
            build_backup_config(&effective_bind_host(|_| None), backup_env(&root))
                .disabled_reason(),
            None
        );
    }

    // ---- TS-55 ----------------------------------------------------------

    /// TS-55 (AC-002 S1, T-13): oversized headers get 431 (not 401) before auth, never
    /// reach the handler, and the body is the backup crate error body.
    #[tokio::test]
    async fn ts55_oversized_headers_get_431_before_auth_and_never_reach_handler() {
        let (services, calls) = backup_probe(Duration::ZERO);
        let ctx = stub_domain_context();
        let app = compose_for_test(&services, &ctx, short_limits(5_000, 1024));
        let padding = ("x-padding", "a".repeat(2000));

        for (method, uri) in [("GET", "/backup/instances"), ("POST", "/backup/jobs")] {
            let response = app
                .clone()
                .oneshot(request(method, uri, std::slice::from_ref(&padding)))
                .await
                .unwrap();
            assert_eq!(
                response.status(),
                StatusCode::REQUEST_HEADER_FIELDS_TOO_LARGE,
                "{method} {uri} unauthenticated with oversized headers"
            );
            let body: serde_json::Value =
                serde_json::from_str(&body_text(response).await).expect("JSON error body");
            assert_eq!(body["error_code"], "HeaderTooLarge");
            assert_eq!(body["reason_code"], "HEADER_TOO_LARGE");
            assert!(body["correlation_id"]
                .as_str()
                .is_some_and(|id| !id.is_empty()));
            assert!(body["errors"].as_array().is_some_and(|e| e.len() == 1));
        }
        // An authenticated request over the limit is refused the same way.
        let response = app
            .clone()
            .oneshot(request(
                "GET",
                "/backup/instances",
                &[bearer(&admin_token()), padding],
            ))
            .await
            .unwrap();
        assert_eq!(
            response.status(),
            StatusCode::REQUEST_HEADER_FIELDS_TOO_LARGE
        );
        assert_eq!(calls_of(&calls), 0);
    }

    /// TS-55: the header budget is exact. At the limit the request goes on to auth (401);
    /// one byte over is 431.
    #[tokio::test]
    async fn ts55_header_limit_boundary_is_exact() {
        let (services, _calls) = backup_probe(Duration::ZERO);
        let ctx = stub_domain_context();
        // Header name + value bytes of the request without padding.
        let base: usize = LOCAL_HOST_HEADER.len() + "host".len();
        let limit = base + "x-padding".len() + 100;
        let app = compose_for_test(&services, &ctx, short_limits(5_000, limit));

        let at_limit = app
            .clone()
            .oneshot(request(
                "GET",
                "/backup/instances",
                &[("x-padding", "a".repeat(100))],
            ))
            .await
            .unwrap();
        assert_eq!(
            at_limit.status(),
            StatusCode::UNAUTHORIZED,
            "exactly at the limit"
        );
        let over = app
            .clone()
            .oneshot(request(
                "GET",
                "/backup/instances",
                &[("x-padding", "a".repeat(101))],
            ))
            .await
            .unwrap();
        assert_eq!(
            over.status(),
            StatusCode::REQUEST_HEADER_FIELDS_TOO_LARGE,
            "one byte over the limit"
        );
    }

    /// TS-55: an unauthenticated request within the limits gets 401 from the auth layer.
    #[tokio::test]
    async fn ts55_unauthenticated_request_within_limits_gets_401() {
        let (services, calls) = backup_probe(Duration::ZERO);
        let ctx = stub_domain_context();
        let app = compose_for_test(&services, &ctx, IngressLimits::OWNER_APPROVED);
        let response = app
            .oneshot(request("GET", "/backup/instances", &[]))
            .await
            .unwrap();
        assert_eq!(response.status(), StatusCode::UNAUTHORIZED);
        assert_eq!(calls_of(&calls), 0);
    }

    /// TS-55 (F-38): a slow authenticated request past CFG-17 gets 408 with the crate error
    /// body, and the handler's work is cancelled.
    #[tokio::test]
    async fn ts55_slow_authenticated_request_gets_408_with_crate_error_body() {
        let (services, calls) = backup_probe(Duration::from_millis(1500));
        let ctx = stub_domain_context();
        let app = compose_for_test(&services, &ctx, short_limits(150, 16384));
        let started = std::time::Instant::now();

        let response = app
            .oneshot(request(
                "GET",
                "/backup/instances",
                &[bearer(&admin_token())],
            ))
            .await
            .unwrap();

        assert_eq!(response.status(), StatusCode::REQUEST_TIMEOUT);
        assert!(
            started.elapsed() < Duration::from_millis(1200),
            "the timeout must answer before the slow handler finishes, took {:?}",
            started.elapsed()
        );
        let body: serde_json::Value =
            serde_json::from_str(&body_text(response).await).expect("JSON error body");
        assert_eq!(body["error_code"], "RequestTimeout");
        assert_eq!(body["reason_code"], "REQUEST_TIMEOUT");
        assert!(body["correlation_id"]
            .as_str()
            .is_some_and(|id| !id.is_empty()));
        assert_eq!(
            calls_of(&calls),
            1,
            "the handler was entered and then cut off"
        );
    }

    /// TS-55: an authenticated request inside both limits reaches the handler.
    #[tokio::test]
    async fn ts55_authenticated_request_within_limits_reaches_handler() {
        let (services, calls) = backup_probe(Duration::from_millis(20));
        let ctx = stub_domain_context();
        let app = compose_for_test(&services, &ctx, short_limits(2_000, 16384));
        let response = app
            .oneshot(request(
                "GET",
                "/backup/instances",
                &[bearer(&admin_token())],
            ))
            .await
            .unwrap();
        assert_eq!(response.status(), StatusCode::OK);
        assert_eq!(calls_of(&calls), 1);
    }

    /// TS-55 (T-13): public, protected and ordinary administrator routes keep their layering:
    /// no header limit, no timeout.
    #[tokio::test]
    async fn ts55_non_backup_routes_are_unaffected_by_ingress_layers() {
        let (service, calls) = ProbeService::boxed(
            vec![
                (Mount::Public, "/wu10/public"),
                (Mount::Protected, "/wu10/protected"),
                (Mount::Administrator, "/wu10/admin"),
            ],
            Duration::from_millis(400),
        );
        let services = vec![service];
        let ctx = stub_domain_context();
        let app = compose_for_test(&services, &ctx, short_limits(100, 512));
        let padding = ("x-padding", "a".repeat(2000));

        let public = app
            .clone()
            .oneshot(request(
                "GET",
                "/wu10/public",
                std::slice::from_ref(&padding),
            ))
            .await
            .unwrap();
        assert_eq!(
            public.status(),
            StatusCode::OK,
            "public: no header limit, no timeout"
        );

        let writer = mint_token(TEST_JWT_SECRET, &[WRITER_ROLE], 3600);
        let protected = app
            .clone()
            .oneshot(request(
                "GET",
                "/wu10/protected",
                &[bearer(&writer), padding.clone()],
            ))
            .await
            .unwrap();
        assert_eq!(protected.status(), StatusCode::OK, "protected: unaffected");

        let admin_unauth = app
            .clone()
            .oneshot(request(
                "GET",
                "/wu10/admin",
                std::slice::from_ref(&padding),
            ))
            .await
            .unwrap();
        assert_eq!(
            admin_unauth.status(),
            StatusCode::UNAUTHORIZED,
            "ordinary administrator route: auth decides, not the header limit"
        );
        let admin = app
            .clone()
            .oneshot(request(
                "GET",
                "/wu10/admin",
                &[bearer(&admin_token()), padding],
            ))
            .await
            .unwrap();
        assert_eq!(
            admin.status(),
            StatusCode::OK,
            "ordinary administrator: no timeout"
        );
        assert_eq!(calls_of(&calls), 3);
    }

    /// TS-55: the layers read CFG-17 and CFG-18 from the validated backup configuration.
    #[test]
    fn ts55_layers_read_cfg17_and_cfg18_from_backup_config() {
        let root = link_free_root("ts55-cfg");
        let env = backup_env(&root);
        let state = build_backup_config("127.0.0.1", |key| match key {
            "BACKUP_REQUEST_TIMEOUT_SECS" => Some("7".to_string()),
            "BACKUP_MAX_HEADER_BYTES" => Some("9000".to_string()),
            other => env(other),
        });
        assert!(matches!(state, BackupConfigState::Enabled(_)), "{state:?}");
        assert_eq!(
            IngressLimits::from_backup_state(&state),
            IngressLimits {
                request_timeout: Duration::from_secs(7),
                max_header_bytes: 9000,
            }
        );
        // A disabled state still wraps the routes, with the owner-approved values (30 s, 16384).
        let disabled = build_backup_config("0.0.0.0", backup_env(&root));
        assert_eq!(
            IngressLimits::from_backup_state(&disabled),
            IngressLimits {
                request_timeout: Duration::from_secs(30),
                max_header_bytes: 16384,
            }
        );
    }

    // ---- F-37 / F-38 wiring --------------------------------------------

    /// F-38: the backup service registers under `Mount::AdministratorIngress`.
    #[test]
    fn backup_service_registers_under_administrator_ingress() {
        let registrations = stub_backup_service().register_routes(&stub_domain_context());
        assert!(!registrations.is_empty());
        for registration in registrations {
            assert_eq!(registration.mount, Mount::AdministratorIngress);
            assert_eq!(registration.prefix, "/backup");
        }
    }

    /// F-37: a start-up that cannot start the runtime (here the namespace lock is held by
    /// another holder) never aborts the gateway; the API answers 503 `BACKUP_NOT_READY`.
    #[tokio::test]
    async fn runtime_start_failure_leaves_backup_not_ready_without_aborting_gateway() {
        use domain_backup::infra::fs_namespace_lock::FsNamespaceLock;
        use domain_backup::ports::namespace_lock::NamespaceLock;

        let root = link_free_root("lock-held");
        let state = build_backup_config("127.0.0.1", backup_env(&root));
        assert!(matches!(state, BackupConfigState::Enabled(_)), "{state:?}");
        let _held = FsNamespaceLock::try_acquire(&root).expect("hold the namespace lock");

        let backup = DomainBackupService::new(state);
        let services: Vec<Box<dyn DomainService>> = vec![Box::new(backup.clone())];
        let ctx = stub_domain_context();
        for service in &services {
            service
                .startup_health(&ctx)
                .await
                .expect("a failed runtime start must not fail startup_health");
        }
        let app = compose_for_test(&services, &ctx, IngressLimits::OWNER_APPROVED);
        let response = app
            .oneshot(request(
                "GET",
                "/backup/instances",
                &[bearer(&admin_token())],
            ))
            .await
            .unwrap();
        assert_eq!(response.status(), StatusCode::SERVICE_UNAVAILABLE);
        let body: serde_json::Value =
            serde_json::from_str(&body_text(response).await).expect("JSON error body");
        assert_eq!(body["reason_code"], "BACKUP_NOT_READY");
        let _ = std::fs::remove_dir_all(&root);
    }

    /// F-37: a disabled backup never builds or starts a runtime.
    #[tokio::test]
    async fn disabled_backup_does_not_start_a_runtime() {
        let backup = stub_backup_service();
        backup
            .startup_health(&stub_domain_context())
            .await
            .expect("startup_health");
        assert!(backup.runtime().is_none());
    }

    /// F-37, RK-15 (backup part): migrations through the gateway orchestrator use the backup
    /// migrator's own table, then the runtime starts after them and the composed API is
    /// ready. Posts and SEO are deliberately not booted here (descoped residual, see
    /// known-limitations).
    #[tokio::test]
    #[ignore = "requires Docker for PostgreSQL testcontainer"]
    async fn backup_runtime_starts_after_migrations_and_composed_api_is_ready() {
        use domain_backup::migrations::BACKUP_MIGRATION_IDS;
        use test_helpers::setup_test_space_with_pgvector;

        let space = setup_test_space_with_pgvector().await;
        let conn = crate::migration_tests::connect_untouched(&space).await;
        let root = link_free_root("runtime-start");
        let state = build_backup_config("127.0.0.1", backup_env(&root));
        assert!(matches!(state, BackupConfigState::Enabled(_)), "{state:?}");
        let backup = DomainBackupService::new(state);
        let services: Vec<Box<dyn DomainService>> = vec![Box::new(backup.clone())];

        run_orchestrator(&services, &conn)
            .await
            .expect("backup migrations apply through the orchestrator");
        let mut expected: Vec<String> =
            BACKUP_MIGRATION_IDS.iter().map(|s| s.to_string()).collect();
        expected.sort();
        assert_eq!(
            crate::migration_tests::recorded_versions(&conn, "seaql_migrations_backup").await,
            expected,
            "backup versions are recorded in the backup migration table"
        );

        let ctx = stub_domain_context_with_conn(Arc::new(conn.clone()));
        for service in &services {
            service.startup_health(&ctx).await.expect("startup_health");
        }
        assert!(
            backup.runtime().is_some(),
            "enabled backup builds a runtime"
        );
        let app = compose_for_test(&services, &ctx, IngressLimits::OWNER_APPROVED);

        let status = app
            .clone()
            .oneshot(request("GET", "/backup/status", &[bearer(&admin_token())]))
            .await
            .unwrap();
        assert_eq!(status.status(), StatusCode::OK);
        let status: serde_json::Value =
            serde_json::from_str(&body_text(status).await).expect("status JSON");
        assert_eq!(status["state"], "ready", "{status}");

        let instances = app
            .clone()
            .oneshot(request(
                "GET",
                "/backup/instances",
                &[bearer(&admin_token())],
            ))
            .await
            .unwrap();
        assert_eq!(
            instances.status(),
            StatusCode::OK,
            "runtime handed to the API state"
        );

        // TS-07: content type is required on the composed router (415, crate error body).
        let wrong_type = app
            .clone()
            .oneshot(request(
                "POST",
                "/backup/jobs",
                &[
                    bearer(&admin_token()),
                    ("content-type", "text/plain".to_string()),
                    (
                        "idempotency-key",
                        "22222222-2222-4222-8222-222222222222".to_string(),
                    ),
                ],
            ))
            .await
            .unwrap();
        assert_eq!(wrong_type.status(), StatusCode::UNSUPPORTED_MEDIA_TYPE);
        let body: serde_json::Value =
            serde_json::from_str(&body_text(wrong_type).await).expect("JSON error body");
        assert_eq!(body["reason_code"], "UNSUPPORTED_MEDIA_TYPE");
        let missing_type = app
            .clone()
            .oneshot(request(
                "POST",
                "/backup/jobs",
                &[
                    bearer(&admin_token()),
                    (
                        "idempotency-key",
                        "33333333-3333-4333-8333-333333333333".to_string(),
                    ),
                ],
            ))
            .await
            .unwrap();
        assert_eq!(missing_type.status(), StatusCode::UNSUPPORTED_MEDIA_TYPE);
        let _ = std::fs::remove_dir_all(&root);
    }

    /// F-37: the scheduler step runs a tick on a started runtime and, after an outage closed
    /// admissions, drives recovery so admissions reopen.
    #[tokio::test]
    #[ignore = "requires Docker for PostgreSQL testcontainer"]
    async fn scheduler_step_ticks_and_recovers_after_outage() {
        use test_helpers::setup_test_space_with_pgvector;

        let space = setup_test_space_with_pgvector().await;
        let conn = crate::migration_tests::connect_untouched(&space).await;
        let root = link_free_root("scheduler");
        let backup = DomainBackupService::new(build_backup_config("127.0.0.1", backup_env(&root)));
        let services: Vec<Box<dyn DomainService>> = vec![Box::new(backup.clone())];
        run_orchestrator(&services, &conn)
            .await
            .expect("migrations");
        let ctx = stub_domain_context_with_conn(Arc::new(conn));
        backup.startup_health(&ctx).await.expect("startup_health");
        let runtime = backup.runtime().expect("runtime");
        assert!(runtime.admissions_open());

        backup.scheduler_step().await;
        assert!(
            runtime.admissions_open(),
            "a healthy tick keeps admissions open"
        );

        runtime.database_outage().await;
        assert!(!runtime.admissions_open());
        backup.scheduler_step().await;
        assert!(runtime.admissions_open(), "recovery reopens admissions");
        let _ = std::fs::remove_dir_all(&root);
    }

    // ---- Documentation and migration identities -------------------------

    const README: &str = include_str!("../../../../README.md");
    const CONFIG_SOURCE: &str = include_str!("../../domain_backup/src/domain/config.rs");

    fn backup_keys_in_config_source() -> Vec<String> {
        let production = CONFIG_SOURCE
            .split("#[cfg(test)]")
            .next()
            .expect("production part");
        let mut keys: Vec<String> = production
            .split("lookup(\"")
            .skip(1)
            .filter_map(|rest| rest.split('"').next())
            .filter(|key| key.starts_with("BACKUP_"))
            .map(str::to_string)
            .collect();
        keys.sort();
        keys.dedup();
        keys
    }

    fn readme_row(key: &str) -> Option<Vec<String>> {
        README
            .lines()
            .find(|line| line.trim_start().starts_with(&format!("| `{key}`")))
            .map(|line| {
                line.trim()
                    .trim_matches('|')
                    .split('|')
                    .map(|cell| cell.trim().to_string())
                    .collect()
            })
    }

    /// WU-10 Done when: the README env section lists every `BACKUP_*` key the backup config
    /// reads with its default or required value, and records the owner-approved CFG-17 and
    /// CFG-18 values.
    #[test]
    fn readme_env_section_lists_every_backup_key_with_default_or_required_value() {
        let keys = backup_keys_in_config_source();
        assert!(
            keys.len() >= 18,
            "expected CFG keys in config source, got {keys:?}"
        );
        for key in &keys {
            let row =
                readme_row(key).unwrap_or_else(|| panic!("README has no table row for `{key}`"));
            assert!(
                row.len() >= 2 && !row[1].is_empty(),
                "README row for `{key}` needs a default or required value, got {row:?}"
            );
        }
        let timeout = readme_row("BACKUP_REQUEST_TIMEOUT_SECS").expect("timeout row");
        assert!(timeout[1].contains("30"), "{timeout:?}");
        assert!(
            timeout.join(" ").to_lowercase().contains("owner-approved"),
            "{timeout:?}"
        );
        let header = readme_row("BACKUP_MAX_HEADER_BYTES").expect("header row");
        assert!(header[1].contains("16384"), "{header:?}");
        assert!(
            header.join(" ").to_lowercase().contains("owner-approved"),
            "{header:?}"
        );
        // The legacy CLI keys are documented as a different surface.
        for legacy in ["BACKUP_S3_ENDPOINT", "BACKUP_LOCAL_ROOT"] {
            assert!(
                README.contains(legacy),
                "README must mention legacy key {legacy}"
            );
        }
    }

    /// RK-08 / WU-10: backup identities are chained after the SEO identities in
    /// `migrate --list`.
    #[test]
    fn migrate_list_chains_backup_identities_after_seo() {
        let ids = crate::migrate_cli::all_migration_identities();
        let backup = domain_backup::migrations_cli::list_identities();
        assert!(!backup.is_empty());
        assert!(ids.ends_with(&backup), "backup ids must come last: {ids:?}");
        let seo = domain_seo::migrations_cli::list_identities();
        let seo_end = ids.len() - backup.len();
        assert!(
            ids[..seo_end].ends_with(&seo),
            "seo ids precede backup ids: {ids:?}"
        );
    }

    /// RK-08: `migrate down` and `status` reach only the posts migrator; the help says so.
    #[test]
    fn migrate_usage_documents_down_status_posts_only_limit() {
        let usage = crate::migrate_cli::USAGE;
        assert!(
            usage.contains("posts migrator only"),
            "USAGE must state that down/status cover the posts migrator only: {usage}"
        );
        assert!(usage.contains("seaql_migrations_backup"), "{usage}");
    }
}
