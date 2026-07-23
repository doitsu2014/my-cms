---
name: design-my-cms-api-change
description: Design or review implementation-ready changes to the My-CMS Rust API using the actual Axum, application-core command-handler, SeaORM, Supabase, media, GraphQL, pgvector, OpenAI, tracing, and testing patterns. Use after product scope is clear to create or revise OpenSpec specs, design.md, tasks.md, API/data contracts, migrations, rollout plans, or architecture reviews.
---

# Design My-CMS API Change

Produce a source-aligned, testable design for the API. Resolve architecture
before task decomposition; do not implement application code.

## Required context

1. Read root `AGENTS.md` and load the project `openspec` skill.
2. Read the selected change's proposal, existing artifacts, relevant canonical
   specs, and Product Designer brief when UI behavior is involved.
3. Use `$map-my-cms-api-architecture` to trace the current capability and
   affected flows from source.
4. Read [references/change-checklists.md](references/change-checklists.md) and
   apply only the subsystem sections relevant to the change.
5. Run `openspec status --change "<change>" --json` and retrieve instructions
   for each artifact before editing.

## Mandatory evidence gate

Call `get_minimal_context(task="<change>")` when code-review-graph is
available. Inspect callers, callees, imports, communities, flows, and tests.
If unavailable, record the limitation and use the mapping skill's source-first
fallback. Never fabricate graph or runtime evidence.

## Design workflow

### 1. Frame the architecture drivers

Extract required behavior, quality attributes, constraints, assumptions,
non-goals, compatibility expectations, and measurable acceptance outcomes.
Return unresolved product choices to the Product Owner.

### 2. Define observable contracts

Specify:

- route/method or GraphQL operation and auth boundary;
- request, response, status/error, pagination, and compatibility behavior;
- validation, permission, empty, conflict, retry, and failure scenarios;
- idempotency and concurrency semantics;
- external-service timeout, retry, degradation, and secret-handling behavior.

Keep normative behavior in specs and implementation choices in `design.md`.

### 3. Trace the vertical slice

Map each requirement through the affected Axum adapter, application-core
handler, transaction/query, entity/migration, cache/background work, external
client, telemetry, and tests. Preserve the dependency direction and command
handler boundary.

### 4. Make explicit decisions

For every material decision:

- state the decision and architecture driver;
- compare credible alternatives;
- explain tradeoffs and consequences;
- identify migration, rollout, rollback, and operational effects;
- identify security, privacy, performance, and data-consistency risks.

Do not copy an existing pattern blindly when the source-derived architecture
baseline identifies it as a review risk.

### 5. Design data evolution

Use schema-first migrations. Define constraints, indexes, foreign keys,
backfill, locking/downtime risk, compatibility window, rollback/data-loss
behavior, and entity regeneration. Never plan manual edits to generated
SeaORM entities.

### 6. Design verification

Map every requirement and risk to a test level:

- pure validation/unit test;
- command-handler test with PostgreSQL testcontainer;
- external contract test with wiremock;
- auth/router test;
- pgvector integration test;
- ignored live-service test only when unavoidable;
- full repository verification gate.

Prefer deterministic seams over tests that need live Supabase or OpenAI.

### 7. Produce executable tasks

Create dependency-aware vertical slices. Each task names the requirement,
affected layer/files, test-first behavior, implementation outcome, graph/impact
review, and targeted verification. Keep one writer per artifact and one
implementation owner per task group.

## Artifact quality gate

Before handoff, confirm:

- each proposal outcome has testable scenarios;
- each scenario maps to a design decision and task;
- route/auth/error/data contracts are explicit;
- concurrency, caching, background work, external failures, and observability
  are addressed where relevant;
- migrations are deployable and reversible to the stated degree;
- source-policy mismatches are resolved or recorded;
- `openspec status` reports all apply-required artifacts complete.

## Handoff

Return the change name, source and graph evidence, contracts, key decisions and
alternatives, affected layers/flows, migration and rollout plan, security and
operational risks, test strategy, artifact paths changed, OpenSpec readiness,
open questions, and next owner. Hand off to the Software Engineer only when
implementation can proceed without inventing architecture or product behavior.
