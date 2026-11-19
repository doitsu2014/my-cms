# Code Generation Plan — u1-backup-foundation

## Summary

- Builds: U1 localhost backup job skeleton, configured-instance admin UI/API, durable metadata and gated private incomplete staging
- Touches: intended `apps/api/gateway` backup module/tests, `apps/web` backup UI/tests, narrowly scoped gateway route/config registration; persistence ownership/migration path must be resolved before schema work
- Tests: Standard test-after (5–8 per implemented component) plus API→job→worker integration; gateway crate and actual UI test paths; 80% project coverage checked on full suite at Build and Test

## Testing Contract

```json
{
  "version": 1,
  "methodology": "test-after",
  "source": "team",
  "ordering": "Triển khai từng lớp có thể kiểm thử, sau đó viết và chạy kiểm thử cho chính lớp đó.",
  "scope": "classic",
  "test_strategy": "standard",
  "project_type": "brownfield",
  "applicable_notes": [
    {
      "layer": "org",
      "text": "We treat tests as a first-class deliverable in every Bolt. The specific\nmethodology (TDD, BDD, ATDD, or classic test-after) is affirmed at\npractices-discovery and recorded in `team.md` under this heading with explicit\n`Methodology` and `Ordering` fields; Code Generation resolves those fields\nindependently from coverage, tooling, and scope notes.\n\nWhen no posture has been affirmed, our default per scope is:\n- **Methodology**: test-after\n- **Ordering**: implement each applicable testable layer, then write and run\n  that layer's tests.\n- `mvp`, `enterprise`, `feature`, `infra`, `classic` add an 80% line-coverage\n  floor and CI execution before merge.\n- `bugfix`, `security-patch` add a targeted regression for the specific\n  bug/vulnerability and require the existing suite to remain green.\n- `express` uses the Minimal strategy: requirement-driven unit tests (one per\n  requirement, with a happy-path floor per component); existing tests remain\n  green.\n- `poc`, `refactor`, `workshop` add no extra new-test floor and require the\n  existing suite to remain green.\n\nThe active `Test Strategy` still applies in every scope and determines test\nvolume/types. Scope floors are additive; they never reduce or replace the\nselected strategy.\n\nBuild and Test verifies defined coverage floors and affirmed quality targets;\nthey may not be weakened to make a step pass.\n\nAffirm a stricter posture in `team.md` if the team commits to one."
    },
    {
      "layer": "team",
      "text": "- **Methodology**: test-after\n- **Methodology evidence**: Nhóm xác nhận dùng `test-after` cho thay đổi này; workflow CI hiện được ghi rõ là tạm vô hiệu hóa, nên trạng thái vận hành hiện không chứng minh được cổng CI.\n- **Ordering**: Triển khai từng lớp có thể kiểm thử, sau đó viết và chạy kiểm thử cho chính lớp đó.\n- Nhóm đặt mục tiêu coverage 80% và yêu cầu CI xanh trước khi hợp nhất.\n- Scope `classic` cũng có mặc định tổ chức về coverage 80% và CI trước merge; câu trả lời của nhóm xác nhận các mục tiêu này.\n- Khoảng trống vận hành: cấu hình `.github/workflows/ci-my-cms.yml` có khai báo test/coverage nhưng đang tạm vô hiệu hóa; cần khôi phục hoặc xác minh cơ chế CI để cổng đã chọn có thể thực thi."
    }
  ],
  "obligations": {
    "strategy": "standard",
    "strategy_volume": [
      "Five to eight tests per component.",
      "Unit tests plus integration tests for key boundaries.",
      "Add E2E, performance, or security tests when requirements demand them."
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
  "input_sha256": "sha256:20836d139be3d7a6bbb36a41d04ba2ebe05366fde1938ac7950476fc7e9d7d29",
  "contract_sha256": "sha256:7a9854c639f73f611a9fdb555ea5c93018b189b8c35636aea6c4e5fcb983fb30"
}
```

## Approved design references

- Entity/business rules: `../functional-design/entities.md`, `../functional-design/rules.md`; known contract inconsistencies (status enum, staging lifecycle, idempotency composite scope and audit cardinality) must be resolved explicitly in this plan before implementation.
- Behavior: `../functional-design/functional-spec.md`; preserve the selected configuration, incomplete/no-download, admin UI and fail-closed semantics while reconciling AC1.2.4 to public `failed` status.
- Security and operations: `../nfr-requirements/security-requirements.md`, `../nfr-requirements/reliability-requirements.md`, `../nfr-design/security-design.md`, `../nfr-design/reliability-design.md`, `../infrastructure-design/infrastructure-specification.md`; TLS, private staging, finite config and durable crash-cleanup are mandatory gates.
- Existing code patterns inspected: `apps/api/gateway/src/main.rs` (service composition/shared DomainContext), `apps/api/domain_interface/src/lib.rs` (Mount/RouteRegistration/AuthenticatedActor), `apps/api/domain_posts/src/api/mod.rs` and `apps/api/domain_user/src/api/routes.rs` (admin route patterns), `apps/api/domain_posts/src/handlers/post/translate/translate_handler.rs` (persist-before-spawn precedent only; not a durable restart design), `apps/api/domain_seo/src/migrations/` (domain-owned migration precedent). Do not copy the in-process task as a crash-safe worker or add migration ownership without explicit approved design.

## Story and acceptance-criteria scope

This plan implements the approved U1 slice in [`unit-of-work.md`](../../../inception/units-generation/unit-of-work.md), [`unit-of-work-story-map.md`](../../../inception/units-generation/unit-of-work-story-map.md), and [`functional-spec.md`](../functional-design/functional-spec.md). U1 owns US1.1 and its four ACs, AC1.2.1, AC1.2.3's private database-staging digest only, AC1.2.4–AC1.2.6 for the incomplete slice, and its own portion of US3.2 audit. AC1.2.2 complete DB/Auth/Storage package and the remaining all-component digest guarantee are deferred to U2. U1 must never return `succeeded`, expose a download route for its incomplete artifact, claim restoreability, or accept client-provided connection details. Unsupported configuration/capability, TLS, finite bounds, encryption, or crash-safe cleanup keeps real-data writes disabled. Existing design artifacts still contain unresolved review items: public `incomplete` status versus contract `failed`, exact cleanup guarantees, NFR detailed-ID semantics, and idempotency/audit/job-lifecycle schema consistency. Resolve these in this plan before implementation or retain the explicitly restrictive behavior; do not implement contradictory design alternatives.

The contract and architecture boundaries are in [`contract-summary.md`](../../../inception/contract-design/contract-summary.md) and [`components.md`](../../../inception/domain-design/components.md). Existing source-data ownership remains with existing domains/platforms; U1 adds only backup job/staging metadata and its server-side route composition.

## Test-after implementation steps

### Step 1 — Workspace and runner readiness

- [ ] Add a bounded backup module to the existing gateway `gateway` package and register its routes through the gateway's existing `DomainService` and `Mount::Administrator` composition pattern; preserve the shared `DomainContext`/database pool. Do not add a new domain crate or migration crate in this U1 plan; any module/crate or migration ownership change must be resolved before implementation because the current service schema/ownership is not confirmed for backup metadata.
- [ ] Isolate backup handlers, repository/state logic and adapter interfaces under the gateway package while keeping CMS source tables owned by their current domains. No new connection pool or direct `gateway/main.rs` business logic.
- [ ] Confirm the existing Vitest configuration and install state; do not install dependencies or run broad formatter/build commands without authorization.
- [ ] Test command candidates are in `unit-test-instructions.md`; verify them after U1 tests exist. `cargo test -p gateway` is package-scoped, not strict U1-only, and must be reported accurately; do not add an unverified name filter. Run UI tests only against actual U1 test paths.
- [ ] Do not start real-data staging until TLS, private/encrypted storage, finite bounds/expiry and restartable cleanup/reconciliation are implemented and tested. Use synthetic fixtures until then.

### Step 2 — U1 metadata, schema ownership and lifecycle resolution

- [ ] Before implementation, reconcile current design contradictions in the approved plan or request a plan revision: public job outcome must use contract/story `failed` when a required component is missing; keep `incomplete` only for the private staging artifact/completeness marker unless the contract and AC are amended together. Also define whether staging/reconciliation is actually supported by the current gateway/runtime or enforce “no real-data/no staging” until a durable cleanup mechanism exists.
- [ ] Resolve entity constraints before schema work: idempotency uniqueness is composite/scoped by actor + operation + key digest (compare request digest for same-key/different-request conflict); model audit as one-to-many; specify durable staging lifecycle/cleanup state and how stale `running` work is represented without success. If no approved persistence/migration owner or recoverable cleanup design exists, do not introduce schema/data writes in U1; implement only an explicitly approved no-staging skeleton or request design clarification.
- [ ] Once these design gates are resolved, implement only the agreed U1 metadata: durable job, operation/status, actor/source configured ID, idempotency digest/request digest, state version, timestamps, safe reason, private incomplete staging reference/digest/expiry when allowed, lifecycle state and audit events.
- [ ] Keep raw idempotency keys, credentials, DB rows and artifact bytes out of metadata. Never create a new domain or migration crate or write into another domain's tables without an approved plan revision.
- [ ] Test-after for persistence: atomic job creation, composite idempotency scope across actors/operations, same-key/same-request reuse, same-key/different-digest conflict, stale state version rejection, one-to-many audit events, stale job representation, durable cleanup/reconciliation state, safe projections, and database error mapping. Use disposable PostgreSQL integration only when Docker/runtime availability is confirmed; retain deterministic unit tests regardless.

### Step 3 — Server configuration and adapter ports

- [ ] Add a server-owned configured-instance resolver that accepts opaque configured IDs only; do not accept browser endpoint/credential overrides.
- [ ] Define narrow database export, private staging, archive digest, cleanup/reconciliation, audit and job-store ports with typed results/safe error codes; register the new routes through the verified gateway admin mount pattern.
- [ ] Resolve secret/config references only in the server boundary; enforce TLS for admin/API, DB and staging/object-store hops, plus at-rest encryption/private policy before data access/write. No loopback TLS exception is inferred.
- [ ] Require typed finite byte/time/temp/concurrency/expiry settings and reject absent, malformed, non-positive or effectively unbounded values before staging.
- [ ] Implement durable staging lifecycle states and reconciliation for write-before-metadata crash windows. If existing environment cannot meet TLS, at-rest protection or durable cleanup requirements, refuse real-data staging; do not substitute plain temp storage.
- [ ] Test-after each boundary: missing/disabled configured instance, invalid config, TLS/policy refusal, finite-bound rejection before write, orphan discovery after interrupted write, bounded cleanup retry, cleanup failure signal, no secret serialization/logging.

### Step 4 — Business lifecycle and incomplete artifact behavior

- [ ] Implement the reconciled public lifecycle from Step 2. At minimum, a job missing required Auth/Storage components must be `failed` per AC1.2.4/JobStatus; an optional private staging artifact may independently carry `completeness=incomplete` and can never be downloaded. No U1 transition reaches `succeeded`.
- [ ] Persist job before returning API accepted; worker processing is bounded. Tokio task alone is not treated as durable queue/restart recovery.
- [ ] If the approved design authorizes staging, stage only synthetic data until TLS, private/encrypted storage, finite limits, durable lifecycle metadata and crash-safe cleanup are implemented and verified. Otherwise fail before writing bytes.
- [ ] On timeout/crash/unknown side effect, persist a defined stale/unknown non-success state or leave `running` explicitly blocked from success while reconciliation is pending; never infer success from file existence/process exit. Do not blind-retry mutation or unknown outcome.
- [ ] Implement cleanup/orphan reconciliation only if the current storage/runtime can support durable, observable cleanup; bounded retries and `DELETE_FAILED` escalation must not imply deletion succeeded. Real-data circuit stays disabled when cleanup is unresolved.
- [ ] Test-after business logic: missing capability, no-success invariant, expiry/orphan cleanup including crash windows, audit-before-side-effect failure, worker timeout/unknown state, and duplicate execution. Cancellation is not exposed until semantics are defined.

### Step 5 — Admin API and UI slice

- [ ] Register admin routes as a `Mount::Administrator` route through the existing gateway service composition/auth layer; use `AuthenticatedActor` and server-side per-resource authorization before every read/write. Confirm exact route/service registration ownership before editing manifests.
- [ ] Implement the U1 subset only: configured instance list, durable backup job create, single-job safe status read. Do not add complete-package download, job history pagination or cancellation APIs unless the approved contract is amended.
- [ ] Enforce strict request DTO/unknown-field rejection and idempotency behavior from `contract-summary.md`.
- [ ] Build the minimal Admin UI in existing React `apps/web`: instance selector, empty/loading/error states, create action, job ID/status, explicit incomplete/failed state and no download affordance. Preserve existing localization, responsive/accessibility patterns; add `data-testid` on interactive elements.
- [ ] Test-after API boundary: auth 401/403, guessed/inaccessible job ID, configured instance allowlist, arbitrary endpoint/secret rejection, idempotent duplicate, persistence failure, safe error serialization and status projection.
- [ ] Test-after UI: configured instance selector, no-instance disabled action, loading/error handling, submit once/idempotency, queued/running status, incomplete warning, no download control, keyboard and text-not-color status. Use existing Vitest/Testing Library patterns and mock the API.

### Step 6 — Cross-boundary integration and package safety

- [ ] Integrate gateway route, durable metadata, bounded worker, database-only adapter and fail-closed incomplete/private staging for synthetic localhost fixtures. The existing `domain_posts` translation job uses in-process `tokio::spawn`; do not copy it as proof of durability/restart safety. The NFR design requires reconciliation and leaves real-data staging disabled until proven.
- [ ] Validate checksum against actual staged bytes, but do not interpret digest as proof that a complete backup exists.
- [ ] Verify no public object URL, download endpoint, or package marked complete is reachable from U1.
- [ ] Add integration tests for API→job persistence→worker→incomplete outcome and crash/cleanup behavior using mocks/fakes; add Docker-backed PostgreSQL test only when available and keep it explicitly opt-in if it requires Docker.
- [ ] Run `cargo test --manifest-path apps/api/Cargo.toml -p gateway` as the existing gateway-crate test suite and record that it is package-scoped, not a strict U1-only filter; also run Vitest against actual U1 test paths after confirming the path-filter syntax. Record exact command/output and do not claim CI-green: `.github/workflows/ci-my-cms.yml` is documented as temporarily disabled.

### Step 7 — Final quality, source inventory and traceability

- [ ] Run unit-scoped tests, formatter/linter/type checks only when existing commands and targets are confirmed; never alter 80% threshold to make tests pass.
- [ ] Inspect diff for accidental changes to pre-existing files and any secret/content leakage.
- [ ] Create `source-manifest.json` listing every U1 application source file created/modified/deleted, including manifests, migration, test config and UI files; plan/summary artifacts are not source paths.
- [ ] Write `code-summary.md` with files changed, design decisions, tests actually executed and not executed with reasons, CI disabled status, and discrepancies/risks.
- [ ] Generate `traceability.json` only after implementation; every `OK` target must be a real workspace-relative source or test path. Keep deferred/out-of-scope AC honest; do not mark U2's completed package criteria as implemented by U1.
