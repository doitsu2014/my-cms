# RE-18 Red evidence: WU-18

- **ID:** RE-18 (WU-18)
- **Stage:** Develop / red-tests (builder-verifier), unit WU-18.
- **Recorded:** 2026-11-26, Windows 11, cargo 1.100.0, Docker for PostgreSQL 15.
- **Manifest:** [failing-tests.md](failing-tests.md)

## Runs (Observed)

| Command (from `apps/api`) | Exit | Result | Log |
|---|---|---|---|
| `cargo test -p domain_backup infra::tests::cleaner -- --ignored` | 101 | 12 run: 12 failed, 0 passed | `build-and-verify/evidence/red-wu18-cleaner-db-ignored.log` |
| `cargo test -p domain_backup` | 101 | 133 passed, 4 failed, 175 ignored. The 4 failures are the WU-19 shell tests (`ts29`, `ts32`, `ts42`, `ts65_audit_refusal_blocks_purge`). None belongs to WU-18. | `build-and-verify/evidence/red-wu18-cargo-test.log` |

The code compiled before every run. The failing tests fail on assertions about returned outcomes or durable state, not on setup, compile or import errors.

## Why each failure is behavioral

The shell returns `NotDue` from `clean_one` and an empty list from `clean_due`, so each test fails at its first behavioral assertion:

| Failure | First failing assertion |
|---|---|
| `ts26_failed_deleted_commit...`, `ts65_a_refused_claim_audit...` | `Ok(NotDue)` where `Err(AuditUnavailable)` was required |
| `ts26_a_missing_file...`, `ts30_retry_is_due...` | `Ok(NotDue)` where `Ok(Deleted)` was required |
| `ts27a_three_unlink_failures...`, `ts27a_a_retry_that_succeeds...` | `Ok([])` / `Ok(NotDue)` where `RetryScheduled` was required |
| `ts20_full_sequence...`, `ts21_full_sequence...`, `ts25_stale_deleting...`, `ts23_orphan...`, `clean_due_handles_due_rows...` | `Ok([])` where `Ok([Deleted])` was required |
| `rows_that_are_not_claimable...` | `Ok(NotDue)` where `Ok(Lost)` was required |

Part of three tests would pass against the shell (the "not due yet" half of `ts30_retry_is_due...` and `ts23_orphan...`, and the untouched row in `clean_due_handles_due_rows...`); each test also contains an assertion that the shell fails, so none is vacuous as a whole. The green stage shows with mutation checks that the "not due" halves can fail.

## Decisions taken in the tests (the design is silent on them)

- An unlink failure is recorded with the job-level reason code `ADAPTER_ERROR` (the closed set has no storage-specific code). It lands in `last_reason_code` and on the `cleanup.exhausted` row.
- `clean_one` returns `NotDue` for a `release_pending` row before its `next_attempt_at` and for an `orphan_discovered` row before its grace, and `Lost` for any other state or an unknown id. Neither case claims, unlinks or audits.
- The cleaner takes no `AuditSink` and no `Clock`. The design text lists both, but the allocation store already writes every cleaner audit row in the transaction of the state change it records, and `now` is passed in. Injecting an unused sink would only invite a second, non-atomic write.
- The restart in TS-26 is the real one: `advance_epoch`, then the WU-17 reconciler, then the cleaner.
- `clean_due` stops at the first error and returns it; earlier outcomes in the batch are durable but not reported.

## Not covered by this red run

- The sweeper's 24 h expiry, 1 h grace and 15 min sweep boundaries (TS-30) and the purge half of TS-65: WU-19.
- A database outage in the middle of a claim: covered by the store-level tests only.
- Platform: Windows only.
