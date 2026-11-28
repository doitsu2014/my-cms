# WU-14 green verification report

- **Status:** Green on the integrated main tree (through WU-13 commit `0021087` + WU-14), 2026-11-28, macOS, node v24.14.1.

| Check | Result | Evidence |
|---|---|---|
| `pnpm exec vitest run src/domains/backup.test.ts src/app/admin/backup` (apps/web) | 12 files, 126 passed, 0 failed | `evidence/green-wu14-focused.log` |
| `pnpm test` (apps/web) | 32 files, 179 passed, 0 failed | `evidence/regression-wu14-web.log` |
| `pnpm exec tsc --noEmit` (apps/web) | 23 errors, all pre-existing; 0 in touched files | `evidence/green-wu14-tsc.log` |
| ESLint on touched files with the repo config minus the `react-hooks` entry (temporary file, deleted) | exit 0 | `evidence/green-wu14-eslint-touched.log` |
| `dtech-aidlc ids check --stage build-and-verify` | `ok: true` | `evidence/green-wu14-ids.log` |

## Done-when trace

| Criterion | Tests (see `red-tests/WU-14/failing-tests.md`) | Result |
|---|---|---|
| TS-12a every status value incl. `succeeded` / `recovery-required` rendered exhaustively, download always denied | `ts12a_*` in `job-status-badge.test.tsx` and `job-detail.test.tsx` | pass |
| Unknown status shows a safe fallback | `ts12a_unknown_status_shows_the_safe_fallback_and_never_a_success_label`, `ts12a_an_unrecognised_status_renders_the_safe_fallback_with_no_download` | pass |
| TS-18 server strings render only as React text | HTML/canary-bearing strings asserted escaped in `job-detail.test.tsx`, `backup-status-banner.test.tsx`, `job-list.test.tsx` | pass |
| TS-36 polling stops at a terminal state, bounded unknown polling, cleanup on unmount | `use-backup-jobs.test.ts` (fake timers) | pass |
| TS-37 accessible status (text label, hidden icon, live regions) | `ts37_*` tests | pass (automated part; manual AT review not claimed) |
| TS-50 banner renders IF-11 payloads incl. `disabled` with each `disabled_reason`, names the project owner | `backup-status-banner.test.tsx`, `use-backup-status.test.ts` | pass |
| TS-51 synthetic preview panel labelled synthetic; fixture package, target instance label, data in scope from IF-09/IF-10 | `synthetic-preview-panel.test.tsx`, preview client tests in `backup.test.ts` | pass |
| Browser verification | blocked by E2E-ENV-01 (stated, not claimed) | n/a |
