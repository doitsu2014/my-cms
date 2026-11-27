# WU-08 known limitations

- **Status:** Partial implementation; WU-08 must not be reported complete yet.
- **Production composition:** `DomainBackupService::register_routes` creates `BackupApiState` without a ready `Runtime`; standalone tests inject a test Runtime. Production read/create therefore remain `BACKUP_NOT_READY` until WU-10 startup and gateway composition. Gateway auth ordering, effective bind, CFG-17/18 ingress, CORS/cookies and composed-router security were not changed/tested here.
- **Metrics:** TS-57 has an in-memory recorder wired to accepted/refused/denied API paths. IF-11 exposes snapshots with the three exact, enum-backed keys and no free-form label values. OTLP/exporter integration and emitted telemetry are not verified.
- **Canary:** Route integration checks the request-key canary is absent from create/replay responses and backup audit, idempotency, job, allocation and epoch records. Full TS-17 scanning across captured logs, all response variants and metrics is still outstanding; web-bundle scan is WU-15.
- **Status:** Durable pending, exhausted, orphan-discovered and ambiguous-ownership counts are queried and TS-27b escalation visibility is tested. Reconciliation completion timestamp/result fields remain simplified and not persisted/projected from runtime history.
- **Request rejection:** Tests cover malformed/unknown-field create JSON, URL/DSN/key/target/synthetic field rejection, unknown instance, declared and streamed oversize bodies, missing/wrong Content-Type, invalid UUID, bad/unknown list limit/cursor, Host and Origin rejection. Complete extractor/transport/parser matrix is not covered; `json_rejection` is not wired to a `Json` extractor.
- **Create flow:** One accepted create/replay plus sequential conflict against an existing active job are exercised at route level with durable counts. This does not race concurrent requests; WU-07 owns the handler's concurrency and audit-outage tests.
- **Platform:** Verified on Windows 11 with Docker/PostgreSQL 15; Unix-specific behavior was not exercised.
- **Out of scope:** WU-09 disabled/download/preview handlers, WU-10 gateway/runtime composition, web units, WU-15 full canary/e2e/performance/release evidence. No schema/migration, deployment, or launch change was made.
- **Rollback:** Source and tests only; no data or migration rollback is required. No rollback action was performed.
