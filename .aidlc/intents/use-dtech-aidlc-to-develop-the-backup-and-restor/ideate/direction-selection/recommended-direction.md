# Recommended direction: Option A

**Status:** Proposed direction for the solution-direction gate; not approved and not construction-ready. This document recommends a choice; the conductor owns gate reporting. Explicit written owner/architect approval of Q-01/Q-02/Q-03 is still required before construction.

## Recommendation

Choose **Option A: admin UI first, a dedicated backup domain, and explicit public `incomplete` status**. Deliver a localhost-only, database-slice workflow using synthetic fixtures, durable job metadata, protected API operations, and truthful diagnostics. Do not implement real capture/staging or restore under the label of a workflow pilot.

The architectural decisions are recorded in [decision-record.md](decision-record.md). [scope-boundary.md](scope-boundary.md) defines the acceptance limits and construction blockers; [architecture-design.md](architecture-design.md) and [threat-model.md](threat-model.md) describe the proposed design and its security obligations. All are proposals, not observations of implemented capabilities.

## Why A wins

1. **Architectural fit.** MAP-01 §6 and the existing [architecture](../../../../../docs/architecture/software-architecture.md) place feature logic, persistence and migrations in domain crates and leave the gateway as composition root. A coherent backup domain follows this rule instead of creating an exception that U2 must undo.
2. **Primary operator workflow.** FEAT-01 §§1–4 centers the admin UI. A delivers the U1 portions of US-01/02/05/06: allowed-instance selection, synthetic job creation/monitoring, protected resources and safe outcome diagnosis. US-02's complete downloadable ZIP is not delivered. US-03/04 receive labeled previews and rejection/no-mutation evidence only, not actual upload or restore acceptance.
3. **Truthful semantics.** Public `incomplete` distinguishes planned absence of required Auth/Storage from execution failure. Persisted state, API and UI use the same meaning; no translation into a generic failure is needed. Every U1 terminal backup outcome is `incomplete` or `failed`, never `succeeded`; no U1 artifact is downloadable (C-01).
4. **Reusable U2 foundation.** The same domain/API can later accept separately approved real database, Auth and Storage adapters. UI and authorization work need not be thrown away. This is an extension path, not evidence that Supabase recovery works (Q-04 remains open).
5. **Clear safety ownership.** One domain owns lifecycle policy, durable allocation tracking, reconciliation and safe audit integration. This reduces the risk of an allocation being left between teams, without pretending the cleanup design is already proven.

## Why the alternatives lost

| Alternative | Credible advantage | Reason not selected | Evidence that would justify choosing it instead |
| --- | --- | --- | --- |
| B: shared backup domain, UI and CLI, internal `incomplete` projected to public `failed` with reason | Preserves the existing public enum and serves two operator interfaces with one engine. | Dual-client authentication, versioning and parity tests add work. Semantic translation can make planned incompleteness indistinguishable from operational failure when consumers ignore the reason. The CLI is not established as a co-equal U1 need. | Owner confirms CLI is a co-equal requirement and compatibility evidence shows public enum expansion is unacceptable. Reopen scope and contract decisions explicitly. |
| C: CLI-first, gateway-owned pilot, isolated U1 contract | Faster initial interface and local operator feedback; avoids changing the general public enum. | Defers admin acceptance validation and violates the established feature ownership pattern unless an exception is approved. Gateway extraction, historical-state migration and journal/database divergence create likely U2 rework. | Owner explicitly replaces admin-first delivery with a temporary CLI experiment and accepts the architecture exception and later migration cost. |

Source options are [solution-options.md](../option-generation/solution-options.md), not a file named `option-generation.md`; the comparison is in [trade-off-matrix.md](../option-generation/trade-off-matrix.md).

## Trade-offs accepted in principle, not yet approved

- **Medium relative effort**, not a schedule estimate: a new crate, domain-owned migrations, authorized API and UI, plus lifecycle fault tests. A thin CLI would be quicker to show but would validate a different workflow.
- Adding an enum variant may break exhaustive serializers or clients. Review consumers, schemas and UI handling before construction; coordinate the contract change rather than silently converting old records.
- Synthetic data exercises orchestration and cleanup mechanics, not capture fidelity, encryption readiness, archive security or recovery. This U1 is not a usable disaster-recovery product.
- The CLI skeleton remains fail-closed and incomplete; no S3 adapter or CLI completion is implied (C-04, R-12).
- Durable cleanup adds work even for a fixture pilot. Deferring real staging does not close Q-03 or remove the need for approved finite bounds.

## Critical blocker disposition

| Question | Proposed resolution in principle | Still required before construction |
| --- | --- | --- |
| Q-01 | ADR-002 adds public terminal `incomplete`, with identical persisted/API/UI meaning and download denial. | Written owner/architect approval of enum expansion, outcome examples and consumer compatibility plan. |
| Q-02 | ADR-003 gives the backup domain ownership of jobs, migrations, state transitions and lifecycle integration; gateway only composes. | Written architect/owner approval of ownership and migration/schema integration. |
| Q-03 | ADR-004 selects a durable database allocation ledger, registration before allocation, inventory reconciliation and bounded idempotent cleanup; real staging remains off. | Written owner/architect approval of lifecycle protocol, crash matrix, DB-outage behavior, storage protection and fault-test exit criteria; owner/operations supply applicable finite Q-05 values and escalation responsibility. |

These recommendations resolve the design direction **in principle only**. The questions must not be marked closed merely because this document exists or a direction gate is approved. Missing finite values block affected construction; missing approval of any critical question blocks construction resumption.

## Evidence and uncertainty

- **Observed:** [MAP-01](../../../../knowledge/_codebase-map.md) §§3, 6–7, 9–10; [FEAT-01](../../../../../docs/features/backup-restore.md) §§9–13; prior failed U1 and current incomplete CLI.
- **Observed requirements:** [constraints C-01–C-07](../../analyze/context-and-risk/constraints-and-assumptions.md), [risks R-01–R-12](../../analyze/context-and-risk/risk-register.md), [questions Q-01–Q-12](../../analyze/context-and-risk/open-questions.md), [AC-001–006](../../analyze/problem-framing/acceptance-criteria.md), [US-01–06](../../analyze/problem-framing/user-stories.md).
- **Assumptions requiring confirmation:** admin UI priority (Q-09), same configured local source/target with no cross-instance mapping (Q-08), and precise resource authorization policy (A-04). Localhost is a deployment boundary, not authorization.
- **Open:** Q-04 supported Supabase recovery; Q-05 package/storage/expiry/retention and execution bounds; Q-06 RPO/RTO and drills; Q-07 production policy; Q-10 consistency; Q-11 encryption/keys; Q-12 current test readiness. No numeric values are selected here.
- **Verification limit:** only document/source review was performed. Historical test failures in FEAT-01 §12 are not a current baseline; Rust execution remains unverified (C-05). No capture, cleanup, recovery, UI or security test is claimed as passing.

## Authority boundary

Approval of a direction does not enable real data, authorize production restore, approve unspecified numeric limits, close the three critical questions automatically, or prove recovery. This worker neither approves nor reports the gate and does not create the conductor-owned `gate-recommendation.md`.
