# IMPL-05 Implementation manifest: WU-05

- **ID:** IMPL-05 (WU-05)
- **Stage:** Develop / build-and-verify, unit WU-05 of WU-01..WU-15.
- **Status:** Implemented and verified on Windows 11 with PostgreSQL 15 via Docker, 2026-11-23. Branch-only until Launch; no merge or deployment.
- **Inputs:** [WU-05 red manifest](../../red-tests/WU-05/failing-tests.md), [WU-05 red evidence](../../red-tests/WU-05/red-evidence.md), [implementation plan](../../implementation-design/implementation-plan.md), [interfaces and contracts](../../implementation-design/interfaces-and-contracts.md), [functional design](../../implementation-design/functional-design.md), [test strategy](../../implementation-design/test-strategy.md), [ids.json](../../implementation-design/ids.json).
- **Evidence:** [green verification](green-verification-report.md), [regression report](regression-report.md), [known limitations](known-limitations.md), raw logs in `../evidence/`.

## Files changed

| Path under `apps/api/domain_backup/src/` | Change |
|---|---|
| `lifecycle/worker.rs` | Implement `run_next`, CAS-checked `execute`, policy re-check, allocation registration/cleanup, capture timeout, terminal transitions and bounded retries. Recompute the remaining capture deadline after awaited allocation registration, and fail/release without capture if no time remains. |
| `lifecycle/fixture_capture.rs` | Write only the deterministic fixed-size synthetic fixture and return the contract components. Storage failures map to a safe `CaptureError::Failed`. |
| `lifecycle/test_support.rs` | Test-only delayed-allocation-registration hook advances FakeClock and paused Tokio time for TS-68 regression coverage. |
| `lifecycle/worker.rs` tests | Added `deadline_budget_excludes_time_spent_registering_allocation`, a red-first regression test (TS-68). |

No other test files or WU-01..WU-04 code changed. `Cargo.toml` and `Cargo.lock` are unchanged from the WU-05 red commit. No environment changes, real-data capture, deployment or external action.

## Behavior

- `run_next` uses `JobStore::claim_next` to claim queued work; no claim gives `Idle`.
- `execute` re-reads the job and verifies it remains `running` at the claimed version, then re-checks the resource through `ClaimPolicy`. Denial or policy unavailability fails closed as `INSTANCE_NOT_ALLOWED`.
- A missing deadline fails with `PERSISTENCE_ERROR`; an elapsed deadline fails with `DEADLINE_EXCEEDED`.
- The worker registers the allocation before invoking the capture adapter. Allocation-budget failures map to `ALLOCATION_LIMIT`; other registration failures map to `PERSISTENCE_ERROR`.
- The capture timeout is computed from `deadline_at - clock.now()` **after registration completes**, so ledger/audit latency consumes the job's deadline budget. If registration consumed all remaining time, the worker records `DEADLINE_EXCEEDED`, releases the allocation and does not invoke capture.
- The only capture implementation is `FixtureCaptureAdapter`. It writes exactly 4096 deterministic ASCII bytes labelled synthetic, and reports the exact three components: database `synthetic_captured` (`DB_SYNTHETIC_ONLY`), auth unavailable (`AUTH_NOT_CAPTURED`), storage unavailable (`STORAGE_NOT_CAPTURED`). No real data or secret is read.
- After capture, `mark_written` is conditional. Failure maps to `PERSISTENCE_ERROR` and attempts release. Success transitions the job to `incomplete` with `REQUIRED_COMPONENTS_UNAVAILABLE`; the written allocation remains until expiry.
- Adapter failure, deadline timeout, policy denial, allocation refusal and dispatch failure use closed reason codes. U1 jobs never transition to `succeeded` or `recovery-required`.
- Terminal job transitions retry one initial attempt plus three retries at one-second intervals on audit/store unavailable. Exhaustion returns `Stalled`; a running job and its allocation remain for the WU-06 sweeper.

## Red-first deadline fix

- New test: `lifecycle::worker::tests::deadline_budget_excludes_time_spent_registering_allocation`.
- Before the implementation fix, targeted test failed with the assertion “capture must time out at the actual deadline, not five minutes later” when the registration double advanced both clocks by 300 seconds.
- The minimal fix recalculates remaining time after `register().await`. The same targeted test now passes, verifies the worker fails at the 600-second absolute deadline, and checks that the allocation enters `release_pending`.

## Verification summary

- `cargo test -p domain_backup --no-fail-fast`: 114 passed, 0 failed, 120 ignored.
- `cargo test -p domain_backup --no-fail-fast -- --ignored`: 120 passed, 0 failed.
- Full workspace: 367 passed, 0 failed, 125 ignored; all 125 ignored tests passed in the `-- --ignored` run.
- Workspace `cargo check --workspace`, `cargo fmt -- --check`, crate `cargo clippy -p domain_backup --all-targets`, workspace `cargo clippy --all-targets`: exit 0.
- `dtech-aidlc ids check`: `ok`, 0 unregistered identifiers.
- Independent WU-05 review after the deadline correction: no concrete defects found.
- Detailed baseline comparison and mutation-proof evidence are in the linked verification and regression reports. The new timeout regression was additionally proven red before the correction and green after it.

## Verification refresh (Observed, 2026-11-24)

- Reviewed current WU-05 source deltas. Worker, fixture adapter, test-only delayed-registration helper and deadline regression test match the behavior already recorded above; no additional unrecorded WU-05-specific behavior or test change was found. Separate in-progress WU-06/WU-16 files are present in the working tree and are outside this unit.
- No source or test files were changed during this refresh. The baseline was reused from [baseline.md](../../baseline.md), not rerun or rewritten.
- Refresh command logs are under `../evidence/refresh-wu05-*.log`; the refreshed pass/failure set and exact failures are recorded in [regression-report.md](regression-report.md).
