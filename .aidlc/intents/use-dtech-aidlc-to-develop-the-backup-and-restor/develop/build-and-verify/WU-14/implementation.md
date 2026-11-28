# WU-14 implementation

- **Status:** Complete against the WU-14 "Done when" (attempt 1, 2026-11-28).
- **Author:** worktree worker (model `sonnet`, commit `9dec63a`), integrated into the main tree and verified by the conductor. Conductor checks: IF-09/IF-10 zod schemas match the `json!` bodies in `domain_backup/src/handlers/preview/mod.rs` field for field; no download control or IF-05 call exists in the UI (every "download" mention is explanatory text); no raw-HTML rendering; polling constants follow IFC 3.2 (2 s interval, at most 10 consecutive `unknown` polls, error backoff ceiling 30 s, cleanup on unmount).


Commit: "WU-14 worker" on worktree branch (base 0021087). All files under apps/web/src/.

## Files
New: app/admin/backup/components/{job-status-badge,job-detail,job-list,synthetic-preview-panel,backup-status-banner}.tsx and colocated .test.tsx; app/admin/backup/hooks/{use-backup-jobs,use-backup-status}.ts and .test.ts (new hooks/ directory, D-12); app/admin/backup/page.status.test.tsx.
Changed: app/admin/backup/page.tsx (wires banner, job list, job detail, preview panel; refreshes jobs and status after create); domains/backup.ts (additions only: BACKUP_FIXTURE_IDS, types, backupPreflightPreviewSchema, backupRestoreConfirmationSchema, previewBackupPreflight, previewBackupRestoreConfirmation; no existing export changed); domains/backup.test.ts (new TS-51 client describe block, import additions); app/admin/backup/page.test.tsx (test-support only: the vi.mock factory also stubs listBackupJobs and getBackupStatus, because the page now calls them; no assertion changed).

## Coverage
F-45 (JobStatusBadge, JobDetail), F-46 (JobList, useBackupJobs), F-47 (SyntheticPreviewPanel), F-48 (no download control or IF-05 call anywhere; tests assert absence), F-49 (BackupStatusBanner, useBackupStatus). IF: consumer of IF-03, IF-09, IF-10, IF-11. TS-12a, TS-18, TS-36, TS-37, TS-50, TS-51.

## Decisions
- JobStatusBadge uses a switch with a `never` check over the six statuses plus `unknown` (IFC 3.1). Incomplete note "synthetic, database-only, not restorable" shows when `detailed` (detail view).
- useBackupJobs follows IFC 3.2: 2 s poll while any job is non-terminal, one request at a time (next timer set after the previous settles), backoff 2,4,8,16,30,30 s on errors, reset on success, unknown counts as non-terminal for at most 10 consecutive polls then `exhausted` with an alert and manual Refresh. Cleanup clears the timer and a `cancelled` flag drops late responses; token change and `refresh()` restart through an effect dependency.
- Banner maps only the closed vocabularies (state, disabled_reason) to fixed copy through exhaustive switches. The one server string shown is `escalation.owner`, as React text with underscores replaced by spaces ("project_owner" -> "project owner"). When state is disabled the owner is always named; otherwise only when escalation.visible. Role is alert for degraded, disabled or escalation, status otherwise.
- SyntheticPreviewPanel: fixture chosen from the closed list, confirmation only offered when IF-09 result is compatible; shows fixture_label, target_instance_label, target_environment and data_scope from IF-09, then from IF-10 after a decision; "Restore not executed." always stated; upload is a disabled "Upload package (unavailable)" button. Failure copy is fixed.
- All server strings render as React text children or attributes; no dangerouslySetInnerHTML anywhere.
- Page keeps the WU-13 create flow and selects the created job after refresh.

## Verification (see green-*.log)
- pnpm exec vitest run src/domains/backup.test.ts src/app/admin/backup: 12 files, 126 tests passed.
- pnpm test (apps/web): 32 files, 179 tests passed.
- pnpm exec tsc --noEmit: 23 errors, all in untouched files; 0 in touched files.
- eslint with a temporary config copy without the reactHooks lines (deleted afterwards) on src/domains/backup.ts, backup.test.ts and src/app/admin/backup: exit 0, no findings.
