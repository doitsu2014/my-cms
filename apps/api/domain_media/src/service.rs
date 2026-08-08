//! `service` — `DomainMediaService`, the `domain_interface::DomainService`
//! implementation that owns media HTTP routes and dependencies.
//!
//! Constructed once at gateway startup; constructs the `MediaConfig`, the
//! media delivery cache, and the bucket-visibility cache once per process.

use std::sync::Arc;

use async_trait::async_trait;
use domain_interface::{
    DomainConfigError, DomainContext, DomainService, HealthDescriptor, MigrationDescriptor,
    RouteRegistration,
};

use crate::{
    api::{routes::routes as build_routes, state::MediaApiState},
    handlers::MediaConfig,
};

/// `DomainMediaService` — the media-domain service registered into the
/// gateway. Constructed once with a validated `MediaConfig` (and shared
/// caches) and queried through the `DomainService` trait for routes, health,
/// and migrations.
pub struct DomainMediaService {
    state: MediaApiState,
}

impl std::fmt::Debug for DomainMediaService {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        f.debug_struct("DomainMediaService")
            .field("state", &self.state)
            .finish_non_exhaustive()
    }
}

impl DomainMediaService {
    /// Build a `DomainMediaService` from an already-validated `MediaConfig`.
    /// The caches are initialized once via the canonical factories.
    pub fn new(media_config: Arc<MediaConfig>) -> Self {
        Self {
            state: MediaApiState::new(media_config),
        }
    }

    /// Build a `DomainMediaService` from a `MediaApiState`. The caller is
    /// responsible for ensuring the state is consistent (config matches the
    /// caches' expectations).
    pub fn from_state(state: MediaApiState) -> Self {
        Self { state }
    }

    /// Borrow the inner `MediaApiState`. Useful for tests and gateway tests
    /// that want to assert against the live media config.
    pub fn state(&self) -> &MediaApiState {
        &self.state
    }
}

#[async_trait]
impl DomainService for DomainMediaService {
    fn health(&self) -> HealthDescriptor {
        HealthDescriptor {
            name: "domain-media",
            version: env!("CARGO_PKG_VERSION"),
        }
    }

    fn required_env(&self) -> &'static [&'static str] {
        &["SUPABASE_URL", "SUPABASE_SERVICE_ROLE_KEY"]
    }

    fn validate_config(&self) -> Result<(), DomainConfigError> {
        // The service holds an already-validated `MediaConfig`; nothing
        // further to check at runtime. The gateway is expected to have
        // constructed this with a valid config.
        Ok(())
    }

    fn migrations(&self) -> Vec<MigrationDescriptor> {
        // The media domain does not own database migrations.
        Vec::new()
    }

    fn register_routes(&self, _ctx: &DomainContext) -> Vec<RouteRegistration> {
        // The media domain owns its own Axum state (MediaApiState); the
        // `DomainContext` is not used here.
        build_routes(self.state.clone())
    }

    async fn startup_health(&self, _ctx: &DomainContext) -> Result<(), DomainConfigError> {
        Ok(())
    }
}
