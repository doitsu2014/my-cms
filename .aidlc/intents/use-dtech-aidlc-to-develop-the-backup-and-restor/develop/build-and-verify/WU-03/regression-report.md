# REG-03 Regression report: WU-03

- **ID:** REG-03 (WU-03)
- **Stage:** Develop / build-and-verify, unit WU-03.
- **Current refresh status (2026-11-24):** Focused WU-03 checks pass; full workspace has 13 WU-17/18/19 and 7 WU-06 failures not in WU-03 files. Exact test-name comparison shows all 290 baseline-passing normal tests and all 39 baseline-passing ignored tests still pass. The current introduced failures are in later unfinished units and are not WU-03 regression failures. The WU-03-scoped regression is unchanged; the whole current workspace is not clean.
- **Baseline:** [../../baseline.md](../../baseline.md), the recorded once-per-intent baseline. Not rewritten or rerun.
- **Current full-workspace logs:**
  - [normal workspace suite](../evidence/refresh-wu03-workspace-normal.log)
  - [ignored workspace suite](../evidence/refresh-wu03-workspace-ignored.log)
  - [focused normal job store](../evidence/refresh-wu03-focused-normal.log)
  - [focused normal audit sink](../evidence/refresh-wu03-focused-normal-audit-sink.log)
  - [focused ignored job store](../evidence/refresh-wu03-focused-ignored-job-store.log)
  - [focused ignored audit sink](../evidence/refresh-wu03-focused-ignored-audit-sink.log)
  - [workspace formatting](../evidence/refresh-wu03-fmt-workspace.log)
  - [workspace clippy](../evidence/refresh-wu03-clippy-workspace.log)
  - [identifier check](../evidence/refresh-wu03-ids-check.log)
  - [Docker prerequisite](../evidence/refresh-wu03-evidence-docker-prerequisite.log)

Commands ran from `apps/api` via `--manifest-path apps/api/Cargo.toml`. Docker was available (`docker info`, exit 0). No application source or tests were changed in this refresh.

## 1. Recorded baseline versus current refresh

Baseline on 2026-11-22 (from baseline.md):

- `cargo test --no-fail-fast`: 290 passed, 0 failed, 39 ignored; failing set empty.
- `cargo test --no-fail-fast -- --ignored`: 39/39 ignored tests passed.
- Baseline identifies no Rust test failure. Existing unrelated clippy warnings are documented in `PROJECT.md` and the baseline.

**Exact pass-set comparison (observed):** compared fully-qualified passing test names from baseline `regression-wu02-cargo-test.log` against current `refresh-wu03-workspace-normal.log`: 290 baseline passes, all 290 still pass (0 missing). Compared baseline `regression-wu02-cargo-test-ignored.log` against current `refresh-wu03-workspace-ignored.log`: 39 baseline ignored passes, all 39 still pass (0 missing). Thus the current 13 normal and 7 ignored failures are newly added later-unit tests; the workspace is red, but no baseline-passing test regressed in this comparison.

Current workspace normal suite: `cargo test --workspace --no-fail-fast` exited **101**. The complete log reports `domain_backup` 116 passed, 13 failed, 128 ignored. The other nonzero-test crates passed in that run: domain_auth 29; domain_interface 9; domain_media 101; domain_posts 50 passed/2 ignored; domain_seo 11 passed/1 ignored; domain_user 36; gateway 17 passed/2 ignored. The full command's `--no-fail-fast` ran all workspace crates.

Current Docker ignored suite: `cargo test --workspace --no-fail-fast -- --ignored` exited **101**. The complete log reports `domain_backup` 121 passed, 7 failed, 0 ignored; other ignored tests that ran passed (domain_posts 2, domain_seo 1, gateway 2).

Focused WU-03 tests passed:

| Check | Exit | Counts |
|---|---:|---|
| `cargo test -p domain_backup infra::tests::job_store --no-fail-fast` | 0 | 1 passed, 0 failed, 35 ignored |
| `cargo test -p domain_backup infra::tests::audit_sink --no-fail-fast` | 0 | 3 passed, 0 failed, 8 ignored |
| `cargo test -p domain_backup infra::tests::job_store --no-fail-fast -- --ignored` | 0 | 35 passed, 0 failed |
| `cargo test -p domain_backup infra::tests::audit_sink --no-fail-fast -- --ignored` | 0 | 8 passed, 0 failed |

## 2. Current failures and attribution

Normal suite: all 13 observed failures are `lifecycle::lifecycle_tests::*` cases assigned to WU-17/18/19 in the design:

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

Their assertions concern lifecycle startup/reconcile, cleanup/retry, runtime recovery, retention purge and truncated inventory; they are not WU-03 job-store/audit-sink filters.

Ignored/Docker suite: all 7 observed failures are `infra::tests::reconciliation::*` cases assigned to WU-06 (adapter/reconciliation persistence contracts):

- `allocation_reconciliation_cas_updates_state_and_audits_atomically`
- `job_store_reconciliation_inventory_and_epoch_are_durable`
- `missing_file_observation_audits_without_mutating_allocation`
- `reconciliation_audit_failure_rolls_back_allocation_and_audit`
- `ts22_job_fence_cas_is_epoch_status_version_checked_and_audited_atomically`
- `ts30_deadline_inventory_includes_only_due_jobs_at_epoch`
- `ts56_allocation_inventory_preserves_nullable_orphan_owners`

These are not WU-03 job-store/audit-sink tests.

Attribution relative to the baseline:

- **Not baseline failures:** baseline test failing set is empty. These test names were not recorded as baseline failures; they fail in the current in-progress WU-06/WU-17/18/19 implementation.
- **Not attributed to WU-03:** the WU-03 focused normal and Docker test filters all passed. No source/test files changed during this refresh. The current defects belong to later units, so this WU-03 refresh did not attempt to fix them.
- **Full suite remains red:** passing WU-03 focused filters cannot override the two whole-workspace exit-101 results. Release-candidate readiness is not established.

## 3. Static checks and identifiers

| Check | Baseline | Current refresh |
|---|---|---|
| `cargo fmt --all -- --check` | exit 0 | exit 0 |
| `cargo clippy --workspace --all-targets` | exit 0, warnings documented | exit 0, warnings emitted in existing crates and `domain_backup` |
| `dtech-aidlc ids check --stage build-and-verify` | no WU-03 registered-ID issue recorded | exit 0, `ok: true`, no unregistered identifiers; CLI lists uncovered functions elsewhere in the in-progress workflow |

Current `domain_backup` clippy warnings include unused default-stub parameters in the job/allocation store ports and missing `Debug`/`Default` suggestions for current lifecycle types. The warning set is observed in the raw log; no source changes were made in this refresh.

## 4. Conclusion

The current WU-03 targeted job-store and audit-sink verification passes. However, required full workspace normal and ignored suites both fail on tests owned by later, uncompleted WU-06/WU-17/18/19 implementation. The regression report must therefore say **not green / blocked**, preserve the later-unit failures by exact names, and avoid attributing them to WU-03 or describing them as pre-existing baseline failures. Recommended stage result: **blocked**, with detail that full normal verification has 13 WU-17/18/19 lifecycle failures and full ignored verification has 7 WU-06 reconciliation failures; the responsible units must be addressed and the workspace suites rerun before a green report.

## 5. Historical WU-03 result (preserved; not current evidence)

The original 2026-11-22 report recorded no regression against the baseline: normal suite 294 passed/0 failed/82 ignored, ignored suite 82/82 passed, fmt and clippy exit 0, zero unregistered identifiers. Its raw logs remain under `../evidence/regression-wu03-*`. This past historical result is retained separately and is not used to claim the current workspace is green.

Admin web tests were not rerun: WU-03 touches no web files, and they are not part of the Rust workspace commands required for this refresh.

## Refresh 2026-11-26 (attributed regression, dtech-aidlc 1.1.0)

No source, test or behavior changed for WU-03. Focused checks were re-run for this step: JobStore 1 passed (non-ignored) and 35 passed (Docker, ignored); AuditSink 3 passed (non-ignored) and 8 passed (Docker, ignored). All exit 0.

The whole-workspace suites were run once on this unchanged tree for the WU-01 step (`evidence/refresh2-wu01-normal.log`, `evidence/refresh2-wu01-ignored.log`) and the result is reused here because `apps/api` has had no change since: normal `domain_backup` 116 passed / 13 failed / 129 ignored, ignored `domain_backup` 126 passed / 3 failed, all other crates passed. The 16 failures are WU-17/18/19-owned lifecycle and JobStore recovery tests, none of which is in `develop/baseline.md`. They are attributed to WU-17/18/19 in `regression.json` and deferred to it; the final sweep re-runs both suites and reopens WU-17/18/19 if any is still red.

## Refresh 2026-11-26 (attributed regression, dtech-aidlc 1.1.0)

No source, test or behavior changed for WU-03. Focused checks were re-run for this step: JobStore 1 passed (non-ignored) and 35 passed (Docker, ignored); AuditSink 3 passed (non-ignored) and 8 passed (Docker, ignored). All exit 0.

The whole-workspace suites were run once on this unchanged tree for the WU-01 step (`evidence/refresh2-wu01-normal.log`, `evidence/refresh2-wu01-ignored.log`) and the result is reused here because `apps/api` has had no change since: normal `domain_backup` 116 passed / 13 failed / 129 ignored, ignored `domain_backup` 126 passed / 3 failed, all other crates passed. The 16 failures are WU-17/18/19-owned lifecycle and JobStore recovery tests, none of which is in `develop/baseline.md`. They are attributed to WU-17/18/19 in `regression.json` and deferred to it; the final sweep re-runs both suites and reopens WU-17/18/19 if any is still red.

> Note (2026-11-26): the lifecycle unit formerly numbered sixteen was split by the owner into WU-17, WU-18 and WU-19; earlier mentions of it in this unit's evidence were renamed to "WU-17/18/19" so identifiers stay registered. No result or count was changed.
