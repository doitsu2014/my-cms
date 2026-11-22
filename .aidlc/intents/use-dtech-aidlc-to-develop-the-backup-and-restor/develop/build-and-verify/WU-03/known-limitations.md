# KL-03 Known limitations: WU-03

- **ID:** KL-03 (WU-03)
- **Stage:** Develop / build-and-verify, unit WU-03.
- **Status:** Open items, carried to later units or to the release-candidate gate. Earlier items are in [../WU-02/known-limitations.md](../WU-02/known-limitations.md) (its L-01 and L-02 are now enforced by WU-03 tests) and [../WU-01/known-limitations.md](../WU-01/known-limitations.md).

| # | Limitation | Impact | Owner or next step |
|---|---|---|---|
| L-01 | `transition` allows queued to running, but sets only `started_at`. It does not set `deadline_at` or `epoch`. The red tests pin this edge as legal through `transition`. | A caller that dispatches through `transition` instead of `claim_next` would leave a running job with no deadline. | WU-05: the worker dispatches only through `claim_next` (FN-16). The conductor or owner decides whether a test should forbid queued to running through `transition`. |
| L-02 | `transition` writes whatever `reason` it is given. It does not check the reason against the edge's FD 7.1 / IFC 10.1 code. | A terminal job could be stored with no reason or the wrong one. | WU-05/WU-06 callers pass the edge's code. Enforcing it in the store would need a test change. |
| L-03 | `IdemRecord.key_digest` and `request_fingerprint` are `String`s at the port, not the `Digest` type. `AuditEvent.reason_code` is `Option<String>`. | Type safety relies on the runtime check in `accept`, which is tested. | Retyping changes the public types the tests use. Do it in WU-07 together with the admission handler. |
| L-04 | If the connection drops during COMMIT, the outcome is ambiguous: the server may have committed, but the store returns `StoreUnavailable`. | A caller could retry a transition that already applied. The CAS makes the retry `Lost`, so no double effect occurs, but the caller sees an error. | WU-05/WU-10: treat `StoreUnavailable` after a write as "unknown, re-read". |
| L-05 | No `lock_timeout` or statement timeout is set on store transactions. | A claim or transition can wait on a row lock held by another long transaction. | WU-06/WU-10: decide timeouts with the outage predicate (FD 6.6). |
| L-06 | The workspace builds `sea-orm` with the `debug-print` feature, which logs every statement with its bind values at debug level. This predates WU-03. | At debug log level, actor ids, instance ids and digests appear in logs. No raw keys or contents are logged. | Workspace or gateway decision, outside U1 scope. Raise before Launch (T-09). |
| L-07 | The claim's `FOR UPDATE SKIP LOCKED` is an implementation choice with no test. Removing `SKIP LOCKED`, or the row lock, keeps every test green. | Without it a claim could wait behind another transaction instead of skipping. Double claims stay impossible because of the outer status recheck, which is tested. | Accepted. FN-16 does not require non-blocking claims. |
| L-08 | The outer `status = 'queued'` filter in the claim is redundant with the inner filter plus the lock. | none | Accepted as defence in depth |
| L-09 | Two tests are test-after, not red-first: the `accept` digest check and the SQLSTATE allow-list, both hardening added during build review. | They have no classic red evidence. | Each is proven by mutation (green report section 3) |
| L-10 | Most mutation rows were run by a subagent, and one agent result was wrong (claim atomicity). | Agent-reported mutation results are not fully trustworthy. | The conductor re-ran the disputed guard and the test-after guards, with logs. The other agent rows are labelled "agent". |
| L-11 | `JobStore` omits `list` and `fence_older_epochs`, and `JobRow` omits `counters`. | none yet | WU-08 (`list`), WU-06 (`fence_older_epochs`), and the unit that needs `counters` |
| L-12 | Not composed into the gateway | WU-03 changes nothing in the running API | WU-10 |

## Rollback

The store and sink are not referenced outside `domain_backup`, and the gateway does not compose the crate. To roll back, revert the WU-03 files. No schema change was made in WU-03.
