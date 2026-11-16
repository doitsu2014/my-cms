# Test Results

## Build Status

Passed for the existing API test command. The workflow-equivalent coverage-instrumented build was not successful on this Windows host: Rust dependency compilation terminated with `STATUS_ACCESS_VIOLATION` (`0xc0000005`) before tests or profile generation.

## Test Results

| Command | Exit / Result | Evidence |
|---|---|---|
| `cargo test --all --all-features --no-fail-fast --manifest-path apps/api/Cargo.toml` | Passed: 246 passed, 3 ignored across 16 suites | Local command output |
| Workflow coverage-instrumented test build and `grcov` report | Not completed; no profiles or report generated | Previously recorded Windows compiler failure; the failed GitHub Actions run showed `grcov` failing on the prior nested binary path |

## Coverage Report

`lcov.info` was not generated locally because the instrumented build failed before producing profiles. The updated workflow's `grcov --binary-path apps/api/target/debug/` and the Coveralls/Codecov uploads require an Ubuntu workflow run for verification.

## Target Verification Matrix

See `build-and-test-summary.md` for the finalized matrix. Applicable targets FR1, FR1.1, FR1.2, and FR2 remain `Unverified` pending the Ubuntu workflow rerun; FR1.3, NFR1, and NFR2 are `Met` based on the workflow diff.

## Failure Assessment

The remaining gap is environment-specific verification, not a newly identified change to the approved path-only implementation. No local report evidence is available. The Ubuntu workflow must confirm successful report generation and uploads before those targets can be marked `Met`.
