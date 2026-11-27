//! Shared wiring for the WU-19 lifecycle tests (sweeper and runtime).
//!
//! Real PostgreSQL stores, a real `FsFixtureStorage` and a real `FsNamespaceLock`, wrapped
//! in thin recording wrappers. Every wrapper call is logged together with whether the
//! runtime's admissions were open at that moment, so an ordering or "closed until the last
//! step" claim is read from the log rather than assumed. One switch (`outage`) makes every
//! wrapped store, storage and audit call fail as an outage would.

use super::alloc_support::INSTANCE;
use super::fs_support::{names_in, TempDir};
use super::support::{count, exec, fx, t0, Fx};
use crate::domain::clock::Clock;
use crate::domain::status::BackupJobStatus;
use crate::infra::db_allocation_store::DbAllocationStore;
use crate::infra::db_audit_sink::DbAuditSink;
use crate::infra::db_job_store::DbJobStore;
use crate::infra::db_retention_store::DbRetentionStore;
use crate::infra::fs_fixture_storage::FsFixtureStorage;
use crate::infra::fs_namespace_lock::FsNamespaceLock;
use crate::lifecycle::cleaner::Cleaner;
use crate::lifecycle::reconciler::{Reconciler, ReconcilerConfig};
use crate::lifecycle::runtime::{Runtime, RuntimeConfig};
use crate::lifecycle::sweeper::{Sweeper, SweeperConfig};
use crate::ports::allocation_store::{
    Allocation, AllocationError, AllocationLimits, AllocationRecoveryRow, AllocationState,
    AllocationStore, AllocationTotals, CleanupFailureOutcome,
};
use crate::ports::audit::{AuditError, AuditEvent, AuditEventKind, AuditSink};
use crate::ports::fixture_storage::{
    DeleteOutcome, FixtureStorage, InventoryPage, StorageError, StorageKey,
};
use crate::ports::job_store::{
    AcceptResult, Components, IdemMatch, IdemRecord, JobRow, JobStore, NewJob, ReasonCode,
    StoreError, TransitionResult,
};
use crate::ports::namespace_lock::NamespaceLock;
use crate::ports::retention_store::{PurgeReport, RetentionStore};
use async_trait::async_trait;
use chrono::{DateTime, Duration, Utc};
use sea_orm::{DatabaseConnection, DatabaseTransaction};
use std::path::{Path, PathBuf};
use std::sync::atomic::{AtomicBool, AtomicUsize, Ordering};
use std::sync::{Arc, Mutex, OnceLock};
use std::time::Duration as StdDuration;
use uuid::Uuid;

#[derive(Debug)]
pub(super) struct TestClock(Mutex<DateTime<Utc>>);

impl TestClock {
    pub fn new(at: DateTime<Utc>) -> Arc<Self> {
        Arc::new(Self(Mutex::new(at)))
    }
    pub fn set(&self, at: DateTime<Utc>) {
        *self.0.lock().unwrap() = at;
    }
}

impl Clock for TestClock {
    fn now(&self) -> DateTime<Utc> {
        *self.0.lock().unwrap()
    }
}

/// Call log, outage switch and the runtime handle the wrappers read admissions from.
#[derive(Debug, Default)]
pub(super) struct Hooks {
    log: Mutex<Vec<String>>,
    pub outage: AtomicBool,
    calls: AtomicUsize,
    pub runtime: OnceLock<Arc<Runtime>>,
    /// Result of probing the namespace lock from inside `advance_epoch`.
    pub lock_held_at_epoch: Mutex<Option<bool>>,
}

impl Hooks {
    fn note(&self, name: &str) {
        let open = self.runtime.get().is_some_and(|r| r.admissions_open());
        self.log
            .lock()
            .unwrap()
            .push(format!("{name}:{}", if open { "open" } else { "closed" }));
        self.calls.fetch_add(1, Ordering::SeqCst);
    }
    fn down(&self) -> bool {
        self.outage.load(Ordering::SeqCst)
    }
    /// Every logged entry, `name:open|closed`.
    pub fn entries(&self) -> Vec<String> {
        self.log.lock().unwrap().clone()
    }
    /// The logged names only, from `mark` on.
    pub fn names_since(&self, mark: usize) -> Vec<String> {
        self.entries()
            .into_iter()
            .skip(mark)
            .map(|e| e.rsplit_once(':').map(|(n, _)| n.to_owned()).unwrap_or(e))
            .collect()
    }
    /// Entries from `mark` on whose admissions were open.
    pub fn open_entries_since(&self, mark: usize) -> Vec<String> {
        self.entries()
            .into_iter()
            .skip(mark)
            .filter(|e| e.ends_with(":open"))
            .collect()
    }
    pub fn mark(&self) -> usize {
        self.log.lock().unwrap().len()
    }
    pub fn calls(&self) -> usize {
        self.calls.load(Ordering::SeqCst)
    }
}

pub(super) struct GuardJobs {
    inner: Arc<dyn JobStore>,
    hooks: Arc<Hooks>,
    lock_root: PathBuf,
}

#[async_trait]
impl JobStore for GuardJobs {
    async fn accept(
        &self,
        tx: &DatabaseTransaction,
        new: NewJob,
        idem: IdemRecord,
    ) -> Result<AcceptResult, StoreError> {
        self.hooks.note("jobs.accept");
        self.inner.accept(tx, new, idem).await
    }
    async fn get(&self, id: Uuid) -> Result<Option<JobRow>, StoreError> {
        self.hooks.note("jobs.get");
        self.inner.get(id).await
    }
    async fn find_idempotency(
        &self,
        actor_sub: &str,
        operation: &str,
        key_digest: &str,
        now: DateTime<Utc>,
    ) -> Result<Option<IdemMatch>, StoreError> {
        self.hooks.note("jobs.find_idempotency");
        self.inner
            .find_idempotency(actor_sub, operation, key_digest, now)
            .await
    }
    async fn delete_expired_idempotency(
        &self,
        tx: &DatabaseTransaction,
        actor_sub: &str,
        operation: &str,
        key_digest: &str,
        now: DateTime<Utc>,
    ) -> Result<(), StoreError> {
        self.hooks.note("jobs.delete_expired_idempotency");
        self.inner
            .delete_expired_idempotency(tx, actor_sub, operation, key_digest, now)
            .await
    }
    async fn transition(
        &self,
        id: Uuid,
        expect: (BackupJobStatus, i64),
        to: BackupJobStatus,
        reason: Option<ReasonCode>,
        comps: Option<Components>,
    ) -> Result<TransitionResult, StoreError> {
        self.hooks.note("jobs.transition");
        if self.hooks.down() {
            return Err(StoreError::StoreUnavailable);
        }
        self.inner.transition(id, expect, to, reason, comps).await
    }
    async fn claim_next(
        &self,
        epoch: i64,
        deadline: StdDuration,
    ) -> Result<Option<JobRow>, StoreError> {
        self.hooks.note("jobs.claim_next");
        self.inner.claim_next(epoch, deadline).await
    }
    async fn advance_epoch(&self, now: DateTime<Utc>) -> Result<i64, StoreError> {
        self.hooks.note("jobs.advance_epoch");
        if self.hooks.down() {
            return Err(StoreError::StoreUnavailable);
        }
        // Prove the lock is already held: a second acquisition from here must fail.
        let held = matches!(
            FsNamespaceLock::try_acquire(&self.lock_root),
            Err(crate::ports::namespace_lock::LockError::AlreadyHeld)
        );
        *self.hooks.lock_held_at_epoch.lock().unwrap() = Some(held);
        self.inner.advance_epoch(now).await
    }
    async fn list_for_reconciliation(&self) -> Result<Vec<JobRow>, StoreError> {
        self.hooks.note("jobs.list_for_reconciliation");
        if self.hooks.down() {
            return Err(StoreError::StoreUnavailable);
        }
        self.inner.list_for_reconciliation().await
    }
    #[allow(clippy::too_many_arguments)]
    async fn fence_job(
        &self,
        id: Uuid,
        expected_epoch: i64,
        expected_status: BackupJobStatus,
        expected_state_version: i64,
        target_status: BackupJobStatus,
        reason: ReasonCode,
        now: DateTime<Utc>,
    ) -> Result<TransitionResult, StoreError> {
        self.hooks.note("jobs.fence_job");
        if self.hooks.down() {
            return Err(StoreError::StoreUnavailable);
        }
        self.inner
            .fence_job(
                id,
                expected_epoch,
                expected_status,
                expected_state_version,
                target_status,
                reason,
                now,
            )
            .await
    }
    async fn list_deadline_expired(
        &self,
        now: DateTime<Utc>,
        epoch: i64,
    ) -> Result<Vec<JobRow>, StoreError> {
        self.hooks.note("jobs.list_deadline_expired");
        if self.hooks.down() {
            return Err(StoreError::StoreUnavailable);
        }
        self.inner.list_deadline_expired(now, epoch).await
    }
}

pub(super) struct GuardAllocs {
    inner: Arc<dyn AllocationStore>,
    hooks: Arc<Hooks>,
}

impl GuardAllocs {
    fn gate(&self, name: &str) -> Result<(), AllocationError> {
        self.hooks.note(name);
        if self.hooks.down() {
            Err(AllocationError::StoreUnavailable)
        } else {
            Ok(())
        }
    }
}

#[async_trait]
impl AllocationStore for GuardAllocs {
    async fn register(
        &self,
        job: Option<Uuid>,
        bytes: u64,
        epoch: i64,
        expires: DateTime<Utc>,
    ) -> Result<Allocation, AllocationError> {
        self.gate("allocs.register")?;
        self.inner.register(job, bytes, epoch, expires).await
    }
    async fn mark_written(&self, id: Uuid, ver: i64) -> Result<(), AllocationError> {
        self.gate("allocs.mark_written")?;
        self.inner.mark_written(id, ver).await
    }
    async fn release(&self, id: Uuid, ver: i64) -> Result<(), AllocationError> {
        self.gate("allocs.release")?;
        self.inner.release(id, ver).await
    }
    async fn claim_for_delete(
        &self,
        id: Uuid,
        ver: i64,
    ) -> Result<Option<Allocation>, AllocationError> {
        self.gate("allocs.claim_for_delete")?;
        self.inner.claim_for_delete(id, ver).await
    }
    async fn mark_deleted(&self, id: Uuid, ver: i64) -> Result<(), AllocationError> {
        self.gate("allocs.mark_deleted")?;
        self.inner.mark_deleted(id, ver).await
    }
    async fn record_cleanup_failure(
        &self,
        id: Uuid,
        ver: i64,
        reason: ReasonCode,
        now: DateTime<Utc>,
    ) -> Result<CleanupFailureOutcome, AllocationError> {
        self.gate("allocs.record_cleanup_failure")?;
        self.inner
            .record_cleanup_failure(id, ver, reason, now)
            .await
    }
    async fn insert_orphan(
        &self,
        id: Uuid,
        key: &StorageKey,
        bytes: u64,
        now: DateTime<Utc>,
    ) -> Result<(), AllocationError> {
        self.gate("allocs.insert_orphan")?;
        self.inner.insert_orphan(id, key, bytes, now).await
    }
    async fn due_for_cleanup(
        &self,
        now: DateTime<Utc>,
    ) -> Result<Vec<Allocation>, AllocationError> {
        self.gate("allocs.due_for_cleanup")?;
        self.inner.due_for_cleanup(now).await
    }
    async fn totals(&self) -> Result<AllocationTotals, AllocationError> {
        self.gate("allocs.totals")?;
        self.inner.totals().await
    }
    async fn get(&self, id: Uuid) -> Result<Option<Allocation>, AllocationError> {
        self.gate("allocs.get")?;
        self.inner.get(id).await
    }
    async fn list_for_reconciliation(&self) -> Result<Vec<AllocationRecoveryRow>, AllocationError> {
        self.gate("allocs.list_for_reconciliation")?;
        self.inner.list_for_reconciliation().await
    }
    async fn reconcile_state(
        &self,
        id: Uuid,
        expected: (AllocationState, i64, i64),
        to: AllocationState,
        now: DateTime<Utc>,
    ) -> Result<TransitionResult, AllocationError> {
        self.gate("allocs.reconcile_state")?;
        self.inner.reconcile_state(id, expected, to, now).await
    }
}

pub(super) struct GuardStorage {
    inner: Arc<dyn FixtureStorage>,
    hooks: Arc<Hooks>,
}

#[async_trait]
impl FixtureStorage for GuardStorage {
    async fn write(&self, key: &StorageKey, bytes: &[u8]) -> Result<(), StorageError> {
        self.hooks.note("storage.write");
        self.inner.write(key, bytes).await
    }
    async fn delete(&self, key: &StorageKey) -> Result<DeleteOutcome, StorageError> {
        self.hooks.note("storage.delete");
        if self.hooks.down() {
            return Err(StorageError::Unavailable);
        }
        self.inner.delete(key).await
    }
    async fn inventory(&self, max_entries: usize) -> Result<InventoryPage, StorageError> {
        self.hooks.note("storage.inventory");
        if self.hooks.down() {
            return Err(StorageError::Unavailable);
        }
        self.inner.inventory(max_entries).await
    }
}

/// Real audit sink that logs events, can refuse one kind and fails everything in an outage.
#[derive(Debug)]
pub(super) struct HookAudit {
    inner: DbAuditSink,
    hooks: Arc<Hooks>,
    fail: Mutex<Option<AuditEventKind>>,
}

impl HookAudit {
    pub fn fail_on(&self, kind: Option<AuditEventKind>) {
        *self.fail.lock().unwrap() = kind;
    }
}

#[async_trait]
impl AuditSink for HookAudit {
    async fn record(
        &self,
        tx: Option<&DatabaseTransaction>,
        ev: AuditEvent,
    ) -> Result<(), AuditError> {
        if self.hooks.down() {
            return Err(AuditError::Unavailable);
        }
        if *self.fail.lock().unwrap() == Some(ev.event) {
            return Err(AuditError::Rejected);
        }
        let name = format!("audit.{}", ev.event.as_str());
        self.inner.record(tx, ev).await?;
        self.hooks.note(&name);
        Ok(())
    }
}

pub(super) struct GuardRetention {
    inner: Arc<dyn RetentionStore>,
    hooks: Arc<Hooks>,
}

#[async_trait]
impl RetentionStore for GuardRetention {
    async fn purge_retained(&self, now: DateTime<Utc>) -> Result<PurgeReport, AllocationError> {
        self.hooks.note("retention.purge_retained");
        if self.hooks.down() {
            return Err(AllocationError::StoreUnavailable);
        }
        self.inner.purge_retained(now).await
    }
}

pub(super) struct LifeRig {
    pub fx: Fx,
    pub tmp: TempDir,
    pub clock: Arc<TestClock>,
    pub hooks: Arc<Hooks>,
    pub audit: Arc<HookAudit>,
    /// Unguarded job store, for the test's own worker-side transitions.
    pub raw_jobs: Arc<DbJobStore>,
    pub reconciler: Arc<Reconciler>,
    pub cleaner: Arc<Cleaner>,
    pub sweeper: Arc<Sweeper>,
    pub runtime: Arc<Runtime>,
}

/// `cap`: the namespace inventory bound the reconciler uses.
pub(super) async fn life_rig(cap: usize) -> LifeRig {
    let fx = fx().await;
    let tmp = TempDir::new("wu19");
    let clock = TestClock::new(t0());
    let hooks = Arc::new(Hooks::default());
    let audit = Arc::new(HookAudit {
        inner: DbAuditSink::new(fx.conn.clone()),
        hooks: hooks.clone(),
        fail: Mutex::new(None),
    });
    let raw_jobs = Arc::new(DbJobStore::new(
        fx.conn.clone(),
        audit.clone(),
        clock.clone(),
    ));
    let real_allocs: Arc<dyn AllocationStore> = Arc::new(DbAllocationStore::new(
        fx.conn.clone(),
        audit.clone(),
        clock.clone(),
        INSTANCE,
        AllocationLimits::default(),
    ));
    let real_storage: Arc<dyn FixtureStorage> =
        Arc::new(FsFixtureStorage::open(&tmp.root(), real_allocs.clone()).expect("open storage"));
    let real_retention: Arc<dyn RetentionStore> = Arc::new(DbRetentionStore::new(
        fx.conn.clone(),
        audit.clone(),
        INSTANCE,
    ));
    let jobs: Arc<dyn JobStore> = Arc::new(GuardJobs {
        inner: raw_jobs.clone(),
        hooks: hooks.clone(),
        lock_root: tmp.root(),
    });
    let allocs: Arc<dyn AllocationStore> = Arc::new(GuardAllocs {
        inner: real_allocs,
        hooks: hooks.clone(),
    });
    let storage: Arc<dyn FixtureStorage> = Arc::new(GuardStorage {
        inner: real_storage,
        hooks: hooks.clone(),
    });
    let retention: Arc<dyn RetentionStore> = Arc::new(GuardRetention {
        inner: real_retention,
        hooks: hooks.clone(),
    });
    let reconciler = Arc::new(Reconciler::new(
        jobs.clone(),
        allocs.clone(),
        storage.clone(),
        audit.clone(),
        ReconcilerConfig::new(INSTANCE).with_inventory_cap(cap),
    ));
    let cleaner = Arc::new(Cleaner::new(allocs.clone(), storage));
    let sweeper = Arc::new(Sweeper::new(
        jobs,
        allocs,
        retention,
        reconciler.clone(),
        SweeperConfig::default(),
    ));
    let runtime = Arc::new(Runtime::new(
        reconciler.clone(),
        cleaner.clone(),
        sweeper.clone(),
        Arc::new(|root: &Path| FsNamespaceLock::try_acquire(root)),
        clock.clone(),
        RuntimeConfig::new(tmp.root()),
    ));
    hooks.runtime.set(runtime.clone()).ok();
    LifeRig {
        fx,
        tmp,
        clock,
        hooks,
        audit,
        raw_jobs,
        reconciler,
        cleaner,
        sweeper,
        runtime,
    }
}

impl LifeRig {
    pub fn conn(&self) -> &DatabaseConnection {
        &self.fx.conn
    }
    fn alloc_dir(&self) -> PathBuf {
        self.tmp.root().join("alloc")
    }
    pub fn put_file(&self, id: Uuid) {
        std::fs::write(
            self.alloc_dir().join(format!("{id}.bin")),
            b"synthetic fixture bytes",
        )
        .expect("write file");
    }
    pub fn has_file(&self, id: Uuid) -> bool {
        self.alloc_dir().join(format!("{id}.bin")).exists()
    }
    pub fn listing(&self) -> Vec<String> {
        names_in(&self.alloc_dir())
    }
    pub async fn audit_total(&self) -> i64 {
        count(
            self.conn(),
            "SELECT COUNT(*)::bigint AS n FROM backup_audit",
            vec![],
        )
        .await
    }
    pub async fn epoch_row(&self) -> Option<i64> {
        let rows = count(
            self.conn(),
            "SELECT COUNT(*)::bigint AS n FROM backup_epoch",
            vec![],
        )
        .await;
        if rows == 0 {
            return None;
        }
        Some(
            count(
                self.conn(),
                "SELECT epoch::bigint AS n FROM backup_epoch WHERE id = 1",
                vec![],
            )
            .await,
        )
    }
    /// Set a seeded job's deadline.
    pub async fn set_deadline(&self, job: Uuid, at: DateTime<Utc>) {
        exec(
            self.conn(),
            "UPDATE backup_jobs SET deadline_at = $1 WHERE id = $2",
            vec![at.into(), job.into()],
        )
        .await;
    }
    pub async fn set_job_epoch(&self, job: Uuid, epoch: i64) {
        exec(
            self.conn(),
            "UPDATE backup_jobs SET epoch = $1 WHERE id = $2",
            vec![epoch.into(), job.into()],
        )
        .await;
    }
}

pub(super) fn after(seconds: i64) -> DateTime<Utc> {
    t0() + Duration::seconds(seconds)
}
