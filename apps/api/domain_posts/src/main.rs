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

use domain_interface::{DomainContext, DomainService};
use domain_posts::service::DomainPostService;
use tracing::info;

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

    let app = domain_posts::api::routes(&ctx);

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

    // The routers returned by `register_routes` are bare; layers are applied
    // by the gateway in composed mode. In standalone mode the binary is the
    // listener, so it is responsible for applying the cross-cutting layers.
    // Task 7 final wiring will compose the public/protected/administrator
    // sub-routers from `app` and attach `cors_layer`, `cookie_layer`,
    // `body_limit_layer`, and `otel_layers` here.
    let _ = (layers::otel_layers(), layers::cors_layer());

    info!("ready");
    if let Err(e) = axum::serve(listener, axum::Router::new()).await {
        eprintln!("serve error: {}", e);
        return ExitCode::FAILURE;
    }

    ExitCode::SUCCESS
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
