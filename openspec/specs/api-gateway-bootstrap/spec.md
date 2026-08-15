# api-gateway-bootstrap Specification

## Purpose
TBD - created by archiving change refactor-api-into-pluggable-domain-libraries. Update Purpose after archive.
## Requirements
### Requirement: Gateway is a thin domain composition root
The `my-cms-api` binary SHALL own environment loading, one database connection, shared context construction, domain registration, cross-cutting auth/CORS/body-limit/tracing/cookie layers, listener lifecycle, and graceful startup errors. It SHALL NOT import domain commands, entities, DTOs, storage clients, or business rules. It SHALL only depend on the `domain_interface` contract crate and on the `DomainService` trait objects provided by each domain crate it composes.

#### Scenario: Startup succeeds with one or more domains
- **WHEN** required configuration is valid and N>=1 domain services are registered
- **THEN** the gateway builds one router and starts the listener
- **AND** readiness/health reports the registered services

### Requirement: Domains compose without gateway route edits
The gateway SHALL register domains by iterating the `DomainService` collection. Adding a conforming domain library SHALL require registration/configuration in the composition manifest only, not new gateway route or handler logic. Each domain SHALL be a self-contained Cargo crate (lib + bin or lib-only) that owns its routes, layers, entities, migrations, and tests.

#### Scenario: Two domains register independently
- **WHEN** domain-posts and a test domain are registered
- **THEN** both route sets are reachable under their declared prefixes
- **AND** neither domain imports the other's internals

### Requirement: Cross-cutting authorization and middleware are applied in every deployment mode
Auth, CORS, cookie, body-limit, and OpenTelemetry middleware SHALL be applied identically in two modes: (a) gateway-composed mode — the gateway applies the layers to the merged router; (b) standalone domain mode — the domain's own `bin` applies the same layers to its own router. In both modes, the middleware SHALL preserve Supabase JWT validation, token request extensions, writer/admin OR semantics, administrator-only routes, CORS behavior, tracing/OpenTelemetry layers, body limits, and cookie handling, and the HTTP 401 contract for unauthenticated requests SHALL remain unchanged. The cross-cutting middleware is NOT shared through a separate "foundation" crate; each domain that serves HTTP owns its own copies of the layer factories, and the gateway owns its own copies at the composition layer.

#### Scenario: Protected domain route rejects missing auth in gateway-composed mode
- **WHEN** an unauthenticated request reaches a protected domain route registered through the gateway
- **THEN** the response is HTTP 401 with the existing error contract
- **AND** the domain handler is not invoked

#### Scenario: Protected domain route rejects missing auth in standalone mode
- **WHEN** the domain's `bin` is run standalone and an unauthenticated request reaches a protected route
- **THEN** the response is HTTP 401 with the existing error contract
- **AND** the domain handler is not invoked

### Requirement: Public behavior is preserved during rollout
The refactor SHALL preserve all current public route paths including `/`, `/health`, `/healthz`, `/media/**`, `/categories/**`, `/posts/**`, `/tags`, `/users/**`, administrator bucket/migration paths, and both GraphQL paths until an explicit future change modifies them. The composed gateway SHALL serve the same route set as the current `my-cms-api` during the transition; the legacy `application_core` crate SHALL be removed from the workspace as part of the Phase A cleanup, and the `migration` crate SHALL remain only as the standalone SeaORM CLI binary library (`apps/api/migration/src/main.rs` and `apps/api/migration/src/lib.rs`) reachable from `apps/api/test_helpers/src/lib.rs`. The `cms` legacy root package and the `legacy_bootstrap` binary SHALL be removed in Phase A; categories/tags/media/users domain cutover into the gateway composition is staged separately.

#### Scenario: Health and representative domain contracts remain stable
- **WHEN** the old and refactored gateway are exercised with the same valid configuration and request fixtures
- **THEN** representative status codes, response envelopes, auth decisions, and GraphQL endpoints match

#### Scenario: Composed gateway and standalone domain share the same observable behavior
- **WHEN** the same `domain_posts` crate is run as `cargo run -p domain_posts` (standalone) and as `cargo run -p gateway` (composed) with the same env-var surface
- **THEN** for each post-related route the response status, envelope, auth decision, and observable behavior are identical between the two runtimes

#### Scenario: application_core is absent from the workspace after Phase A
- **WHEN** the Phase A cleanup completes
- **THEN** `apps/api/application_core/` no longer exists in the repository
- **AND** `apps/api/Cargo.toml` `members` does not list `application_core`
- **AND** no `Cargo.toml` under `apps/api/` declares an `application_core` path-dependency

#### Scenario: migration crate is removed after the gateway cutover
- **WHEN** the Phase A cleanup completes
- **THEN** no `apps/api/migration/` directory or `migration` workspace member exists
- **AND** `apps/api/test_helpers/src/lib.rs` imports `domain_posts::migrations::{Migrator, MigratorTrait}` directly
- **AND** the gateway continues to call `domain_posts::migrations_cli::run` without a migration-crate indirection

