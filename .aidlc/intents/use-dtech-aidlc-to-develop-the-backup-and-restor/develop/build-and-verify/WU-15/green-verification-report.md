# WU-15 green verification report and evidence record

- **Status:** Green on the integrated main tree (through WU-14 commit `8ceb568` + WU-15), 2026-11-28, macOS Darwin 25.6.0, Docker `postgres:15-alpine` testcontainers, debug builds. The release-candidate gate decision is NOT made here.

## Main-tree runs (conductor, after integration)

| Command | Result | Log |
|---|---|---|
| `cargo test --workspace --no-fail-fast` (apps/api) | 440 passed, 0 failed, 219 ignored | `evidence/regression-wu15-rust-normal.log` |
| `cargo test --workspace --no-fail-fast -- --ignored --test-threads=4` (apps/api) | 219 passed, 0 failed (first run) | `evidence/regression-wu15-rust-ignored.log` |
| `cargo test -p domain_backup verification_sweep -- --include-ignored --nocapture` | 4 passed; `TS39_LATENCY_MS samples=30 min=1.15 median=1.75 p95=3.32 max=6.26`; `TS40_LOAD accepted=5 concurrency_limited=55 worker_runs=5 max_active_jobs=1 max_live_reserved_bytes=20480 max_disk_bytes=20480` | `evidence/green-wu15-sweep-focused.log` |
| `cargo fmt --all -- --check`; `cargo clippy -p domain_backup -p gateway --all-targets`; `cargo check --workspace` | exit 0 each; no domain_backup clippy warnings | `evidence/green-wu15-fmt.log`, `-clippy.log`, `-check.log` |
| `pnpm test` (apps/web) | 33 files, 183 passed, 0 failed | `evidence/regression-wu15-web.log` |
| `pnpm scan:bundle-canary` (apps/web) | PASS: 6 canaries, 79 dist files, 0 hits; injected control detected; leaking-build control detected | `evidence/green-wu15-bundle-canary-scan.log` |
| TS-58 implementation-plan check (one-off) | Q-04, Q-06, Q-07, Q-10, Q-11, Q-12, audit-record retention, non-owner guards listed open in plan section 3.1 | `evidence/ts58-implementation-plan-check.log` |
| `dtech-aidlc ids check --stage build-and-verify` | `ok: true` | `evidence/green-wu15-ids.log` |
| Leftover test containers after the runs | 0 | observed |

Worker-side runs are kept as `evidence/worker-*.log` (two worker Docker runs lost 1-2 unrelated tests to container start-up failures, each passing on rerun).

## Integration change

The worker's `ts58_implementation_plan_lists_open_items_as_open` test read `.aidlc/intents/.../implementation-plan.md` from the product test suite. Removed: product tests must not depend on intent artifacts (it would break when the intent is archived). The check is recorded as a one-off evidence run instead (row above). The feature-doc check `ts58_feature_doc_lists_open_items_as_open_and_synthetic_only` stays, because `docs/features/backup-restore.md` is a product document.


## Evidence record

### 2. Requested checks

#### TS-17 full (T-09)

- Bundle scan: `apps/web` built with 6 canary values in the build environment (`BACKUP_DATABASE_URL`, `BACKUP_STORAGE_KEY`, `VITE_BACKUP_SECRET`, `PUBLIC_BACKUP_SECRET`, `SUPABASE_SERVICE_ROLE_KEY`, `DATABASE_URL`). Result: 0 hits in 79 files of `dist`. A plain `grep -rl CANARY dist` also returned 0.
- Positive controls (required by the leakage canary approach): (A) a canary appended to a script in a copy of the real `dist` was detected; (B) a separate build whose source reads `process.env.PUBLIC_BACKUP_SECRET` (the one mechanism by which rsbuild embeds an env value) was detected. Scanner unit tests add html/css/json/map file types and a clean negative control.
- Note: rsbuild only exposes `PUBLIC_*` variables, so `BACKUP_*` and `VITE_*` values cannot reach the bundle by construction, and no source reads a `PUBLIC_BACKUP_*` variable. `public/config.js` ships the existing public Supabase anon key by design (not a canary).
- API canary tests re-run and pass: `http_api::ts45_instances_and_status_return_the_configured_safe_contract` and `http_api::ts17_store_outage_paths_are_safe_and_status_stays_200_degraded` (`evidence/regression-wu15-rust-ignored.log`; also in the 219/0 ignored run). Token header fields are excluded from canary injection as the strategy states.

#### TS-39 job-creation latency (no target; P-01..P-05 open)

30 sequential creates through the started `DomainBackupService` routes, each followed by `worker_step`, Docker Postgres, debug build, local machine:

| Run | min ms | median ms | p95 ms | max ms |
|---|---|---|---|---|
| red run (first) | 1.09 | 1.38 | 4.04 | 6.32 |
| green focused run | 1.08 | 1.71 | 3.36 | 6.00 |

p99 is not reported (30 samples). These are measurements, not a pass/fail against a target.

#### TS-40 bounded concurrency and temporary bytes

5 rounds x (12 concurrent creates and 4 concurrent `worker_step`), a 5 ms sampler on the database and the storage directory (`evidence/green-wu15-sweep-focused.log`):

| Quantity | Observed | Bound |
|---|---|---|
| creates accepted / `CONCURRENCY_LIMIT` (409) | 5 / 55 of 60 | at most one active job per instance |
| worker runs | 5 (= accepted; each job run once) | concurrency 1 |
| max active jobs sampled | 1 | 1 |
| max live reserved bytes | 20480 | 1 GiB (CFG-12) |
| max single allocation | 4096 | 256 MiB (CFG-11) |
| max bytes on disk under the storage root | 20480 | 1 GiB |
| jobs terminal | 5 of 5 (`incomplete` or `failed`) | none stuck |

The fixture is small, so the byte numbers show the accounting works and stays far below the budgets; they do not stress the budget edge (that is TS-30 in WU-04).

#### TS-41 API part

Passes (`ts41_admin_creates_job_ends_incomplete_and_download_is_denied`): admin creates, worker ends `incomplete`, `download_eligible` false, `GET .../download` is 403 `DOWNLOAD_DENIED` with no Location, Content-Disposition or Content-Range, and does not echo the job id. UI part: BLOCKED by E2E-ENV-01 (no environment decision). Component evidence instead: WU-14 `job-detail.test.tsx` `ts12a_status_*_parses_renders_and_offers_no_download` (every status renders no download button, no link or `[download]` element, and shows "Download is not available"), `D/build-and-verify/WU-14/green-verification-report.md`. Gateway-boot runtime part stays the descoped RK-15 residual (WU-10 known-limitations item 1).

#### TS-58

Passes: `ts58_feature_doc_lists_open_items_as_open_and_synthetic_only` (red first, then green after the doc update) and the one-off plan check (`evidence/ts58-implementation-plan-check.log`). Open and listed: Q-04, Q-06, Q-07, Q-10, Q-11, Q-12, audit retention, non-owner guards (16 KiB body cap, list size 20/100, fixture size, 60 s probe), in `docs/features/backup-restore.md` section 14.3 and `implementation-plan.md` section 3.1. Synthetic-only labels: API `status.synthetic_only`, `instances.synthetic`, job `synthetic` (asserted in the TS-41 test), previews `synthetic` (`handlers/preview/mod.rs`, TS-64 in WU-09); UI banner text "Jobs are synthetic and database-only" (`backup-status-banner.test.tsx`), `SyntheticPreviewPanel` (TS-51, WU-14) and job detail note.

### 3. TS outcomes by owner unit

Outcome words are as recorded by the unit. "unit-green" means the unit's GVR reports its done-when green and the TS is itemised in its FT manifest but not by name in the GVR; all of these also pass in the WU-15 whole-suite rerun above.

| TS | Unit | Outcome | Evidence |
|---|---|---|---|
| TS-01, TS-02 | WU-01 | pass (config focused tests) | WU-01 GVR; FT |
| TS-03, TS-07, TS-08b, TS-55 | WU-10 | pass | WU-10 GVR done-when trace; `D/red-tests/WU-10/failing-tests.md` |
| TS-04, TS-05b, TS-06, TS-12b, TS-27b, TS-45, TS-49, TS-57, TS-59, TS-60, TS-61, TS-62, TS-63 | WU-08 | pass | WU-08 GVR done-when trace |
| TS-17 (API outputs) | WU-08 | pass | WU-08 GVR |
| TS-17 (bundle scan) | WU-15 | pass with positive controls | `evidence/green-wu15-bundle-canary-scan.log` |
| TS-05a, TS-47, TS-48, TS-67, TS-68 | WU-05 | pass (TS-47, TS-48 listed passed in GVR; TS-05a, TS-67, TS-68 unit-green) | WU-05 GVR; `D/red-tests/WU-05/failing-tests.md` |
| TS-08a | WU-01 | pass (config suite) | WU-01 GVR |
| TS-09, TS-10, TS-43, TS-53, TS-64 | WU-09 | pass | WU-09 GVR |
| TS-11 | WU-01, WU-02 | pass (pure part WU-01; DB constraint and trigger part WU-02 unit-green) | WU-01 GVR; `D/red-tests/WU-02/failing-tests.md` |
| TS-12a, TS-18, TS-36, TS-37, TS-50, TS-51 | WU-14 | pass (TS-37 automated part only; manual AT review not claimed) | WU-14 GVR |
| TS-13, TS-14b, TS-15, TS-16, TS-44a | WU-07 | pass | WU-07 GVR |
| TS-14a, TS-69 | WU-03 | unit-green | WU-03 GVR; `D/red-tests/WU-03/failing-tests.md` |
| TS-19, TS-30 (budget), TS-32 (lock), TS-56 (bounded inventory) | WU-04 | pass | WU-04 GVR |
| TS-31 | WU-04 | PARTIAL on Windows (no owner-only ACL check; owner-accepted synthetic-only residual); Unix 0700/0600 checks compile on macOS and run in the WU-15 suite | WU-04 GVR; implementation-plan 3.1 |
| TS-20..TS-29, TS-27a, TS-30 lifecycle, TS-32 runtime, TS-42 injection, TS-44b lifecycle, TS-56 fail-closed, TS-65 lifecycle | WU-17/18/19 | pass (TS-20..TS-25 and TS-26, TS-27a itemised in WU-17/WU-18 GVR; TS-28, TS-29 unit-green) | WU-17/18/19 GVR; FT |
| TS-42 adapter, TS-44b store CAS, TS-56 join, TS-65 store | WU-06 | pass (1/1 each) | WU-06 GVR |
| TS-33 | WU-11 | pass (5 new tests; mutation-verified at build-and-verify) | WU-11 GVR |
| TS-34 | WU-02, WU-10 | partial: own migration table and fresh database pass; "upgrade beside posts and seo" stays an open residual (RK-15 descoped) | WU-10 known-limitations item 1 |
| TS-35, TS-38, TS-46, TS-52 panel | WU-13 | pass | WU-13 GVR |
| TS-52 client | WU-12 | pass | WU-12 GVR |
| TS-66 | WU-02 | unit-green (audit rows immutable, down refuses with audit rows) | `D/red-tests/WU-02/failing-tests.md` |
| TS-39, TS-40, TS-41 (API), TS-58 | WU-15 | recorded, section 2 | this record |
| TS-41 (UI) | WU-15 | BLOCKED (E2E-ENV-01) | section 2 |

### 4. Threat to outcome

| T | TS and outcome | Evidence files |
|---|---|---|
| T-01 | TS-03 pass; TS-35 pass (UX only) | WU-10, WU-13 GVR |
| T-02 | TS-04 pass; TS-05a unit-green; TS-05b pass | WU-08, WU-05 GVR |
| T-03 | TS-06 pass; TS-38 pass; TS-45 pass; TS-46 pass; TS-63 pass | WU-08, WU-13 GVR |
| T-04 | TS-08a pass; TS-08b pass; TS-59 pass; TS-07 pass; TS-02 pass; TS-62 pass. Residual: a reverse proxy in front of a loopback bind is undetected (compose stack sets `HOST=0.0.0.0`, so backup is disabled there) | WU-01, WU-08, WU-10 GVR; WU-10 known-limitations 5 |
| T-05 | TS-09 pass; TS-10 pass; TS-33 pass; TS-41 API pass and UI BLOCKED; TS-43 pass; TS-47 pass; TS-53 pass; TS-64 pass | WU-09, WU-11, WU-05 GVR; section 2 |
| T-06 | TS-11 pass; TS-12a pass; TS-12b pass; TS-41 API pass, UI BLOCKED; TS-47 pass; TS-49 pass | WU-01, WU-14, WU-08 GVR |
| T-07 | TS-13 pass; TS-14a unit-green; TS-14b pass; TS-52 pass (client WU-12, panel WU-13); TS-67 unit-green | WU-07, WU-12, WU-13 GVR |
| T-08 | TS-15 pass; TS-16 pass; TS-65 pass (store WU-06, lifecycle WU-17/18/19); TS-66 unit-green; TS-69 unit-green | WU-07, WU-06 GVR |
| T-09 | TS-17 pass (API WU-08 plus bundle scan WU-15); TS-44a pass; TS-44b pass; TS-49 pass; TS-57 pass; TS-66 unit-green | WU-08, WU-07, WU-06 GVR; `evidence/green-wu15-bundle-canary-scan.log` |
| T-10 | TS-18 pass; TS-12a pass | WU-14 GVR |
| T-11 | TS-19 pass; TS-32 pass (lock WU-04, runtime WU-19) | WU-04 GVR |
| T-12 | TS-20..TS-29 pass (TS-27a, TS-27b); TS-32 pass; TS-47 pass; TS-48 pass; TS-50 pass; TS-67 unit-green; TS-68 unit-green | WU-17/18/19, WU-05, WU-14 GVR |
| T-13 | TS-30 pass; TS-01, TS-02 pass; TS-14a unit-green, TS-14b pass; TS-27a, TS-27b pass; TS-36 pass; TS-40 recorded (section 2); TS-42 pass; TS-48 pass; TS-50 pass; TS-55 pass; TS-56 pass; TS-57 pass; TS-60 pass; TS-61 pass; TS-68 unit-green. Residual: slow header delivery is handled by hyper before the router (WU-10 known-limitations 3, 4) | WU-04, WU-10, WU-08, WU-14 GVR; section 2 |
| T-14 | TS-31 PARTIAL on Windows (owner-only ACL not verified; owner-accepted synthetic-only residual, rechecked at the release-candidate gate, Ubuntu CI follow-up); TS-02 pass. T-14 is recorded as partial | WU-04 GVR; implementation-plan 3.1 |

### 5. `succeeded` is unreachable (T-06)

- State machine guard: `domain/state_machine.rs` and the pure `TS-11` tests (WU-01 GVR).
- Database: constraint `backup_jobs_u1_status_guard` (`NOT (operation = 'backup_db_slice' AND status IN ('succeeded','recovery-required'))`) in `m20261122_000001_backup_core.rs`, tested by TS-11 DB part (WU-02).
- Serializer: `JobResponse::from_row` projects through `project_status` and refuses `succeeded` and `recovery-required` for U1 jobs (TS-12b, TS-49, WU-08). In the WU-15 runs every job ended `incomplete` or `failed` (TS-39, TS-40, TS-41) and the TS-41 test asserts the status is not `succeeded`.
- UI: renders every status through exhaustive handling and always denies download (TS-12a, WU-14).
- Worker: no path ends `succeeded` or `recovery-required` (`no_path_ends_succeeded_or_recovery_required`, WU-05).

### 6. State and risk records (no claims beyond the evidence)

- **CI (RK-09):** `ci-my-cms.yml` runs `cargo test --all --all-features --no-fail-fast` without `--ignored`, so the Docker-backed tests (most of the backup suite) do not run in CI, and it does not run web tests, lint or build. No CI run was observed in this work; green CI is NOT claimed.
- **Backup rollback limit (RK-08):** `migrate down` and `migrate status` cover the posts migrator only; backup identities appear in `migrate --list`. There is no CLI rollback path for backup tables (WU-10 GVR RK-08 row; README).
- **Performance:** numbers in section 2 only; no target exists (P-01..P-05 open).
- **Owner decisions of 2026-11-28 (recorded in `D/build-and-verify/WU-10/implementation.md`):** CFG-17 = 30 (`BACKUP_REQUEST_TIMEOUT_SECS`), CFG-18 = 16384 (`BACKUP_MAX_HEADER_BYTES`), RK-15 descoped as a residual (gateway boot with posts, seo and backup on one database not exercised; `domain_seo` and `domain_posts` untouched), RK-16 keep U1 off `main` (migrations register unconditionally, so an image from this branch would create `backup_*` tables on every start).
- **Still-unconfirmed AI-recorded owner decisions:** the CLI in AI mode records them as "AI Orchestrator", so the gate lists them as unconfirmed: (1) WU-17 scheduling: the owner directed in chat on 2026-11-26 to run WU-17, WU-18, WU-19 after WU-07 (revisit of the earlier owner-decision park); (2) WU-08 revisit: the owner requested on 2026-11-27 to revisit WU-08 and finish its remaining tests within its own scope (`D/../state.json` revisit records for WU-17 and WU-08).
- **Other open items for the gate:** E2E-ENV-01 (browser verification), TS-31 on a Unix host or Ubuntu CI, TS-34 upgrade beside posts and seo, manual assistive-technology review (TS-37), Q-04, Q-06, Q-07, Q-10, Q-11, Q-12, audit retention, non-owner guards.
