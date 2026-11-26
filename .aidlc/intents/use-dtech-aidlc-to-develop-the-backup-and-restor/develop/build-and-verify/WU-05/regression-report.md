# REG-05 Regression report: WU-05

- **ID:** REG-05 (WU-05)
- **Stage:** Develop / build-and-verify, unit WU-05.
- **Status:** No regression observed on Windows 11, 2026-11-23; Docker-backed tests all ran green.
- **Baseline:** [baseline.md](../../baseline.md), recorded at the start of this intent: Rust workspace 290 passed, 0 failed, 39 ignored; all 39 ignored tests passed when run with Docker.
- **After:** WU-05 adds worker, fixture capture and tests; full outputs are linked below.

## Verification

| Check | After WU-05 | Result |
|---|---|---|
| `cargo test --no-fail-fast` from `apps/api` | 367 passed, 0 failed, 125 ignored | exit 0 |
| `cargo test --no-fail-fast -- --ignored` from `apps/api` | 125 passed, 0 failed | exit 0 |
| `cargo check --workspace` | completed | exit 0 |
| `cargo fmt -- --check` | completed | exit 0 |
| `cargo clippy -p domain_backup --all-targets` | completed, no warning attributed to `domain_backup` | exit 0 |
| `cargo clippy --all-targets` | completed, existing warnings in other crates remain | exit 0 |
| `dtech-aidlc ids check` | 0 unregistered identifiers | ok |

Logs:
- [workspace tests](../evidence/regression-wu05-cargo-test.log)
- [workspace ignored tests](../evidence/regression-wu05-cargo-test-ignored.log)
- [workspace check](../evidence/green-wu05-cargo-check-workspace.log)
- [format](../evidence/green-wu05-cargo-fmt.log)
- [workspace Clippy](../evidence/regression-wu05-cargo-clippy.log)
- [crate Clippy](../evidence/green-wu05-cargo-clippy.log)

## Baseline comparison

- **Baseline failing set:** none with Docker available. The 39 tests marked ignored in the baseline require PostgreSQL testcontainers; all passed in the baseline ignored run.
- **After WU-05:** the workspace contains 367 passing tests, no failures, and 125 ignored by default. The explicit ignored run passes all 125.
- **Difference:** The aggregate workspace count increased by 47 from the WU-04 result (335 passed + 110 ignored = 445) to the WU-05 result (367 passed + 125 ignored = 492). The WU-05 red manifest records 46 tests and one additional deadline rework regression; the command's suite totals are authoritative for this report. No previously passing test fails.
- The WU-05 package totals are 114 passed / 0 failed / 120 ignored in the normal run and 120 passed / 0 failed with `-- --ignored`.
- No web files changed; web tests, lint and typecheck were not rerun for this backend-only work unit.

## Scope and caveats

- Test files from the red commit remain intact; the only addition is the registration-delay deadline regression and test-only fake allocation-store delay control. `Cargo.lock` is unchanged.
- Unix-only permission and file-symlink checks remain unverified on this Windows host; WU-04 records Ubuntu CI as the required verification before the release-candidate gate.
- Existing Clippy warnings outside `domain_backup` remain unchanged.

## 5. Verification refresh (Observed, 2026-11-24)

The recorded baseline in [baseline.md](../../baseline.md) was reused, not rerun or rewritten. Docker prerequisite passed. No red replay was repeated because scoped WU-05 tests are green and historic WU-05 red evidence is preserved.

| Command (from `apps/api`) | Current result | Log |
|---|---|---|
| `cargo test -p domain_backup lifecycle::worker::tests --no-fail-fast` | 27 passed, 0 failed | [focused worker](../evidence/refresh-wu05-focused-worker.log) |
| `cargo test -p domain_backup lifecycle::fixture_capture::tests --no-fail-fast` | 5 passed, 0 failed | [focused fixture](../evidence/refresh-wu05-focused-fixture.log) |
| `cargo test -p domain_backup infra::tests::worker:: --no-fail-fast -- --ignored` | 15 passed, 0 failed | [Docker worker integration](../evidence/refresh-wu05-focused-worker-docker.log) |
| `cargo test --no-fail-fast` | **369 passed, 13 failed, 133 ignored** across workspace; exit 101 | [workspace normal](../evidence/refresh-wu05-workspace-normal.log) |
| `cargo test --no-fail-fast -- --ignored` | **126 passed, 7 failed**; exit 101 | [workspace ignored](../evidence/refresh-wu05-workspace-ignored.log) |
| `cargo fmt -- --check` | exit 0 | [fmt](../evidence/refresh-wu05-fmt.log) |
| `cargo clippy --all-targets` | exit 0; existing warnings | [Clippy](../evidence/refresh-wu05-clippy.log) |
| `dtech-aidlc ids check` | `ok`; 0 unregistered identifiers; 35 uncovered functions advisory | [IDs](../evidence/refresh-wu05-ids-check.log) |

### Baseline pass-set comparison

- Baseline normal run: **290 passed, 0 failed, 39 ignored**. Every one of the 290 baseline-passing test names remains passing in the current normal run (**290/290 retained**); none of the 13 failures below was in that baseline pass set. The current run has 79 additional passing test names and 13 new failing names.
- Baseline ignored run: **39 passed, 0 failed**. Every one of those 39 baseline-passing test names remains passing (**39/39 retained**); none of the 7 failures below was in that baseline pass set. The current run has 87 additional passing test names and 7 new failing names.
- Therefore, no baseline-passing test regression was observed. The current full-workspace commands are nevertheless failed, not green: all 20 failures are newly introduced tests in uncompleted WU-06/WU-16 work visible in the working tree, and are outside WU-05's declared test ownership. The WU-05 focused worker/fixture/Docker tests all pass.

**Exact new failures in normal workspace run (WU-16 lifecycle tests):**

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

**Exact new failures in ignored workspace run (WU-06 reconciliation-store tests):**

1. `infra::tests::reconciliation::allocation_reconciliation_cas_updates_state_and_audits_atomically`
2. `infra::tests::reconciliation::job_store_reconciliation_inventory_and_epoch_are_durable`
3. `infra::tests::reconciliation::missing_file_observation_audits_without_mutating_allocation`
4. `infra::tests::reconciliation::reconciliation_audit_failure_rolls_back_allocation_and_audit`
5. `infra::tests::reconciliation::ts22_job_fence_cas_is_epoch_status_version_checked_and_audited_atomically`
6. `infra::tests::reconciliation::ts30_deadline_inventory_includes_only_due_jobs_at_epoch`
7. `infra::tests::reconciliation::ts56_allocation_inventory_preserves_nullable_orphan_owners`

**Recommended stage result:** complete for this WU-05 build-and-verify unit: its declared focused tests, Docker worker integration tests, formatter, workspace Clippy and ID registry check pass; no baseline-passing tests failed. Carry forward the explicitly failed full-workspace status and the named WU-06/WU-16 failures for their owning work units. This recommendation is not a gate approval or CLI report.
