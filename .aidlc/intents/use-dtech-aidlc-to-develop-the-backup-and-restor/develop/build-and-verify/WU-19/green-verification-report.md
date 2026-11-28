# GV-19 Green verification report: WU-19

- **ID:** GV-19 (WU-19)
- **Stage:** Develop / build-and-verify, unit WU-19.
- **Status:** focused tests and workspace checks passed on 2026-11-26. Whole-workspace regression comparison: see [regression-report.md](regression-report.md).
- **Environment:** Windows 11, cargo/rustc 1.100.0, Docker 29.7.2, PostgreSQL 15 testcontainers. Logs are in [../evidence/](../evidence/).

## Results observed

| Command (from `apps/api`) | Exit | Result | Log |
|---|---:|---|---|
| `cargo test -p domain_backup infra::tests::sweeper -- --ignored` | 0 | 11 passed, 0 failed | `green-wu19-sweeper-db.log` |
| `cargo test -p domain_backup infra::tests::runtime -- --ignored` | 0 | 12 passed, 0 failed | `green-wu19-runtime-db.log` |
| `cargo test -p domain_backup infra::tests::reconciliation -- --ignored` | 0 | 7 passed, 0 failed; includes `ts30_deadline_inventory_includes_only_due_jobs_at_epoch` | `green-wu19-jobstore-db.log` |
| `cargo fmt --all -- --check` | 0 | formatted | `green-wu19-fmt.log` (empty on success) |
| `cargo clippy --workspace --all-targets` | 0 | completed; no WU-19 warnings. Existing warnings remain in other crates and an existing complex type in `domain_auth`. | `green-wu19-clippy.log` |
| `dtech-aidlc --project D:/orca/my-cms/makara ids check --stage build-and-verify` | 0 | `ok: true`; no unregistered IDs | CLI output observed |

The new regression test `a_deadline_failed_job_releases_its_unexpired_written_allocation` first failed behaviorally: at the terminal-owner release assertion, observed 0 versus expected 1. After the sweeper was corrected, it passed as part of the 11-test focused suite.

## Mutation checks

The initial mutation pass killed 18 of 21 mutants; three survived. Tests were added for the three survivors, then the same mutants were rerun and all three were killed:

| Initially surviving mutant | Test that killed it on rerun |
|---|---|
| recovery reopens after reconcile audit refusal | `ts29_a_refused_reconcile_audit_keeps_recovery_closed` |
| successful sweep fails to reset refused-audit count | `ts29_a_good_sweep_resets_the_refused_audit_count` |
| truncated tick incorrectly closes admissions | `ts56_a_truncated_sweep_scan_degrades_but_keeps_admissions_open` |

The final WU-19 mutator rerun restored `runtime.rs` byte-for-byte (`cmp` verified). The sweeper mutant set all killed except where noted in the first-pass log: its complete row lists nine sweeper mutants, each killed. First-pass raw data: `green-wu19-mutations.json`; second-pass three-mutant results were observed in the session but the scratchpad was subsequently unavailable, so the report records their observed test results above and does not claim a durable second-pass JSON file.

## Durable behavior exercised

Tests seed PostgreSQL rows, use the real filesystem adapter and audit sink, and assert stored rows, audit events and file state. Coverage includes deadline and queued-age boundaries, current-epoch filtering, active-owner expiry protection, terminal release, 24-hour expiry, purge through the injected retention port, retained audit rows, discovery without deletion, truncated inventory, startup ordering, lock contention, outages, recovery ordering, audit refusal bounds and tick intervals.

## Trace table

| Story | Criterion | Function | Test | Result |
|---|---|---|---|---|
| US-02 | AC-002 S2 | FN-20 | `ts30_a_running_job_fails_deadline_exceeded_exactly_at_its_deadline_and_not_before`; `the_sweep_ignores_a_job_of_another_epoch_even_past_its_deadline`; `ts30_a_queued_job_fails_dispatch_failed_at_the_queued_age_bound`; `a_deadline_failed_job_releases_its_unexpired_written_allocation` | passed |
| US-02 | AC-002 S2 | FN-32 | `ts28_an_expired_written_allocation_of_a_running_job_is_kept_until_the_owner_ends`; `ts30_a_written_allocation_is_released_exactly_at_its_24_hour_expiry`; `ts30_the_cleaner_ticks_every_60_seconds_and_the_sweep_every_15_minutes` | passed |
| US-02 | AC-002 S2; AC-005 S3 | FN-31 | `ts32_startup_runs_lock_epoch_reconcile_cleanup_then_opens_admissions`; `ts32_a_second_runtime_cannot_acquire_the_lock_and_mutates_nothing`; `ts56_a_truncated_inventory_keeps_startup_closed_and_not_ready`; `a_runtime_that_was_never_started_refuses_ticks_and_recovery`; `ts29_a_store_outage_closes_admissions_and_stops_all_store_work`; `ts29_recovery_reconciles_then_cleans_then_sweeps_then_reopens_without_a_new_epoch`; `ts29_a_truncated_recovery_stays_closed_and_degraded_until_it_succeeds`; `ts29_three_refused_audit_sweeps_close_admissions_and_degrade`; `ts29_a_good_sweep_resets_the_refused_audit_count`; `ts56_a_truncated_sweep_scan_degrades_but_keeps_admissions_open`; `ts29_a_refused_reconcile_audit_keeps_recovery_closed` | passed |
| US-02 | AC-002 S2; AC-003 S3 | FN-32 | `one_sweep_runs_deadline_then_expiry_then_purge_then_discovery`; `a_truncated_scan_fails_the_sweep_after_the_earlier_steps_and_persists_no_orphan`; `the_sweep_discovers_an_unowned_file_without_deleting_it` | passed |
| US-02 | AC-002 S2; AC-003 S3 | FN-33 | `ts42_the_sweep_purges_through_the_injected_retention_store_and_keeps_audit`; `ts65_a_refused_purge_audit_removes_nothing` | passed |
| US-03 | AC-003 S3 | FN-32 | `ts30_deadline_inventory_includes_only_due_jobs_at_epoch` (JobStore implementation exercised in reconciliation integration suite) | passed |

No additional `AC-005` scenario is claimed: audit refusal cases above exercise the relevant lifecycle safety behavior.
