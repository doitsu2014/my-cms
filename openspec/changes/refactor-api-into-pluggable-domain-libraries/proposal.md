## Why

The API is difficult to extend because the current `cms` package, `apps/api/application_core`, and `migration` package are broad shared containers rather than domain boundaries. The observed pain points are: `bin/my-cms-api` carries bootstrap, routing, state, storage, GraphQL, and middleware setup; `application_core`, `migration`, `presentation_models`, `common`, and `api` are dumped into one application surface; and cross-domain imports couple posts to tags/categories and the gateway directly to application-core internals. The current source demonstrates this in `apps/api/src/bin/my-cms-api.rs` (a 331-line bootstrap/router module), `apps/api/src/lib.rs` (a single AppState aggregating all domain concerns), and `apps/api/application_core/src/commands/*`.

## What Changes

- Add a stable, dyn-compatible domain interface crate and shared foundation crate.
- Make the API gateway a thin composition root that creates one shared runtime context, registers domain services, applies cross-cutting middleware, and serves Axum.
- Introduce `domain-post` as a self-contained library containing post HTTP adapters, application commands, DTOs, common domain code, entities required by the post aggregate, GraphQL contribution, and migrations.
- Define ordered migration ownership and orchestration without manually editing generated SeaORM entities.
- Preserve existing REST paths, GraphQL paths, auth roles, error mappings, and external integration behavior.
- Provide a new-domain scaffold/template and documentation.

## Capabilities

### New Capabilities
- `domain-service-interface`: Stable service contract for route, health, configuration, GraphQL, and migration registration.
- `domain-post-service`: Blog Post Service boundary and preserved post/translation behavior.
- `api-gateway-bootstrap`: Pluggable gateway composition and cross-cutting runtime lifecycle.

### Modified Capabilities
- None. This is intended to be behavior-preserving; existing capability requirements remain applicable.

## Impact

Affected source includes `apps/api/Cargo.toml`, `apps/api/src/bin/my-cms-api.rs`, `apps/api/src/lib.rs`, `apps/api/src/api/**`, `apps/api/src/common/**`, `apps/api/src/presentation_models/**`, `apps/api/application_core/src/**`, `apps/api/migration/src/**`, and `apps/api/test_helpers/src/lib.rs`. New workspace members will be added under `apps/api/`. Client-visible routes and schemas must remain compatible. The refactor changes crate imports, entity generation locations, migration ownership, startup composition, build graph, test fixtures, and developer documentation, but introduces no intentional database or HTTP contract change.
