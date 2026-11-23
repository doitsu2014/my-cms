//! `FixtureCaptureAdapter` (FN-17): writes one fixed synthetic fixture through
//! `FixtureStorage` and reports the U1 components. The only `CaptureAdapter` in the
//! crate (T-05, ADR-001): real-data capture is not authorised.

use async_trait::async_trait;
use std::fmt;
use std::sync::Arc;

use crate::ports::capture::{CaptureAdapter, CaptureContext, CaptureError, CaptureOutcome};
use crate::ports::fixture_storage::FixtureStorage;
use crate::ports::job_store::Components;

/// Fixed synthetic fixture size in bytes: a code constant, well under CFG-12
/// (IFC section 8, non-owner guard [E]).
pub const FIXTURE_BYTES: u64 = 4096;

pub struct FixtureCaptureAdapter {
    #[allow(dead_code)]
    storage: Arc<dyn FixtureStorage>,
}

impl fmt::Debug for FixtureCaptureAdapter {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        f.debug_struct("FixtureCaptureAdapter")
            .finish_non_exhaustive()
    }
}

impl FixtureCaptureAdapter {
    pub fn new(storage: Arc<dyn FixtureStorage>) -> Self {
        Self { storage }
    }

    /// The synthetic fixture content: exactly `FIXTURE_BYTES` bytes, deterministic,
    /// ASCII, carrying nothing from the environment.
    pub fn fixture_content() -> Vec<u8> {
        Vec::new()
    }
}

#[async_trait]
impl CaptureAdapter for FixtureCaptureAdapter {
    async fn capture(&self, _ctx: &CaptureContext) -> Result<CaptureOutcome, CaptureError> {
        Ok(CaptureOutcome {
            components: Components::default(),
        })
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::lifecycle::test_support::{contract_components, ctx, MemStorage};
    use crate::ports::fixture_storage::StorageError;

    fn adapter(storage: &Arc<MemStorage>) -> FixtureCaptureAdapter {
        FixtureCaptureAdapter::new(storage.clone())
    }

    /// FN-17, T-05: one write under the opaque key of the registered allocation, of the
    /// fixed fixture size.
    #[tokio::test]
    async fn writes_one_fixed_size_fixture_under_the_context_key() {
        let storage = MemStorage::new();
        let c = ctx();
        let out = adapter(&storage).capture(&c).await;
        assert!(out.is_ok(), "capture must succeed: {out:?}");
        let writes = storage.writes();
        assert_eq!(writes.len(), 1, "exactly one fixture write");
        assert_eq!(writes[0].0, c.storage_key);
        assert_eq!(writes[0].1.len() as u64, FIXTURE_BYTES);
        assert_eq!(writes[0].1, FixtureCaptureAdapter::fixture_content());
    }

    /// FN-17, T-05: the fixture is synthetic: fixed size, deterministic, ASCII and free of
    /// environment, path and credential-looking content.
    #[test]
    fn fixture_content_is_fixed_size_deterministic_ascii() {
        let a = FixtureCaptureAdapter::fixture_content();
        let b = FixtureCaptureAdapter::fixture_content();
        assert_eq!(a.len() as u64, FIXTURE_BYTES);
        assert_eq!(a, b);
        assert!(a.is_ascii());
        let text = String::from_utf8(a).unwrap();
        assert!(text.contains("SYNTHETIC"), "fixture is labelled synthetic");
        for banned in [
            "postgres://",
            "Bearer ",
            "password",
            "secret",
            "/home/",
            "C:\\",
        ] {
            assert!(!text.contains(banned), "fixture must not contain {banned}");
        }
    }

    /// FN-17 / TS-47 evidence: exact components array of the contract.
    #[tokio::test]
    async fn reports_the_exact_contract_components() {
        let storage = MemStorage::new();
        let out = adapter(&storage).capture(&ctx()).await.expect("capture ok");
        assert_eq!(out.components, contract_components());
        let json = serde_json::to_value(&out.components).unwrap();
        assert_eq!(
            json,
            serde_json::json!([
                {"name": "database", "state": "synthetic_captured", "reason_code": "DB_SYNTHETIC_ONLY"},
                {"name": "auth", "state": "unavailable", "reason_code": "AUTH_NOT_CAPTURED"},
                {"name": "storage", "state": "unavailable", "reason_code": "STORAGE_NOT_CAPTURED"}
            ])
        );
    }

    /// FN-17, FN-19: a storage failure is a capture failure, with no component reported.
    #[tokio::test]
    async fn storage_failure_is_a_capture_error() {
        let storage = MemStorage::failing(StorageError::Unavailable);
        let out = adapter(&storage).capture(&ctx()).await;
        assert_eq!(out, Err(CaptureError::Failed));
    }

    /// FN-17: a refused write (no registered allocation) is a capture error too.
    #[tokio::test]
    async fn refused_write_is_a_capture_error() {
        let storage = MemStorage::failing(StorageError::NotRegistered);
        let out = adapter(&storage).capture(&ctx()).await;
        assert_eq!(out, Err(CaptureError::Failed));
    }
}
