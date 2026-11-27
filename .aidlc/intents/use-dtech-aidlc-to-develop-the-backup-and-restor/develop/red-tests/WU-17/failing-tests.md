# FT-17 Failing tests (code manifest): WU-17

- **ID:** FT-17 (WU-17)
- **Stage:** Develop / red-tests (builder-verifier), unit WU-17 "Lifecycle reconciler: startup and recovery reconciliation, job fencing, orphan discovery".
- **Status:** red run recorded 2026-11-26 on Windows, PostgreSQL 15 via Docker.
- **Inputs:**
  - [work-units](../../implementation-design/work-units.md) WU-17
  - [ids.json](../../implementation-design/ids.json)
  - [test-strategy](../../implementation-design/test-strategy.md) TS-20, TS-21, TS-22, TS-23, TS-24, TS-25, TS-44 (reconcile half), TS-56 (lifecycle half)
  - [interfaces-and-contracts](../../implementation-design/interfaces-and-contracts.md) sections 6.1a (reconcile table), 6.2 (crash matrix), 9.1
  - [functional-design](../../implementation-design/functional-design.md) FN-25..FN-28 (legacy F-25..F-28)
- **Evidence:** [red-evidence.md](red-evidence.md); raw logs in [../../build-and-verify/evidence/](../../build-and-verify/evidence/): `red-wu17-reconciler-db-ignored.log`, `red-wu17-jobstore-db-ignored.log`, `red-wu17-cargo-test.log`.

## Why the earlier lifecycle tests are not used as red evidence

`lifecycle/lifecycle_tests.rs` held seven reconciler tests (TS-20..TS-25, TS-56) that called `Reconciler::new().reconcile(now)`. That shell took no store, storage, audit or clock, so the tests could only assert report counts of a function with no inputs. They were assertion-red but proved nothing about durable behavior. They are removed from that file and replaced by the PostgreSQL-backed suite below. The WU-18 and WU-19 shells in the same file are untouched.

## Files

Under `apps/api/domain_backup/src/` unless noted.

| Path | Change | Red-stage behaviour |
|---|---|---|
| `lifecycle/reconciler.rs` | rewritten | Final injection surface and signatures: `Reconciler::new(jobs, allocs, storage, audit, ReconcilerConfig)`, `advance_epoch(now)`, `reconcile(epoch, now)`, `discover_orphans(now)`. `ReconcileReport` gains `released_allocations`, `reset_claims`, `ambiguous_entries`. `advance_epoch` returns `Ok(0)`; `reconcile` and `discover_orphans` do nothing. |
| `infra/tests/reconciler.rs` | new | 19 Docker-backed tests (`#[ignore]`). |
| `infra/tests/mod.rs` | modified | `mod reconciler;` |
| `lifecycle/lifecycle_tests.rs` | modified | the seven WU-17 shell tests removed; the other eight tests unchanged |

No production behavior of an earlier unit changed. The three inert `JobStore` defaults (`advance_epoch`, `list_for_reconciliation`, `fence_job`) that `DbJobStore` does not implement yet are exercised by the existing store-level tests `infra::tests::reconciliation::*`, which this unit owns for those cases (see [red-evidence.md](red-evidence.md)).

## Tests (19 new, Docker-backed)

Each test seeds durable rows with raw SQL, puts real files in a real temp-root `FsFixtureStorage`, runs the reconciler and reads rows, the audit table and the directory back. Audit order comes from a recording sink wrapped around the real `DbAuditSink`, because all rows of one run share one timestamp.

| Test | TS | Asserts |
|---|---|---|
| `advance_epoch_is_durable_and_strictly_increasing` | TS-22 | epoch 1 on a fresh database, then 2; the raw `backup_epoch` row matches |
| `ts20_old_epoch_job_and_registered_allocation_without_file_are_fenced_then_released` | TS-20, TS-44 | job `failed INTERRUPTED_RESTART` version 4 to 5; allocation `release_pending` version 2 to 3, attempts 0; audit order `reconcile.fenced` then `allocation.reconciled`; field values of both rows; no file created; no claim or delete event |
| `ts21_registered_allocation_with_file_is_retained_as_owned_and_released_not_orphaned` | TS-21 | owned file is not an orphan, no second allocation row, file still present, only `allocation.reconciled` for the allocation |
| `ts22_restart_advances_epoch_fences_running_and_queued_jobs_and_never_resumes` | TS-22 | epoch 1 then 2; running job `INTERRUPTED_RESTART`, queued job `ABANDONED_QUEUED`; two fence events, one reconcile event; no queued or running job left |
| `ts22_lost_job_fence_cas_changes_nothing_and_emits_no_event` | TS-22, TS-44 | stale job snapshot: job unchanged, zero fence events, zero audit rows |
| `ts22_lost_allocation_cas_changes_nothing_and_emits_no_event` | TS-22, TS-44 | stale allocation snapshot: allocation unchanged, zero events |
| `ts44b_ownerless_registered_allocation_is_released_with_allocation_correlation` | TS-44 | NULL job id and `correlation_id` = allocation UUID, configured instance |
| `ts23_unowned_file_becomes_a_graced_orphan_and_ambiguous_names_stay_untouched` | TS-23 | orphan row with grace `now + 1 h`, one `orphan.discovered`, stray file and directory raise two `ownership.ambiguous`, nothing deleted, also after the grace |
| `ts23_sweep_discovery_persists_a_file_added_after_start_and_never_deletes_or_fences` | TS-23 | `discover_orphans` persists the late file, keeps it, does not fence a running job, finds nothing the second time |
| `ts23_truncated_sweep_discovery_persists_nothing` | TS-23, TS-56 | cap 1 and two files: `InventoryTruncated`, no row, no audit |
| `ts24_unexpired_incomplete_allocation_with_missing_file_audits_each_full_scan_unchanged` | TS-24, TS-44 | two scans append two `reconcile.missing_file` rows with every field of IFC section 7; allocation row identical (state, version, `updated_at`); count 1 per scan |
| `ts24_failed_owner_or_expired_written_allocation_becomes_release_eligible` | TS-24 | failed owner and 24 h expiry both give `release_pending` with `allocation.reconciled`; no missing-file event |
| `ts24_unexpired_incomplete_allocation_with_its_file_present_is_retained_silently` | TS-24 | unchanged, no audit |
| `ts24_missing_file_audit_failure_fails_reconciliation_and_changes_no_state` | TS-24 | `AuditUnavailable`; allocation unchanged; no audit row |
| `ts25_stale_deleting_claim_returns_to_release_pending_without_consuming_a_retry` | TS-25 | old-epoch `deleting` to `release_pending`, attempts unchanged, version +1, file kept, no claim event |
| `ts25_current_epoch_deleting_claim_belongs_to_the_live_cleaner` | TS-25 | a current-epoch claim is untouched |
| `current_epoch_running_job_and_its_registered_allocation_are_left_to_the_worker` | TS-20 | nothing changes, no audit |
| `cleanup_failed_allocation_is_terminal_for_automation_and_never_touched` | TS-25 | unchanged, no audit |
| `ts56_truncated_inventory_fails_closed_before_any_write` | TS-56 | `InventoryTruncated`; job, allocation, row count, audit count and directory listing identical before and after |

## Provenance

- **Observed:** the results recorded in `red-evidence.md`.
- **Identifiers:** registry and Analyze only. `dtech-aidlc ids check` reports none unregistered for this unit.
