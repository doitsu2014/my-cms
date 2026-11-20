# Requirements: CI Coverage Report Path Repair

## Sources and Evidence Limits

The failing GitHub Actions run completed the Rust test command, then `grcov` failed opening its configured `apps/api/target/debug/deps/` binary directory. The command shown in the workflow and run log has no embedded quote characters; quotes appear in the error's rendered path. The exact CI-only resolution failure remains unconfirmed. [desc] [Q1] [workflow]

The intended outcome is to create `lcov.info` and upload it through the existing Coveralls and Codecov steps. The approved change scope is limited to the existing coverage-report path; test scope, instrumentation, threshold, format, and providers stay unchanged. [Q2] [Q3]

The existing code knowledge is partial and did not verify workflow execution. Local ordinary Rust tests passed (246 passed, 3 ignored); coverage-instrumented compilation on Windows failed with Rust `STATUS_ACCESS_VIOLATION` before producing test profiles, so local report generation could not be verified. [Q1] [code-quality-assessment.md]

## Functional Requirements

### Coverage Report Generation

**FR1 — Generate LCOV from the existing API test run.** Given the existing Rust API CI test step completes successfully and produces coverage profiles and test binaries, when the report step runs from the repository root, `grcov` must read the API workspace's compiled test binaries and coverage profiles and create `lcov.info` in the repository root. [Q1] [Q2] [workflow]

- **FR1.1 — Use the valid Cargo binary directory.** The `--binary-path` argument must resolve, from the job's repository-root working directory, to the API workspace's actual `target/debug/deps` directory produced by the existing test command. It must not pass embedded quote characters as part of the path value. [Q1] [workflow]
- **FR1.2 — Preserve the report contract.** Keep grcov's LCOV output format and output filename `lcov.info`. [Q2] [workflow]
- **FR1.3 — Preserve test inputs and instrumentation.** Keep the existing Cargo test command, test scope, `CARGO_INCREMENTAL`, `RUSTFLAGS`, and `LLVM_PROFILE_FILE` unchanged. Do not move or remove generated test binaries or profile files before report generation. [Q3] [workflow]

### Existing Coverage Uploads

**FR2 — Keep existing report uploads.** When `lcov.info` is generated successfully, the existing Coveralls and Codecov steps must continue to consume that file with their current action configuration and credentials. [Q2] [workflow]

## Non-Functional Requirements

**NFR1 — Preserve coverage policy.** Do not lower, disable, bypass, or otherwise change any existing coverage threshold or instrumentation setting as a workaround for report-path resolution. [Q3] [workflow]

**NFR2 — Limit change scope.** Modify only the report-path configuration necessary to address the observed `grcov` directory-open failure; leave test coverage scope, format, providers, and unrelated workflow configuration intact. [Q3] [workflow]

## Constraints

- Target file: `.github/workflows/ci-my-cms.yml`. [workflow]
- Job working directory is the repository root; the test command uses `--manifest-path apps/api/Cargo.toml`. [workflow]
- Keep LCOV format and output at `./lcov.info`; retain Coveralls and Codecov upload steps. [Q2] [workflow]
- Preserve all existing tests, instrumentation options, coverage thresholds, and provider configuration. [Q3] [workflow]
- Do not add separate diagnostics or new upload/report behavior; such additions exceed the approved report-path-only change. [Q3]

## Assumptions

- The Cargo workspace target directory for the CI job is `apps/api/target`, consistent with local Cargo metadata. The workflow must continue to work with the target directory actually produced in CI. [Q1] [workflow]
- The exact CI path-open failure was not reproduced on this Windows host; the workflow rerun is the definitive verification for the environment-specific failure. [Q1]

## Out of Scope

- Changing test suites or test selection. [Q3]
- Changing LLVM instrumentation, profile naming, coverage thresholds, LCOV format, or either upload provider. [Q3]
- Replacing grcov or adding new failure diagnostics, fallback paths, or upload behavior. [Q3]
- Fixing unrelated workflow deprecation warnings or the existing workflow comment. [workflow]

## Open Questions

- Will the corrected path resolve successfully on the Ubuntu GitHub Actions runner and allow both existing coverage upload steps to complete? This requires a rerun of the workflow; local Windows coverage compilation failed before profiles were produced. [Q1]
