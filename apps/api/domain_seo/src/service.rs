use async_trait::async_trait;
use domain_interface::{
    DomainConfigError, DomainContext, DomainService, HealthDescriptor, MigrationDescriptor,
    RouteRegistration,
};
use sea_orm::ConnectionTrait;

#[derive(Debug, Clone, Default)]
pub struct DomainSeoService;
impl DomainSeoService {
    pub fn new() -> Self {
        Self
    }
}

#[async_trait]
impl DomainService for DomainSeoService {
    fn health(&self) -> HealthDescriptor {
        HealthDescriptor {
            name: "domain-seo",
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
        crate::api::routes(ctx)
    }
    async fn startup_health(&self, ctx: &DomainContext) -> Result<(), DomainConfigError> {
        ctx.conn.execute_unprepared("SELECT 1").await.map_err(|e| {
            DomainConfigError::StartupHealth(format!("seo table probe failed: {e}"))
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
            .map_err(|e| DomainConfigError::MigrationExecution(format!("domain_seo: {e}")))
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn service_exposes_stable_health_and_empty_config_contract() {
        let service = DomainSeoService::new();
        assert_eq!(service.health().name, "domain-seo");
        assert!(service.required_env().is_empty());
        assert!(service.validate_config().is_ok());
    }

    #[test]
    fn service_owns_one_migration() {
        let service = DomainSeoService::new();
        assert_eq!(service.migrations().len(), 1);
        assert_eq!(
            service.migrations()[0].id,
            "m20260822_000001_seo_head_assets"
        );
    }

    #[test]
    fn service_registers_public_and_administrator_route_boundaries() {
        let service = DomainSeoService::new();
        let query =
            async_graphql::dynamic::Object::new("Query").field(async_graphql::dynamic::Field::new(
                "health",
                async_graphql::dynamic::TypeRef::named(async_graphql::dynamic::TypeRef::STRING),
                |_| {
                    async_graphql::dynamic::FieldFuture::new(async {
                        Ok(Some(async_graphql::Value::from("ok")))
                    })
                },
            ));
        let schema = async_graphql::dynamic::Schema::build("Query", None, None)
            .register(query)
            .finish()
            .expect("schema");
        let context = DomainContext {
            conn: std::sync::Arc::new(sea_orm::DatabaseConnection::Disconnected),
            graphql_immutable: std::sync::Arc::new(schema.clone()),
            graphql_mutable: std::sync::Arc::new(schema),
        };
        let routes = service.register_routes(&context);
        assert_eq!(routes.len(), 2);
        assert!(routes.iter().any(
            |route| route.mount == domain_interface::Mount::Administrator
                && route.prefix == "/seo/head-assets"
        ));
        assert!(routes
            .iter()
            .any(|route| route.mount == domain_interface::Mount::Public
                && route.prefix == "/seo/head-assets/ducth-dev"));
    }
}
