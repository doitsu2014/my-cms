//! WU-19 red tests, PostgreSQL-backed: the `Runtime` over the real reconciler, cleaner and
//! sweeper, a real `FsNamespaceLock` and the real audit sink (TS-32 start-up barrier,
//! TS-29 outage and recovery order, the 60 s and 15 min schedule of TS-30, the lifecycle
//! half of TS-56).
//!
//! The recording wrappers of `lifecycle_support` log every store, storage and audit call
//! together with whether admissions were open at that moment. "Closed until the last
//! step" is therefore read from the log, not assumed.

use super::alloc_support::{raw_alloc, seed_alloc, SeedAlloc};
use super::lifecycle_support::{after, life_rig, LifeRig};
use super::support::{count, raw_job, seed, Seed};
use crate::domain::status::BackupJobStatus;
use crate::infra::fs_namespace_lock::FsNamespaceLock;
use crate::lifecycle::cleaner::CleanOutcome;
use crate::lifecycle::runtime::{Runtime, RuntimeConfig, RuntimeError, RuntimeState, TickReport};
use crate::ports::allocation_store::AllocationState;
use crate::ports::audit::AuditEventKind;
use crate::ports::namespace_lock::NamespaceLock;
use std::path::Path;
use std::sync::atomic::Ordering;
use std::sync::Arc;
use uuid::Uuid;

/// A job of an older epoch (0) with its `registered` allocation and the file it created.
async fn old_work(r: &LifeRig) -> (Uuid, Uuid) {
    let job = seed(
        r.conn(),
        Seed::new("old-epoch-instance", BackupJobStatus::Running).version(4),
    )
    .await;
    r.set_job_epoch(job, 0).await;
    let alloc = seed_alloc(
        r.conn(),
        SeedAlloc::new(AllocationState::Registered, 100)
            .job_id(job)
            .version(2),
    )
    .await;
    r.put_file(alloc);
    (job, alloc)
}

fn position(names: &[String], name: &str) -> usize {
    names
        .iter()
        .position(|n| n == name)
        .unwrap_or_else(|| panic!("{name} not in {names:?}"))
}

fn audits(names: &[String]) -> Vec<&str> {
    names
        .iter()
        .filter_map(|n| n.strip_prefix("audit."))
        .collect()
}

// ------------------------------------------------------------------ TS-32

/// First start: lock, durable epoch advance, complete reconcile and fence, start-up
/// cleanup, and only then admissions. Admissions are closed at every earlier step.
#[tokio::test]
#[ignore = "requires Docker for PostgreSQL testcontainer"]
async fn ts32_startup_runs_lock_epoch_reconcile_cleanup_then_opens_admissions() {
    let r = life_rig(10_000).await;
    let (job, alloc) = old_work(&r).await;
    assert!(!r.runtime.admissions_open());
    assert_eq!(r.runtime.state(), RuntimeState::NotReady);
    let mark = r.hooks.mark();

    r.runtime.start().await.expect("start");

    let names = r.hooks.names_since(mark);
    assert!(position(&names, "jobs.advance_epoch") < position(&names, "storage.inventory"));
    assert!(
        position(&names, "storage.inventory") < position(&names, "jobs.list_for_reconciliation")
    );
    assert!(position(&names, "jobs.fence_job") < position(&names, "allocs.reconcile_state"));
    assert!(
        position(&names, "allocs.reconcile_state") < position(&names, "allocs.due_for_cleanup")
    );
    assert!(
        position(&names, "allocs.due_for_cleanup") < position(&names, "allocs.claim_for_delete")
    );
    assert!(position(&names, "allocs.claim_for_delete") < position(&names, "storage.delete"));
    assert_eq!(
        r.hooks.open_entries_since(mark),
        Vec::<String>::new(),
        "admissions stay closed at every step before the last"
    );
    assert_eq!(
        *r.hooks.lock_held_at_epoch.lock().unwrap(),
        Some(true),
        "the namespace lock was already held when the epoch advanced"
    );
    assert!(r.runtime.admissions_open());
    assert_eq!(r.runtime.state(), RuntimeState::Ready);
    assert_eq!(r.runtime.epoch(), Some(1));
    assert_eq!(r.epoch_row().await, Some(1));
    assert_eq!(
        audits(&names),
        vec![
            "reconcile.fenced",
            "allocation.reconciled",
            "allocation.delete_claimed",
            "allocation.deleted"
        ]
    );
    assert_eq!(raw_job(r.conn(), job).await.unwrap().status, "failed");
    assert_eq!(raw_alloc(r.conn(), alloc).await.unwrap().state, "deleted");
    assert!(!r.has_file(alloc));
}

/// A second runtime cannot take the lock and changes nothing.
#[tokio::test]
#[ignore = "requires Docker for PostgreSQL testcontainer"]
async fn ts32_a_second_runtime_cannot_acquire_the_lock_and_mutates_nothing() {
    let r = life_rig(10_000).await;
    r.runtime.start().await.expect("first runtime starts");
    let (job, alloc) = old_work(&r).await;
    let second = Runtime::new(
        r.reconciler.clone(),
        r.cleaner.clone(),
        r.sweeper.clone(),
        Arc::new(|root: &Path| FsNamespaceLock::try_acquire(root)),
        r.clock.clone(),
        RuntimeConfig::new(r.tmp.root()),
    );
    let (job_before, alloc_before) = (
        raw_job(r.conn(), job).await.unwrap(),
        raw_alloc(r.conn(), alloc).await.unwrap(),
    );
    let (epoch_before, audit_before, listing_before) =
        (r.epoch_row().await, r.audit_total().await, r.listing());

    let result = second.start().await;

    assert_eq!(result, Err(RuntimeError::LockUnavailable));
    assert!(!second.admissions_open());
    assert_eq!(second.state(), RuntimeState::NotReady);
    assert_eq!(raw_job(r.conn(), job).await.unwrap(), job_before);
    assert_eq!(raw_alloc(r.conn(), alloc).await.unwrap(), alloc_before);
    assert_eq!(r.epoch_row().await, epoch_before, "no epoch advance");
    assert_eq!(r.audit_total().await, audit_before);
    assert_eq!(r.listing(), listing_before);
    assert!(
        r.runtime.admissions_open(),
        "the first runtime is unaffected"
    );
}

/// TS-56 lifecycle half: a truncated listing keeps start-up closed and not ready, writes
/// nothing, and releases the lock.
#[tokio::test]
#[ignore = "requires Docker for PostgreSQL testcontainer"]
async fn ts56_a_truncated_inventory_keeps_startup_closed_and_not_ready() {
    let r = life_rig(1).await;
    let (job, alloc) = old_work(&r).await;
    r.put_file(Uuid::new_v4());
    let (job_before, alloc_before, listing_before) = (
        raw_job(r.conn(), job).await.unwrap(),
        raw_alloc(r.conn(), alloc).await.unwrap(),
        r.listing(),
    );
    let audit_before = r.audit_total().await;

    let result = r.runtime.start().await;

    assert_eq!(result, Err(RuntimeError::InventoryTruncated));
    assert!(!r.runtime.admissions_open());
    assert_eq!(r.runtime.state(), RuntimeState::NotReady);
    assert_eq!(raw_job(r.conn(), job).await.unwrap(), job_before);
    assert_eq!(raw_alloc(r.conn(), alloc).await.unwrap(), alloc_before);
    assert_eq!(r.listing(), listing_before);
    assert_eq!(r.audit_total().await, audit_before);
    assert!(
        FsNamespaceLock::try_acquire(&r.tmp.root()).is_ok(),
        "a failed start-up does not keep the lock"
    );
}

#[tokio::test]
#[ignore = "requires Docker for PostgreSQL testcontainer"]
async fn a_runtime_that_was_never_started_refuses_ticks_and_recovery() {
    let r = life_rig(10_000).await;

    assert_eq!(r.runtime.run_tick().await, Err(RuntimeError::NotStarted));
    assert_eq!(
        r.runtime.database_recovered().await,
        Err(RuntimeError::NotStarted)
    );
    assert!(!r.runtime.admissions_open());
    assert_eq!(r.hooks.calls(), 0, "no store call was made");
}

// ------------------------------------------------------------------ TS-29

/// Outage with persisted pending work and a real file: admissions close, store work stops,
/// nothing changes, and the safe state is `Degraded`.
#[tokio::test]
#[ignore = "requires Docker for PostgreSQL testcontainer"]
async fn ts29_a_store_outage_closes_admissions_and_stops_all_store_work() {
    let r = life_rig(10_000).await;
    r.runtime.start().await.expect("start");
    let job = seed(
        r.conn(),
        Seed::new("outage-instance", BackupJobStatus::Running).version(4),
    )
    .await;
    r.set_deadline(job, after(10)).await;
    let alloc = seed_alloc(
        r.conn(),
        SeedAlloc::new(AllocationState::ReleasePending, 100).version(2),
    )
    .await;
    r.put_file(alloc);
    r.clock.set(after(20));
    r.hooks.outage.store(true, Ordering::SeqCst);

    let failed = r.runtime.run_tick().await;

    assert_eq!(failed, Err(RuntimeError::StoreUnavailable));
    assert!(!r.runtime.admissions_open(), "the outage closes admissions");
    assert_eq!(r.runtime.state(), RuntimeState::Degraded);

    let calls = r.hooks.calls();
    let idle = r.runtime.run_tick().await.expect("a closed runtime idles");
    assert_eq!(idle, TickReport::default());
    assert_eq!(
        r.hooks.calls(),
        calls,
        "no allocation, delete or sweep call"
    );

    assert_eq!(raw_job(r.conn(), job).await.unwrap().status, "running");
    assert_eq!(
        raw_alloc(r.conn(), alloc).await.unwrap().state,
        "release_pending"
    );
    assert!(r.has_file(alloc), "the file is retained");
}

/// Recovery: reconcile (no epoch bump), cleanup, sweep, and only then reopen.
#[tokio::test]
#[ignore = "requires Docker for PostgreSQL testcontainer"]
async fn ts29_recovery_reconciles_then_cleans_then_sweeps_then_reopens_without_a_new_epoch() {
    let r = life_rig(10_000).await;
    r.runtime.start().await.expect("start");
    let job = seed(
        r.conn(),
        Seed::new("recovery-instance", BackupJobStatus::Running).version(4),
    )
    .await;
    r.set_deadline(job, after(10)).await;
    let alloc = seed_alloc(
        r.conn(),
        SeedAlloc::new(AllocationState::ReleasePending, 100).version(2),
    )
    .await;
    r.put_file(alloc);
    r.clock.set(after(20));
    r.hooks.outage.store(true, Ordering::SeqCst);
    r.runtime.run_tick().await.expect_err("outage");
    r.hooks.outage.store(false, Ordering::SeqCst);
    let mark = r.hooks.mark();

    r.runtime.database_recovered().await.expect("recovery");

    let names = r.hooks.names_since(mark);
    assert!(
        position(&names, "storage.inventory") < position(&names, "jobs.list_for_reconciliation")
    );
    assert!(
        position(&names, "jobs.list_for_reconciliation")
            < position(&names, "allocs.list_for_reconciliation")
    );
    assert!(
        position(&names, "allocs.list_for_reconciliation")
            < position(&names, "allocs.due_for_cleanup")
    );
    assert!(
        position(&names, "allocs.due_for_cleanup") < position(&names, "jobs.list_deadline_expired")
    );
    assert!(
        position(&names, "jobs.list_deadline_expired")
            < position(&names, "retention.purge_retained")
    );
    assert!(
        !names.contains(&"jobs.advance_epoch".to_owned()),
        "the process did not restart: no epoch bump"
    );
    assert_eq!(
        r.hooks.open_entries_since(mark),
        Vec::<String>::new(),
        "admissions stay closed through reconcile, cleanup and sweep"
    );
    assert!(r.runtime.admissions_open());
    assert_eq!(r.runtime.state(), RuntimeState::Ready);
    assert_eq!(r.epoch_row().await, Some(1));
    assert_eq!(raw_alloc(r.conn(), alloc).await.unwrap().state, "deleted");
    assert!(!r.has_file(alloc));
    let failed = raw_job(r.conn(), job).await.unwrap();
    assert_eq!(
        (failed.status.as_str(), failed.reason_code.as_deref()),
        ("failed", Some("DEADLINE_EXCEEDED"))
    );
}

/// A truncated listing during recovery leaves admissions closed and the runtime degraded;
/// once the listing is complete again a later recovery succeeds.
#[tokio::test]
#[ignore = "requires Docker for PostgreSQL testcontainer"]
async fn ts29_a_truncated_recovery_stays_closed_and_degraded_until_it_succeeds() {
    let r = life_rig(1).await;
    r.runtime
        .start()
        .await
        .expect("start with an empty namespace");
    r.hooks.outage.store(true, Ordering::SeqCst);
    r.runtime.run_tick().await.expect_err("outage");
    r.hooks.outage.store(false, Ordering::SeqCst);
    let extra = [Uuid::new_v4(), Uuid::new_v4()];
    for id in extra {
        r.put_file(id);
    }
    let audit_before = r.audit_total().await;

    let truncated = r.runtime.database_recovered().await;

    assert_eq!(truncated, Err(RuntimeError::InventoryTruncated));
    assert!(!r.runtime.admissions_open());
    assert_eq!(r.runtime.state(), RuntimeState::Degraded);
    assert_eq!(r.audit_total().await, audit_before, "nothing was written");
    assert_eq!(
        count(
            r.conn(),
            "SELECT COUNT(*)::bigint AS n FROM backup_allocations",
            vec![]
        )
        .await,
        0
    );

    for id in extra {
        std::fs::remove_file(r.tmp.root().join("alloc").join(format!("{id}.bin"))).unwrap();
    }
    r.runtime.database_recovered().await.expect("recovery");
    assert!(r.runtime.admissions_open());
    assert_eq!(r.runtime.state(), RuntimeState::Ready);
}

/// FD 6.7 audit-blocked bound: three consecutive refused sweeps close admissions and
/// degrade the runtime; the job stays `running` until the audit recovers.
#[tokio::test]
#[ignore = "requires Docker for PostgreSQL testcontainer"]
async fn ts29_three_refused_audit_sweeps_close_admissions_and_degrade() {
    let r = life_rig(10_000).await;
    r.runtime.start().await.expect("start");
    let job = seed(
        r.conn(),
        Seed::new("audit-blocked-instance", BackupJobStatus::Running).version(4),
    )
    .await;
    r.set_deadline(job, after(10)).await;
    r.audit.fail_on(Some(AuditEventKind::JobFailed));

    for (tick, at) in [(1, 1000), (2, 1001)] {
        r.clock.set(after(at));
        assert_eq!(
            r.runtime.run_tick().await,
            Err(RuntimeError::AuditUnavailable),
            "tick {tick}"
        );
        assert!(
            r.runtime.admissions_open(),
            "still open after refusal {tick}"
        );
        assert_eq!(r.runtime.state(), RuntimeState::Ready);
    }
    r.clock.set(after(1002));
    assert_eq!(
        r.runtime.run_tick().await,
        Err(RuntimeError::AuditUnavailable)
    );
    assert!(
        !r.runtime.admissions_open(),
        "closed after the third refusal"
    );
    assert_eq!(r.runtime.state(), RuntimeState::Degraded);
    assert_eq!(
        raw_job(r.conn(), job).await.unwrap().status,
        "running",
        "no unaudited effect: the job keeps its slot"
    );

    r.audit.fail_on(None);
    r.runtime.database_recovered().await.expect("recovery");
    assert!(r.runtime.admissions_open());
    assert_eq!(raw_job(r.conn(), job).await.unwrap().status, "failed");
}

/// The refusal bound counts consecutive refusals: a sweep that succeeds in between resets
/// it, so two refusals, a good sweep and two more refusals never close admissions.
#[tokio::test]
#[ignore = "requires Docker for PostgreSQL testcontainer"]
async fn ts29_a_good_sweep_resets_the_refused_audit_count() {
    let r = life_rig(10_000).await;
    r.runtime.start().await.expect("start");
    let first = seed(
        r.conn(),
        Seed::new("first-blocked-instance", BackupJobStatus::Running).version(4),
    )
    .await;
    r.set_deadline(first, after(10)).await;
    r.audit.fail_on(Some(AuditEventKind::JobFailed));
    for at in [1000, 1001] {
        r.clock.set(after(at));
        assert_eq!(
            r.runtime.run_tick().await,
            Err(RuntimeError::AuditUnavailable)
        );
    }

    r.audit.fail_on(None);
    r.clock.set(after(1002));
    let good = r.runtime.run_tick().await.expect("the audit is back");
    assert_eq!(good.swept.expect("swept").deadlines, 1);

    let second = seed(
        r.conn(),
        Seed::new("second-blocked-instance", BackupJobStatus::Running).version(4),
    )
    .await;
    r.set_deadline(second, after(1500)).await;
    r.audit.fail_on(Some(AuditEventKind::JobFailed));
    for at in [1902, 1903] {
        r.clock.set(after(at));
        assert_eq!(
            r.runtime.run_tick().await,
            Err(RuntimeError::AuditUnavailable),
            "refusal at {at}"
        );
    }

    assert!(
        r.runtime.admissions_open(),
        "two consecutive refusals after a good sweep stay below the bound"
    );
    assert_eq!(r.runtime.state(), RuntimeState::Ready);
}

/// A truncated sweep scan degrades the runtime without closing admissions, and the next
/// complete sweep clears the signal.
#[tokio::test]
#[ignore = "requires Docker for PostgreSQL testcontainer"]
async fn ts56_a_truncated_sweep_scan_degrades_but_keeps_admissions_open() {
    let r = life_rig(1).await;
    r.runtime
        .start()
        .await
        .expect("start with an empty namespace");
    let extra = [Uuid::new_v4(), Uuid::new_v4()];
    for id in extra {
        r.put_file(id);
    }

    r.clock.set(after(1000));
    assert_eq!(
        r.runtime.run_tick().await,
        Err(RuntimeError::InventoryTruncated)
    );
    assert!(
        r.runtime.admissions_open(),
        "a truncated scan is not an outage"
    );
    assert_eq!(r.runtime.state(), RuntimeState::Degraded);

    for id in extra {
        std::fs::remove_file(r.tmp.root().join("alloc").join(format!("{id}.bin"))).unwrap();
    }
    r.clock.set(after(2000));
    r.runtime.run_tick().await.expect("a complete scan");
    assert_eq!(r.runtime.state(), RuntimeState::Ready);
}

/// Recovery stops at a failed reconcile: no cleanup or sweep runs on top of an unreconciled
/// store and admissions stay closed, even when a later step could have succeeded.
#[tokio::test]
#[ignore = "requires Docker for PostgreSQL testcontainer"]
async fn ts29_a_refused_reconcile_audit_keeps_recovery_closed() {
    let r = life_rig(10_000).await;
    r.runtime.start().await.expect("start");
    let (job, _alloc) = old_work(&r).await;
    r.hooks.outage.store(true, Ordering::SeqCst);
    r.runtime.run_tick().await.expect_err("outage");
    r.hooks.outage.store(false, Ordering::SeqCst);
    r.audit.fail_on(Some(AuditEventKind::ReconcileFenced));

    let refused = r.runtime.database_recovered().await;

    assert_eq!(refused, Err(RuntimeError::AuditUnavailable));
    assert!(!r.runtime.admissions_open());
    assert_eq!(r.runtime.state(), RuntimeState::Degraded);
    assert_eq!(
        raw_job(r.conn(), job).await.unwrap().status,
        "running",
        "the unaudited fence did not happen"
    );

    r.audit.fail_on(None);
    r.runtime.database_recovered().await.expect("recovery");
    assert!(r.runtime.admissions_open());
    assert_eq!(raw_job(r.conn(), job).await.unwrap().status, "failed");
}

// ------------------------------------------------------------------ TS-30

/// The cleaner tick (60 s) and the sweep (15 min) fire exactly at their boundaries.
#[tokio::test]
#[ignore = "requires Docker for PostgreSQL testcontainer"]
async fn ts30_the_cleaner_ticks_every_60_seconds_and_the_sweep_every_15_minutes() {
    let r = life_rig(10_000).await;
    r.runtime.start().await.expect("start");
    let job = seed(
        r.conn(),
        Seed::new("schedule-instance", BackupJobStatus::Running).version(4),
    )
    .await;
    r.set_deadline(job, after(899)).await;
    let first = seed_alloc(
        r.conn(),
        SeedAlloc::new(AllocationState::ReleasePending, 100).version(2),
    )
    .await;
    r.put_file(first);

    // First tick: both are due.
    let tick = r.runtime.run_tick().await.expect("first tick");
    assert_eq!(tick.cleaned, Some(vec![CleanOutcome::Deleted]));
    assert!(tick.swept.is_some(), "the sweep runs at start-up");
    assert_eq!(raw_job(r.conn(), job).await.unwrap().status, "running");

    let second = seed_alloc(
        r.conn(),
        SeedAlloc::new(AllocationState::ReleasePending, 100).version(2),
    )
    .await;
    r.put_file(second);

    r.clock.set(after(59));
    let early = r.runtime.run_tick().await.expect("tick at 59 s");
    assert_eq!((early.cleaned, early.swept), (None, None), "neither is due");
    assert!(r.has_file(second));

    r.clock.set(after(60));
    let clean = r.runtime.run_tick().await.expect("tick at 60 s");
    assert_eq!(clean.cleaned, Some(vec![CleanOutcome::Deleted]));
    assert_eq!(clean.swept, None, "the sweep is not due at 60 s");
    assert!(!r.has_file(second));

    r.clock.set(after(899));
    let before_sweep = r.runtime.run_tick().await.expect("tick at 899 s");
    assert_eq!(before_sweep.swept, None, "one second before the 15 minutes");
    assert_eq!(
        raw_job(r.conn(), job).await.unwrap().status,
        "running",
        "the deadline has passed but the sweep has not run"
    );

    r.clock.set(after(900));
    let sweep = r.runtime.run_tick().await.expect("tick at 900 s");
    assert_eq!(sweep.swept.map(|s| s.deadlines), Some(1));
    assert_eq!(raw_job(r.conn(), job).await.unwrap().status, "failed");
}
