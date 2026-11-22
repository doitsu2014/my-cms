# Risk Register

## Purpose
This register identifies material risks to successful delivery of the backup/restore feature analysis baseline and subsequent development work.

## Risk Classification
- **Likelihood:** High (>50%), Medium (10-50%), Low (<10%)
- **Impact:** Critical (blocks delivery), High (major rework), Medium (delays), Low (minor adjustments)

---

## R-01: Unresolved U1 Design Blockers
- **Category:** Requirements / Design
- **Likelihood:** High
- **Impact:** Critical
- **Description:** The prior U1 code generation attempt stopped without making any application source changes due to three unresolved blockers: (1) persistence/migration ownership not decided, (2) status contract mismatch between public enum and internal `incomplete` state, (3) cleanup/reconciliation design not proven (FEAT-01 §12, PROB-01).
- **Consequence:** Cannot proceed to construction without explicit resolution of these blockers. Attempting to generate code without resolution will reproduce the same stoppage.
- **Mitigation:** Require explicit design decisions addressing each blocker before entering Ideate or construction phases. Document decisions in a design artifact with owner approval.
- **Status:** Open; blockers remain unresolved per current evidence.

---

## R-02: Real-Data Staging Implementation Gap
- **Category:** Technical / Safety
- **Likelihood:** High
- **Impact:** High
- **Description:** Real-data staging is explicitly disabled (C-02) until finite-expiry cleanup and durable crash reconciliation are designed and tested. Without this, the upload/preflight/restore workflow cannot handle actual backup packages.
- **Consequence:** U1 implementation may be limited to stub or test-double flows. Production-quality staging requires significant additional design and implementation work not yet scoped.
- **Mitigation:** Option 1: Accept U1 as workflow validation with test doubles; defer real staging to U2. Option 2: Prioritize cleanup/reconciliation design in Ideate and include in U1 scope with extended timeline.
- **Status:** Open; staging disabled per FEAT-01 §10.2, §12 and AC-003 scenario 3.

---

## R-03: Supabase Recovery Scope Unverified
- **Category:** Technical / Integration
- **Likelihood:** High
- **Impact:** Critical
- **Description:** Full Supabase export and recovery support is explicitly unverified (FEAT-01 §1.2, §2.1, §9). The feature specification asks which Supabase versions, hosting models, schema, Auth providers, and Storage metadata are supported (FEAT-01 §8 questions 1-3), and the codebase map notes that restore deliberately refuses to run because Supabase recovery is unverified (MAP-01 §9).
- **Consequence:** Cannot claim or implement complete backup/restore without verification. Auth (GoTrue) and Storage recovery may have platform limitations or version-specific requirements not yet understood.
- **Mitigation:** Research phase must include Supabase documentation review, version compatibility matrix, and proof-of-concept recovery tests before claiming supported scope. Clearly separate verified capabilities from aspirational targets.
- **Status:** Open; recovery unverified per FEAT-01 §1.2 and MAP-01 §3.

---

## R-04: Missing Numeric Requirements
- **Category:** Requirements
- **Likelihood:** Medium
- **Impact:** Medium
- **Description:** No numeric thresholds approved for RPO, RTO, package size limits, retention policies, concurrency limits, or SLOs (C-06, U-05, U-06). The specification explicitly states these require owner decisions (FEAT-01 §§8, 10, SUCCESS-01).
- **Consequence:** Designs requiring bounds (e.g., staging expiry, upload limits, archive entry caps) cannot be finalized. Testing cannot validate performance targets. Production readiness cannot be assessed.
- **Mitigation:** Flag each missing numeric requirement as an open question. Request owner decisions before construction of affected components. Use placeholders in designs with explicit "requires approval" annotations.
- **Status:** Open; thresholds pending owner approval per SUCCESS-01 and FEAT-01 §10.

---

## R-05: Authorization Model Incomplete
- **Category:** Security / Design
- **Likelihood:** Medium
- **Impact:** High
- **Description:** While the specification requires server-side authorization (FEAT-01 §§5, 10.1, BR-01, BR-02, US-05), the implementation patterns, role definitions, instance allowlists, and production approval workflows are not designed (A-04, U-07).
- **Consequence:** Security requirements may not be correctly implemented. Production authorization and second-person approval policy decisions are unresolved (U-07).
- **Mitigation:** Include authorization model design in Ideate phase. Require security review before construction. Explicitly resolve production authorization policy (U-07) before enabling production instance selection.
- **Status:** Open; authorization assumed but not designed per A-04.

---

## R-06: Status Contract Mapping Ambiguity
- **Category:** Requirements / Design
- **Likelihood:** High
- **Impact:** High
- **Description:** The public status enum is `queued | running | succeeded | failed | recovery-required` per FEAT-01 §12, but the functional spec and U1 design allow `incomplete` as a distinct terminal state for database-only jobs. The mapping between internal and public contracts is unresolved (U-01).
- **Consequence:** API responses, job state machine, and UI status displays cannot be implemented consistently without this mapping. Prior U1 stopped on this issue.
- **Mitigation:** Resolve mapping before database schema design. Options: (1) add `incomplete` to public enum, (2) map `incomplete` → `failed` with reason code, (3) use separate U1-specific contract. Require explicit approval of chosen option.
- **Status:** Open; blocker per FEAT-01 §12 and U-01.

---

## R-07: Cargo Toolchain Unavailable in Analysis Environment
- **Category:** Tooling / Environment
- **Likelihood:** High (for this environment)
- **Impact:** Low (for analysis); Medium (if persists to construction)
- **Description:** The Rust toolchain is not available in the current analysis environment (C-05, MAP-01 §7), preventing test suite baseline establishment.
- **Consequence:** Cannot verify current test pass/fail status. Unknown whether pre-existing failures exist. Construction environment must have working Rust toolchain or same issue will block development.
- **Mitigation:** For this analysis: document baseline as unknown and proceed. For construction: validate Rust toolchain availability before starting development work. Run `cargo test` to establish baseline on day one.
- **Status:** Known constraint; mitigation planned.

---

## R-08: Cross-Instance Restore Scope Unclear
- **Category:** Requirements
- **Likelihood:** Medium
- **Impact:** Medium
- **Description:** FEAT-01 §8 question 5 asks whether the target instance can differ from the source instance and whether ID/URL/domain/bucket mapping is required. U1 localhost-only scope may sidestep this, but the broader target is unclear (A-05).
- **Consequence:** If cross-instance restore is required, significant design complexity (ID remapping, URL rewriting, bucket name translation, referential integrity) is added. If not required, some user stories may be misinterpreted.
- **Mitigation:** Clarify whether U1 includes separate source/target instances or only same-instance restore. For broader target, request explicit cross-instance requirements before Ideate.
- **Status:** Open; requires owner clarification per U-05.

---

## R-09: Domain Ownership and Architecture Fit
- **Category:** Design / Architecture
- **Likelihood:** Medium
- **Impact:** Medium
- **Description:** Backup/restore is a cross-cutting capability touching database, Auth, and Storage. The current architecture has domain-owned logic and gateway composition root (MAP-01 §6), but backup/restore ownership and layering are not decided (A-06, U-02).
- **Consequence:** Without ownership decision, cannot determine where to place endpoints, state machine, orchestration logic, or migrations. May conflict with existing domain boundaries.
- **Mitigation:** Include architecture/ownership decision in Ideate. Options: (1) new backup domain, (2) gateway-owned with domain adapters, (3) distributed across existing domains. Require architect approval.
- **Status:** Open; requires architectural design per U-02 and A-06.

---

## R-10: Language and Translation Risk
- **Category:** Communication
- **Likelihood:** Low
- **Impact:** Medium
- **Description:** The feature specification is in Vietnamese (FEAT-01), while AI-DLC workflow artifacts are in English (A-08). Potential for translation or interpretation errors when mapping requirements.
- **Consequence:** Requirements misunderstanding could lead to incorrect designs or missing acceptance criteria.
- **Mitigation:** Bilingual review of requirements interpretation if inconsistencies are detected. Trace each design decision back to specific FEAT-01 section references to enable validation.
- **Status:** Low risk; mitigated by explicit evidence sourcing in analysis artifacts.

---

## R-11: Production Restore Not Authorized
- **Category:** Scope / Policy
- **Likelihood:** N/A (constraint, not risk)
- **Impact:** N/A
- **Description:** Production restore is explicitly not authorized by this framing (C-03, PROB-01, SUCCESS-01). This is a known constraint documented for completeness.
- **Consequence:** No production restore design, implementation, or testing is in scope. Release conditions (FEAT-01 §7) are not evaluated because production is out of scope.
- **Mitigation:** N/A; this is the approved boundary. If production restore is later required, a separate approval and design cycle is needed addressing all §7 conditions.
- **Status:** Documented constraint; no mitigation needed.

---

## R-12: Incomplete CLI May Be Abandoned
- **Category:** Technical Debt / Scope
- **Likelihood:** Medium
- **Impact:** Low
- **Description:** The existing `backup_cli.rs` skeleton is incomplete and non-functional (C-04). If the admin UI path is chosen for U1, the CLI may be abandoned or need rework to align with new designs.
- **Consequence:** Effort invested in CLI skeleton may be wasted. Conversely, if CLI completion is the U1 path, UI stories are deprioritized.
- **Mitigation:** Clarify whether CLI, UI, or both are U1 delivery targets (A-03). If UI-only, consider whether to remove or preserve CLI skeleton. If both, plan integration.
- **Status:** Open; requires scope clarification per A-03.

---

## Risk Summary

| Risk ID | Category | Likelihood | Impact | Status |
|---------|----------|------------|--------|--------|
| R-01 | Requirements/Design | High | Critical | Open |
| R-02 | Technical/Safety | High | High | Open |
| R-03 | Technical/Integration | High | Critical | Open |
| R-04 | Requirements | Medium | Medium | Open |
| R-05 | Security/Design | Medium | High | Open |
| R-06 | Requirements/Design | High | High | Open |
| R-07 | Tooling/Environment | High | Low/Medium | Mitigated |
| R-08 | Requirements | Medium | Medium | Open |
| R-09 | Design/Architecture | Medium | Medium | Open |
| R-10 | Communication | Low | Medium | Mitigated |
| R-11 | Scope/Policy | N/A | N/A | Constraint |
| R-12 | Technical Debt | Medium | Low | Open |

**Critical risks requiring resolution before Ideate:** R-01, R-03, R-06
**High-impact risks requiring early attention:** R-02, R-05
