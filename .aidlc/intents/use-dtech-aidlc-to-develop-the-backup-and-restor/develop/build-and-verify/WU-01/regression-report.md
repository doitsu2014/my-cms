# REG-01 Regression report: WU-01

- **Stage:** Develop / build-and-verify, work unit WU-01.
- **Status:** **Regression gate not green.** Focused WU-01 tests pass, but the full current normal and ignored workspace commands each exit 101. Do not report this verification as overall green or complete under `PROJECT.md`'s regression rule.
- **Baseline:** Historical `develop/baseline.md`, recorded 2026-11-22; it was neither rerun nor rewritten. It reports normal workspace 290 passed, 0 failed, 39 ignored, and 39/39 ignored tests passed with Docker.
- **Current environment:** Windows 11; Cargo/rustc 1.100.0; Docker Desktop 29.7.2 available. `dtech-aidlc config prerequisite check` passed.
- **Evidence:** Fresh logs under `../evidence/refresh-wu01-*`; historical baseline logs are linked in `develop/baseline.md`.

## Current full normal workspace run

Command from the repository root: `cargo test --manifest-path apps/api/Cargo.toml --workspace --no-fail-fast`. Exit 101. Raw metadata, stdout and stderr: `../evidence/refresh-wu01-workspace-normal.log`, `.stdout.log`, `.stderr.log`.

| Crate | Passed | Failed | Ignored |
|---|---:|---:|---:|
| `domain_auth` | 29 | 0 | 0 |
| `domain_backup` | 116 | 13 | 129 |
| `domain_interface` | 9 | 0 | 0 |
| `domain_media` | 101 | 0 | 0 |
| `domain_posts` | 50 | 0 | 2 |
| `domain_seo` | 11 | 0 | 1 |
| `domain_user` | 36 | 0 | 0 |
| `gateway` | 17 | 0 | 2 |
| **Workspace total** | **369** | **13** | **134** |

Exact failing identities (all `domain_backup::lifecycle::lifecycle_tests`, currently registered as WU-16 lifecycle work):

1. `ts20_registered_row_without_file_is_transitioned_to_release_pending`
2. `ts21_file_after_register_before_mark_written_is_retained_as_owned`
3. `ts22_old_epoch_running_job_is_failed_with_restart_reason_and_audit`
4. `ts23_orphan_is_persisted_before_grace_and_ambiguous_entry_is_untouched`
5. `ts25_crashed_old_epoch_deleting_claim_returns_to_cleanup_without_retry`
6. `ts26_unlinked_file_with_deleting_row_commits_deleted_idempotently`
7. `ts27a_three_unlink_failures_are_spaced_exhausted_and_never_attempted_a_fourth_time`
8. `ts29_store_outage_closes_work_and_recovery_reconciles_before_reopening`
9. `ts30_fake_time_enforces_retry_expiry_grace_and_sweep_boundaries`
10. `ts32_startup_lock_epoch_reconcile_cleanup_barrier_precedes_admissions`
11. `ts42_purge_at_retention_boundary_removes_eligible_metadata_but_keeps_audit`
12. `ts56_truncated_inventory_makes_reconciliation_fail_closed`
13. `ts65_audit_refusal_blocks_delete_claim_delete_commit_and_purge`

Observed assertion failures are in the captured log; examples include missing reconciliation/fencing work, cleanup results `NotDue` instead of `Deleted`, absent expected purge, and admission/recovery assertions. No cause or authorship is assigned by this WU-01 refresh.

## Current Docker-ignored workspace run

Command from the repository root: `cargo test --manifest-path apps/api/Cargo.toml --workspace --no-fail-fast -- --ignored`. Exit 101. Raw metadata, stdout and stderr: `../evidence/refresh-wu01-workspace-ignored.log`, `.stdout.log`, `.stderr.log`.

- Workspace selected totals: **131 passed, 3 failed**; Docker was available, so this is a real test result, not an environment skip.
- The three failing identities are `domain_backup::infra::tests::reconciliation::job_store_reconciliation_inventory_and_epoch_are_durable`, `domain_backup::infra::tests::reconciliation::ts22_job_fence_cas_is_epoch_status_version_checked_and_audited_atomically`, and `domain_backup::infra::tests::reconciliation::ts30_deadline_inventory_includes_only_due_jobs_at_epoch`.
- Observed assertions include epoch 0 instead of 1, `Lost` instead of `Applied`, and an empty result instead of the expected due-job inventory.
- Current work-unit registration identifies these pending reconciliation/JobStore failures with WU-16 work. This classification does not make the aggregate result green or complete WU-16/WU-06.

## Baseline identity comparison

The immutable historical Rust baseline is the comparison point, not a new run. Test identities were compared across the captured baseline and current raw output:

| Suite | Baseline passing identities | Current matching identities passing | Current failures overlapping baseline passing identities | Result |
|---|---:|---:|---:|---|
| Normal workspace | 290 | 290 | 0 | Every baseline-passing identity still passes; 13 additional current test identities fail. Strict regression gate still fails because the current failing set is nonempty. |
| Ignored workspace | 39 | 39 | 0 | Every baseline ignored-test identity still passes; 3 additional current test identities fail. Strict regression gate still fails because the current failing set is nonempty. |

Thus the observed failures are not identities in the baseline passing set, but they remain failures and prevent the release-candidate regression criterion from passing. This WU-01 report does not attribute them to WU-01. The registered execution order defers WU-06's final strict normal/ignored full-workspace regression closure until after WU-16; that required WU-16-before-WU-06 work remains outstanding.

## Focused verification and other checks

- WU-01 config: 29 passed; state machine: 3 passed; status: 3 passed. Total **35 passed, 0 failed**. Logs: `refresh-wu01-focused-config`, `refresh-wu01-focused-state-machine`, `refresh-wu01-focused-status` (each has metadata, stdout and stderr files).
- `cargo fmt --manifest-path apps/api/Cargo.toml -- --check`: exit 0 (`refresh-wu01-fmt`).
- `cargo clippy --manifest-path apps/api/Cargo.toml --workspace --all-targets`: exit 0; warnings remain (`refresh-wu01-clippy`).
- `cargo check --manifest-path apps/api/Cargo.toml --workspace`: exit 0 (`refresh-wu01-check-workspace`).
- `dtech-aidlc ids check --stage build-and-verify`: exit 0, `ok: true`, no unregistered identifiers (`refresh-wu01-ids-check`).
- Docker prerequisite and tool versions are recorded in the `refresh-wu01-prerequisite` metadata/stdout/stderr logs.

## Conclusion and disposition

WU-01 focused behavior tests pass and this verification made no source or behavior changes. Workspace fmt, clippy and check pass. The current full normal and ignored suites are **red**, with 13 and 3 non-baseline test failures respectively. The project regression rule and Develop exit criteria therefore block any claim of a green overall verification or release candidate. Preserve the failures for their registered owner; execute the required WU-16 work before the deferred WU-06 strict full-regression report. No test was skipped, weakened or reclassified; no baseline, red artifact, workflow state or audit file was changed. No orchestration/report command was run.
