---
name: map-my-cms-api-architecture
description: Map the current My-CMS Rust API architecture directly from source, tracing routes, auth boundaries, AppState dependencies, application-core handlers, SeaORM data flows, external integrations, tests, and affected files. Use before API architecture decisions, OpenSpec design, impact analysis, refactors, or when a Software Architect must verify how an existing capability actually works.
---

# Map My-CMS API Architecture

Build an evidence-backed architecture map from the current repository. Treat
source as the current state, `AGENTS.md` as the target engineering policy, and
OpenSpec as the intended behavior. Report differences instead of blending them.

## Load context

1. Read the root `AGENTS.md`.
2. Read [references/api-architecture.md](references/api-architecture.md) for the
   source-derived baseline and its high-risk review points.
3. Check `openspec list --json` and inspect relevant canonical or active specs.
4. Revalidate every material baseline claim against current source; the
   reference is a navigation aid, not a substitute for inspection.
5. If code-review-graph is callable, begin with
   `get_minimal_context(task="<capability or change>")`. Inspect communities,
   callers, callees, imports, flows, and tests. If unavailable, record that and
   use `rg`, targeted file reads, and current tests.

## Trace a capability

Follow the real execution path in this order:

1. Locate route registration in `apps/api/src/bin/my-cms-api.rs`.
2. Record router class, HTTP contract, middleware, auth audience, and roles.
3. Trace the Axum API handler under `apps/api/src/api/`.
4. Trace the application-core handler trait and implementation under
   `apps/api/application_core/src/commands/`.
5. Trace request/response DTO conversion, validation, actor propagation, and
   `AppError` handling.
6. Trace every database entity, relation, transaction, migration, cache, task,
   GraphQL schema, or external service involved.
7. Locate unit, testcontainer, wiremock, auth middleware, and ignored
   integration tests that cover the path.
8. Search for other callers and consumers before declaring the impact radius.

For cross-cutting work, inspect `AppState`, router construction, auth
middleware, response mapping, tracing, CORS/body limits, configuration, and
test helpers even when the initial request names only one handler.

## Evidence rules

- Label statements as **Observed**, **Specified**, **Policy**, or **Inference**.
- Cite repository paths and symbols for every material observed claim.
- Do not treat comments, generated entities, or an OpenSpec proposal as proof
  of runtime behavior without checking the implementation.
- Do not edit generated files under
  `apps/api/application_core/src/entities/`.
- Call out stale comments or policy/source drift without silently correcting it.
- Distinguish in-process state from durable state and request-time calls from
  background work.

## Return this architecture map

Use the smallest format that preserves these fields:

- capability and user/system entry points;
- route, method, middleware, audience, and roles;
- call flow from API to application core to persistence/integration;
- state and configuration dependencies;
- data entities, relations, transactions, indexes, and migration ownership;
- REST/GraphQL/external contracts and error mapping;
- concurrency, idempotency, caching, background-work, and consistency behavior;
- observability and security boundaries;
- existing tests and verification commands;
- affected callers/consumers and likely change surface;
- policy/spec/source mismatches, risks, assumptions, and unknowns.

End with a concise handoff: evidence inspected, key findings, affected paths,
test gaps, architecture questions, and the next recommended owner or skill.
Do not implement code while using this mapping skill.
