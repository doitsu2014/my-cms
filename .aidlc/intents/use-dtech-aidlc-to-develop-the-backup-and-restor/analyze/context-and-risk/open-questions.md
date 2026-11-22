# Open Questions

## Purpose
This document captures questions that must be answered by stakeholders before design or construction can proceed for affected capabilities.

---

## Q-01: Status Contract Mapping (Critical)
- **Question:** How should the internal `incomplete` status map to the public job status contract enum (`queued | running | succeeded | failed | recovery-required`)?
- **Context:** U1 database-only jobs must end `incomplete` or `failed` per the approved scope (FEAT-01 §§10.2, 12). The public contract does not include `incomplete`. Prior U1 stopped on this mismatch.
- **Options:**
  1. Add `incomplete` to the public enum as a valid terminal state
  2. Map internal `incomplete` → public `failed` with a specific reason code indicating partial capture
  3. Use a separate U1-specific contract and migrate to public contract when full-stack backup is supported
- **Impact if Unanswered:** Cannot design job state machine, database schema, API responses, or UI status displays. Critical blocker from prior U1 (R-01, R-06).
- **Stakeholder:** System/product owner, architect
- **Priority:** Critical — must resolve before Ideate

---

## Q-02: Persistence and Migration Ownership (Critical)
- **Question:** Which component owns backup job persistence, database schema migrations, and the job state machine?
- **Context:** Current architecture has domain-owned logic and migrations with gateway as composition root (MAP-01 §6). Backup/restore is cross-cutting (database, Auth, Storage). Prior U1 listed this as unresolved (FEAT-01 §12).
- **Options:**
  1. Create a new backup domain crate with its own migrations and entities
  2. Gateway-owned with adapters to existing domains
  3. Distributed: each domain handles its own backup slice, gateway orchestrates
  4. Shared infrastructure domain
- **Impact if Unanswered:** Cannot write schema migrations, create entities, or place API endpoints. Critical blocker from prior U1 (R-01, R-09, U-02).
- **Stakeholder:** Architect, development team lead
- **Priority:** Critical — must resolve before Ideate

---

## Q-03: Cleanup and Reconciliation Design (Critical)
- **Question:** What is the durable mechanism for identifying and cleaning up orphaned staging data after crashes, and what are the finite expiry/retention bounds?
- **Context:** Real-data staging is explicitly disabled until this is designed, implemented, and tested (FEAT-01 §10.2, §12, C-02). Without it, upload/preflight/restore cannot handle real packages.
- **Requirements:**
  - Finite time bounds for staging data expiry
  - Crash-safe orphan detection (survives process restart)
  - Bounded retry for cleanup operations
  - No silent failure to clean up
- **Impact if Unanswered:** Real-data staging remains disabled. U1 limited to test doubles or no-op stubs for upload/restore workflows. Critical blocker from prior U1 (R-01, R-02, U-03).
- **Stakeholder:** Architect, development team, operations
- **Priority:** Critical if real staging is in U1 scope; High if deferred to U2

---

## Q-04: Supported Supabase Scope (Critical)
- **Question:** What Supabase versions, hosting models (self-hosted, hosted, both), schema scope, Auth providers, and Storage metadata are officially supported for export and recovery?
- **Context:** Feature spec asks this explicitly (FEAT-01 §8 questions 1-3). Codebase map notes restore refuses to run because Supabase recovery is unverified (MAP-01 §3). Full recovery support is unverified (FEAT-01 §1.2, §2.1).
- **Specific Sub-Questions:**
  - Which Supabase versions? (e.g., "v2.x self-hosted only")
  - Self-hosted, Supabase-hosted, or both?
  - Which schema/tables: My-CMS application data only, or also GoTrue/auth schema?
  - Auth: password hashes, OAuth tokens, session data, provider configurations?
  - Storage: object bytes only, or also bucket metadata, policies, access logs?
  - System-managed Supabase schema included or excluded?
- **Impact if Unanswered:** Cannot design Auth/Storage capture or verify full-stack recovery. Cannot claim supported scope in documentation. Critical to moving beyond database-only (R-03, U-04).
- **Stakeholder:** System/product owner, architect, platform engineer
- **Priority:** Critical for full-stack backup; can be deferred if U1 remains database-only

---

## Q-05: Package Limits and Retention Policy (High)
- **Question:** What are the finite bounds on package size, archive entry count, upload size, staging duration, retention period, and storage location?
- **Context:** Specification requires finite limits for safety (FEAT-01 §§6, 10.1). No numeric thresholds are approved (C-06, U-05).
- **Specific Values Needed:**
  - Maximum package ZIP size (e.g., "5 GB")
  - Maximum archive entries (e.g., "100,000 files")
  - Maximum upload request size (e.g., "2 GB per chunk")
  - Staging data expiry (e.g., "24 hours")
  - Package retention period (e.g., "30 days")
  - Storage location (local filesystem, S3, other?)
- **Impact if Unanswered:** Cannot implement staging validation, archive safety checks, or retention cleanup. Unbounded defaults are unsafe (R-04, U-05).
- **Stakeholder:** System/product owner, operations, infrastructure
- **Priority:** High — needed before real staging design

---

## Q-06: RPO/RTO and Restore Drill Requirements (High)
- **Question:** What are the approved recovery point objective (RPO), recovery time objective (RTO), and restore drill frequency/success criteria?
- **Context:** Feature spec lists these as readiness conditions (FEAT-01 §7, §8 question 8) but provides no approved values (SUCCESS-01).
- **Specific Values Needed:**
  - RPO: acceptable data loss window (e.g., "24 hours")
  - RTO: acceptable recovery time (e.g., "4 hours")
  - Restore drill frequency (e.g., "quarterly")
  - Drill success criteria (e.g., "successful restore with <1% data variance")
- **Impact if Unanswered:** Cannot design backup scheduling, validate recovery times, or assess production readiness. Not needed for U1 but required for production (R-04, U-06).
- **Stakeholder:** System/product owner, operations
- **Priority:** High for production planning; Low for U1

---

## Q-07: Production Authorization and Second-Person Approval (High)
- **Question:** Who is authorized to restore into production, and is second-person approval required?
- **Context:** Feature spec asks this explicitly (FEAT-01 §8 question 7) and lists it as a release condition (FEAT-01 §7 item 7). Production restore is not authorized in current framing (C-03).
- **Specific Sub-Questions:**
  - Role requirements for production restore (e.g., "senior admin" vs. "any admin")
  - Second-person approval required? (e.g., "yes, via signed request")
  - Approval workflow (manual sign-off, ticketing system, other?)
  - Audit requirements beyond standard audit log
- **Impact if Unanswered:** Cannot design production authorization model or approval workflows. Not needed while production restore is out of scope (C-03) but required before enabling production (R-05, U-07).
- **Stakeholder:** System/product owner, security/compliance
- **Priority:** High for production; N/A for U1 scope

---

## Q-08: Cross-Instance Restore Requirements (Medium)
- **Question:** Can the target instance differ from the source instance, and if so, what ID/URL/domain/bucket mapping is required?
- **Context:** Feature spec asks this explicitly (FEAT-01 §8 question 5). U1 localhost-only scope may imply same-instance only (A-05).
- **Specific Sub-Questions:**
  - Restore to same instance only, or cross-instance supported?
  - If cross-instance: database ID remapping needed?
  - URL/domain rewriting for Storage object references?
  - Bucket name translation?
  - How to handle referential integrity across remapped IDs?
- **Impact if Unanswered:** Cannot determine whether cross-instance complexity is in scope. U1 may proceed assuming same-instance only, but broader target scope is unclear (R-08, U-05).
- **Stakeholder:** System/product owner, architect
- **Priority:** Medium — clarify for U1 scope; critical if cross-instance is target

---

## Q-09: CLI vs. UI Delivery Path (Medium)
- **Question:** Is U1 delivered via CLI completion, admin UI endpoints, or both?
- **Context:** Existing `backup_cli.rs` skeleton is incomplete and non-functional (C-04). User stories are framed for admin UI operators (FEAT-01 §§1.2, 3, US-01–US-06), but CLI exists (A-03).
- **Options:**
  1. Complete the CLI; admin UI calls CLI via subprocess or wraps its functionality
  2. Implement server-side endpoints; admin UI calls API; CLI is deprecated or reworked later
  3. Both: CLI for operators, API for UI, shared core logic
- **Impact if Unanswered:** Cannot determine where to place implementation effort. Risk of wasted work on CLI skeleton if UI path is chosen (R-12).
- **Stakeholder:** Product owner, development team
- **Priority:** Medium — should clarify before Ideate to guide design

---

## Q-10: Consistency Guarantees (Medium)
- **Question:** What level of consistency is required for a backup, and what are the acceptable limits/impacts on user sessions?
- **Context:** Feature spec warns not to claim point-in-time consistency unless proven (FEAT-01 §§2.1, 6) and asks about pausing writes during backup (FEAT-01 §8 question 4).
- **Specific Sub-Questions:**
  - Is best-effort consistency acceptable, or must it be transactionally consistent?
  - Can backup pause writes (maintenance window), or must it run online?
  - Acceptable session impact during backup (e.g., "read-only mode" vs. "no impact")?
  - Consistency between database, Auth, and Storage: required or best-effort?
- **Impact if Unanswered:** Cannot design backup orchestration or set user expectations. May over-promise consistency or under-deliver on user experience (R-03, U-04).
- **Stakeholder:** System/product owner, operations, architect
- **Priority:** Medium — needed for full-stack backup design

---

## Q-11: Encryption and Key Management (Medium)
- **Question:** Are backup packages encrypted at rest and in transit, and how are encryption keys managed?
- **Context:** Listed as a release condition (FEAT-01 §7 item 4) and NFR (FEAT-01 §6). Not detailed in U1 scope.
- **Specific Sub-Questions:**
  - Encryption required for staging data?
  - Encryption required for downloadable packages?
  - Key management: instance-specific keys, user-provided keys, or key service?
  - Key rotation and access policies?
- **Impact if Unanswered:** Cannot finalize staging security design or package format. Not critical for U1 if production is out of scope, but needed for production readiness.
- **Stakeholder:** Security/compliance, architect
- **Priority:** Medium for design; High for production

---

## Q-12: Test Environment and Baseline (Low)
- **Question:** What is the current Rust test suite baseline (pass/fail counts), and is the test execution environment available for construction?
- **Context:** Cargo toolchain unavailable in analysis environment (C-05, MAP-01 §7). Test baseline unknown (A-07).
- **Specific Sub-Questions:**
  - How many tests currently pass/fail in `apps/api`?
  - Are there known flaky or environment-dependent tests?
  - Is Docker/Supabase available for integration tests?
  - Coverage tooling (grcov) availability status?
- **Impact if Unanswered:** Cannot verify test execution readiness or establish quality gate baselines. Low risk if construction environment is known to have working Rust toolchain (R-07).
- **Stakeholder:** Development team, QA
- **Priority:** Low for analysis; Medium for construction day-one

---

## Questions Summary Table

| ID | Question | Priority | Blocks | Stakeholder |
|----|----------|----------|--------|-------------|
| Q-01 | Status contract mapping | Critical | State machine, API, UI | Owner, architect |
| Q-02 | Persistence ownership | Critical | Schema, migrations, endpoints | Architect, dev lead |
| Q-03 | Cleanup/reconciliation | Critical (if U1 staging) | Real staging enablement | Architect, dev, ops |
| Q-04 | Supabase scope | Critical (for full-stack) | Auth/Storage design | Owner, architect, platform |
| Q-05 | Package limits | High | Staging validation | Owner, ops, infra |
| Q-06 | RPO/RTO/drills | High (for production) | Production readiness | Owner, ops |
| Q-07 | Production authorization | High (for production) | Authorization model | Owner, security |
| Q-08 | Cross-instance restore | Medium | Scope clarity | Owner, architect |
| Q-09 | CLI vs. UI path | Medium | Implementation strategy | Owner, dev team |
| Q-10 | Consistency guarantees | Medium | Backup orchestration | Owner, ops, architect |
| Q-11 | Encryption/keys | Medium | Security design | Security, architect |
| Q-12 | Test baseline | Low | Quality gates | Dev team, QA |

**Critical questions requiring resolution before Ideate:** Q-01, Q-02, Q-03 (if staging in U1), Q-04 (if beyond database-only)
