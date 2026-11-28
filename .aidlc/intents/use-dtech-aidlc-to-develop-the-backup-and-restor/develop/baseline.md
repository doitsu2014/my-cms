# Develop baseline

- **Purpose:** The existing-suite baseline that every later work unit's regression report compares against (dtech-aidlc 0.7.0 `context.baseline`). Recorded once, reused per unit.
- **Recorded:** 2026-11-22 by the conductor.
- **Source run:** the full regression after WU-02 (commit `293431c`). No application code changed between that commit and `9595740`, which added only the skill refresh, the ID registry and documentation. The run therefore matches the code WU-03 starts from.
- **Host:** Windows 11 Pro 10.0.26300, cargo/rustc 1.100.0, Docker 29.7.2, node v24.18.0, pnpm 10.30.3.

Earlier baselines kept for history:
- [red-tests/WU-01/red-evidence.md](red-tests/WU-01/red-evidence.md) section 2: before any change, no Docker.
- [red-tests/WU-02/red-evidence.md](red-tests/WU-02/red-evidence.md) section 2: with Docker, before RK-15 and WU-02.

## Rust workspace (Observed)

Commands from `apps/api`. Logs:
- [regression-wu02-cargo-test.log](build-and-verify/evidence/regression-wu02-cargo-test.log)
- [ignored](build-and-verify/evidence/regression-wu02-cargo-test-ignored.log)
- [fmt](build-and-verify/evidence/regression-wu02-cargo-fmt.log)
- [clippy](build-and-verify/evidence/regression-wu02-cargo-clippy.log)

| Crate | `cargo test --no-fail-fast` (pass / fail / ignored) | `-- --ignored` |
|---|---|---|
| domain_auth | 29 / 0 / 0 | none |
| domain_backup | 37 / 0 / 34 | 34 passed |
| domain_interface | 9 / 0 / 0 | none |
| domain_media | 101 / 0 / 0 | none |
| domain_posts | 50 / 0 / 2 | 2 passed |
| domain_seo | 11 / 0 / 1 | 1 passed |
| domain_user | 36 / 0 / 0 | none |
| gateway (`my_cms_api`) | 17 / 0 / 2 | 2 passed |
| **Total** | **290 / 0 / 39** | **39 / 39 passed** |

- **Failing set:** empty.
- **Ignored set:** 39 tests, all marked `requires Docker for PostgreSQL testcontainer`.
- `cargo fmt -- --check`: exit 0.
- `cargo clippy --all-targets`: exit 0, with existing warnings only in other crates (`domain_auth`, `domain_media`). None in `domain_backup`.

## Admin web (Observed, before any web change)

From `apps/web` (logs under [red-tests/evidence](red-tests/evidence/)):

| Check | Result |
|---|---|
| `pnpm test` | 19 files, 49 tests passed |
| `pnpm build` | passes |
| `pnpm lint` | fails: ESLint flat-config error for the `react-hooks` plugin (pre-existing) |
| `pnpm exec tsc --noEmit` | fails: 23 errors in `src/app/admin/components/inputs/rich-text-editor/**` and `src/app/admin/seo/head-assets/head-asset-form.tsx` (pre-existing) |

No backup unit has changed `apps/web` yet. Re-run this baseline before the first web unit (WU-12) if anything in `apps/web` changes on this branch in the meantime.

## Not baselined

- Public reader (`apps/ducth-dev-website`): not run; no backup unit touches it.
- Unix-only Rust paths: not compiled on Windows. CI (Ubuntu) compiles them on a pull request to `main`.

## Regression rule

- A test that passed here must still pass after each unit.
- The failing set must stay empty.
- New ignored tests must carry the Docker reason string.
- No new lint or typecheck errors in files a unit touches.
