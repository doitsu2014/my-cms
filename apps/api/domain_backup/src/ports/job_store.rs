//! `JobStore` port (FN-15, FN-16; IFC section 9). WU-03 delivers `accept`, `get`,
//! `transition` and `claim_next`. `list` (WU-08) and `fence_older_epochs`
//! (WU-06) join the trait in the unit that first needs them.

use async_trait::async_trait;
use chrono::{DateTime, Utc};
use sea_orm::DatabaseTransaction;
use serde::{Deserialize, Serialize};
use std::time::Duration;
use thiserror::Error;
use uuid::Uuid;

use crate::domain::status::BackupJobStatus;

/// Job-level stable reason codes (IFC section 10.1, "Job-level reason codes").
/// Closed set: free text can never reach the `reason_code` column.
#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
pub enum ReasonCode {
    RequiredComponentsUnavailable,
    AdapterError,
    DeadlineExceeded,
    AllocationLimit,
    PersistenceError,
    InterruptedRestart,
    DispatchFailed,
    AbandonedQueued,
    InstanceNotAllowed,
}

impl ReasonCode {
    pub const ALL: [ReasonCode; 9] = [
        ReasonCode::RequiredComponentsUnavailable,
        ReasonCode::AdapterError,
        ReasonCode::DeadlineExceeded,
        ReasonCode::AllocationLimit,
        ReasonCode::PersistenceError,
        ReasonCode::InterruptedRestart,
        ReasonCode::DispatchFailed,
        ReasonCode::AbandonedQueued,
        ReasonCode::InstanceNotAllowed,
    ];

    /// Stored spelling, for example `ADAPTER_ERROR`.
    pub fn as_str(self) -> &'static str {
        match self {
            ReasonCode::RequiredComponentsUnavailable => "REQUIRED_COMPONENTS_UNAVAILABLE",
            ReasonCode::AdapterError => "ADAPTER_ERROR",
            ReasonCode::DeadlineExceeded => "DEADLINE_EXCEEDED",
            ReasonCode::AllocationLimit => "ALLOCATION_LIMIT",
            ReasonCode::PersistenceError => "PERSISTENCE_ERROR",
            ReasonCode::InterruptedRestart => "INTERRUPTED_RESTART",
            ReasonCode::DispatchFailed => "DISPATCH_FAILED",
            ReasonCode::AbandonedQueued => "ABANDONED_QUEUED",
            ReasonCode::InstanceNotAllowed => "INSTANCE_NOT_ALLOWED",
        }
    }
}

/// Component name (IFC section 2.3).
#[derive(Debug, Clone, Copy, PartialEq, Eq, Serialize, Deserialize)]
#[serde(rename_all = "snake_case")]
pub enum ComponentName {
    Database,
    Auth,
    Storage,
}

/// Component state (IFC section 2.3).
#[derive(Debug, Clone, Copy, PartialEq, Eq, Serialize, Deserialize)]
#[serde(rename_all = "snake_case")]
pub enum ComponentOutcome {
    SyntheticCaptured,
    Unavailable,
}

/// One entry of the `components` jsonb array: `{name, state, reason_code}`
/// (IFC section 2.3). Reason codes are the component codes
/// `DB_SYNTHETIC_ONLY`, `AUTH_NOT_CAPTURED`, `STORAGE_NOT_CAPTURED`.
#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
pub struct ComponentState {
    pub name: ComponentName,
    pub state: ComponentOutcome,
    pub reason_code: Option<String>,
}

/// Stored as the `components` jsonb array.
#[derive(Debug, Clone, PartialEq, Eq, Default, Serialize, Deserialize)]
pub struct Components(pub Vec<ComponentState>);

/// A job to insert. There is deliberately no status field: the store only ever
/// inserts `queued` (WU-02 known limitation: the store inserts only queued).
#[derive(Debug, Clone, PartialEq, Eq)]
pub struct NewJob {
    pub id: Uuid,
    pub actor_sub: String,
    pub instance_id: String,
    pub correlation_id: String,
    pub epoch: i64,
    pub created_at: DateTime<Utc>,
    pub retain_until: DateTime<Utc>,
}

/// Idempotency scope row inserted with the job. Digests only, never the raw key.
#[derive(Debug, Clone, PartialEq, Eq)]
pub struct IdemRecord {
    pub id: Uuid,
    pub actor_sub: String,
    pub operation: String,
    pub key_digest: String,
    pub request_fingerprint: String,
    pub expires_at: DateTime<Utc>,
}

#[derive(Debug, Clone, PartialEq)]
pub struct JobRow {
    pub id: Uuid,
    pub actor_sub: String,
    pub instance_id: String,
    /// Always `backup_db_slice` in U1.
    pub operation: String,
    /// Always true in U1.
    pub synthetic: bool,
    pub status: BackupJobStatus,
    pub state_version: i64,
    pub reason_code: Option<String>,
    pub components: Components,
    pub epoch: i64,
    pub correlation_id: String,
    pub created_at: DateTime<Utc>,
    pub updated_at: DateTime<Utc>,
    pub started_at: Option<DateTime<Utc>>,
    pub finished_at: Option<DateTime<Utc>>,
    pub deadline_at: Option<DateTime<Utc>>,
    pub retain_until: DateTime<Utc>,
}

/// `Replayed` joins in WU-07 (idempotency).
#[derive(Debug, Clone, PartialEq)]
pub enum AcceptResult {
    Created(JobRow),
}

/// `Lost`: another actor won the compare-and-set; the caller must not replay effects.
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum TransitionResult {
    Applied,
    Lost,
}

/// Display strings are fixed and carry no database detail.
#[derive(Debug, Error, PartialEq, Eq)]
pub enum StoreError {
    /// A unique violation on insert: the partial unique index (one queued or running
    /// job per instance) or the idempotency UNIQUE `(actor_sub, operation, key_digest)`.
    /// IFC section 4: on ANY unique violation the caller rolls back and re-reads
    /// `backup_idempotency` by `key_digest` to choose replay, `IDEMPOTENCY_KEY_REUSED`
    /// or `CONCURRENCY_LIMIT`. A primary-key clash is `Persistence`.
    #[error("another job is already queued or running for this instance")]
    ConcurrencyConflict,
    #[error("job not found")]
    NotFound,
    /// The edge is outside queued->running, queued->failed, running->incomplete,
    /// running->failed (includes every move out of a terminal state).
    #[error("transition not allowed")]
    IllegalTransition {
        from: BackupJobStatus,
        to: BackupJobStatus,
    },
    /// The audit write was rejected on a healthy connection, so the sensitive
    /// operation was refused and rolled back (IFC 6.3; FD 6.6 `AuditError::Rejected`).
    #[error("audit unavailable")]
    AuditUnavailable,
    /// Outage: connection loss, pool exhaustion or statement timeout, from the store's
    /// own statements or from the audit sink (`AuditError::Unavailable`). Maps to 503
    /// `STORE_UNAVAILABLE` (FD 6.6).
    #[error("store unavailable")]
    StoreUnavailable,
    /// Carries no database text, so no SQL or driver detail can leak through `Debug`.
    #[error("store persistence error")]
    Persistence,
}

#[async_trait]
pub trait JobStore: Send + Sync {
    /// Insert a `queued` job and its idempotency row on the caller's transaction.
    async fn accept(
        &self,
        tx: &DatabaseTransaction,
        new: NewJob,
        idem: IdemRecord,
    ) -> Result<AcceptResult, StoreError>;

    async fn get(&self, id: Uuid) -> Result<Option<JobRow>, StoreError>;

    /// Compare-and-set on `(status, state_version)`. Writes the matching `job.*`
    /// audit row in the same transaction; an audit failure refuses the transition.
    async fn transition(
        &self,
        id: Uuid,
        expect: (BackupJobStatus, i64),
        to: BackupJobStatus,
        reason: Option<ReasonCode>,
        comps: Option<Components>,
    ) -> Result<TransitionResult, StoreError>;

    /// Conditional claim of the oldest `queued` job to `running`: sets `epoch`,
    /// `started_at` and `deadline_at = now + deadline`, bumps `state_version`, and
    /// writes `job.running` in the same transaction. `None` when nothing was claimed.
    async fn claim_next(
        &self,
        epoch: i64,
        deadline: Duration,
    ) -> Result<Option<JobRow>, StoreError>;
}
