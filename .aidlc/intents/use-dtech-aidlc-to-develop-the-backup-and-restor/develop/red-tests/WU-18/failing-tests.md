# FT-18 Failing tests (code manifest): WU-18

- **ID:** FT-18 (WU-18)
- **Stage:** Develop / red-tests (builder-verifier), unit WU-18 "Lifecycle cleaner: audited delete claim, unlink, retry and exhaustion".
- **Status:** red run recorded 2026-11-26 on Windows, PostgreSQL 15 via Docker.
- **Inputs:**
  - [work-units](../../implementation-design/work-units.md) WU-18
  - [ids.json](../../implementation-design/ids.json)
  - [test-strategy](../../implementation-design/test-strategy.md) TS-26, TS-27, TS-30 (backoff and grace boundaries), TS-44 (delete-claim ordering), TS-65 (claim and deleted paths); the cleaner half of TS-20, TS-21, TS-23 and TS-25
  - [interfaces-and-contracts](../../implementation-design/interfaces-and-contracts.md) sections 6.1a, 6.2 (windows 7 and 8), 9.1
  - [functional-design](../../implementation-design/functional-design.md) FN-29, FN-30 (legacy F-29, F-30)
- **Evidence:** [red-evidence.md](red-evidence.md); raw logs in [../../build-and-verify/evidence/](../../build-and-verify/evidence/): `red-wu18-cleaner-db-ignored.log`, `red-wu18-cargo-test.log`.

## Files

Under `apps/api/domain_backup/src/` unless noted.

| Path | Change | Red-stage behaviour |
|---|---|---|
| `lifecycle/cleaner.rs` | rewritten | Final injection surface and signatures: `Cleaner::new(allocs, storage)`, `clean_due(now)`, `clean_one(id, now)`. `clean_due` returns an empty list; `clean_one` returns `NotDue`. The audit and the clock are not injected: the allocation store writes `allocation.delete_claimed`, `allocation.deleted` and `cleanup.exhausted` in the same transaction as each state change, and `now` is a parameter. |
| `infra/tests/cleaner.rs` | new | 12 Docker-backed tests (`#[ignore]`). |
| `infra/tests/mod.rs` | modified | `mod cleaner;` |
| `lifecycle/lifecycle_tests.rs` | modified | shell tests `ts26`, `ts27a` and `ts30` removed; `ts65` reduced to its sweeper half (WU-19). |

No production behavior of an earlier unit changed.

## Shell tests removed or reduced, and what that loses

The argument-free shell tests called `Cleaner::new()` and asserted outcomes of a cleaner that had no ports, so they could not see any row or file. Their replacement is the suite below.

- `ts26`, `ts27a`: replaced by real tests.
- `ts30` (clock arithmetic plus two cleaner calls): removed. Its cleaner half is replaced by `ts30_retry_is_due_...`. Its sweeper half (24 h expiry, 1 h grace, 15 min sweep boundaries) has **no test until WU-19 writes one**; the persisted-deadline half remains red as `infra::tests::reconciliation::ts30_deadline_inventory_...`.
- `ts65` (cleaner claim, cleaner deleted commit, sweeper purge): the cleaner halves are replaced; the shell now holds only the purge half, still red, owned by WU-19.

## Tests (12 new, Docker-backed)

The allocation store and the test share a settable `TestClock`. Unlink failures come from a wrapper around the real `FsFixtureStorage`. At every unlink the wrapper reads the database and records how many `allocation.delete_claimed` rows were already committed and the row state, so an unlink without its own prior audited claim fails an assertion. Audit order comes from a recording sink around the real `DbAuditSink`.

| Test | TS | Asserts |
|---|---|---|
| `ts26_failed_deleted_commit_after_unlink_leaves_deleting_and_restart_recovers_idempotently` | TS-26, TS-65, TS-44 | refused `allocation.deleted`: file really unlinked, row `deleting` version 3, attempts 0, only the claim audited; after a restart (epoch 2, reconcile) a fresh claim precedes the retry, `deleted` commits, attempts still 0; audit order claim, reconciled, claim, deleted |
| `ts26_a_missing_file_counts_as_removed_and_commits_deleted_with_audit` | TS-26, TS-44 | no file: `deleted`, version 4, `deleted_at` set, two audit rows with owner job and instance, no actor |
| `ts27a_three_unlink_failures_are_spaced_exhausted_and_never_attempted_a_fourth_time` | TS-27a, TS-30 | attempts at t0, t0+60 s, t0+120 s (nothing at t0+59 s); `cleanup_failed`, attempts 3, one persisted `cleanup.exhausted`; an hour later still three unlinks, each after its own claim |
| `ts27a_a_retry_that_succeeds_commits_deleted_and_keeps_the_attempt_count` | TS-27a | one failure, then `deleted` with attempts 1 |
| `ts30_retry_is_due_exactly_at_its_backoff_time_and_not_before` | TS-30 | `NotDue` at 59 s with no unlink and no audit row; `Deleted` at 60 s and at 61 s |
| `ts23_orphan_is_removed_only_at_or_after_its_grace_and_after_an_audited_claim` | TS-23, TS-30 | kept one second before the grace; removed at the grace behind an audited claim |
| `ts65_a_refused_claim_audit_blocks_the_claim_and_the_unlink` | TS-65 | `AuditUnavailable`; row byte for byte unchanged; no audit row; file present; no unlink |
| `ts20_full_sequence_fence_reconcile_claim_then_deleted_for_a_missing_file` | TS-20, TS-44 | `reconcile.fenced`, `allocation.reconciled`, `allocation.delete_claimed`, `allocation.deleted`; job `failed`; allocation `deleted` |
| `ts21_full_sequence_an_existing_file_is_removed_only_after_the_audited_claim` | TS-21, TS-44 | the file survives reconciliation and is removed only after the claim |
| `ts25_stale_deleting_claim_is_reset_then_reclaimed_with_a_fresh_audited_claim` | TS-25 | `deleted` at version 6 with attempts still 1; audit order reconciled, claim, deleted |
| `rows_that_are_not_claimable_are_lost_without_unlink_or_audit` | TS-65 | `registered`, `written`, `deleting`, `deleted`, `cleanup_failed` and an unknown id are `Lost`; no unlink, no audit row, no file removed |
| `clean_due_handles_due_rows_and_leaves_future_retries_untouched` | TS-30 | only the due row is cleaned; a future retry is byte for byte unchanged |

## Provenance

- **Observed:** the results recorded in `red-evidence.md`.
- **Identifiers:** registry and Analyze only. `dtech-aidlc ids check` reports none unregistered for this unit.
