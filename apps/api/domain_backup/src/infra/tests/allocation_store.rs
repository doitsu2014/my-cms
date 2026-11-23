//! `DbAllocationStore` red tests (PostgreSQL 15 testcontainer; opt-in with
//! `-- --ignored`). Register-before-write and the ledger audit coupling (FN-21),
//! durable budgets (TS-30 budget part: 256 MiB per allocation, 1 GiB total, CFG-12,
//! CFG-13), compare-and-set moves (FN-23), cleanup bookkeeping and orphan rows
//! (unit-own, FD 4.4). AC-003 S3, AC-002 S2, AC-006 S2 (budgets, gate trace);
//! T-11, T-12, T-13.
//!
//! Contract deviations from IFC section 9 that need conductor sign-off before WU-05 and
//! WU-06 build on them (all five are encoded by these tests):
//! 1. every `AllocationStore` method returns `AllocationError` (with `BudgetExceeded`
//!    and `Lost`), not the contract's `StoreError`;
//! 2. `StorageKey::parse(&str)` is a second public constructor (IFC 9: "constructed only
//!    from an allocation id"); it is the sanctioned path for inventory names only;
//! 3. `FsFixtureStorage::open(root, Arc<dyn AllocationStore>)` couples file storage to
//!    the ledger so a write without a registered allocation is refused;
//! 4. `AllocationStore::get` is added;
//! 5. `AllocationLimits` and `DbAllocationStore::new(conn, audit, clock, instance_id,
//!    limits)` are new public shapes.
//!
//! Design choices recorded for the conductor:
//! - `claim_for_delete` stamps the row with the CURRENT epoch read from `backup_epoch`
//!   (IFC 6.1a: only an older-epoch `deleting` row is a crashed claim); with no
//!   `backup_epoch` row the row keeps its own epoch.
//! - `insert_orphan` is idempotent for an existing id (a no-op that keeps the first
//!   grace and writes no second audit row) and writes `orphan.discovered` in its own
//!   transaction (the producer of that event; WU-06 TS-44b consumes it).
//! - A 0-byte `register` is accepted (the bound is `<=`).
//! - `AllocationStore::get` is added to the IFC list so `FsFixtureStorage` can refuse a
//!   write without a registered allocation (the IFC trait has no read method).
//! - `DbAllocationStore::new(conn, audit, clock, instance_id, AllocationLimits)`: the
//!   instance id is a constructor argument because `register` carries none, and the
//!   limits carry the configured bounds (CFG-06, 08, 09, 12, 13).
//! - `insert_orphan` takes the epoch from the `backup_epoch` row and never refuses for
//!   budget: the file already exists, so the bytes are reality (FD 4.4, bytes capped at
//!   CFG-12).

use super::alloc_support::*;
use super::fs_support::{names_in, TempDir};
use super::support::*;
use crate::infra::db_allocation_store::DbAllocationStore;
use crate::infra::db_audit_sink::DbAuditSink;
use crate::infra::fs_fixture_storage::FsFixtureStorage;
use crate::ports::allocation_store::{
    AllocationError, AllocationLimits, AllocationState::*, AllocationStore, AllocationTotals,
    BudgetKind, CleanupFailureOutcome,
};
use crate::ports::audit::AuditEventKind;
use crate::ports::fixture_storage::{FixtureStorage, StorageError, StorageKey};
use crate::ports::job_store::ReasonCode;
use chrono::Duration;
use sea_orm::ConnectionTrait;
use std::collections::HashSet;
use std::sync::Arc;
use tokio::sync::Barrier;
use uuid::Uuid;

async fn running_job(fx: &AFx) -> Uuid {
    seed(
        &fx.conn,
        Seed::new(INSTANCE, crate::domain::status::BackupJobStatus::Running),
    )
    .await
}

// ---------- register: row, audit, fail-closed (FN-21) ----------

/// FN-21, AC-003 S3, T-12: `register` commits a `registered` row with the derived opaque
/// key, the given epoch, size and expiry, before the caller can write any byte.
#[tokio::test]
#[ignore = "requires Docker for PostgreSQL testcontainer"]
async fn register_commits_a_registered_row_with_the_opaque_key() {
    let fx = afx().await;
    let job = running_job(&fx).await;
    let a = fx
        .store
        .register(Some(job), 1000, 5, expires())
        .await
        .expect("register");
    assert_eq!(a.storage_key, StorageKey::from_allocation_id(a.id));
    assert_eq!(
        (a.state, a.state_version, a.epoch, a.bytes_reserved),
        (Registered, 0, 5, 1000)
    );
    assert_eq!(a.job_id, Some(job));

    let raw = require_alloc(&fx.conn, a.id).await;
    assert_eq!(raw.state, "registered");
    assert_eq!(raw.storage_key, format!("alloc/{}.bin", a.id));
    assert_eq!(
        (raw.state_version, raw.epoch, raw.bytes_reserved),
        (0, 5, 1000)
    );
    assert_eq!(raw.job_id, Some(job));
    assert_eq!(raw.expires_at, expires());
    assert_eq!(raw.created_at, t0());
    assert_eq!(raw.cleanup_attempts, 0);
    assert!(raw.orphan_grace_until.is_none() && raw.deleted_at.is_none());
}

/// FN-21, unit-own (same-transaction rule, IFC 6.3), T-08: the `allocation.registered`
/// audit row is written on the
/// store's own transaction, before commit, and carries the allocation id.
#[tokio::test]
#[ignore = "requires Docker for PostgreSQL testcontainer"]
async fn register_writes_allocation_registered_audit_in_the_same_transaction() {
    let fx = afx().await;
    let probe = Arc::new(AllocProbeSink::new(&fx.conn));
    let store = astore(&fx.conn, probe.clone());
    let job = running_job(&fx).await;
    let a = store
        .register(Some(job), 64, 1, expires())
        .await
        .expect("register");

    let seen = probe.seen.lock().unwrap().clone();
    assert_eq!(seen.len(), 1, "exactly one audit call: {seen:?}");
    assert!(
        seen[0].saw_tx,
        "the sink must receive the store's transaction"
    );
    assert_eq!(seen[0].event, AuditEventKind::AllocationRegistered);
    assert_eq!(
        seen[0].outside_state, None,
        "the ledger row must still be uncommitted while the audit row is written"
    );
    assert_eq!(
        audit_events_for_allocation(&fx.conn, a.id).await,
        vec![("allocation.registered".to_owned(), "success".to_owned())]
    );
    let row = fx
        .conn
        .query_one(pg(
            "SELECT instance_id, job_id, actor_sub FROM backup_audit WHERE allocation_id = $1",
            vec![a.id.into()],
        ))
        .await
        .unwrap()
        .expect("audit row");
    assert_eq!(row.try_get::<String>("", "instance_id").unwrap(), INSTANCE);
    assert_eq!(
        row.try_get::<Option<Uuid>>("", "job_id").unwrap(),
        Some(job)
    );
    assert_eq!(
        row.try_get::<Option<String>>("", "actor_sub").unwrap(),
        None
    );
}

/// FN-21, IFC 6.3, T-08: a failing audit refuses the registration. No ledger row and no
/// audit row survives, including when the sink wrote its row before failing.
#[tokio::test]
#[ignore = "requires Docker for PostgreSQL testcontainer"]
async fn register_is_refused_and_rolled_back_when_audit_fails() {
    let fx = afx().await;
    let r = astore(&fx.conn, Arc::new(FailingAuditSink))
        .register(None, 10, 1, expires())
        .await;
    assert_eq!(r.err(), Some(AllocationError::AuditUnavailable));
    let r = astore(&fx.conn, Arc::new(OutageAuditSink))
        .register(None, 10, 1, expires())
        .await;
    assert_eq!(r.err(), Some(AllocationError::StoreUnavailable));
    let r = astore(&fx.conn, Arc::new(WriteThenFailSink::new(&fx.conn)))
        .register(None, 10, 1, expires())
        .await;
    assert_eq!(r.err(), Some(AllocationError::AuditUnavailable));
    assert_eq!(
        alloc_rows(&fx.conn).await,
        0,
        "no ledger row may survive a refused audit"
    );
    assert_eq!(
        audit_rows_total(&fx.conn).await,
        0,
        "no audit row may survive the rollback"
    );
}

// ---------- budgets (TS-30 budget part, CFG-12, CFG-13) ----------

/// TS-30, CFG-12, AC-006 S2: exactly 256 MiB is accepted; one byte more, and a value that
/// does not fit the column, are refused with `PerAllocation` and leave no row or audit.
#[tokio::test]
#[ignore = "requires Docker for PostgreSQL testcontainer"]
async fn register_enforces_the_256_mib_per_allocation_budget() {
    let fx = afx().await;
    let ok = fx
        .store
        .register(None, ALLOC_MAX, 1, expires())
        .await
        .expect("exactly 256 MiB");
    assert_eq!(
        raw_alloc(&fx.conn, ok.id).await.map(|r| r.bytes_reserved),
        Some(ALLOC_MAX as i64)
    );
    for bytes in [ALLOC_MAX + 1, 2 * ALLOC_MAX, u64::MAX] {
        let r = fx.store.register(None, bytes, 1, expires()).await;
        assert_eq!(
            r.err(),
            Some(AllocationError::BudgetExceeded(BudgetKind::PerAllocation)),
            "{bytes} bytes"
        );
    }
    assert_eq!(alloc_rows(&fx.conn).await, 1);
    assert_eq!(
        audit_rows_total(&fx.conn).await,
        1,
        "a refused budget writes no registered audit"
    );
}

/// TS-30, CFG-13, AC-006 S2: four 256 MiB allocations fill the 1 GiB total exactly; the
/// next byte is refused with `Total`. A second store instance over the same database
/// sees the same total (durable, not in-memory).
#[tokio::test]
#[ignore = "requires Docker for PostgreSQL testcontainer"]
async fn register_enforces_the_1_gib_total_budget_durably() {
    let fx = afx().await;
    for _ in 0..4 {
        fx.store
            .register(None, ALLOC_MAX, 1, expires())
            .await
            .expect("within total");
    }
    let restarted = astore(&fx.conn, Arc::new(DbAuditSink::new(fx.conn.clone())));
    let r = restarted.register(None, 1, 1, expires()).await;
    assert_eq!(
        r.err(),
        Some(AllocationError::BudgetExceeded(BudgetKind::Total))
    );
    assert_eq!(alloc_rows(&fx.conn).await, 4);
    assert_eq!(live_bytes(&fx.conn).await, TOTAL_MAX as i64);
    assert_eq!(
        restarted.totals().await.unwrap(),
        AllocationTotals {
            live_bytes: TOTAL_MAX,
            live_count: 4
        }
    );
}

/// FN-21, FD 4.4: every state except `deleted` counts toward the total (orphans and
/// `cleanup_failed` included). A `deleted` row frees its bytes.
#[tokio::test]
#[ignore = "requires Docker for PostgreSQL testcontainer"]
async fn total_budget_counts_every_state_except_deleted() {
    let fx = afx().await;
    seed_alloc(&fx.conn, SeedAlloc::new(Deleted, ALLOC_MAX)).await;
    for state in [OrphanDiscovered, CleanupFailed] {
        seed_alloc(&fx.conn, SeedAlloc::new(state, ALLOC_MAX)).await;
    }
    for state in [ReleasePending, Written, Deleting, Registered] {
        seed_alloc(&fx.conn, SeedAlloc::new(state, 100 * MIB)).await;
    }
    let live = 2 * ALLOC_MAX + 4 * 100 * MIB;
    assert_eq!(
        fx.store.totals().await.unwrap(),
        AllocationTotals {
            live_bytes: live,
            live_count: 6
        }
    );
    let headroom = TOTAL_MAX - live;
    fx.store
        .register(None, headroom, 1, expires())
        .await
        .expect("fills the headroom exactly");
    let r = fx.store.register(None, 1, 1, expires()).await;
    assert_eq!(
        r.err(),
        Some(AllocationError::BudgetExceeded(BudgetKind::Total))
    );
    assert_eq!(live_bytes(&fx.conn).await, TOTAL_MAX as i64);
}

/// TS-30, CFG-13, T-13: concurrent registrations never push the total over 1 GiB. With
/// 768 MiB live and eight simultaneous 256 MiB requests, exactly one wins.
#[tokio::test(flavor = "multi_thread", worker_threads = 4)]
#[ignore = "requires Docker for PostgreSQL testcontainer"]
async fn concurrent_registrations_never_exceed_the_total_budget() {
    let fx = afx().await;
    for _ in 0..3 {
        seed_alloc(&fx.conn, SeedAlloc::new(Written, ALLOC_MAX)).await;
    }
    let racers = 8;
    let barrier = Arc::new(Barrier::new(racers));
    let mut handles = Vec::new();
    for _ in 0..racers {
        let store = fx.store.clone();
        let barrier = barrier.clone();
        handles.push(tokio::spawn(async move {
            barrier.wait().await;
            store.register(None, ALLOC_MAX, 1, expires()).await
        }));
    }
    let mut won = 0;
    for h in handles {
        match h.await.unwrap() {
            Ok(_) => won += 1,
            Err(e) => assert_eq!(e, AllocationError::BudgetExceeded(BudgetKind::Total)),
        }
    }
    assert_eq!(won, 1, "exactly one racer fits in the remaining 256 MiB");
    assert_eq!(live_bytes(&fx.conn).await, TOTAL_MAX as i64);
    assert_eq!(alloc_rows(&fx.conn).await, 4);
    assert_eq!(
        audit_rows_total(&fx.conn).await,
        1,
        "only the winner is audited"
    );
}

// ---------- read, compare-and-set moves (FN-23) ----------

/// FN-21: `get` returns the committed row, or `None` for an unknown id.
#[tokio::test]
#[ignore = "requires Docker for PostgreSQL testcontainer"]
async fn get_returns_the_committed_row() {
    let fx = afx().await;
    let a = fx
        .store
        .register(None, 7, 2, expires())
        .await
        .expect("register");
    assert_eq!(fx.store.get(a.id).await.unwrap(), Some(a));
    assert_eq!(fx.store.get(Uuid::new_v4()).await.unwrap(), None);
}

/// FN-23: `mark_written` is a compare-and-set on `state_version`; it moves only
/// `registered -> written` and bumps the version once.
#[tokio::test]
#[ignore = "requires Docker for PostgreSQL testcontainer"]
async fn mark_written_is_a_compare_and_set() {
    let fx = afx().await;
    let a = fx
        .store
        .register(None, 8, 1, expires())
        .await
        .expect("register");
    fx.store
        .mark_written(a.id, 0)
        .await
        .expect("registered -> written");
    let raw = require_alloc(&fx.conn, a.id).await;
    assert_eq!((raw.state.as_str(), raw.state_version), ("written", 1));
    assert_eq!(
        fx.store.mark_written(a.id, 0).await,
        Err(AllocationError::Lost),
        "stale version"
    );
    assert_eq!(
        fx.store.mark_written(a.id, 1).await,
        Err(AllocationError::Lost),
        "written -> written"
    );
    let again = require_alloc(&fx.conn, a.id).await;
    assert_eq!((again.state.as_str(), again.state_version), ("written", 1));
    assert_eq!(
        fx.store.mark_written(Uuid::new_v4(), 0).await,
        Err(AllocationError::Lost),
        "unknown id"
    );
}

/// FN-23: `mark_written` compares the version, not only the state. A caller holding a
/// stale version of a row that is still `registered` (for example after another writer
/// released and re-registered nothing, but bumped the version) must lose. Test-after,
/// added in build-and-verify WU-04: the version check had no test of its own.
#[tokio::test]
#[ignore = "requires Docker for PostgreSQL testcontainer"]
async fn mark_written_with_a_wrong_version_on_a_registered_row_loses() {
    let fx = afx().await;
    let a = fx
        .store
        .register(None, 8, 1, expires())
        .await
        .expect("register");
    for wrong in [1_i64, 2, 99] {
        assert_eq!(
            fx.store.mark_written(a.id, wrong).await,
            Err(AllocationError::Lost),
            "version {wrong} must not match a registered row at version 0"
        );
    }
    let raw = require_alloc(&fx.conn, a.id).await;
    assert_eq!((raw.state.as_str(), raw.state_version), ("registered", 0));
}

/// FN-23, IFC 6.1 step 4: `release` moves `registered` or `written` to `release_pending`
/// by compare-and-set; nothing else can be released.
#[tokio::test]
#[ignore = "requires Docker for PostgreSQL testcontainer"]
async fn release_moves_registered_and_written_to_release_pending() {
    let fx = afx().await;
    let a = fx.store.register(None, 8, 1, expires()).await.unwrap();
    fx.store
        .release(a.id, 0)
        .await
        .expect("registered -> release_pending");
    let raw = require_alloc(&fx.conn, a.id).await;
    assert_eq!(
        (raw.state.as_str(), raw.state_version),
        ("release_pending", 1)
    );
    assert_eq!(
        fx.store.release(a.id, 0).await,
        Err(AllocationError::Lost),
        "stale version"
    );

    let b = fx.store.register(None, 8, 1, expires()).await.unwrap();
    fx.store.mark_written(b.id, 0).await.unwrap();
    fx.store
        .release(b.id, 1)
        .await
        .expect("written -> release_pending");
    assert_eq!(require_alloc(&fx.conn, b.id).await.state, "release_pending");

    for state in [Deleting, Deleted, OrphanDiscovered, CleanupFailed] {
        let id = seed_alloc(&fx.conn, SeedAlloc::new(state, 8).version(4)).await;
        assert_eq!(
            fx.store.release(id, 4).await,
            Err(AllocationError::Lost),
            "{state:?}"
        );
        let raw = require_alloc(&fx.conn, id).await;
        assert_eq!((raw.state.as_str(), raw.state_version), (state.as_str(), 4));
    }
}

/// FN-23, T-12: a `mark_written` and a `release` racing on the same version have exactly
/// one winner; the loser reports `Lost` and the version moves once.
#[tokio::test(flavor = "multi_thread", worker_threads = 4)]
#[ignore = "requires Docker for PostgreSQL testcontainer"]
async fn racing_mark_written_and_release_have_one_winner() {
    let fx = afx().await;
    let a = fx.store.register(None, 8, 1, expires()).await.unwrap();
    let barrier = Arc::new(Barrier::new(2));
    let (s1, b1) = (fx.store.clone(), barrier.clone());
    let (s2, b2) = (fx.store.clone(), barrier.clone());
    let id = a.id;
    let w = tokio::spawn(async move {
        b1.wait().await;
        s1.mark_written(id, 0).await
    });
    let r = tokio::spawn(async move {
        b2.wait().await;
        s2.release(id, 0).await
    });
    let results = [w.await.unwrap(), r.await.unwrap()];
    let ok = results.iter().filter(|r| r.is_ok()).count();
    let lost = results
        .iter()
        .filter(|r| **r == Err(AllocationError::Lost))
        .count();
    assert_eq!((ok, lost), (1, 1), "{results:?}");
    assert_eq!(require_alloc(&fx.conn, id).await.state_version, 1);
}

// ---------- claim and delete bookkeeping (FD 4.4, IFC 6.1 steps 5-7) ----------

/// IFC 6.1 step 5, IFC 6.3: the claim `release_pending -> deleting` writes
/// `allocation.delete_claimed` in the same transaction and refuses a stale version, a
/// wrong state, and an orphan still inside its grace.
#[tokio::test]
#[ignore = "requires Docker for PostgreSQL testcontainer"]
async fn claim_for_delete_audits_in_the_same_transaction_and_is_conditional() {
    let fx = afx().await;
    let probe = Arc::new(AllocProbeSink::new(&fx.conn));
    let store = astore(&fx.conn, probe.clone());
    let id = seed_alloc(&fx.conn, SeedAlloc::new(ReleasePending, 5).version(3)).await;

    let claimed = store.claim_for_delete(id, 3).await.unwrap();
    assert!(
        claimed.is_some(),
        "a due release_pending row must be claimed"
    );
    let claimed = claimed.unwrap();
    assert_eq!(
        (claimed.id, claimed.state, claimed.state_version),
        (id, Deleting, 4)
    );
    let raw = require_alloc(&fx.conn, id).await;
    assert_eq!((raw.state.as_str(), raw.state_version), ("deleting", 4));
    let seen = probe.seen.lock().unwrap().clone();
    assert_eq!(seen.len(), 1, "{seen:?}");
    assert!(seen[0].saw_tx);
    assert_eq!(seen[0].event, AuditEventKind::AllocationDeleteClaimed);
    assert_eq!(
        seen[0].outside_state.as_deref(),
        Some("release_pending"),
        "the claim must still be uncommitted while its audit row is written"
    );
    assert_eq!(
        audit_events_for_allocation(&fx.conn, id).await,
        vec![("allocation.delete_claimed".to_owned(), "success".to_owned())]
    );

    assert!(
        store.claim_for_delete(id, 3).await.unwrap().is_none(),
        "stale version"
    );
    assert!(
        store.claim_for_delete(id, 4).await.unwrap().is_none(),
        "already deleting"
    );
    let written = seed_alloc(&fx.conn, SeedAlloc::new(Written, 5)).await;
    assert!(
        store.claim_for_delete(written, 0).await.unwrap().is_none(),
        "written is not claimable"
    );
    let early = seed_alloc(
        &fx.conn,
        SeedAlloc::new(OrphanDiscovered, 5).grace_until(t0() + Duration::seconds(1)),
    )
    .await;
    assert!(
        store.claim_for_delete(early, 0).await.unwrap().is_none(),
        "orphan inside grace"
    );
    assert_eq!(
        audit_rows_total(&fx.conn).await,
        1,
        "refused claims write no audit"
    );
    let late = seed_alloc(
        &fx.conn,
        SeedAlloc::new(OrphanDiscovered, 5).grace_until(t0() - Duration::seconds(1)),
    )
    .await;
    assert!(
        store.claim_for_delete(late, 0).await.unwrap().is_some(),
        "orphan after grace"
    );
}

/// IFC 6.3: audit failure refuses the claim and the commit; the row stays put and no
/// unlink may follow.
#[tokio::test]
#[ignore = "requires Docker for PostgreSQL testcontainer"]
async fn claim_and_delete_commit_are_refused_when_audit_fails() {
    let fx = afx().await;
    let store = astore(&fx.conn, Arc::new(FailingAuditSink));
    let pending = seed_alloc(&fx.conn, SeedAlloc::new(ReleasePending, 5).version(1)).await;
    assert_eq!(
        store.claim_for_delete(pending, 1).await.err(),
        Some(AllocationError::AuditUnavailable)
    );
    assert_eq!(
        require_alloc(&fx.conn, pending).await.state,
        "release_pending"
    );

    let deleting = seed_alloc(&fx.conn, SeedAlloc::new(Deleting, 5).version(1)).await;
    assert_eq!(
        store.mark_deleted(deleting, 1).await.err(),
        Some(AllocationError::AuditUnavailable)
    );
    let raw = require_alloc(&fx.conn, deleting).await;
    assert_eq!((raw.state.as_str(), raw.state_version), ("deleting", 1));
    assert!(raw.deleted_at.is_none());
}

/// IFC 6.1 step 6, 6.1a: `mark_deleted` commits `deleting -> deleted` with
/// `allocation.deleted` in the same transaction; there is no shortcut from any other
/// state.
#[tokio::test]
#[ignore = "requires Docker for PostgreSQL testcontainer"]
async fn mark_deleted_commits_with_audit_and_has_no_shortcut() {
    let fx = afx().await;
    let probe = Arc::new(AllocProbeSink::new(&fx.conn));
    let store = astore(&fx.conn, probe.clone());
    let id = seed_alloc(&fx.conn, SeedAlloc::new(Deleting, 5).version(2)).await;
    store
        .mark_deleted(id, 2)
        .await
        .expect("deleting -> deleted");
    let raw = require_alloc(&fx.conn, id).await;
    assert_eq!((raw.state.as_str(), raw.state_version), ("deleted", 3));
    assert_eq!(raw.deleted_at, Some(t0()));
    let seen = probe.seen.lock().unwrap().clone();
    assert_eq!(seen.len(), 1);
    assert!(seen[0].saw_tx);
    assert_eq!(seen[0].event, AuditEventKind::AllocationDeleted);
    assert_eq!(seen[0].outside_state.as_deref(), Some("deleting"));
    assert_eq!(
        audit_events_for_allocation(&fx.conn, id).await,
        vec![("allocation.deleted".to_owned(), "success".to_owned())]
    );
    assert_eq!(
        store.mark_deleted(id, 2).await,
        Err(AllocationError::Lost),
        "stale version"
    );

    for state in [
        Registered,
        Written,
        ReleasePending,
        OrphanDiscovered,
        CleanupFailed,
    ] {
        let other = seed_alloc(&fx.conn, SeedAlloc::new(state, 5)).await;
        assert_eq!(
            store.mark_deleted(other, 0).await,
            Err(AllocationError::Lost),
            "{state:?}"
        );
        assert_eq!(require_alloc(&fx.conn, other).await.state, state.as_str());
    }
}

/// IFC 6.1 step 7, CFG-08, CFG-09, TS-30 (cleanup budget): a transient failure returns the
/// row to `release_pending` with `cleanup_attempts + 1` and `next_attempt_at = now + 60 s`;
/// the third failure ends in `cleanup_failed` with `cleanup.exhausted` audit.
#[tokio::test]
#[ignore = "requires Docker for PostgreSQL testcontainer"]
async fn cleanup_failures_retry_with_backoff_then_exhaust_after_three() {
    let fx = afx().await;
    let now = t0();
    for attempts in [0, 1] {
        let id = seed_alloc(
            &fx.conn,
            SeedAlloc::new(Deleting, 5).version(1).attempts(attempts),
        )
        .await;
        let out = fx
            .store
            .record_cleanup_failure(id, 1, ReasonCode::PersistenceError, now)
            .await
            .unwrap();
        assert_eq!(
            out,
            CleanupFailureOutcome::Retry,
            "attempt {}",
            attempts + 1
        );
        let raw = require_alloc(&fx.conn, id).await;
        assert_eq!(raw.state, "release_pending");
        assert_eq!((raw.cleanup_attempts, raw.state_version), (attempts + 1, 2));
        assert_eq!(raw.next_attempt_at, Some(now + Duration::seconds(60)));
        assert_eq!(raw.last_reason_code.as_deref(), Some("PERSISTENCE_ERROR"));
        assert!(audit_events_for_allocation(&fx.conn, id).await.is_empty());
    }
    let last = seed_alloc(&fx.conn, SeedAlloc::new(Deleting, 5).version(1).attempts(2)).await;
    let out = fx
        .store
        .record_cleanup_failure(last, 1, ReasonCode::PersistenceError, now)
        .await
        .unwrap();
    assert_eq!(out, CleanupFailureOutcome::Exhausted);
    let raw = require_alloc(&fx.conn, last).await;
    assert_eq!(
        (raw.state.as_str(), raw.cleanup_attempts),
        ("cleanup_failed", 3)
    );
    assert_eq!(
        audit_events_for_allocation(&fx.conn, last).await,
        vec![("cleanup.exhausted".to_owned(), "success".to_owned())]
    );
    assert_eq!(
        fx.store
            .record_cleanup_failure(last, 1, ReasonCode::PersistenceError, now)
            .await,
        Err(AllocationError::Lost),
        "stale version"
    );
}

/// FD 4.4: an orphan row has no job, the id and key of the file, `orphan_discovered`, the
/// current epoch, size capped at CFG-12, and grace and expiry at now + 1 h. It counts toward
/// the total but is never refused for budget (the file exists). A key that does not match
/// the id is refused.
#[tokio::test]
#[ignore = "requires Docker for PostgreSQL testcontainer"]
async fn insert_orphan_persists_the_discovered_file_with_grace() {
    let fx = afx().await;
    exec(
        &fx.conn,
        "INSERT INTO backup_epoch (id, epoch, started_at) VALUES (1, 7, $1)",
        vec![t0().into()],
    )
    .await;
    let id = Uuid::new_v4();
    fx.store
        .insert_orphan(id, &StorageKey::from_allocation_id(id), 5000, t0())
        .await
        .expect("insert orphan");
    let raw = require_alloc(&fx.conn, id).await;
    assert_eq!(raw.state, "orphan_discovered");
    assert_eq!(raw.storage_key, format!("alloc/{id}.bin"));
    assert_eq!((raw.job_id, raw.epoch, raw.bytes_reserved), (None, 7, 5000));
    assert_eq!(raw.orphan_grace_until, Some(t0() + Duration::hours(1)));
    assert_eq!(raw.expires_at, t0() + Duration::hours(1));

    let big = Uuid::new_v4();
    fx.store
        .insert_orphan(big, &StorageKey::from_allocation_id(big), 300 * MIB, t0())
        .await
        .expect("oversized orphan");
    assert_eq!(
        require_alloc(&fx.conn, big).await.bytes_reserved,
        ALLOC_MAX as i64
    );

    for _ in 0..3 {
        seed_alloc(&fx.conn, SeedAlloc::new(Written, ALLOC_MAX)).await;
    }
    let over = Uuid::new_v4();
    fx.store
        .insert_orphan(over, &StorageKey::from_allocation_id(over), ALLOC_MAX, t0())
        .await
        .expect("an orphan is recorded even over the total");
    assert!(raw_alloc(&fx.conn, over).await.is_some());

    let other = Uuid::new_v4();
    let r = fx
        .store
        .insert_orphan(
            other,
            &StorageKey::from_allocation_id(Uuid::new_v4()),
            1,
            t0(),
        )
        .await;
    assert!(r.is_err(), "key must be derived from the same id");
    assert!(raw_alloc(&fx.conn, other).await.is_none());
}

/// IFC 6.1 steps 5-7: `due_for_cleanup` returns exactly the `release_pending` rows whose
/// `next_attempt_at` is null or reached, and the `orphan_discovered` rows past their grace.
#[tokio::test]
#[ignore = "requires Docker for PostgreSQL testcontainer"]
async fn due_for_cleanup_returns_exactly_the_due_rows() {
    let fx = afx().await;
    let now = t0();
    let due_null = seed_alloc(&fx.conn, SeedAlloc::new(ReleasePending, 1)).await;
    let due_past = seed_alloc(
        &fx.conn,
        SeedAlloc::new(ReleasePending, 1).next_attempt_at(now - Duration::seconds(1)),
    )
    .await;
    let due_now = seed_alloc(
        &fx.conn,
        SeedAlloc::new(ReleasePending, 1).next_attempt_at(now),
    )
    .await;
    let due_orphan = seed_alloc(
        &fx.conn,
        SeedAlloc::new(OrphanDiscovered, 1).grace_until(now - Duration::seconds(1)),
    )
    .await;
    let _backoff = seed_alloc(
        &fx.conn,
        SeedAlloc::new(ReleasePending, 1).next_attempt_at(now + Duration::seconds(1)),
    )
    .await;
    let _early_orphan = seed_alloc(
        &fx.conn,
        SeedAlloc::new(OrphanDiscovered, 1).grace_until(now + Duration::seconds(1)),
    )
    .await;
    for state in [Registered, Written, Deleting, Deleted, CleanupFailed] {
        seed_alloc(&fx.conn, SeedAlloc::new(state, 1)).await;
    }
    let got: HashSet<Uuid> = fx
        .store
        .due_for_cleanup(now)
        .await
        .unwrap()
        .into_iter()
        .map(|a| a.id)
        .collect();
    let want: HashSet<Uuid> = [due_null, due_past, due_now, due_orphan]
        .into_iter()
        .collect();
    assert_eq!(got, want);
}

// ---------- register before write, end to end (FN-21, FN-22) ----------

/// FN-21, FN-22, T-11, T-12: with the real ledger, a write for an unregistered key is
/// refused; after `register` the row exists before the first byte is written, the write
/// lands under exactly `alloc/<id>.bin`, and the file store itself never advances the row.
#[tokio::test]
#[ignore = "requires Docker for PostgreSQL testcontainer"]
async fn the_ledger_row_exists_before_the_first_byte_is_written() {
    let fx = afx().await;
    let tmp = TempDir::new("register-before-write");
    let storage = FsFixtureStorage::open(&tmp.root(), fx.store.clone()).expect("open storage");
    let alloc_dir = tmp.root().join("alloc");
    std::fs::create_dir_all(&alloc_dir).unwrap();

    let stray = StorageKey::from_allocation_id(Uuid::new_v4());
    assert_eq!(
        storage.write(&stray, b"x").await,
        Err(StorageError::NotRegistered)
    );
    assert!(names_in(&alloc_dir).is_empty());

    let a = fx
        .store
        .register(None, 6, 1, expires())
        .await
        .expect("register");
    assert!(
        raw_alloc(&fx.conn, a.id).await.is_some(),
        "row committed before any write"
    );
    let file = alloc_dir.join(format!("{}.bin", a.id));
    assert!(!file.exists(), "register alone writes no file");
    storage
        .write(&a.storage_key, b"abcdef")
        .await
        .expect("write after register");
    assert_eq!(std::fs::read(&file).unwrap(), b"abcdef");
    assert_eq!(names_in(&alloc_dir), vec![format!("{}.bin", a.id)]);
    assert_eq!(require_alloc(&fx.conn, a.id).await.state, "registered");
}

/// FN-21: rows are stamped from the injected clock, not the wall clock. The clock is set
/// years away from now so a wall-clock stamp cannot match; both the returned allocation
/// and the stored row carry it.
#[tokio::test]
#[ignore = "requires Docker for PostgreSQL testcontainer"]
async fn rows_are_stamped_from_the_injected_clock() {
    let fx = afx().await;
    let far = chrono::TimeZone::with_ymd_and_hms(&chrono::Utc, 2031, 1, 1, 12, 0, 0).unwrap();
    let store = astore_at(&fx, far);
    let a = store
        .register(None, 1, 1, far + Duration::hours(24))
        .await
        .expect("register");
    assert_eq!((a.created_at, a.updated_at), (far, far));
    let raw = require_alloc(&fx.conn, a.id).await;
    assert_eq!(raw.created_at, far);
    let updated: chrono::DateTime<chrono::Utc> = fx
        .conn
        .query_one(pg(
            "SELECT updated_at FROM backup_allocations WHERE id = $1",
            vec![a.id.into()],
        ))
        .await
        .unwrap()
        .expect("row")
        .try_get("", "updated_at")
        .unwrap();
    assert_eq!(updated, far);
}

fn astore_at(fx: &AFx, now: chrono::DateTime<chrono::Utc>) -> DbAllocationStore {
    DbAllocationStore::new(
        fx.conn.clone(),
        Arc::new(DbAuditSink::new(fx.conn.clone())),
        Arc::new(FixedClock(now)),
        INSTANCE,
        AllocationLimits::default(),
    )
}

// ---------- epoch, due-ness, idempotence, edge inputs (review findings) ----------

async fn seed_epoch(fx: &AFx, epoch: i64) {
    exec(
        &fx.conn,
        "INSERT INTO backup_epoch (id, epoch, started_at) VALUES (1, $1, $2)",
        vec![epoch.into(), t0().into()],
    )
    .await;
}

/// IFC 6.1a [D]: the claim stamps the CURRENT epoch (read from `backup_epoch`), so a row
/// registered in an earlier epoch and claimed by the live cleaner is never mistaken for a
/// crashed claim by a reconcile on DB recovery. Row and returned allocation both carry it.
#[tokio::test]
#[ignore = "requires Docker for PostgreSQL testcontainer"]
async fn claim_for_delete_stamps_the_current_epoch() {
    let fx = afx().await;
    seed_epoch(&fx, 7).await; // seeded rows carry epoch 1: an earlier epoch
    for state in [ReleasePending, OrphanDiscovered] {
        let seed = SeedAlloc::new(state, 5).version(2);
        let seed = if state == OrphanDiscovered {
            seed.grace_until(t0() - Duration::seconds(1))
        } else {
            seed
        };
        let id = seed_alloc(&fx.conn, seed).await;
        assert_eq!(require_alloc(&fx.conn, id).await.epoch, 1);
        let claimed = fx.store.claim_for_delete(id, 2).await.unwrap();
        assert!(claimed.is_some(), "{state:?} must be claimed");
        assert_eq!(claimed.unwrap().epoch, 7, "{state:?}: returned allocation");
        assert_eq!(require_alloc(&fx.conn, id).await.epoch, 7, "{state:?}: row");
    }
}

/// IFC 6.1a: a `release_pending` row is claimed only when due (`next_attempt_at <= now`).
/// A row in backoff is not claimed and writes no audit; once its time has come the same
/// call claims it.
#[tokio::test]
#[ignore = "requires Docker for PostgreSQL testcontainer"]
async fn claim_for_delete_waits_for_the_backoff_to_pass() {
    let fx = afx().await;
    let id = seed_alloc(
        &fx.conn,
        SeedAlloc::new(ReleasePending, 5)
            .version(1)
            .attempts(1)
            .next_attempt_at(t0() + Duration::seconds(30)),
    )
    .await;
    assert!(
        fx.store.claim_for_delete(id, 1).await.unwrap().is_none(),
        "a row in backoff is not claimable"
    );
    let raw = require_alloc(&fx.conn, id).await;
    assert_eq!(
        (raw.state.as_str(), raw.state_version),
        ("release_pending", 1)
    );
    assert_eq!(
        audit_rows_total(&fx.conn).await,
        0,
        "no audit for a refusal"
    );

    exec(
        &fx.conn,
        "UPDATE backup_allocations SET next_attempt_at = $2 WHERE id = $1",
        vec![id.into(), (t0() - Duration::seconds(1)).into()],
    )
    .await;
    assert!(
        fx.store.claim_for_delete(id, 1).await.unwrap().is_some(),
        "claimable once next_attempt_at has passed"
    );
    assert_eq!(require_alloc(&fx.conn, id).await.state, "deleting");
}

/// IFC 6.1 step 7: a cleanup failure is recorded only against a `deleting` row. On any
/// other state, even with a matching version, it reports `Lost` and changes nothing.
#[tokio::test]
#[ignore = "requires Docker for PostgreSQL testcontainer"]
async fn record_cleanup_failure_only_applies_to_a_deleting_row() {
    let fx = afx().await;
    for state in [
        Registered,
        Written,
        ReleasePending,
        Deleted,
        OrphanDiscovered,
        CleanupFailed,
    ] {
        let id = seed_alloc(&fx.conn, SeedAlloc::new(state, 5).version(3)).await;
        let before = require_alloc(&fx.conn, id).await;
        assert_eq!(
            fx.store
                .record_cleanup_failure(id, 3, ReasonCode::PersistenceError, t0())
                .await,
            Err(AllocationError::Lost),
            "{state:?}"
        );
        assert_eq!(require_alloc(&fx.conn, id).await, before, "{state:?}");
    }
    assert_eq!(audit_rows_total(&fx.conn).await, 0);
}

/// [D] FD 4.4 / IFC 6.1a: re-discovering an orphan the ledger already holds (every 900 s
/// sweep sees the same file) is an idempotent no-op: no error, one row, the FIRST grace
/// and expiry kept (a reset grace would make the orphan never due), and no second audit
/// row.
#[tokio::test]
#[ignore = "requires Docker for PostgreSQL testcontainer"]
async fn insert_orphan_twice_keeps_the_first_grace() {
    let fx = afx().await;
    let id = Uuid::new_v4();
    let key = StorageKey::from_allocation_id(id);
    fx.store
        .insert_orphan(id, &key, 5000, t0())
        .await
        .expect("first discovery");
    let later = astore_at(&fx, t0() + Duration::minutes(15));
    later
        .insert_orphan(id, &key, 5000, t0() + Duration::minutes(15))
        .await
        .expect("second discovery is an idempotent no-op");
    assert_eq!(alloc_rows(&fx.conn).await, 1);
    let raw = require_alloc(&fx.conn, id).await;
    assert_eq!(raw.state, "orphan_discovered");
    assert_eq!(raw.orphan_grace_until, Some(t0() + Duration::hours(1)));
    assert_eq!(raw.expires_at, t0() + Duration::hours(1));
    assert_eq!(
        audit_events_for_allocation(&fx.conn, id).await,
        vec![("orphan.discovered".to_owned(), "success".to_owned())],
        "one discovery, one audit row"
    );
}

/// [D] IFC 7, IFC 6.3: `insert_orphan` writes `orphan.discovered` in its own transaction
/// (the row is still uncommitted while the audit row is written), and a failing audit
/// refuses the insert and leaves no row.
#[tokio::test]
#[ignore = "requires Docker for PostgreSQL testcontainer"]
async fn insert_orphan_audits_in_the_same_transaction_and_fails_closed() {
    let fx = afx().await;
    let probe = Arc::new(AllocProbeSink::new(&fx.conn));
    let store = astore(&fx.conn, probe.clone());
    let id = Uuid::new_v4();
    store
        .insert_orphan(id, &StorageKey::from_allocation_id(id), 10, t0())
        .await
        .expect("insert orphan");
    let seen = probe.seen.lock().unwrap().clone();
    assert_eq!(seen.len(), 1, "{seen:?}");
    assert!(seen[0].saw_tx);
    assert_eq!(seen[0].event, AuditEventKind::OrphanDiscovered);
    assert_eq!(
        seen[0].outside_state, None,
        "row uncommitted during the audit write"
    );
    assert_eq!(
        audit_events_for_allocation(&fx.conn, id).await,
        vec![("orphan.discovered".to_owned(), "success".to_owned())]
    );

    let refused = Uuid::new_v4();
    let r = astore(&fx.conn, Arc::new(FailingAuditSink))
        .insert_orphan(refused, &StorageKey::from_allocation_id(refused), 10, t0())
        .await;
    assert_eq!(r.err(), Some(AllocationError::AuditUnavailable));
    assert!(raw_alloc(&fx.conn, refused).await.is_none());
}

/// [D] FN-21: a 0-byte reservation is within the bounds (`<=`), is registered and audited
/// like any other.
#[tokio::test]
#[ignore = "requires Docker for PostgreSQL testcontainer"]
async fn register_accepts_a_zero_byte_reservation() {
    let fx = afx().await;
    let a = fx
        .store
        .register(None, 0, 1, expires())
        .await
        .expect("0 bytes are within the bounds");
    assert_eq!(require_alloc(&fx.conn, a.id).await.bytes_reserved, 0);
    assert_eq!(
        audit_events_for_allocation(&fx.conn, a.id).await,
        vec![("allocation.registered".to_owned(), "success".to_owned())]
    );
}

/// IFC 6.1 step 1, T-12: when the ledger itself is unreachable, `register` fails closed
/// with `StoreUnavailable` (a second container whose pool is closed stands in for the
/// outage; the audit sink is healthy). The healthy database gets no row.
#[tokio::test]
#[ignore = "requires Docker for PostgreSQL testcontainer"]
async fn register_fails_closed_when_the_ledger_is_unreachable() {
    let fx = afx().await;
    let dead = crate::test_db::backup_test_db().await;
    dead.conn.clone().close().await.expect("close the pool");
    let store = astore(&dead.conn, Arc::new(DbAuditSink::new(fx.conn.clone())));
    let r = store.register(None, 10, 1, expires()).await;
    assert_eq!(r.err(), Some(AllocationError::StoreUnavailable));
    assert_eq!(alloc_rows(&fx.conn).await, 0);
    assert_eq!(audit_rows_total(&fx.conn).await, 0);
}

/// TS-30 (budget part), T-13: a deterministic interleaving. Two racers each need the last
/// 256 MiB; each reaches its audit call (after its budget check and insert, still inside
/// its transaction) only once the other has too. A check-then-insert at READ COMMITTED
/// lets both in and the total ends at 1.25 GiB. A serialised store lets one in; the other
/// is refused with `Total`. Takes `PATIENCE` once against a correct store.
#[tokio::test(flavor = "multi_thread", worker_threads = 4)]
#[ignore = "requires Docker for PostgreSQL testcontainer"]
async fn forced_interleaving_never_exceeds_the_total_budget() {
    let fx = afx().await;
    for _ in 0..3 {
        seed_alloc(&fx.conn, SeedAlloc::new(Written, ALLOC_MAX)).await;
    }
    let store = astore(&fx.conn, Arc::new(RendezvousSink::new(&fx.conn)));
    let (s1, s2) = (store.clone(), store.clone());
    let a = tokio::spawn(async move { s1.register(None, ALLOC_MAX, 1, expires()).await });
    let b = tokio::spawn(async move { s2.register(None, ALLOC_MAX, 1, expires()).await });
    let results = [a.await.unwrap(), b.await.unwrap()];
    let won = results.iter().filter(|r| r.is_ok()).count();
    assert_eq!(won, 1, "exactly one racer fits: {results:?}");
    for r in results.iter().filter_map(|r| r.as_ref().err()) {
        assert_eq!(*r, AllocationError::BudgetExceeded(BudgetKind::Total));
    }
    assert_eq!(live_bytes(&fx.conn).await, TOTAL_MAX as i64);
    assert_eq!(alloc_rows(&fx.conn).await, 4);
    assert_eq!(
        audit_rows_total(&fx.conn).await,
        1,
        "only the winner is audited"
    );
}
