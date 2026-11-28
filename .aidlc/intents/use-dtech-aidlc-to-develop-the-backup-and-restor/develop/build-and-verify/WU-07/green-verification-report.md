# GV-07 Green verification report: WU-07

- **ID:** GV-07 (WU-07)
- **Stage:** Develop / build-and-verify (builder-verifier), unit WU-07.
- **Status:** green on the unit's own tests, 2026-11-26. Workspace regression: see [regression-report.md](regression-report.md).
- **Environment:** Windows 11, cargo 1.100.0, Docker, `postgres:15-alpine`. Logs are in [../evidence/](../evidence/).

## 1. Results (Observed)

| Command (from `apps/api`) | Exit | Result | Log |
|---|---|---|---|
| `cargo test -p domain_backup -- domain::idempotency handlers::create_job::request` | 0 | 18 passed, 0 failed | `green-wu07-pure.log` |
| `cargo test -p domain_backup -- handlers::create_job::tests --ignored` | 0 | 14 passed, 0 failed (Docker) | `green-wu07-db-ignored.log` |
| `cargo fmt --all -- --check` | 0 | formatted | `green-wu07-fmt.log` |
| `cargo clippy --workspace --all-targets` | 0 | no warning in WU-07 files (a `manual_repeat_n` warning in a new test was fixed) | `green-wu07-clippy.log` |
| `dtech-aidlc ids check --stage build-and-verify` | 0 | `ok: true` | n/a |

Red to green: the 32 tests of [failing-tests.md](../../red-tests/WU-07/failing-tests.md) went from 14 + 13 failing (5 structural passes) to 32 passing, with no test changed between the red and green runs except formatting and one clippy fix (`repeat_n`) in the 16 KiB cap test.

## 2. Traceability

| Test | Result |
|---|---|
| TS-13 (7 tests): replay, key reuse, actor scope, raw key absent from every table, 8-way concurrent same key, concurrent same key with different body (4 rounds), expired unpurged key | pass |
| TS-14b (2 tests): `CONCURRENCY_LIMIT` for a new key and never for a replay; 6-way concurrent distinct keys leave exactly one active job | pass |
| TS-15 (2 tests): rejected audit is `AUDIT_UNAVAILABLE`, outage is `STORE_UNAVAILABLE`, nothing persisted in either case | pass |
| TS-16 (1 test): a COMMIT-time fault gives no accepted answer and persists nothing; the same key then creates one job and replays | pass |
| TS-44a (2 tests): every `job.accepted` column as specified; the audit row is written on the acceptance transaction while the job is invisible to another connection | pass |
| Pure unit tests (18): key bounds and character set, SHA-256 reference vectors computed independently with `sha256sum`, field separator, parser check order, 16 KiB cap inclusive | pass |

## 3. Checks that the tests can fail (mutation checks, Observed)

Each mutation was applied to the handler, the relevant tests were run, and the handler was restored byte for byte.

| Mutation | Result |
|---|---|
| audit written with `None` (its own connection) instead of the acceptance transaction | `ts44a_the_accepted_event_is_written_on_the_acceptance_transaction` fails |
| expired idempotency row not deleted | `ts13_an_expired_but_unpurged_key_is_a_new_request` fails |

The concurrent tests were also run three times in a row and passed each time.

## 4. Not verified here

- Windows only; the same tests were not run on Linux.
- The HTTP layer does not exist yet, so status mapping is asserted only through `CreateJobError::status()` and `reason_code()` (covered by compilation, not by an HTTP test). WU-08 owns that.
