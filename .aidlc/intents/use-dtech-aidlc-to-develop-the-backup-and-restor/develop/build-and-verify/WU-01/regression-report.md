# REG-01 Regression report: WU-01

- **ID:** REG-01 (WU-01)
- **Stage:** Develop / build-and-verify, unit WU-01.
- **Status:** No regression in the suites that could run on this host. Docker-dependent tests could not run before or after; they are unverified, not passing.
- **Baseline:** [red-tests/WU-01/red-evidence.md](../../red-tests/WU-01/red-evidence.md) section 2 (taken before any code change, HEAD `69704de`).
- **After:** [evidence/regression-wu01-cargo-test.log](../evidence/regression-wu01-cargo-test.log), [fmt](../evidence/regression-wu01-cargo-fmt.log), [clippy](../evidence/regression-wu01-cargo-clippy.log).

Provenance: Observed, conductor runs 2026-11-22, same host and toolchain as the baseline.

## Rust workspace

Command, from `apps/api`: `cargo test --no-fail-fast`. Exit 101 before and after, caused by the same failures.

| Crate | Baseline (pass/fail/ignored) | After WU-01 | Difference |
|---|---|---|---|
| domain_auth | 29 / 0 / 0 | 29 / 0 / 0 | none |
| domain_backup | did not exist | 35 / 0 / 0 | new crate |
| domain_interface | 9 / 0 / 0 | 9 / 0 / 0 | none |
| domain_media | 101 / 0 / 0 | 101 / 0 / 0 | none |
| domain_posts | 42 / 8 / 2 | 42 / 8 / 2 | none |
| domain_seo | 10 / 0 / 1 | 10 / 0 / 1 | none |
| domain_user | 36 / 0 / 0 | 36 / 0 / 0 | none |
| gateway (`my_cms_api`) | 17 / 0 / 0 | 17 / 0 / 0 | none |
| **Total** | **244 / 8 / 3** | **279 / 8 / 3** | +35 passing (new), 0 regressions |

Failing set comparison: the 8 failing test names after WU-01 are exactly the 8 baseline names. All are `domain_posts` handler tests that panic in `test_helpers` when starting a Postgres testcontainer without Docker. No test that passed at baseline fails now. The ignored set outside `domain_backup` did not grow (3 before, 3 after), and `domain_backup` adds no ignored tests.

| Check | Baseline | After | Difference |
|---|---|---|---|
| `cargo fmt -- --check` | exit 0 | exit 0 | none |
| `cargo clippy` | exit 0, warnings (baseline ran without `--all-targets`) | `cargo clippy --all-targets`: exit 0, no warnings in `domain_backup` | flags differ; no new warning attributed to the change |

## Admin web

Not re-run for WU-01: the unit changes no file under `apps/web`. The baseline was 49/49 tests passing and the build passing, with pre-existing lint and tsc failures. It will be compared when the web units (WU-12..WU-14) change files there.

## Unverified

- `cargo test -- --ignored` and all testcontainers-backed tests: Docker Desktop does not start on this host, both at baseline and now. That includes the 8 baseline failures, which pass only with Docker.
- Unix-only code paths in `domain_backup` (`NotPrivate`): not compiled here.
