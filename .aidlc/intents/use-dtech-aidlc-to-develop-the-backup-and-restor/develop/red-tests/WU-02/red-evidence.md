# RE-02 Red evidence: WU-02

- **ID:** RE-02 (WU-02)
- **Stage:** Develop / red-tests (builder-verifier), unit WU-02.
- **Status:** Baseline with Docker, RK-15 prerequisite fix and WU-02 red run recorded on 2026-11-22.
- **Companion:** [failing-tests.md](failing-tests.md). Raw logs are in [../evidence/](../evidence/).
- **Earlier unit:** [../WU-01/red-evidence.md](../WU-01/red-evidence.md) (baseline without Docker, owner decisions from the start of red-tests).

Provenance: everything under Observed was run by the conductor on 2026-11-22 on the Windows 11 dev host (cargo 1.100.0, Docker 29.7.2), with logs saved under `evidence/`.

## 1. History of this stage

1. 2026-11-22 07:26Z: red-tests for WU-02 was reported **blocked**. WU-02 needs PostgreSQL, Docker Desktop would not start, and the owner had asked for RK-15 to be checked before WU-02.
2. Docker was restored by the owner. The conductor resumed with `orchestrate next`, which re-selected red-tests for WU-02.

## 2. Baseline with Docker (Observed)

Taken after WU-01 (commit `9dbce50`) and before any RK-15 or WU-02 change. Logs: [baseline-docker-cargo-test.log](../evidence/baseline-docker-cargo-test.log) and [baseline-docker-cargo-test-ignored.log](../evidence/baseline-docker-cargo-test-ignored.log).

| Crate | Passed / failed / ignored | `-- --ignored` |
|---|---|---|
| domain_auth | 29 / 0 / 0 | none |
| domain_backup | 35 / 0 / 0 | none |
| domain_interface | 9 / 0 / 0 | none |
| domain_media | 101 / 0 / 0 | none |
| domain_posts | 50 / 0 / 2 | 2 passed |
| domain_seo | 10 / 0 / 1 | 1 passed |
| domain_user | 36 / 0 / 0 | none |
| gateway (`my_cms_api`) | 17 / 0 / 0 | none |
| **Total** | **287 / 0 / 3** | **3 / 3 passed** |

With Docker the 8 `domain_posts` failures seen in the no-Docker baseline pass. That confirms they were environmental. The existing `domain_seo` integration test passes because it runs SEO's migrations on their own, which is why it never caught RK-15.

## 3. Prerequisite: RK-15 fix (outside the U1 work units)

### 3.1 Owner decisions (User-provided, project owner, 2026-11-22, via AskUserQuestion)

- RK-15 handling: **"Fix seo first, own table."** A separate, red-first prerequisite change on this branch, before WU-02. Posts is untouched.
- Production impact: **the owner checks production** (which API image runs and whether it starts). The conductor touches no deployment.

### 3.2 Confirmation (Observed)

[rk15-check.log](../evidence/rk15-check.log): a throwaway `pgvector/pgvector:pg15` container, the real `my-cms-api migrate up` run twice, with dummy values for the non-database settings.

- **Run 1 fails:** posts records its 4 versions in `seaql_migrations`. SEO's `Migrator::up` then refuses with `Migration file of version 'm20240409_151952_release_100' is missing, this migration has been applied but its file is missing`, and the same for the other 3 versions.
- **Run 2 fails the same way.** SEO's tables are never created.

The gateway runs the same orchestrator at startup, so the API cannot start against any database where posts migrations ran. The container was removed afterwards.

### 3.3 Red (Observed, [red-rk15.log](../evidence/red-rk15.log))

| Test | Where | Red reason |
|---|---|---|
| `migrations::tests::seo_uses_its_own_migration_table` | `domain_seo/src/migrations/mod.rs` | assertion: table is `seaql_migrations` |
| `migration_tests::posts_and_seo_migrations_apply_on_fresh_database_and_second_boot` | `gateway/src/migration_tests.rs` (new, ignored) | first boot fails with the RK-15 "missing" error |
| `migration_tests::database_with_only_posts_applied_upgrades_cleanly` | same | boot on a posts-only database fails with the RK-15 error |

The first attempt at the gateway tests failed for the wrong reason, a connection error from the shared `test_helpers` pgvector helper's init-restart race. That run was not counted. The tests were changed to open their own connection with a retry and no side-effect migrations before the red was recorded.

### 3.4 Fix and green (Observed)

- **Fix:** `domain_seo` `Migrator::migration_table_name()` returns `seaql_migrations_seo`. That is one override in `apps/api/domain_seo/src/migrations/mod.rs`. Gateway change: a test-only `mod migration_tests;` line in `main.rs`.
- **Green** ([green-rk15.log](../evidence/green-rk15.log)):
  - `domain_seo` 11 passed, 0 failed, 1 ignored.
  - The gateway `migration_tests` pass 2 of 2.
  - SEO's existing integration test passes.
  - The red test files are byte-identical to the red snapshot.
- **End to end** ([rk15-after-fix.log](../evidence/rk15-after-fix.log)): the real `migrate up` exits 0 on run 1 and run 2 against a fresh database. `seaql_migrations` holds exactly the 4 posts versions, and `seaql_migrations_seo` holds `m20260822_000001_seo_head_assets`.
- **Regression** ([regression-rk15-cargo-test.log](../evidence/regression-rk15-cargo-test.log), [ignored](../evidence/regression-rk15-cargo-test-ignored.log)):
  - Workspace 288 passed, 0 failed, 5 ignored (baseline 287/0/3; +1 SEO unit test, +2 gateway DB tests).
  - `-- --ignored` 5 of 5 pass.
  - `cargo fmt --check` and `cargo clippy --all-targets` exit 0.
  - No test that passed at baseline fails.

### 3.5 Departure from the approved option, and limits

- **No row move.** The option as worded also moved any SEO rows out of the shared table. That step was not implemented, because no reachable database has them: the orchestrator always runs posts before SEO, and SEO has always failed at that point. A database where SEO somehow ran first would make the posts step fail first, so moving rows on SEO's side could not rescue it. The posts-only upgrade test covers the real state of existing databases. This is reported to the owner for confirmation.
- **`migrate status` / `migrate down`** only delegate to the posts runner and print "status unavailable". This was already the case before the fix, it is unrelated to RK-15, and it is tracked for WU-10 (RK-08).
- **Shared `test_helpers` gotchas**, not changed here:
  - The pgvector helper can report ready during PostgreSQL's init restart.
  - `get_database_connection()` applies posts migrations as a side effect.

  Both are recorded in `PROJECT.md`.
- **Production:** not checked by the conductor. The owner owns that check (section 3.1).

## 4. WU-02 red run (Observed)

- **Command,** from `apps/api`: `cargo test -p domain_backup --no-fail-fast`, then the same with `-- --ignored`. Log: [red-wu02-cargo-test.log](../evidence/red-wu02-cargo-test.log).
- **No-Docker run:** 36 passed, 1 failed (`backup_uses_its_own_migration_table`), 33 ignored.
- **Docker run:** 33 failed, 0 passed (about 52 s). The failures are schema absence (missing relations, failed positive-control inserts, missing trigger and index), plus the shared-table error for the posts/seo coexistence test. None are connection or compile errors.
- **Database:** `postgres:15-alpine`, matching production's `supabase/postgres:15.14`.
- **Stability:** the workspace still builds (`cargo check --workspace` exit 0, [log](../evidence/red-wu02-cargo-check-workspace.log)), and the WU-01 tests still pass.
- **Snapshot:** taken after `cargo fmt` (the lesson from WU-01 L-07), so later formatting cannot touch the red tests.

### 4.1 How the red was produced

A dynamic workflow ran one builder (sonnet, `stage.model`), two independent reviewers (spec fidelity; red integrity, which ran the Docker tests) and a fixer. The reviewers returned 21 findings, including 1 critical: the idempotency test could never pass. The fixer applied them, adding:

- exhaustive status-transition pairs;
- a 000001 to 000002 upgrade test;
- a behavioural D-10 test;
- revert through `migrations_cli`;
- allocation positive controls;
- constraint matching by definition instead of name.

The conductor then made the two changes in section 4.2 and recorded the run in section 4 independently.

### 4.2 Design decisions made during red

| Id | Decision | Why |
|---|---|---|
| D-14 (conductor) | `backup_audit` append-only also blocks `TRUNCATE` (statement-level guard; message contains `backup_audit_append_only`). New test `audit_cannot_be_truncated`. | A row-level UPDATE/DELETE trigger does not fire on TRUNCATE, so the contract's append-only rule (T-08) had a hole. This adds to IFC section 7. |
| D-15 (conductor) | DB tests run on PostgreSQL 15 (`postgres:15-alpine`), not the module default (11). | Production runs `supabase/postgres:15.14`. The plan requires migrations proven on the real version. |
| builder [D] | CHECK constraints and the concurrency index are matched by definition, not name. Trigger names `backup_jobs_terminal_immutable` and `backup_audit_append_only` are contract, and their messages contain the trigger name. | IFC fixes trigger names but not CHECK names |
| builder [D] | Allowed status edges are exactly queued to running, queued to failed, running to incomplete and running to failed. A terminal row is immutable, including non-status columns. A non-terminal self-set is a no-op. | F-15 / ADR-002 |
| builder [D] | `down` refuses while `backup_audit` has rows or any allocation is not `deleted`. With no audit rows, it drops everything except `backup_audit` and its trigger, so a later `up` must recreate idempotently (`IF NOT EXISTS`, `DROP TRIGGER IF EXISTS`). | TS-66, IFC 5.6 |
| builder [D] | `down(Some(1))` removes only the jobs trigger | per-migration revert |

## 5. Gaps and limits

- No new behaviour is implemented; nothing is green for WU-02.
- Most DB tests fail first on missing schema, so their target assertions must get mutation checks in build-and-verify (failing-tests, last section).
- Composed evidence (backup registered in the gateway beside posts and SEO) comes in WU-10. WU-02 proves coexistence by pre-populating the posts and SEO version tables, with no dependency on other domain crates.
- Release effect (RK-16): the owner's branch-only rule stands. No merge to `main`.
