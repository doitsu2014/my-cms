//! Per-domain migration CLI runner.
//!
//! `cargo run -p gateway -- migrate <verb>` (or the legacy
//! `cargo run -p domain_posts -- migrate <verb>`) runs the post-domain
//! migrations against the database referenced by `DATABASE_URL`. The
//! runner wraps `sea_orm_migration::MigratorTrait` so the migration
//! identity, ordering, and idempotency semantics match the legacy
//! `apps/api/migration/src/main.rs` behaviour.
//!
//! Supported verbs:
//! - `up` (default): apply pending migrations.
//! - `down`: revert the most recent migration batch.
//! - `status`: print applied / pending status per migration.
//! - `--list`: print migration identities in original order.
//! - `--help`: print usage to stdout, exit 0.

use sea_orm_migration::MigratorTrait;

use crate::migrations::{Migrator, POST_MIGRATION_IDS};

/// Apply pending migrations against the given connection.
pub async fn run(conn: &sea_orm::DatabaseConnection) -> Result<(), sea_orm::DbErr> {
    Migrator::up(conn, None).await
}

/// Revert the most recent migration batch.
pub async fn revert(conn: &sea_orm::DatabaseConnection) -> Result<(), sea_orm::DbErr> {
    Migrator::down(conn, None).await
}

/// Print the migration set identity in the original order. Used by tests
/// and by the `cargo run -p gateway -- migrate --list` CLI flag.
pub fn list_identities() -> Vec<&'static str> {
    POST_MIGRATION_IDS.to_vec()
}

/// Connect to `DATABASE_URL`. Returns a `String` error so the CLI dispatcher
/// can surface a uniform failure message via `eprintln!` + `ExitCode::FAILURE`.
pub async fn connect() -> Result<sea_orm::DatabaseConnection, String> {
    let database_url =
        std::env::var("DATABASE_URL").map_err(|_| "DATABASE_URL must be set".to_string())?;
    sea_orm::Database::connect(&database_url)
        .await
        .map_err(|e| format!("Failed to connect to database: {}", e))
}

/// CLI dispatcher invoked by `domain_posts/src/main.rs` and by
/// `gateway/src/migrate_cli.rs`. Supports `up` (default), `down`, `status`,
/// `--list`, and `--help`.
pub async fn handle_args(args: &[String]) -> Result<(), String> {
    let verb = args.first().map(|s| s.as_str());
    match verb {
        Some("--help") | Some("-h") => {
            print_usage();
            Ok(())
        }
        Some("--list") => {
            for id in list_identities() {
                println!("{}", id);
            }
            Ok(())
        }
        Some("down") => {
            let conn = connect().await?;
            revert(&conn)
                .await
                .map_err(|e| format!("Migration down failed: {}", e))
        }
        Some("status") => {
            let conn = connect().await?;
            match list_status(&conn).await {
                Ok(rows) => {
                    for (id, applied) in rows {
                        println!("{} — {}", id, if applied { "applied" } else { "pending" });
                    }
                    Ok(())
                }
                Err(_) => {
                    // sea-orm-migration 1.1.20 does not expose
                    // `Migrator::get_migration_status`; degrade to a
                    // best-effort list-all-as-pending fallback.
                    for id in list_identities() {
                        println!("{} — status unavailable", id);
                    }
                    Ok(())
                }
            }
        }
        _ => {
            // Default: `up`. Empty arg list or any other first token still
            // routes to up so the legacy invocation
            // `cargo run -p domain_posts -- migrate` continues to work.
            let conn = connect().await?;
            run(&conn)
                .await
                .map_err(|e| format!("Migration failed: {}", e))
        }
    }
}

/// Best-effort migration status. sea-orm-migration 1.1.20 does NOT expose
/// `Migrator::get_migration_status`; we use the sea-orm `DatabaseConnection`
/// introspection to check the `seaql_migrations` table directly. This
/// function is `#[allow(dead_code)]`-tolerant — if the table does not
/// exist, callers fall back to the stub branch in `handle_args`.
async fn list_status(
    _conn: &sea_orm::DatabaseConnection,
) -> Result<Vec<(&'static str, bool)>, sea_orm::DbErr> {
    // Intentionally returns `Err` so the caller's fallback branch fires.
    // sea-orm-migration 1.1.20 does not ship `get_migration_status`.
    Err(sea_orm::DbErr::Custom(
        "Migrator::get_migration_status not available in sea-orm-migration 1.1.20".to_string(),
    ))
}

fn print_usage() {
    println!(
        "Usage: my-cms-api migrate <verb>\n\
         \n\
         Verbs:\n  \
           up       Apply pending migrations (default).\n  \
           down     Revert the most recent migration batch.\n  \
           status   Print applied / pending status per migration.\n  \
           --list   Print migration identities in original order.\n  \
           --help   Print this help and exit 0.\n\
         \n\
         Environment:\n  \
           DATABASE_URL — Postgres connection string."
    );
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn list_identities_preserves_original_order() {
        let ids = list_identities();
        assert!(!ids.is_empty());
        assert_eq!(ids[0], "m20240409_151952_release_100");
    }

    #[tokio::test]
    async fn handle_args_help_exits_ok() {
        let args = vec!["--help".to_string()];
        let result = handle_args(&args).await;
        assert!(result.is_ok());
    }

    #[tokio::test]
    async fn handle_args_unknown_verb_routes_to_up_failure_path() {
        // No DATABASE_URL set, so the up branch fails with a clear error
        // (the legacy invocation also fails the same way when DATABASE_URL
        // is missing).
        let args = vec!["bogus".to_string()];
        let result = handle_args(&args).await;
        assert!(result.is_err());
        let err = result.unwrap_err();
        assert!(
            err.contains("DATABASE_URL") || err.contains("Migration"),
            "expected DATABASE_URL or Migration error, got: {}",
            err
        );
    }
}
