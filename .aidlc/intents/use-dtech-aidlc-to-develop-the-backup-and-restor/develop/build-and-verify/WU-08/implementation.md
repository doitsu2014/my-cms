# WU-08 implementation

- **Status:** Partial implementation with verified bounded API coverage; WU-08 is not complete.
- **Scope:** WU-08 only. Gateway composition, disabled/download/preview handlers, web work, schemas and migrations were not changed.
- **Identifiers:** Cites registered identifiers only; final `ids check` returned `ok: true`.

## Implemented and verified

- `InstanceRegistry` and `BackupPolicy` implement the configured local instance, safe label/capability projection, admin-first role assertion, loopback Host and configured Origin checks, plus the worker claim-time resource check (FN-02..FN-04; TS-05b, TS-45, TS-59).
- `JobResponse` protects the U1 projection: no actor/internal fields, `download_eligible` is always false, forbidden U1 terminal statuses are refused, and progress is null unless a measured DB component exists (FN-10; TS-12b, TS-49). Projection tests and four focused mutations pass/are killed (see `evidence/green-wu08-mutations.json`).
- `JobStore`/`DbJobStore` now list instance-scoped jobs newest-first with opaque cursor pagination and the bounded page limit (FN-09; TS-61).
- The Axum adapter exposes IF-01..IF-04 and IF-11 handlers, safe error bodies, create parsing/accept/replay, role/resource/Host/Origin checks, capped body reading, and durable status counts for pending, cleanup-exhausted, orphan-discovered and ambiguous-ownership rows (FN-09, FN-35, FN-36).
- PostgreSQL-backed HTTP tests cover instance/status output, durable TS-27b counts/escalation, TS-04 foreign/unknown 404 equivalence, TS-49 safe projection, invalid UUID (TS-63), invalid/unknown list query variants (TS-61), missing/wrong content type (TS-62), malformed JSON, unknown field, URL/DSN/key/synthetic fields, disallowed instance and no persistence effects (TS-06), successful create plus idempotent replay and sequential active-job conflict, durable job/idempotency/audit counts, raw-key canary absence from response/audit/idempotency/job/allocation/epoch rows, wrong Host/Origin/writer denial, and declared/streamed over-limit bodies (TS-59/TS-60). The sequential conflict check is not a concurrency-race test.
- `MetricRecorder` counts accepted/refused/denied API operations using fixed enum dimensions only; `BackupApiState` exposes the in-memory snapshot via `metric_snapshot()` for testing (TS-57). No production metrics export/telemetry endpoint was added; WU-08 has no approved exporter composition interface in scope.

## Remaining WU-08 work / boundary

- **TS-17:** the focused route test checks a raw-key canary in responses, audit/idempotency tables, jobs, allocations and epoch rows. It is not the full canary scan across captured logs, all API outputs, and emitted telemetry. WU-15 owns the full web-bundle scan.
- **TS-57:** typed in-memory counters are exercised on accepted/refused/denied HTTP paths; the IF-11 snapshot exposes only closed enum label values and exact fixed keys. Gateway OTLP/exporter telemetry is not integrated or verified.
- **TS-27b/FN-36:** durable cleanup counts and escalation visibility are now tested. Reconciliation `last_completed_at` / `last_result` remain simplified runtime projection fields and are not persisted/projected from a runtime record.
- **TS-62/TS-63:** tests cover missing/wrong content-type, malformed JSON, unknown fields, invalid UUID, invalid/unknown limit and cursor. Every extractor/transport/parser rejection combination is not covered; the standalone `json_rejection` helper is not wired to a `Json` extractor.
- **FN-37 production composition:** `DomainBackupService::register_routes` constructs `BackupApiState` without a ready `Runtime`; standalone API tests inject a ready test runtime. Production reads/create remain `BACKUP_NOT_READY` until runtime/gateway startup wiring in WU-10. Gateway auth-before-handler ordering, effective bind, CFG-17/CFG-18 ingress, CORS/cookie and composed-router security are WU-10, not verified here.
- No WU-09 disabled/download/preview work, WU-10 gateway integration, web work, schema/migration, deployment or launch is included.
