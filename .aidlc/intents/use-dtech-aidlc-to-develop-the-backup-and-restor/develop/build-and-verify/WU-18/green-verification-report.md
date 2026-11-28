# GV-18 Green verification report: WU-18

- **ID:** GV-18 (WU-18)
- **Stage:** Develop / build-and-verify (builder-verifier), unit WU-18.
- **Status:** green on the unit's own tests, 2026-11-26. Workspace regression: see [regression-report.md](regression-report.md).
- **Environment:** Windows 11, cargo 1.100.0, Docker for PostgreSQL 15. Logs are in [../evidence/](../evidence/).

## 1. Results (Observed)

| Command (from `apps/api`) | Exit | Result | Log |
|---|---|---|---|
| `cargo test -p domain_backup infra::tests::cleaner -- --ignored` | 0 | 13 passed | `green-wu18-focused-db.log` |
| `cargo fmt --all -- --check` | 0 | formatted | `green-wu18-fmt.log` |
| `cargo clippy --workspace --all-targets` | 0 | no warning in `cleaner.rs` or `infra/tests/cleaner.rs`; the rest is code that existed before WU-18 | `green-wu18-clippy.log` |
| `dtech-aidlc ids check --stage build-and-verify` | 0 | `ok: true` | n/a |

The 12 tests written at red all passed on the first green run. The 13th (`a_concurrent_change_during_the_unlink_makes_the_deleted_commit_lost`) was added because the first mutation run showed no test covered a lost `deleted` commit; a clippy warning about an unused import in the test file was fixed in the same step.

## 2. Tests assert durable state (Observed)

The allocation store and the test share a settable clock, unlink failures are injected into the real `FsFixtureStorage`, and at every unlink a probe reads the database: how many `allocation.delete_claimed` rows were already committed and what state the row was in. Examples of what the green run proves:

- TS-26: with the `deleted` audit refused, the file is really gone, the row stays `deleting` at version 3 with attempts 0 and only the claim is audited. After a restart (epoch 2, WU-17 reconcile) a fresh claim precedes the retry and `deleted` commits with attempts still 0. Audit order: claim, reconciled, claim, deleted.
- TS-27a and TS-30: failures at t0, t0+60 s and t0+120 s give attempts 1, 2, 3, retry times t0+60 s and t0+120 s, final `cleanup_failed` with one persisted `cleanup.exhausted`; nothing happens at t0+59 s; an hour later there are still exactly three unlinks, each after its own claim.
- TS-65: with the claim audit refused the row is byte for byte unchanged, no audit row exists and the file is untouched.
- TS-20, TS-21, TS-25: the complete sequences. `reconcile.fenced`, `allocation.reconciled`, `allocation.delete_claimed`, `allocation.deleted`; the file survives reconciliation and goes only after the claim; a stale `deleting` row costs no retry. This closes the limitation recorded in KL-17.

## 3. The tests can fail (mutation checks, Observed)

Each edit was applied to `cleaner.rs`, the cleaner tests were run, and the file was restored byte for byte (checked with `cmp`). Raw result: `green-wu18-mutations.json`.

| Mutation | Tests that failed |
|---|---|
| a `release_pending` row is claimed before its backoff ends | `ts30_retry_is_due_exactly_at_its_backoff_time_and_not_before` |
| an orphan is claimed before its grace ends | `ts23_orphan_is_removed_only_at_or_after_its_grace_and_after_an_audited_claim` |
| an unlink failure is treated as success | both `ts27a` tests |
| a failed unlink reports `Retry` as `Exhausted` | both `ts27a` tests |
| an exhausted unlink is reported as a scheduled retry | `ts27a_three_unlink_failures_...` |
| a lost `deleted` commit is reported as `Deleted` | `a_concurrent_change_during_the_unlink_makes_the_deleted_commit_lost` |
| `clean_due` cleans nothing | six tests |

All seven mutants are killed. An earlier eighth mutant (a guard on the failure arm) did not compile and was replaced by the "exhausted reported as retry" mutant; it is not counted.

## 4. Not verified here

- The sweeper's 24 h expiry, 1 h grace and 15 min sweep boundaries and the purge half of TS-65: WU-19.
- A database outage in the middle of a claim or a deleted commit is covered by the store-level tests, not by a cleaner test. A refused audit is covered.
- A claim that is lost (not a lost deleted commit) has no test of its own; the unclaimable-state test covers the read-side `Lost`.
- The cleaner is not called by any runtime yet (WU-19).
- Windows only.
