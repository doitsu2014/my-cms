# WU-08 known limitations

- **Status:** Open; the WU-08 implementation is partial.
- The verification proves only the bounded slice in `implementation.md`, not all WU-08 done criteria.
- `DomainBackupService::register_routes` constructs `BackupApiState` without a ready `Runtime`; service read/create handlers therefore return `BACKUP_NOT_READY`. Startup/runtime injection is incomplete and gateway lifecycle composition is WU-10.
- Standalone route integration tests create a ready runtime and a placeholder `DomainContext`. In production, the returned `Router<DomainContext>` carries handler state via `Extension<Arc<BackupApiState>>`; survival and ordering through the real gateway auth/compose layers remain unverified.
- TS-17 API canary scanning across responses/logs/audit/metrics is not implemented/run. Full web bundle canary is WU-15.
- TS-57 has a closed typed label vocabulary but no metric emitter; actual emitted label sets are untested.
- IF-11 uses simplified in-memory status values. Durable cleanup/reconciliation/ownership counts and complete escalation visibility are not integrated; TS-27b remains partial.
- TS-06 route-level create rejection coverage, full TS-60 streamed/Content-Length boundary coverage, and complete TS-62/TS-63 extractor mapping remain. `json_rejection` exists but is not wired across the route extractors.
- Gateway auth-before-handler ordering, effective bind, CFG-17/CFG-18 ingress limits, CORS/cookie behavior and composed router security are outside this slice and unverified (WU-10).
- No web checks, canary/bundle scan, CI, e2e, performance, launch or deployment check is claimed.
- **Rollback note:** source-only changes; no migrations or data changes. No gateway registration or deployment was changed. A rollback can revert the WU-08 code files; no destructive rollback operation was performed.
