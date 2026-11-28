# FT-07 Failing tests (code manifest): WU-07

- **ID:** FT-07 (WU-07)
- **Stage:** Develop / red-tests (builder-verifier), unit WU-07 "Idempotency, atomic acceptance".
- **Status:** red run recorded 2026-11-26 on Windows, PostgreSQL 15 via Docker.
- **Inputs:**
  - [work-units](../../implementation-design/work-units.md) WU-07
  - [ids.json](../../implementation-design/ids.json)
  - [test-strategy](../../implementation-design/test-strategy.md) TS-13, TS-14b, TS-15, TS-16, TS-44a
  - [interfaces-and-contracts](../../implementation-design/interfaces-and-contracts.md) section 4 (idempotency contract), 5.2 (`backup_idempotency`), 7 (audit schema), 2.5 (IF-02)
  - [functional-design](../../implementation-design/functional-design.md) FN-05..FN-08 (legacy F-05..F-08)
- **Evidence:** [red-evidence.md](red-evidence.md); raw logs in [../../build-and-verify/evidence/](../../build-and-verify/evidence/): `red-wu07-pure.log`, `red-wu07-db-ignored.log`, `red-wu07-cargo-test.log`.

## Files

Under `apps/api/domain_backup/src/` unless noted.

| Path | Change | Red-stage behaviour |
|---|---|---|
| `domain/idempotency.rs` | new | `validate_key` accepts anything, `key_digest` and `request_fingerprint` return 64 zeros. Contains the pure tests. |
| `handlers/create_job/request.rs` | new | `parse_create_request` always returns `MalformedRequest`. Contains the pure parser tests. |
| `handlers/create_job/create_job_handler.rs` | new | `CreateJobHandler::create` always returns `StoreUnavailable`. |
| `handlers/create_job/mod.rs` | new | `CreateJobError` (final: 10 variants, `reason_code`, `status`), re-exports. |
| `handlers/create_job/tests.rs` | new | 14 Docker-backed tests (`#[ignore]`). |
| `ports/job_store.rs` | modified | adds `IdemMatch`, `find_idempotency`, `delete_expired_idempotency` with inert defaults (`Ok(None)` / `Ok(())`). `DbJobStore` does not implement them yet. |
| `domain/mod.rs`, `handlers/mod.rs`, `lib.rs` | modified / new | module wiring |
| `Cargo.toml` | modified | `sha2 = "0.10"` (already in the lockfile through `gateway`) |

No earlier unit's test or production behavior changed.

## Tests (32)

Pure (18, run by default):
- `domain::idempotency::tests`: 9 (key presence, length bounds inclusive 8..=128, character set, digest reference vector, digest scoping, field separator, digest never contains the key, fingerprint reference vector, fingerprint scoping). Reference vectors were computed independently with `sha256sum`.
- `handlers::create_job::request::tests`: 9 (valid request, media type, 16 KiB cap inclusive, size before parse, strict JSON, missing vs invalid key, instance not allowed, check order, raw key absent from error text).

Docker-backed (14, run with `-- --ignored`), in `handlers/create_job/tests.rs`:

| Test | TS | Asserts |
|---|---|---|
| `ts13_same_key_same_body_replays_the_existing_job` | TS-13 | replay returns the accepted job; one job row, one idempotency row, one `job.accepted` event |
| `ts13_same_key_different_body_is_key_reused` | TS-13 | `IDEMPOTENCY_KEY_REUSED`, no second job |
| `ts13_the_key_is_scoped_by_actor` | TS-13 | another actor with the same key is a new request |
| `ts13_raw_key_is_absent_from_every_table_and_only_digests_are_stored` | TS-13, TS-44a | the raw key is absent from every backup table; stored digest and fingerprint equal the reference functions |
| `ts13_concurrent_same_key_same_body_yields_one_job_and_replays` | TS-13 | 8 concurrent: one `Created`, seven `Replayed`, one job |
| `ts13_concurrent_same_key_different_body_yields_accept_and_key_reused` | TS-13 | 4 rounds of 2 concurrent: one accepted, one `IDEMPOTENCY_KEY_REUSED` |
| `ts13_an_expired_but_unpurged_key_is_a_new_request` | TS-13 | replay one second before expiry, new job at expiry, old row replaced in the transaction, old job row kept |
| `ts14b_a_second_key_is_concurrency_limit_and_a_replay_never_is` | TS-14b | `CONCURRENCY_LIMIT` for a new key; a replay is never answered `CONCURRENCY_LIMIT` |
| `ts14b_concurrent_different_keys_leave_exactly_one_active_job` | TS-14b | 6 concurrent: one accepted, five `CONCURRENCY_LIMIT`, one active job |
| `ts15_a_rejected_audit_write_persists_nothing` | TS-15 | `AUDIT_UNAVAILABLE`; no job, idempotency row, allocation or accepted event |
| `ts15_an_audit_outage_is_store_unavailable_and_persists_nothing` | TS-15 | `STORE_UNAVAILABLE`; nothing persisted |
| `ts16_a_failed_commit_is_not_accepted_and_a_retry_resolves` | TS-16 | a deferred constraint trigger fails the COMMIT; nothing persisted; the same key then creates one job and replays |
| `ts44a_job_accepted_event_has_every_field_and_no_excluded_content` | TS-44a | every audit column populated as specified, digest is the key digest, raw key absent |
| `ts44a_the_accepted_event_is_written_on_the_acceptance_transaction` | TS-44a, TS-15 | the sink receives the transaction and the job is not yet visible to another connection |

## Provenance

- **Observed:** the results recorded in `red-evidence.md`.
- **Identifiers:** registry and Analyze only. `dtech-aidlc ids check` reports none unregistered for this unit.
