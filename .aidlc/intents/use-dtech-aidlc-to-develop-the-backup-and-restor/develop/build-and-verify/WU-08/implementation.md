# WU-08 implementation

- **Status:** Partial implementation with a verified bounded slice; WU-08 is not complete.
- **Scope:** Only WU-08. Gateway composition, disabled/download/preview behavior, web work, persistence schema and migrations were not changed.
- **IDs:** All identifiers cited here exist in `develop/implementation-design/ids.json`; none were minted.

## Implemented slice

- `domain::instance::InstanceRegistry` projects the configured local instance, safe label, `local` environment and U1 capability flags (FN-02, TS-45).
- `domain::policy` implements administrator-first checks, loopback Host+configured port and configured POST Origin checks, plus the instance-based claim-time policy (FN-03/FN-04, TS-05b, TS-59).
- Job projection forces synthetic-only, always denies download, rejects U1-unreachable statuses, omits actor/internal fields and reports progress only for a measured database fixture (FN-10, TS-12b, TS-49). The three red tests remain and pass.
- `JobStore` and `DbJobStore` gained an instance-scoped, newest-first cursor-paginated list (FN-09, TS-61).
- Safe HTTP error bodies and create-error projections were added. REST handler code exists for IF-01..IF-04 and IF-11.
- A closed metric-label vocabulary type was added (FN-36, TS-57); there is no metrics emitter.
- Standalone API integration tests verify the configured instance/status payload, foreign/unknown job non-disclosure and safe projection, invalid UUID handling, malformed list limit/cursor mapping, Host/Origin rejection, media type rejection and declared Content-Length cap.

## Remaining WU-08 work

This is not the WU-08 done condition. Remaining items:

- Add route-level TS-06 rejection cases and verify create success/replay/conflict with acceptance persistence/audit.
- Complete TS-17 API canary scans across output, logs, audit and emitted metrics.
- Complete TS-27b using durable cleanup/reconciliation/ownership counts and the designed escalation fields; current status projection is simplified/in-memory.
- Add actual metric emission and test that TS-57 labels are closed in emitted metrics.
- Fully test TS-60 streamed overflow and both declared/streamed request-size paths. Wire/test JSON, query and path rejection mapping for TS-62/TS-63 throughout the actual routes; `json_rejection` is currently not wired to a JSON extractor.
- Complete runtime/state wiring. `DomainBackupService::register_routes` creates `BackupApiState` without a ready `Runtime`; production reads/create therefore remain `BACKUP_NOT_READY`. The adapter returns `Router<DomainContext>` with handler state supplied as `Extension<Arc<BackupApiState>>`; standalone route integration tests constructed a ready state and placeholder `DomainContext`, not the production service/gateway stack. Verify this extension and auth layer through actual composition in WU-10.
- Gateway auth-before-handler precedence, effective bind, ingress CFG-17/18, CORS/cookies and composed-router security are not verified here and remain WU-10 scope.

No gateway composition change was made. Disabled capabilities/download/previews belong to WU-09. No schema, migration, persistence, deployment or launch change was made.
