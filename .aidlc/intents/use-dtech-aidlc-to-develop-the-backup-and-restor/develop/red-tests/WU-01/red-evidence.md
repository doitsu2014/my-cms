# RE-01 Red evidence

- **ID:** RE-01
- **Stage:** Develop / red-tests (builder-verifier). Repeats once per work unit; one section per unit.
- **Status:** Baseline and WU-01 red run recorded 2026-11-22. Later units not started.
- **Companion:** [failing-tests.md](failing-tests.md) (per-test manifest). Raw logs in [evidence/](../evidence/).
- **Inputs:** [implementation-plan](../../implementation-design/implementation-plan.md), [test-strategy](../../implementation-design/test-strategy.md) (baseline procedure, WU-01 concrete red tests), [work-units](../../implementation-design/work-units.md), [acceptance-criteria](../../../analyze/problem-framing/acceptance-criteria.md).

Provenance: everything under "Observed" was run by the conductor on 2026-11-22 on this host (Windows 11 Pro 10.0.26300, cargo/rustc 1.100.0, pnpm 10.30.3, node v24.18.0) and the output is saved in `evidence/`. Nothing here is a green or passing claim for new behaviour.

## 1. Owner decisions received at the start of this stage

Project owner (solo scope), 2026-11-22, via conductor AskUserQuestion. User-provided; these supplement PLAN-01 section 3 (the implementation-design artifacts were already reported and are not edited after the fact).

| Decision | Choice | Effect on units |
|---|---|---|
| CFG-04 fixture storage and T-14 | Dedicated directory set by config (no default), private permissions on Unix; on Windows the missing owner-only ACL check is accepted for synthetic-only data and recorded as a T-14 residual for the release-candidate gate | Lifts the CFG-04 hard stop on WU-04..WU-06 and the T-14 ACL hard stop on WU-04 |
| RK-16 release effect | Branch-only until Launch: all U1 work stays on this feature branch; no merge to `main` during Develop; the merge/release decision moves to the Launch phase with its own approval | WU-02/WU-10 "no merge to main" stop becomes a standing rule for the whole of Develop |
| RK-15 shared migration table | Verify at baseline first; if confirmed, bring fix options to the owner before WU-02; do not change posts or seo meanwhile | See section 2.4: NOT yet verified (Docker unavailable) |
| E2E-ENV-01 browser E2E | Same-origin dev proxy (rsbuild `server.proxy`, dev only); TS-07 no-CORS rule stays; CFG-16 set to the dev origin | Unblocks WU-13/WU-14 browser verification design |

Still open (unchanged): CFG-17/CFG-18 owner values (needed before WU-10), Q-04, Q-06, Q-07, Q-10, Q-11.

## 2. Baseline (existing suites, before any code change)

Taken before `domain_backup` existed (git HEAD `69704de`, working tree changes limited to `.aidlc/`).

### 2.1 Rust workspace (Observed)

Command, from `apps/api`: `cargo test --no-fail-fast` — log [evidence/baseline-cargo-test.log](../evidence/baseline-cargo-test.log). Exit 101.

| Crate (unit tests) | Passed | Failed | Ignored |
|---|---|---|---|
| domain_auth | 29 | 0 | 0 |
| domain_interface | 9 | 0 | 0 |
| domain_media | 101 | 0 | 0 |
| domain_posts | 42 | 8 | 2 |
| domain_seo | 10 | 0 | 1 |
| domain_user | 36 | 0 | 0 |
| gateway (`my_cms_api`) | 17 | 0 | 0 |
| test_helpers, doc-tests, other targets | 0 | 0 | 0 |
| **Total** | **244** | **8** | **3** |

Baseline failing set (all in `domain_posts`, all panic at `test_helpers/src/lib.rs:20:53`, which is `Postgres::default().start().await.unwrap()` — testcontainers cannot reach Docker):

1. `handlers::tag_helper::create::create_handler::tests::handle_create_tags_in_transaction_test01`
2. `handlers::category::create::create_handler::tests::handle_create_cartegory_testcase_parent`
3. `handlers::tag_helper::read::read_handler::tests::handle_read_tags_test01`
4. `handlers::category::read::category_read_handler::tests::handle_get_all_cartegory_testcase_01`
5. `handlers::category::create::create_handler::tests::handle_create_cartegory_testcase_01`
6. `handlers::category::modify::modify_handler::tests::handle_modify_category_testcase_successfully`
7. `handlers::category::modify::modify_handler::tests::handle_modify_category_testcase_failed_due_to_rowversion`
8. `handlers::category::delete::delete_handler::tests::handle_delete_categories_with_children`

This confirms the historical "8 domain_posts failures" and the test-strategy assumption about their cause (environmental, Docker). Ignored count is 3, matching the source grep in test-strategy step 4.

Also: `cargo fmt -- --check` exit 0 ([log](../evidence/baseline-cargo-fmt.log)); `cargo clippy` exit 0 with warnings only ([log](../evidence/baseline-cargo-clippy.log)). `cargo test -- --ignored` was NOT run (Docker unavailable).

### 2.2 Admin web (Observed)

From `apps/web`, after `pnpm install --frozen-lockfile` (exit 0; lockfile unchanged; `node_modules` was absent):

| Check | Command | Result | Log |
|---|---|---|---|
| Unit tests | `pnpm test` | exit 0; 19 files, 49 tests passed | [baseline-web-test.log](../evidence/baseline-web-test.log) |
| Build | `pnpm build` | exit 0 | [baseline-web-build.log](../evidence/baseline-web-build.log) |
| Lint | `pnpm lint` | exit 2 (pre-existing): ESLint flat-config error for the `react-hooks` plugin object | [baseline-web-lint.log](../evidence/baseline-web-lint.log) |
| Typecheck | `pnpm exec tsc --noEmit` | exit 2 (pre-existing): 23 errors, all in `src/app/admin/components/inputs/rich-text-editor/**` and `src/app/admin/seo/head-assets/head-asset-form.tsx` | [baseline-web-tsc.log](../evidence/baseline-web-tsc.log) |

Regression rule for later units: lint and typecheck failures above are baseline; the U1 web units must add no new lint or tsc error in files they touch, and the failing set must not grow.

### 2.3 Docker (Observed)

`docker info` reports "Docker Desktop is unable to start" after launching Docker Desktop on this host; no local PostgreSQL is installed. Consequences: every testcontainers-backed test (the 8 baseline failures above, the 3 ignored tests, and all future `domain_backup` DB tests marked `#[ignore = "requires Docker for PostgreSQL testcontainer"]`) is **not run** on this host until Docker works. They are never reported as passed.

### 2.4 RK-15 check (NOT run)

The owner chose "verify at baseline first". Verification needs PostgreSQL (run posts then seo migrations on one database and observe whether seo's `up` fails with "Migration file of version ... is missing"). It could not run: Docker unavailable, no local PostgreSQL. **Status: unverified.** It remains a hard stop before WU-02's "upgrade beside posts and seo" case, WU-10 runtime start, and TS-41. Next step: the owner restores Docker Desktop, then the conductor runs the check and returns with fix options if confirmed.

## 3. WU-01 red run

### 3.1 Observed

- Command, from `apps/api`: `cargo test -p domain_backup --no-fail-fast` — log [evidence/red-wu01-cargo-test.log](../evidence/red-wu01-cargo-test.log).
- Result: exit 101; **23 ran, 22 failed, 1 passed, 0 ignored**; doc-tests 0.
- Every failure is a panic at an assertion line inside a `#[cfg(test)]` module (config.rs, state_machine.rs, status.rs), not a compile error and not a `todo!()`: `grep todo` finds no `todo!` in the crate (one comment in `lib.rs`).
- The one pass, `status::tests::all_lists_every_variant_once`, is a structural guard and is not counted as red evidence.
- One test, `config::tests::storage_root_must_be_private_on_unix`, is `#[cfg(unix)]` and was not compiled on this host: not run.
- Workspace still builds: `cargo check --workspace` exit 0 ([log](../evidence/red-wu01-cargo-check-workspace.log)); warnings only (unused `thiserror` in `domain_backup`, existing resolver notice, existing `domain_user` manifest warnings).
- Scope check (`git status --short`): only `apps/api/Cargo.toml`, `apps/api/Cargo.lock`, `apps/api/domain_backup/**` and `.aidlc/**` changed.
- Stub bodies were read by the conductor and contain no behaviour (empty string, `true`, `Ok(())`, zero/empty `Enabled`, plain serde derive). The red therefore shows each assertion rejects a permissive implementation, which is the WU-01 rule in test-strategy.

Per-test rows with TS, AC, T and failing line: [failing-tests.md](failing-tests.md#wu-01-crate-scaffold-domain-types-config-clock-errors).

### 3.2 How the red was produced (process record)

A dynamic workflow ran: one builder (sonnet, the directive's `stage.model`) wrote the shell and tests; two independent reviewers (spec fidelity, red integrity) returned 19 findings (6 major); the builder applied 14, rejected 1 (uppercase-scheme and space-after-comma origin cases: design silent) and deferred 4 (listed in failing-tests). The first draft used `todo!()` stubs (16 of 17 reds were panics); this was corrected to the permissive-stub rule before the recorded run. The conductor then ran the suite independently; section 3.1 is that run.

### 3.3 Design decisions made during red, for build-and-verify

These are not in the design artifacts; recorded here so the build does not have to guess. Conductor decisions [D] unless marked otherwise.

| Topic | Decision | Test that pins it |
|---|---|---|
| disabled_reason precedence | NOT_CONFIGURED (flag not exactly `true`) > NON_LOOPBACK_BIND > INVALID_CONFIG. Rationale: report the most fundamental cause first; a non-loopback bind is a security boundary and must never be masked by a config typo | `not_configured_takes_precedence`, `non_loopback_bind_takes_precedence_over_invalid_config` |
| `BACKUP_ENABLED` spelling | only the exact string `true` enables; `TRUE`, `1`, `yes`, padded values do not | `disabled_unless_backup_enabled_is_true` |
| Numeric spelling | canonical unsigned decimal only; `+N`, leading zeros, padding and overflow are INVALID_CONFIG | `non_canonical_and_overflowing_numbers_are_invalid` |
| CFG-15 escalation owner | build implements: any value other than `project_owner` gives INVALID_CONFIG (same rule as CFG-05..CFG-14). The test currently accepts either "ignored" or INVALID_CONFIG; build must choose INVALID_CONFIG | `escalation_owner_cannot_be_overridden` |
| Loopback forms | IP literals in 127.0.0.0/8 and `::1` only; `[::1]` and `::ffff:127.0.0.1` are not loopback for the bind check | `is_loopback_bind_accepts_only_loopback_ip_literals` |
| Config loading | `BackupConfig::load(lookup)` with an injected key lookup and a thin `from_env()`; no process-env mutation in tests (deviation from the test-strategy `ENV_LOCK`/`test_support.rs` idea; equivalent coverage, no shared global state) | all config tests |
| `thiserror` | declared, unused at red; build either uses it for `AppError` or removes it | n/a (cargo warning) |

### 3.4 Work-unit registration correction (process record)

The implementation-design stage omitted `work-units.json`, which the work-units template requires beside `work-units.md` and which the CLI reads to drive the per-unit loop. Without it the CLI treated the whole phase as one implicit unit, and this red run was first reported under that implicit unit (`_`) at 2026-11-22T07:12Z. The conductor then added `develop/implementation-design/work-units.json` (WU-01..WU-15, derived from the "Depends on" lines of work-units.md; no content change to the design), the CLI re-selected red-tests for WU-01 with per-unit paths, and these files were moved to `develop/red-tests/WU-01/` unchanged except for relative links. The red run itself (section 3.1) was not repeated or altered.

## 4. Gaps and limits

- No new behaviour is implemented; nothing is green.
- Docker-dependent baseline and RK-15 are not run (section 2.3, 2.4).
- Unix-only privacy test not compiled here; Windows ACL is an owner-accepted residual (T-14 partial).
- AC/T mappings marked `~` in failing-tests are nearest-criterion assumptions.
- Acceptance criteria with no WU-01 red test are covered by later units per work-units.md; this stage iteration covers WU-01 only.
