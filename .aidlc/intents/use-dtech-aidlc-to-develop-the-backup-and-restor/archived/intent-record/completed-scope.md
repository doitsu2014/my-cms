# Completed scope

## Stages (from `state.json`)

| Phase | Stage | Status | Gate | Notes |
|---|---|---|---|---|
| Analyze | problem-framing | complete | | Problem statement, success measures, stakeholders, acceptance criteria (AC-001..AC-006), user stories (US-01..US-06) |
| Analyze | context-and-risk | complete | `analysis-baseline` approved (AI Orchestrator, 2026-11-21) | Constraints, evidence log, risk register, open questions, quality requirements |
| Ideate | option-generation | complete | | Options A/B/C and trade-off matrix |
| Ideate | direction-selection | complete | `solution-direction` approved (AI Orchestrator, 2026-11-21) | Option A: admin UI first, dedicated backup domain, public `incomplete` status; U1 scope boundary; threat model T-01..T-14 |
| Develop | implementation-design | complete | | Implementation plan, interfaces and contracts (IF-01..IF-11), functional design, test strategy, work units (WU-01..WU-19, WU-16 split into WU-17..19), ID registry |
| Develop | red-tests | complete | | Per unit; WU-01..WU-06 and WU-11 recorded as `skipped` at the red step in earlier sessions (reasons in their reports); the other 11 units recorded behavioural red evidence |
| Develop | build-and-verify | complete | `release-candidate` approved (AI Orchestrator, 2026-11-28) | 18/18 units satisfied; final sweep `unit: sweep`, no failures |
| Launch | (all) | outside scope | | `solo` scope skips Launch |
| Curate | (all) | outside scope | | `solo` scope skips Curate |
| Archived | intent-record | this record | | |
| Archived | knowledge-writeback | pending | `intent-closure` | Next stage |

## Work units

| Unit | Title | Status |
|---|---|---|
| WU-01 | Config, status enum, state machine, safe errors | done |
| WU-02 | Migrations (own migration table), schema guards | done |
| WU-03 | Job store and audit sink | done |
| WU-04 | Allocation ledger, fixture storage, namespace lock | done (TS-31 partial on Windows) |
| WU-05 | Worker, fixture capture, terminal rules, deadline | done |
| WU-06 | Allocation reconciliation and retention store contracts | done |
| WU-07 | Idempotency, atomic acceptance | done |
| WU-17 | Lifecycle reconciler | done |
| WU-18 | Lifecycle cleaner | done |
| WU-19 | Lifecycle sweeper and runtime | done |
| WU-08 | Authorization, REST read/create, status, error projection | done |
| WU-09 | Download denial, disabled capabilities, previews | done |
| WU-10 | Gateway integration, ingress layers, runtime and worker start | done |
| WU-11 | CLI fail-closed | done |
| WU-12 | Web REST client | done |
| WU-13 | Web page, route, menu, instance selector, create panel | done |
| WU-14 | Web status, polling, preview, banner | done |
| WU-15 | Verification sweep, bundle canary, docs, evidence | done |

## Owner decisions recorded

- 2026-11-26: start WU-17..WU-19 after WU-07 (recorded by AI Orchestrator; confirmed by Duc Tran 2026-11-28).
- 2026-11-27: revisit WU-08 within its own scope (recorded by AI Orchestrator; confirmed by Duc Tran 2026-11-28).
- 2026-11-28 (Duc Tran): CFG-17 `BACKUP_REQUEST_TIMEOUT_SECS` = 30; CFG-18 `BACKUP_MAX_HEADER_BYTES` = 16384; RK-15 descoped as residual; RK-16 keep U1 off `main`.

## Explicitly not done

- Not authorized in U1: real data staging, upload or preflight acceptance, restore execution, download, production use, `succeeded` / `recovery-required` outcomes.
- Residuals and follow-ups: release/merge decision (RK-16); E2E-ENV-01 browser verification; RK-15 gateway boot beside posts/seo; CI coverage of Docker and web tests (RK-09); TS-31 on Windows; manual accessibility review (TS-37); Docker test-harness reliability at full parallelism; open owner questions Q-04, Q-06, Q-07, Q-10, Q-11, Q-12, audit retention, non-owner technical guards, performance targets P-01..P-05.
