# WU-13 implementation

- **Status:** Complete against the WU-13 "Done when" (attempt 1, 2026-11-28).
- **Author:** parallel worktree worker (model `sonnet`, commit `fedfc30`), integrated into the main tree and verified by the conductor. The conductor also restored a whitespace the worker had removed in the unrelated `/admin/login` route line of `App.tsx`.
- **Covers:** F-42, F-43, F-44; TS-35, TS-38, TS-46, TS-52 (panel part); AC-001, AC-002 S1; T-01 (UX only), T-03, T-07.

## Files

| File | Change |
|---|---|
| `apps/web/src/App.tsx` | Route `/admin/backup` inside `ProtectedRoute > AdminLayout > AdminOnlyRoute`. |
| `apps/web/src/app/admin/components/left-menu.tsx` | "Backup" menu item gated by `isAdministrator`. |
| `apps/web/src/app/admin/backup/page.tsx` (new) | Loads instances with `listBackupInstances(token)`; auto-selects the single listed instance (IF-01 lists zero or one); passes a submit instance id only when the list is ready and contains the selection; shows the created job reference. |
| `apps/web/src/app/admin/backup/components/instance-selector.tsx` (new) | Radio list with label and environment badge; loading, error (with retry) and empty states offer nothing to select; server strings render as text. |
| `apps/web/src/app/admin/backup/components/job-create-panel.tsx` (new) | Fresh `crypto.randomUUID()` idempotency key per submit; ref-based in-flight guard plus disabled button (double-submit guard); fixed failure copy, server strings never shown. |
| Tests | `page.test.tsx`, `instance-selector.test.tsx`, `job-create-panel.test.tsx` (new), `App.test.tsx` (new), `left-menu.test.tsx` (extended). |

## Decisions

- The WU-12 client is used unchanged.
- Job status display, polling, list, previews and the status banner are WU-14; the page shows only the created job id.
- TS-35 and the client-side guards are UX only; the server enforces authorization (WU-08, WU-10).
