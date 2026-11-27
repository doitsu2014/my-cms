# FT-19 Failing tests (code manifest): WU-19

- **ID:** FT-19 (WU-19)
- **Stage:** Develop / red-tests (builder-verifier), unit WU-19 "Lifecycle sweeper and runtime: expiry and deadline sweeps, retention purge, startup barrier and outage recovery".
- **Status:** red run recorded 2026-11-26 on Windows, PostgreSQL 15 via Docker.
- **Inputs:**
  - [work-units](../../implementation-design/work-units.md) WU-19
  - [ids.json](../../implementation-design/ids.json)
  - [test-strategy](../../implementation-design/test-strategy.md) TS-28, TS-29, TS-30 (deadline, queued-age, expiry and schedule boundaries), TS-32, TS-42 (injection), TS-65 (purge path), TS-56 (lifecycle half for start-up and recovery)
  - [interfaces-and-contracts](../../implementation-design/interfaces-and-contracts.md) sections 6.1a, 9.1
  - [functional-design](../../implementation-design/functional-design.md) FN-20, FN-31, FN-32, FN-33 (legacy F-20, F-31, F-32, F-33), sections 6.6, 6.7 and 7.1
- **Evidence:** [red-evidence.md](red-evidence.md); raw logs in [../../build-and-verify/evidence/](../../build-and-verify/evidence/): `red-wu19-sweeper-db-ignored.log`, `red-wu19-runtime-db-ignored.log`, `red-wu19-jobstore-db-ignored.log`, `red-wu19-cargo-test.log`.

## Files

Under `apps/api/domain_backup/src/` unless noted.

| Path | Change | Red-stage behaviour |
|---|---|---|
| `lifecycle/sweeper.rs` | rewritten | Final injection surface and signatures: `Sweeper::new(jobs, allocs, retention, reconciler, SweeperConfig)`, `sweep(epoch, now)`. `SweepReport` gains `queued_expired` and `discovered_orphans`. `sweep` does nothing. |
| `lifecycle/runtime.rs` | rewritten | Final surface: `Runtime::new(reconciler, cleaner, sweeper, lock acquirer, clock, RuntimeConfig)`, `start`, `database_outage`, `database_recovered`, `run_tick`, `state`, `epoch`, `admissions_open`; `TickReport`, `RuntimeConfig`, `LockAcquirer`. Nothing opens admissions. |
| `infra/tests/lifecycle_support.rs` | new | shared rig: real stores, real `FsFixtureStorage`, real `FsNamespaceLock`, recording wrappers with an outage switch |
| `infra/tests/sweeper.rs` | new | 10 Docker-backed tests (`#[ignore]`) |
| `infra/tests/runtime.rs` | new | 9 Docker-backed tests (`#[ignore]`) |
| `infra/tests/mod.rs` | modified | `mod lifecycle_support; mod sweeper; mod runtime;` |
| `lifecycle/lifecycle_tests.rs`, `lifecycle/mod.rs` | removed / modified | the five remaining shell tests (`ts28`, `ts29`, `ts32`, `ts42`, `ts65` purge half) and the module are removed; they called `Sweeper::new()` and `Runtime::new()` without ports |

`DbJobStore::list_deadline_expired` stays an inert default at red; its existing store-level test `infra::tests::reconciliation::ts30_deadline_inventory_includes_only_due_jobs_at_epoch` is the red evidence for it.

No production behavior of an earlier unit changed.

## Tests (19 new, Docker-backed, plus 1 existing)

Every test seeds rows with raw SQL, runs the sweeper or runtime and reads rows, the audit table and the directory back. The rig wraps the real stores in recorders. Each call is logged with whether admissions were open at that moment. One switch makes every wrapped store, storage and audit call fail like an outage.

Sweeper (`infra/tests/sweeper.rs`):

| Test | TS | Asserts |
|---|---|---|
| `ts30_a_running_job_fails_deadline_exceeded_exactly_at_its_deadline_and_not_before` | TS-30 | nothing at 599 s; at 600 s two jobs (one exactly at `now`) are `failed DEADLINE_EXCEEDED` version 4 to 5, one `job.failed` row each |
| `the_sweep_ignores_a_job_of_another_epoch_even_past_its_deadline` | TS-30 | only the current epoch is failed |
| `ts30_a_queued_job_fails_dispatch_failed_at_the_queued_age_bound` | TS-30 | queued at 599 s, `failed DISPATCH_FAILED` with `dispatch.failed` at 600 s |
| `ts28_an_expired_written_allocation_of_a_running_job_is_kept_until_the_owner_ends` | TS-28 | no release, no audit, file kept while the owner runs; after the owner ends, `release_pending` with `allocation.reconciled`, file still kept |
| `ts30_a_written_allocation_is_released_exactly_at_its_24_hour_expiry` | TS-30 | unchanged one second before, released at `expires_at` |
| `ts42_the_sweep_purges_through_the_injected_retention_store_and_keeps_audit` | TS-42 | the retention port is called; one eligible job and its deleted allocation go; `cleanup_failed`-owned, running and not-yet-retained jobs stay; one `retention.purged` row added, none removed; a repeat purges 0 |
| `ts65_a_refused_purge_audit_removes_nothing` | TS-65 | `AuditUnavailable`; job, allocation and audit counts unchanged |
| `the_sweep_discovers_an_unowned_file_without_deleting_it` | TS-23 | orphan row with grace, file kept |
| `a_truncated_scan_fails_the_sweep_after_the_earlier_steps_and_persists_no_orphan` | TS-56 | `InventoryTruncated`; the deadline step already done; no orphan row |
| `one_sweep_runs_deadline_then_expiry_then_purge_then_discovery` | TS-30 | audit order `job.failed`, `allocation.reconciled`, `retention.purged`, `orphan.discovered`; report counts |

Runtime (`infra/tests/runtime.rs`):

| Test | TS | Asserts |
|---|---|---|
| `ts32_startup_runs_lock_epoch_reconcile_cleanup_then_opens_admissions` | TS-32 | order epoch, inventory, job inventory, fence, allocation reconcile, due cleanup, claim, unlink; every logged step had admissions closed; the lock was held when the epoch advanced; then open, `Ready`, epoch 1 durable; audit order fenced, reconciled, claimed, deleted |
| `ts32_a_second_runtime_cannot_acquire_the_lock_and_mutates_nothing` | TS-32 | `LockUnavailable`; job, allocation, epoch row, audit count and directory unchanged |
| `ts56_a_truncated_inventory_keeps_startup_closed_and_not_ready` | TS-56 | `InventoryTruncated`; closed, `NotReady`, nothing written, lock released |
| `a_runtime_that_was_never_started_refuses_ticks_and_recovery` | TS-29 | `NotStarted` for both; no store call |
| `ts29_a_store_outage_closes_admissions_and_stops_all_store_work` | TS-29 | `StoreUnavailable`, admissions closed, `Degraded`; the next tick makes no call; job, allocation and file unchanged |
| `ts29_recovery_reconciles_then_cleans_then_sweeps_then_reopens_without_a_new_epoch` | TS-29 | order inventory, reconcile, cleanup, deadline sweep, purge; closed at every step; no epoch advance; then `Ready` |
| `ts29_a_truncated_recovery_stays_closed_and_degraded_until_it_succeeds` | TS-29, TS-56 | `InventoryTruncated`, closed, `Degraded`, nothing written; succeeds once the listing is complete |
| `ts29_three_refused_audit_sweeps_close_admissions_and_degrade` | TS-29 | open after refusals 1 and 2, closed and `Degraded` after 3, job still `running`; recovery fails it |
| `ts30_the_cleaner_ticks_every_60_seconds_and_the_sweep_every_15_minutes` | TS-30 | both due at the first tick; neither at 59 s; cleaner only at 60 s; no sweep at 899 s although the deadline passed; sweep at 900 s |

## Shell tests removed, and what that loses

The five shell tests could not see any row or file. Their TS ids are covered by the suites above. The old `ts28` passed vacuously against the no-op sweep; its replacement fails against the shell.

## Provenance

- **Observed:** the results recorded in `red-evidence.md`.
- **Identifiers:** registry and Analyze only. `dtech-aidlc ids check` reports none unregistered for this unit.
