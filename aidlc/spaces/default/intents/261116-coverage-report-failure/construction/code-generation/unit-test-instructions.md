# Unit Test Instructions

## Scope

Coverage-report repair is in `.github/workflows/ci-my-cms.yml`. The reported failure occurs after the Rust test suites succeed, in the `grcov` report-generation step. Keep the existing tests and coverage instrumentation unchanged.

## Test Framework and Configuration

The workflow uses Rust's built-in test harness for the `apps/api` Cargo workspace. It configures LLVM source coverage with nightly Rust, `llvm-tools-preview`, `CARGO_INCREMENTAL=0`, the existing `RUSTFLAGS`, and `LLVM_PROFILE_FILE=test-llvm-reports/my-cms-%p-%m.profraw`.

## Run Commands

Run from the repository root using the workflow's existing configuration:

1. The exact test invocation is the existing workflow command: `cargo test --all --all-features --no-fail-fast --manifest-path apps/api/Cargo.toml` with the workflow environment variables and coverage flags.
2. Run the exact `grcov` command from `.github/workflows/ci-my-cms.yml` after tests finish. It must use the existing LCOV format and `lcov.info` output, with a binary path that resolves to Cargo's actual `apps/api` target directory and contains no literal quote characters.

Expected result: tests pass, `grcov` exits zero and creates a non-empty `lcov.info`; Coveralls and Codecov continue receiving that file in the subsequent workflow steps.

## Coverage Targets

Preserve all existing coverage instrumentation and any configured threshold. Do not lower or bypass a threshold to resolve a path error.

## Regression Evidence

The targeted regression is the report-generation command succeeding against the same profile and test binaries emitted by the existing CI test command. The original run URL is recorded in the requirements Q&A file. No source-code unit test is appropriate for this workflow-argument-only change.
