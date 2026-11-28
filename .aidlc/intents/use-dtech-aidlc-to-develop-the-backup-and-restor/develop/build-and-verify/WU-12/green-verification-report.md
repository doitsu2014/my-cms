# WU-12 green verification report

- **Status:** Green on the integrated main tree (WU-08 `36361ae`, WU-09 `a0915ad`, WU-12 changes), 2026-11-28, macOS, node v24.14.1.

| Check | Result | Evidence |
|---|---|---|
| `pnpm exec vitest run src/domains/backup.test.ts` (apps/web) | 33 passed, 0 failed | `evidence/green-wu12-focused.log` |
| `pnpm test` (apps/web) | 20 files, 82 tests passed, 0 failed (base: 19 files, 49 tests) | `evidence/green-wu12-pnpm-test.log` |
| `pnpm exec tsc --noEmit` (apps/web) | exit 2 with 23 errors, identical set to the base commit; none in `backup.ts`, `backup.test.ts` or `api.config.ts` | `evidence/green-wu12-tsc.log`, `evidence/wu12-tsc-base.txt` |
| `pnpm lint` (apps/web) | exit 2 at ESLint config load (`react-hooks` `recommended-latest` not a flat-config object) before any file is linted; `eslint.config.mjs` unchanged since 2026-08-15; same failure on base | `evidence/green-wu12-lint.log`, `evidence/wu12-lint-base.txt` |
| ESLint on the 3 touched files with the repo config minus the `react-hooks` entry (temporary file, deleted) | exit 0, no findings | `evidence/green-wu12-eslint-touched.log` |
| `dtech-aidlc ids check --stage build-and-verify` | `ok: true` | `evidence/green-wu12-ids.log` |

## Done-when trace

| Criterion | Evidence | Result |
|---|---|---|
| TS-52 client part green incl. unknown-status tolerance | `backup.test.ts` status-schema and job-schema cases | pass |
| Status schema accepts every documented value plus `incomplete` (D-10) | 6 per-status cases | pass |
| `disabled_reason` closed set `NOT_CONFIGURED \| NON_LOOPBACK_BIND \| INVALID_CONFIG \| null` | status-object cases (accept 3 + null, reject others and missing) | pass |
| `pnpm lint` and `tsc --noEmit` pass for the touched files | tsc: zero errors in touched files; lint: touched files clean with the broken plugin entry removed. `pnpm lint` as a whole cannot run in this repo (pre-existing config defect) | pass for touched files, with the stated substitution |
