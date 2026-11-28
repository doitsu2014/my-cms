//! WU-18 red tests, PostgreSQL-backed: the `Cleaner` over `DbAllocationStore`,
//! `DbAuditSink` and a real `FsFixtureStorage` under a temp root (TS-26, TS-27a, the
//! backoff and grace boundaries of TS-30, the claim and deleted paths of TS-65, the
//! delete-claim ordering of TS-44, and the cleaner half of TS-20, TS-21, TS-23 and TS-25).
//!
//! The clock is a settable `TestClock` shared by the allocation store and the test, so a
//! backoff boundary is crossed by moving the clock, never by a hard-coded outcome. Unlink
//! failures are injected by a `FlakyStorage` that wraps the real storage. At every unlink
//! the wrapper reads the database and records how many `allocation.delete_claimed` rows
//! were already committed and what state the row was in: an unlink without its own prior
//! audited claim is therefore visible in the assertions.

use super::alloc_support::{audit_rows_for_allocation, raw_alloc, seed_alloc, SeedAlloc, INSTANCE};
use super::fs_support::TempDir;
use super::support::{count, exec, fx, raw_job, seed, t0, Fx, Seed};
use crate::domain::clock::Clock;
use crate::domain::status::BackupJobStatus;
use crate::infra::db_allocation_store::DbAllocationStore;
use crate::infra::db_audit_sink::DbAuditSink;
use crate::infra::db_job_store::DbJobStore;
use crate::infra::fs_fixture_storage::FsFixtureStorage;
use crate::lifecycle::cleaner::{CleanError, CleanOutcome, Cleaner};
use crate::lifecycle::reconciler::{Reconciler, ReconcilerConfig};
use crate::ports::allocation_store::{AllocationLimits, AllocationState, AllocationStore};
use crate::ports::audit::{AuditError, AuditEvent, AuditEventKind, AuditSink};
use crate::ports::fixture_storage::{
    DeleteOutcome, FixtureStorage, InventoryPage, StorageError, StorageKey,
};
use crate::ports::job_store::JobStore;
use async_trait::async_trait;
use chrono::{DateTime, Duration, Utc};
use sea_orm::{DatabaseConnection, DatabaseTransaction};
use std::sync::atomic::{AtomicBool, AtomicUsize, Ordering};
use std::sync::{Arc, Mutex};
use uuid::Uuid;

#[derive(Debug)]
struct TestClock(Mutex<DateTime<Utc>>);

impl TestClock {
    fn new(at: DateTime<Utc>) -> Arc<Self> {
        Arc::new(Self(Mutex::new(at)))
    }
    fn set(&self, at: DateTime<Utc>) {
        *self.0.lock().unwrap() = at;
    }
}

impl Clock for TestClock {
    fn now(&self) -> DateTime<Utc> {
        *self.0.lock().unwrap()
    }
}

/// Real audit sink that records the order of events and can be told to refuse one kind.
#[derive(Debug)]
struct SeqSink {
    inner: DbAuditSink,
    fail: Mutex<Option<AuditEventKind>>,
    seen: Mutex<Vec<AuditEventKind>>,
}

impl SeqSink {
    fn new(conn: &DatabaseConnection) -> Arc<Self> {
        Arc::new(Self {
            inner: DbAuditSink::new(conn.clone()),
            fail: Mutex::new(None),
            seen: Mutex::new(Vec::new()),
        })
    }
    fn fail_on(&self, kind: Option<AuditEventKind>) {
        *self.fail.lock().unwrap() = kind;
    }
    fn events(&self) -> Vec<&'static str> {
        self.seen
            .lock()
            .unwrap()
            .iter()
            .map(|k| k.as_str())
            .collect()
    }
}

#[async_trait]
impl AuditSink for SeqSink {
    async fn record(
        &self,
        tx: Option<&DatabaseTransaction>,
        ev: AuditEvent,
    ) -> Result<(), AuditError> {
        if *self.fail.lock().unwrap() == Some(ev.event) {
            return Err(AuditError::Rejected);
        }
        let kind = ev.event;
        self.inner.record(tx, ev).await?;
        self.seen.lock().unwrap().push(kind);
        Ok(())
    }
}

/// What the database said at the moment an unlink was attempted.
#[derive(Debug, Clone, PartialEq, Eq)]
struct UnlinkProbe {
    claims_audited: i64,
    state: String,
}

/// The real storage, with injected unlink failures and a probe at every unlink.
struct FlakyStorage {
    inner: Arc<dyn FixtureStorage>,
    conn: DatabaseConnection,
    fail_remaining: AtomicUsize,
    /// When set, another writer bumps the row's `state_version` during the unlink.
    bump_during_unlink: AtomicBool,
    probes: Mutex<Vec<UnlinkProbe>>,
}

#[async_trait]
impl FixtureStorage for FlakyStorage {
    async fn write(&self, key: &StorageKey, bytes: &[u8]) -> Result<(), StorageError> {
        self.inner.write(key, bytes).await
    }
    async fn delete(&self, key: &StorageKey) -> Result<DeleteOutcome, StorageError> {
        let id = key.allocation_id();
        let claims_audited = count(
            &self.conn,
            "SELECT COUNT(*)::bigint AS n FROM backup_audit \
             WHERE allocation_id = $1 AND event = 'allocation.delete_claimed'",
            vec![id.into()],
        )
        .await;
        let state = raw_alloc(&self.conn, id)
            .await
            .map(|a| a.state)
            .unwrap_or_default();
        self.probes.lock().unwrap().push(UnlinkProbe {
            claims_audited,
            state,
        });
        if self.fail_remaining.load(Ordering::SeqCst) > 0 {
            self.fail_remaining.fetch_sub(1, Ordering::SeqCst);
            return Err(StorageError::Unavailable);
        }
        let removed = self.inner.delete(key).await;
        if self.bump_during_unlink.load(Ordering::SeqCst) {
            exec(
                &self.conn,
                "UPDATE backup_allocations SET state_version = state_version + 1 WHERE id = $1",
                vec![id.into()],
            )
            .await;
        }
        removed
    }
    async fn inventory(&self, max_entries: usize) -> Result<InventoryPage, StorageError> {
        self.inner.inventory(max_entries).await
    }
}

struct Rig {
    fx: Fx,
    tmp: TempDir,
    clock: Arc<TestClock>,
    seq: Arc<SeqSink>,
    storage: Arc<FlakyStorage>,
    cleaner: Cleaner,
    rec: Reconciler,
}

/// `delete_failures`: how many unlink attempts fail before the real unlink is used.
async fn rig(delete_failures: usize) -> Rig {
    let fx = fx().await;
    let tmp = TempDir::new("wu18");
    let clock = TestClock::new(t0());
    let seq = SeqSink::new(&fx.conn);
    let jobs: Arc<dyn JobStore> =
        Arc::new(DbJobStore::new(fx.conn.clone(), seq.clone(), clock.clone()));
    let allocs: Arc<dyn AllocationStore> = Arc::new(DbAllocationStore::new(
        fx.conn.clone(),
        seq.clone(),
        clock.clone(),
        INSTANCE,
        AllocationLimits::default(),
    ));
    let real: Arc<dyn FixtureStorage> =
        Arc::new(FsFixtureStorage::open(&tmp.root(), allocs.clone()).expect("open storage"));
    let storage = Arc::new(FlakyStorage {
        inner: real,
        conn: fx.conn.clone(),
        fail_remaining: AtomicUsize::new(delete_failures),
        bump_during_unlink: AtomicBool::new(false),
        probes: Mutex::new(Vec::new()),
    });
    let cleaner = Cleaner::new(allocs.clone(), storage.clone());
    let rec = Reconciler::new(
        jobs,
        allocs,
        storage.clone(),
        seq.clone(),
        ReconcilerConfig::new(INSTANCE),
    );
    Rig {
        fx,
        tmp,
        clock,
        seq,
        storage,
        cleaner,
        rec,
    }
}

impl Rig {
    fn put_file(&self, id: Uuid) {
        let path = self.tmp.root().join("alloc").join(format!("{id}.bin"));
        std::fs::write(path, b"synthetic fixture bytes").expect("write file");
    }
    fn has_file(&self, id: Uuid) -> bool {
        self.tmp
            .root()
            .join("alloc")
            .join(format!("{id}.bin"))
            .exists()
    }
    fn probes(&self) -> Vec<UnlinkProbe> {
        self.storage.probes.lock().unwrap().clone()
    }
    async fn events_for(&self, alloc: Uuid) -> Vec<String> {
        audit_rows_for_allocation(&self.fx.conn, alloc)
            .await
            .into_iter()
            .map(|r| r.event)
            .collect()
    }
    async fn audit_total(&self) -> i64 {
        count(
            &self.fx.conn,
            "SELECT COUNT(*)::bigint AS n FROM backup_audit",
            vec![],
        )
        .await
    }
    /// Two epoch advances: the first stands for the previous run (epoch 1, which every
    /// seeded row carries), the second for the restart under test.
    async fn restart(&self) -> i64 {
        assert_eq!(self.rec.advance_epoch(t0()).await, Ok(1));
        self.rec.advance_epoch(t0()).await.expect("restart epoch")
    }
}

fn after(seconds: i64) -> DateTime<Utc> {
    t0() + Duration::seconds(seconds)
}

fn probe(claims_audited: i64) -> UnlinkProbe {
    UnlinkProbe {
        claims_audited,
        state: "deleting".to_owned(),
    }
}

// ------------------------------------------------------------------ TS-26

/// Window 7: the file is unlinked after the audited claim, then the audited `deleted`
/// commit fails. The row must stay `deleting`; the restart must claim again before it
/// retries the unlink of the already missing file and commits `deleted`.
#[tokio::test]
#[ignore = "requires Docker for PostgreSQL testcontainer"]
async fn ts26_failed_deleted_commit_after_unlink_leaves_deleting_and_restart_recovers_idempotently()
{
    let r = rig(0).await;
    assert_eq!(r.rec.advance_epoch(t0()).await, Ok(1));
    let alloc = seed_alloc(
        &r.fx.conn,
        SeedAlloc::new(AllocationState::ReleasePending, 100).version(2),
    )
    .await;
    r.put_file(alloc);
    r.seq.fail_on(Some(AuditEventKind::AllocationDeleted));

    let failed = r.cleaner.clean_one(alloc, t0()).await;

    assert_eq!(failed, Err(CleanError::AuditUnavailable));
    let a = raw_alloc(&r.fx.conn, alloc).await.unwrap();
    assert_eq!(
        (a.state.as_str(), a.state_version, a.cleanup_attempts),
        ("deleting", 3, 0),
        "the claim committed; the deleted commit rolled back; no retry was consumed"
    );
    assert!(!r.has_file(alloc), "the unlink really happened");
    assert_eq!(r.events_for(alloc).await, vec!["allocation.delete_claimed"]);
    assert_eq!(r.probes(), vec![probe(1)]);

    // Restart: a new epoch, reconciliation resets the stale claim, a healthy sink.
    r.seq.fail_on(None);
    let epoch = r.rec.advance_epoch(t0()).await.expect("restart epoch");
    assert_eq!(epoch, 2);
    r.rec.reconcile(epoch, t0()).await.expect("reconcile");
    let reset = raw_alloc(&r.fx.conn, alloc).await.unwrap();
    assert_eq!(
        (reset.state.as_str(), reset.cleanup_attempts),
        ("release_pending", 0)
    );

    let recovered = r.cleaner.clean_one(alloc, t0()).await;

    assert_eq!(recovered, Ok(CleanOutcome::Deleted));
    let done = raw_alloc(&r.fx.conn, alloc).await.unwrap();
    assert_eq!(
        (done.state.as_str(), done.cleanup_attempts, done.deleted_at),
        ("deleted", 0, Some(t0())),
        "retry count unchanged; deleted_at committed"
    );
    assert_eq!(
        r.probes(),
        vec![probe(1), probe(2)],
        "the retry had its own audited claim before its unlink"
    );
    assert_eq!(
        r.seq.events(),
        vec![
            "allocation.delete_claimed",
            "allocation.reconciled",
            "allocation.delete_claimed",
            "allocation.deleted"
        ]
    );
}

/// A missing file counts as removed: the claim is audited, the unlink reports it absent,
/// and `deleted` commits with its own audit row.
#[tokio::test]
#[ignore = "requires Docker for PostgreSQL testcontainer"]
async fn ts26_a_missing_file_counts_as_removed_and_commits_deleted_with_audit() {
    let r = rig(0).await;
    let job = seed(
        &r.fx.conn,
        Seed::new("ts26-missing-instance", BackupJobStatus::Failed),
    )
    .await;
    let alloc = seed_alloc(
        &r.fx.conn,
        SeedAlloc::new(AllocationState::ReleasePending, 100)
            .job_id(job)
            .version(2),
    )
    .await;

    let outcome = r.cleaner.clean_one(alloc, t0()).await;

    assert_eq!(outcome, Ok(CleanOutcome::Deleted));
    let a = raw_alloc(&r.fx.conn, alloc).await.unwrap();
    assert_eq!(
        (a.state.as_str(), a.state_version, a.deleted_at),
        ("deleted", 4, Some(t0()))
    );
    let rows = audit_rows_for_allocation(&r.fx.conn, alloc).await;
    let mut events: Vec<_> = rows.iter().map(|x| x.event.as_str()).collect();
    events.sort();
    assert_eq!(
        events,
        vec!["allocation.delete_claimed", "allocation.deleted"]
    );
    for row in &rows {
        assert_eq!(row.outcome, "success");
        assert_eq!(row.job_id, Some(job));
        assert_eq!(row.instance_id, INSTANCE);
        assert_eq!(row.actor_sub, None);
    }
    assert_eq!(r.probes(), vec![probe(1)]);
}

// ------------------------------------------------------------------ TS-27a

/// Window 8: three real unlink failures spaced by the clock; exactly three attempts.
#[tokio::test]
#[ignore = "requires Docker for PostgreSQL testcontainer"]
async fn ts27a_three_unlink_failures_are_spaced_exhausted_and_never_attempted_a_fourth_time() {
    let r = rig(usize::MAX / 2).await;
    let alloc = seed_alloc(
        &r.fx.conn,
        SeedAlloc::new(AllocationState::ReleasePending, 100).version(2),
    )
    .await;
    r.put_file(alloc);

    // Attempt 1 at t0.
    assert_eq!(
        r.cleaner.clean_due(t0()).await,
        Ok(vec![CleanOutcome::RetryScheduled])
    );
    let a1 = raw_alloc(&r.fx.conn, alloc).await.unwrap();
    assert_eq!(
        (a1.state.as_str(), a1.cleanup_attempts, a1.next_attempt_at),
        ("release_pending", 1, Some(after(60)))
    );
    assert_eq!(a1.last_reason_code.as_deref(), Some("ADAPTER_ERROR"));

    // 59 s later: not due, no unlink.
    r.clock.set(after(59));
    assert_eq!(r.cleaner.clean_due(after(59)).await, Ok(vec![]));
    assert_eq!(r.probes().len(), 1);

    // Attempt 2 at exactly 60 s.
    r.clock.set(after(60));
    assert_eq!(
        r.cleaner.clean_due(after(60)).await,
        Ok(vec![CleanOutcome::RetryScheduled])
    );
    let a2 = raw_alloc(&r.fx.conn, alloc).await.unwrap();
    assert_eq!(
        (a2.cleanup_attempts, a2.next_attempt_at),
        (2, Some(after(120)))
    );

    // Attempt 3 at exactly 120 s exhausts.
    r.clock.set(after(120));
    assert_eq!(
        r.cleaner.clean_due(after(120)).await,
        Ok(vec![CleanOutcome::Exhausted])
    );
    let a3 = raw_alloc(&r.fx.conn, alloc).await.unwrap();
    assert_eq!(
        (a3.state.as_str(), a3.cleanup_attempts, a3.next_attempt_at),
        ("cleanup_failed", 3, None)
    );
    let exhausted = audit_rows_for_allocation(&r.fx.conn, alloc)
        .await
        .into_iter()
        .filter(|x| x.event == "cleanup.exhausted")
        .count();
    assert_eq!(exhausted, 1, "one persisted cleanup.exhausted row");

    // An hour later: still exactly three unlink attempts, three audited claims.
    r.clock.set(after(3600));
    assert_eq!(r.cleaner.clean_due(after(3600)).await, Ok(vec![]));
    assert_eq!(
        r.probes(),
        vec![probe(1), probe(2), probe(3)],
        "three attempts, each after its own audited claim, and no fourth"
    );
    assert!(r.has_file(alloc), "the file stays for the owner to inspect");
    assert_eq!(
        count(
            &r.fx.conn,
            "SELECT COUNT(*)::bigint AS n FROM backup_allocations WHERE state = 'cleanup_failed'",
            vec![]
        )
        .await,
        1,
        "the escalation row is still visible"
    );
}

/// One failure then success: the attempt count stays at 1 and `deleted` commits.
#[tokio::test]
#[ignore = "requires Docker for PostgreSQL testcontainer"]
async fn ts27a_a_retry_that_succeeds_commits_deleted_and_keeps_the_attempt_count() {
    let r = rig(1).await;
    let alloc = seed_alloc(
        &r.fx.conn,
        SeedAlloc::new(AllocationState::ReleasePending, 100).version(2),
    )
    .await;
    r.put_file(alloc);

    assert_eq!(
        r.cleaner.clean_one(alloc, t0()).await,
        Ok(CleanOutcome::RetryScheduled)
    );
    r.clock.set(after(60));
    assert_eq!(
        r.cleaner.clean_one(alloc, after(60)).await,
        Ok(CleanOutcome::Deleted)
    );

    let a = raw_alloc(&r.fx.conn, alloc).await.unwrap();
    assert_eq!((a.state.as_str(), a.cleanup_attempts), ("deleted", 1));
    assert!(!r.has_file(alloc));
    assert_eq!(r.probes(), vec![probe(1), probe(2)]);
}

// ------------------------------------------------------------------ TS-30

/// Backoff boundaries: just before, at and just after the one-minute retry time.
#[tokio::test]
#[ignore = "requires Docker for PostgreSQL testcontainer"]
async fn ts30_retry_is_due_exactly_at_its_backoff_time_and_not_before() {
    let r = rig(0).await;
    let at_boundary = seed_alloc(
        &r.fx.conn,
        SeedAlloc::new(AllocationState::ReleasePending, 100)
            .version(2)
            .attempts(1)
            .next_attempt_at(after(60)),
    )
    .await;
    let just_after = seed_alloc(
        &r.fx.conn,
        SeedAlloc::new(AllocationState::ReleasePending, 100)
            .version(2)
            .attempts(1)
            .next_attempt_at(after(60)),
    )
    .await;
    r.put_file(at_boundary);
    r.put_file(just_after);

    r.clock.set(after(59));
    assert_eq!(
        r.cleaner.clean_one(at_boundary, after(59)).await,
        Ok(CleanOutcome::NotDue),
        "one second before the backoff ends"
    );
    assert!(r.has_file(at_boundary));
    assert_eq!(r.probes(), vec![], "no unlink before the boundary");
    assert_eq!(r.audit_total().await, 0, "no claim before the boundary");

    r.clock.set(after(60));
    assert_eq!(
        r.cleaner.clean_one(at_boundary, after(60)).await,
        Ok(CleanOutcome::Deleted),
        "due exactly at next_attempt_at"
    );
    r.clock.set(after(61));
    assert_eq!(
        r.cleaner.clean_one(just_after, after(61)).await,
        Ok(CleanOutcome::Deleted),
        "still due one second after"
    );
    assert!(!r.has_file(at_boundary) && !r.has_file(just_after));
}

/// Orphan grace (TS-23, TS-30): removed only at or after the grace, behind an audited claim.
#[tokio::test]
#[ignore = "requires Docker for PostgreSQL testcontainer"]
async fn ts23_orphan_is_removed_only_at_or_after_its_grace_and_after_an_audited_claim() {
    let r = rig(0).await;
    let grace = t0() + Duration::hours(1);
    let orphan = seed_alloc(
        &r.fx.conn,
        SeedAlloc::new(AllocationState::OrphanDiscovered, 100).grace_until(grace),
    )
    .await;
    r.put_file(orphan);

    let before = grace - Duration::seconds(1);
    r.clock.set(before);
    assert_eq!(r.cleaner.clean_due(before).await, Ok(vec![]));
    assert_eq!(
        r.cleaner.clean_one(orphan, before).await,
        Ok(CleanOutcome::NotDue),
        "a direct request before the grace is not claimed either"
    );
    assert!(
        r.has_file(orphan),
        "the orphan is kept until its grace ends"
    );
    assert_eq!(r.probes(), vec![]);

    r.clock.set(grace);
    assert_eq!(
        r.cleaner.clean_due(grace).await,
        Ok(vec![CleanOutcome::Deleted])
    );
    assert!(!r.has_file(orphan));
    assert_eq!(r.probes(), vec![probe(1)]);
    assert_eq!(
        raw_alloc(&r.fx.conn, orphan).await.unwrap().state,
        "deleted"
    );
}

// ------------------------------------------------------------------ TS-65

/// The claim audit is refused: no `deleting` row, no audit row, and above all no unlink.
#[tokio::test]
#[ignore = "requires Docker for PostgreSQL testcontainer"]
async fn ts65_a_refused_claim_audit_blocks_the_claim_and_the_unlink() {
    let r = rig(0).await;
    let alloc = seed_alloc(
        &r.fx.conn,
        SeedAlloc::new(AllocationState::ReleasePending, 100).version(2),
    )
    .await;
    r.put_file(alloc);
    let before = raw_alloc(&r.fx.conn, alloc).await.unwrap();
    r.seq.fail_on(Some(AuditEventKind::AllocationDeleteClaimed));

    let outcome = r.cleaner.clean_one(alloc, t0()).await;

    assert_eq!(outcome, Err(CleanError::AuditUnavailable));
    assert_eq!(raw_alloc(&r.fx.conn, alloc).await.unwrap(), before);
    assert_eq!(r.audit_total().await, 0);
    assert!(r.has_file(alloc), "no unlink without a durable claim audit");
    assert_eq!(r.probes(), vec![]);
}

// --------------------------------------- the cleaner half of TS-20, TS-21 and TS-25

/// TS-20 complete: fence, reconcile, claim, unlink of a file that never existed, deleted.
#[tokio::test]
#[ignore = "requires Docker for PostgreSQL testcontainer"]
async fn ts20_full_sequence_fence_reconcile_claim_then_deleted_for_a_missing_file() {
    let r = rig(0).await;
    let epoch = r.restart().await;
    let job = seed(
        &r.fx.conn,
        Seed::new("ts20-full-instance", BackupJobStatus::Running).version(4),
    )
    .await;
    let alloc = seed_alloc(
        &r.fx.conn,
        SeedAlloc::new(AllocationState::Registered, 100)
            .job_id(job)
            .version(2),
    )
    .await;

    r.rec.reconcile(epoch, t0()).await.expect("reconcile");
    let outcomes = r.cleaner.clean_due(t0()).await;

    assert_eq!(outcomes, Ok(vec![CleanOutcome::Deleted]));
    assert_eq!(
        r.seq.events(),
        vec![
            "reconcile.fenced",
            "allocation.reconciled",
            "allocation.delete_claimed",
            "allocation.deleted"
        ]
    );
    assert_eq!(raw_job(&r.fx.conn, job).await.unwrap().status, "failed");
    let a = raw_alloc(&r.fx.conn, alloc).await.unwrap();
    assert_eq!((a.state.as_str(), a.state_version), ("deleted", 5));
    assert_eq!(r.probes(), vec![probe(1)]);
}

/// TS-21 complete: the file exists, so it is removed, but only behind the audited claim.
#[tokio::test]
#[ignore = "requires Docker for PostgreSQL testcontainer"]
async fn ts21_full_sequence_an_existing_file_is_removed_only_after_the_audited_claim() {
    let r = rig(0).await;
    let epoch = r.restart().await;
    let job = seed(
        &r.fx.conn,
        Seed::new("ts21-full-instance", BackupJobStatus::Running).version(4),
    )
    .await;
    let alloc = seed_alloc(
        &r.fx.conn,
        SeedAlloc::new(AllocationState::Registered, 100)
            .job_id(job)
            .version(2),
    )
    .await;
    r.put_file(alloc);

    r.rec.reconcile(epoch, t0()).await.expect("reconcile");
    assert!(r.has_file(alloc), "reconciliation alone removes nothing");
    let outcomes = r.cleaner.clean_due(t0()).await;

    assert_eq!(outcomes, Ok(vec![CleanOutcome::Deleted]));
    assert!(!r.has_file(alloc));
    assert_eq!(
        r.seq.events(),
        vec![
            "reconcile.fenced",
            "allocation.reconciled",
            "allocation.delete_claimed",
            "allocation.deleted"
        ]
    );
    assert_eq!(r.probes(), vec![probe(1)]);
}

/// TS-25 complete: a stale `deleting` row is reset by reconciliation, then claimed afresh.
#[tokio::test]
#[ignore = "requires Docker for PostgreSQL testcontainer"]
async fn ts25_stale_deleting_claim_is_reset_then_reclaimed_with_a_fresh_audited_claim() {
    let r = rig(0).await;
    let epoch = r.restart().await;
    let alloc = seed_alloc(
        &r.fx.conn,
        SeedAlloc::new(AllocationState::Deleting, 100)
            .version(3)
            .attempts(1),
    )
    .await;
    r.put_file(alloc);

    r.rec.reconcile(epoch, t0()).await.expect("reconcile");
    let outcomes = r.cleaner.clean_due(t0()).await;

    assert_eq!(outcomes, Ok(vec![CleanOutcome::Deleted]));
    let a = raw_alloc(&r.fx.conn, alloc).await.unwrap();
    assert_eq!(
        (a.state.as_str(), a.state_version, a.cleanup_attempts),
        ("deleted", 6, 1),
        "reset +1, claim +1, deleted +1; the crash cost no retry"
    );
    assert_eq!(
        r.seq.events(),
        vec![
            "allocation.reconciled",
            "allocation.delete_claimed",
            "allocation.deleted"
        ]
    );
    assert_eq!(r.probes(), vec![probe(1)]);
    assert!(!r.has_file(alloc));
}

// ------------------------------------------------------------- selection rules

/// Only `release_pending` and, after grace, `orphan_discovered` rows are claimable. Any
/// other row, or an unknown id, is `Lost` with no unlink and no audit.
#[tokio::test]
#[ignore = "requires Docker for PostgreSQL testcontainer"]
async fn rows_that_are_not_claimable_are_lost_without_unlink_or_audit() {
    let r = rig(0).await;
    let mut ids = Vec::new();
    for state in [
        AllocationState::Registered,
        AllocationState::Written,
        AllocationState::Deleting,
        AllocationState::Deleted,
        AllocationState::CleanupFailed,
    ] {
        let id = seed_alloc(&r.fx.conn, SeedAlloc::new(state, 100).version(2)).await;
        r.put_file(id);
        ids.push(id);
    }
    ids.push(Uuid::new_v4());
    let rows_before = count(
        &r.fx.conn,
        "SELECT COUNT(*)::bigint AS n FROM backup_allocations",
        vec![],
    )
    .await;

    for id in &ids {
        assert_eq!(
            r.cleaner.clean_one(*id, t0()).await,
            Ok(CleanOutcome::Lost),
            "{id}"
        );
    }

    assert_eq!(r.probes(), vec![]);
    assert_eq!(r.audit_total().await, 0);
    for id in &ids[..5] {
        assert!(r.has_file(*id), "no file of a non-claimable row is removed");
    }
    assert_eq!(
        count(
            &r.fx.conn,
            "SELECT COUNT(*)::bigint AS n FROM backup_allocations",
            vec![]
        )
        .await,
        rows_before
    );
}

/// `clean_due` works only on what is due and leaves the rest untouched.
#[tokio::test]
#[ignore = "requires Docker for PostgreSQL testcontainer"]
async fn clean_due_handles_due_rows_and_leaves_future_retries_untouched() {
    let r = rig(0).await;
    let due = seed_alloc(
        &r.fx.conn,
        SeedAlloc::new(AllocationState::ReleasePending, 100).version(2),
    )
    .await;
    let later = seed_alloc(
        &r.fx.conn,
        SeedAlloc::new(AllocationState::ReleasePending, 100)
            .version(2)
            .attempts(1)
            .next_attempt_at(after(60)),
    )
    .await;
    r.put_file(due);
    r.put_file(later);
    let later_before = raw_alloc(&r.fx.conn, later).await.unwrap();

    assert_eq!(
        r.cleaner.clean_due(t0()).await,
        Ok(vec![CleanOutcome::Deleted])
    );

    assert!(!r.has_file(due));
    assert!(r.has_file(later));
    assert_eq!(raw_alloc(&r.fx.conn, later).await.unwrap(), later_before);
}

/// Added at green after the mutation check: another writer changes the row while the file
/// is being unlinked, so the `deleted` compare-and-set is lost. The cleaner reports `Lost`,
/// commits nothing and writes no `allocation.deleted`; the row stays `deleting` for
/// recovery.
#[tokio::test]
#[ignore = "requires Docker for PostgreSQL testcontainer"]
async fn a_concurrent_change_during_the_unlink_makes_the_deleted_commit_lost() {
    let r = rig(0).await;
    let alloc = seed_alloc(
        &r.fx.conn,
        SeedAlloc::new(AllocationState::ReleasePending, 100).version(2),
    )
    .await;
    r.put_file(alloc);
    r.storage.bump_during_unlink.store(true, Ordering::SeqCst);

    let outcome = r.cleaner.clean_one(alloc, t0()).await;

    assert_eq!(outcome, Ok(CleanOutcome::Lost));
    let a = raw_alloc(&r.fx.conn, alloc).await.unwrap();
    assert_eq!(
        (a.state.as_str(), a.state_version, a.deleted_at),
        ("deleting", 4, None),
        "claim +1, then the other writer's +1; no deleted commit"
    );
    assert_eq!(r.events_for(alloc).await, vec!["allocation.delete_claimed"]);
}
