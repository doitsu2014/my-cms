# RE-07 Red evidence: WU-07

- **ID:** RE-07 (WU-07)
- **Stage:** Develop / red-tests (builder-verifier), unit WU-07.
- **Status:** red run recorded 2026-11-26.
- **Companion:** [failing-tests.md](failing-tests.md).
- **Baseline:** [../../baseline.md](../../baseline.md), reused.

Provenance:
- **Observed:** the conductor ran everything listed here on 2026-11-26, Windows 11, cargo, Docker and `postgres:15-alpine`.

## 1. Environment

- `docker` prerequisite passed in the `orchestrate next` directive. No environment change was made.
- WU-07 was run before WU-17 at the owner's direction (WU-17 was parked as an `owner-decision` for that scheduling choice). WU-07 depends only on WU-03.

## 2. Red run (Observed)

| Command (from `apps/api`) | Exit | Result |
|---|---|---|
| `cargo test -p domain_backup --no-run` | 0 | compiles; the only warnings are in code that existed before WU-07 |
| `cargo test -p domain_backup -- domain::idempotency handlers::create_job::request` ([log](../../build-and-verify/evidence/red-wu07-pure.log)) | 101 | 4 passed, 14 failed |
| `cargo test -p domain_backup -- handlers::create_job::tests --ignored` ([log](../../build-and-verify/evidence/red-wu07-db-ignored.log)) | 101 | 1 passed, 13 failed |
| `cargo test -p domain_backup --no-fail-fast` ([log](../../build-and-verify/evidence/red-wu07-cargo-test.log)) | 101 | 120 passed, 27 failed: 14 new WU-07 tests plus the 13 lifecycle tests that belong to WU-17, WU-18 and WU-19 |
| `cargo fmt -p domain_backup -- --check` | 0 | formatted |

- **Failure quality:** every failure is an assertion on behavior, not a compile, setup or connection error.
  - Pure tests: the stub's answers differ from the contract (digest and fingerprint are 64 zeros and differ from the independent `sha256sum` vectors; every key is accepted; the parser answers `MalformedRequest` for valid input).
  - DB tests: `expected Created, got Err(StoreUnavailable)` in nine tests; `ts13_concurrent_same_key_same_body` and `ts14b_concurrent_different_keys` fail at their outcome counts; `ts15_a_rejected_audit_write` fails on `assert_eq!(r, Err(AuditUnavailable))` because the stub answers `StoreUnavailable`.
- **Structural passes (not red evidence, stated openly):** 5 tests pass against the stubs because the stub's inert answer happens to match them.
  - `missing_key_is_missing_not_invalid`, `digest_never_contains_the_raw_key`, `strict_json_rejects_malformed_unknown_and_missing_fields` and `the_raw_key_is_not_in_any_error_text` (the stub always errors or returns zeros);
  - `ts15_an_audit_outage_is_store_unavailable_and_persists_nothing` (the stub answers `StoreUnavailable` and persists nothing).
  They guard the final behavior against regressions but did not fail first.
- **Earlier tests:** every WU-01..WU-06 test that passed before still passes; the only other failures in the default run are the 13 lifecycle tests assigned to WU-17..WU-19.
- **TS-16 fault injection:** a deferred constraint trigger on `backup_idempotency` raises at COMMIT, so every statement succeeds and only the commit fails. A flag table disarms it for the retry.

## 3. Open items

- None for this stage. The handler, parser and digest functions are stubs by design; `build-and-verify` implements them and the `DbJobStore` idempotency methods.
