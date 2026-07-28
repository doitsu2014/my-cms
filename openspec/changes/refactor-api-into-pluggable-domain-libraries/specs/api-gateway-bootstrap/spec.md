## ADDED Requirements

### Requirement: Gateway is a thin domain composition root
The `my-cms-api` binary SHALL own environment loading, one database connection, shared context construction, domain registration, cross-cutting auth/CORS/body-limit/tracing/cookie layers, listener lifecycle, and graceful startup errors. It SHALL NOT import domain commands, entities, DTOs, storage clients, or business rules.

#### Scenario: Startup succeeds with one or more domains
- **WHEN** required configuration is valid and N>=1 domain services are registered
- **THEN** the gateway builds one router and starts the listener
- **AND** readiness/health reports the registered services

### Requirement: Domains compose without gateway route edits
The gateway SHALL register domains by iterating the `DomainService` collection. Adding a conforming domain library SHALL require registration/configuration in the composition manifest only, not new gateway route or handler logic.

#### Scenario: Two domains register independently
- **WHEN** domain-post and a test domain are registered
- **THEN** both route sets are reachable under their declared prefixes
- **AND** neither domain imports the other's internals

### Requirement: Cross-cutting authorization remains compatible
Auth middleware SHALL remain in the foundation/gateway layer and SHALL preserve Supabase JWT validation, token request extensions, writer/admin OR semantics, administrator-only routes, CORS behavior, tracing/OpenTelemetry layers, body limits, and cookie handling.

#### Scenario: Protected domain route rejects missing auth
- **WHEN** an unauthenticated request reaches a protected domain route
- **THEN** the response is HTTP 401 with the existing error contract
- **AND** the domain handler is not invoked

### Requirement: Public behavior is preserved during rollout
The refactor SHALL preserve all current public route paths including `/`, `/health`, `/healthz`, `/media/**`, `/categories/**`, `/posts/**`, `/tags`, `/users/**`, administrator bucket/migration paths, and both GraphQL paths until an explicit future change modifies them.

#### Scenario: Health and representative domain contracts remain stable
- **WHEN** the old and refactored gateway are exercised with the same valid configuration and request fixtures
- **THEN** representative status codes, response envelopes, auth decisions, and GraphQL endpoints match
