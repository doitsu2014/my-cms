# WU-12 failing tests (code manifest)

File: apps/web/src/domains/backup.test.ts. TS: TS-52 (client part). Feeds TS-12a. 33 cases in 5 describe blocks. The `@/config/runtime-config` module is mocked (restApiUrl https://api.example.test/) and global fetch is stubbed, so real api.config URL building and authenticatedFetch headers are exercised.

## TS-52 client: status schema
- accepts each of the six documented statuses unchanged (queued, running, succeeded, failed, recovery-required, incomplete), 6 cases (D-10).
- maps an unknown status string to `unknown`.
- rejects a non-string status (undefined, null, number, object, array).
- isTerminalBackupJobStatus: queued, running and unknown are non-terminal; incomplete, failed, succeeded, recovery-required are terminal.

## TS-52 client: job schema
- parses a queued job (components [], progress null) and an incomplete job (3 components, progress 1/1).
- an unknown status inside a job becomes `unknown` and download_eligible stays false.
- fails closed when download_eligible is true or synthetic is false.
- rejects a missing id, non-string created_at, unknown component name or state, negative progress.
- list envelope requires next_cursor; instances envelope requires synthetic true; zero or one instance both parse.

## TS-52 client: status object (IF-11)
- parses a ready status; accepts disabled_reason NOT_CONFIGURED, NON_LOOPBACK_BIND, INVALID_CONFIG (3 cases) with null cleanup and reconciliation.
- rejects a disabled_reason outside the closed set and a missing disabled_reason.
- accepts states ready, not_ready, degraded, disabled; rejects others.
- requires synthetic_only true and non-negative integer counters.

## TS-52 client: REST calls
- listBackupInstances: GET https://api.example.test/backup/instances with Bearer token.
- createBackupJob: POST /backup/jobs with Idempotency-Key, Content-Type application/json, body exactly {instance_id}; 202 gives replayed false; 200 with Idempotent-Replay: true gives replayed true.
- listBackupJobs: limit and cursor sent as query params only when supplied.
- getBackupJob: id is URL-encoded; unknown status tolerated.
- getBackupStatus: parses a disabled state with reason.

## TS-52 client: error handling
- crate error body becomes BackupApiError with status, errorCode, reasonCode, correlationId.
- 503 BACKUP_DISABLED reason preserved.
- non-JSON error body (plain text 413) gives BackupApiError with status only.
- a 2xx body violating the contract (download_eligible true, or HTML) gives reasonCode INVALID_RESPONSE.
- server error strings (an XSS-style string) are never copied into the error message.

Author: parallel worktree worker (model `sonnet`, commit `f6fd156` on `worktree-wf_6fc7eae6-d0c-2`), integrated and checked by the conductor.
