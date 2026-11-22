# Solution-direction gate recommendation

**Gate:** solution-direction (Ideate phase, final approval for Develop entry)  
**Recommendation:** APPROVE Option A — Admin UI first, dedicated backup domain, explicit public `incomplete` status  
**Approved by:** AI Orchestrator  
**Analysis date:** 2026-11-21  
**Evidence:** solution-options.md, trade-off-matrix.md, decision-record.md (ADRs 001–005), threat-model.md, scope-boundary.md, and prior analysis phase artifacts

---

## Executive summary

Option A is the soundest direction for U1 because it follows established architectural patterns, aligns with documented admin-first requirements, and provides a reusable foundation for future real-data capabilities. The three critical design blockers (Q-01/Q-02/Q-03) are resolved **in principle** through proposed ADRs, but require explicit written owner/architect approvals before construction can proceed. Finite configuration values (Q-05) and threat-model security review are additional hard blockers. This approval authorizes entry to Develop planning, not construction start.

---

## Options analyzed

### Option A: Admin UI first, backup domain, explicit `incomplete` status

**Architecture:** New `domain_backup/` crate owns job persistence, migrations, state machine, lifecycle ledger and audit integration. Gateway composes through existing domain contracts. Admin UI and API deliver allowed-instance selection, synthetic job creation/monitoring, protected diagnostics and labeled previews. Real package operations rejected before side effects.

**Strengths:**
- Strongest architectural fit: follows MAP-01 §6 domain ownership and gateway composition patterns instead of creating exceptions
- Direct alignment with documented admin workflow (FEAT-01 §§1–4, US-01/02/05/06)
- Truthful semantics: `incomplete` status needs no translation; no confusion with operational failure
- Reusable U2 foundation: same domain/API can accept real database, Auth, Storage adapters without rework
- Clear safety ownership: one domain owns durable lifecycle policy and reconciliation
- Documented in comprehensive ADRs (001–005) with traced dependencies and trade-offs

**Costs:**
- Medium relative effort: new crate, migrations, authorized API and UI, plus lifecycle fault tests
- Public enum expansion requires consumer inventory and coordinated rollout
- Synthetic data validates orchestration, not capture fidelity or recovery

**Residual risks:**
- ADR-001/Q-08/Q-09: Admin-first scope and configured-local-only assumption require owner confirmation
- ADR-002/Q-01: Status contract expansion requires written owner/architect approval; consumer compatibility must be reviewed
- ADR-003/Q-02: Domain ownership, schema integration and migration authority require written architect approval
- ADR-004/Q-03: Durable lifecycle protocol (register-before-write, inventory reconciliation, crash handling) requires written owner/architect/operations approval with supplied finite bounds (Q-05)
- Q-05/C-06: Owner/operations must supply expiry, orphan grace, sweep cadence, retry budget, operation deadlines, byte/concurrency limits, storage protection, and escalation responsibility
- Threat model (T-01 through T-17) requires owner/architect/security review before construction

**Recommendation:** APPROVE for Develop entry with the conditions listed below.

### Option B: Shared domain, UI and CLI, internal `incomplete` → public `failed` + reason

**Architecture:** Same domain as A, but serves both UI and CLI clients using semantic translation: internal `incomplete` maps to public `failed` with a mandatory stable reason (e.g., `DATABASE_ONLY_INCOMPLETE`).

**Strengths:**
- Preserves existing public enum; avoids consumer compatibility rework
- Serves both admin and operator interfaces with one authoritative state machine
- CLI and UI use the same API and lifecycle, no duplicate capture engines

**Why not selected:**
- Dual-client authentication, versioning and parity testing add work
- Semantic translation can make planned incompleteness indistinguishable from operational failure when consumers ignore the reason
- CLI delivery is not established as a U1 co-equal requirement (Q-09 open)
- Adds permanent translation layer and support burden without commensurate Ideate evidence

**Credible only if:** Owner confirms CLI is a co-equal requirement and public-enum compatibility evidence shows expansion is unacceptable. Would require reopening scope/contract decisions with explicit owner approval.

### Option C: CLI-first, gateway-owned pilot, isolated U1 contract

**Architecture:** Gateway-local application service with durable jobs; authenticated CLI create/list/status; isolated versioned U1 contract (`queued|running|incomplete|failed`). Admin UI explicitly deferred.

**Strengths:**
- Fastest initial interface for local operator feedback
- Avoids changing general public enum
- Fewer immediate UI/API concerns

**Why not selected:**
- Violates established feature ownership pattern (MAP-01 §6) unless explicit exception is approved; gateway remains composition root
- Defers admin acceptance validation entirely (US-01/02/05/06 not delivered)
- Creates likely U2 rework: gateway extraction, historical-state migration, journal/database divergence
- Filesystem journal creates additional durability/consistency obligations
- CLI-only evidence does not validate documented admin-first workflow

**Credible only if:** Owner explicitly replaces admin-first delivery with a temporary CLI experiment and accepts architecture exception plus later migration cost. Not preferred for a feature being resurrected after prior stoppage.

---

## Critical path: three proposed ADR resolutions (in principle)

These are **proposed** in the decision record, not approved. They resolve the blockers that stopped prior U1 (FEAT-01 §12, PROB-01), but require explicit written approvals.

| ADR | Question | Proposed in principle | Required approval |
|-----|----------|----------------------|-------------------|
| ADR-002 | Q-01 | Add public terminal `incomplete`; persist and serialize without translation; download always denied; all clients handle `incomplete` | Written owner/architect approval of enum expansion, outcome examples (normal incompleteness vs. failure), and consumer compatibility plan |
| ADR-003 | Q-02 | `domain_backup/` owns jobs, migrations, state machine, lifecycle integration; gateway composes only | Written architect/owner approval of domain ownership, schema boundaries, migration authority and audit adapter integration |
| ADR-004 | Q-03 | Durable database allocation ledger: register-before-write, inventory reconciliation, startup fencing, exclusive process namespace ownership, idempotent cleanup with bounded retry, visible escalation on failure; DB outage stops new work and deletion | Written owner/architect/operations approval of ledger protocol, crash-case matrix, storage protection, finite bounds (expiry, sweep, retry, operation deadline, byte/concurrency limits) and escalation responsibility |

**Status:** Proposed, not approved. Missing any of these three approvals blocks construction resumption.

---

## Additional hard blockers

### Q-05: Finite bounds (owner/operations)
Before constructing lifecycle/allocation/worker components:
- Staging data expiry (e.g., "24 hours")
- Orphan grace period (e.g., "1 hour after expiry")
- Sweep cadence (e.g., "hourly" or "on request")
- Retry budget and backoff (e.g., "3 attempts, 1m backoff")
- Operation deadline (e.g., "10 minutes per job")
- Temporary byte budget (e.g., "1 GB per allocation", "10 GB cluster total")
- Concurrency limit (e.g., "1 concurrent job")
- Metadata/idempotency retention (e.g., "30 days")

**No invented defaults.** Missing values block affected construction.

### Threat model security review (owner/architect/security)
Residual risks T-01 through T-17 require design/construction-phase verification:
- **T-01:** JWT/session composition and test coverage of actual router
- **T-02:** Authorization matrix and revocation semantics (A-04 not yet approved)
- **T-03:** Instance registry, SSRF prevention and localhost boundary enforcement
- **T-04:** Effective loopback binding, reverse-proxy exposure, CSRF controls
- **T-05:** Capability wiring regression tests
- **T-06:** Status/download invariant and client enum handling
- **T-07:** Idempotency/replay durability and finite retention enforcement
- **T-08:** Audit-before-effect durability and transaction boundary
- **T-09:** Error/telemetry leakage canaries and safe projections
- **T-10:** XSS and browser security
- **T-11:** File allocation/deletion path traversal and namespace exclusivity
- **T-12:** Crash/race scenarios and active-work fencing
- **T-13:** Resource exhaustion and concurrency/rate limits
- **T-14:** Fixture storage confidentiality (real encryption/key deferred)
- **T-15–T-17:** Restore substitution, archive validation and migration privilege (deferred with U1 real packages)

### Q-12: Toolchain and baseline (development/QA)
Before construction: establish working Rust/Cargo environment, frontend build, integration-test harness; document any pre-existing failures; verify quality-gate feasibility.

---

## What this approval authorizes

✅ **Approved:**
- Entry to Develop phase for Option A implementation planning
- Use of proposed ADRs 001–005 as the basis for design and construction planning
- Continued work under the scope boundary defined (localhost, synthetic database-only, protected operations, no real staging/download/production)
- Security review and threat-model verification during Develop planning

❌ **NOT approved (and explicitly not authorized):**
- Real-data staging, backup upload, preflight/restore acceptance, or any production-related work
- Closing or superseding Q-01/Q-02/Q-03 without explicit written owner/architect approvals recorded separately
- Inventing numeric defaults for Q-05 bounds
- Beginning construction without threat-model security review and approval
- Marking database-only work `succeeded`, exposing incomplete downloads, or presenting synthetic tests as disaster-recovery proof
- Production restore, cross-instance mapping, or Auth/Storage recovery (Q-04 unverified, Q-07 open)

---

## Evidence and assumptions

**Observed facts:**
- Prior U1 stopped without making application changes due to Q-01/Q-02/Q-03 blockers (FEAT-01 §12, PROB-01, R-01)
- Current CLI skeleton is non-functional (MAP-01 §3, §9)
- Existing architecture uses domain ownership and gateway composition (MAP-01 §6, architecture docs)
- Documented requirements center admin UI workflow (FEAT-01 §§1–4)
- 12 open questions and critical risks identified (Q-01 through Q-12, R-01 through R-12)
- Real-data staging explicitly prohibited until cleanup/reconciliation design approved (C-02)
- Production restore not authorized (C-03, SUCCESS-01)
- Localhost-only, database-only, terminal `incomplete`/`failed` constraints approved (C-01, AC-002)

**Assumptions requiring confirmation:**
- Admin-first delivery is the priority (Q-09, A-03)
- Configured local source/target, no cross-instance mapping (Q-08, A-05)
- Server-side authorization enforced (A-04, requires matrix approval)

**Unresolved and deferred:**
- Supported Supabase versions, hosting models, Auth/Storage recovery (Q-04, R-03)
- Package limits, retention policy, storage location and protection (Q-05, C-06, R-04)
- RPO/RTO and restore drill requirements (Q-06, R-04)
- Production authorization and second-person approval (Q-07, Q-11 encryption/keys)
- Cross-instance mapping (Q-08, deferred to U2)
- Consistency guarantees (Q-10, deferred)
- Current test baseline (C-05, Q-12 unverified)

**Verification completed:**
- Document/source review and traceability analysis
- Option comparison with trade-offs and rejection reasons
- Architectural fit assessment against MAP-01 §6 and existing patterns
- Threat model STRIDE analysis with 17 identified threats and mitigation requirements
- Scope boundary definition with acceptance accounting and construction blockers
- ADR proposals with dependencies and approval requirements

**Not verified:**
- Rust execution environment readiness
- Current test suite baseline or pre-existing failures
- Capture fidelity, archive safety or recovery capability
- Actual implementation of any proposed code or components
- Exact consumer impact of enum expansion or enum handling in existing clients

---

## Residual risks and ongoing ownership

**R-01, R-06, R-09 (critical path blockers):** Not closed; ADR approvals required.

**R-02, R-04, R-05 (high-impact):** Partially mitigated by proposed ADRs; Q-05 values and threat-model review required.

**R-03 (Supabase recovery unverified):** Deferred to Q-04 research; no recovery claim made in U1.

**R-07 (toolchain unavailable):** Mitigation planned for Develop phase; Q-12 verification required.

**R-10, R-12 (medium):** Addressed by Option A (UI priority, scope clarity); A-03/Q-09/Q-12 confirmation still needed.

**R-11 (production not authorized):** Documented constraint; no mitigation needed for U1.

---

## Gate outcome

**✅ APPROVE Option A to enter Develop phase.**

This recommendation approves the proposed direction and authorizes implementation planning within the Develop phase. It does NOT close the three critical questions, supply missing finite bounds, or authorize real-data staging or production work.

**Before construction can resume, the following must be explicitly recorded:**
1. Written approvals of ADR-002, ADR-003, ADR-004 (Q-01/Q-02/Q-03) by owner, architect, and operations
2. Supplied finite values for Q-05 (expiry, orphan grace, sweep, retry, deadlines, limits, storage protection)
3. Security review and approval of threat model by architect and security reviewer
4. Owner confirmation of admin-first/configured-local scope (Q-08/Q-09) and authorization matrix (A-04)
5. Toolchain readiness verification (Q-12) in the development environment

Develop planning should:
- Create detailed implementation specs from the proposed ADRs
- Trace each spec to its ADR and approval status
- Flag every missing blocker approval and finite value as a hard gate
- Design construction phases such that real-data capability remains disabled until separate safety approval
- Plan security/threat-model verification tests
- Plan toolchain validation before construction start

---

**Prepared by:** AI Orchestrator  
**Date:** 2026-11-21  
**Authority:** Ideate phase gate decision (orchestrationMode: "ai"; conductor owns gate reporting)
