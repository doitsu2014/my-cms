# Build Instructions

## Prerequisites

Run from the repository root with the Rust nightly toolchain and `llvm-tools-preview`, matching `.github/workflows/ci-my-cms.yml`.

## Build and Verification

Run the API test suite using the existing CI command:

```bash
cargo test --all --all-features --no-fail-fast --manifest-path apps/api/Cargo.toml
```

The local test suite passed: 246 passed, 3 ignored across 16 suites. The separate workflow-equivalent coverage-instrumented build previously failed during Windows dependency compilation with `STATUS_ACCESS_VIOLATION`; it did not produce coverage profiles.

## Troubleshooting

Coverage report generation is CI-specific for this change. The Ubuntu workflow run must verify profile generation, successful `grcov` execution, non-empty `lcov.info`, and both existing upload steps. Do not change instrumentation or coverage policy to work around a failure.
