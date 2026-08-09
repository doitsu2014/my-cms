//! `migrate_cli` — `my-cms-api migrate <verb>` operator CLI.
//!
//! Dispatches the `migrate` subcommand without binding the HTTP listener.
//! Routes:
//! - `up` (default for the migrate CLI): connects, builds the manifest, runs
//!   the gateway's `run_orchestrator` against the registered domains.
//! - `down` / `status` / `--list` / `--help`: thin forward to
//!   `domain_posts::migrations_cli::handle_args`.
//! - unknown verb: prints usage to stderr, exits with `ExitCode::FAILURE`.

use std::process::ExitCode;

/// Operator-facing help banner. Mirrors the migration CLI's usage string.
const USAGE: &str = "Usage: my-cms-api migrate <verb>
\n\
Verbs:\n  \
  up       Apply pending migrations via the gateway orchestrator (default).\n  \
  down     Revert the most recent migration batch.\n  \
  status   Print applied / pending status per migration.\n  \
  --list   Print migration identities in original order.\n  \
  --help   Print this help and exit 0.\n\
\n\
Environment:\n  \
  DATABASE_URL, SUPABASE_URL, SUPABASE_SERVICE_ROLE_KEY, MEDIA_BUCKET,\n  \
  MEDIA_BASE_URL — required by the gateway composition root.";

/// Entry point. Parses `args` (already stripped of the leading `migrate`
/// token by `main`) and returns the process exit code.
pub async fn handle_args(args: &[String]) -> ExitCode {
    let verb = args.first().map(|s| s.as_str());
    match verb {
        Some("--help") | Some("-h") | None => {
            println!("{}", USAGE);
            ExitCode::SUCCESS
        }
        Some("--list") => match forward_to_domain_posts(args).await {
            Ok(()) => ExitCode::SUCCESS,
            Err(e) => {
                eprintln!("migrate --list failed: {}", e);
                ExitCode::FAILURE
            }
        },
        Some("down") | Some("status") => match forward_to_domain_posts(args).await {
            Ok(()) => ExitCode::SUCCESS,
            Err(e) => {
                eprintln!("migrate {} failed: {}", verb.unwrap_or("?"), e);
                ExitCode::FAILURE
            }
        },
        Some("up") => match run_up_orchestrator().await {
            Ok(()) => ExitCode::SUCCESS,
            Err(e) => {
                eprintln!("migrate up failed: {}", e);
                ExitCode::FAILURE
            }
        },
        Some(other) => {
            eprintln!("unknown migrate verb: '{}'\n\n{}", other, USAGE);
            ExitCode::FAILURE
        }
    }
}

/// Forward to `domain_posts::migrations_cli::handle_args` for verbs that
/// only touch the post-domain migrator.
async fn forward_to_domain_posts(args: &[String]) -> Result<(), String> {
    domain_posts::migrations_cli::handle_args(args).await
}

/// Build the manifest, connect to the database, run the orchestrator.
///
/// The `MediaConfig` + `UserApiState` are required by the manifest signature
/// (added in Slice 1). The orchestrator only calls `run_migrations` on
/// `DomainPostService`; the other domains' default `Ok(())` no-ops are
/// skipped via the `descriptors.is_empty()` guard.
async fn run_up_orchestrator() -> Result<(), String> {
    use std::sync::Arc;

    let media_config = {
        let cfg = crate::build_media_config().map_err(|e| format!("media config: {}", e))?;
        Arc::new(cfg)
    };
    let user_state = crate::build_user_state().map_err(|e| format!("user state: {}", e))?;

    let services = crate::manifest(media_config, user_state);
    let conn = crate::connect_database()
        .await
        .map_err(|e| format!("database: {}", e))?;

    // Log each domain that owns migrations for operator visibility.
    for svc in &services {
        let descs = svc.migrations();
        if !descs.is_empty() {
            tracing::info!(
                "migrate up: dispatching {} migration(s) for {}",
                descs.len(),
                svc.health().name
            );
        }
    }

    crate::run_orchestrator(&services, conn.as_ref())
        .await
        .map_err(|e| format!("orchestrator: {}", e))
}

#[cfg(test)]
mod tests {
    use super::*;

    #[tokio::test]
    async fn handle_args_help_exits_zero() {
        let args: Vec<String> = vec![];
        let code = handle_args(&args).await;
        assert_eq!(code, ExitCode::SUCCESS);
    }

    #[tokio::test]
    async fn handle_args_help_dash_dash_exits_zero() {
        let args = vec!["--help".to_string()];
        let code = handle_args(&args).await;
        assert_eq!(code, ExitCode::SUCCESS);
    }

    #[tokio::test]
    async fn handle_args_unknown_verb_exits_one() {
        let args = vec!["bogus".to_string()];
        let code = handle_args(&args).await;
        assert_eq!(code, ExitCode::FAILURE);
    }

    #[tokio::test]
    async fn handle_args_up_missing_database_url_exits_one() {
        // Ensure DATABASE_URL is unset so the up path fails fast.
        let previous = std::env::var("DATABASE_URL").ok();
        std::env::remove_var("DATABASE_URL");
        let args = vec!["up".to_string()];
        let code = handle_args(&args).await;
        // Restore
        match previous {
            Some(v) => std::env::set_var("DATABASE_URL", v),
            None => {}
        }
        assert_eq!(code, ExitCode::FAILURE);
    }

    #[tokio::test]
    async fn handle_args_list_succeeds_even_without_database_url() {
        // --list must not connect to the database; it just prints the
        // migration identities from the in-memory POST_MIGRATION_IDS table.
        let args = vec!["--list".to_string()];
        let code = handle_args(&args).await;
        assert_eq!(code, ExitCode::SUCCESS);
    }
}
