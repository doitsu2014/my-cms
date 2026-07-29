//! Database connection helper. Mirrors the legacy
//! `apps/api/src/bin/my-cms-api.rs::construct_app_state` lines 257–301 for the
//! database connection portion. Supabase/GraphQL/MediaCache/etc. construction
//! is left to the gateway.

use std::sync::Arc;

use sea_orm::{Database, DatabaseConnection};

/// Open a single `DatabaseConnection` from the `DATABASE_URL` env var.
///
/// Errors are surfaced as `String` so the gateway can convert them to
/// `DomainConfigError::UnreachableDependency` without depending on the
/// post domain's error type.
pub async fn connect_database() -> Result<Arc<DatabaseConnection>, String> {
    let database_url = std::env::var("DATABASE_URL")
        .map_err(|_| "DATABASE_URL must be set".to_string())?;
    let conn = Database::connect(&database_url)
        .await
        .map_err(|e| format!("Failed to connect to database: {}", e))?;
    Ok(Arc::new(conn))
}