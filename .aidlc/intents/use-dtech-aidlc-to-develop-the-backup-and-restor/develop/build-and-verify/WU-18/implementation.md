# IM-18 Implementation: WU-18 lifecycle cleaner

- **ID:** IM-18 (WU-18)
- **Stage:** Develop / build-and-verify (builder-verifier), unit WU-18.
- **Status:** implemented and verified 2026-11-26. Red evidence: [../../red-tests/WU-18/red-evidence.md](../../red-tests/WU-18/red-evidence.md).

## Change

Under `apps/api/domain_backup/src/`.

| File | Change |
|---|---|
| `lifecycle/cleaner.rs` | `Cleaner` implemented: `clean_one(id, now)` and `clean_due(now)`. It receives `Arc<dyn AllocationStore>` and `Arc<dyn FixtureStorage>` and constructs no concrete adapter. |
| `lifecycle/lifecycle_tests.rs` | shell tests `ts26`, `ts27a`, `ts30` removed and `ts65` reduced to its sweeper half (done at the red stage; see the red manifest). |
| `infra/tests/cleaner.rs`, `infra/tests/mod.rs` | 13 Docker-backed tests (12 written at red, 1 added at green). |

No store or port changed: `claim_for_delete`, `mark_deleted`, `record_cleanup_failure` and `due_for_cleanup` already exist (WU-06) and already write `allocation.delete_claimed`, `allocation.deleted` and `cleanup.exhausted` in the transaction of the state change.

## Behavior of `clean_one(id, now)`

1. Read the row. Unknown id or a state other than `release_pending` or `orphan_discovered`: `Lost`, with no claim, unlink or audit.
2. `release_pending` before its `next_attempt_at`, or `orphan_discovered` before its `orphan_grace_until`: `NotDue`, nothing claimed.
3. Claim by compare-and-set on `state_version`. The claim and its `allocation.delete_claimed` row commit together. If the audit is refused there is no `deleting` row and the function returns `AuditUnavailable` without touching the file. A lost claim is `Lost`.
4. Unlink under the claim. A missing file counts as removed.
5. On success commit `deleted` with `allocation.deleted` in one transaction. If that commit fails the row stays `deleting` (the file may already be gone) and the error is returned; restart reconciliation (WU-17) resets the row and the next attempt starts with a fresh claim. A lost compare-and-set is `Lost`.
6. On an unlink failure record the failure through the store with reason `ADAPTER_ERROR`: the row returns to `release_pending` with `cleanup_attempts + 1` and `next_attempt_at = now + 60 s` (`RetryScheduled`), or on the third failure becomes `cleanup_failed` with `cleanup.exhausted` (`Exhausted`). No fourth unlink is ever attempted.

`clean_due(now)` cleans the rows `due_for_cleanup(now)` returns, one at a time in the store's order, and stops at the first error.

## Identifiers

No identifier was minted. `dtech-aidlc ids check` is clean for this unit.
