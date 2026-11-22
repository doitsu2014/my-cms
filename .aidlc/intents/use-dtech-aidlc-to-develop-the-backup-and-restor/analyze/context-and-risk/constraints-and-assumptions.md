# Constraints and Assumptions

## Constraints

### C-01: U1 Scope Limitation (Observed)
- **Source:** FEAT-01 §§10–13, AC-01 AC-002 scenario 3, SUCCESS-01 SM-02
- **Statement:** U1 implementation is constrained to localhost-only operation and database-only capture.
- **Implication:** Any U1 database-only backup job MUST end with status `incomplete` or `failed`, never `succeeded`. Incomplete artifacts MUST NOT be downloadable. This is not a defect; it is the approved U1 contract.
- **Rationale:** Auth and Storage components are required for a complete backup per the target specification. U1 deliberately captures only the database slice to validate the workflow without the unresolved full-stack concerns.

### C-02: Real-Data Staging Disabled (Observed)
- **Source:** FEAT-01 §§10.2, 12, AC-01 AC-003 scenario 3, SUCCESS-01 SM-02
- **Statement:** Real-data staging for backup and restore remains disabled until finite-expiry cleanup and durable crash reconciliation mechanisms are designed, implemented, and tested.
- **Implication:** Upload/preflight/restore workflows cannot process real backup data in U1. Test doubles or no-op stubs are acceptable for U1; production-quality staging is not authorized.
- **Rationale:** Without bounded expiry and orphan cleanup after crashes, staging could accumulate unbounded temporary data or leak sensitive content.

### C-03: Production Restore Not Authorized (Observed)
- **Source:** FEAT-01 §§7, 13, PROB-01, STORY-01, SUCCESS-01
- **Statement:** Restore operations targeting production environments are explicitly not authorized by this framing or the U1 scope.
- **Implication:** No implementation, testing, or approval workflow for production restore is in scope. Instance selection and authorization must enforce this boundary.
- **Rationale:** Production restore requires verified Auth/Storage recovery, consistency guarantees, encryption/key management, tested rollback, and approved recovery drills per the release conditions (FEAT-01 §7). None of these are resolved.

### C-04: CLI Skeleton Incomplete (Observed)
- **Source:** MAP-01 §3, FEAT-01 §9
- **Statement:** The existing `backup_cli.rs` skeleton has no S3 transfer adapter (create/list fail) and restore deliberately refuses to run because Supabase recovery is unverified.
- **Implication:** No working backup or restore flow exists. Implementation must either complete the CLI or provide an alternative path (e.g., admin UI invoking new server-side endpoints).
- **Evidence:** File `apps/api/gateway/src/backup_cli.rs` exists (6862 bytes) but is non-functional per MAP-01.

### C-05: Cargo Toolchain Unavailable (Observed)
- **Source:** MAP-01 §7, BUILD-01
- **Statement:** The Rust toolchain (cargo) is not available in the current analysis environment.
- **Implication:** Cannot execute `cargo test` to establish the current test suite baseline. Test status must be documented as unknown rather than assumed passing or failing.
- **Mitigation:** Development and construction stages will run in environments with Rust toolchain available.

### C-06: No Numeric Requirements (Observed)
- **Source:** FEAT-01 §§8, 10, SUCCESS-01
- **Statement:** Numeric thresholds for RPO, RTO, package size limits, retention policies, concurrency limits, and SLOs have not been approved by the system owner.
- **Implication:** Implementation cannot assume default values. Designs requiring numeric bounds must flag them as open questions and request owner decisions before proceeding.

### C-07: Brownfield Integration (Observed)
- **Source:** MAP-01 §1, FEAT-01 §13
- **Statement:** My-CMS is a brownfield system with existing Rust API gateway, React admin UI, and self-hosted Supabase (PostgreSQL/GoTrue/Storage).
- **Implication:** New backup/restore code must integrate with the existing Axum/Tokio/SeaORM stack, domain ownership conventions, and admin UI patterns without disrupting operational services.

## Assumptions

### A-01: Feature Specification Completeness (Assumption)
- **Statement:** The feature specification (FEAT-01) is treated as the authoritative source for business rules, acceptance criteria, and NFRs for the purpose of this analysis baseline.
- **Risk:** The specification explicitly lists many open questions (FEAT-01 §8) and unresolved U1 blockers (FEAT-01 §12). Treating it as complete for framing purposes does not mean those questions are answered.
- **Validation Required:** Product owner or business analyst must resolve the open questions in §8 before design or construction of capabilities dependent on those answers.

### A-02: Supabase Self-Hosted (Assumption)
- **Statement:** The deployment model is self-hosted Supabase based on MAP-01 and project context.
- **Risk:** The feature specification asks whether Supabase hosted, self-hosted, or both are supported (FEAT-01 §8 question 1). This analysis assumes self-hosted based on current codebase evidence, but the answer may expand scope.
- **Validation Required:** Confirm supported Supabase hosting models and versions before Auth/Storage export design.

### A-03: Admin UI Is Primary Interface (Assumption)
- **Statement:** The target workflow requires admin UI support for instance selection, job creation/monitoring, package download, upload, preflight, and restore confirmation (FEAT-01 §§1.1, 3, US-01 through US-06).
- **Risk:** The existing CLI skeleton could be completed as an alternative, but user stories are framed for admin operators using a UI, not command-line operators.
- **Validation Required:** Confirm whether CLI completion, admin UI endpoints, or both are the delivery target for U1.

### A-04: Server-Side Authorization Enforced (Assumption)
- **Statement:** All backup/restore operations enforce server-side authorization checks and do not trust client-provided instance identifiers, credentials, or connection strings (FEAT-01 §§5, 10.1, BR-01, BR-02).
- **Risk:** This is a security requirement stated in the spec, but implementation patterns are not yet designed.
- **Validation Required:** Design review must verify authorization enforcement before construction.

### A-05: No Cross-Instance Mapping in U1 (Assumption)
- **Statement:** U1 localhost-only scope implies source and target are the same instance, so cross-instance ID/domain/bucket mapping is not required.
- **Risk:** FEAT-01 §8 question 5 asks whether target can differ from source. If yes, mapping concerns must be addressed even in U1 test scenarios.
- **Validation Required:** Clarify whether U1 includes separate source/target instances or only same-instance restore.

### A-06: Existing Domain Patterns Remain (Assumption)
- **Statement:** The current domain ownership and gateway composition root pattern (MAP-01 §6) will be preserved; backup/restore feature will follow the same conventions.
- **Risk:** Backup/restore is a cross-cutting capability touching database, Auth, and Storage. Ownership and layering decisions must be made explicitly.
- **Validation Required:** Architecture review to confirm where backup/restore logic, endpoints, persistence, and orchestration belong in the domain model.

### A-07: Test Suite Baseline Unknown but Not Blocking (Assumption)
- **Statement:** The current Rust test suite baseline is unknown due to toolchain unavailability (C-05). This analysis proceeds assuming that test execution will be possible during construction.
- **Risk:** If the suite has pre-existing failures or the environment cannot run tests, construction quality gates may be blocked.
- **Validation Required:** Establish test execution environment and baseline before construction begins.

### A-08: English Communication for Workflow (Assumption)
- **Statement:** AI-DLC workflow artifacts are authored in English even though the feature specification is in Vietnamese.
- **Risk:** Potential translation or interpretation errors when mapping Vietnamese requirements to English analysis documents.
- **Validation Required:** Bilingual review of requirements interpretation if inconsistencies are suspected.

## Unknown / Requires Decision

### U-01: Status Contract Mapping
- **Source:** FEAT-01 §12 blocker 2
- **Question:** How should the internal `incomplete` status map to the public contract enum (`queued | running | succeeded | failed | recovery-required`)? Is `incomplete` surfaced as-is, or mapped to `failed` with a specific reason code?
- **Blocker:** Prior U1 stopped on this mismatch. Must be resolved before job status implementation.

### U-02: Persistence and Migration Ownership
- **Source:** FEAT-01 §12 blocker 1, MAP-01 §6
- **Question:** Which domain owns backup job persistence, schema migrations, and the job state machine? Is there a new backup domain, or does this live in the gateway?
- **Blocker:** Cannot design database schema or write migrations without ownership decision.

### U-03: Cleanup and Reconciliation Design
- **Source:** FEAT-01 §12 blocker 1, C-02
- **Question:** What is the durable mechanism for identifying and cleaning up orphaned staging data after crashes, and what are the finite expiry/retention bounds?
- **Blocker:** Real-data staging cannot be enabled until this is designed, tested, and proven.

### U-04: Supported Supabase Scope
- **Source:** FEAT-01 §§2.1, 8 questions 1–3
- **Question:** What Supabase versions, hosting models, schema scope (including GoTrue internals), Auth provider configuration, and Storage metadata are officially supported for export and recovery?
- **Blocker:** Cannot design Auth/Storage capture or verify full-stack recovery without this scope definition.

### U-05: Package Limits and Retention Policy
- **Source:** FEAT-01 §§6, 8 question 6, 10.1
- **Question:** What are the finite bounds on package size, archive entry count, upload size, staging duration, retention period, and storage location?
- **Blocker:** Cannot implement safe staging or package validation without finite limits.

### U-06: RPO/RTO and Restore Drill Requirements
- **Source:** FEAT-01 §§7, 8 question 8, SUCCESS-01
- **Question:** What are the approved recovery point objective (RPO), recovery time objective (RTO), and restore drill frequency/success criteria?
- **Blocker:** Production readiness gate cannot be evaluated without these targets.

### U-07: Production Authorization and Second-Person Approval
- **Source:** FEAT-01 §§7, 8 question 7
- **Question:** Who is authorized to restore into production, and is second-person approval required?
- **Blocker:** Authorization model design requires this policy decision.
