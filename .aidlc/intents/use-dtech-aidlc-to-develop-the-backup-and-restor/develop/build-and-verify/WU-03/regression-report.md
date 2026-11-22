# REG-03 Regression report: WU-03

- **ID:** REG-03 (WU-03)
- **Stage:** Develop / build-and-verify, unit WU-03.
- **Status:** No regression. The full suite, including all Docker-backed tests, was run against the recorded baseline.
- **Baseline:** [../../baseline.md](../../baseline.md), recorded once for dtech-aidlc 0.7.0 from the post-WU-02 run.
- **After:**
  - [regression-wu03-cargo-test.log](../evidence/regression-wu03-cargo-test.log)
  - [ignored](../evidence/regression-wu03-cargo-test-ignored.log)
  - [fmt](../evidence/regression-wu03-cargo-fmt.log)
  - [clippy](../evidence/regression-wu03-cargo-clippy.log)

Provenance: Observed. Conductor runs on 2026-11-22, same host, toolchain and Docker as the baseline.

## Rust workspace

Command, from `apps/api`: `cargo test --no-fail-fast`, then the same with `-- --ignored`.

| Crate | Baseline (pass / fail / ignored) | After WU-03 | `--ignored` after WU-03 |
|---|---|---|---|
| domain_auth | 29 / 0 / 0 | 29 / 0 / 0 | none |
| domain_backup | 37 / 0 / 34 | 41 / 0 / 77 | 77 passed |
| domain_interface | 9 / 0 / 0 | 9 / 0 / 0 | none |
| domain_media | 101 / 0 / 0 | 101 / 0 / 0 | none |
| domain_posts | 50 / 0 / 2 | 50 / 0 / 2 | 2 passed |
| domain_seo | 11 / 0 / 1 | 11 / 0 / 1 | 1 passed |
| domain_user | 36 / 0 / 0 | 36 / 0 / 0 | none |
| gateway (`my_cms_api`) | 17 / 0 / 2 | 17 / 0 / 2 | 2 passed |
| **Total** | **290 / 0 / 39** | **294 / 0 / 82** | **82 / 82 passed** |

- **Differences:** `domain_backup` gains 4 unit tests and 43 DB tests (41 from red-tests, 2 test-after), all passing. Every other crate is unchanged.
- **Failing set:** empty before and after.
- **Ignored tests:** every new one carries the reason `requires Docker for PostgreSQL testcontainer`.

| Check | Baseline | After WU-03 |
|---|---|---|
| `cargo fmt -- --check` | exit 0 | exit 0 |
| `cargo clippy --all-targets` | exit 0, existing warnings in other crates | exit 0, no warning attributed to `domain_backup` |
| `dtech-aidlc ids check` | n/a | ok, 0 unregistered |

## Admin web

Not re-run, because WU-03 changes no file under `apps/web`. The baseline in `baseline.md` stands.

## Unverified

- Unix-only WU-01 path (`NotPrivate`): still not compiled here.
