# IM-17 Implementation: WU-17 lifecycle reconciler

- **ID:** IM-17 (WU-17)
- **Stage:** Develop / build-and-verify (builder-verifier), unit WU-17.
- **Status:** implemented and verified 2026-11-26. Red evidence: [../../red-tests/WU-17/red-evidence.md](../../red-tests/WU-17/red-evidence.md).

## Change

Under `apps/api/domain_backup/src/`.

| File | Change |
|---|---|
| `lifecycle/reconciler.rs` | `Reconciler` implemented: `advance_epoch`, `reconcile(epoch, now)`, `discover_orphans(now)`. Receives `Arc<dyn JobStore>`, `Arc<dyn AllocationStore>`, `Arc<dyn FixtureStorage>`, `Arc<dyn AuditSink>` and a `ReconcilerConfig` (instance id, inventory cap). It constructs no concrete adapter. |
| `infra/db_job_store.rs` | `DbJobStore` now implements three `JobStore` methods that were inert defaults: `advance_epoch` (one `INSERT ... ON CONFLICT DO UPDATE ... RETURNING`), `list_for_reconciliation` (complete, unfiltered, errors never become an empty list) and `fence_job` (CAS on id, epoch, status and version; the `reconcile.fenced` audit row commits in the same transaction; `Lost` writes nothing). `list_deadline_expired` stays inert: it belongs to WU-19. |
| `lifecycle/lifecycle_tests.rs` | the seven WU-17 shell tests removed (replaced by the PostgreSQL suite). |
| `infra/tests/reconciler.rs`, `infra/tests/mod.rs` | 20 Docker-backed tests. |

## Behavior of `reconcile(epoch, now)`

1. Read the bounded namespace listing first. A truncated listing returns `InventoryTruncated` before any other read or write.
2. Read the complete job inventory and fence every job of an older epoch by CAS: `running` to `failed INTERRUPTED_RESTART`, `queued` to `failed ABANDONED_QUEUED`. A lost CAS changes nothing and writes nothing.
3. Read the allocation inventory again, so owners fenced in step 2 read as `failed`, and apply the 6.1a table by conditional allocation CAS (each applied CAS writes `allocation.reconciled` in its own transaction):
   - `registered`, owner not running: `release_pending`.
   - `written`, owner not running, and the owner is failed or absent or the 24 h expiry is reached: `release_pending`.
   - `written`, unexpired, owner `incomplete`, file absent: no state change; one `reconcile.missing_file` audit row per full scan; the in-memory count goes up by one.
   - `deleting` of an older epoch: `release_pending`, attempts unchanged.
   - everything else (running owner of any epoch, `cleanup_failed`, `orphan_discovered`, `release_pending`, `deleted`): untouched.
4. Classify every namespace entry against the ledger with `classify_entry`: an unowned valid name is persisted as `orphan_discovered` (grace comes from the allocation store, 1 h), an ambiguous entry raises an `ownership.ambiguous` audit row and is never touched.

`discover_orphans(now)` runs only the orphan part of step 4, for the periodic sweep. It never fences, releases or raises incidents.

The reconciler never unlinks a file. The audited claim, unlink and audited delete belong to the cleaner (WU-18).

## Failure mapping

Store and allocation errors map to `StoreUnavailable`, or to `AuditUnavailable` when the store reports a rejected audit. A rejected audit write maps to `AuditUnavailable`, an audit outage to `StoreUnavailable`. Storage errors map to `StorageUnavailable`. Any error aborts the run, so the caller keeps admissions closed.

## Identifiers

No identifier was minted. `dtech-aidlc ids check` is clean for this unit.
