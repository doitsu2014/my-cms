# GVR-03 Green verification report: WU-03

- **ID:** GVR-03 (WU-03)
- **Stage:** Develop / build-and-verify, work unit WU-03. This is not the release-candidate gate.
- **Current refresh status:** **Focused WU-03 checks passed; full workspace verification is blocked / not green** (2026-11-24). The historical 2026-11-22 green report is retained in section 7 and in [historical reports](../evidence/).
- **Inputs:** [red-tests/WU-03](../../red-tests/WU-03/failing-tests.md), [implementation.md](implementation.md), [ids.json](../../implementation-design/ids.json), Analyze acceptance criteria and quality requirements.
- **Refresh logs:**
  - [refresh-wu03-focused-normal.log](../evidence/refresh-wu03-focused-normal.log)
  - [refresh-wu03-focused-normal-audit-sink.log](../evidence/refresh-wu03-focused-normal-audit-sink.log)
  - [refresh-wu03-focused-ignored-job-store.log](../evidence/refresh-wu03-focused-ignored-job-store.log)
  - [refresh-wu03-focused-ignored-audit-sink.log](../evidence/refresh-wu03-focused-ignored-audit-sink.log)
  - [refresh-wu03-workspace-normal.log](../evidence/refresh-wu03-workspace-normal.log)
  - [refresh-wu03-workspace-ignored.log](../evidence/refresh-wu03-workspace-ignored.log)
  - [refresh-wu03-fmt-workspace.log](../evidence/refresh-wu03-fmt-workspace.log)
  - [refresh-wu03-clippy-workspace.log](../evidence/refresh-wu03-clippy-workspace.log)
  - [refresh-wu03-ids-check.log](../evidence/refresh-wu03-ids-check.log)
  - [refresh-wu03-evidence-docker-prerequisite.log](../evidence/refresh-wu03-evidence-docker-prerequisite.log)

## 1. Observed verification results (2026-11-24)

All commands ran from the API workspace path `apps/api` using its manifest. Docker prerequisite `docker info` exited 0.

| Scope / command | Exit | Observed result |
|---|---:|---|
| Focused normal: `cargo test -p domain_backup infra::tests::job_store --no-fail-fast` | 0 | 1 passed, 0 failed, 35 ignored (the DB tests are ignored by default) |
| Focused normal: `cargo test -p domain_backup infra::tests::audit_sink --no-fail-fast` | 0 | 3 passed, 0 failed, 8 ignored |
| Focused Docker: `cargo test -p domain_backup infra::tests::job_store --no-fail-fast -- --ignored` | 0 | 35 passed, 0 failed, 0 ignored |
| Focused Docker: `cargo test -p domain_backup infra::tests::audit_sink --no-fail-fast -- --ignored` | 0 | 8 passed, 0 failed, 0 ignored |
| Full workspace: `cargo test --workspace --no-fail-fast` | **101** | **Not green.** `domain_backup`: 116 passed, 13 failed, 128 ignored. Other nonzero-count suites passed: domain_auth 29; domain_interface 9; domain_media 101; domain_posts 50 passed/2 ignored; domain_seo 11 passed/1 ignored; domain_user 36; gateway 17 passed/2 ignored. |
| Full ignored workspace: `cargo test --workspace --no-fail-fast -- --ignored` | **101** | **Not green.** `domain_backup`: 121 passed, 7 failed, 0 ignored. Other DB ignored tests passed in this run (domain_posts 2, domain_seo 1, gateway 2). |
| `cargo fmt --all -- --check` | 0 | Workspace formatting passed. |
| `cargo clippy --workspace --all-targets` | 0 | Command passed; warnings were emitted, including existing warnings in other crates and current warnings in `domain_backup` (stub unused parameters and lifecycle types without Debug/Default). No source changed to address them. |
| `dtech-aidlc ids check --stage build-and-verify` | 0 | `ok: true`; no unregistered identifiers. CLI also listed uncovered functions across the in-progress overall workflow; no WU-03 ID was unregistered. |

Exit 101 in the table is Cargo's test failure exit (PowerShell `$LASTEXITCODE`).

## 2. Current suite failures and scope attribution

The failing full-workspace normal suite names 13 in-memory lifecycle tests under WU-17/18/19:

- `lifecycle::lifecycle_tests::ts20_registered_row_without_file_is_transitioned_to_release_pending`
- `lifecycle::lifecycle_tests::ts21_file_after_register_before_mark_written_is_retained_as_owned`
- `lifecycle::lifecycle_tests::ts22_old_epoch_running_job_is_failed_with_restart_reason_and_audit`
- `lifecycle::lifecycle_tests::ts23_orphan_is_persisted_before_grace_and_ambiguous_entry_is_untouched`
- `lifecycle::lifecycle_tests::ts25_crashed_old_epoch_deleting_claim_returns_to_cleanup_without_retry`
- `lifecycle::lifecycle_tests::ts26_unlinked_file_with_deleting_row_commits_deleted_idempotently`
- `lifecycle::lifecycle_tests::ts27a_three_unlink_failures_are_spaced_exhausted_and_never_attempted_a_fourth_time`
- `lifecycle::lifecycle_tests::ts29_store_outage_closes_work_and_recovery_reconciles_before_reopening`
- `lifecycle::lifecycle_tests::ts30_fake_time_enforces_retry_expiry_grace_and_sweep_boundaries`
- `lifecycle::lifecycle_tests::ts32_startup_lock_epoch_reconcile_cleanup_barrier_precedes_admissions`
- `lifecycle::lifecycle_tests::ts42_purge_at_retention_boundary_removes_eligible_metadata_but_keeps_audit`
- `lifecycle::lifecycle_tests::ts56_truncated_inventory_makes_reconciliation_fail_closed`
- `lifecycle::lifecycle_tests::ts65_audit_refusal_blocks_delete_claim_delete_commit_and_purge`

The failing full-workspace ignored suite names seven DB reconciliation tests under WU-06:

- `infra::tests::reconciliation::allocation_reconciliation_cas_updates_state_and_audits_atomically`
- `infra::tests::reconciliation::job_store_reconciliation_inventory_and_epoch_are_durable`
- `infra::tests::reconciliation::missing_file_observation_audits_without_mutating_allocation`
- `infra::tests::reconciliation::reconciliation_audit_failure_rolls_back_allocation_and_audit`
- `infra::tests::reconciliation::ts22_job_fence_cas_is_epoch_status_version_checked_and_audited_atomically`
- `infra::tests::reconciliation::ts30_deadline_inventory_includes_only_due_jobs_at_epoch`
- `infra::tests::reconciliation::ts56_allocation_inventory_preserves_nullable_orphan_owners`

The failures occur in lifecycle/reconciliation implementation and assertions outside the WU-03 job-store and audit-sink test filters. They are current failures, not baseline failures: the recorded 2026-11-22 WU-03 baseline is 290 passed, 0 failed, 39 ignored (39/39 ignored passed), and its failing set is empty. No corresponding baseline test failure is recorded. The current workspace outcomes must therefore remain explicitly red; these later-unit failures are not evidence that WU-03 itself regressed, nor do they permit an overall green claim.

## 3. WU-03 test evidence and red-to-green relationship

- Historical red run (2026-11-22): 45 new tests failed on assertions against the logic-free shell; see [red evidence](../../red-tests/WU-03/red-evidence.md) and its linked raw log.
- Historical green run (2026-11-22): 41 unit tests passed, 77 ignored DB tests passed when run with Docker. Two test-after hardening tests were added and mutation-checked; see section 7 for historical result links.
- Current focused run: job-store and audit-sink selected tests passed as recorded in section 1. Each targeted WU-03 Docker test passed (43 tests total across the two filters), alongside the selected unit tests (4 total across filters). Workspace normal/ignored results are not green due to the later-unit failures in section 2.
- No tests were changed, skipped, weakened, or added during this refresh. No production source or test files were edited.

## 4. Results per level and quality requirements

| Level | Current WU-03 refresh result |
|---|---|
| Unit | Focused `job_store` and `audit_sink` tests passed: 4 passed, 0 failed across normal runs; 43 Docker-ignored-filtered tests passed. Full workspace normal suite is blocked by 13 WU-17/18/19 tests. |
| Integration | Focused Docker job-store and audit-sink tests passed: 43 passed, 0 failed. Full workspace ignored suite is blocked by 7 WU-06 reconciliation tests. |
| Concurrency | Historical WU-03 claim/CAS races were green; current focused job-store filter passed. Full current workspace remains red for the independently identified WU-06/WU-17/18/19 tests. |
| End to end | Not run; WU-03 store is not composed into the running API (WU-08/WU-10). |
| Performance | Not run; outside WU-03. |
| Security | WU-03 store-level audit transaction behavior has current focused test passes. Broader T-08/T-09 application checks remain for their owning units. |
| Static | `cargo fmt` passed; workspace `cargo clippy` exited 0 with warnings (including `domain_backup` warnings). |

| Quality requirement | Current WU-03 refresh status |
|---|---|
| A-01 Job idempotency | Partial: atomic job/idempotency insert is covered by focused Docker job-store test; handler replay and key-reuse decision are WU-07. |
| P-05 Concurrent job limits | Focused Docker test suite passed, including durable concurrency and racing accepts. |
| A-03 Crash recovery | Partial: store CAS and late-worker protections remain WU-03; reconciliation/fencing tests are WU-06/WU-17/18/19 and are currently failing as listed. |
| PR-04 Audit content safety | Partial: focused audit/job-store checks passed; full projection/canary checks remain later work. |
| O-02 Safe error projection | Partial: store/audit errors are safe; HTTP projection is WU-08. |
| Other requirements | Not in WU-03 scope or not run. |

## 5. Codebase map

`domain_backup` gains ports and infra but is not composed into the gateway. The codebase map update remains assigned to WU-10; not changed here.

## 6. Trace table (cumulative WU-01..WU-03)

| Story | Criterion | Function | Test | Result |
|---|---|---|---|---|
| US-01, US-05 | AC-001 Instance selection and authorization | FN-01 | WU-01 config tests; listing and authorization are WU-08/WU-13. | Config part passed historically; rest not run here. |
| US-02, US-06 | AC-002 S1 Job status is observable | FN-15, FN-16 | `accept_inserts_a_queued_job_and_its_idempotency_row`, `get_returns_the_stored_row_and_none_for_an_unknown_id`, `claim_next_claims_the_oldest_queued_job`, `concurrent_accepts_for_one_instance_create_exactly_one_job`, `exactly_one_of_n_concurrent_claims_wins` | Focused current WU-03 Docker tests passed; API/UI not run. |
| US-02, US-06 | AC-002 S2 Required capture fails | FN-15 | `each_legal_edge_applies_bumps_the_version_and_stamps_times`, `a_late_worker_cannot_overwrite_a_terminal_result`, `racing_terminal_transitions_have_exactly_one_winner`, `a_stale_version_or_status_loses_and_changes_nothing` | Focused current tests passed; worker outcome WU-05 not run as part of targeted WU-03 check. |
| US-02, US-06 | AC-002 S3 U1 database-only result | FN-15 | WU-01/WU-02 guard tests; `terminal_rows_are_immutable_through_the_store`, `edges_outside_the_u1_graph_are_refused_without_changes` | Historical pass; current targeted job-store run passed. |
| US-03 | AC-003 S1 Invalid or incompatible package | none (rejection previews in WU-09) | none yet | Not run. |
| US-03 | AC-003 S2 Unsafe archive | none (deferred) | none | Not run (deferred). |
| US-02, US-03, US-04, US-06 | AC-003 S3 U1 real-data staging is not enabled | FN-01; allocation schema | WU-01 config tests; WU-02 allocation checks | Historical pass; not part of this focused WU-03 refresh. |
| US-04 | AC-004 S1 Confirmation describes impact | none yet (WU-09) | none yet | Not run. |
| US-04 | AC-004 S2 Restore is cancelled | none yet (WU-09) | none yet | Not run. |
| US-04 | AC-004 S3 Restore failure | none (restore not authorized) | none | Not run (deferred). |
| US-05, US-06 | AC-005 S1 Unauthorized operation or resource access | FN-39, FN-01; idempotency scope | WU-01 loopback and origin tests; WU-02 idempotency scope; `accept_refuses_digests_that_are_not_sha256_hex_without_writing` | Historical guard/config/store scope passed; authorization not run here. |
| US-05, US-06 | AC-005 S2 Audit records a sensitive operation | FN-34 | `a_transition_writes_one_job_audit_row_with_its_event_and_ids`, `a_transition_hands_its_own_uncommitted_transaction_to_the_audit_sink`, `a_row_recorded_on_the_callers_transaction_is_visible_only_after_commit`, `rolling_back_the_callers_transaction_leaves_no_audit_row`, `every_column_of_the_event_is_persisted`, `the_job_audit_row_excludes_free_text_stored_on_the_job` | Current focused job-store/audit-sink tests passed. |
| US-05, US-06 | AC-005 S3 Audit store unavailable before effect | FN-34 | `an_audit_failure_refuses_the_transition`, `an_audit_failure_refuses_the_claim`, `an_audit_outage_is_a_store_outage_for_transitions_and_claims`, `sqlstates_split_into_statement_rejections_and_fail_closed_outages`, `connection_level_and_unclassifiable_failures_are_an_outage` | Current focused tests passed. |
| US-02, US-03, US-04 | AC-006 S1 Production restore readiness | none (production not authorized) | none | Not run (out of scope). |
| US-02, US-03, US-04 | AC-006 S2 Unresolved U1 design blockers | FN-01, FN-37 | WU-01/WU-02 tests | Historical pass. |

## 7. Historical WU-03 verification (preserved; 2026-11-22)

The original report recorded a full green WU-03 run: normal crate suite 41 passed/0 failed/77 ignored; ignored DB suite 77 passed/0 failed; format and clippy exits 0; zero unregistered identifiers. Historical mutation and red-to-green evidence remain available in:

- [green-wu03-cargo-test.log](../evidence/green-wu03-cargo-test.log)
- [mutation-wu03-claim.log](../evidence/mutation-wu03-claim.log)
- [mutation-wu03-test-after.log](../evidence/mutation-wu03-test-after.log)
- [historical regression report](regression-report.md) links to the original `regression-wu03-*` logs.

This historical run is not a substitute for the current full-workspace refresh and does not change the current blocked status.

## Refresh 2026-11-26

Re-verified after the 1.1.0 migration with no WU-03 source or test changes: JobStore 1 passed (non-ignored) and 35 passed (Docker, ignored); AuditSink 3 passed (non-ignored) and 8 passed (Docker, ignored). Whole-workspace regression and its attribution are in `regression-report.md` and `regression.json`.

## Refresh 2026-11-26

Re-verified after the 1.1.0 migration with no WU-03 source or test changes: JobStore 1 passed (non-ignored) and 35 passed (Docker, ignored); AuditSink 3 passed (non-ignored) and 8 passed (Docker, ignored). Whole-workspace regression and its attribution are in `regression-report.md` and `regression.json`.
