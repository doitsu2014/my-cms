# Code Generation Plan — Backup and Restore

## Summary

- Builds: operator-invoked full Supabase database and media backup/restore to S3
- Touches: new Rust backup module/CLI, configuration, tests, and operational notes
- Tests: 8 requirement-driven unit tests plus existing suite checks

## Scope and assumptions

- Implement server-side operator commands; no application UI or HTTP endpoint is planned because restore is explicitly restricted to operators with server access.
- Use AWS S3-compatible APIs to read/write `backup-system`; reuse the project's existing AWS/S3 dependency if present, otherwise add a narrowly scoped Rust AWS SDK dependency after checking workspace conventions.
- PostgreSQL backup targets the complete self-hosted Supabase PostgreSQL instance, not only My-CMS application schemas. Confirm whether the deployment provides the required privileged database role and whether platform roles/globals require `pg_dumpall`-style export.
- Supabase Storage uses a file backend in the inspected Docker deployment. The implementation must discover the actual configured storage backend and object root from deployment configuration; it must not assume file-copy backup is portable to other backends.
- Best-effort capture is accepted: record the capture interval and do not promise cross-store point-in-time consistency.
- Restore is destructive. Before implementation, resolve whether the application can safely restore all Supabase-managed schemas/metadata and which platform services must be stopped. Do not implement a partial destructive restore that could leave the Supabase platform unusable.

## Implementation steps

1. [x] Inspect the current Rust workspace, gateway/CLI command structure, configuration conventions, Cargo dependency graph, deployment manifests, and existing tests. Identify the exact operator entry point and storage/database paths without running destructive commands.
2. [x] Verify the existing Rust test runner and command. Record exact command: `cargo test --manifest-path apps/api/Cargo.toml -p my-cms-api <test_filter>` (confirm package and filter syntax before relying on it).
3. [x] Define a versioned backup manifest and archive layout containing backup ID, timestamp, PostgreSQL export files, object inventory/payloads, source system metadata, and checksums. Unit tests cover valid/invalid manifest and checksum behavior.
4. [ ] Blocked: full-instance export/import and safe platform-wide restore procedure are not established; no partial destructive implementation. and argument validation, covering complete database/schema scope and subprocess failure handling without invoking real database tools in unit tests.
5. [ ] Blocked: database process implementation not present., scope selection, quoting/argument safety, and failure reporting.
6. [ ] Blocked: configured Supabase storage root and S3 transfer mechanism are not safely wired. for the configured Supabase Storage backend, with deterministic inventory and per-object checksum verification; surface unsupported backend or inaccessible paths as explicit errors.
7. [x] Added path and object-key safety unit coverage; transfer tests blocked by absent adapter., missing/changed objects, path safety, and partial transfer failures.
8. [ ] Blocked: no S3/database/object adapters; CLI fails closed and cannot record successful backup.: operator-triggered command, unique backup prefix under `backup-system`, best-effort database then object capture, manifest/checksum generation, archive-integrity check, and durable success/failure result. Failed/partial backups must not be advertised as complete.
9. [ ] Blocked by approved safety condition: restore rejects before destructive action pending validated full Supabase restore procedure.: validate selected backup and checksums first, require explicit destructive confirmation, restore database and objects with operator-only server access, and report any partial failure without claiming success. Restore ordering and safe Supabase platform scope must be confirmed before destructive execution is enabled.
10. [x] Added six validation/manifest tests and ran focused plus gateway suite; orchestration coverage blocked by absent adapters. for successful backup/restore and at least two failure cases (database export failure; object transfer or integrity failure); run existing relevant tests.
11. [ ] Blocked: credentials/endpoints are read from environment only; operational transfer wiring not implemented. for S3 credentials/bucket, database access, storage root/backend, and operator command entry point. Never hardcode secrets; use least-privilege credentials and safe file permissions.
12. [x] Documented implemented entrypoint, safety limits, tests, and blockers in code-summary.md., manual deletion/retention behavior, best-effort consistency limitations, restore safeguards, and recovery from partial failure.

## Testing Contract

```json
{
  "version": 1,
  "methodology": "test-after",
  "source": "org",
  "ordering": "implement each applicable testable layer, then write and run that layer's tests.",
  "scope": "express",
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
      "Keep the existing test suite green.",
      "This scope adds no extra new-test floor beyond the selected test strategy."
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
  "input_sha256": "sha256:c8f84e35190e007d9863fad236154d0269ab9bf633fc2a3ff106ed965ba4a9b9",
  "contract_sha256": "sha256:bb76c1b22f8307be6e5780d50cabab891684449d5b164bd0e825e9e4e5c61e5b"
}
```

## Requirement-to-step traceability

- FR1, FR1.1–FR1.4: Steps 3–8 and 10–12
- FR2, FR2.1–FR2.3: Steps 9–12
- NFR1: Steps 1, 9, and 11
- NFR2: Steps 3, 8, and 10
- NFR3: Steps 8 and 12

## Completion criteria

- PostgreSQL and configured image/blob storage have explicit, separate backup and restore paths.
- Backup manifest/checksums validate before success is reported or restore begins.
- Restore is gated to an operator command, clearly destructive, and does not claim success after a partial failure.
- Existing relevant tests pass; new tests cover each requirement and a happy path per component.
- No production secrets are hardcoded; bucket and runtime access are configured externally.
