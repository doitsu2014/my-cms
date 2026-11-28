# U1 trade-off matrix

## Decision frame

Compare conditional designs, not approved implementation plans. All options remain localhost-only/database-only, terminal `incomplete` or `failed`, never `succeeded`, with no incomplete downloads (C-01; AC-002). Real-data staging and real upload/preflight/restore remain disabled in U1 (C-02; R-02). Synthetic fixtures demonstrate orchestration only. No option authorizes production restore or claims Auth/Storage recovery (C-03; R-03; Q-04).

Q-01/Q-02/Q-03 must each be explicitly resolved in Ideate before construction. A deferral of real staging does **not** close Q-03. Prior U1 stopped on these same blockers without application changes (FEAT-01 §12; R-01/R-06/R-09).

## Comparison

| Dimension | A — UI + backup domain | B — UI/CLI + shared domain | C — Gateway CLI pilot |
| --- | --- | --- | --- |
| Q-01 assumed resolution | Add terminal `incomplete` to public enum; persistence/API/UI use aligned semantics. | Keep public enum; internal `incomplete` maps to public `failed` with mandatory distinct reason; preserve internal outcome in audit/diagnostics. | Isolated, versioned U1 enum includes `incomplete`; general public contract remains unimplemented for this pilot. |
| Q-02 assumed resolution | New backup domain owns jobs, migrations, state machine and lifecycle policy; gateway composes. | Same new domain; API is authoritative; CLI/UI share the server-side service. | Explicit exception: gateway owns jobs, migrations, orchestration and cleanup journal. |
| Q-03 assumed resolution | Durable DB allocation ledger before future writes; startup/recurring inventory reconciliation; idempotent bounded cleanup and visible failures. | Same ledger; shared server policy covers both clients. | Durable private filesystem allocation journal; atomic publication and restart inventory reconciliation; explicit linkage to job DB and corrupt-journal handling. |
| Q-03 approval conditions, all mandatory | Approve finite expiry, sweep/retry bounds, storage protection, ownership guards, crash cases and metadata-DB outage behavior. | Same as A; interface parity adds checks against CLI bypass. | Approve finite bounds/protection plus journal durability, publication crash windows and DB/journal divergence handling. |
| Implementation timeline, relative | **Medium** after decisions; domain/API/UI plus fixture lifecycle tests. | **Slow**; A's core plus authenticated CLI and cross-interface tests. | **Fast interface / medium overall**; less UI work but durable journal is not a shortcut. |
| Architectural fit | Strongest: follows domain-owned logic/migrations and gateway composition (MAP-01 §6; C-07; A-06). | Strong: same layering; two clients, one state machine. | Weak: requires explicit architecture exception; extraction likely (R-09). |
| Staging capability | Synthetic adapters/private files only; real package routes reject before body persistence or extraction. | Same; centralized checks deny real packages through either client. | Synthetic files/journal tests only; no real package entry point. |
| UI/CLI delivery | UI/API U1 workflow and clearly labeled previews; CLI remains fail-closed. | UI/API plus CLI create/list/status for synthetic jobs; restore refusal stays. | CLI-only pilot; UI acceptance scenarios explicitly deferred (Q-09). |
| Production ready in U1? | **No.** Establishes a clean U2 domain/API/UI foundation; does not unblock real-data enablement by itself. | **No.** Same U2 foundation plus reusable operator client; does not establish recovery support. | **No.** Tests local orchestration; U2 probably requires domain/API extraction and contract migration. |
| Rework risk if blockers resolve differently | Medium: public enum compatibility or ownership reversal affects schema/client work. | Medium–high: changing failed projection affects both clients; changing ownership affects shared core. | High: separate contract, gateway exception and dual persistence linkage all invite migration. |
| Primary advantage | Best alignment with admin stories, truthful status and existing ownership patterns. | Public enum compatibility and two useful interfaces without two orchestration engines. | Smaller initial interface surface and fast operator feedback. |
| Primary disadvantage | Public enum expansion can break exhaustive consumers; new crate and UI cost. | Most implementation work; `failed` reason can be ignored or misinterpreted. | Least direct UI value and greatest architecture/contract debt. |
| Main rejection trigger | Owner cannot accept enum expansion or does not prioritize UI. | CLI demand or compatibility benefit does not justify added work/semantic translation. | Owner wants a durable admin feature rather than an explicitly temporary CLI pilot. |

Timeline is qualitative scope comparison, not a delivery commitment. C-05/R-07/Q-12 leave Rust test execution readiness and the current baseline unverified. FEAT-01 §12 records historical failures, which are not a current test result. No benchmark, throughput, RPO/RTO, expiry or retention value is asserted (C-06; R-04; Q-05/Q-06).

## What must be decided, not rediscovered

| Decision | Information needed for approval | Consequence of leaving unresolved |
| --- | --- | --- |
| Q-01 status | Choose public enum expansion, failed-with-reason projection, or isolated U1 contract; approve examples for normal incompleteness vs actual failure and non-downloadability. | Cannot finalize state/schema/serializers or status display (R-06). |
| Q-02 ownership | Choose domain vs gateway exception; name migration authority, repository owner, state-machine owner and audit/cleanup integration boundary. | Cannot place persistence or migrations safely (R-09). |
| Q-03 lifecycle | Approve ledger vs journal design, pre-write registration, orphan inventory, active-work safety, restart/deletion failure handling and fault-test exit criteria. | Same prior-U1 blocker persists (R-01/R-02). |
| Q-05 finite bounds | Owner/operations approve finite expiry, sweep/retry/operation budgets and applicable byte/concurrency limits; specify storage protection. Missing mandatory values cause refusal, never unbounded defaults. | Affected construction remains blocked. Values cannot be invented by selecting an option (C-06/R-04). |
| Q-09 delivery | Choose admin-first, both clients, or explicit CLI-first deferral of UI stories. | Interface effort and acceptance expectations remain ambiguous (A-03/R-12). |
| A-04 protection | Approve server-side actor/administrator/resource checks, local allowlist and safe audit-before-effect behavior for every delivered path. | No interface is safe to implement merely because it runs on localhost (R-05; AC-005). |

All needed direction choices are exposed here; no additional discovery is required to compare them. Approval of actual numeric values and detailed lifecycle safety decisions is still required before dependent construction. Proposed mechanisms are not observed capabilities or approved answers.

## Recommendation and acceptance limits

**Prefer A** when the objective remains the documented admin workflow (FEAT-01 §§1–4). It follows existing ownership conventions and communicates U1's incompleteness directly. **Choose B instead** when retaining the public enum and delivering an operator CLI are both important enough to fund extra client work. **Choose C only** for an explicitly temporary CLI-first experiment that accepts UI deferral and future extraction.

A fourth idea, real staging with placeholder cleanup, is **infeasible** under C-02 and therefore not selectable. Designing cleanup in Ideate, or passing synthetic crash tests later, does not itself authorize handling real packages.

For any choice, U1 evidence must include truthful terminal outcomes, durable acceptance failure behavior, unauthorized/resource-ID denial, safe audit failure refusal, download rejection, disabled real staging, no target mutation and synthetic cleanup/restart failure tests (AC-001–006; FEAT-01 §10). No current test run is claimed. U2 still needs separately approved real-data safety verification and supported Supabase recovery scope; production additionally requires FEAT-01 §7 readiness conditions, including Q-06/Q-07/Q-10/Q-11 decisions.

## Evidence references

- `docs/features/backup-restore.md` §§9–13: existing skeleton, U1 limits, prior stoppage and architecture context.
- `.aidlc/knowledge/_codebase-map.md` §§3, 6–7, 9–10: observed entry points, ownership conventions and evidence gaps.
- Analysis baseline `analyze/context-and-risk/constraints-and-assumptions.md`, `open-questions.md`, `risk-register.md`: C/A/Q/R identifiers.
- Analysis baseline `analyze/problem-framing/acceptance-criteria.md`: AC-001–006 and U1-specific negative/safety criteria.
- `solution-options.md` alongside this file: detailed option sequencing, proposed code placement and residual risks.
