//! `AuditSink` port (FN-34; IFC sections 7 and 9). Append-only audit record
//! written in the caller's transaction where one exists.
//!

use async_trait::async_trait;
use chrono::{DateTime, Utc};
use sea_orm::DatabaseTransaction;
use thiserror::Error;
use uuid::Uuid;

/// Closed audit event set (IFC section 7).
#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
pub enum AuditEventKind {
    JobAccepted,
    JobRunning,
    JobIncomplete,
    JobFailed,
    DispatchFailed,
    DownloadDenied,
    CapabilityDenied,
    AllocationRegistered,
    AllocationDeleteClaimed,
    AllocationDeleted,
    OrphanDiscovered,
    CleanupExhausted,
    OwnershipAmbiguous,
    ReconcileFenced,
    ReconcileMissingFile,
    RetentionPurged,
}

impl AuditEventKind {
    pub const ALL: [AuditEventKind; 16] = [
        AuditEventKind::JobAccepted,
        AuditEventKind::JobRunning,
        AuditEventKind::JobIncomplete,
        AuditEventKind::JobFailed,
        AuditEventKind::DispatchFailed,
        AuditEventKind::DownloadDenied,
        AuditEventKind::CapabilityDenied,
        AuditEventKind::AllocationRegistered,
        AuditEventKind::AllocationDeleteClaimed,
        AuditEventKind::AllocationDeleted,
        AuditEventKind::OrphanDiscovered,
        AuditEventKind::CleanupExhausted,
        AuditEventKind::OwnershipAmbiguous,
        AuditEventKind::ReconcileFenced,
        AuditEventKind::ReconcileMissingFile,
        AuditEventKind::RetentionPurged,
    ];

    /// Stored spelling (`job.accepted`, ...).
    pub fn as_str(self) -> &'static str {
        match self {
            AuditEventKind::JobAccepted => "job.accepted",
            AuditEventKind::JobRunning => "job.running",
            AuditEventKind::JobIncomplete => "job.incomplete",
            AuditEventKind::JobFailed => "job.failed",
            AuditEventKind::DispatchFailed => "dispatch.failed",
            AuditEventKind::DownloadDenied => "download.denied",
            AuditEventKind::CapabilityDenied => "capability.denied",
            AuditEventKind::AllocationRegistered => "allocation.registered",
            AuditEventKind::AllocationDeleteClaimed => "allocation.delete_claimed",
            AuditEventKind::AllocationDeleted => "allocation.deleted",
            AuditEventKind::OrphanDiscovered => "orphan.discovered",
            AuditEventKind::CleanupExhausted => "cleanup.exhausted",
            AuditEventKind::OwnershipAmbiguous => "ownership.ambiguous",
            AuditEventKind::ReconcileFenced => "reconcile.fenced",
            AuditEventKind::ReconcileMissingFile => "reconcile.missing_file",
            AuditEventKind::RetentionPurged => "retention.purged",
        }
    }
}

#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
pub enum AuditOutcome {
    Success,
    Failure,
    Denied,
}

impl AuditOutcome {
    /// Stored spelling.
    pub fn as_str(self) -> &'static str {
        match self {
            AuditOutcome::Success => "success",
            AuditOutcome::Failure => "failure",
            AuditOutcome::Denied => "denied",
        }
    }
}

/// SHA-256 hex (64 hex characters) of a non-secret identifier. A raw value (an
/// Idempotency-Key, a token) is refused, so it can never reach the `digest` column.
#[derive(Debug, Clone, PartialEq, Eq)]
pub struct Digest(String);

#[derive(Debug, Error, PartialEq, Eq)]
#[error("not a sha-256 hex digest")]
pub struct InvalidDigest;

impl Digest {
    /// Accepts exactly 64 ASCII hex characters.
    pub fn new(hex: &str) -> Result<Self, InvalidDigest> {
        if hex.len() == 64 && hex.bytes().all(|b| b.is_ascii_hexdigit()) {
            Ok(Self(hex.to_owned()))
        } else {
            Err(InvalidDigest)
        }
    }
    pub fn as_str(&self) -> &str {
        &self.0
    }
}

/// One audit record. Exactly the `backup_audit` columns except the generated id:
/// there is no payload or free-text field (IFC section 7 excluded fields).
#[derive(Debug, Clone, PartialEq, Eq)]
pub struct AuditEvent {
    pub occurred_at: DateTime<Utc>,
    /// `None` for system events.
    pub actor_sub: Option<String>,
    pub instance_id: String,
    pub job_id: Option<Uuid>,
    pub allocation_id: Option<Uuid>,
    pub operation: String,
    pub event: AuditEventKind,
    pub outcome: AuditOutcome,
    pub reason_code: Option<String>,
    pub digest: Option<Digest>,
    pub correlation_id: String,
}

/// Carries no database detail (SQL text and upstream bodies are excluded fields).
#[derive(Debug, Error, PartialEq, Eq)]
pub enum AuditError {
    /// The audit statement was rejected while the connection is healthy (constraint,
    /// trigger or schema rejection). The caller refuses the operation with 503
    /// `AUDIT_UNAVAILABLE`; store health is unchanged (FD 6.6).
    #[error("audit write rejected")]
    Rejected,
    /// Connection loss, pool exhaustion or statement timeout: the outage predicate
    /// applies (503 `STORE_UNAVAILABLE`). A failure that cannot be classified is
    /// treated as this outage, failing closed (FD 6.6).
    #[error("audit store unavailable")]
    Unavailable,
}

#[async_trait]
pub trait AuditSink: Send + Sync {
    /// Append one row. With `Some(tx)` the row is written on that transaction and
    /// is visible only if the caller commits it; with `None` it commits on its own.
    async fn record(
        &self,
        tx: Option<&DatabaseTransaction>,
        ev: AuditEvent,
    ) -> Result<(), AuditError>;
}
