# RE-19 Red evidence: WU-19

- **ID:** RE-19 (WU-19)
- **Stage:** Develop / red-tests (builder-verifier), unit WU-19.
- **Recorded:** 2026-11-26, Windows 11, cargo 1.100.0, Docker for PostgreSQL 15.
- **Manifest:** [failing-tests.md](failing-tests.md)

## Runs (Observed)

| Command (from `apps/api`) | Exit | Result | Log |
|---|---|---|---|
| `cargo test -p domain_backup infra::tests::sweeper -- --ignored` | 101 | 10 run: 10 failed | `build-and-verify/evidence/red-wu19-sweeper-db-ignored.log` |
| `cargo test -p domain_backup infra::tests::runtime -- --ignored` | 101 | 9 run: 9 failed | `build-and-verify/evidence/red-wu19-runtime-db-ignored.log` |
| `cargo test -p domain_backup infra::tests::reconciliation::ts30 -- --ignored` | 101 | 1 run: 1 failed (`ts30_deadline_inventory_includes_only_due_jobs_at_epoch`, the inert `list_deadline_expired` default) | `build-and-verify/evidence/red-wu19-jobstore-db-ignored.log` |
| `cargo test -p domain_backup` | 0 | 132 passed, 0 failed, 195 ignored. The five WU-19 shell tests are gone, so the normal run has no failure. | `build-and-verify/evidence/red-wu19-cargo-test.log` |

The code compiled before every run. The failing tests fail on assertions about returned results or durable state, not on setup, compile or import errors.

## Why each failure is behavioral

The shell does nothing: `sweep` returns an empty report, `start` returns `Ok` without opening admissions, `run_tick` returns an empty `TickReport`.

| Failure | First failing assertion |
|---|---|
| `ts30_a_running_job_fails...`, `the_sweep_ignores...`, `ts30_a_queued_job...`, `ts28...`, `ts30_a_written_allocation...`, `the_sweep_discovers...` | the report count is 0 where 1 or 2 was required |
| `ts42_...` | `purged` 0 where 1 was required |
| `ts65_a_refused_purge_audit...`, `ts29_*`, `a_runtime_that_was_never_started...`, `ts32_a_second_runtime...`, `ts56_*`, `a_truncated_scan...` | `Ok(..)` where an `Err(..)` (`AuditUnavailable`, `StoreUnavailable`, `LockUnavailable`, `InventoryTruncated`, `NotStarted`) was required |
| `one_sweep_runs...` | the report is all zeros, expected one count per step |
| `ts32_startup_runs...` | the call log is empty: no epoch, inventory, fence or cleanup call happened (reported by the order helper) |
| `ts30_the_cleaner_ticks...` | `cleaned` is `None` where `Some([Deleted])` was required |
| `ts29_recovery...`, `ts29_a_store_outage...`, `ts29_a_truncated_recovery...` | the outage step expected an error from `run_tick` and got `Ok` |

No test passes against the shell.

## Decisions taken in the tests (the design is silent on them)

- The queued-age bound is 600 s (FD 7.1: equal to CFG-10) and the boundary is inclusive, like the deadline: a queued job whose age is exactly 600 s fails `DISPATCH_FAILED`.
- The orphan-grace promotion step of FD 6.7 (step 3) has no code and no test: `claim_for_delete` and `due_for_cleanup` already treat an `orphan_discovered` row past its grace as claimable (WU-06), and the cleaner tests (WU-18) cover the boundary.
- The sweeper releases an expired `written` allocation through the audited `reconcile_state` CAS, so the release writes `allocation.reconciled` like every other allocation reconciliation.
- A sweep fails at its first error and returns it; effects of earlier steps are durable (deadline enforcement is not undone when the scan is truncated).
- The runtime is not wired to a timer: `run_tick` is one scheduler iteration driven by the injected clock; the periodic driver belongs to the gateway composition (WU-10). A failed sweep does not move the "last sweep" time, so the next tick retries it.
- Outage rule: a store or storage failure closes admissions at once. A rejected audit (healthy connection) is counted; the third consecutive refusal closes admissions and degrades the runtime (FD 6.7). A truncated sweep scan sets a degraded signal and leaves admissions as they are.
- The namespace lock port has an associated function, not a method, so the runtime receives it as an injected function (`LockAcquirer`); production passes `FsNamespaceLock::try_acquire`.
- The sweeper takes the reconciler (not a storage and an audit sink) for its discover-only scan, so the scan has one implementation (WU-17).
- The race of TS-28 is tested sequentially (running owner, then terminal owner) rather than with a thread barrier.

## Not covered by this red run

- A real timer loop and shutdown (WU-10 composition).
- A hard database kill (container stop) in the middle of a sweep: the outage is injected at the port wrappers, as the test strategy allows.
- Platform: Windows only.
