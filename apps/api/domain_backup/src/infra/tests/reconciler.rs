//! WU-17 red tests, PostgreSQL-backed: the `Reconciler` over `DbJobStore`,
//! `DbAllocationStore`, `DbAuditSink` and a real `FsFixtureStorage` under a temp root
//! (TS-20..TS-25, the reconcile half of TS-44b and the lifecycle half of TS-56).
//!
//! Every test seeds durable rows with raw SQL, puts real files in the namespace, runs the
//! reconciler and then reads the rows, the audit table and the directory back. Audit
//! ORDER is read from a recording sink (all audit rows of one run share one timestamp).
//! The unlink half of TS-20/21/22 (`allocation.delete_claimed` before the unlink and
//! `allocation.deleted` after it) belongs to the cleaner (WU-18); here the reconciler is
//! asserted to leave every file in place and to write neither of those events.

use super::alloc_support::{audit_rows_for_allocation, raw_alloc, seed_alloc, SeedAlloc, INSTANCE};
use super::fs_support::{names_in, TempDir};
use super::support::{
    audit_rows_for_job, count, exec, fx, raw_job, seed, t0, FixedClock, Fx, Seed,
};
use crate::domain::status::BackupJobStatus;
use crate::infra::db_allocation_store::DbAllocationStore;
use crate::infra::db_audit_sink::DbAuditSink;
use crate::infra::db_job_store::DbJobStore;
use crate::infra::fs_fixture_storage::FsFixtureStorage;
use crate::lifecycle::reconciler::{ReconcileError, Reconciler, ReconcilerConfig};
use crate::ports::allocation_store::{
    Allocation, AllocationError, AllocationLimits, AllocationRecoveryRow, AllocationState,
    AllocationStore, AllocationTotals, CleanupFailureOutcome,
};
use crate::ports::audit::{AuditError, AuditEvent, AuditEventKind, AuditSink};
use crate::ports::fixture_storage::{FixtureStorage, StorageKey};
use crate::ports::job_store::{
    AcceptResult, Components, IdemRecord, JobRow, JobStore, NewJob, ReasonCode, StoreError,
    TransitionResult,
};
use async_trait::async_trait;
use chrono::{DateTime, Duration, Utc};
use sea_orm::{ConnectionTrait, DatabaseConnection, DatabaseTransaction};
use std::sync::{Arc, Mutex};
use std::time::Duration as StdDuration;
use uuid::Uuid;

type Seen = Vec<(AuditEventKind, Option<Uuid>, Option<Uuid>)>;

/// Real audit sink that records the order of events and can refuse one kind.
#[derive(Debug)]
struct SeqSink {
    inner: DbAuditSink,
    fail: Option<AuditEventKind>,
    seen: Mutex<Seen>,
}

impl SeqSink {
    fn new(conn: &DatabaseConnection, fail: Option<AuditEventKind>) -> Arc<Self> {
        Arc::new(Self {
            inner: DbAuditSink::new(conn.clone()),
            fail,
            seen: Mutex::new(Vec::new()),
        })
    }
    fn events(&self) -> Vec<&'static str> {
        self.seen
            .lock()
            .unwrap()
            .iter()
            .map(|(k, _, _)| k.as_str())
            .collect()
    }
    fn count_of(&self, kind: AuditEventKind) -> usize {
        self.seen
            .lock()
            .unwrap()
            .iter()
            .filter(|(k, _, _)| *k == kind)
            .count()
    }
}

#[async_trait]
impl AuditSink for SeqSink {
    async fn record(
        &self,
        tx: Option<&DatabaseTransaction>,
        ev: AuditEvent,
    ) -> Result<(), AuditError> {
        if self.fail == Some(ev.event) {
            return Err(AuditError::Rejected);
        }
        let entry = (ev.event, ev.job_id, ev.allocation_id);
        self.inner.record(tx, ev).await?;
        self.seen.lock().unwrap().push(entry);
        Ok(())
    }
}

struct Rig {
    fx: Fx,
    tmp: TempDir,
    seq: Arc<SeqSink>,
    rec: Reconciler,
}

async fn rig() -> Rig {
    rig_with(None, 10_000, |_, jobs, allocs| (jobs, allocs)).await
}

/// `wrap` lets a test decorate the stores the reconciler reads (stale-snapshot doubles).
async fn rig_with(
    fail: Option<AuditEventKind>,
    cap: usize,
    wrap: impl FnOnce(
        &DatabaseConnection,
        Arc<dyn JobStore>,
        Arc<dyn AllocationStore>,
    ) -> (Arc<dyn JobStore>, Arc<dyn AllocationStore>),
) -> Rig {
    let fx = fx().await;
    let tmp = TempDir::new("wu17");
    let seq = SeqSink::new(&fx.conn, fail);
    let jobs: Arc<dyn JobStore> = Arc::new(DbJobStore::new(
        fx.conn.clone(),
        seq.clone(),
        Arc::new(FixedClock(t0())),
    ));
    let allocs: Arc<dyn AllocationStore> = Arc::new(DbAllocationStore::new(
        fx.conn.clone(),
        seq.clone(),
        Arc::new(FixedClock(t0())),
        INSTANCE,
        AllocationLimits::default(),
    ));
    let storage: Arc<dyn FixtureStorage> =
        Arc::new(FsFixtureStorage::open(&tmp.root(), allocs.clone()).expect("open storage"));
    let (jobs, allocs) = wrap(&fx.conn, jobs, allocs);
    let rec = Reconciler::new(
        jobs,
        allocs,
        storage,
        seq.clone(),
        ReconcilerConfig::new(INSTANCE).with_inventory_cap(cap),
    );
    Rig { fx, tmp, seq, rec }
}

impl Rig {
    fn alloc_dir(&self) -> std::path::PathBuf {
        self.tmp.root().join("alloc")
    }
    fn put_file(&self, id: Uuid, bytes: &[u8]) {
        std::fs::write(self.alloc_dir().join(format!("{id}.bin")), bytes).expect("write file");
    }
    fn has_file(&self, id: Uuid) -> bool {
        self.alloc_dir().join(format!("{id}.bin")).exists()
    }
    fn listing(&self) -> Vec<String> {
        names_in(&self.alloc_dir())
    }
    async fn audit_total(&self) -> i64 {
        count(
            &self.fx.conn,
            "SELECT COUNT(*)::bigint AS n FROM backup_audit",
            vec![],
        )
        .await
    }
    async fn events_for(&self, alloc: Uuid) -> Vec<String> {
        audit_rows_for_allocation(&self.fx.conn, alloc)
            .await
            .into_iter()
            .map(|r| r.event)
            .collect()
    }
}

fn now() -> DateTime<Utc> {
    t0()
}

async fn old_running_job(r: &Rig, instance: &str) -> Uuid {
    seed(
        &r.fx.conn,
        Seed::new(instance, BackupJobStatus::Running).version(4),
    )
    .await
}

fn is_sequence(actual: &[&str], expected: &[&str]) -> bool {
    actual == expected
}

// ---------------------------------------------------------------- epoch (FN-25)

#[tokio::test]
#[ignore = "requires Docker for PostgreSQL testcontainer"]
async fn advance_epoch_is_durable_and_strictly_increasing() {
    let r = rig().await;
    assert_eq!(r.rec.advance_epoch(now()).await, Ok(1));
    assert_eq!(r.rec.advance_epoch(now()).await, Ok(2));
    let stored = count(
        &r.fx.conn,
        "SELECT epoch::bigint AS n FROM backup_epoch WHERE id = 1",
        vec![],
    )
    .await;
    assert_eq!(
        stored, 2,
        "the raw backup_epoch row holds the returned epoch"
    );
}

// ------------------------------------------------- crash windows 1 to 3 (TS-20..22)

/// TS-20: ledger committed, file never created.
#[tokio::test]
#[ignore = "requires Docker for PostgreSQL testcontainer"]
async fn ts20_old_epoch_job_and_registered_allocation_without_file_are_fenced_then_released() {
    let r = rig().await;
    let job = old_running_job(&r, "ts20-instance").await;
    let alloc = seed_alloc(
        &r.fx.conn,
        SeedAlloc::new(AllocationState::Registered, 1024)
            .job_id(job)
            .version(2),
    )
    .await;
    let before = (raw_job(&r.fx.conn, job).await.unwrap(), r.listing());
    assert_eq!(
        (
            before.0.status.as_str(),
            before.0.state_version,
            before.0.epoch
        ),
        ("running", 4, 1)
    );

    let report = r.rec.reconcile(2, now()).await.expect("reconcile");

    let after = raw_job(&r.fx.conn, job).await.unwrap();
    assert_eq!(after.status, "failed");
    assert_eq!(after.reason_code.as_deref(), Some("INTERRUPTED_RESTART"));
    assert_eq!((after.state_version, after.epoch), (5, 1));
    let a = raw_alloc(&r.fx.conn, alloc).await.unwrap();
    assert_eq!(
        (
            a.state.as_str(),
            a.state_version,
            a.epoch,
            a.cleanup_attempts
        ),
        ("release_pending", 3, 1, 0)
    );
    assert!(
        is_sequence(
            &r.seq.events(),
            &["reconcile.fenced", "allocation.reconciled"]
        ),
        "job fence first, then the allocation CAS: {:?}",
        r.seq.events()
    );
    let fenced = audit_rows_for_job(&r.fx.conn, job)
        .await
        .into_iter()
        .find(|e| e.event == "reconcile.fenced")
        .expect("fence audit row");
    assert_eq!(
        (fenced.outcome.as_str(), fenced.reason_code.as_deref()),
        ("success", Some("INTERRUPTED_RESTART"))
    );
    assert_eq!((fenced.actor_sub, fenced.allocation_id), (None, None));
    assert_eq!(fenced.instance_id, "ts20-instance");
    let rec_row = audit_rows_for_allocation(&r.fx.conn, alloc)
        .await
        .into_iter()
        .find(|e| e.event == "allocation.reconciled")
        .expect("reconciled audit row");
    assert_eq!(rec_row.reason_code.as_deref(), Some("RECONCILED"));
    assert_eq!(rec_row.job_id, Some(job));
    assert_eq!(
        r.listing(),
        before.1,
        "the reconciler never creates or unlinks a file"
    );
    assert!(!r.has_file(alloc));
    assert_eq!((report.fenced_jobs, report.released_allocations), (1, 1));
    assert_eq!(r.seq.count_of(AuditEventKind::AllocationDeleteClaimed), 0);
    assert_eq!(r.seq.count_of(AuditEventKind::AllocationDeleted), 0);
}

/// TS-21: file created, ledger still `registered`.
#[tokio::test]
#[ignore = "requires Docker for PostgreSQL testcontainer"]
async fn ts21_registered_allocation_with_file_is_retained_as_owned_and_released_not_orphaned() {
    let r = rig().await;
    let job = old_running_job(&r, "ts21-instance").await;
    let alloc = seed_alloc(
        &r.fx.conn,
        SeedAlloc::new(AllocationState::Registered, 1024)
            .job_id(job)
            .version(2),
    )
    .await;
    r.put_file(alloc, b"synthetic fixture bytes");

    let report = r.rec.reconcile(2, now()).await.expect("reconcile");

    assert_eq!(raw_job(&r.fx.conn, job).await.unwrap().status, "failed");
    let a = raw_alloc(&r.fx.conn, alloc).await.unwrap();
    assert_eq!(
        (a.state.as_str(), a.state_version),
        ("release_pending", 3),
        "owned file stays under its ledger row"
    );
    assert!(
        r.has_file(alloc),
        "removal is the cleaner's audited step, not the reconciler's"
    );
    assert_eq!(
        report.discovered_orphans, 0,
        "an owned file is never an orphan"
    );
    assert_eq!(r.events_for(alloc).await, vec!["allocation.reconciled"]);
    assert!(
        is_sequence(
            &r.seq.events(),
            &["reconcile.fenced", "allocation.reconciled"]
        ),
        "{:?}",
        r.seq.events()
    );
    assert_eq!(
        count(
            &r.fx.conn,
            "SELECT COUNT(*)::bigint AS n FROM backup_allocations",
            vec![]
        )
        .await,
        1,
        "no second (orphan) row was created for the owned file"
    );
}

/// TS-22: worker interrupted mid-job, restart advances the epoch and fences.
#[tokio::test]
#[ignore = "requires Docker for PostgreSQL testcontainer"]
async fn ts22_restart_advances_epoch_fences_running_and_queued_jobs_and_never_resumes() {
    let r = rig().await;
    let first = r.rec.advance_epoch(now()).await.expect("first boot epoch");
    assert_eq!(first, 1);
    let running = old_running_job(&r, "ts22-running").await;
    let queued = seed(
        &r.fx.conn,
        Seed::new("ts22-queued", BackupJobStatus::Queued),
    )
    .await;
    let alloc = seed_alloc(
        &r.fx.conn,
        SeedAlloc::new(AllocationState::Written, 2048)
            .job_id(running)
            .version(3),
    )
    .await;
    r.put_file(alloc, b"partial");

    let epoch = r.rec.advance_epoch(now()).await.expect("restart epoch");
    assert_eq!(epoch, 2, "a restart advances the durable epoch");
    let report = r.rec.reconcile(epoch, now()).await.expect("reconcile");

    let run = raw_job(&r.fx.conn, running).await.unwrap();
    assert_eq!(
        (
            run.status.as_str(),
            run.reason_code.as_deref(),
            run.state_version
        ),
        ("failed", Some("INTERRUPTED_RESTART"), 5)
    );
    let que = raw_job(&r.fx.conn, queued).await.unwrap();
    assert_eq!(
        (
            que.status.as_str(),
            que.reason_code.as_deref(),
            que.state_version
        ),
        ("failed", Some("ABANDONED_QUEUED"), 1)
    );
    assert_eq!(report.fenced_jobs, 2);
    let a = raw_alloc(&r.fx.conn, alloc).await.unwrap();
    assert_eq!((a.state.as_str(), a.state_version), ("release_pending", 4));
    assert_eq!(
        r.seq.count_of(AuditEventKind::ReconcileFenced),
        2,
        "one fence event per fenced job"
    );
    assert_eq!(r.seq.count_of(AuditEventKind::AllocationReconciled), 1);
    assert_eq!(
        count(
            &r.fx.conn,
            "SELECT COUNT(*)::bigint AS n FROM backup_jobs WHERE status IN ('queued','running')",
            vec![]
        )
        .await,
        0,
        "no old-epoch job stays active and none is resumed"
    );
}

/// TS-22 lost-CAS rule, job side: a stale job snapshot fences nothing and audits nothing.
#[tokio::test]
#[ignore = "requires Docker for PostgreSQL testcontainer"]
async fn ts22_lost_job_fence_cas_changes_nothing_and_emits_no_event() {
    let r = rig_with(None, 10_000, |conn, jobs, allocs| {
        let stale: Arc<dyn JobStore> = Arc::new(StaleJobs {
            inner: jobs,
            conn: conn.clone(),
        });
        (stale, allocs)
    })
    .await;
    let job = old_running_job(&r, "ts22-lost-instance").await;

    let report = r
        .rec
        .reconcile(2, now())
        .await
        .expect("a lost CAS is a result, not an error");

    let after = raw_job(&r.fx.conn, job).await.unwrap();
    assert_eq!(
        (after.status.as_str(), after.state_version),
        ("running", 5),
        "only the interleaved writer's version bump is visible"
    );
    assert_eq!(report.fenced_jobs, 0);
    assert_eq!(r.seq.count_of(AuditEventKind::ReconcileFenced), 0);
    assert!(audit_rows_for_job(&r.fx.conn, job).await.is_empty());
}

/// TS-22 lost-CAS rule, allocation side.
#[tokio::test]
#[ignore = "requires Docker for PostgreSQL testcontainer"]
async fn ts22_lost_allocation_cas_changes_nothing_and_emits_no_event() {
    let r = rig_with(None, 10_000, |conn, jobs, allocs| {
        let stale: Arc<dyn AllocationStore> = Arc::new(StaleAllocs {
            inner: allocs,
            conn: conn.clone(),
        });
        (jobs, stale)
    })
    .await;
    let failed = seed(
        &r.fx.conn,
        Seed::new("ts22-lost-alloc-instance", BackupJobStatus::Failed),
    )
    .await;
    let alloc = seed_alloc(
        &r.fx.conn,
        SeedAlloc::new(AllocationState::Written, 512)
            .job_id(failed)
            .version(2),
    )
    .await;

    let report = r.rec.reconcile(2, now()).await.expect("reconcile");

    let a = raw_alloc(&r.fx.conn, alloc).await.unwrap();
    assert_eq!(
        (a.state.as_str(), a.state_version),
        ("written", 3),
        "only the interleaved writer's version bump is visible"
    );
    assert_eq!(report.released_allocations, 0);
    assert!(
        r.events_for(alloc).await.is_empty(),
        "a lost CAS emits no event"
    );
}

/// TS-44b: an allocation with no owner row is released with the allocation id as the
/// non-null correlation id and a NULL job id.
#[tokio::test]
#[ignore = "requires Docker for PostgreSQL testcontainer"]
async fn ts44b_ownerless_registered_allocation_is_released_with_allocation_correlation() {
    let r = rig().await;
    let alloc = seed_alloc(
        &r.fx.conn,
        SeedAlloc::new(AllocationState::Registered, 64).version(1),
    )
    .await;

    r.rec.reconcile(2, now()).await.expect("reconcile");

    let a = raw_alloc(&r.fx.conn, alloc).await.unwrap();
    assert_eq!((a.state.as_str(), a.state_version), ("release_pending", 2));
    let rows = audit_rows_for_allocation(&r.fx.conn, alloc).await;
    assert_eq!(rows.len(), 1);
    assert_eq!(rows[0].event, "allocation.reconciled");
    assert_eq!(rows[0].job_id, None);
    assert_eq!(rows[0].correlation_id, alloc.to_string());
    assert_eq!(rows[0].instance_id, INSTANCE);
}

// ------------------------------------------------------- window 4: orphans (TS-23)

#[tokio::test]
#[ignore = "requires Docker for PostgreSQL testcontainer"]
async fn ts23_unowned_file_becomes_a_graced_orphan_and_ambiguous_names_stay_untouched() {
    let r = rig().await;
    let orphan = Uuid::new_v4();
    r.put_file(orphan, b"synthetic orphan");
    std::fs::write(r.alloc_dir().join("stray.txt"), b"not ours").unwrap();
    std::fs::create_dir(r.alloc_dir().join("not-a-key")).unwrap();
    let listing_before = r.listing();
    assert_eq!(listing_before.len(), 3);

    let report = r.rec.reconcile(2, now()).await.expect("reconcile");

    assert_eq!(report.discovered_orphans, 1);
    assert_eq!(report.ambiguous_entries, 2);
    let row = raw_alloc(&r.fx.conn, orphan)
        .await
        .expect("orphan row persisted");
    assert_eq!(row.state, "orphan_discovered");
    assert_eq!(row.job_id, None);
    assert_eq!(
        row.orphan_grace_until,
        Some(now() + Duration::hours(1)),
        "grace is persisted before any deletion"
    );
    assert_eq!(r.events_for(orphan).await, vec!["orphan.discovered"]);
    assert_eq!(
        r.listing(),
        listing_before,
        "nothing is deleted before grace"
    );
    let incidents = audit_rows_by_event(&r, "ownership.ambiguous").await;
    assert_eq!(incidents.len(), 2, "one incident per ambiguous entry");
    for incident in &incidents {
        assert_eq!(incident.instance_id, INSTANCE);
        assert_eq!(
            (
                incident.job_id,
                incident.actor_sub.clone(),
                incident.digest.clone()
            ),
            (None, None, None)
        );
        assert_eq!(incident.operation, "reconciliation");
        assert!(!incident.correlation_id.is_empty());
    }

    // A later full scan (after grace) still leaves the orphan file and the ambiguous
    // entries alone: removal belongs to the cleaner after its audited claim.
    r.rec
        .reconcile(2, now() + Duration::hours(2))
        .await
        .expect("second reconcile");
    assert_eq!(r.listing(), listing_before);
    assert_eq!(
        r.events_for(orphan).await,
        vec!["orphan.discovered"],
        "an already tracked orphan is not re-announced"
    );
    assert_eq!(
        raw_alloc(&r.fx.conn, orphan).await.unwrap().state_version,
        0
    );
}

#[tokio::test]
#[ignore = "requires Docker for PostgreSQL testcontainer"]
async fn ts23_sweep_discovery_persists_a_file_added_after_start_and_never_deletes_or_fences() {
    let r = rig().await;
    let running = old_running_job(&r, "ts23-sweep-instance").await;
    r.rec.reconcile(1, now()).await.expect("start-up reconcile");
    let late = Uuid::new_v4();
    r.put_file(late, b"arrived after start");
    let at = now() + Duration::minutes(15);

    let found = r.rec.discover_orphans(at).await.expect("sweep discovery");

    assert_eq!(found, 1);
    let row = raw_alloc(&r.fx.conn, late).await.expect("discovered row");
    assert_eq!(row.state, "orphan_discovered");
    assert_eq!(row.orphan_grace_until, Some(at + Duration::hours(1)));
    assert!(r.has_file(late), "discover-only: nothing is unlinked");
    assert_eq!(
        raw_job(&r.fx.conn, running).await.unwrap().status,
        "running",
        "discover-only never fences a job"
    );
    assert_eq!(
        r.rec.discover_orphans(at).await,
        Ok(0),
        "a second scan finds nothing new"
    );
    assert_eq!(r.events_for(late).await, vec!["orphan.discovered"]);
}

#[tokio::test]
#[ignore = "requires Docker for PostgreSQL testcontainer"]
async fn ts23_truncated_sweep_discovery_persists_nothing() {
    let r = rig_with(None, 1, |_, jobs, allocs| (jobs, allocs)).await;
    r.put_file(Uuid::new_v4(), b"one");
    r.put_file(Uuid::new_v4(), b"two");
    let audit_before = r.audit_total().await;

    let result = r.rec.discover_orphans(now()).await;

    assert_eq!(result, Err(ReconcileError::InventoryTruncated));
    assert_eq!(
        alloc_row_count(&r).await,
        0,
        "no orphan from a partial view"
    );
    assert_eq!(r.audit_total().await, audit_before);
    assert_eq!(r.listing().len(), 2);
}

// ------------------------------------------------- window 5: missing file (TS-24)

#[tokio::test]
#[ignore = "requires Docker for PostgreSQL testcontainer"]
async fn ts24_unexpired_incomplete_allocation_with_missing_file_audits_each_full_scan_unchanged() {
    let r = rig().await;
    let job = seed(
        &r.fx.conn,
        Seed::new("ts24-instance", BackupJobStatus::Incomplete).version(2),
    )
    .await;
    let alloc = seed_alloc(
        &r.fx.conn,
        SeedAlloc::new(AllocationState::Written, 4096)
            .job_id(job)
            .version(5),
    )
    .await;
    let snapshot = raw_alloc(&r.fx.conn, alloc).await.unwrap();

    let first = r.rec.reconcile(2, now()).await.expect("first scan");
    let second = r
        .rec
        .reconcile(2, now() + Duration::hours(1))
        .await
        .expect("second scan");

    assert_eq!((first.missing_files, second.missing_files), (1, 1));
    assert_eq!(
        raw_alloc(&r.fx.conn, alloc).await.unwrap(),
        snapshot,
        "state, state_version and updated_at stay unchanged"
    );
    let rows = audit_rows_for_allocation(&r.fx.conn, alloc).await;
    assert_eq!(
        rows.len(),
        2,
        "appended on every complete scan, no deduplication"
    );
    for row in &rows {
        assert_eq!(row.event, "reconcile.missing_file");
        assert_eq!(row.outcome, "success");
        assert_eq!(row.reason_code.as_deref(), Some("MISSING_FILE"));
        assert_eq!(row.actor_sub, None);
        assert_eq!(row.instance_id, INSTANCE);
        assert_eq!(row.job_id, Some(job));
        assert_eq!(row.allocation_id, Some(alloc));
        assert_eq!(row.operation, "reconciliation");
        assert_eq!(row.digest, None);
        assert_eq!(row.correlation_id, format!("seed-corr-{job}"));
    }
}

#[tokio::test]
#[ignore = "requires Docker for PostgreSQL testcontainer"]
async fn ts24_failed_owner_or_expired_written_allocation_becomes_release_eligible() {
    let r = rig().await;
    let failed = seed(
        &r.fx.conn,
        Seed::new("ts24-failed-instance", BackupJobStatus::Failed),
    )
    .await;
    let by_failed = seed_alloc(
        &r.fx.conn,
        SeedAlloc::new(AllocationState::Written, 100)
            .job_id(failed)
            .version(1),
    )
    .await;
    let incomplete = seed(
        &r.fx.conn,
        Seed::new("ts24-expired-instance", BackupJobStatus::Incomplete),
    )
    .await;
    let by_expiry = seed_alloc(
        &r.fx.conn,
        SeedAlloc::new(AllocationState::Written, 100)
            .job_id(incomplete)
            .version(1),
    )
    .await;

    // Both files are absent. 25 h after the seed: the 24 h expiry is reached.
    let report = r
        .rec
        .reconcile(2, now() + Duration::hours(25))
        .await
        .expect("reconcile");

    for id in [by_failed, by_expiry] {
        let a = raw_alloc(&r.fx.conn, id).await.unwrap();
        assert_eq!((a.state.as_str(), a.state_version), ("release_pending", 2));
        assert_eq!(r.events_for(id).await, vec!["allocation.reconciled"]);
    }
    assert_eq!(report.released_allocations, 2);
    assert_eq!(report.missing_files, 0, "released, not merely observed");
}

#[tokio::test]
#[ignore = "requires Docker for PostgreSQL testcontainer"]
async fn ts24_unexpired_incomplete_allocation_with_its_file_present_is_retained_silently() {
    let r = rig().await;
    let job = seed(
        &r.fx.conn,
        Seed::new("ts24-present-instance", BackupJobStatus::Incomplete),
    )
    .await;
    let alloc = seed_alloc(
        &r.fx.conn,
        SeedAlloc::new(AllocationState::Written, 100)
            .job_id(job)
            .version(1),
    )
    .await;
    r.put_file(alloc, b"retained");
    let snapshot = raw_alloc(&r.fx.conn, alloc).await.unwrap();

    let report = r.rec.reconcile(2, now()).await.expect("reconcile");

    assert_eq!(raw_alloc(&r.fx.conn, alloc).await.unwrap(), snapshot);
    assert!(r.events_for(alloc).await.is_empty());
    assert_eq!(report.missing_files, 0);
    assert!(r.has_file(alloc));
}

#[tokio::test]
#[ignore = "requires Docker for PostgreSQL testcontainer"]
async fn ts24_missing_file_audit_failure_fails_reconciliation_and_changes_no_state() {
    let r = rig_with(
        Some(AuditEventKind::ReconcileMissingFile),
        10_000,
        |_, jobs, allocs| (jobs, allocs),
    )
    .await;
    let job = seed(
        &r.fx.conn,
        Seed::new("ts24-audit-instance", BackupJobStatus::Incomplete),
    )
    .await;
    let alloc = seed_alloc(
        &r.fx.conn,
        SeedAlloc::new(AllocationState::Written, 100)
            .job_id(job)
            .version(1),
    )
    .await;
    let snapshot = raw_alloc(&r.fx.conn, alloc).await.unwrap();

    let result = r.rec.reconcile(2, now()).await;

    assert_eq!(
        result,
        Err(ReconcileError::AuditUnavailable),
        "audit failure fails reconciliation so admissions stay closed"
    );
    assert_eq!(raw_alloc(&r.fx.conn, alloc).await.unwrap(), snapshot);
    assert_eq!(r.audit_total().await, 0);
}

// ------------------------------------------- window 6 and untouched rows (TS-25)

#[tokio::test]
#[ignore = "requires Docker for PostgreSQL testcontainer"]
async fn ts25_stale_deleting_claim_returns_to_release_pending_without_consuming_a_retry() {
    let r = rig().await;
    let alloc = seed_alloc(
        &r.fx.conn,
        SeedAlloc::new(AllocationState::Deleting, 100)
            .version(3)
            .attempts(1),
    )
    .await;
    r.put_file(alloc, b"claimed before the crash");

    let report = r.rec.reconcile(2, now()).await.expect("reconcile");

    let a = raw_alloc(&r.fx.conn, alloc).await.unwrap();
    assert_eq!(
        (a.state.as_str(), a.state_version, a.cleanup_attempts),
        ("release_pending", 4, 1),
        "a crash is not an unlink failure: attempts are unchanged"
    );
    assert_eq!(r.events_for(alloc).await, vec!["allocation.reconciled"]);
    assert_eq!(report.reset_claims, 1);
    assert!(
        r.has_file(alloc),
        "the unlink needs a fresh audited claim from the cleaner"
    );
    assert_eq!(r.seq.count_of(AuditEventKind::AllocationDeleteClaimed), 0);
}

#[tokio::test]
#[ignore = "requires Docker for PostgreSQL testcontainer"]
async fn ts25_current_epoch_deleting_claim_belongs_to_the_live_cleaner() {
    let r = rig().await;
    let alloc = seed_alloc(
        &r.fx.conn,
        SeedAlloc::new(AllocationState::Deleting, 100).version(3),
    )
    .await;
    let snapshot = raw_alloc(&r.fx.conn, alloc).await.unwrap();

    // The row's epoch (1) equals the reconcile epoch: not a crashed claim.
    r.rec.reconcile(1, now()).await.expect("reconcile");

    assert_eq!(raw_alloc(&r.fx.conn, alloc).await.unwrap(), snapshot);
    assert!(r.events_for(alloc).await.is_empty());
}

#[tokio::test]
#[ignore = "requires Docker for PostgreSQL testcontainer"]
async fn current_epoch_running_job_and_its_registered_allocation_are_left_to_the_worker() {
    let r = rig().await;
    let job = old_running_job(&r, "worker-owned-instance").await;
    let alloc = seed_alloc(
        &r.fx.conn,
        SeedAlloc::new(AllocationState::Registered, 100)
            .job_id(job)
            .version(2),
    )
    .await;
    let (job_before, alloc_before) = (
        raw_job(&r.fx.conn, job).await.unwrap(),
        raw_alloc(&r.fx.conn, alloc).await.unwrap(),
    );

    let report = r.rec.reconcile(1, now()).await.expect("reconcile");

    assert_eq!(raw_job(&r.fx.conn, job).await.unwrap(), job_before);
    assert_eq!(raw_alloc(&r.fx.conn, alloc).await.unwrap(), alloc_before);
    assert_eq!(report.fenced_jobs, 0);
    assert_eq!(r.audit_total().await, 0);
}

#[tokio::test]
#[ignore = "requires Docker for PostgreSQL testcontainer"]
async fn cleanup_failed_allocation_is_terminal_for_automation_and_never_touched() {
    let r = rig().await;
    let alloc = seed_alloc(
        &r.fx.conn,
        SeedAlloc::new(AllocationState::CleanupFailed, 100)
            .version(4)
            .attempts(3),
    )
    .await;
    let snapshot = raw_alloc(&r.fx.conn, alloc).await.unwrap();

    r.rec.reconcile(2, now()).await.expect("reconcile");

    assert_eq!(raw_alloc(&r.fx.conn, alloc).await.unwrap(), snapshot);
    assert!(r.events_for(alloc).await.is_empty());
}

// ------------------------------------------------------------ TS-56 fail closed

#[tokio::test]
#[ignore = "requires Docker for PostgreSQL testcontainer"]
async fn ts56_truncated_inventory_fails_closed_before_any_write() {
    let r = rig_with(None, 1, |_, jobs, allocs| (jobs, allocs)).await;
    let job = old_running_job(&r, "ts56-instance").await;
    let alloc = seed_alloc(
        &r.fx.conn,
        SeedAlloc::new(AllocationState::Registered, 100)
            .job_id(job)
            .version(2),
    )
    .await;
    r.put_file(
        Uuid::new_v4(),
        b"an unowned file that must not become an orphan",
    );
    r.put_file(alloc, b"owned file");
    let (job_before, alloc_before, listing_before, audit_before, rows_before) = (
        raw_job(&r.fx.conn, job).await.unwrap(),
        raw_alloc(&r.fx.conn, alloc).await.unwrap(),
        r.listing(),
        r.audit_total().await,
        alloc_row_count(&r).await,
    );

    let result = r.rec.reconcile(2, now()).await;

    assert_eq!(result, Err(ReconcileError::InventoryTruncated));
    assert_eq!(
        raw_job(&r.fx.conn, job).await.unwrap(),
        job_before,
        "no partial-view fence"
    );
    assert_eq!(raw_alloc(&r.fx.conn, alloc).await.unwrap(), alloc_before);
    assert_eq!(alloc_row_count(&r).await, rows_before, "no orphan insert");
    assert_eq!(
        r.audit_total().await,
        audit_before,
        "no audit row of any kind"
    );
    assert_eq!(r.listing(), listing_before, "no unlink");
}

// ------------------------------------------------------------------ test doubles

async fn audit_rows_by_event(r: &Rig, event: &str) -> Vec<super::support::AuditRow> {
    r.fx.conn
        .query_all(super::support::pg(
            "SELECT occurred_at, actor_sub, instance_id, job_id, allocation_id, operation, \
             event, outcome, reason_code, digest::text AS digest, correlation_id \
             FROM backup_audit WHERE event = $1",
            vec![event.to_owned().into()],
        ))
        .await
        .expect("audit by event")
        .iter()
        .map(|x| super::support::AuditRow {
            occurred_at: x.try_get("", "occurred_at").unwrap(),
            actor_sub: x.try_get("", "actor_sub").unwrap(),
            instance_id: x.try_get("", "instance_id").unwrap(),
            job_id: x.try_get("", "job_id").unwrap(),
            allocation_id: x.try_get("", "allocation_id").unwrap(),
            operation: x.try_get("", "operation").unwrap(),
            event: x.try_get("", "event").unwrap(),
            outcome: x.try_get("", "outcome").unwrap(),
            reason_code: x.try_get("", "reason_code").unwrap(),
            digest: x.try_get("", "digest").unwrap(),
            correlation_id: x.try_get("", "correlation_id").unwrap(),
        })
        .collect()
}

async fn alloc_row_count(r: &Rig) -> i64 {
    count(
        &r.fx.conn,
        "SELECT COUNT(*)::bigint AS n FROM backup_allocations",
        vec![],
    )
    .await
}

/// Returns the real job inventory, then bumps every job's `state_version`, so the
/// snapshot the reconciler holds is stale when it fences.
struct StaleJobs {
    inner: Arc<dyn JobStore>,
    conn: DatabaseConnection,
}

#[async_trait]
impl JobStore for StaleJobs {
    async fn accept(
        &self,
        tx: &DatabaseTransaction,
        new: NewJob,
        idem: IdemRecord,
    ) -> Result<AcceptResult, StoreError> {
        self.inner.accept(tx, new, idem).await
    }
    async fn get(&self, id: Uuid) -> Result<Option<JobRow>, StoreError> {
        self.inner.get(id).await
    }
    async fn transition(
        &self,
        id: Uuid,
        expect: (BackupJobStatus, i64),
        to: BackupJobStatus,
        reason: Option<ReasonCode>,
        comps: Option<Components>,
    ) -> Result<TransitionResult, StoreError> {
        self.inner.transition(id, expect, to, reason, comps).await
    }
    async fn claim_next(
        &self,
        epoch: i64,
        deadline: StdDuration,
    ) -> Result<Option<JobRow>, StoreError> {
        self.inner.claim_next(epoch, deadline).await
    }
    async fn advance_epoch(&self, now: DateTime<Utc>) -> Result<i64, StoreError> {
        self.inner.advance_epoch(now).await
    }
    async fn list_for_reconciliation(&self) -> Result<Vec<JobRow>, StoreError> {
        let rows = self.inner.list_for_reconciliation().await?;
        exec(
            &self.conn,
            "UPDATE backup_jobs SET state_version = state_version + 1",
            vec![],
        )
        .await;
        Ok(rows)
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
        self.inner.list_deadline_expired(now, epoch).await
    }
}

/// Returns the real allocation inventory, then bumps every `written` allocation's
/// `state_version`, so the reconciler's allocation CAS is stale.
struct StaleAllocs {
    inner: Arc<dyn AllocationStore>,
    conn: DatabaseConnection,
}

#[async_trait]
impl AllocationStore for StaleAllocs {
    async fn register(
        &self,
        job: Option<Uuid>,
        bytes: u64,
        epoch: i64,
        expires: DateTime<Utc>,
    ) -> Result<Allocation, AllocationError> {
        self.inner.register(job, bytes, epoch, expires).await
    }
    async fn mark_written(&self, id: Uuid, ver: i64) -> Result<(), AllocationError> {
        self.inner.mark_written(id, ver).await
    }
    async fn release(&self, id: Uuid, ver: i64) -> Result<(), AllocationError> {
        self.inner.release(id, ver).await
    }
    async fn claim_for_delete(
        &self,
        id: Uuid,
        ver: i64,
    ) -> Result<Option<Allocation>, AllocationError> {
        self.inner.claim_for_delete(id, ver).await
    }
    async fn mark_deleted(&self, id: Uuid, ver: i64) -> Result<(), AllocationError> {
        self.inner.mark_deleted(id, ver).await
    }
    async fn record_cleanup_failure(
        &self,
        id: Uuid,
        ver: i64,
        reason: ReasonCode,
        now: DateTime<Utc>,
    ) -> Result<CleanupFailureOutcome, AllocationError> {
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
        self.inner.insert_orphan(id, key, bytes, now).await
    }
    async fn due_for_cleanup(
        &self,
        now: DateTime<Utc>,
    ) -> Result<Vec<Allocation>, AllocationError> {
        self.inner.due_for_cleanup(now).await
    }
    async fn totals(&self) -> Result<AllocationTotals, AllocationError> {
        self.inner.totals().await
    }
    async fn get(&self, id: Uuid) -> Result<Option<Allocation>, AllocationError> {
        self.inner.get(id).await
    }
    async fn list_for_reconciliation(&self) -> Result<Vec<AllocationRecoveryRow>, AllocationError> {
        let rows = self.inner.list_for_reconciliation().await?;
        exec(
            &self.conn,
            "UPDATE backup_allocations SET state_version = state_version + 1 \
             WHERE state = 'written'",
            vec![],
        )
        .await;
        Ok(rows)
    }
    async fn reconcile_state(
        &self,
        id: Uuid,
        expected: (AllocationState, i64, i64),
        to: AllocationState,
        now: DateTime<Utc>,
    ) -> Result<TransitionResult, AllocationError> {
        self.inner.reconcile_state(id, expected, to, now).await
    }
}

/// Added at green after the mutation check: expiry alone never releases active work, so a
/// `written` allocation of a `running` job stays untouched even after its 24 h expiry.
#[tokio::test]
#[ignore = "requires Docker for PostgreSQL testcontainer"]
async fn an_expired_written_allocation_of_a_running_job_is_not_released() {
    let r = rig().await;
    let job = old_running_job(&r, "active-work-instance").await;
    let alloc = seed_alloc(
        &r.fx.conn,
        SeedAlloc::new(AllocationState::Written, 100)
            .job_id(job)
            .version(2),
    )
    .await;
    r.put_file(alloc, b"active work");
    let (job_before, alloc_before) = (
        raw_job(&r.fx.conn, job).await.unwrap(),
        raw_alloc(&r.fx.conn, alloc).await.unwrap(),
    );

    // Epoch 1 equals the job's epoch, so the job is current; 25 h passes the expiry.
    let report = r
        .rec
        .reconcile(1, now() + Duration::hours(25))
        .await
        .expect("reconcile");

    assert_eq!(raw_job(&r.fx.conn, job).await.unwrap(), job_before);
    assert_eq!(raw_alloc(&r.fx.conn, alloc).await.unwrap(), alloc_before);
    assert_eq!(report.released_allocations, 0);
    assert!(r.has_file(alloc));
    assert_eq!(r.audit_total().await, 0);
}
