# Risk and Sequencing Rationale — Backup & Restore

## Chosen heuristic

This rationale is grounded in the accepted boundaries and responsibility split in [`unit-of-work.md`](../units-generation/unit-of-work.md) and the sequential DAG/integration facts in [`unit-of-work-dependency.md`](../units-generation/unit-of-work-dependency.md). A **walking skeleton** is the first smallest integrated slice through real project boundaries. The team affirmed this approach for this backup/restore scope. A **Bolt** is one Construction pass for a Unit that ends in a result that can be verified. The sequence uses walking-skeleton-first for U1 and risk-first emphasis for later work, without overriding the approved dependency DAG. No WSJF or numeric scoring was selected; available evidence does not support assigning defensible value, urgency or effort scores.

## Why this order

1. **B1 / U1 — integration and truthful incompleteness.** Validate configured-instance selection, admin boundary, durable job metadata, local worker and archive/checksum integration. The expected demo is an incomplete database-only slice. Its decisive safety test is that missing required Auth/Storage capabilities prevent the package from being marked complete or downloaded as a restorable backup. This earns evidence about the local runtime before expanding capture scope.
2. **B2 / U2 — risk-first capability verification inside the DAG.** Before promising a complete backup, resolve exact database scope and supported dump method, GoTrue export/import plus password/session effects, and Storage enumeration/metadata/integrity. Verify configured object-store encryption/access/retention/size prerequisites. A required unsupported capability leads to explicit failure or a human scope decision, not a weakened success claim.
3. **B3 / U3 — prevent destructive surprises.** Validate upload bounds, archive structure, compatibility, exact target and replace impact before changes. Prove that rejection and preflight leave the target unchanged; require a usable recovery point and bind later confirmation to digest and target identity.
4. **B4 / U4 — execute only on verified foundations.** Restore across independent systems only after recovery/rollback semantics and idempotency are designed. Test injected failures and post-restore validation; terminal state must be success only after all required checks, otherwise verified rollback or `recovery-required`.

## Dependency and topology validation

The order exactly follows the Unit DAG `U1 → U2 → U3 → U4`; there is no deviation and no independent Unit pair for parallel delivery. Each Bolt contains one Unit because the human selected “one Unit per Bolt” after clarifying their earlier cross-cutting preference against the actual DAG. This keeps design/test ownership and Unit checkpoints legible; cross-cutting end-to-end checks belong in DoD and verification, not in a reordered DAG.

## Risk register

| Risk | Likelihood | Impact | Mitigation / sequencing response |
|---|---|---|---|
| GoTrue/Auth cannot safely export/import required identity data or password/session semantics differ from expectations | Medium (capability unknown) | Critical | Verify supported self-hosted path and policy before a complete-backup claim; represent missing required Auth as failed/incomplete; request owner decision if scope must change. |
| Database dump scope or restore compatibility includes unsupported/managed schemas | Medium (scope unresolved) | Critical | Name approved My-CMS tables and dump method; version-check source/target; exclude unowned/managed data; test on disposable localhost instances. |
| Storage paging, metadata or concurrent changes produce missing or inconsistent objects | Medium | High | Verify pagination and object metadata; count/digest checks; record best-effort consistency; fail when required inventory is incomplete. |
| ZIP upload/extraction allows traversal, archive bombs or resource exhaustion | Medium | Critical | Enforce configured byte/entry/depth/time/temp bounds before target mutation; reject path/symlink/conflict hazards; adversarial test corpus. |
| Restore partially mutates database/Auth/Storage and no rollback exists | Medium (cross-system semantics unresolved) | Critical | Establish recovery point and compensating/recovery behavior per adapter before mutation; no global atomicity claim; test failure injection and persistent recovery state. |
| Worker crash/restart leaves jobs stuck or reports a false terminal success | Medium | High | Persist state and leases/heartbeats, define unknown side-effect handling, timeout and reconciliation; never infer success from task exit alone. |
| Object-store access/encryption/retention/size policy is incomplete | Medium | High | Keep actual data usage gated; require explicit valid server-side configuration and least privilege before writing real packages. |
| CI workflow is temporarily disabled, so approved CI/coverage gate is not currently enforced | High (documented disabled) | High | Use verified local tests/coverage for development evidence; separately restore or confirm an authorized CI mechanism before treating merge gate as operational. Do not claim CI green based on an inactive workflow. |
| UI ownership is spread across service-kind Units and may omit dedicated frontend component design | Medium | Medium | Use existing refined mockups; assign UI acceptance criteria explicitly during Functional Design and ensure the responsible Unit carries accessible screen/state implementation. Revisit Unit kind only if an independent UI surface is needed. |
| Story acceptance criteria are cross-assigned to multiple Units and may be over-scoped in per-Unit design | Medium | Medium | In Functional Design, partition concrete AC responsibility per Unit and ensure later Units close remaining criteria; do not treat partial U1 as complete story coverage. |

Likelihood is qualitative, based on unresolved capabilities rather than measured occurrence rates. Impact reflects potential data loss, confidentiality exposure or false backup/restore assurance.

## Confidence hypotheses

- B1 proves the project can execute a durable localhost backup job end-to-end without falsely certifying incomplete data.
- B2 proves whether configured self-hosted database, Auth and Storage can produce an integrity-checked supported package with honest consistency semantics.
- B3 proves package and target compatibility can be decided before target mutation.
- B4 proves a supported restore can complete with verified postconditions or produce an explicit recoverable failure state.

## Operating assumptions

All work remains localhost-only with no release/deploy action. Production restore is out of scope. External platform capability checks have no confirmed external owner or ETA; track them as verification gates, not scheduled hand-offs. Concrete timeouts, concurrency, archive limits, retention, RPO/RTO and recovery-point policy remain for later design/configuration before real data use.
