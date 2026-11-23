//! Claim-time resource policy port (FN-16 with FN-04; work-units discrepancy 1).
//!
//! IFC section 9 lists no policy port. The worker must re-run the FN-04 resource
//! check at claim, but `domain/policy.rs` is built in WU-08, so WU-05 introduces this
//! narrow port; WU-08 supplies the real implementation (TS-05b).

use async_trait::async_trait;
use thiserror::Error;
use uuid::Uuid;

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum PolicyDecision {
    Allowed,
    /// The instance is revoked or unknown, or does not belong to the configured one.
    Denied,
}

/// The policy could not decide. The worker fails closed.
#[derive(Debug, Error, PartialEq, Eq)]
#[error("policy unavailable")]
pub struct PolicyError;

#[async_trait]
pub trait ClaimPolicy: Send + Sync {
    /// FN-04 at worker time: does this job still belong to the configured instance?
    async fn authorize_resource(
        &self,
        instance_id: &str,
        job_id: Uuid,
    ) -> Result<PolicyDecision, PolicyError>;
}
