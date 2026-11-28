# WU-13 green verification report

- **Status:** Green on the integrated main tree (through WU-10 commit `52c5875` + WU-13), 2026-11-28, macOS, node v24.14.1.

| Check | Result | Evidence |
|---|---|---|
| `pnpm exec vitest run src/App.test.tsx src/app/admin/backup src/app/admin/components/left-menu.test.tsx` (apps/web) | 5 files, 21 passed, 0 failed | `evidence/green-wu13-focused.log` |
| `pnpm test` (apps/web) | 24 files, 102 passed, 0 failed | `evidence/regression-wu13-web.log` |
| `pnpm exec tsc --noEmit` (apps/web) | 23 errors, all pre-existing; 0 in touched files | `evidence/green-wu13-tsc.log` |
| ESLint on touched files with the repo config minus the `react-hooks` entry (temporary file, deleted) | exit 0 | `evidence/green-wu13-eslint-touched.log` |
| `pnpm lint` | pre-existing config-load failure (see WU-12), not re-run as evidence | n/a |
| `dtech-aidlc ids check --stage build-and-verify` | `ok: true` | `evidence/green-wu13-ids.log` |

## Done-when trace

| Criterion | Tests | Result |
|---|---|---|
| TS-35 menu gated by `isAdministrator`; route inside `ProtectedRoute > AdminLayout > AdminOnlyRoute` | `ts35_only_shows_backup_navigation_to_administrators`, `ts35_backup_route_renders_for_administrators_inside_the_admin_layout`, `ts35_backup_route_shows_access_denied_to_non_administrators`, `ts35_backup_route_redirects_unauthenticated_visitors_to_login` | pass |
| TS-38 empty or unavailable instance list disables submission | `ts38_empty_instance_list_disables_submission`, `ts38_unavailable_instance_list_disables_submission_and_can_be_retried`, `ts38_submission_is_disabled_while_the_list_is_loading`, selector `ts38_*` | pass |
| TS-46 selector renders instance label and environment | `ts46_renders_the_instance_label_and_environment_label`, `ts46_marks_the_selected_instance_and_reports_selection` | pass |
| TS-52 panel: fresh `crypto.randomUUID()` per submit, double-submit guard | `ts52_sends_a_fresh_crypto_random_uuid_idempotency_key_for_every_submit`, `ts52_double_submit_guard_sends_one_request_while_one_is_in_flight` | pass |
