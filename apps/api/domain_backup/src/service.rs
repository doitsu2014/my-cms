//! `DomainBackupService`: domain-owned config, migrations and administrator REST routes (FN-37).

use async_trait::async_trait;
use domain_interface::{
    DomainConfigError, DomainContext, DomainService, HealthDescriptor, MigrationDescriptor, Mount,
    RouteRegistration,
};
use sea_orm::DatabaseConnection;

use crate::domain::config::BackupConfig;
use crate::handlers::http::{routes, BackupApiState};

#[derive(Debug, Clone)]
pub struct DomainBackupService {
    config: crate::domain::config::BackupConfigState,
}

impl DomainBackupService {
    pub fn from_env() -> Self {
        Self {
            config: BackupConfig::from_env(),
        }
    }

    pub fn new(config: crate::domain::config::BackupConfigState) -> Self {
        Self { config }
    }

    pub fn config(&self) -> &crate::domain::config::BackupConfigState {
        &self.config
    }
}

#[async_trait]
impl DomainService for DomainBackupService {
    fn health(&self) -> HealthDescriptor {
        HealthDescriptor {
            name: "domain-backup",
            version: env!("CARGO_PKG_VERSION"),
        }
    }

    fn required_env(&self) -> &'static [&'static str] {
        &[]
    }

    fn validate_config(&self) -> Result<(), DomainConfigError> {
        Ok(())
    }

    fn migrations(&self) -> Vec<MigrationDescriptor> {
        crate::migrations::migration_descriptors()
    }

    fn register_routes(&self, ctx: &DomainContext) -> Vec<RouteRegistration> {
        vec![RouteRegistration {
            mount: Mount::Administrator,
            router: routes(BackupApiState::new(
                ctx.conn.as_ref().clone(),
                self.config.clone(),
            )),
            prefix: "/backup",
        }]
    }

    async fn startup_health(&self, _ctx: &DomainContext) -> Result<(), DomainConfigError> {
        Ok(())
    }

    async fn run_migrations(
        &self,
        conn: &DatabaseConnection,
        descriptors: &[MigrationDescriptor],
    ) -> Result<(), DomainConfigError> {
        if descriptors.is_empty() {
            return Ok(());
        }
        crate::migrations_cli::run(conn)
            .await
            .map_err(|error| DomainConfigError::MigrationExecution(error.to_string()))
    }
}
