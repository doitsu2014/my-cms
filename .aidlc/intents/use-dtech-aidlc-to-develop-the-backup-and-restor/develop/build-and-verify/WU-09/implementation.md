# WU-09 implementation

- **Status:** Complete against the WU-09 "Done when" (attempt 1, 2026-11-28).
- **Author:** parallel worktree worker (model `sonnet`, commit `79fe2e8`), integrated into the main tree and verified by the conductor.
- **Scope:** F-11..F-14, IF-05..IF-10. No schema, migration, gateway or web change.

## Files

| File | Change |
|---|---|
| `domain_backup/src/domain/fixtures.rs` (new) | Compiled-in trusted catalog: `fixture-valid` (compatible), `fixture-incompatible`, `fixture-corrupt`; exact-id `lookup` on a closed set, so path- or package-like ids are unknown (F-13, TS-53). |
| `domain_backup/src/handlers/download_denied/` (new) | IF-05: 503 disabled/not-ready/store first, then malformed/unknown/foreign id = same 404 `JOB_NOT_FOUND`, then 403 `DOWNLOAD_DENIED` for every status. No file read, no URL or redirect; one attempted `download.denied` audit (F-11, TS-09). |
| `domain_backup/src/handlers/disabled_capability/` (new) | IF-06..IF-08 `POST /backup/packages`, `/backup/preflight`, `/backup/restore`: no body extractor (body never polled), 403 `CAPABILITY_DISABLED` regardless of `BACKUP_ENABLED` and readiness, no store/ledger/storage access; attempt-only `capability.denied` audit capped at 2 s, denial returned even if the audit fails (F-12, TS-09). Test support with body-polling spy. |
| `domain_backup/src/handlers/preview/` (new) | IF-09 `POST /backup/previews/preflight` `{fixture_id}` and IF-10 `POST /backup/previews/restore-confirmation` `{fixture_id, decision}`: stateless, strict JSON (`deny_unknown_fields`), 415 / 413 (16 KiB) / 400 order after readiness; canned labelled synthetic results; target fields from the configured instance only, never from the request; confirm and cancel enqueue nothing and write nothing (F-13, F-14, TS-43, TS-53, TS-64). |
| `domain_backup/src/handlers/http.rs` | Six routes registered behind the WU-08 authorization middleware; helpers made `pub(crate)` for reuse (`record_metric`, `record_refusal`, `operational_error`, `policy`, `store_error`, ...). |
| `domain_backup/src/handlers/mod.rs`, `domain/mod.rs`, `infra/tests/mod.rs` | Module wiring. |
| `domain_backup/src/infra/tests/capabilities_api.rs` (new) | Docker-backed TS-09/TS-10 tests over PostgreSQL. |
| `domain_backup/src/infra/tests/http_api.rs` | `ready_runtime` made `pub(super)` for reuse. |
| `domain_backup/Cargo.toml`, `apps/api/Cargo.lock` | dev-dependency `futures-core` (already in the lock graph) for the body-polling spy. |

## Decisions

- Denial audit is skipped when backup is disabled: `backup_audit.instance_id` is NOT NULL and no instance is configured. The denial itself is still returned.
- Download checks resource authorization before denying so that unknown and foreign ids stay indistinguishable (IF-05 404 before 403, IFC 2.1).
- Previews write no audit record: they create no state and the contract lists no preview audit event.
