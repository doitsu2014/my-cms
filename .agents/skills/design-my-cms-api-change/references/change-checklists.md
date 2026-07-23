# My-CMS API Change Checklists

Use only the sections touched by the proposed change. Revalidate file paths and
runtime behavior with `$map-my-cms-api-architecture`.

## REST capability slice

- Register the route in the correct public, writer/admin, or administrator
  router in `apps/api/src/bin/my-cms-api.rs`.
- Keep extraction/serialization in `apps/api/src/api/<capability>/`.
- Put validation and business behavior in a trait-backed handler under
  `apps/api/application_core/src/commands/<capability>/`.
- Define camelCase request/response behavior and actor propagation.
- Specify `AppError` and HTTP mappings, including conflict and not-found
  information-disclosure behavior.
- Test the command and any meaningful router/auth contract.

## Database and aggregate change

- Add an ordered migration with a meaningful `down` path or document why data
  rollback is unsafe.
- Define nullability, defaults, unique constraints, foreign keys, cascades,
  indexes, and query-plan implications.
- Keep category/post/tag/translation aggregate mutations transactional.
- For `row_version` updates, define the exact stale-write error contract and
  ensure reads participating in the decision use the intended transaction
  boundary.
- Regenerate SeaORM entities after applying migrations; never edit generated
  entity files manually.
- Update Seaography registration if a new entity should be exposed.
- Cover migration/transaction behavior with PostgreSQL testcontainers.

## Authentication and user administration

- Choose audience and role semantics deliberately; current required-role
  matching is OR-based.
- Extract `SupabaseToken` only behind the matching auth middleware.
- Distinguish 401 authentication failure, 403 authorization failure, and
  obscured 404 behavior for sensitive resources.
- Keep service-role keys server-only, redacted, and absent from errors/traces.
- Preserve self-delete/self-password protections and recognized role rules.
- Test GoTrue status/error mapping with wiremock.

## Media and bucket change

- Define public/private access, administrator bypass, and not-found obscuring.
- Include bucket name, object path, and render dimensions in cache reasoning.
- Define invalidation after create/update/delete/bucket visibility changes.
- Specify MIME allow-list, upload/body size, path normalization, and reserved
  bucket behavior.
- Preserve the media proxy/public URL contract and rendering behavior.
- Test Supabase Storage requests and cache effects with wiremock.

## AI translation and pgvector change

- Preserve or deliberately revise the three-tier order: existing translation,
  similar vector result, OpenAI.
- Define `force_retranslate` semantics, especially when the provider fails.
- Specify model selection, token/chunk limits, cost controls, similarity
  threshold, embedding dimension, and pgvector index compatibility.
- Decide behavior when pgvector is unavailable; the current path degrades to
  direct OpenAI translation.
- Treat `tokio::spawn` as in-process work. If delivery guarantees matter,
  design a durable queue/worker and recovery semantics.
- Define translation job states, progress, duplicate-job behavior, retries,
  cancellation, and sensitive error storage.
- Add deterministic client seams before claiming OpenAI behavior is testable.

## GraphQL change

- Decide whether the entity belongs in immutable, mutable, both, or neither.
- Review generated query/mutation exposure and field-level authorization.
- Define depth, complexity, pagination, and expensive relation behavior.
- Preserve REST/GraphQL consistency for validation, authorization, and errors.
- Verify schema construction and representative operations.

## Cross-cutting runtime change

- Review whether `AppState` should be shared across merged routers.
- Review DB pool, client, schema, and cache lifecycle.
- Define configuration at startup versus per request; fail safely on invalid
  configuration.
- Avoid blocking operations inside async paths.
- Add `#[instrument]` with sensitive/large fields skipped and define useful
  state-change, warning, and failure events.
- Review CORS, body limits, timeouts, rate limits, and graceful shutdown.
- Replace new production `unwrap`/`expect` paths with explicit errors.

## Architecture decision record

For each material decision capture:

| Field | Required content |
|---|---|
| Driver | Requirement, quality attribute, or constraint |
| Current state | Source paths and observed behavior |
| Decision | Concrete selected design |
| Alternatives | Credible options and why rejected |
| Consequences | Positive, negative, and follow-up effects |
| Contracts | API, event, data, auth, or operational behavior |
| Migration | Backfill, compatibility, rollout, rollback |
| Verification | Tests, graph checks, and commands |

## Task traceability

Ensure the final artifacts can be followed as:

```text
proposal outcome
  -> spec requirement/scenario
  -> design decision
  -> migration/API/application-core task
  -> focused test
  -> full verification
```
