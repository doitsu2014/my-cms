# WU-06 regression report

- **Unit:** WU-06 — Allocation reconciliation and retention store contracts
- **Status:** Assigned WU-06 database tests passed in focused runs while Docker/PostgreSQL was available. Later retries were blocked when Docker Desktop failed to start. Full-workspace verification is not green because WU-16-owned lifecycle and JobStore recovery/deadline tests fail; no WU-06-owned test failed in the recorded full runs.
- **Baseline:** Reused unchanged from [Develop baseline](../../baseline.md), recorded 2026-11-22. Normal Rust suite: 290 passed, 0 failed, 39 ignored. Explicit Docker-ignored suite: 39 passed, 0 failed. Baseline not rerun or rewritten.

## Current full-suite outcomes

| Command | Observed result | Evidence |
|---|---|---|
| `cargo test --manifest-path apps/api/Cargo.toml --workspace --no-fail-fast` | Exit 101; 253 tests passed in other workspace crates; `domain_backup` 116 passed / 13 failed / 129 ignored. All 13 failures are WU-16 lifecycle tests. | `evidence/regression-wu06-workspace-tests.log` |
| `cargo test --manifest-path apps/api/Cargo.toml --workspace --no-fail-fast -- --ignored` | Exit 101; 126 passed / 3 failed in `domain_backup`. All other Docker-ignored tests passed. The three failed tests are WU-16-owned JobStore epoch/fence/deadline behaviors. | `evidence/regression-wu06-workspace-ignored.log` |
| `cargo test --manifest-path apps/api/Cargo.toml -p domain_backup --no-fail-fast` | Exit 101; 116 passed / 13 failed / 129 ignored. All 13 failures are WU-16 lifecycle tests. | `evidence/regression-wu06-domain-backup.log` |
| `cargo fmt --manifest-path apps/api/Cargo.toml -- --check` | Exit 0. | `evidence/green-wu06-cargo-fmt.log` |
| `cargo clippy --manifest-path apps/api/Cargo.toml -p domain_backup --all-targets` | Exit 0. Remaining warnings are in baseline JobStore scaffold and unowned WU-16 lifecycle code. | `evidence/green-wu06-cargo-clippy.log` |
| `dtech-aidlc ids check` | Exit 0, `ok: true`, no unregistered identifiers. | `evidence/green-wu06-ids-check.log` |

## Failures and ownership

### Normal run — WU-16 lifecycle tests

1. `lifecycle::lifecycle_tests::ts20_registered_row_without_file_is_transitioned_to_release_pending`
2. `lifecycle::lifecycle_tests::ts21_file_after_register_before_mark_written_is_retained_as_owned`
3. `lifecycle::lifecycle_tests::ts22_old_epoch_running_job_is_failed_with_restart_reason_and_audit`
4. `lifecycle::lifecycle_tests::ts23_orphan_is_persisted_before_grace_and_ambiguous_entry_is_untouched`
5. `lifecycle::lifecycle_tests::ts25_crashed_old_epoch_deleting_claim_returns_to_cleanup_without_retry`
6. `lifecycle::lifecycle_tests::ts26_unlinked_file_with_deleting_row_commits_deleted_idempotently`
7. `lifecycle::lifecycle_tests::ts27a_three_unlink_failures_are_spaced_exhausted_and_never_attempted_a_fourth_time`
8. `lifecycle::lifecycle_tests::ts29_store_outage_closes_work_and_recovery_reconciles_before_reopening`
9. `lifecycle::lifecycle_tests::ts30_fake_time_enforces_retry_expiry_grace_and_sweep_boundaries`
10. `lifecycle::lifecycle_tests::ts32_startup_lock_epoch_reconcile_cleanup_barrier_precedes_admissions`
11. `lifecycle::lifecycle_tests::ts42_purge_at_retention_boundary_removes_eligible_metadata_but_keeps_audit`
12. `lifecycle::lifecycle_tests::ts56_truncated_inventory_makes_reconciliation_fail_closed`
13. `lifecycle::lifecycle_tests::ts65_audit_refusal_blocks_delete_claim_delete_commit_and_purge`

These exercise WU-16 orchestration/cleaner/sweeper/startup behavior, explicitly out of WU-06. They reside in pre-existing worktree files; WU-06 did not implement lifecycle behavior.

### Docker-ignored run — WU-16 JobStore tests

1. `infra::tests::reconciliation::job_store_reconciliation_inventory_and_epoch_are_durable` — JobStore epoch advancement and job inventory.
2. `infra::tests::reconciliation::ts22_job_fence_cas_is_epoch_status_version_checked_and_audited_atomically` — JobStore fencing.
3. `infra::tests::reconciliation::ts30_deadline_inventory_includes_only_due_jobs_at_epoch` — JobStore deadline enumeration.

These are separate JobStore work, not the WU-06 AllocationStore contract.

## WU-06 focused tests and Docker interruption

Earlier focused PostgreSQL 15 runs passed all assigned store coverage. Exact successful output excerpts are preserved in `evidence/observed-green-wu06-*.log`:

- TS-56 owner-joined inventory: 1 passed.
- TS-44b applied/lost allocation CAS: 2 passed.
- TS-44b allocation audit rejection with raw allocation/audit rollback checks: 1 passed.
- TS-42 purge ordering/exclusions/idempotency/audit retention: 1 passed.
- TS-65 purge audit refusal with exactly-one sink invocation and raw row/audit rollback assertions: 1 passed.

Later focused reruns failed before assertions because testcontainer creation returned `RequestTimeoutError`. `docker info` subsequently reported Docker Desktop unable to start. Those attempts are captured in `evidence/green-wu06-*.log` and are environmental, not assertion failures.

## Baseline pass-set comparison

The WU-05 verification refresh in `build-and-verify/WU-05/regression-report.md` recorded that all 290 baseline-passing normal test identities and all 39 baseline-passing ignored identities remained passing at that checkpoint. In WU-06 full normal verification, none of the 13 WU-16 lifecycle failures is in the baseline pass set. In the ignored run, none of the three WU-16 JobStore failures is among the 39 baseline-passing ignored identities. No baseline-passing test regression was observed; newly failing names belong to additional worktree tests from WU-06/WU-16. Aggregate counts are not compared directly because subsequent work units add tests.
