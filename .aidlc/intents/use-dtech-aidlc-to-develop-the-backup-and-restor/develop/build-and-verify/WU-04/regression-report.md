# REG-04 Regression report: WU-04

- **ID:** REG-04 (WU-04)
- **Stage:** Develop / build-and-verify, work unit WU-04.
- **Status:** WU-04 focused tests pass; **intent-wide regression is blocked** by failures in later-unit WU-06 and WU-16 code. No WU-04 regression was identified: no source/test changes were made during this refresh, focused WU-04 suites pass, and all exact baseline-passing test names remain passing.
- **Baseline:** `develop/baseline.md`, recorded once before implementation on 2026-11-22 and reused unchanged. Do not re-run or rewrite that baseline.

## Commands and logs

Run from `apps/api` unless noted. Full output is captured under `develop/build-and-verify/evidence/`.

| Command | Exit | Evidence |
|---|---:|---|
| `cargo test -p domain_backup infra::tests::fs_fixture_storage:: --no-fail-fast` | 0 | `refresh-wu04-focused-fs-fixture.log` |
| `cargo test -p domain_backup infra::tests::fs_namespace_lock:: --no-fail-fast` | 0 | `refresh-wu04-focused-fs-lock.log` |
| `cargo test -p domain_backup infra::tests::allocation_store:: --no-fail-fast` | 0 | `refresh-wu04-focused-allocation-normal.log` |
| `cargo test -p domain_backup infra::tests::allocation_store:: --no-fail-fast -- --ignored` | 0 | `refresh-wu04-focused-allocation-ignored.log` |
| `cargo test --no-fail-fast` | 101 | `refresh-wu04-workspace-normal.log` |
| `cargo test --no-fail-fast -- --ignored` | 101 | `refresh-wu04-workspace-ignored.log` |
| `cargo fmt -- --check` | 0 | `refresh-wu04-fmt.log` |
| `cargo clippy --all-targets` | 0 | `refresh-wu04-clippy.log` |
| `dtech-aidlc ids check` | 0 | `refresh-wu04-ids-check.log` |

Docker prerequisite passed (`dtech-aidlc config prerequisite check`, `docker info`); PostgreSQL-backed tests ran. Formatting, lint and identifiers passed at command exit level. Clippy emitted warnings, including workspace warnings and warnings in later lifecycle/store code; see `known-limitations.md` and green report.

## Baseline comparison

Baseline source is the recorded baseline and original full logs `regression-wu02-cargo-test.log` and `regression-wu02-cargo-test-ignored.log`. The current full-workspace test-name comparison used exact Rust test names parsed from those logs and the current refresh logs:

| Set | Baseline | Current corresponding passing names | Missing baseline passes |
|---|---:|---:|---:|
| Normal `cargo test` | 290 passed, 0 failed, 39 ignored | all 290 pass | 0 |
| Docker `-- --ignored` | 39 passed, 0 failed | all 39 pass | 0 |

The current full runs also include later tests and fail on them. This exact name comparison distinguishes the recorded baseline set from added work and does not relabel the overall current run as passing.

## Current full-suite results and failure attribution

### Normal workspace suite

`cargo test --no-fail-fast` exited 101. The `domain_backup` library target reported 116 passed, 13 failed, 128 ignored. All 13 failures are `lifecycle::lifecycle_tests::*` (WU-16 lifecycle implementation), not WU-04 filesystem/lock/allocation tests. Other workspace targets in the log passed. The same 13 named lifecycle failures were already present in `refresh-wu03-workspace-normal.log`, before this WU-04 refresh:

- `ts20_registered_row_without_file_is_transitioned_to_release_pending`
- `ts21_file_after_register_before_mark_written_is_retained_as_owned`
- `ts22_old_epoch_running_job_is_failed_with_restart_reason_and_audit`
- `ts23_orphan_is_persisted_before_grace_and_ambiguous_entry_is_untouched`
- `ts25_crashed_old_epoch_deleting_claim_returns_to_cleanup_without_retry`
- `ts26_unlinked_file_with_deleting_row_commits_deleted_idempotently`
- `ts27a_three_unlink_failures_are_spaced_exhausted_and_never_attempted_a_fourth_time`
- `ts29_store_outage_closes_work_and_recovery_reconciles_before_reopening`
- `ts30_fake_time_enforces_retry_expiry_grace_and_sweep_boundaries`
- `ts32_startup_lock_epoch_reconcile_cleanup_barrier_precedes_admissions`
- `ts42_purge_at_retention_boundary_removes_eligible_metadata_but_keeps_audit`
- `ts56_truncated_inventory_makes_reconciliation_fail_closed`
- `ts65_audit_refusal_blocks_delete_claim_delete_commit_and_purge`

### Docker ignored workspace suite

`cargo test --no-fail-fast -- --ignored` exited 101. The `domain_backup` library target reported 121 passed, 7 failed, 0 ignored. All 7 failures are `infra::tests::reconciliation::*` (WU-06 allocation/job reconciliation store behavior). The same seven names fail in `refresh-wu03-workspace-ignored.log`, before this WU-04 refresh:

- `allocation_reconciliation_cas_updates_state_and_audits_atomically`
- `job_store_reconciliation_inventory_and_epoch_are_durable`
- `missing_file_observation_audits_without_mutating_allocation`
- `reconciliation_audit_failure_rolls_back_allocation_and_audit`
- `ts22_job_fence_cas_is_epoch_status_version_checked_and_audited_atomically`
- `ts30_deadline_inventory_includes_only_due_jobs_at_epoch`
- `ts56_allocation_inventory_preserves_nullable_orphan_owners`

Failure details are in the complete refreshed logs. No WU-04-source defect is inferred from these different WU-06/WU-16 paths; however, both full runs remain failures and the broader release candidate cannot be called regression-clean until the owning later-unit defects are addressed and the suites pass.

## WU-04-specific result

The filesystem storage suite passed 30/30; namespace locking passed 11/11; DB allocation-store suite passed 28/28 with Docker. The normal allocation-store command correctly marked all 28 DB tests ignored. Historical WU-04 red evidence documents the original failing behavior followed by green implementation and is not rerun for this refresh. None of the 13 WU-16 failures nor 7 WU-06 failures occurred in these focused WU-04 suites.

No source or test files were modified for this refresh. Web tests were not rerun because WU-04 changes no web paths. Full backend suite exit 101 means intent-level regression is blocked, despite exact preservation of all baseline test names.
