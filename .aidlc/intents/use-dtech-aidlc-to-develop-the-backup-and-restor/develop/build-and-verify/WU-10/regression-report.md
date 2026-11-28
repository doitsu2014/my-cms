# WU-10 regression report

- **Status:** Full workspace normal and Docker-backed suites pass on the integrated tree (2026-11-28).
- **Baseline:** previous unit runs (WU-12): Rust 420 normal / 213 Docker. Now 439 / 216 (new gateway, domain_interface, domain_backup tests). No previously passing test fails; the three existing `compose_routers_*` tests and `migration_tests` stay green.

| Command (from `apps/api`) | Exit | Result | Log |
|---|---:|---|---|
| `cargo test --workspace --no-fail-fast` | 0 | 439 passed, 0 failed, 216 ignored | `evidence/regression-wu10-normal.log` |
| `cargo test --workspace --no-fail-fast -- --ignored --test-threads=4` | 0 | 216 passed, 0 failed | `evidence/regression-wu10-ignored.log` |

The web app is untouched by WU-10. Worker-side flaky Docker runs (container start-up failures under host load, unrelated tests) are kept as `evidence/worker-green-workspace-ignored-run*-flaky.log`.
