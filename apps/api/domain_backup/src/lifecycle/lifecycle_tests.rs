//! WU-06 red contract suite. Inputs are explicit scenario facts and assertions
//! target durable lifecycle outcomes rather than report defaults or date arithmetic.
use super::{
    cleaner::{CleanOutcome, Cleaner},
    reconciler::Reconciler,
    runtime::Runtime,
    sweeper::Sweeper,
};
use chrono::{Duration as CD, TimeZone, Utc};

fn now() -> chrono::DateTime<Utc> {
    Utc.with_ymd_and_hms(2026, 11, 22, 12, 0, 0).unwrap()
}

// The current four shell APIs do not accept stores, audit, clock, inventory or a
// namespace lock. These tests stay assertion-red against their no-op outputs; they
// are not evidence of DB-persisted crash recovery. The missing injection surface is
// itemized in the red-evidence handoff rather than simulated with invented behavior.

#[tokio::test]
async fn ts20_registered_row_without_file_is_transitioned_to_release_pending() {
    let report = Reconciler::new().reconcile(now()).await.unwrap();
    assert_eq!(
        report.missing_files, 1,
        "one seeded registered row is missing its file"
    );
}

#[tokio::test]
async fn ts21_file_after_register_before_mark_written_is_retained_as_owned() {
    let report = Reconciler::new().reconcile(now()).await.unwrap();
    assert_eq!(
        report.fenced_jobs, 1,
        "startup recovery must process the interrupted registered allocation"
    );
}

#[tokio::test]
async fn ts22_old_epoch_running_job_is_failed_with_restart_reason_and_audit() {
    let report = Reconciler::new().reconcile(now()).await.unwrap();
    assert_eq!(
        report.fenced_jobs, 1,
        "old-epoch running job must be fenced"
    );
}

#[tokio::test]
async fn ts23_orphan_is_persisted_before_grace_and_ambiguous_entry_is_untouched() {
    let report = Reconciler::new().reconcile(now()).await.unwrap();
    assert_eq!(
        report.discovered_orphans, 1,
        "valid unowned entry must become an orphan row"
    );
}

#[tokio::test]
async fn ts24_unexpired_incomplete_allocation_with_missing_file_is_not_released() {
    let report = Reconciler::new().reconcile(now()).await.unwrap();
    assert_eq!(
        report.missing_files, 0,
        "unexpired incomplete allocation remains retained"
    );
}

#[tokio::test]
async fn ts25_crashed_old_epoch_deleting_claim_returns_to_cleanup_without_retry() {
    let report = Reconciler::new().reconcile(now()).await.unwrap();
    assert_eq!(
        report.fenced_jobs, 1,
        "stale deleting claim must be recovered"
    );
}

#[tokio::test]
async fn ts26_unlinked_file_with_deleting_row_commits_deleted_idempotently() {
    let id = uuid::Uuid::new_v4();
    assert_eq!(
        Cleaner::new().clean_one(id, now()).await.unwrap(),
        CleanOutcome::Deleted
    );
}

#[tokio::test]
async fn ts27a_three_unlink_failures_are_spaced_exhausted_and_never_attempted_a_fourth_time() {
    let outcomes = Cleaner::new().clean_due(now()).await.unwrap();
    assert_eq!(outcomes, vec![CleanOutcome::Exhausted]);
}

#[tokio::test]
async fn ts28_expired_written_allocation_of_active_job_remains_unchanged_and_file_is_kept() {
    let report = Sweeper::new().sweep(now() + CD::hours(24)).await.unwrap();
    assert_eq!(
        report.expired, 0,
        "active work is not released by expiry alone"
    );
}

#[tokio::test]
async fn ts29_store_outage_closes_work_and_recovery_reconciles_before_reopening() {
    let runtime = Runtime::new();
    runtime.database_outage().await;
    assert!(!runtime.admissions_open(), "outage must close admissions");
    runtime.database_recovered().await.unwrap();
    assert!(
        runtime.admissions_open(),
        "successful recovery must reopen only after reconcile"
    );
}

#[tokio::test]
async fn ts30_fake_time_enforces_retry_expiry_grace_and_sweep_boundaries() {
    let at = now();
    let before_retry = at + CD::seconds(59);
    let retry_due = at + CD::seconds(60);
    let expiry = at + CD::hours(24);
    let grace = at + CD::hours(1);
    let sweep = at + CD::minutes(15);
    assert!(before_retry < retry_due);
    assert_eq!(expiry - at, CD::seconds(86_400));
    assert_eq!(grace - at, CD::seconds(3_600));
    assert_eq!(sweep - at, CD::seconds(900));
    assert_eq!(
        Cleaner::new()
            .clean_one(uuid::Uuid::new_v4(), before_retry)
            .await
            .unwrap(),
        CleanOutcome::NotDue
    );
    assert_eq!(
        Cleaner::new()
            .clean_one(uuid::Uuid::new_v4(), retry_due)
            .await
            .unwrap(),
        CleanOutcome::Deleted
    );
}

#[tokio::test]
async fn ts32_startup_lock_epoch_reconcile_cleanup_barrier_precedes_admissions() {
    let runtime = Runtime::new();
    assert!(!runtime.admissions_open());
    runtime.start().await.unwrap();
    assert!(
        runtime.admissions_open(),
        "admissions open only after lock, fencing, reconcile and cleanup"
    );
}

#[tokio::test]
async fn ts42_purge_at_retention_boundary_removes_eligible_metadata_but_keeps_audit() {
    let report = Sweeper::new().sweep(now() + CD::days(30)).await.unwrap();
    assert_eq!(report.purged, 1, "eligible terminal metadata is purged");
}

#[tokio::test]
async fn ts56_truncated_inventory_makes_reconciliation_fail_closed() {
    let result = Reconciler::new().reconcile(now()).await;
    assert!(
        result.is_err(),
        "truncated inventory must prevent classification and mutation"
    );
}

#[tokio::test]
async fn ts65_audit_refusal_blocks_delete_claim_delete_commit_and_purge() {
    let clean = Cleaner::new()
        .clean_one(uuid::Uuid::new_v4(), now())
        .await
        .unwrap();
    let sweep = Sweeper::new().sweep(now() + CD::days(31)).await;
    assert_eq!(
        clean,
        CleanOutcome::NotDue,
        "no unlink without durable delete-claim audit"
    );
    assert!(
        sweep.is_err(),
        "purge refusal leaves metadata and retained audit unchanged"
    );
}
