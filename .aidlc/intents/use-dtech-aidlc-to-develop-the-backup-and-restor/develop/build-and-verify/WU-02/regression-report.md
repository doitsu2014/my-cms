# REG-02 Regression report: WU-02

- **ID:** REG-02 (WU-02)
- **Stage:** Develop / build-and-verify, unit WU-02.
- **Status:** No regression. The full suite, including Docker-backed tests, ran before and after.
- **Baseline:** [red-tests/WU-02/red-evidence.md](../../red-tests/WU-02/red-evidence.md) section 2. That baseline used Docker, was taken after WU-01 and before RK-15 or WU-02. The intermediate state after the RK-15 fix is in section 3.4 of the same file.
- **After:** [evidence/regression-wu02-cargo-test.log](../evidence/regression-wu02-cargo-test.log), [ignored](../evidence/regression-wu02-cargo-test-ignored.log), [fmt](../evidence/regression-wu02-cargo-fmt.log), [clippy](../evidence/regression-wu02-cargo-clippy.log).

Provenance: Observed, conductor runs 2026-11-22, same host and toolchain, Docker 29.7.2.

## Rust workspace

Command, from `apps/api`: `cargo test --no-fail-fast`, then the same with `-- --ignored`.

| Crate | Baseline (pass / fail / ignored) | After RK-15 fix | After WU-02 | `--ignored` after WU-02 |
|---|---|---|---|---|
| domain_auth | 29 / 0 / 0 | 29 / 0 / 0 | 29 / 0 / 0 | none |
| domain_backup | 35 / 0 / 0 | 35 / 0 / 0 | 37 / 0 / 34 | 34 passed |
| domain_interface | 9 / 0 / 0 | 9 / 0 / 0 | 9 / 0 / 0 | none |
| domain_media | 101 / 0 / 0 | 101 / 0 / 0 | 101 / 0 / 0 | none |
| domain_posts | 50 / 0 / 2 | 50 / 0 / 2 | 50 / 0 / 2 | 2 passed |
| domain_seo | 10 / 0 / 1 | 11 / 0 / 1 | 11 / 0 / 1 | 1 passed |
| domain_user | 36 / 0 / 0 | 36 / 0 / 0 | 36 / 0 / 0 | none |
| gateway (`my_cms_api`) | 17 / 0 / 0 | 17 / 0 / 2 | 17 / 0 / 2 | 2 passed |
| **Total** | **287 / 0 / 3** | **288 / 0 / 5** | **290 / 0 / 39** | **39 / 39 passed** |

Differences:
- `domain_backup`: +2 unit tests (table name, migrator list) and +34 DB tests, all passing.
- `domain_seo` +1 and gateway +2 ignored: these come from the RK-15 prerequisite, already recorded.

No test that passed at baseline fails now. The failing set is empty before and after. New ignored tests all use the exact reason string `requires Docker for PostgreSQL testcontainer`.

| Check | Baseline | After WU-02 |
|---|---|---|
| `cargo fmt -- --check` | exit 0 | exit 0 |
| `cargo clippy --all-targets` | exit 0, warnings in existing crates | exit 0; no warning attributed to `domain_backup` |

## Admin web

Not re-run: WU-02 changes no file under `apps/web`. The baseline is unchanged: 49/49 tests and build pass, with lint and tsc failing as they did before this work.

## Unverified

- Unix-only WU-01 path (`NotPrivate`): still not compiled here. CI on Ubuntu will compile and run it on a pull request to `main`.
