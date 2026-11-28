//! `CaptureAdapter` port (FN-17; IFC section 9). The worker hands the adapter the
//! opaque storage key of an allocation it has already registered (register before
//! write); the adapter writes the synthetic fixture through `FixtureStorage` and
//! reports component outcomes. The only implementation is `FixtureCaptureAdapter`
//! (T-05): real-data capture is not authorised.

use async_trait::async_trait;
use thiserror::Error;
use uuid::Uuid;

use super::fixture_storage::StorageKey;
use super::job_store::Components;

/// What the adapter may know about the job. No paths, no credentials.
#[derive(Debug, Clone, PartialEq, Eq)]
pub struct CaptureContext {
    pub job_id: Uuid,
    pub instance_id: String,
    pub correlation_id: String,
    /// Opaque key of the allocation registered for this capture.
    pub storage_key: StorageKey,
}

/// Component outcomes the adapter reports (IFC section 2.3).
#[derive(Debug, Clone, PartialEq, Eq)]
pub struct CaptureOutcome {
    pub components: Components,
}

/// Carries no detail: the worker maps every failure to the job reason `ADAPTER_ERROR`.
#[derive(Debug, Error, PartialEq, Eq)]
pub enum CaptureError {
    #[error("capture failed")]
    Failed,
}

#[async_trait]
pub trait CaptureAdapter: Send + Sync {
    async fn capture(&self, ctx: &CaptureContext) -> Result<CaptureOutcome, CaptureError>;
}
