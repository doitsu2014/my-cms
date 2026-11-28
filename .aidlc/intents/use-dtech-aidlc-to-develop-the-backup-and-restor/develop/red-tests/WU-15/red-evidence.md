# WU-15 red evidence

Red step honesty: WU-15 is a verification sweep over behaviour built by WU-01..WU-14, so most tests characterize existing behaviour. Only two groups genuinely failed on behaviour first. Full verbatim output: `evidence/red.log`, `evidence/red-rust.log`, `evidence/red-web.log`.

## Commands and counts

| Command | Result |
|---|---|
| `cargo test -p domain_backup --lib verification_sweep --no-fail-fast -- --include-ignored --test-threads=3 --nocapture` (apps/api) | 5 tests: 4 passed, 1 failed (TS-58 doc check) |
| `pnpm exec vitest run scripts` (apps/web, scanner stub returning `[]`) | 4 tests: 2 failed, 2 passed |

## Behavioural red (assertion failures)

TS-58 doc check, before any doc change:

```
thread 'infra::tests::verification_sweep::ts58_feature_doc_lists_open_items_as_open_and_synthetic_only' panicked at domain_backup/src/infra/tests/verification_sweep.rs:379:9:
docs/features/backup-restore.md must list Q-04 as open
```

TS-17 bundle scanner against a stub that detects nothing:

```
FAIL ... positive control: reports a canary placed in a js asset
AssertionError: expected [] to deeply equal [ ObjectContaining{...} ]
FAIL ... positive control: reports a canary in html, css, json and map files
AssertionError: expected [] to deeply equal [ 'CANARY-backup-dsn-0001', ...(3) ]
```

Why behavioural: both are assertion failures against missing behaviour (docs lacking the open items; a scanner that finds nothing), after the code compiled and the test setup succeeded.

## Guards (passed on first run, characterization)

- TS-39, TS-40, TS-41 API part (Docker): passed on first run; these exercise behaviour from WU-05..WU-10.
  - First run output: `TS39_LATENCY_MS samples=30 min=1.09 median=1.38 p95=4.04 max=6.32`; `TS40_LOAD ... accepted=5 concurrency_limited=55 worker_runs=5 max_active_jobs=1 max_live_reserved_bytes=20480 max_disk_bytes=20480`.
- TS-58 plan check, scanner declaration test, scanner negative control.
- Ability to fail: TS-17 scanner proven by its two red tests plus two controls in the full run (injection into real dist, and a deliberately leaking `PUBLIC_*` build); TS-58 doc check proven red above; TS-40 and TS-41 assertions are on counts and status codes that the earlier unit tests mutate-verified; no mutation run was done in WU-15.

## Process note

The first red and first green Docker runs leaked 6 `postgres:15-alpine` containers because the helper used `std::mem::forget` on the test database handle. Fixed (handle now returned and held by each test), the 6 leaked containers were removed, and every later run left 0 containers.
