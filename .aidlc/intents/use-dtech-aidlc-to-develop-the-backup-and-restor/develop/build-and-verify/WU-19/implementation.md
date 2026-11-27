# IM-19 Implementation: WU-19 lifecycle sweeper and runtime

- **ID:** IM-19 (WU-19)
- **Stage:** Develop / build-and-verify, unit WU-19.
- **Status:** implemented and verified 2026-11-26; full normal and Docker-backed workspace regressions pass after the final fix.
- **Red evidence:** [../../red-tests/WU-19/red-evidence.md](../../red-tests/WU-19/red-evidence.md).

## Change

Under `apps/api/domain_backup/src/`:

| File | Change |
|---|---|
| `lifecycle/sweeper.rs` | Implemented `Sweeper` and `SweepReport`: current-epoch deadline enforcement, inclusive queued-age bound, terminal/expired written allocation release, retention purge through `RetentionStore`, and discover-only orphan scan. Added `terminal_released` to distinguish cleanup after terminal job failure from expiry counts. |
| `lifecycle/runtime.rs` | Implemented startup lock/epoch/reconcile/cleanup barrier, outage closure, recovery ordering, runtime health state, and clock-driven `run_tick` intervals. |
| `infra/db_job_store.rs` | Implemented `list_deadline_expired(now, epoch)` with current-epoch running filter and inclusive deadline comparison. |
| `ports/job_store.rs` | Named unused arguments in default `fence_job` and annotated its intentionally wide port signature. |
| `infra/tests/sweeper.rs` | Added durable regression coverage that a deadline-failed job releases an unexpired written allocation in the same sweep. |
| `infra/tests/runtime.rs` | 3 additional tests cover refused reconcile audit during recovery, consecutive-audit-refusal reset after a healthy sweep, and truncated sweep behavior that degrades without closing admissions. |
| `lifecycle/lifecycle_tests.rs`, `lifecycle/mod.rs` | Removed obsolete shell tests for replaced lifecycle behavior; Docker-backed tests assert persisted DB, audit and filesystem state. |

## Runtime and sweep behavior

- Startup holds the namespace lock before advancing the durable epoch, reconciles/fences, performs due cleanup, then opens admissions. Any failure leaves the runtime `NotReady`, admissions closed, and the acquired lock released.
- Database recovery keeps the epoch, then reconciles, cleans, sweeps, and reopens only after all steps succeed.
- `run_tick` performs cleaner work every 60 seconds and sweeps every 900 seconds, with inclusive due boundaries. A closed runtime performs no store work. Store/storage failures close admissions; the third consecutive refused sweep audit closes admissions; successful sweeps reset that counter. Truncated inventory sets degraded state without itself closing admissions.
- One sweep performs deadline enforcement and queued-age expiry, allocation release, retention purge, then bounded discover-only scan, failing at the first error. Earlier successful effects remain durable.
- Written allocations are not released merely because their expiry passed while the owner is running. After a job is terminal, its allocation is eligible for release immediately even if the 24-hour expiry has not elapsed; this implements the design's `failed job -> release_pending` invariant and prevents a deadline-failed job from retaining capacity until expiry.
- The sweeper does not unlink files. Release is audited through allocation reconciliation; the cleaner performs the audited claim and unlink.

## Identifiers

Cited only registered IDs: FN-20, FN-31, FN-32, FN-33; TS-28, TS-29, TS-30, TS-32, TS-42, TS-65. The explicit ID check returned `ok: true` for build-and-verify.