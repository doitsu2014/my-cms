//! `DomainAuthService` — `domain_interface::DomainService` implementation for
//! the auth domain.
//!
//! See `openspec/changes/extract-auth-into-domain-auth/design.md` for the
//! architectural context. The service is constructed once by either the
//! standalone `domain_auth` bin or the `gateway` composition root and queried
//! through the `DomainService` trait for routing, migrations, and health.

use async_trait::async_trait;
use domain_interface::{
    DomainConfigError, DomainContext, DomainService, HealthDescriptor, MigrationDescriptor,
    RouteRegistration,
};

/// `DomainAuthService` — the auth-domain service registered into the gateway.
///
/// Auth is HTTP-middleware (not routes) and infrastructure-only (no DB probe).
/// `startup_health` uses the default `Ok(())` impl from `DomainService`.
#[derive(Debug, Clone, Default)]
pub struct DomainAuthService;

impl DomainAuthService {
    /// Build a `DomainAuthService`. Auth does not need any construction-time
    /// dependencies — its env-var surface is validated in `validate_config`.
    pub fn new() -> Self {
        Self
    }
}

#[async_trait]
impl DomainService for DomainAuthService {
    fn health(&self) -> HealthDescriptor {
        HealthDescriptor {
            name: "domain-auth",
            version: env!("CARGO_PKG_VERSION"),
        }
    }

    fn required_env(&self) -> &'static [&'static str] {
        &[
            "SUPABASE_URL",
            "SUPABASE_JWT_SECRET",
            "AUTHORIZATION_AUDIENCE",
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
        Vec::new()
    }

    fn register_routes(&self, _ctx: &DomainContext) -> Vec<RouteRegistration> {
        Vec::new()
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::test_lock::{with_env_var, ENV_LOCK};

    /// Static assertion that `DomainAuthService` is object-safe through
    /// the `DomainService` trait. If this compiles, `dyn DomainService`
    /// is sound for the auth impl.
    #[test]
    fn domain_auth_service_is_object_safe() {
        let _: Box<dyn DomainService> = Box::new(DomainAuthService::new());
    }

    #[test]
    fn domain_auth_service_health_descriptor() {
        let service = DomainAuthService::new();
        let health = service.health();
        assert_eq!(health.name, "domain-auth");
        // Version comes from CARGO_PKG_VERSION and is non-empty.
        assert!(!health.version.is_empty());
    }

    #[test]
    fn domain_auth_service_required_env_lists_auth_vars_only() {
        let service = DomainAuthService::new();
        let env = service.required_env();
        assert_eq!(env.len(), 3);
        assert!(env.contains(&"SUPABASE_URL"));
        assert!(env.contains(&"SUPABASE_JWT_SECRET"));
        assert!(env.contains(&"AUTHORIZATION_AUDIENCE"));
    }

    #[test]
    fn domain_auth_service_migrations_is_empty() {
        let service = DomainAuthService::new();
        assert!(service.migrations().is_empty());
    }

    #[test]
    fn validate_config_returns_missing_env_for_supabase_url_when_unset() {
        let _guard = ENV_LOCK.lock().unwrap_or_else(|e| e.into_inner());
        let service = DomainAuthService::new();
        // The other two vars are present so we observe only the SUPABASE_URL
        // failure, not an early-exit on the first missing var.
        let result = with_env_var("SUPABASE_URL", None, || {
            with_env_var("SUPABASE_JWT_SECRET", Some("secret"), || {
                with_env_var("AUTHORIZATION_AUDIENCE", Some("authenticated"), || {
                    service.validate_config()
                })
            })
        });
        match result {
            Err(DomainConfigError::MissingEnv(v)) => assert_eq!(v, "SUPABASE_URL"),
            other => panic!("expected MissingEnv(SUPABASE_URL), got {:?}", other),
        }
    }

    #[test]
    fn validate_config_returns_missing_env_for_supabase_jwt_secret_when_unset() {
        let _guard = ENV_LOCK.lock().unwrap_or_else(|e| e.into_inner());
        let service = DomainAuthService::new();
        let result = with_env_var("SUPABASE_JWT_SECRET", None, || {
            with_env_var("SUPABASE_URL", Some("http://localhost"), || {
                with_env_var("AUTHORIZATION_AUDIENCE", Some("authenticated"), || {
                    service.validate_config()
                })
            })
        });
        match result {
            Err(DomainConfigError::MissingEnv(v)) => assert_eq!(v, "SUPABASE_JWT_SECRET"),
            other => panic!("expected MissingEnv(SUPABASE_JWT_SECRET), got {:?}", other),
        }
    }

    #[test]
    fn validate_config_returns_missing_env_for_authorization_audience_when_unset() {
        let _guard = ENV_LOCK.lock().unwrap_or_else(|e| e.into_inner());
        let service = DomainAuthService::new();
        let result = with_env_var("AUTHORIZATION_AUDIENCE", None, || {
            with_env_var("SUPABASE_URL", Some("http://localhost"), || {
                with_env_var("SUPABASE_JWT_SECRET", Some("secret"), || {
                    service.validate_config()
                })
            })
        });
        match result {
            Err(DomainConfigError::MissingEnv(v)) => assert_eq!(v, "AUTHORIZATION_AUDIENCE"),
            other => panic!(
                "expected MissingEnv(AUTHORIZATION_AUDIENCE), got {:?}",
                other
            ),
        }
    }

    #[test]
    fn validate_config_succeeds_when_all_required_env_vars_are_set() {
        let _guard = ENV_LOCK.lock().unwrap_or_else(|e| e.into_inner());
        let service = DomainAuthService::new();
        let result = with_env_var("SUPABASE_URL", Some("http://localhost"), || {
            with_env_var("SUPABASE_JWT_SECRET", Some("secret"), || {
                with_env_var("AUTHORIZATION_AUDIENCE", Some("authenticated"), || {
                    service.validate_config()
                })
            })
        });
        assert!(result.is_ok(), "expected Ok(()), got {:?}", result);
    }
}
