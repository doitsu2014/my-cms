# Cross-Unit Traceability

## Coverage

The change is limited to the CI coverage workflow, so all requirements map to the Code Generation workflow change. The local API test suite passed, and the source diff preserves the requested test and instrumentation settings. CI-only report generation and uploads remain unverified pending an Ubuntu workflow rerun.

| ID | Requirement / criterion | Coverage | Owning stage / Unit | Target | Evidence |
|---|---|---|---|---|---|
| FR1 | Generate root `lcov.info` from the existing API test run | Covered in implementation; runtime result unverified | Code Generation / N/A | `.github/workflows/ci-my-cms.yml` | Updated grcov binary path; Ubuntu workflow run still needed |
| FR1.1 | Use the valid Cargo binary directory, without embedded quotes | Covered in implementation; runtime result unverified | Code Generation / N/A | `.github/workflows/ci-my-cms.yml` | Path is `apps/api/target/debug/`; CI resolution not verified |
| FR1.2 | Preserve LCOV format and `lcov.info` filename | Covered | Code Generation / N/A | `.github/workflows/ci-my-cms.yml` | Workflow retains `-t lcov` and `-o ./lcov.info` |
| FR1.3 | Preserve test command and coverage instrumentation | Covered | Code Generation / N/A | `.github/workflows/ci-my-cms.yml` | Test command and instrumentation environment are unchanged |
| FR2 | Preserve Coveralls and Codecov uploads | Covered in configuration; upload execution unverified | Code Generation / N/A | `.github/workflows/ci-my-cms.yml` | Both existing upload steps remain configured for `lcov.info` |
| NFR1 | Preserve coverage policy and instrumentation | Covered | Code Generation / N/A | `.github/workflows/ci-my-cms.yml` | No threshold or instrumentation changes |
| NFR2 | Limit change to the report path | Covered | Code Generation / N/A | `.github/workflows/ci-my-cms.yml` | Diff is limited to the grcov binary path |

## Uncovered Elements

None. Runtime verification of FR1, FR1.1, FR1.2 report creation, and FR2 uploads remains an evidence limitation, recorded as `Unverified` in the Target Verification Matrix, not an uncovered requirement.

## Verdict

Traceability coverage passes: every requirement and acceptance criterion maps to the workflow change. The Build and Test quality outcome remains unsuccessful under its verification criteria because the Ubuntu report-generation and upload checks have not run.
