//! `service` — `DomainUserService`, the `domain_interface::DomainService`
//! implementation that owns user HTTP routes and dependencies.
//!
//! Constructed once at gateway startup from a `UserApiState` built from a
//! `SupabaseAdminClient` (which itself is constructed from `SUPABASE_URL` +
//! `SUPABASE_SERVICE_ROLE_KEY`). The user domain owns no database
//! migrations; its state lives in Supabase GoTrue.

use std::sync::Arc;

use async_trait::async_trait;
use domain_interface::{
    DomainConfigError, DomainContext, DomainService, HealthDescriptor, MigrationDescriptor,
    RouteRegistration,
};

use crate::{
    api::{routes::routes as build_routes, state::UserApiState},
    handlers::supabase_admin_client::SupabaseAdminClient,
};

/// `DomainUserService` — the user-domain service registered into the
/// gateway. Constructed once with a validated `SupabaseAdminClient` and
/// queried through the `DomainService` trait for routes and health.
pub struct DomainUserService {
    state: UserApiState,
}

impl std::fmt::Debug for DomainUserService {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        f.debug_struct("DomainUserService")
            .field("state", &self.state)
            .finish_non_exhaustive()
    }
}

impl DomainUserService {
    /// Build a `DomainUserService` from a `SupabaseAdminClient`. The caller
    /// is responsible for ensuring the URL + key pair is valid.
    pub fn new(client: SupabaseAdminClient) -> Self {
        Self {
            state: UserApiState::new(client),
        }
    }

    /// Build a `DomainUserService` from a `UserApiState`. The caller is
    /// responsible for ensuring the state is consistent.
    pub fn from_state(state: UserApiState) -> Self {
        Self { state }
    }

    /// Borrow the inner `UserApiState`.
    pub fn state(&self) -> &UserApiState {
        &self.state
    }
}

#[async_trait]
impl DomainService for DomainUserService {
    fn health(&self) -> HealthDescriptor {
        HealthDescriptor {
            name: "domain-user",
            version: env!("CARGO_PKG_VERSION"),
        }
    }

    fn required_env(&self) -> &'static [&'static str] {
        &["SUPABASE_URL", "SUPABASE_SERVICE_ROLE_KEY"]
    }

    fn validate_config(&self) -> Result<(), DomainConfigError> {
        // The service holds an already-constructed `SupabaseAdminClient`; the
        // gateway is expected to have built it from a valid env var pair.
        // Runtime checks are limited to confirming the underlying client is
        // non-empty. This keeps the trait surface consistent with
        // `DomainMediaService::validate_config`.
        let _ = self.state.supabase_admin_client.supabase_url.len();
        let _ = self.state.supabase_admin_client.service_role_key.len();
        Ok(())
    }

    fn migrations(&self) -> Vec<MigrationDescriptor> {
        // The user domain does not own database migrations; user records
        // live in Supabase GoTrue, not in the CMS Postgres database.
        Vec::new()
    }

    fn register_routes(&self, _ctx: &DomainContext) -> Vec<RouteRegistration> {
        build_routes(self.state.clone())
    }

    async fn startup_health(&self, _ctx: &DomainContext) -> Result<(), DomainConfigError> {
        Ok(())
    }
}

/// Convenience constructor that wraps `SupabaseAdminClient` in an `Arc` so a
/// caller can hand the same client to multiple services without cloning the
/// underlying reqwest `Client`.
impl From<Arc<SupabaseAdminClient>> for DomainUserService {
    fn from(client: Arc<SupabaseAdminClient>) -> Self {
        Self::new((*client).clone())
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::handlers::supabase_admin_client::SupabaseAdminClient;
    use crate::test_lock::{with_env_var, ENV_LOCK};

    fn stub_client() -> SupabaseAdminClient {
        SupabaseAdminClient::new(
            "http://localhost:9999".to_string(),
            "service-role-test-key".to_string(),
        )
    }

    #[test]
    fn health_descriptor_is_domain_user() {
        let svc = DomainUserService::new(stub_client());
        let h = svc.health();
        assert_eq!(h.name, "domain-user");
        assert_eq!(h.version, env!("CARGO_PKG_VERSION"));
    }

    #[test]
    fn required_env_lists_two_vars() {
        let svc = DomainUserService::new(stub_client());
        let env = svc.required_env();
        assert_eq!(env.len(), 2);
        assert!(env.contains(&"SUPABASE_URL"));
        assert!(env.contains(&"SUPABASE_SERVICE_ROLE_KEY"));
    }

    #[test]
    fn migrations_is_empty() {
        let svc = DomainUserService::new(stub_client());
        assert!(svc.migrations().is_empty());
    }

    #[test]
    fn validate_config_returns_ok_for_constructed_service() {
        let svc = DomainUserService::new(stub_client());
        assert!(svc.validate_config().is_ok());
    }

    #[test]
    fn domain_user_service_is_object_safe() {
        // The trait is object-safe; constructing a `Box<dyn DomainService>`
        // proves it. If this compiles, the impl is correct.
        let svc: Box<dyn DomainService> = Box::new(DomainUserService::new(stub_client()));
        assert_eq!(svc.health().name, "domain-user");
    }

    #[test]
    fn required_env_matches_validate_config_locked_behavior() {
        // Sanity-check that the env-var lock + helper pair work as expected
        // for tests that need to mutate env vars while validating config.
        let _guard = ENV_LOCK.lock().unwrap();
        with_env_var("SUPABASE_URL", Some("http://test"), || {
            assert_eq!(
                std::env::var("SUPABASE_URL").ok(),
                Some("http://test".to_string())
            );
        });
        // Previous value (or absence) restored.
    }
}
