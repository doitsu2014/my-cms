//! `domain_posts` standalone binary.
//!
//! Boots a full Axum server with the post domain's HTTP routers, the
//! Supabase auth/CORS/cookie/body-limit/Otel layers, a shared database
//! connection, and the GraphQL endpoints. Equivalent to the gateway
//! composition but limited to the post domain.
//!
//! See `openspec/changes/refactor-api-into-pluggable-domain-libraries/design.md`
//! Decision 3 for the standalone-vs-composed deployment modes.

use std::env;
use std::process::ExitCode;
use std::sync::Arc;

use domain_auth::factory::auth_layer_from_env;
use domain_interface::{DomainContext, DomainService, Mount, RouteRegistration};
use domain_posts::service::DomainPostService;
use tracing::info;

use domain_posts::api;
use domain_posts::domain::graphql::contribute_post_schema;
use domain_posts::domain::layers;
use domain_posts::domain::postgres::connect_database;
use domain_posts::observability;

#[tokio::main]
async fn main() -> ExitCode {
    let args: Vec<String> = std::env::args().skip(1).collect();
    if args.first().map(|s| s.as_str()) == Some("migrate") {
        return run_migrate_cli(&args[1..]).await;
    }

    let _ = dotenv::dotenv();
    init_observability();

    let conn = match connect_database().await {
        Ok(c) => c,
        Err(e) => {
            eprintln!("database connection failed: {}", e);
            return ExitCode::FAILURE;
        }
    };

    let graphql_immutable = match contribute_post_schema(conn.as_ref().clone(), None, None, false) {
        Ok(s) => Arc::new(s),
        Err(e) => {
            eprintln!("immutable schema build failed: {}", e);
            return ExitCode::FAILURE;
        }
    };
    let graphql_mutable = match contribute_post_schema(conn.as_ref().clone(), None, None, true) {
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

    let service = DomainPostService::new();
    if let Err(e) = service.validate_config() {
        eprintln!("config validation failed: {}", e);
        return ExitCode::FAILURE;
    }
    if let Err(e) = service.startup_health(&ctx).await {
        eprintln!("startup health failed: {}", e);
        return ExitCode::FAILURE;
    }

    let app = match build_app(&ctx) {
        Ok(a) => a,
        Err(e) => {
            eprintln!("auth layer construction failed: {}", e);
            return ExitCode::FAILURE;
        }
    };

    let host = env::var("HOST").unwrap_or("127.0.0.1".to_string());
    let port = env::var("PORT").unwrap_or("8989".to_string());
    let host_port = format!("{}:{}", host, port);
    info!(
        "domain_posts standalone microservice listening on http://{}",
        host_port
    );

    let listener = match tokio::net::TcpListener::bind(&host_port).await {
        Ok(l) => l,
        Err(e) => {
            eprintln!("bind failed: {}", e);
            return ExitCode::FAILURE;
        }
    };

    info!("ready");
    let app = app.with_state(ctx.clone());
    if let Err(e) = axum::serve(listener, app).await {
        eprintln!("serve error: {}", e);
        return ExitCode::FAILURE;
    }

    ExitCode::SUCCESS
}

/// Compose the standalone binary's `Router` from the post-domain
/// `RouteRegistration`s plus the cross-cutting layers. The mutable GraphQL
/// mount (`/posts/graphql/mutable`) and the other `Mount::Protected`
/// routes are gated by the Supabase auth layer with the role vector
/// `["my-headless-cms-writer", "my-headless-cms-administrator"]` — the
/// same role set the gateway composition applies at the new mount point.
/// See `openspec/changes/merge-graphql-into-posts-domain/design.md` Decision 6.
fn build_app(
    ctx: &DomainContext,
) -> Result<axum::Router<DomainContext>, domain_interface::DomainConfigError> {
    use axum::Router;

    let mut public: Router<DomainContext> = Router::new();
    let mut protected: Router<DomainContext> = Router::new();
    let mut administrator: Router<DomainContext> = Router::new();

    for RouteRegistration {
        mount,
        router,
        prefix: _,
    } in api::routes(ctx)
    {
        match mount {
            Mount::Public => public = public.merge(router),
            Mount::Protected => protected = protected.merge(router),
            Mount::Administrator => administrator = administrator.merge(router),
        }
    }

    let (otel_in_response, otel_axum) = layers::otel_layers();

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

    Ok(public
        .merge(protected.layer(protected_layer))
        .merge(administrator.layer(administrator_layer))
        .layer(layers::cookie_layer())
        .layer(layers::body_limit_layer())
        .layer(layers::cors_layer())
        .layer(otel_in_response)
        .layer(otel_axum))
}

async fn run_migrate_cli(args: &[String]) -> ExitCode {
    match domain_posts::migrations_cli::handle_args(args).await {
        Ok(()) => ExitCode::SUCCESS,
        Err(e) => {
            eprintln!("migration cli error: {}", e);
            ExitCode::FAILURE
        }
    }
}

fn init_observability() {
    let _ = observability::init();
    observability::init_text_logging();
}
