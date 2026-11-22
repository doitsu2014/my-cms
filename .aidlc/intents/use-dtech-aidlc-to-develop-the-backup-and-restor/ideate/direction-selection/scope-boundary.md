# U1 scope boundary

**Status:** Proposed under Option A; no gate approval or construction authorization. Purpose: distinguish the delivered workflow slice from target feature behavior, and identify the conditions that still block construction.

Authority and rationale: [decision-record.md](decision-record.md), [recommended-direction.md](recommended-direction.md). Baseline: [constraints](../../analyze/context-and-risk/constraints-and-assumptions.md), [questions](../../analyze/context-and-risk/open-questions.md), [acceptance criteria](../../analyze/problem-framing/acceptance-criteria.md), [stories](../../analyze/problem-framing/user-stories.md), and [FEAT-01](../../../../../docs/features/backup-restore.md) §§10–13.

## In scope after required approvals

- Localhost-only admin UI and API workflow, with server-configured local instance IDs and explicit environment labels. The configured source/target is treated as the same local instance pending Q-08 confirmation; no arbitrary endpoint entry.
- A new backup domain owning durable job metadata, migrations, idempotency, state machine, lifecycle ledger and audit integration; gateway composition through existing domain contracts, not feature logic in gateway.
- Synthetic database-slice adapter and non-sensitive fixtures only. No real DB rows, Auth identities, object bytes or real dumps enter the fixture path. Durable metadata storage is allowed; it is not permission to capture application data.
- Authorized create/list/read/monitor operations, audit correlation and safe reason codes. Measured progress only. Terminal U1 backup status is `incomplete` or `failed`, never `succeeded`; all U1 download attempts are denied by the server.
- Authenticated administrator, instance-operation and resource-level authorization on every delivered path; fail-closed durable job acceptance and audit-before-effect checks.
- Synthetic private-file allocation and cleanup/reconciliation tests using approved finite configuration. Fault injection covers restart, missing/orphan records, active-work fencing, DB outage and bounded cleanup failure. Real-data staging remains disabled even if these tests pass.
- Clearly labeled synthetic upload/preflight/confirmation previews and cancellation/error paths; disabled real package handlers reject before package bytes are processed, persisted or extracted. Confirmation previews never enqueue a restore or mutate target data.
- Unit, contract, integration and admin UI tests for this slice after toolchain readiness is established. No passing tests or coverage are asserted by these design artifacts.

## Acceptance accounting

| Story / criteria | U1 evidence expected | Not delivered / cannot be claimed |
| --- | --- | --- |
| US-01 / AC-001 | Authorized local instance list; non-admin/unknown instance rejection; empty/unavailable list disables submission. | Remote or production instance support. |
| US-02 / AC-002 | Durable synthetic job creation/monitoring; truthful `incomplete` vs `failed`; download rejection through UI and direct API. | Actual database capture, complete ZIP, full-component verification or any downloadable backup. |
| US-03 / AC-003 | Real upload/staging denial; non-mutating fixture previews and negative-path tests. | Real package upload, archive validation or successful real-data preflight. |
| US-04 / AC-004 | Labeled synthetic impact preview; cancellation/no confirmation produces no mutation; direct restore calls denied. | Actual replacement, recovery point, restore, rollback or successful post-restore checks. |
| US-05 / AC-001/005 | Server-side actor/administrator/instance/resource checks, safe audit, audit-failure refusal and secret-canary tests for delivered operations. | Proof of encrypted real-package staging or a production approval workflow. |
| US-06 / AC-002/005 | Stable references, safe failure/incomplete reasons, cleanup failure visibility and intervention diagnosis. | Automatic resume, exactly-once execution, or evidence of recovery from real data loss. |
| AC-006 | Written resolutions/approvals and dependency checks before construction; deferred capability gates stay explicit. | Closing open questions because a document or fixture exists. |

## Deferred work requiring a later decision and evidence

- Real database capture/staging; private provider selection, finite budgets and expiry, encryption/key controls, reviewed cleanup implementation and real-data safety tests before enablement (Q-03/Q-05/Q-11). This is deferred even though the eventual capability remains database-only at first.
- Real package creation/download/upload, ZIP/manifest/checksum validation and supported format/version contracts. Future validators must cover path traversal, absolute paths, links, entry collisions, bombs and expansion limits before extraction.
- Supported Supabase hosting/version/schema/Auth/Storage recovery research, adapters and representative restore tests (Q-04). Q-04 remains open; no supported recovery matrix exists here.
- Full backup success semantics, consistency guarantees and session/write-impact policy (Q-10).
- Real preflight/restore confirmation binding, recovery points, replacement, verification, rollback and recovery-required operations.
- CLI completion and API-backed CLI parity; existing skeleton remains fail-closed, with no working S3 transfer claim (C-04).
- Cross-instance ID/URL/domain/bucket mapping (Q-08), remote deployment and multi-process/host coordination.
- Owner-approved RPO/RTO, drill frequency/success criteria and production-readiness evaluation (Q-06), and production authority/second-person policy (Q-07).

Deferred does not mean automatically included in U2. Later scope and safety approvals determine sequencing.

## Out of scope / expressly not authorized

- Any production restore implementation, execution, test against production, deployment or release action (C-03).
- Marking database-only work `succeeded`, exposing incomplete downloads, enabling real adapters through a client flag, or presenting synthetic tests as disaster-recovery proof (C-01/C-02).
- Executing commands/scripts supplied by an archive; accepting browser-provided secrets, connection strings or arbitrary instance endpoints (FEAT-01 BR-02/BR-06).
- Exporting project settings, secrets, OAuth provider configuration, edge functions or deployment configuration; FEAT-01 §2.2 excludes these by default.
- Scheduled backups, reinstating the superseded manual-S3 policy, unbounded retention, broad unrelated domain refactoring or unrelated coverage-report-failure repair.
- Workflow-state edits, gate approval/reporting or externally publishing these repository artifacts by this worker.

## Construction blockers and evidence needed

| Blocker | Required resolution / evidence | Responsible role | Status |
| --- | --- | --- | --- |
| Q-01 / R-06 | Explicit written approval of ADR-002, status examples, enum consumer impact and compatibility plan. | Owner and architect | Proposed in principle; approval absent. |
| Q-02 / R-09 | Explicit written approval of ADR-003, named domain/schema/migration ownership, integration and audit adapter boundaries. | Owner/architect, development lead | Proposed in principle; approval absent. |
| Q-03 / R-01/R-02 | Explicit written approval of ADR-004's durable allocation protocol, inventory, fencing, outage behavior, storage protection, escalation and crash-test exit criteria. Deferring real staging alone is insufficient. | Owner/architect with operations | Proposed in principle; detailed policy inputs absent. |
| Q-05 / C-06 / R-04 | Owner-sourced finite bounds for enabled operations: expiry, orphan policy, sweep/retry/deadline/temporary-byte/concurrency and metadata/idempotency retention; selected protected fixture provider/location. Approve applicable protection/key boundary. | Owner and operations; security for protection | Pending. Missing mandatory values refuse work; no invented defaults. |
| A-03/A-05 / Q-08/Q-09 | Confirm admin-first synthetic-only delivery and configured-local same-instance scope; approve the limited acceptance accounting above. | Product/system owner | Pending. |
| A-04 / R-05 | Approve administrator-instance-job authorization matrix and audit durability/transaction design; security review addresses the threat model. | Owner/architect and security reviewer | Pending; role mounting alone does not settle resource policy. |
| C-05 / Q-12 / R-07 | Establish working Rust/Cargo, frontend and required integration-test environment; run baseline from the Rust workspace and web project, document pre-existing failures and quality gate feasibility. | Development/QA | Unverified here. |
| Ideate gate and continuation | Conductor records the correct gate disposition and reviewed versions; updated plan/design must address the prior U1 stoppage. AI gate approval must not be substituted for required explicit written blocker approvals. | Conductor within its authority; owner/architect for blockers | Not performed by this worker. |

The three critical questions are resolved by the recommendation **in principle**, not closed in fact. Construction resumption remains blocked until their explicit written approvals exist. Missing finite configuration/protection blocks the lifecycle/worker components that depend on it; the recommendation does not authorize coding around missing inputs. Q-06 values are pending owner decisions but do not block this synthetic U1 slice; they block production readiness. Broader Q-05 package bounds may stay open while real package operations are absent, but all bounds used by the fixture lifecycle/worker must be supplied before affected construction.

## Later real-data enablement checklist

Before real staging: approve the provider/protection/finite policies; implement and test the reviewed cleanup/reconciliation design, including real-data confidentiality and crash behavior; establish secure transport/key boundaries; obtain a separate enablement decision. No U1 fixture-only test satisfies this gate by itself. Before production restore: satisfy and approve every FEAT-01 §7 condition, including Q-04/Q-06/Q-07/Q-10/Q-11 and representative recovery/rollback evidence. No such readiness is claimed.

## Verification limits

Document/source review only. FEAT-01 §12 records historical `domain_posts` failures and Docker-related ignored tests; these are not current results. Analysis reported Cargo unavailable, and this worker did not run Rust/frontend/integration/security tests, benchmarks, coverage, capture or Supabase recovery. Current baseline and build readiness remain unknown.
