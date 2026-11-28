//! Administrator and resource authorization rules (FN-03, FN-04; TS-04, TS-05b, TS-59).

use axum::http::{header, HeaderMap, StatusCode};
use domain_interface::AuthenticatedActor;
use thiserror::Error;
use uuid::Uuid;

use super::instance::InstanceRegistry;
use crate::ports::policy::{ClaimPolicy, PolicyDecision, PolicyError};

pub const ADMIN_ROLE: &str = "my-headless-cms-administrator";

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum AuthorizationFailure {
    RoleRequired,
    HostNotAllowed,
    OriginNotAllowed,
}

impl AuthorizationFailure {
    pub fn status(self) -> StatusCode {
        match self {
            Self::RoleRequired | Self::HostNotAllowed | Self::OriginNotAllowed => {
                StatusCode::FORBIDDEN
            }
        }
    }

    pub fn reason_code(self) -> &'static str {
        match self {
            Self::RoleRequired => "ROLE_REQUIRED",
            Self::HostNotAllowed => "HOST_NOT_ALLOWED",
            Self::OriginNotAllowed => "ORIGIN_NOT_ALLOWED",
        }
    }
}

#[derive(Debug, Error, PartialEq, Eq)]
#[error("configured instance does not match the job")]
pub struct ResourceDenied;

/// Role check precedes Host and Origin validation (the auth layer's 401 runs before this).
pub fn authorize_operation(
    actor: Option<&AuthenticatedActor>,
    headers: &HeaderMap,
    require_origin: bool,
    configured_port: u16,
    allowed_origins: &[String],
) -> Result<(), AuthorizationFailure> {
    let Some(actor) = actor else {
        return Err(AuthorizationFailure::RoleRequired);
    };
    if !actor.has_any_role(&[ADMIN_ROLE]) {
        return Err(AuthorizationFailure::RoleRequired);
    }
    if !host_allowed(headers, configured_port) {
        return Err(AuthorizationFailure::HostNotAllowed);
    }
    if require_origin && !origin_allowed(headers, allowed_origins) {
        return Err(AuthorizationFailure::OriginNotAllowed);
    }
    Ok(())
}

fn host_allowed(headers: &HeaderMap, configured_port: u16) -> bool {
    let Some(host) = headers
        .get(header::HOST)
        .and_then(|value| value.to_str().ok())
    else {
        return false;
    };
    let allowed = [
        format!("127.0.0.1:{configured_port}"),
        format!("localhost:{configured_port}"),
        format!("[::1]:{configured_port}"),
    ];
    allowed
        .iter()
        .any(|candidate| host.eq_ignore_ascii_case(candidate))
}

fn origin_allowed(headers: &HeaderMap, allowed_origins: &[String]) -> bool {
    match headers.get(header::ORIGIN) {
        None => true,
        Some(origin) => origin
            .to_str()
            .ok()
            .is_some_and(|origin| allowed_origins.iter().any(|allowed| origin == allowed)),
    }
}

/// F-04 shared by HTTP resource reads and the worker's claim-time re-check.
#[derive(Debug, Clone)]
pub struct BackupPolicy {
    instances: InstanceRegistry,
}

impl BackupPolicy {
    pub fn new(instances: InstanceRegistry) -> Self {
        Self { instances }
    }

    pub fn authorize_resource(&self, requested_instance: &str) -> Result<(), ResourceDenied> {
        self.instances
            .get(requested_instance)
            .map(|_| ())
            .ok_or(ResourceDenied)
    }

    pub fn instance_label(&self, instance_id: &str) -> Option<&str> {
        self.instances
            .get(instance_id)
            .map(|instance| instance.label.as_str())
    }
}

#[async_trait::async_trait]
impl ClaimPolicy for BackupPolicy {
    async fn authorize_resource(
        &self,
        instance_id: &str,
        _job_id: Uuid,
    ) -> Result<PolicyDecision, PolicyError> {
        Ok(if self.instances.get(instance_id).is_some() {
            PolicyDecision::Allowed
        } else {
            PolicyDecision::Denied
        })
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    fn actor(roles: &[&str]) -> AuthenticatedActor {
        AuthenticatedActor {
            user_id: "admin-sub".into(),
            email: None,
            primary_role: "authenticated".into(),
            app_roles: roles.iter().map(|role| (*role).into()).collect(),
        }
    }

    fn registry() -> InstanceRegistry {
        let root = crate::test_temp_dir().join(format!("backup-policy-{}", std::process::id()));
        let state = crate::domain::config::BackupConfig::load(|key| {
            Some(match key {
                "BACKUP_ENABLED" => "true".into(),
                "BACKUP_INSTANCE_ID" => "local-1".into(),
                "BACKUP_INSTANCE_LABEL" => "Local".into(),
                "BACKUP_STORAGE_ROOT" => root.to_string_lossy().into_owned(),
                "BACKUP_ALLOWED_ORIGINS" => "http://localhost:3002".into(),
                "BACKUP_REQUEST_TIMEOUT_SECS" => "30".into(),
                "BACKUP_MAX_HEADER_BYTES" => "16384".into(),
                _ => return None,
            })
        });
        let config = match state {
            crate::domain::config::BackupConfigState::Enabled(config) => config,
            crate::domain::config::BackupConfigState::Disabled { reason } => {
                panic!("test config must be enabled, got {reason:?}")
            }
        };
        InstanceRegistry::from_config(&config)
    }

    fn headers(host: &str, origin: Option<&str>) -> HeaderMap {
        let mut headers = HeaderMap::new();
        headers.insert(header::HOST, host.parse().unwrap());
        if let Some(origin) = origin {
            headers.insert(header::ORIGIN, origin.parse().unwrap());
        }
        headers
    }

    /// RK-03: `AuthenticatedActor::has_any_role(&[])` is true, so the in-handler re-assertion
    /// must use a non-empty role constant; an actor with no roles is still refused.
    #[test]
    fn rk03_role_reassertion_never_uses_an_empty_role_list() {
        assert!(!ADMIN_ROLE.is_empty());
        assert!(
            actor(&[]).has_any_role(&[]),
            "upstream semantics this guards against"
        );
        assert_eq!(
            authorize_operation(
                Some(&actor(&[])),
                &headers("localhost:8989", None),
                false,
                8989,
                &["http://localhost:3002".to_owned()],
            ),
            Err(AuthorizationFailure::RoleRequired)
        );
    }

    #[test]
    fn ts59_requires_admin_then_loopback_host_and_exact_post_origin() {
        let policy = ["http://localhost:3002".to_owned()];
        let allowed = ["my-headless-cms-administrator"];
        assert_eq!(
            authorize_operation(None, &HeaderMap::new(), false, 8989, &policy),
            Err(AuthorizationFailure::RoleRequired)
        );
        assert_eq!(
            authorize_operation(
                Some(&actor(&["my-headless-cms-writer"])),
                &headers("evil.example:8989", None),
                false,
                8989,
                &policy,
            ),
            Err(AuthorizationFailure::RoleRequired)
        );
        assert_eq!(
            authorize_operation(
                Some(&actor(&allowed)),
                &headers("evil.example:8989", None),
                false,
                8989,
                &policy,
            ),
            Err(AuthorizationFailure::HostNotAllowed)
        );
        assert_eq!(
            authorize_operation(
                Some(&actor(&allowed)),
                &headers("localhost:8989", Some("http://evil.example")),
                true,
                8989,
                &policy,
            ),
            Err(AuthorizationFailure::OriginNotAllowed)
        );
        assert!(authorize_operation(
            Some(&actor(&allowed)),
            &headers("[::1]:8989", Some("http://localhost:3002")),
            true,
            8989,
            &policy,
        )
        .is_ok());
        assert!(authorize_operation(
            Some(&actor(&allowed)),
            &headers("127.0.0.1:8989", None),
            true,
            8989,
            &policy,
        )
        .is_ok());
    }

    #[tokio::test]
    async fn ts05b_claim_policy_rechecks_only_the_registered_instance() {
        let policy = BackupPolicy::new(registry());
        let id = Uuid::new_v4();
        assert_eq!(
            ClaimPolicy::authorize_resource(&policy, "local-1", id)
                .await
                .unwrap(),
            PolicyDecision::Allowed
        );
        assert_eq!(
            ClaimPolicy::authorize_resource(&policy, "foreign", id)
                .await
                .unwrap(),
            PolicyDecision::Denied
        );
    }
}
