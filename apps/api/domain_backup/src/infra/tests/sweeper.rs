//! WU-19 red tests, PostgreSQL-backed: the `Sweeper` over `DbJobStore`,
//! `DbAllocationStore`, `DbRetentionStore`, a real `FsFixtureStorage` and the real audit
//! sink (TS-28 expiry race, the deadline, queued-age and expiry boundaries of TS-30, TS-42
//! injection and the purge path of TS-65).
//!
//! Each test seeds durable rows with raw SQL, runs one sweep at an explicit `epoch` and
//! `now`, and reads rows, the audit table and the directory back. The call log of the
//! shared rig proves which port the sweeper used and in which order.

use super::alloc_support::{audit_rows_for_allocation, raw_alloc, seed_alloc, SeedAlloc};
use super::lifecycle_support::{after, life_rig, LifeRig};
use super::support::{audit_rows_for_job, count, raw_job, seed, t0, Seed};
use crate::domain::status::BackupJobStatus;
use crate::lifecycle::sweeper::{SweepError, SweepReport};
use crate::ports::allocation_store::AllocationState;
use crate::ports::audit::AuditEventKind;
use crate::ports::job_store::{JobStore, ReasonCode, TransitionResult};
use chrono::Duration;
use uuid::Uuid;

const EPOCH: i64 = 1;

async fn running_job(r: &LifeRig, instance: &str, deadline: chrono::DateTime<chrono::Utc>) -> Uuid {
    let job = seed(
        r.conn(),
        Seed::new(instance, BackupJobStatus::Running).version(4),
    )
    .await;
    r.set_deadline(job, deadline).await;
    job
}

async fn table_rows(r: &LifeRig, table: &str) -> i64 {
    count(
        r.conn(),
        &format!("SELECT COUNT(*)::bigint AS n FROM {table}"),
        vec![],
    )
    .await
}

// -------------------------------------------------------- deadline (F-20, TS-30)

/// `deadline_at <= now` is eligible, including a job exactly at `now`.
#[tokio::test]
#[ignore = "requires Docker for PostgreSQL testcontainer"]
async fn ts30_a_running_job_fails_deadline_exceeded_exactly_at_its_deadline_and_not_before() {
    let r = life_rig(10_000).await;
    let at_deadline = running_job(&r, "ts30-at", after(600)).await;
    let just_after = running_job(&r, "ts30-after", after(600)).await;

    let before = r.sweeper.sweep(EPOCH, after(599)).await.expect("sweep");
    assert_eq!(before.deadlines, 0, "one second before the deadline");
    assert_eq!(
        raw_job(r.conn(), at_deadline).await.unwrap().status,
        "running"
    );

    let at = r.sweeper.sweep(EPOCH, after(600)).await.expect("sweep");
    assert_eq!(at.deadlines, 2, "exactly at the deadline is eligible");
    for job in [at_deadline, just_after] {
        let row = raw_job(r.conn(), job).await.unwrap();
        assert_eq!(
            (
                row.status.as_str(),
                row.reason_code.as_deref(),
                row.state_version
            ),
            ("failed", Some("DEADLINE_EXCEEDED"), 5)
        );
        let events = audit_rows_for_job(r.conn(), job).await;
        assert_eq!(events.len(), 1);
        assert_eq!(events[0].event, "job.failed");
        assert_eq!(events[0].reason_code.as_deref(), Some("DEADLINE_EXCEEDED"));
    }
}

/// A deadline failure makes the job terminal, so its still-unexpired written allocation
/// must be released in this same sweep rather than remaining reserved until expiry.
#[tokio::test]
#[ignore = "requires Docker for PostgreSQL testcontainer"]
async fn a_deadline_failed_job_releases_its_unexpired_written_allocation() {
    let r = life_rig(10_000).await;
    let job = running_job(&r, "deadline-release", after(10)).await;
    let alloc = seed_alloc(
        r.conn(),
        SeedAlloc::new(AllocationState::Written, 100)
            .job_id(job)
            .version(2),
    )
    .await;
    r.put_file(alloc);

    let report = r.sweeper.sweep(EPOCH, after(60)).await.expect("sweep");

    assert_eq!(report.deadlines, 1);
    assert_eq!(report.expired, 0, "not released by expiry");
    assert_eq!(
        report.terminal_released, 1,
        "terminal owner releases allocation"
    );
    assert_eq!(raw_job(r.conn(), job).await.unwrap().status, "failed");
    let allocation = raw_alloc(r.conn(), alloc).await.unwrap();
    assert_eq!(
        (allocation.state.as_str(), allocation.state_version),
        ("release_pending", 3)
    );
    assert!(
        r.has_file(alloc),
        "sweep schedules cleanup; cleaner unlinks"
    );
    let events = audit_rows_for_allocation(r.conn(), alloc).await;
    assert_eq!(events.len(), 1);
    assert_eq!(events[0].event, "allocation.reconciled");
    assert_eq!(events[0].reason_code.as_deref(), Some("RECONCILED"));
}

/// Only jobs of the current epoch are failed by the sweep; an older epoch is the
/// reconciler's (fence), not the sweeper's.
#[tokio::test]
#[ignore = "requires Docker for PostgreSQL testcontainer"]
async fn the_sweep_ignores_a_job_of_another_epoch_even_past_its_deadline() {
    let r = life_rig(10_000).await;
    let current = running_job(&r, "epoch-current", after(10)).await;
    let older = running_job(&r, "epoch-older", after(10)).await;
    r.set_job_epoch(older, 0).await;

    let report = r.sweeper.sweep(EPOCH, after(60)).await.expect("sweep");

    assert_eq!(report.deadlines, 1);
    assert_eq!(raw_job(r.conn(), current).await.unwrap().status, "failed");
    assert_eq!(raw_job(r.conn(), older).await.unwrap().status, "running");
}

/// A queued job not claimed within the queued-age bound (600 s) fails `DISPATCH_FAILED`.
#[tokio::test]
#[ignore = "requires Docker for PostgreSQL testcontainer"]
async fn ts30_a_queued_job_fails_dispatch_failed_at_the_queued_age_bound() {
    let r = life_rig(10_000).await;
    let queued = seed(
        r.conn(),
        Seed::new("queued-age", BackupJobStatus::Queued).created_at(t0()),
    )
    .await;

    let early = r.sweeper.sweep(EPOCH, after(599)).await.expect("sweep");
    assert_eq!(early.queued_expired, 0);
    assert_eq!(raw_job(r.conn(), queued).await.unwrap().status, "queued");

    let due = r.sweeper.sweep(EPOCH, after(600)).await.expect("sweep");
    assert_eq!(due.queued_expired, 1);
    let row = raw_job(r.conn(), queued).await.unwrap();
    assert_eq!(
        (row.status.as_str(), row.reason_code.as_deref()),
        ("failed", Some("DISPATCH_FAILED"))
    );
    let events = audit_rows_for_job(r.conn(), queued).await;
    assert_eq!(events.len(), 1);
    assert_eq!(events[0].event, "dispatch.failed");
}

// ------------------------------------------------- expiry release (TS-28, TS-30)

/// TS-28: expiry alone never releases the allocation of an active job; once the owner is
/// terminal the next sweep releases it, and the file is still kept for the cleaner.
#[tokio::test]
#[ignore = "requires Docker for PostgreSQL testcontainer"]
async fn ts28_an_expired_written_allocation_of_a_running_job_is_kept_until_the_owner_ends() {
    let r = life_rig(10_000).await;
    let job = running_job(&r, "ts28-instance", after(30 * 24 * 3600)).await;
    let alloc = seed_alloc(
        r.conn(),
        SeedAlloc::new(AllocationState::Written, 100)
            .job_id(job)
            .version(2),
    )
    .await;
    r.put_file(alloc);
    let before = raw_alloc(r.conn(), alloc).await.unwrap();
    let past_expiry = t0() + Duration::hours(25);

    let held = r.sweeper.sweep(EPOCH, past_expiry).await.expect("sweep");

    assert_eq!(held.expired, 0);
    assert_eq!(raw_alloc(r.conn(), alloc).await.unwrap(), before);
    assert!(
        r.has_file(alloc),
        "no release, delete or unlink while the owner runs"
    );
    assert!(audit_rows_for_allocation(r.conn(), alloc).await.is_empty());

    // The worker finishes: the owner is terminal.
    let ended = r
        .raw_jobs
        .transition(
            job,
            (BackupJobStatus::Running, 4),
            BackupJobStatus::Failed,
            Some(ReasonCode::AdapterError),
            None,
        )
        .await
        .expect("worker terminal transition");
    assert_eq!(ended, TransitionResult::Applied);

    let released = r.sweeper.sweep(EPOCH, past_expiry).await.expect("sweep");

    assert_eq!(released.expired, 1);
    let a = raw_alloc(r.conn(), alloc).await.unwrap();
    assert_eq!((a.state.as_str(), a.state_version), ("release_pending", 3));
    assert!(
        r.has_file(alloc),
        "the unlink is the cleaner's audited step"
    );
    let rows = audit_rows_for_allocation(r.conn(), alloc).await;
    assert_eq!(rows.len(), 1);
    assert_eq!(rows[0].event, "allocation.reconciled");
    assert_eq!(rows[0].reason_code.as_deref(), Some("RECONCILED"));
}

/// 24 h expiry boundary over persisted state: just before, at and just after.
#[tokio::test]
#[ignore = "requires Docker for PostgreSQL testcontainer"]
async fn ts30_a_written_allocation_is_released_exactly_at_its_24_hour_expiry() {
    let r = life_rig(10_000).await;
    let job = seed(
        r.conn(),
        Seed::new("ts30-expiry", BackupJobStatus::Incomplete),
    )
    .await;
    let alloc = seed_alloc(
        r.conn(),
        SeedAlloc::new(AllocationState::Written, 100)
            .job_id(job)
            .version(2),
    )
    .await;
    let expiry = t0() + Duration::hours(24);
    let snapshot = raw_alloc(r.conn(), alloc).await.unwrap();
    assert_eq!(snapshot.expires_at, expiry);

    let early = r
        .sweeper
        .sweep(EPOCH, expiry - Duration::seconds(1))
        .await
        .expect("sweep");
    assert_eq!(early.expired, 0);
    assert_eq!(raw_alloc(r.conn(), alloc).await.unwrap(), snapshot);

    let at = r.sweeper.sweep(EPOCH, expiry).await.expect("sweep");
    assert_eq!(at.expired, 1, "released exactly at expires_at");
    assert_eq!(
        raw_alloc(r.conn(), alloc).await.unwrap().state,
        "release_pending"
    );
}

// ---------------------------------------------------- retention purge (TS-42, TS-65)

/// TS-42: the sweeper calls the injected `RetentionStore`; eligible metadata goes, owned
/// or unfinished rows stay, audit rows stay, and a repeat purge is idempotent.
#[tokio::test]
#[ignore = "requires Docker for PostgreSQL testcontainer"]
async fn ts42_the_sweep_purges_through_the_injected_retention_store_and_keeps_audit() {
    let r = life_rig(10_000).await;
    let purgeable = seed(
        r.conn(),
        Seed::new("ts42-eligible", BackupJobStatus::Failed),
    )
    .await;
    let purgeable_alloc = seed_alloc(
        r.conn(),
        SeedAlloc::new(AllocationState::Deleted, 100)
            .job_id(purgeable)
            .version(3),
    )
    .await;
    let uncleaned = seed(
        r.conn(),
        Seed::new("ts42-cleanup-failed", BackupJobStatus::Failed),
    )
    .await;
    seed_alloc(
        r.conn(),
        SeedAlloc::new(AllocationState::CleanupFailed, 100)
            .job_id(uncleaned)
            .version(3),
    )
    .await;
    let active = seed(
        r.conn(),
        Seed::new("ts42-running", BackupJobStatus::Running),
    )
    .await;
    let young = seed(
        r.conn(),
        Seed::new("ts42-not-yet", BackupJobStatus::Failed).created_at(t0() + Duration::days(5)),
    )
    .await;
    let now = t0() + Duration::days(31);
    let audit_before = table_rows(&r, "backup_audit").await;
    let mark = r.hooks.mark();

    let report = r.sweeper.sweep(EPOCH, now).await.expect("sweep");

    assert_eq!(report.purged, 1, "only the eligible job is purged");
    assert!(r
        .hooks
        .names_since(mark)
        .contains(&"retention.purge_retained".to_owned()));
    assert!(raw_job(r.conn(), purgeable).await.is_none());
    assert!(raw_alloc(r.conn(), purgeable_alloc).await.is_none());
    for kept in [uncleaned, active, young] {
        assert!(raw_job(r.conn(), kept).await.is_some(), "{kept} is kept");
    }
    let audit_after = table_rows(&r, "backup_audit").await;
    assert_eq!(
        audit_after,
        audit_before + 1,
        "one retention.purged row added; none deleted"
    );
    assert_eq!(
        count(
            r.conn(),
            "SELECT COUNT(*)::bigint AS n FROM backup_audit WHERE event = 'retention.purged'",
            vec![]
        )
        .await,
        1
    );

    let again = r.sweeper.sweep(EPOCH, now).await.expect("repeat sweep");
    assert_eq!(again.purged, 0, "idempotent");
    assert_eq!(table_rows(&r, "backup_audit").await, audit_after);
}

/// TS-65 purge path: a refused `retention.purged` audit leaves every row in place.
#[tokio::test]
#[ignore = "requires Docker for PostgreSQL testcontainer"]
async fn ts65_a_refused_purge_audit_removes_nothing() {
    let r = life_rig(10_000).await;
    let job = seed(r.conn(), Seed::new("ts65-purge", BackupJobStatus::Failed)).await;
    seed_alloc(
        r.conn(),
        SeedAlloc::new(AllocationState::Deleted, 100)
            .job_id(job)
            .version(3),
    )
    .await;
    let jobs_before = table_rows(&r, "backup_jobs").await;
    let allocs_before = table_rows(&r, "backup_allocations").await;
    let audit_before = table_rows(&r, "backup_audit").await;
    r.audit.fail_on(Some(AuditEventKind::RetentionPurged));

    let result = r.sweeper.sweep(EPOCH, t0() + Duration::days(31)).await;

    assert_eq!(result, Err(SweepError::AuditUnavailable));
    assert_eq!(table_rows(&r, "backup_jobs").await, jobs_before);
    assert_eq!(table_rows(&r, "backup_allocations").await, allocs_before);
    assert_eq!(table_rows(&r, "backup_audit").await, audit_before);
}

// ----------------------------------------------------- discovery and step order

#[tokio::test]
#[ignore = "requires Docker for PostgreSQL testcontainer"]
async fn the_sweep_discovers_an_unowned_file_without_deleting_it() {
    let r = life_rig(10_000).await;
    let orphan = Uuid::new_v4();
    r.put_file(orphan);

    let report = r.sweeper.sweep(EPOCH, t0()).await.expect("sweep");

    assert_eq!(report.discovered_orphans, 1);
    let row = raw_alloc(r.conn(), orphan).await.expect("orphan row");
    assert_eq!(row.state, "orphan_discovered");
    assert_eq!(row.orphan_grace_until, Some(t0() + Duration::hours(1)));
    assert!(r.has_file(orphan), "discover-only: nothing is unlinked");
}

/// A truncated namespace listing persists nothing from the scan, but the steps before it
/// (deadline enforcement here) stay done.
#[tokio::test]
#[ignore = "requires Docker for PostgreSQL testcontainer"]
async fn a_truncated_scan_fails_the_sweep_after_the_earlier_steps_and_persists_no_orphan() {
    let r = life_rig(1).await;
    r.put_file(Uuid::new_v4());
    r.put_file(Uuid::new_v4());
    let job = running_job(&r, "truncated-instance", after(10)).await;

    let result = r.sweeper.sweep(EPOCH, after(60)).await;

    assert_eq!(result, Err(SweepError::InventoryTruncated));
    assert_eq!(
        raw_job(r.conn(), job).await.unwrap().status,
        "failed",
        "the deadline step ran before the scan"
    );
    assert_eq!(
        table_rows(&r, "backup_allocations").await,
        0,
        "no orphan row"
    );
}

/// Step order of one sweep: deadline, expiry release, retention purge, orphan scan.
#[tokio::test]
#[ignore = "requires Docker for PostgreSQL testcontainer"]
async fn one_sweep_runs_deadline_then_expiry_then_purge_then_discovery() {
    let r = life_rig(10_000).await;
    let now = t0() + Duration::days(31);
    // deadline: a current-epoch running job past its deadline
    running_job(&r, "order-deadline", now - Duration::seconds(5)).await;
    // expiry: a written allocation of an incomplete job past its 24 h expiry
    let incomplete = seed(
        r.conn(),
        Seed::new("order-expiry", BackupJobStatus::Incomplete),
    )
    .await;
    seed_alloc(
        r.conn(),
        SeedAlloc::new(AllocationState::Written, 100)
            .job_id(incomplete)
            .version(1),
    )
    .await;
    // purge: a failed job past retention with no allocation
    seed(r.conn(), Seed::new("order-purge", BackupJobStatus::Failed)).await;
    // discovery: an unowned file
    r.put_file(Uuid::new_v4());
    let mark = r.hooks.mark();

    let report = r.sweeper.sweep(EPOCH, now).await.expect("sweep");

    assert_eq!(
        report,
        SweepReport {
            deadlines: 1,
            queued_expired: 0,
            expired: 1,
            terminal_released: 0,
            // the job failed by the deadline step is already past retention, so the
            // purge step that follows removes it together with the seeded failed job
            purged: 2,
            discovered_orphans: 1,
        }
    );
    let audits: Vec<String> = r
        .hooks
        .names_since(mark)
        .into_iter()
        .filter(|n| n.starts_with("audit."))
        .collect();
    assert_eq!(
        audits,
        vec![
            "audit.job.failed",
            "audit.allocation.reconciled",
            "audit.retention.purged",
            "audit.orphan.discovered"
        ]
    );
}
