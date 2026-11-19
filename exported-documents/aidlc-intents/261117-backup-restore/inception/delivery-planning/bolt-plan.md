# Bolt Plan — Backup & Restore

## Goal and delivery shape

This sequence derives from the approved Unit boundaries in [`unit-of-work.md`](../units-generation/unit-of-work.md) and the dependency edges/integration contracts in [`unit-of-work-dependency.md`](../units-generation/unit-of-work-dependency.md). A **Bolt** is one pass through the Construction design/build/test stages for one Unit, ending in a result that can be checked. The four Bolts follow the approved dependency DAG strictly: U1 → U2 → U3 → U4. No Unit is started before its prerequisite Unit is built and verified. The team's requested thin end-to-end **walking skeleton** is U1: it proves the configured admin request → durable job → local worker → archive/checksum path, but fails closed and is never offered as a complete or restorable backup while required Auth and Storage are absent.

The sequence uses walking-skeleton-first for U1, then risk-first emphasis within the DAG: U2 must resolve database scope, Auth/GoTrue capability and Storage/archive constraints before any full-backup claim; U3 validates safe, non-mutating preflight; U4 validates recovery semantics and controlled restore. This order is an economic plan, not a change to the topological DAG.

## Bolt sequence

| Bolt | Unit | Dependencies | Included work | Definition of Done | Confidence hypothesis | Expected demo |
|---|---|---|---|---|---|---|
| B1 — Local job-to-package skeleton | U1 `u1-backup-foundation` | None | Configured source selection; admin authorization; durable job metadata; local worker; bounded package/manifest/checksum foundation; database-only localhost vertical slice. | An authorized local admin creates a job; persisted metadata flows through API→worker→archive/integrity. Tests prove unavailable required Auth/Storage capabilities prevent `succeeded` and prevent download as a complete backup. No production target, no credentials from browser, no archive execution. | The existing gateway can host a safe, observable asynchronous path without a new broker, while keeping partial capture honest and fail-closed. | Create local job, inspect status and integrity evidence; demonstrate required missing Auth/Storage marks the job failed/incomplete and blocks package download. |
| B2 — Complete supported backup | U2 `u2-complete-backup` | U1 | Verify and implement allowed database scope, GoTrue/Auth capability, Storage object capture, package inventory/manifest and durable package storage/download. Resolve policy/config gates for real data. | Required components are captured or the job fails; per-component digest/count/size validates; package format and exclusions are truthful; completed package downloads only after integrity verification; unsupported Auth/password/session or storage capability is explicit and cannot be represented as fully protected. | Capability checks and strict package semantics can make the difference between a usable backup and an honest refusal measurable before restore work is funded. | Create a localhost package, inspect manifest/inventory/checksums, download it, and demonstrate fail-closed behavior for an unsupported required adapter. |
| B3 — Safe restore preflight | U3 `u3-restore-preflight` | U2 | Bounded private ZIP upload; archive safety/limits; manifest/digest validation; target compatibility matrix; replace-impact summary; recovery-point feasibility and non-mutating preflight. | Malformed, oversized, unsafe, unsupported or incompatible package is rejected before target mutation; passing preflight binds package digest and configured target; UI/API clearly reports replace impact and required warnings. | Preflight can reject dangerous or unsupported packages before destructive operations and binds confirmation to the exact package and target. | Upload a valid and invalid test package; show compatibility and replace-impact summary; prove rejected packages and preflight leave target unchanged. |
| B4 — Controlled restore and recovery | U4 `u4-restore-execution` | U3 | Explicit target/digest-bound confirmation; restore orchestration across supported adapters; target lock/idempotency; post-restore checks; rollback or `recovery-required`; safe history/audit outcome. | Restore starts only after passing preflight, explicit admin confirmation and verified recovery point. Success requires every required component and post-check; failures report verified rollback or actionable `recovery-required`, never partial success. Conflicting target restores are serialized/rejected. | A persisted saga/state machine can coordinate independent stores honestly without assuming global atomicity; each uncertain side effect remains recoverable and auditable. | Restore an approved localhost test package into a disposable configured target; demonstrate post-validation and a controlled failure that yields rollback or recovery-required, not success. |

## Acceptance gates and prerequisites

- Every Bolt completes its applicable Functional Design, NFR, implementation and test stages, and its human checkpoint before the next dependent Unit begins.
- U1 demo is a skeleton checkpoint, not approval for production or a complete backup claim.
- Before B2 claims complete backup: verify supported My-CMS tables/dump method, GoTrue/Auth export/import plus password/session semantics, Storage paging/metadata, and configured object-store encryption/access/retention/size policy. Missing required capability stops or narrows the claim, not the safety checks.
- Before B3/B4: verify supported source/target compatibility, archive limits, recovery-point mechanism, cancellation/lease semantics and rollback behavior. No restore to production in this scope.
- Existing delivery assumptions: localhost-only, no release/deploy task, new branch plus PR for changes. A transition from localhost to staging/production requires a separately confirmed promotion policy; do not infer it from organizational defaults.

## Stage configuration

| Construction stage | Decision | Rationale |
|---|---|---|
| Functional Design | EXECUTE for each Unit | State machines, archive formats, authorization, UI behavior and multi-step restore require detailed functional design. |
| NFR Requirements | EXECUTE for each Unit | Sensitive data, admin authorization, package safety and recovery are security/reliability critical. |
| NFR Design | EXECUTE where NFR Requirements is non-trivial | Backup/restore has non-trivial confidentiality, integrity, resource-bound and recovery requirements. |
| Infrastructure Design | SKIP for initial localhost-only scope | No new deployment target is approved; revisit if infrastructure or release scope changes. |
| Code Generation | EXECUTE for each Unit | Each Unit must implement its assigned slice, with test-after at each testable layer. |
| Build and Test | EXECUTE for each Unit | Verify regressions, required acceptance criteria, coverage target and actual runnable checks. CI is currently documented as temporarily disabled, so local evidence is not described as CI-green. |

