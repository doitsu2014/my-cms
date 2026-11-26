# KL-03 Known limitations: WU-03

- **ID:** KL-03 (WU-03)
- **Stage:** Develop / build-and-verify, unit WU-03.
- **Current status (refresh 2026-11-24):** WU-03 focused store and audit tests pass. Current full workspace verification is blocked by later-unit failures in WU-06 and WU-16. See [regression-report.md](regression-report.md) and [green-verification-report.md](green-verification-report.md). Historical WU-03 notes from 2026-11-22 are preserved in section 2.

## Current verification limitations and blockers

| # | Limitation | Impact | Owner / next step |
|---|---|---|---|
| R-01 | Full workspace normal suite exited 101 with 13 failures in the in-memory lifecycle test module, all assigned to WU-16. | Current workspace normal suite is not green; cannot recommend complete stage report. These are not WU-03 focused test failures and were not recorded in the baseline. | WU-16 owner must diagnose and resolve its failures using its own test-first process, then rerun full normal and ignored workspace suites. This WU-03 refresh made no source/test changes. |
| R-02 | Full workspace ignored/Docker suite exited 101 with 7 `infra::tests::reconciliation::*` failures assigned to WU-06. | Current DB-backed workspace suite is not green; cannot recommend complete stage report. These are not WU-03 focused test failures and were not recorded in the baseline. | WU-06 owner must diagnose and resolve its failures using its own test-first process, then rerun full suites. No WU-06 implementation change was made in this refresh. |
| R-03 | `cargo clippy --workspace --all-targets` exited 0 but emitted warnings, including in `domain_backup` for unused stub-method parameters and missing `Debug`/`Default` implementations on lifecycle types. | Lint is command-successful but not warning-free. Warnings are not treated as test failures and were not changed during this refresh. | Track with owning work units; check touched code before their completion. |
| R-04 | Identifiers check reports `ok: true` and no unregistered identifiers, but separately lists uncovered functions across the in-progress workflow. | No WU-03 registry error, but workflow-wide coverage inventory is incomplete because later units remain in progress. | Continue coverage in assigned later units; do not claim whole-intent coverage from this stage check. |

## WU-03 limitations carried forward from the original implementation (2026-11-22)

| # | Limitation | Impact | Owner or next step |
|---|---|---|---|
| L-01 | `transition` allows queued to running, but sets only `started_at`. It does not set `deadline_at` or `epoch`. The red tests pin this edge as legal through `transition`. | A caller that dispatches through `transition` instead of `claim_next` would leave a running job with no deadline. | WU-05: worker dispatches only through `claim_next` (FN-16). The conductor or owner decides whether a test should forbid queued to running through `transition`. |
| L-02 | `transition` writes whatever `reason` it is given. It does not check the reason against the edge's FD 7.1 / IFC 10.1 code. | A terminal job could be stored with no reason or the wrong one. | WU-05/WU-06 callers pass the edge's code. Enforcing it in the store would need a test change. |
| L-03 | `IdemRecord.key_digest` and `request_fingerprint` are `String`s at the port, not the `Digest` type. `AuditEvent.reason_code` is `Option<String>`. | Type safety relies on the runtime check in `accept`, which is covered by a test. | Retyping changes public types the tests use; consider in WU-07 with the admission handler. |
| L-04 | If the connection drops during COMMIT, the outcome is ambiguous: the server may have committed, but the store returns `StoreUnavailable`. | A caller could retry a transition that already applied. The CAS makes the retry `Lost`, so no double effect occurs, but the caller sees an error. | WU-05/WU-10: treat `StoreUnavailable` after a write as unknown and re-read. |
| L-05 | No `lock_timeout` or statement timeout is set on store transactions. | A claim or transition can wait on a row lock held by another long transaction. | WU-06/WU-10: decide timeouts with the outage predicate (FD 6.6). |
| L-06 | The workspace builds `sea-orm` with the `debug-print` feature, which logs statements with bind values at debug level. This predates WU-03. | At debug log level, actor ids, instance ids and digests appear in logs. No raw keys or contents are logged. | Workspace or gateway decision, outside U1 scope. Raise before Launch (T-09). |
| L-07 | The claim's `FOR UPDATE SKIP LOCKED` is an implementation choice with no test. Removing `SKIP LOCKED`, or the row lock, keeps every test green. | Without it a claim could wait behind another transaction instead of skipping. Double claims stay impossible because the outer status recheck is tested. | Accepted; FN-16 does not require non-blocking claims. |
| L-08 | The outer `status = 'queued'` filter in the claim is redundant with the inner filter plus the lock. | None. | Accepted as defence in depth. |
| L-09 | Two tests are test-after, not red-first: the `accept` digest check and SQLSTATE allow-list, both hardening added during build review. | They have no classic red evidence. | Each was proven by mutation in the historical green report. |
| L-10 | Most historical mutation rows were run by a subagent, and one agent result was wrong (claim atomicity). | Agent-reported mutation results are not fully trustworthy. | The conductor re-ran the disputed guard and test-after guards, with logs; other agent rows are labelled in the historical report. |
| L-11 | `JobStore` omits `list` and `fence_older_epochs`, and `JobRow` omits `counters`. | None yet. | WU-08 (`list`), WU-06 (`fence_older_epochs`), and the unit that needs counters. |
| L-12 | Not composed into the gateway. | WU-03 changes nothing in the running API. | WU-10. |

## Rollback

The store and sink are not referenced outside `domain_backup`, and the gateway does not compose the crate. To roll back, revert the WU-03 files. No schema change was made in WU-03. The 2026-11-24 refresh changed only this unit's four declared evidence artifacts and added raw command logs under the existing evidence directory; it did not alter application source, tests, workflow state, or audit records.
