# Code Generation Plan

## Summary

- Builds: correct the CI coverage report's Cargo binary path
- Touches: `.github/workflows/ci-my-cms.yml`
- Tests: rerun the existing Rust CI test command and verify `grcov` generates `lcov.info`

## Scope and Evidence

The failed Actions runs completed Rust test suites but failed in `grcov` while opening the configured `--binary-path`; the error-rendered directory is `apps/api/target/debug/deps/`. The workflow command itself has no embedded quotes. Cargo metadata reports `apps/api/target` for this workspace; in the instrumented GitHub Actions run, Cargo output showed test executables under that target's `debug/build` tree. This Windows host cannot generate compatible coverage profiles, so the report-path failure remains CI-specific.

Point `grcov --binary-path` to Cargo's target debug root, `apps/api/target/debug/`, which contains the compiled test executables under `deps/`. Preserve the existing test command/scope, `RUSTFLAGS`, `CARGO_INCREMENTAL`, `LLVM_PROFILE_FILE`, LCOV output `lcov.info`, and uploads to Coveralls and Codecov. Do not change coverage thresholds, instrumentation, providers, or unrelated workflow settings. Verify the correction on the Ubuntu PR runner.

## Implementation Steps

1. [x] Confirm the checked-out CI workflow line and the failed job's actual invocation/error; verify Cargo's target directory. The failed workflow run was reviewed; Cargo metadata reports `apps/api/target`.
2. [x] Change only `grcov --binary-path` to `apps/api/target/debug/`, retaining the test, instrumentation, LCOV, and provider settings.
3. [ ] Verify on Ubuntu CI that the existing API tests pass, grcov creates non-empty `lcov.info`, and both upload steps complete. Local Windows coverage compilation failed with `STATUS_ACCESS_VIOLATION`; no local profiles or report were produced.
4. [ ] Record the PR CI result and confirm the final diff remains limited to the coverage-report path.

## Testing Contract

<!-- AIDLC-TESTING-CONTRACT: BEGIN -->
```json
{
  "version": 1,
  "methodology": "test-after",
  "source": "org",
  "ordering": "implement each applicable testable layer, then write and run that layer's tests.",
  "scope": "bugfix",
  "test_strategy": "minimal",
  "project_type": "brownfield",
  "applicable_notes": [
    {
      "layer": "org",
      "text": "We treat tests as a first-class deliverable in every Bolt. The specific\nmethodology (TDD, BDD, ATDD, or classic test-after) is affirmed at\npractices-discovery and recorded in `team.md` under this heading with explicit\n`Methodology` and `Ordering` fields; Code Generation resolves those fields\nindependently from coverage, tooling, and scope notes.\n\nWhen no posture has been affirmed, our default per scope is:\n- **Methodology**: test-after\n- **Ordering**: implement each applicable testable layer, then write and run\n  that layer's tests.\n- `mvp`, `enterprise`, `feature`, `infra`, `classic` add an 80% line-coverage\n  floor and CI execution before merge.\n- `bugfix`, `security-patch` add a targeted regression for the specific\n  bug/vulnerability and require the existing suite to remain green.\n- `express` uses the Minimal strategy: requirement-driven unit tests (one per\n  requirement, with a happy-path floor per component); existing tests remain\n  green.\n- `poc`, `refactor`, `workshop` add no extra new-test floor and require the\n  existing suite to remain green.\n\nThe active `Test Strategy` still applies in every scope and determines test\nvolume/types. Scope floors are additive; they never reduce or replace the\nselected strategy.\n\nBuild and Test verifies defined coverage floors and affirmed quality targets;\nthey may not be weakened to make a step pass.\n\nAffirm a stricter posture in `team.md` if the team commits to one."
    }
  ],
  "obligations": {
    "strategy": "minimal",
    "strategy_volume": [
      "One verifiable test per requirement at the narrowest effective level.",
      "At least one happy-path unit test per component.",
      "Unit tests are the default; a bugfix/security scope floor may require an integration or E2E regression when that is the narrowest level that reproduces the defect."
    ],
    "scope_floor": [
      "Include a targeted regression for the bug or vulnerability.",
      "Keep the existing test suite green."
    ],
    "combination_rule": "Apply every selected-strategy obligation and every scope-floor obligation; neither replaces the other, and a targeted scope regression may add the narrowest necessary test type beyond the strategy default."
  },
  "plan_profile": {
    "methodology": "test-after",
    "runner_step": "Verify the existing test runner/configuration and record the exact unit-scoped command.",
    "runner_ready_before_first_test": true,
    "testable_layers": [
      "Data model / database behavior",
      "Repository / data access",
      "Business logic",
      "API / endpoint",
      "Frontend behavior"
    ],
    "steps": [
      "Project structure and production configuration skeleton.",
      "Verify the existing test runner/configuration and record the exact unit-scoped command.",
      "Data model / database behavior - implement.",
      "Data model / database behavior - write and run its tests after implementation.",
      "Repository / data access - implement.",
      "Repository / data access - write and run its tests after implementation.",
      "Business logic - implement.",
      "Business logic - write and run its tests after implementation.",
      "API / endpoint - implement.",
      "API / endpoint - write and run its tests after implementation.",
      "Frontend behavior - implement.",
      "Frontend behavior - write and run its tests after implementation.",
      "Environment/build configuration.",
      "Documentation and traceability."
    ]
  },
  "input_sha256": "sha256:933f32e0a09112bec79e6ad9d9b07d8fa79b601a7b325fbe1f9a283f191ae54c",
  "contract_sha256": "sha256:8b88443a8c617567a1344d4ec2a14fc8e4e0abe702a4048e68bc207a690b0910"
}
```
<!-- AIDLC-TESTING-CONTRACT: END -->
