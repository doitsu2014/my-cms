# FT-05 Failing tests (code manifest): WU-05

- **ID:** FT-05 (WU-05)
- **Stage:** Develop / red-tests (builder-verifier), unit WU-05 of WU-01..WU-15.
- **Status:** red run recorded 2026-11-23 on Windows, with PostgreSQL 15 via Docker.
- **Inputs:**
  - [work-units](../../implementation-design/work-units.md) WU-05 and discrepancy 1
  - [ids.json](../../implementation-design/ids.json)
  - [test-strategy](../../implementation-design/test-strategy.md) TS-05a, TS-44b (job events), TS-47, TS-48, TS-67, TS-68
  - [interfaces-and-contracts](../../implementation-design/interfaces-and-contracts.md) sections 2.3, 6.3, 7, 8, 9 and 10.1
  - [functional-design](../../implementation-design/functional-design.md) FN-16..FN-20 (legacy F-16..F-20), FN-04 at claim, and 6.6
- **Evidence:** [red-evidence.md](red-evidence.md); raw logs:
  - [../evidence/red-wu05-cargo-test.log](../evidence/red-wu05-cargo-test.log)
  - [../evidence/red-wu05-cargo-test-ignored.log](../evidence/red-wu05-cargo-test-ignored.log)

Provenance:
- **Observed:** results from the conductor's run on 2026-11-23.
- **Identifiers:** registry and Analyze only. `dtech-aidlc ids check` reports none unregistered.
- **Mappings:** T and FN as the test-strategy rows and the builder's table assign them. AC-002 is the U1 job outcome criterion; AC-005 S1 covers the claim re-check and AC-005 S2 the audit events.

## Files

Production shells and tests live under `apps/api/domain_backup/src/`.

| Path | Change | Shell behaviour |
|---|---|---|
| `ports/capture.rs` | new | `CaptureAdapter` trait, plus `CaptureContext` (job, instance, correlation id, opaque storage key), `CaptureOutcome` and `CaptureError::Failed` |
| `ports/policy.rs` | new | `ClaimPolicy::authorize_resource(instance_id, job_id)`: the claim-time FN-04 re-check port. WU-08 supplies the real policy. |
| `lifecycle/worker.rs` | new | `Worker` (`run_next`, `execute`, `fail_dispatch`), `WorkerConfig`, `RunOutcome`. Logic-free stub: `run_next` returns `Idle`; `execute` and `fail_dispatch` return `Lost`; no deadline. |
| `lifecycle/fixture_capture.rs` | new | `FixtureCaptureAdapter` and `FIXTURE_BYTES` (4096). Logic-free stub: writes nothing and reports empty components. |
| `lifecycle/test_support.rs` | new, `#[cfg(test)]` | `FakeClock`, `advance_both` (FakeClock plus paused tokio time), in-memory job, allocation and storage doubles, a scripted adapter, a policy double, `wait_entered`, `join_within` |
| `infra/tests/worker.rs` | new | DB-backed worker tests on the real WU-03/WU-04 stores and filesystem |
| `lib.rs`, `lifecycle/mod.rs`, `ports/mod.rs`, `infra/tests/mod.rs` | new or modified | Module wiring only |
| `Cargo.toml` | modified | `tokio` 1.52.3 added to `[dependencies]` (`macros`, `rt`, `sync`, `time`, no default features). The dev-dependency gains `test-util` for paused time. |

- **Lock file:** `apps/api/Cargo.lock` is unchanged and no crate was added; `tokio-util` was not needed.
- **Earlier units:** no WU-01..WU-04 file changed. That includes the test support files.
- **Only adapter:** `FixtureCaptureAdapter` is the only `CaptureAdapter` in the crate (T-05).

## Tests

- **Commands:** from `apps/api`, `cargo test -p domain_backup --no-fail-fast`, then the same with `-- --ignored`.
- **Docker:** `infra::tests::worker::` tests carry `#[ignore = "requires Docker for PostgreSQL testcontainer"]`. The `lifecycle::` tests run without Docker.
- **Time:** deadline tests use FakeClock together with paused tokio time. Every join after a time advance is bounded by `join_within` (1 s of paused time), so a partial implementation fails rather than hangs.

### `lifecycle::worker::tests` (no Docker): 24 red, 2 not red

| Test | TS | T | FN | Red reason |
|---|---|---|---|---|
| `happy_path_ends_incomplete_with_exact_components` | TS-47 | T-05, T-06 | FN-16..FN-18 | job stays `queued` |
| `no_path_ends_succeeded_or_recovery_required` | TS-47 | T-05, T-06 | FN-16..FN-19 | job stays `queued` |
| `adapter_error_fails_job_and_releases_allocation` | TS-48 | T-12 | FN-19 | not `failed` |
| `mark_written_failure_fails_job_persistence_error`, `registration_refused_fails_job_persistence_error` | TS-48 | T-12 | FN-19, FN-21, FN-23 | not `failed` |
| `budget_exceeded_fails_job_allocation_limit`, `total_budget_exhausted_fails_job_allocation_limit` | TS-48 | T-13 | FN-21 | not `failed` |
| `undispatchable_queued_job_fails_dispatch_failed`, `dispatch_failure_does_not_touch_a_running_job` | TS-48, TS-67 | T-12 | FN-19 | `Lost`, or worker never reached the adapter |
| `duplicate_dispatch_has_exactly_one_winner`, `simultaneous_claims_have_exactly_one_winner` | TS-67 | T-07 | FN-16 | no winner (`[Idle, Idle]`) |
| `deadline_exceeded_fails_job`, `deadline_is_not_enforced_before_it_is_due`, `deadline_is_the_time_remaining_until_deadline_at`, `deadline_already_elapsed_fails_without_capture` | TS-68 | T-12, T-13 | FN-20 | worker never reached the adapter, or not `failed` |
| `late_worker_does_not_overwrite_sweeper_result`, `sweeper_first_then_worker_timer_has_one_terminal_result`, `worker_timer_first_then_late_sweeper_loses` | TS-68 | T-12 | FN-20 | worker never reached the adapter |
| `policy_denial_at_claim_fails_job_instance_not_allowed`, `policy_unavailable_at_claim_fails_closed`, `claim_recheck_is_made_once_for_the_jobs_instance` | TS-05a | T-02 | FN-04, FN-16 | job not claimed or re-checked |
| `refused_terminal_write_retries_three_times_then_leaves_job_running`, `refused_terminal_write_stalls_on_every_path_and_keeps_the_allocation`, `transient_audit_refusal_recovers_within_the_retries` | unit-own (FD 6.6 terminal retry) | T-09 | FN-16..FN-20 | not `Stalled` or `Incomplete` |
| `late_worker_cannot_overwrite_a_terminal_job` | TS-67 | T-12 | FN-16..FN-20 | **not red (structural):** a no-op shell already leaves the terminal row unchanged. Build must keep it green. |
| `worker_config_defaults_equal_the_validated_bounds` | unit-own | T-13 | FN-01 | **not red (structural):** drift guard; the deadline and allocation TTL equal CFG-10 and CFG-05. |

### `lifecycle::fixture_capture::tests` (no Docker): 5 red

| Test | TS | T | FN | Red reason |
|---|---|---|---|---|
| `writes_one_fixed_size_fixture_under_the_context_key` | unit-own | T-05 | FN-17 | no write |
| `fixture_content_is_fixed_size_deterministic_ascii` | unit-own | T-05, T-09 | FN-17 | empty content |
| `reports_the_exact_contract_components` | TS-47 | T-05 | FN-17 | empty components |
| `storage_failure_is_a_capture_error`, `refused_write_is_a_capture_error` | unit-own | T-12 | FN-17, FN-19 | shell returns `Ok` |

### `infra::tests::worker` (Docker): 14 red, 1 not red

| Test | TS | T | FN | Red reason |
|---|---|---|---|---|
| `ts47_queued_job_runs_through_fixture_adapter_and_ends_incomplete` | TS-47 | T-05, T-06 | FN-16..FN-18 | `Idle` |
| `ts44b_job_events_are_complete_and_free_of_excluded_fields` | TS-44b | T-09 | FN-34 | `Idle` |
| `ts44b_job_events_use_the_system_actor` | TS-44b (decision C-1) | T-09 | FN-34 | `Idle` |
| `ts44b_incomplete_event_outcome_is_failure` | TS-44b (decision C-2) | T-09 | FN-34 | `Idle` |
| `ts44b_refused_running_audit_leaves_job_queued_and_nothing_runs`, `ts44b_refused_terminal_audit_leaves_job_running_without_unaudited_effect` | TS-44b | T-08, T-09 | FN-16, FN-18 | not `Err` or `Stalled` |
| `ts48_allocation_limit_fails_job_without_a_row_or_a_file`, `ts48_total_budget_exhausted_fails_job_allocation_limit` | TS-48 | T-13 | FN-21 | `Idle` |
| `ts48_registration_refused_fails_job_persistence_error`, `ts48_adapter_error_fails_job_and_releases_allocation`, `ts48_dispatch_failed_fails_queued_job` | TS-48 | T-12 | FN-19, FN-21 | `Idle` or `Lost` |
| `ts67_duplicate_dispatch_has_exactly_one_winner`, `ts67_simultaneous_dispatch_has_exactly_one_winner` | TS-67 | T-07 | FN-16 | no winner |
| `ts05a_denied_policy_at_claim_fails_job_without_running_it` | TS-05a | T-02 | FN-04, FN-16 | `Idle` |
| `ts67_late_worker_cannot_overwrite_a_terminal_job` | TS-67 | T-12 | FN-16..FN-20 | **not red (structural)**, for the same reason as the unit variant |

**Totals (Observed on Windows):**

| Run | Exit | Passed | Failed | Ignored or filtered |
|---|---|---|---|---|
| No Docker | 101 | 84 (82 earlier + 2 structural) | 29 (all new) | 120 ignored |
| `--ignored` (Docker, 92 s) | 101 | 106 (105 earlier + 1 structural) | 14 (all new) | 113 filtered |

That makes 46 new tests: 43 red and 3 structural.
