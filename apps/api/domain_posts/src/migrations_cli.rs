//! Per-domain migration CLI runner.
//!
//! `cargo run -p domain_posts -- migrate` runs the post-domain migrations
//! against the database referenced by `DATABASE_URL`. The runner wraps
//! `sea_orm_migration::MigratorTrait::up` so the migration identity,
//! ordering, and idempotency semantics match the legacy
//! `apps/api/migration/src/main.rs` behaviour.

use sea_orm_migration::MigratorTrait;

use crate::migrations::{Migrator, POST_MIGRATION_IDS};

/// Apply pending migrations against the given connection.
pub async fn run(conn: &sea_orm::DatabaseConnection) -> Result<(), sea_orm::DbErr> {
    Migrator::up(conn, None).await
}

/// Print the migration set identity in the original order. Used by tests
/// and by the `cargo run -p domain_posts -- migrate --list` CLI flag.
pub fn list_identities() -> Vec<&'static str> {
    POST_MIGRATION_IDS.to_vec()
}

/// CLI dispatcher invoked by `domain_posts/src/main.rs`. Supports
/// `--list` (print identities, exit 0) and the default behaviour (run
/// migrations against `DATABASE_URL`, exit 0 on success / 1 on failure).
pub async fn handle_args(args: &[String]) -> Result<(), String> {
    if args.iter().any(|a| a == "--list") {
        for id in list_identities() {
            println!("{}", id);
        }
        return Ok(());
    }

    let database_url =
        std::env::var("DATABASE_URL").map_err(|_| "DATABASE_URL must be set".to_string())?;
    let conn = sea_orm::Database::connect(&database_url)
        .await
        .map_err(|e| format!("Failed to connect to database: {}", e))?;

    run(&conn)
        .await
        .map_err(|e| format!("Migration failed: {}", e))
}
