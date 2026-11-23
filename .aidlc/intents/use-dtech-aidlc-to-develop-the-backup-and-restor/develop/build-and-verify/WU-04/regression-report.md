# REG-04 Regression report: WU-04

- **ID:** REG-04 (WU-04)
- **Stage:** Develop / build-and-verify, unit WU-04.
- **Status:** No regression. The full suite, including all Docker-backed tests, was run against the recorded baseline.
- **Baseline:** [../../baseline.md](../../baseline.md)
- **After:**
  - [regression-wu04-cargo-test.log](../evidence/regression-wu04-cargo-test.log)
  - [ignored](../evidence/regression-wu04-cargo-test-ignored.log)
  - [fmt](../evidence/regression-wu04-cargo-fmt.log)
  - [clippy](../evidence/regression-wu04-cargo-clippy.log)

Provenance: Observed in conductor runs on 2026-11-22, using the same host, toolchain and Docker as the baseline.

## Rust workspace

Command, from `apps/api`: `cargo test --no-fail-fast`, then the same with `-- --ignored`.

| Crate | Baseline (pass / fail / ignored) | After WU-04 | `--ignored` after WU-04 |
|---|---|---|---|
| domain_auth | 29 / 0 / 0 | 29 / 0 / 0 | none |
| domain_backup | 37 / 0 / 34 | 82 / 0 / 105 | 105 passed |
| domain_interface | 9 / 0 / 0 | 9 / 0 / 0 | none |
| domain_media | 101 / 0 / 0 | 101 / 0 / 0 | none |
| domain_posts | 50 / 0 / 2 | 50 / 0 / 2 | 2 passed |
| domain_seo | 11 / 0 / 1 | 11 / 0 / 1 | 1 passed |
| domain_user | 36 / 0 / 0 | 36 / 0 / 0 | none |
| gateway (`my_cms_api`) | 17 / 0 / 2 | 17 / 0 / 2 | 2 passed |
| **Total** | **290 / 0 / 39** | **335 / 0 / 110** | **110 / 110 passed** |

- **Differences from baseline:**
  - `domain_backup` gained tests from WU-03 and WU-04, all passing:
    - WU-03: 4 unit and 43 DB tests.
    - WU-04: 41 unit/filesystem tests and 28 DB tests, including the two conductor additions.
  - Every other crate is unchanged.
- **Failing set:** empty before and after.
- **Ignored tests:** every new one carries the reason `requires Docker for PostgreSQL testcontainer`.
- **Temp directories:** no `makara-wu04-*` directories remain after the run.

| Check | Baseline | After WU-04 |
|---|---|---|
| `cargo fmt -- --check` | exit 0 | exit 0 |
| `cargo clippy --all-targets` | exit 0, existing warnings in other crates | exit 0, no warning attributed to `domain_backup` |
| `dtech-aidlc ids check` | n/a | ok, 0 unregistered |

## Admin web

Not re-run: WU-04 changes no file under `apps/web`.

## Unverified on this host

- Four `cfg(unix)` permission tests (TS-31): not compiled on Windows.
- Four file-symlink tests: skipped, because creating a file symlink needs a privilege this host lacks.
- Both sets were builder-reported red on Linux at red-tests. Neither has been run green yet. CI on Ubuntu must run them, including `--ignored`, before the release-candidate gate.
