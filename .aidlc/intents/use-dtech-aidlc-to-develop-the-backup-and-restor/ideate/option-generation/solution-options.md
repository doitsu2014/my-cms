# U1 solution options

## Decision boundary and evidence

These are conditional proposals, not approved blocker resolutions. Select a direction now; record and approve Q-01/Q-02/Q-03 decisions during Ideate before construction. Prior U1 made no application changes because these same decisions were unresolved (FEAT-01 §12; R-01, R-06, R-09). No option bypasses that outcome.

**Observed baseline:** FEAT-01 §§9–12 and `.aidlc/knowledge/_codebase-map.md` §§3, 6 establish an incomplete CLI, gateway composition, domain-owned logic/migrations, and the public/internal status mismatch. Analysis `constraints-and-assumptions.md`, `open-questions.md`, `risk-register.md`, and `acceptance-criteria.md` supply the IDs below. Proposed paths are placement choices, not existing implementations.

**All options:** localhost-only, database-only; terminal job outcome `incomplete` or `failed`, never `succeeded`; no incomplete downloads (C-01; AC-002). Real upload/preflight/restore and real-data staging stay disabled throughout U1 (C-02; R-02). Use synthetic database fixtures and injected adapters; a workflow demonstration is not proof of actual database capture or recovery. No real dump may be written as an incidental implementation shortcut. Restore previews never mutate a target. Auth/Storage recovery, complete packages, S3 delivery, production restore, and cross-instance mapping are deferred (C-03/C-04; R-03; Q-04/Q-08). Q-08's same-instance interpretation remains an assumption.

Server-side administrator and resource authorization, server-configured local instance IDs, safe audit before sensitive effects, durable job acceptance, idempotency, and guarded state transitions apply to every delivered interface (A-04; AC-001/005; FEAT-01 §10). A localhost binding or CLI invocation alone is not authorization.

## Common Q-03 proposal: durable database ledger

Options A and B assume approval of this design, not merely approval to defer cleanup:

- Backup-owned persistence records each allocation intent, opaque storage identifier, job association, expiry, cleanup state, retry count and next attempt **before** any future storage allocation. A private storage namespace is derived from that durable identifier; reconcile allocations interrupted between ledger commit and file creation.
- Startup reconciliation and a recurring sweep find expired, interrupted and unlinked allocations, including storage entries without a live job; compare ledger and namespace inventory rather than relying only on process memory. Prevent deletion of active allocations using explicitly designed ownership/state guards.
- Cleanup is idempotent; interrupted deletion is retried. Bounded retries end in a visible failed-cleanup record and operational escalation, never silent deletion of the evidence. No claim of automatic job resume or exactly-once execution.
- Ideate must approve storage protection, schema/transaction boundaries, crash cases, sweep cadence, finite expiry, retry budget and escalation owner. Values are unresolved Q-05/C-06 inputs; no invented defaults (R-04). The approval must define handling when the metadata database itself is unavailable.
- U1 validates the lifecycle with synthetic private files and restart/fault tests. Real staging remains disabled even if those tests pass. Later enablement requires reviewed implementation, real-data safety evidence and separate approval (C-02).

This is a candidate resolution of Q-03, not a claim that cleanup has been implemented or verified. Missing mandatory bounds prevent construction of the affected component; selecting an option does not close Q-03.

## A — Admin UI first, backup domain, explicit incomplete status

**Description and order:** Approve contracts and ownership, build domain persistence/state machine with synthetic adapter tests, mount authorized API handlers, then add admin UI instance selection, create/monitor and diagnostics. Add clearly labeled synthetic upload/preflight/confirmation previews; real routes reject before accepting package bytes or invoking extraction.

**Scope:** U1 covers observable job workflow and protected operations through the UI (US-01/02/05/06). US-03/04 receive previews/negative-path checks only, not real-package acceptance. CLI completion, real capture and all full-recovery capabilities are deferred.

**Critical blocker resolution (proposed):** Q-01 adds terminal `incomplete` to the public enum and aligns persisted status, serializers and UI; unavailable required components can yield `incomplete`, actual execution failure yields `failed` (R-06). Q-02 creates a backup domain owning jobs, audit integration, migrations and state transitions. Q-03 adopts the common durable ledger design with approved bounds and fault-test criteria. All three require explicit Ideate decisions (R-01).

**Implementation path:** Proposed `apps/api/domain_backup/`, added to `apps/api/Cargo.toml` and wired by `apps/api/gateway/src/main.rs`; domain-owned entities/migrations/application service and adapters. Gateway mounts HTTP endpoints for allowed instances, job submission and authorized job reads; it contains no backup state machine. Admin components live in `apps/web/` using existing routing/API patterns. Exact route names await contract design.

**Staging approach:** Synthetic adapters/files only; fail-closed real package endpoints and download denial. Cleanup tests exercise restart behavior without sensitive content.

**CLI vs UI:** Admin UI/API completed only for the U1 slice; preserve existing CLI refusal behavior without claiming it works (C-04; R-12).

**Benefits:** Best match to UI stories and domain ownership (C-07; A-03/A-06). Truthful public outcome requires no semantic translation. Clear U2 home for future recovery adapters.

**Costs:** Medium relative effort: new crate/migrations, API/UI work and enum compatibility review. Public status expansion requires exhaustive client handling and documentation updates.

**Residual risks:** Consumers may reject an unknown enum variant; synthetic tests cannot prove archive or Supabase safety (R-03). Rust execution readiness remains unverified (C-05; R-07; Q-12).

**Main reason to reject:** Reject if public-contract consumers cannot accept `incomplete`, or owner prioritizes CLI delivery over the UI.

## B — Shared backup domain, API plus CLI, compatible failed projection

**Description and order:** Approve mapping and domain design; implement shared persistence/state machine and authorized API first, then UI and CLI clients against that API. CLI calls the service, not a subprocess behind UI; no duplicate capture engine.

**Scope:** Same U1 UI coverage as A plus CLI create/list/status for synthetic jobs. CLI restore remains a safe refusal. No S3 adapter completion or real recovery is implied (C-04).

**Critical blocker resolution (proposed):** Q-01 preserves the existing public enum: durable internal `incomplete` projects to public `failed` plus a mandatory stable reason such as `DATABASE_ONLY_INCOMPLETE` (proposed code), with explicit non-downloadable capability. Internal `failed` uses distinct failure reasons. The mapping must preserve the internal outcome in audit and diagnostics and be approved across API/UI/CLI; never map either to `succeeded` (C-01; R-06). Q-02 selects the same new backup domain as A. Q-03 adopts the common ledger design and finite-bound approvals.

**Implementation path:** Proposed `apps/api/domain_backup/` owns orchestration, migrations and canonical state. Gateway mounts the same API surface as A; `apps/api/gateway/src/backup_cli.rs` becomes an authenticated API client for the U1 subset. UI lives in `apps/web/`. Both interfaces use server-side authorization; credentials and arbitrary target connection strings are not CLI request parameters.

**Staging approach:** Synthetic adapters/files only. Neither interface can submit real packages; shared server checks prevent a CLI bypass.

**CLI vs UI:** Both deliver the limited synthetic backup workflow; neither completes real upload, preflight or restore.

**Benefits:** Avoids expanding the public status enum while preserving truthful failure reasons. Serves operators and admins with one authoritative state machine; better U2 reuse if both interfaces are genuinely needed (Q-09; R-12).

**Costs:** Slowest of these options: dual client implementation, authentication UX and contract-parity tests. Failed projection introduces permanent semantic translation and support burden.

**Residual risks:** Consumers may ignore the reason and conflate planned incompleteness with operational failure; monitoring counts need documented semantics. CLI/API version skew and credential handling add tests (R-05/R-06). Same synthetic-evidence and tooling limits as A.

**Main reason to reject:** Reject if CLI value does not justify extra delivery time, or `failed` would materially mislead operators even with a reason code.

## C — CLI first, gateway-owned pilot, isolated U1 contract

**Description and order:** Approve an explicit architecture exception and CLI-first delivery; implement a gateway-local application service with durable jobs and fixture adapters, then authenticated CLI create/list/status and diagnostics. Keep HTTP package operations absent or fail-closed. Treat this as a disposable local workflow pilot.

**Scope:** U1 validates CLI orchestration and negative security paths. Admin UI acceptance scenarios are explicitly deferred, not declared satisfied (A-03; Q-09). No real restore, upload, capture output or download.

**Critical blocker resolution (proposed):** Q-01 uses a separate versioned U1 contract with `queued`, `running`, `incomplete`, `failed`; public general-contract endpoints are not claimed implemented. A future migration plan must preserve historical outcomes. Q-02 assigns job schema, migrations and state machine to gateway-owned modules, requiring an approved exception to domain ownership (R-09). Q-03 instead proposes a durable private filesystem allocation journal with atomic publication, expiry and cleanup records, restart inventory reconciliation and bounded deletion retries. It must cover journal/file write-order crash windows, corrupt journals and missing job records; gateway owns both journal recovery and job linkage. Finite bounds and storage protection still require approval before construction (Q-05; C-06).

**Implementation path:** `apps/api/gateway/src/backup_cli.rs` delegates to proposed gateway-local service/persistence modules. Gateway migration wiring owns the new job schema; no existing domain is assigned backup data implicitly. CLI verifies actor identity/administrator/resource access in that service before reading config/jobs; OS access alone grants no authority. No admin UI endpoints are completed in U1.

**Staging approach:** Synthetic private files only; fake storage still exercises journal restart tests. A journal is not permission for real staging (C-02).

**CLI vs UI:** CLI-only limited workflow; UI and shared API are later work.

**Benefits:** Fastest interface slice; direct feedback from local operators; avoids changing the general public enum. Shows a credible cheaper alternative rather than assuming UI is mandatory for the first pilot.

**Costs:** Medium total effort despite fast CLI surface: durable journal engineering remains substantial. Gateway ownership conflicts with existing patterns; later domain/API extraction and status migration are likely.

**Residual risks:** Two persistence mechanisms can diverge; filesystem semantics require platform-specific fault tests. CLI-first may not validate admin usability at all; historical statuses require conversion (R-06/R-09/R-12).

**Main reason to reject:** Weakest long-term fit: sacrifices UI story delivery and creates ownership/contract rework. Reject unless owner explicitly values a temporary CLI pilot over a reusable admin feature.

## Recommendation boundary

Prefer A for the documented admin workflow and architectural fit. B is a serious alternative when public enum compatibility and CLI demand outweigh added effort. C is intentionally weaker, but defensible for a deliberately temporary operator pilot. Real staging with cleanup placeholders is rejected as infeasible under C-02, not presented as a fourth selectable option. None is construction-ready until the three blocker decisions and required finite bounds are recorded and approved; none verifies Supabase recovery or authorizes production.
