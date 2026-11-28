# WU-12 implementation

- **Status:** Complete against the WU-12 "Done when" (attempt 1, 2026-11-28).
- **Author:** parallel worktree worker (model `sonnet`, commit `f6fd156`), integrated into the main tree and verified by the conductor.

## Files
- apps/web/src/domains/backup.ts (new): zod schemas, types and REST client (F-41, D-13).
- apps/web/src/domains/backup.test.ts (new): TS-52 client part.
- apps/web/src/config/api.config.ts: added `getBackupApiUrl(path)`, which prefixes `/backup` and delegates to getApiUrl.

## Covers
F-41; IF-01, IF-02, IF-03, IF-04, IF-11 as a consumer; TS-52 (client part); feeds TS-12a; acceptance AC-001 S1 and AC-002 S1; T-06.

## Behaviour
- Status schema: union of the six documented values (queued, running, succeeded, failed, recovery-required, incomplete) and a string-to-`unknown` fallback. Non-strings fail.
- Job schema: snake_case, matching job_response.rs. `synthetic` is literal true and `download_eligible` literal false (fail closed). Components are limited to database, auth, storage with states synthetic_captured and unavailable. Progress is nullable.
- Status schema: state limited to ready, not_ready, degraded, disabled; `disabled_reason` is the closed set NOT_CONFIGURED, NON_LOOPBACK_BIND, INVALID_CONFIG or null; reconciliation and cleanup are nullable; synthetic_only is literal true.
- Client: listBackupInstances, createBackupJob (Idempotency-Key and Content-Type headers, body {instance_id}, `replayed` from the Idempotent-Replay header), listBackupJobs (limit and cursor), getBackupJob (id encoded), getBackupStatus. All use authenticatedFetch with cache no-store on reads.
- Errors: BackupApiError(status, errorCode, reasonCode, correlationId) parsed from the crate error body. Plain-text 4xx bodies (413, 415, 422 from axum extractors) give status only. A 2xx body that fails the schema gives reasonCode INVALID_RESPONSE. The message is fixed text, so server strings never leak to the UI.
- isTerminalBackupJobStatus: exhaustive switch with a never check.

## Decisions
- Success bodies and errors are parsed separately from the `{message, data}` envelope used by seo-head-asset.ts, as D-11 requires.
- Extra unknown object fields are stripped, not rejected, for forward compatibility. Value-level fields stay strict.
- `unknown` is the client-side name for an unrecognised status. It is non-terminal; the bounded 10-poll cap in IFC 3.2 belongs to WU-14.
- `recovery-required` and `succeeded` are classed terminal (never emitted for U1 jobs). Revisit in WU-14 if needed.
- Tests mock runtime-config rather than api.config, so URL construction is covered end to end.
- Schemas were re-checked by the conductor against the committed Rust serializers: `JobResponse` fields, `Components` (newtype over an array of `{name, state, reason_code}`), `MeasuredProgress {completed, total}`, `BackupInstance` (`environment`, `operations`, `capabilities.{create,upload,preflight,restore,download}`), instances envelope `{synthetic: true, instances}`, `Idempotent-Replay` header, IF-11 status object.
