# WU-08 regression report

- **Status:** Full workspace normal and Docker-backed ignored suites pass on the final WU-08 tree (2026-11-28, macOS).
- **Comparison baseline:** `develop/baseline.md` (2026-11-22, Windows: 290 normal passed; 39/39 ignored passed). Counts have grown with later units; no previously passing test fails.
- **Attribution:** `regression.json`, scope `full`, empty failures array.

## Runs (from `apps/api`)

| Command | Exit | Result | Log |
|---|---:|---|---|
| `cargo test --workspace --no-fail-fast` | 0 | 404 passed, 0 failed, 208 ignored | `evidence/regression-wu08-macos-normal.log` |
| `cargo test --workspace --no-fail-fast -- --ignored` | 0 | 208 passed, 0 failed | `evidence/regression-wu08-macos-ignored.log` |

## Observations, not omitted

- Before the test-only temp-dir fix, macOS runs failed 18 normal tests (WU-01 `domain::config`, WU-08 `domain::instance`/`domain::policy`) and 4 WU-08 Docker HTTP tests with `INVALID_CONFIG`, caused by the OS temp dir sitting behind `/var -> /private/var`. Fixed in `#[cfg(test)]` code; production CFG-04 validation unchanged.
- One intermediate Docker workspace run failed `infra::tests::runtime::a_runtime_that_was_never_started_refuses_ticks_and_recovery` with testcontainers `PortNotExposed` at container start (infrastructure). The same test passed in the package run before it and in both later full runs. Log kept: `evidence/regression-wu08-macos-ignored-run1-infra-flake.log`.
