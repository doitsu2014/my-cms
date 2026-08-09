//! `DomainPostService` — `domain_interface::DomainService` implementation for
//! the post domain.
//!
//! See `openspec/changes/refactor-api-into-pluggable-domain-libraries/design.md`
//! for the architectural context. The service is constructed once by either
//! the standalone `domain_posts` bin or the `gateway` composition root and
//! queried through the `DomainService` trait for routing, migrations, and
//! health.

use std::sync::Arc;

use async_trait::async_trait;
use domain_interface::{
    DomainConfigError, DomainContext, DomainService, HealthDescriptor, MigrationDescriptor,
    RouteRegistration,
};
use sea_orm::ConnectionTrait;

/// `DomainPostService` — the post-domain service registered into the gateway.
///
/// Constructed once with the dependencies it needs (database, env) and then
/// queried through the `DomainService` trait for routes, migrations, and
/// health. The standalone `domain_posts` bin uses the same struct.
#[derive(Debug, Clone, Default)]
pub struct DomainPostService;

impl DomainPostService {
    /// Build a `DomainPostService`. The constructor accepts the same
    /// environment surface the legacy `my-cms-api` bootstrap read.
    pub fn new() -> Self {
        Self
    }
}

#[async_trait]
impl DomainService for DomainPostService {
    fn health(&self) -> HealthDescriptor {
        HealthDescriptor {
            name: "domain-posts",
            version: env!("CARGO_PKG_VERSION"),
        }
    }

    fn required_env(&self) -> &'static [&'static str] {
        &[
            "DATABASE_URL",
            "SUPABASE_URL",
            "SUPABASE_JWT_SECRET",
            "OPENAI_API_KEY",
        ]
    }

    fn validate_config(&self) -> Result<(), DomainConfigError> {
        for var in self.required_env() {
            if std::env::var(var).is_err() {
                return Err(DomainConfigError::MissingEnv(var));
            }
        }
        Ok(())
    }

    fn migrations(&self) -> Vec<MigrationDescriptor> {
        crate::migrations::migration_descriptors()
    }

    fn register_routes(&self, ctx: &DomainContext) -> Vec<RouteRegistration> {
        crate::api::routes(ctx)
    }

    async fn startup_health(&self, ctx: &DomainContext) -> Result<(), DomainConfigError> {
        let conn: &Arc<sea_orm::DatabaseConnection> = &ctx.conn;
        conn.execute_unprepared("SELECT 1").await.map_err(|e| {
            DomainConfigError::StartupHealth(format!("posts table probe failed: {}", e))
        })?;
        Ok(())
    }

    async fn run_migrations(
        &self,
        conn: &sea_orm::DatabaseConnection,
        _descriptors: &[MigrationDescriptor],
    ) -> Result<(), DomainConfigError> {
        crate::migrations_cli::run(conn)
            .await
            .map_err(|e| DomainConfigError::MigrationExecution(format!("domain_posts: {}", e)))
    }
}
