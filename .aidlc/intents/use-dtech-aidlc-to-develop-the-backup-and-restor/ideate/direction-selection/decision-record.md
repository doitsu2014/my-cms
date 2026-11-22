# Decision record: backup/restore U1

**Status:** Proposed ADRs for review, dated 2026-11-21. No acceptance, approver identity or approval date is recorded. The selected recommendation is A; an approved choice has not been recorded. Written owner/architect approval of Q-01/Q-02/Q-03 remains a preconstruction requirement even in AI orchestrator mode.

## Numbering and provenance

No prior numbered ADR was found in the feature specification, codebase map, registered source catalog (`aidlc.knowledge.json` has no ADR source), repository ADR filename search, or repository search for numbered ADR identifiers. Accordingly this record begins at ADR-001; no existing accepted ADR is declared superseded. The historical manual-S3 policy in FEAT-01 §11 is already superseded by the admin ZIP specification; it is not a newly discovered accepted ADR. If an unindexed higher-numbered record is supplied, renumber these proposals before acceptance rather than creating duplicate identifiers.

Shared inputs: [options](../option-generation/solution-options.md), [comparison](../option-generation/trade-off-matrix.md), [FEAT-01](../../../../../docs/features/backup-restore.md), [MAP-01](../../../../knowledge/_codebase-map.md), [questions](../../analyze/context-and-risk/open-questions.md), and [constraints](../../analyze/context-and-risk/constraints-and-assumptions.md). Proposed implementation details are not observed capabilities.

## ADR-001 — Select an admin-first synthetic U1 workflow

**Status:** Proposed. **Questions:** Q-08/Q-09; critical Q-03 boundary. **Trace:** C-01–C-04, US-01–06, AC-001–006, R-02/R-08/R-12.

### Context

The documented target is an admin UI workflow; the CLI is incomplete. U1 is localhost-only and database-only, and real-data staging is prohibited until cleanup/reconciliation is designed and tested. The option-generation baseline further limits U1 to synthetic fixtures and previews. A database-only prototype cannot deliver a complete restorable package.

### Decision

Recommend Option A. Deliver admin UI/API allowed-instance selection, synthetic database-slice job creation and monitoring, authorized diagnostics and audit. Show upload/preflight/confirmation only as explicitly labeled synthetic previews or unavailable actions; real request paths reject before package body processing, persistence or extraction. No restore adapter or production target is enabled. No real dump is an incidental shortcut. Use the configured local-instance-only interpretation pending Q-08 approval. Preserve existing CLI refusals; do not complete, remove or silently repurpose the CLI in U1.

### Consequences

Directly validates the U1 slice of US-01/02/05/06 and prepares reusable UI/API foundations. Complete ZIP/download under US-02 and real US-03/04 remain undelivered. Relative effort is medium; no elapsed-time commitment is made. Same-instance and interface-priority assumptions require owner confirmation. A fixture demonstration proves neither database export nor Supabase recovery. Changing these limits requires reopening the affected ADRs and safety gates.

### Alternatives

B adds an authenticated CLI client and preserves enum compatibility, but increases scope and semantic translation. C provides a faster CLI interface, but sacrifices UI validation and requires an explicit gateway-ownership exception. Both remain credible if the owner changes interface priorities. Real staging with placeholder cleanup is prohibited, not a selectable alternative.

## ADR-002 — Expose terminal incomplete without semantic projection

**Status:** Proposed; Q-01 not closed. **Trace:** C-01, AC-002/006, FEAT-01 §§10.2/12, R-01/R-06.

### Context

The documented general public enum lacks `incomplete`, while U1 must terminate `incomplete` or `failed`. Translating planned partial capture into operational failure is avoidable but public expansion can break exhaustive consumers.

### Decision

Add terminal `incomplete` to the general public job contract, retaining existing values for compatibility with future/full workflows. Persist and serialize the same canonical outcome and handle it explicitly in UI and audit. U1 permits `queued → running → incomplete|failed` and `queued → failed` on guarded dispatch/reconciliation failure. `incomplete` means the synthetic database-slice workflow finished but required full-backup components are unavailable; `failed` means an attempted step failed, timed out or has an unknown interrupted result. U1 must never transition to `succeeded` or imply a restorable package. Keep `recovery-required` as a general restore concept, not a successful U1 backup outcome. Always deny U1 downloads server-side and expose no package URL; file existence cannot change eligibility. Require a consumer inventory and coordinated schema/serializer/UI rollout before accepting the expansion.

### Consequences

Public and internal semantics remain aligned, with no failed-reason translation. Safe reason codes and per-component capability metadata still explain outcomes. Enum consumers and any historical serialization assumptions require review; no unreviewed historical data rewrite is authorized. A failed terminal write leaves durable non-success state for reconciliation, never a fabricated final result.

### Alternatives

B preserves the enum by projecting internal `incomplete` to public `failed` with mandatory reason; choose it if compatibility outweighs semantic clarity. C isolates a versioned U1 contract but defers migration costs. Reporting database-only work as `succeeded` violates C-01 and is rejected.

## ADR-003 — Assign backup state and migration ownership to a domain

**Status:** Proposed; Q-02 not closed. **Trace:** C-07, MAP-01 §6, FEAT-01 §12, R-01/R-09.

### Context

Backup spans several data systems, but shared infrastructure does not confer shared table ownership. Existing architecture assigns routes, feature logic, entities and migrations to domains; the gateway composes services.

### Decision

Introduce proposed `apps/api/domain_backup/` as owner of job repositories/entities, idempotency records, state machine, allocation/cleanup ledger, migrations, domain-owned routes, worker orchestration and audit integration. Register it through existing `DomainService`/`RouteRegistration`/`MigrationDescriptor` contracts. Its routes use `Mount::Administrator` and resource checks in application handlers. The gateway loads dependencies, registers the domain, dispatches its declared migrations and applies common authentication/cookie/transport layers once. It does not own backup handlers or entities. Domain migrations affect only backup-owned metadata; no Auth/Storage recovery schema is claimed. The domain owns audit write guarantees for this feature; the concrete audit adapter/transaction boundary must be approved, not assumed to exist already.

### Consequences

Ownership is coherent and U2 adapters have a stable home. A new crate and migrations need workspace integration, migration-order/upgrade testing and explicit schema review. Existing posts/media/user ownership stays unchanged. `domain_interface` remains a stable composition contract, not a home for backup business logic. No cross-domain transaction or common audit service is implied merely by sharing the database pool.

### Alternatives

Gateway ownership shortens initial placement but contradicts the architectural rule and invites extraction. Distributed capture ownership fragments lifecycle responsibility. A generic shared-infrastructure domain lacks the clear feature ownership offered by a dedicated backup domain; it adds no demonstrated U1 benefit.

## ADR-004 — Use a durable allocation ledger and fail-closed reconciliation

**Status:** Proposed; Q-03 not closed; Q-05 values and storage-protection approval missing. **Trace:** C-02/C-06, AC-003/006, R-01/R-02/R-04, FEAT-01 §§10.1–10.4/12.

### Context

A crash can leave files without jobs or jobs without files. In-memory cleanup and deletion-on-success do not cover these windows. Finite expiry, bounded retry and durable orphan discovery are required; real staging remains disabled.

### Decision

Use a backup-owned database ledger and private, feature-owned storage namespace. The following protocol is proposed for synthetic lifecycle validation and future reviewed implementation; it grants no real-data enablement:

1. Validate approved finite bounds and storage protection. Commit allocation intent with opaque allocation ID, job association, owner/state version, expiry, cleanup state, retry count and next attempt before any synthetic file allocation. Derive paths from trusted opaque identifiers, never client paths. No file allocation when ledger/audit persistence is unavailable.
2. Write only to the private namespace using an injected fixture storage adapter. Record allocation completion conditionally. Missing files after a committed intent are harmless reconciliation cases, not successful capture.
3. At startup, fence admissions and new allocation until a reconciliation pass can establish safe state. One local process owns the namespace; overlapping processes must be refused by an exclusive ownership mechanism. That mechanism's implementation and Windows behavior require review and tests. State-version checks serialize worker and sweeper claims; an expired deadline alone is not permission to delete active work.
4. On startup and recurring sweeps, compare ledger, job state and namespace inventory in both directions. Fence interrupted prior-process work, mark its job non-success where possible, and schedule inactive/expired allocations for cleanup. Inventory entries without ledger records must be within a verifiably feature-owned namespace; persist a discovered-orphan record before deletion, applying the approved orphan grace/expiry policy. Ambiguous ownership is not deleted; emit a safe incident requiring operator review.
5. A cleaner claims inactive allocations with a conditional transition to deleting before unlinking. Cleanup is idempotent: a missing file counts as already removed. Commit the final deleted state after deletion; a crash in between retries safely. An active worker cannot reacquire an allocation claimed for deletion.
6. Retry only idempotent transient cleanup failures within an approved finite budget. Exhaustion leaves a durable visible failed-cleanup record with safe reason and escalation responsibility, not success or disappearing evidence. Expiry marks cleanup due; physical deletion cannot be guaranteed during host/storage outages.
7. If the metadata database is unavailable, stop new work and deletion because ownership/expiry cannot be trusted; emit a safe local health signal without file content or raw paths. Retain private files rather than guessing ownership. On recovery, reconcile before resuming synthetic admissions. This may exceed expiry; operations must resolve the incident and measure cleanup lag. Storage unavailability similarly leaves pending cleanup visible in durable metadata where available.
8. Test every write-order/crash window, inventory mismatch, active-work race, delete failure, restart and database outage with non-sensitive fixtures. No automatic job resume or exactly-once guarantee. Even passing synthetic tests does not enable real staging; separate implementation/safety evidence and approval are mandatory.

The approved configuration must supply finite allocation expiry, orphan handling/grace, sweep cadence, retry budget/backoff, operation/whole-job deadlines, temporary byte budget, fixture/entry limits where applicable, concurrency, and metadata/idempotency retention. No numeric defaults are selected. Storage location, least-privilege protection, encryption policy/key boundary where applicable and escalation owner must be named before affected construction.

### Consequences

Durable inventory supports restart discovery and one lifecycle owner. Database dependence avoids a second journal authority, but DB outages intentionally sacrifice cleanup timeliness for safe ownership checks. Namespace locking and fencing are explicit design/test obligations, not proven implementation. Missing mandatory values/protection leave Q-03 incomplete and affected construction blocked. Real data stays prohibited throughout U1.

### Alternatives

A filesystem journal (C) can work but creates journal/job-database divergence and platform durability obligations. In-memory lists, job-table-only enumeration and blind temp-directory deletion miss orphans or risk deleting active/unrelated files. Deferring real staging without defining this protocol does not resolve the prior U1 blocker.

## ADR-005 — Enforce authorization and durable audit before effects

**Status:** Proposed; security/resource policy and adapter review required. **Trace:** A-04, AC-001/005, US-05/06, FEAT-01 §§5/10.1, R-05.

### Context

Localhost clients and UI visibility are not security boundaries. Job IDs, arbitrary connection parameters, raw adapter errors and audit outages can expose or redirect sensitive operations.

### Decision

Use existing validated actor extraction and administrator route mounting, followed by domain-level operation/instance/resource policy on every access. A job creator relationship is recorded, but it does not by itself define who else may read it: owner/architect must approve the administrator-to-instance/job access matrix before construction. Accept only server-configured eligible local instance IDs; reject client URLs, credentials and connection strings. Revalidate policy at execution and deny unavailable capabilities independently of UI state. Durable acceptance commits job, actor/operation-scoped idempotency digest plus request fingerprint, and safe intent audit atomically when stored together; otherwise a reviewed durable audit-before-effect protocol is required. Do not perform work or return accepted until the required commit is durable. Same key/same request returns the authorized existing job; changed request under that key is rejected. Audit unavailability refuses sensitive effects. Emit safe projected errors and metadata only, no raw adapter bodies, credentials, content or raw idempotency keys. U1 runtime cannot select real capture/restore adapters, and previews cannot enqueue a mutating restore.

### Consequences

A direct API call cannot bypass protections by skipping the UI. Database/audit outages lower availability deliberately. Security, policy, transaction and secret-canary tests are required; the existing authentication stack is reused, not declared sufficient without tests. Future production policy, encryption/key management and real package access require separate decisions (Q-07/Q-11).

### Alternatives

Client-only authorization, localhost trust and best-effort audit after effects are rejected by AC-005. Duplicating global authentication layers inside the new domain conflicts with the existing route composition contract. Enabling a real adapter behind a browser-supplied flag is rejected.

## Approval ledger and unresolved decisions

| Item | Approval required | Current disposition |
| --- | --- | --- |
| ADR-001 and Q-08/Q-09 | Owner confirms admin-first synthetic, configured-local-only scope | Pending |
| ADR-002 / Q-01 | Written owner/architect contract and compatibility approval | Pending |
| ADR-003 / Q-02 | Written owner/architect ownership/schema/migration approval | Pending |
| ADR-004 / Q-03 | Written owner/architect lifecycle approval; operations confirms bounds, protection, escalation and fault-test criteria | Pending; Q-05 inputs missing |
| ADR-005 / A-04 | Owner/architect security/resource matrix, audit protocol and reviewer approval | Pending |

Q-04, Q-05, Q-06, Q-07, Q-10 and Q-11 remain open for the affected later capabilities; no recovery support, numeric policy or production authority is invented. Q-12 requires toolchain/baseline evidence before construction. Gate disposition and any AI orchestrator recommendation/report belong to the conductor, not this record.
