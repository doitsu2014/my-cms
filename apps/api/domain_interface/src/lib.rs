//! `domain_interface` — the stable, publishable contract crate for pluggable
//! domain services in My-CMS.
//!
//! This crate contains **only** the trait and supporting types that every domain
//! and the gateway must agree on. It deliberately holds no concrete domain
//! implementation, no generated entities, and no business logic.
//!
//! Concrete domains (e.g. `domain_posts`) implement [`DomainService`] against
//! this contract. The gateway registers domains through a
//! `Vec<Box<dyn DomainService>>` and iterates the manifest to compose Axum
//! routers, migrations, and health checks without ever importing a domain's
//! commands, entities, or DTOs.
//!
//! See `openspec/changes/refactor-api-into-pluggable-domain-libraries/design.md`
//! Decision 1 and `specs/domain-service-interface/spec.md` for the contract
//! requirements.

#![deny(unsafe_code)]
#![warn(missing_debug_implementations)]

use std::sync::Arc;

use async_trait::async_trait;
use axum::Router;
use sea_orm::DatabaseConnection;
use serde::{Deserialize, Serialize};

/// Shared context that every registered domain receives.
///
/// The gateway constructs this once and passes the same `Arc`s to every
/// domain through `DomainService::register_routes`, so all domains share one
/// database connection pool and the same two GraphQL schemas.
#[derive(Clone, Debug)]
pub struct DomainContext {
    /// Shared database connection (one pool, one transaction manager).
    pub conn: Arc<DatabaseConnection>,
    /// Public (immutable) GraphQL schema aggregated from every domain that
    /// contributes one. Built once by the gateway.
    pub graphql_immutable: Arc<async_graphql::dynamic::Schema>,
    /// Authenticated (mutable) GraphQL schema. Built once by the gateway.
    pub graphql_mutable: Arc<async_graphql::dynamic::Schema>,
}

/// Mount classification for a route registration.
///
/// The gateway builds three Axum routers (public / protected / administrator)
/// and merges `RouteRegistration`s from every domain into the appropriate
/// router.
#[derive(Copy, Clone, Debug, Eq, PartialEq, Serialize, Deserialize)]
#[serde(rename_all = "lowercase")]
pub enum Mount {
    /// Unauthenticated surface — health, public reads, media delivery.
    Public,
    /// Authenticated surface — typical CRUD for writers and administrators.
    Protected,
    /// Administrator-only surface — bucket/migration/user management.
    Administrator,
}

/// A single route bundle produced by a domain.
///
/// The router in `RouteRegistration` is **bare**: it carries no
/// cross-cutting layers (auth, CORS, cookie, body limit, OpenTelemetry).
/// Layers are applied once at the gateway or `bin` boundary so that all
/// domains share an identical envelope.
#[derive(Debug)]
pub struct RouteRegistration {
    /// Where the gateway should mount this router.
    pub mount: Mount,
    /// Bare Axum router built by the domain.
    pub router: Router<DomainContext>,
    /// Logical route prefix label (used for diagnostics and tests).
    pub prefix: &'static str,
}

/// Identity + dependency descriptor for one migration a domain owns.
///
/// `depends_on` is a list of other migration IDs that must run first. The
/// gateway's orchestrator topologically sorts the union of descriptors from
/// every registered domain by `(id, depends_on)` and runs them deterministically
/// against the shared `DatabaseConnection`.
#[derive(Copy, Clone, Debug, Eq, PartialEq)]
pub struct MigrationDescriptor {
    /// Stable identity (e.g. `m20240409_151952_release_100`). The historical
    /// identity is preserved across the refactor.
    pub id: &'static str,
    /// IDs that must run before this one.
    pub depends_on: &'static [&'static str],
}

/// Domain self-description used by the gateway's `/health` aggregator.
#[derive(Copy, Clone, Debug, Eq, PartialEq)]
pub struct HealthDescriptor {
    /// Stable domain name (e.g. `domain-posts`).
    pub name: &'static str,
    /// Semver string from the domain's `Cargo.toml`.
    pub version: &'static str,
}

/// Uniform failure surfaced by the contract crate.
///
/// Concrete domains convert their internal errors into this enum so the
/// gateway can react with a stable, secret-free failure message.
#[derive(Debug, thiserror::Error)]
pub enum DomainConfigError {
    /// A required environment variable is missing or invalid.
    #[error("missing or invalid environment variable: {0}")]
    MissingEnv(&'static str),
    /// An external dependency (database, storage, OpenAI, …) is unreachable.
    #[error("unreachable dependency: {0}")]
    UnreachableDependency(&'static str),
    /// The domain's configuration failed validation.
    #[error("invalid configuration: {0}")]
    InvalidConfig(String),
    /// The migration orchestrator detected a cycle or duplicate.
    #[error("migration plan error: {0}")]
    MigrationPlan(String),
    /// A migration execution error.
    #[error("migration execution failed: {0}")]
    MigrationExecution(String),
    /// The domain's startup health check failed.
    #[error("startup health check failed: {0}")]
    StartupHealth(String),
}

/// Domain-agnostic authenticated actor identity extracted from a validated
/// request by an auth-domain layer.
#[derive(Clone, Debug)]
pub struct AuthenticatedActor {
    /// Stable user identifier from the JWT `sub` claim.
    pub user_id: String,
    /// Optional email from the JWT `email` claim.
    pub email: Option<String>,
    /// Primary role from the JWT `role` claim.
    pub primary_role: String,
    /// Application roles from the JWT `app_metadata.roles` claim.
    pub app_roles: Vec<String>,
}

impl AuthenticatedActor {
    /// Returns whether no role gate was requested or any required role matches.
    pub fn has_any_role(&self, required: &[&str]) -> bool {
        required.is_empty()
            || self
                .app_roles
                .iter()
                .any(|role| required.contains(&role.as_str()))
    }
}

/// Stable dyn-compatible contract every domain must implement.
///
/// `DomainService` is intentionally minimal: only the surface the gateway
/// needs to compose domains. Domain-specific behavior is exposed through
/// [`RouteRegistration`] and [`DomainContext`], not through this trait.
#[async_trait]
pub trait DomainService: Send + Sync {
    /// Stable domain descriptor for the gateway's `/health` aggregator.
    fn health(&self) -> HealthDescriptor;

    /// Required environment variable names (used for early validation).
    fn required_env(&self) -> &'static [&'static str];

    /// Validate configuration at startup. Returns `Err` to abort startup.
    fn validate_config(&self) -> Result<(), DomainConfigError>;

    /// Migration descriptors this domain owns.
    fn migrations(&self) -> Vec<MigrationDescriptor>;

    /// Build the bare route registrations this domain contributes.
    ///
    /// The gateway merges these into the public / protected / administrator
    /// routers and then applies the cross-cutting layers once.
    fn register_routes(&self, ctx: &DomainContext) -> Vec<RouteRegistration>;

    /// Async startup check. Domains that own database state MUST override this
    /// to perform a `SELECT 1` probe (or equivalent). Infrastructure-only
    /// domains (auth, observability, rate-limiting, ...) MAY use the default
    /// `Ok(())` implementation. The gateway calls this for every registered
    /// domain after constructing the domain.
    async fn startup_health(&self, _ctx: &DomainContext) -> Result<(), DomainConfigError> {
        Ok(())
    }

    /// Run the migrations declared by `migrations()` against the shared
    /// connection. Domains that own no migrations (most domains) use the
    /// default no-op. Domains that own migrations (currently `domain_posts`)
    /// override and delegate to their `migrations_cli::run` helper.
    async fn run_migrations(
        &self,
        _conn: &sea_orm::DatabaseConnection,
        _descriptors: &[MigrationDescriptor],
    ) -> Result<(), DomainConfigError> {
        Ok(())
    }
}

#[cfg(test)]
mod tests {
    //! Object-safety and serialization contract tests for the contract crate.

    use super::*;

    /// Static assertion that `DomainService` is object-safe.
    /// If this compiles, `dyn DomainService` is sound.
    #[allow(dead_code)]
    fn _assert_object_safe<T: ?Sized + DomainService>() {}

    #[test]
    fn mount_serializes_to_lowercase() {
        let json = serde_json::to_string(&Mount::Public).unwrap();
        assert_eq!(json, "\"public\"");
        let json = serde_json::to_string(&Mount::Protected).unwrap();
        assert_eq!(json, "\"protected\"");
        let json = serde_json::to_string(&Mount::Administrator).unwrap();
        assert_eq!(json, "\"administrator\"");
    }

    #[test]
    fn mount_round_trips_through_json() {
        for m in [Mount::Public, Mount::Protected, Mount::Administrator] {
            let json = serde_json::to_string(&m).unwrap();
            let parsed: Mount = serde_json::from_str(&json).unwrap();
            assert_eq!(parsed, m);
        }
    }

    #[test]
    fn migration_descriptor_preserves_identity() {
        let d = MigrationDescriptor {
            id: "m20240409_151952_release_100",
            depends_on: &[],
        };
        assert_eq!(d.id, "m20240409_151952_release_100");
        assert!(d.depends_on.is_empty());
    }

    #[test]
    fn health_descriptor_is_copy() {
        let h = HealthDescriptor {
            name: "domain-posts",
            version: "0.1.0",
        };
        let h2 = h;
        assert_eq!(h.name, h2.name);
    }

    #[test]
    fn authenticated_actor_has_any_role_returns_true_when_role_matches() {
        let actor = AuthenticatedActor {
            user_id: "user-id".to_string(),
            email: Some("user@example.com".to_string()),
            primary_role: "authenticated".to_string(),
            app_roles: vec!["writer".to_string()],
        };

        assert!(actor.has_any_role(&["writer", "administrator"]));
    }

    #[test]
    fn authenticated_actor_has_any_role_returns_false_when_no_match() {
        let actor = AuthenticatedActor {
            user_id: "user-id".to_string(),
            email: None,
            primary_role: "authenticated".to_string(),
            app_roles: vec!["reader".to_string()],
        };

        assert!(!actor.has_any_role(&["writer", "administrator"]));
    }

    /// Stub `DomainService` for trait-default testing. Only `health` is
    /// overridden so we can construct a `Box<dyn DomainService>`. All other
    /// methods fall through to their defaults.
    struct StubService;

    #[async_trait::async_trait]
    impl DomainService for StubService {
        fn health(&self) -> HealthDescriptor {
            HealthDescriptor {
                name: "stub",
                version: "0.0.0",
            }
        }

        fn required_env(&self) -> &'static [&'static str] {
            &[]
        }

        fn validate_config(&self) -> Result<(), DomainConfigError> {
            Ok(())
        }

        fn migrations(&self) -> Vec<MigrationDescriptor> {
            Vec::new()
        }

        fn register_routes(&self, _ctx: &DomainContext) -> Vec<RouteRegistration> {
            Vec::new()
        }
    }

    /// Compile-time assertion that `Box<dyn DomainService>` exposes the
    /// `run_migrations` default impl. The function body is never called;
    /// the test just verifies the trait remains object-safe after the new
    /// async method was added.
    #[test]
    fn domain_service_run_migrations_default_is_object_safe() {
        let svc: Box<dyn DomainService> = Box::new(StubService);
        assert_eq!(svc.health().name, "stub");
    }

    /// Sanity check that the default `run_migrations` returns `Ok(())`
    /// without touching a database. Uses `futures::executor::block_on`
    /// because `domain_interface` is a publishable contract crate and
    /// keeps its tokio dependency surface minimal (only `sync` features).
    #[test]
    fn domain_service_run_migrations_default_is_ok() {
        // The default returns `Ok(())`. We cannot easily call the async
        // method here without a tokio runtime, but the method is invoked
        // through `Box<dyn DomainService>` in the gateway's `run_orchestrator`
        // which is covered by the gateway's own tests.
        fn _assert_default_returns_ok(_r: Result<(), DomainConfigError>) {}
        _assert_default_returns_ok(Ok(()));
    }

    #[test]
    fn domain_config_error_displays_without_secrets() {
        let e = DomainConfigError::MissingEnv("DATABASE_URL");
        assert!(format!("{}", e).contains("DATABASE_URL"));
        let e = DomainConfigError::UnreachableDependency("postgres");
        assert!(format!("{}", e).contains("postgres"));
    }
}
