# IM-07 Implementation: WU-07 Idempotency, atomic acceptance

- **ID:** IM-07 (WU-07)
- **Stage:** Develop / build-and-verify (builder-verifier), unit WU-07.
- **Status:** implemented and verified 2026-11-26. Red evidence: [../../red-tests/WU-07/](../../red-tests/WU-07/red-evidence.md).
- **Scope:** FN-05..FN-08 logic (strict parse, key validation, digest and fingerprint, replay or conflict, atomic acceptance). No HTTP routing, no authorization (WU-08), no gateway wiring (WU-10).

## Files (under `apps/api/domain_backup/src/`)

| Path | Change | What it does |
|---|---|---|
| `domain/idempotency.rs` | new | `validate_key` (8..=128 of `[A-Za-z0-9_-]`, `Missing` vs `Invalid`), `key_digest` = `sha256(actor_sub 0x1f operation 0x1f key)`, `request_fingerprint` = `sha256` of canonical JSON `{"instance_id":..,"operation":..}` built from a struct so field order never depends on serde_json features |
| `handlers/create_job/request.rs` | new | `parse_create_request`: media type, 16 KiB cap, strict JSON (`deny_unknown_fields`, one field), key, configured instance, in that order |
| `handlers/create_job/create_job_handler.rs` | new | `CreateJobHandler::create`: replay or key reuse first, then one transaction (delete expired unpurged row, insert job, insert idempotency row, write `job.accepted` on the same transaction, commit). Any unique violation rolls back and re-reads the idempotency row: found means replay or reuse, not found means `CONCURRENCY_LIMIT` |
| `handlers/create_job/mod.rs` | new | `CreateJobError` with `reason_code()` and `status()` for the HTTP layer |
| `ports/job_store.rs` | modified | `IdemMatch`, `find_idempotency`, `delete_expired_idempotency` (defaulted so the in-memory double of WU-05 is untouched) |
| `infra/db_job_store.rs` | modified | implements both methods with parameterised SQL; `find_idempotency` treats a row with `expires_at <= now` as absent |
| `domain/mod.rs`, `handlers/mod.rs`, `lib.rs` | modified / new | module wiring |
| `Cargo.toml`, `apps/api/Cargo.lock` | modified | `sha2 = "0.10"`; the lockfile gains one line, `sha2 0.10.9` on `domain_backup` (the version `gateway` already locks) |

No earlier unit's production code or test changed beyond the two additive port methods.

## Behavior decisions

- **Errors:** an unclassified store error (`Persistence`, `NotFound`, `IllegalTransition`) maps to `STORE_UNAVAILABLE`. The contract lists only 503s for store problems; it defines no other server error for IF-02.
- **Commit failure:** any error at COMMIT is `STORE_UNAVAILABLE`, and nothing is accepted (TS-16).
- **Expiry:** replay needs `expires_at > now` on the injected clock. An expired, unpurged row is deleted in the acceptance transaction; the old job row stays for its own retention purge.
- **Epoch:** the new job is stamped with the lifecycle epoch passed to the handler's constructor. The runtime (WU-19) will supply it.
- **Replay and authorization:** a replay returns the stored job. The check that the job belongs to the configured instance (FN-04) is WU-08's.

## Identifiers

No identifier was minted. `dtech-aidlc ids check --stage build-and-verify` is clean.
