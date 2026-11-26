# GVR-05 Green verification report: WU-05

- **ID:** GVR-05 (WU-05)
- **Stage:** Develop / build-and-verify, unit WU-05 of WU-01..WU-15. Not the release-candidate gate.
- **Status:** WU-05 green on Windows 11, 2026-11-23, after a red-first deadline correction.
- **Inputs:** [red tests](../../red-tests/WU-05/failing-tests.md), [red evidence](../../red-tests/WU-05/red-evidence.md), [implementation](implementation.md), [ids.json](../../implementation-design/ids.json), [acceptance criteria](../../../analyze/problem-framing/acceptance-criteria.md), [quality requirements](../../../analyze/context-and-risk/quality-requirements.md), [threat model](../../../ideate/direction-selection/threat-model.md).
- **Logs:**
  - [green package tests](../evidence/green-wu05-cargo-test.log)
  - [green ignored DB tests](../evidence/green-wu05-cargo-test-ignored.log)
  - [focused deadline regression](../evidence/green-wu05-deadline-regression.log)
  - [workspace check](../evidence/green-wu05-cargo-check-workspace.log)
  - [crate Clippy](../evidence/green-wu05-cargo-clippy.log)
  - [workspace regression tests](../evidence/regression-wu05-cargo-test.log) and [ignored tests](../evidence/regression-wu05-cargo-test-ignored.log)
  - [format check](../evidence/green-wu05-cargo-fmt.log), [workspace Clippy](../evidence/regression-wu05-cargo-clippy.log)
  - [deadline red test before fix](../../red-tests/evidence/red-wu05-deadline-budget.log)

Provenance:
- **Observed:** conductor runs on 2026-11-23; Windows 11, cargo 1.100.0, Docker available, PostgreSQL test container `postgres:15-alpine`.
- **Independent review:** after final deadline fix, an Opus review found no concrete defects in the implementation.
- **IDs:** `dtech-aidlc ids check` passed with 0 unregistered ids; 39 uncovered FN identifiers remain an advisory from the CLI.

## 1. Red then green

- **Existing WU-05 red suite:** the WU-05 red tests are unchanged from commit `53a9570`; the only test addition is `deadline_budget_excludes_time_spent_registering_allocation` plus its test-only delayed-registration fixture.
- **Deadline red:** with a 300-second deterministic registration delay applied to FakeClock and paused Tokio time, the new test failed before the fix at the assertion “capture must time out at the actual deadline, not five minutes later.” The failure reproduces the stale 600-second capture timeout that was calculated before registration.
- **Deadline green:** the same test passes after recomputing `deadline_at - clock.now()` after registration completes. It verifies `DEADLINE_EXCEEDED` at the absolute deadline, no components recorded, and allocation released.
- **Package suite:** `cargo test -p domain_backup --no-fail-fast` -> 114 passed, 0 failed, 120 ignored. `cargo test -p domain_backup --no-fail-fast -- --ignored` -> 120 passed, 0 failed.
- **No skipped tests:** existing ignored DB tests were explicitly run; no new test was ignored or weakened.

## 2. Results by test level

| Level | Result |
|---|---|
| Unit / filesystem, no Docker | 114 passed, 0 failed. This includes WU-05 worker and fixture adapter tests plus earlier units. |
| PostgreSQL integration (`postgres:15-alpine`) | 120 ignored tests passed when run with `-- --ignored`; no failures. |
| Deadline / deterministic time | `deadline_budget_excludes_time_spent_registering_allocation` passed with paused Tokio time and FakeClock advancing together. Existing remaining-deadline, already-elapsed and timer/sweeper ordering tests also passed. |
| Concurrency | Duplicate queued claims and simultaneous in-memory claims passed; terminal CAS preserves a single terminal result. |
| Static | `cargo check --workspace`, `cargo fmt -- --check`, `cargo clippy -p domain_backup --all-targets`, `cargo clippy --all-targets`: exit 0. Clippy retains existing warnings outside `domain_backup`. |
| IDs | `dtech-aidlc ids check`: `ok`, 0 unregistered. |
| Platform-specific security | Windows environment; Unix-only file permission tests remain unverified here and are carried from WU-04 to Ubuntu CI before the release-candidate gate. |
| E2E / performance | Not run; not part of WU-05. |

## 3. Regression versus baseline

The recorded baseline is [baseline.md](../../baseline.md): workspace Rust suite 290 passed, 0 failed, 39 ignored; all 39 ignored tests pass with Docker. After WU-05:

- Workspace `cargo test --no-fail-fast`: 367 passed, 0 failed, 125 ignored.
- Workspace `cargo test --no-fail-fast -- --ignored`: 125 passed, 0 failed.
- The workspace count increases by 47 pass/ignored counts versus the WU-04 result (335 passed + 110 ignored = 445, versus 367 passed + 125 ignored = 492). This is the observed test-count change; no previously passing test regressed.
- Workspace check, format and Clippy all exit 0. No `domain_backup` Clippy warning.
- Admin web was not rerun; this WU changes no web files.
- See the full command output in [regression report](regression-report.md) and linked logs.

## 4. Mutation proof

Earlier WU-05 mutation evidence is recorded in [red evidence](../../red-tests/WU-05/red-evidence.md) for guards that are contractually distinct from the deadline correction. For the rework:

| Guard | Mutation | Test | Result |
|---|---|---|---|
| Deadline budget excludes allocation registration | Reuse the stale pre-registration remaining duration | `deadline_budget_excludes_time_spent_registering_allocation` | **Caught**: test failed at the expected absolute-deadline assertion. The mutation was restored and source bytes verified. |
| Register-before-write | Cause capture to run on a registration-refusal path | `registration_refused_fails_job_persistence_error`; happy-path call order assertion | **Caught** in prior mutation run; details in [red evidence](../../red-tests/WU-05/red-evidence.md) and conductor mutation log if present. |

The final post-mutation source is restored and matches the implementation that passed the tests. No tests or environment settings were changed by mutation.

## 5. Quality requirements and trace

- **AC-002 S1/S2/S3:** store claim, running state, bounded deadline, safe failure or incomplete outcome, fixed component result, never success: passed at worker/store levels.
- **AC-005 S1:** claim-time resource re-check through policy port; denial fails closed: passed with policy test double and DB test.
- **AC-005 S2:** job transition audit events are transactionally coupled through WU-03 store; exact fields and excluded-content checks: passed in PostgreSQL tests. Actor NULL and `job.incomplete` outcome `failure` follow the WU-05 red evidence C-1/C-2 decisions and WU-03 store mapping.
- **AC-006 S2:** Q-05 deadline, staging expiry and allocation budget values remain at approved defaults; deadline and expiry tests passed.

| Story | Criterion | Function | Test(s) | Result |
|---|---|---|---|---|
| US-02, US-06 | AC-002 S1/S2/S3 | FN-16 | `duplicate_dispatch_has_exactly_one_winner`, `simultaneous_claims_have_exactly_one_winner`, DB `ts67_*`; workspace store tests | passed |
| US-02, US-06 | AC-002 S2/S3 | FN-17 | five `fixture_capture::tests`; DB `ts47_queued_job_runs_through_fixture_adapter_and_ends_incomplete` | passed |
| US-02, US-06 | AC-002 S2/S3 | FN-18 | `happy_path_ends_incomplete_with_exact_components`, `no_path_ends_succeeded_or_recovery_required`, DB TS-47 | passed |
| US-02, US-06 | AC-002 S2 | FN-19 | TS-48 worker failure tests for adapter, persistence, allocation, dispatch; deadline tests | passed |
| US-02, US-06 | AC-002 S2 | FN-20 | `deadline_exceeded_fails_job`, `deadline_is_the_time_remaining_until_deadline_at`, `deadline_budget_excludes_time_spent_registering_allocation`, sweeper/timer tests | passed |
| US-05, US-06 | AC-005 S1 | FN-04 | `policy_denial_at_claim_fails_job_instance_not_allowed`, `policy_unavailable_at_claim_fails_closed`, DB `ts05a_denied_policy_at_claim_fails_job_without_running_it` | passed |
| US-05, US-06 | AC-005 S2 | FN-34 | `ts44b_job_events_are_complete_and_free_of_excluded_fields`, system actor and incomplete-outcome decision tests | passed |
| US-02, US-06 | AC-006 S2 | FN-01, FN-21 | `worker_config_defaults_equal_the_validated_bounds`, WU-04 budget and WU-05 deadline/expiry tests | passed |

## 6. Scope and codebase map

- Production WU-05 changes are in `lifecycle/worker.rs` and `lifecycle/fixture_capture.rs`; test-only delay support and the new regression are in the lifecycle test module/support.
- No dependencies, entry points, commands, or components in the codebase map changed; no map update required.
- Real-data capture, gateway composition, runtime orchestration, cleanup/reconciliation and web UI remain out of WU-05 scope.

## 7. Verification refresh (Observed, 2026-11-24)

Environment: Docker prerequisite passed (`docker info`, exit 0). No red replay was performed: the historical WU-05 red and the red-first deadline regression remain preserved above; current scoped tests were green.

| Command (from `apps/api`, unless noted) | Observed result | Raw log |
|---|---|---|
| `cargo test -p domain_backup lifecycle::worker::tests --no-fail-fast` | 27 passed, 0 failed | [refresh-wu05-focused-worker.log](../evidence/refresh-wu05-focused-worker.log) |
| `cargo test -p domain_backup lifecycle::fixture_capture::tests --no-fail-fast` | 5 passed, 0 failed | [refresh-wu05-focused-fixture.log](../evidence/refresh-wu05-focused-fixture.log) |
| `cargo test -p domain_backup infra::tests::worker:: --no-fail-fast -- --ignored` | 15 passed, 0 failed (Docker/PostgreSQL) | [refresh-wu05-focused-worker-docker.log](../evidence/refresh-wu05-focused-worker-docker.log) |
| `cargo fmt -- --check` | exit 0 | [refresh-wu05-fmt.log](../evidence/refresh-wu05-fmt.log) |
| `cargo clippy --all-targets` | exit 0; existing warnings remain | [refresh-wu05-clippy.log](../evidence/refresh-wu05-clippy.log) |
| `dtech-aidlc ids check` | `ok`; 0 unregistered identifiers; 35 uncovered functions are advisory | [refresh-wu05-ids-check.log](../evidence/refresh-wu05-ids-check.log) |

The full-workspace normal and ignored commands did **not** pass because they included in-progress WU-16 lifecycle tests and WU-06 reconciliation-store tests. All failures are newly added tests, not failures among baseline-passing tests; exact names and comparison are in the refreshed regression report. WU-05's focused worker, fixture and Docker worker tests passed. This broader failure status is not represented as a green workspace run.
