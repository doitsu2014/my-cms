//! Fixtures and raw-SQL readers for the `DbAllocationStore` tests. Ground truth is
//! always read with raw SQL, never through the store under test.

use super::support::{count, exec, pg, t0, FixedClock};
use crate::infra::db_allocation_store::DbAllocationStore;
use crate::infra::db_audit_sink::DbAuditSink;
use crate::ports::allocation_store::{AllocationLimits, AllocationState};
use crate::ports::audit::{AuditError, AuditEvent, AuditEventKind, AuditSink};
use crate::test_db::{backup_test_db, BackupTestDb};
use async_trait::async_trait;
use chrono::{DateTime, Duration, Utc};
use sea_orm::{ConnectionTrait, DatabaseConnection, DatabaseTransaction};
use std::sync::{Arc, Mutex};
use uuid::Uuid;

pub(super) const MIB: u64 = 1024 * 1024;
pub(super) const ALLOC_MAX: u64 = 256 * MIB;
pub(super) const TOTAL_MAX: u64 = 1024 * MIB;
pub(super) const INSTANCE: &str = "local-1";

#[derive(Debug)]
pub(super) struct AFx {
    _db: BackupTestDb,
    pub conn: DatabaseConnection,
    pub store: Arc<DbAllocationStore>,
}

/// Fresh PostgreSQL 15 with the WU-02 migrations and a store wired to the real sink,
/// a clock fixed at `t0()` and the Q-05 limits.
pub(super) async fn afx() -> AFx {
    let db = backup_test_db().await;
    let conn = db.conn.clone();
    let store = astore(&conn, Arc::new(DbAuditSink::new(conn.clone())));
    AFx {
        _db: db,
        conn,
        store,
    }
}

pub(super) fn astore(
    conn: &DatabaseConnection,
    audit: Arc<dyn AuditSink>,
) -> Arc<DbAllocationStore> {
    Arc::new(DbAllocationStore::new(
        conn.clone(),
        audit,
        Arc::new(FixedClock(t0())),
        INSTANCE,
        AllocationLimits::default(),
    ))
}

pub(super) fn expires() -> DateTime<Utc> {
    t0() + Duration::hours(24)
}

/// Fields of a `backup_allocations` row as raw SQL sees them.
#[derive(Debug, Clone, PartialEq, Eq)]
pub(super) struct RawAlloc {
    pub job_id: Option<Uuid>,
    pub storage_key: String,
    pub state: String,
    pub state_version: i64,
    pub epoch: i64,
    pub bytes_reserved: i64,
    pub expires_at: DateTime<Utc>,
    pub orphan_grace_until: Option<DateTime<Utc>>,
    pub cleanup_attempts: i32,
    pub next_attempt_at: Option<DateTime<Utc>>,
    pub last_reason_code: Option<String>,
    pub created_at: DateTime<Utc>,
    pub deleted_at: Option<DateTime<Utc>>,
}

pub(super) async fn raw_alloc(c: &impl ConnectionTrait, id: Uuid) -> Option<RawAlloc> {
    let row = c
        .query_one(pg(
            "SELECT job_id, storage_key, state, state_version, epoch, bytes_reserved, \
             expires_at, orphan_grace_until, cleanup_attempts, next_attempt_at, \
             last_reason_code, created_at, deleted_at \
             FROM backup_allocations WHERE id = $1",
            vec![id.into()],
        ))
        .await
        .expect("allocation query")?;
    Some(RawAlloc {
        job_id: row.try_get("", "job_id").unwrap(),
        storage_key: row.try_get("", "storage_key").unwrap(),
        state: row.try_get("", "state").unwrap(),
        state_version: row.try_get("", "state_version").unwrap(),
        epoch: row.try_get("", "epoch").unwrap(),
        bytes_reserved: row.try_get("", "bytes_reserved").unwrap(),
        expires_at: row.try_get("", "expires_at").unwrap(),
        orphan_grace_until: row.try_get("", "orphan_grace_until").unwrap(),
        cleanup_attempts: row.try_get("", "cleanup_attempts").unwrap(),
        next_attempt_at: row.try_get("", "next_attempt_at").unwrap(),
        last_reason_code: row.try_get("", "last_reason_code").unwrap(),
        created_at: row.try_get("", "created_at").unwrap(),
        deleted_at: row.try_get("", "deleted_at").unwrap(),
    })
}

pub(super) async fn require_alloc(c: &impl ConnectionTrait, id: Uuid) -> RawAlloc {
    let row = raw_alloc(c, id).await;
    assert!(row.is_some(), "allocation row {id} must exist");
    row.unwrap()
}

pub(super) async fn alloc_rows(c: &impl ConnectionTrait) -> i64 {
    count(
        c,
        "SELECT COUNT(*)::bigint AS n FROM backup_allocations",
        vec![],
    )
    .await
}

/// Sum of `bytes_reserved` over every row that is not `deleted`.
pub(super) async fn live_bytes(c: &impl ConnectionTrait) -> i64 {
    count(
        c,
        "SELECT COALESCE(SUM(bytes_reserved), 0)::bigint AS n FROM backup_allocations \
         WHERE state <> 'deleted'",
        vec![],
    )
    .await
}

pub(super) async fn audit_rows_total(c: &impl ConnectionTrait) -> i64 {
    count(c, "SELECT COUNT(*)::bigint AS n FROM backup_audit", vec![]).await
}

/// `(event, outcome)` of every audit row for one allocation, oldest first.
pub(super) async fn audit_events_for_allocation(
    c: &impl ConnectionTrait,
    id: Uuid,
) -> Vec<(String, String)> {
    c.query_all(pg(
        "SELECT event, outcome FROM backup_audit WHERE allocation_id = $1 \
         ORDER BY occurred_at, event",
        vec![id.into()],
    ))
    .await
    .expect("audit query")
    .iter()
    .map(|r| {
        (
            r.try_get::<String>("", "event").unwrap(),
            r.try_get::<String>("", "outcome").unwrap(),
        )
    })
    .collect()
}

/// An allocation row to seed with raw SQL (bypasses the store).
#[derive(Debug, Clone)]
pub(super) struct SeedAlloc {
    pub state: AllocationState,
    pub bytes: u64,
    pub version: i64,
    pub attempts: i32,
    pub next_attempt_at: Option<DateTime<Utc>>,
    pub grace_until: Option<DateTime<Utc>>,
}

impl SeedAlloc {
    pub fn new(state: AllocationState, bytes: u64) -> Self {
        Self {
            state,
            bytes,
            version: 0,
            attempts: 0,
            next_attempt_at: None,
            grace_until: None,
        }
    }
    pub fn version(mut self, v: i64) -> Self {
        self.version = v;
        self
    }
    pub fn attempts(mut self, a: i32) -> Self {
        self.attempts = a;
        self
    }
    pub fn next_attempt_at(mut self, at: DateTime<Utc>) -> Self {
        self.next_attempt_at = Some(at);
        self
    }
    pub fn grace_until(mut self, at: DateTime<Utc>) -> Self {
        self.grace_until = Some(at);
        self
    }
}

pub(super) async fn seed_alloc(c: &impl ConnectionTrait, s: SeedAlloc) -> Uuid {
    let id = Uuid::new_v4();
    let deleted_at = (s.state == AllocationState::Deleted).then(t0);
    exec(
        c,
        "INSERT INTO backup_allocations (id, job_id, storage_key, state, state_version, epoch, \
         bytes_reserved, expires_at, orphan_grace_until, cleanup_attempts, next_attempt_at, \
         created_at, updated_at, deleted_at) \
         VALUES ($1, NULL, $2, $3, $4, 1, $5, $6, $7, $8, $9, $10, $10, $11)",
        vec![
            id.into(),
            format!("alloc/{id}.bin").into(),
            s.state.as_str().into(),
            s.version.into(),
            (s.bytes as i64).into(),
            expires().into(),
            s.grace_until.into(),
            s.attempts.into(),
            s.next_attempt_at.into(),
            t0().into(),
            deleted_at.into(),
        ],
    )
    .await;
    id
}

/// What a `AllocProbeSink` saw on one `record` call.
#[derive(Debug, Clone, PartialEq, Eq)]
pub(super) struct AllocProbe {
    pub saw_tx: bool,
    pub event: AuditEventKind,
    /// The allocation row's state as an independent connection sees it during the call
    /// (`None` = not visible yet, i.e. the row is not committed).
    pub outside_state: Option<String>,
}

/// Spy around the real sink: records whether the store handed it a transaction and what
/// an independent connection sees of the allocation row at that moment, then delegates.
#[derive(Debug)]
pub(super) struct AllocProbeSink {
    inner: DbAuditSink,
    conn: DatabaseConnection,
    pub seen: Arc<Mutex<Vec<AllocProbe>>>,
}

impl AllocProbeSink {
    pub fn new(conn: &DatabaseConnection) -> Self {
        Self {
            inner: DbAuditSink::new(conn.clone()),
            conn: conn.clone(),
            seen: Arc::default(),
        }
    }
}

#[async_trait]
impl AuditSink for AllocProbeSink {
    async fn record(
        &self,
        tx: Option<&DatabaseTransaction>,
        ev: AuditEvent,
    ) -> Result<(), AuditError> {
        let outside_state = match ev.allocation_id {
            Some(id) => self
                .conn
                .query_one(pg(
                    "SELECT state FROM backup_allocations WHERE id = $1",
                    vec![id.into()],
                ))
                .await
                .expect("probe query")
                .map(|r| r.try_get::<String>("", "state").unwrap()),
            None => None,
        };
        self.seen.lock().unwrap().push(AllocProbe {
            saw_tx: tx.is_some(),
            event: ev.event,
            outside_state,
        });
        self.inner.record(tx, ev).await
    }
}

/// Forces the interleaving of two registrations. Each `allocation.registered` audit call
/// (made inside the store's transaction, after its budget check and row insert) waits at a
/// two-party barrier until the other racer has reached its own audit call. A naive
/// check-then-insert reaches it with both racers uncommitted, both having passed the
/// budget check, so both commit. A correctly serialised store (advisory lock, row lock)
/// never lets the second racer in while the first is inside, so the first waits out
/// `PATIENCE`, gives up on the barrier and carries on. Only the audit kind
/// `AllocationRegistered` is held; everything else passes straight through.
#[derive(Debug)]
pub(super) struct RendezvousSink {
    inner: DbAuditSink,
    barrier: Arc<tokio::sync::Barrier>,
    /// Racers that met at the barrier (as opposed to timing out alone).
    pub met: Arc<std::sync::atomic::AtomicUsize>,
}

/// How long a racer waits for its partner. A serialising store pays this once.
pub(super) const PATIENCE: std::time::Duration = std::time::Duration::from_millis(1500);

impl RendezvousSink {
    pub fn new(conn: &DatabaseConnection) -> Self {
        Self {
            inner: DbAuditSink::new(conn.clone()),
            barrier: Arc::new(tokio::sync::Barrier::new(2)),
            met: Arc::default(),
        }
    }
}

#[async_trait]
impl AuditSink for RendezvousSink {
    async fn record(
        &self,
        tx: Option<&DatabaseTransaction>,
        ev: AuditEvent,
    ) -> Result<(), AuditError> {
        if ev.event == AuditEventKind::AllocationRegistered
            && tokio::time::timeout(PATIENCE, self.barrier.wait())
                .await
                .is_ok()
        {
            self.met.fetch_add(1, std::sync::atomic::Ordering::SeqCst);
        }
        self.inner.record(tx, ev).await
    }
}
