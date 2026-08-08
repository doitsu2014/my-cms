## ADDED Requirements

### Requirement: Blog Post Service owns the post vertical slice
`domain-posts` SHALL be a self-contained Cargo crate (lib + bin) that exposes post REST adapters, post application-layer command handlers, presentation models, domain common/error composition, required generated entities, translation integration, GraphQL contribution, and post migrations through one package boundary. It SHALL NOT depend on a shared `application_core`, `domain_foundation`, or central `migration` crate. It SHALL preserve the existing post and translation HTTP and GraphQL contracts.

#### Scenario: Existing post route remains compatible after extraction
- **WHEN** an authenticated client calls the existing post create, read, modify, delete, or translate route
- **THEN** the same path, method, authorization role, response envelope, and error mapping are returned
- **AND** command business logic remains outside the Axum gateway

### Requirement: Post GraphQL contribution is registered by the domain
The post service SHALL contribute its Seaography schema/query registration to the gateway's immutable and mutable GraphQL mounts without the gateway importing post entities directly. Existing `/graphql/immutable` and `/graphql/mutable` behavior SHALL remain available.

#### Scenario: GraphQL schema includes post operations
- **WHEN** the gateway registers domain-posts and builds both schemas
- **THEN** post query fields remain available on immutable GraphQL
- **AND** the existing mutable authorization boundary remains enforced

### Requirement: Post migration ownership preserves schema
The post service SHALL own or explicitly reference the migrations for posts, post relations, translations, and translation jobs, while preserving the current ordered database history and schema-first entity regeneration. The migration identities (`m20240409_151952_release_100`, `m20250330_151455_release_110`, `m20260126_040610_release_300`, `m20260531_000001_pgvector`) SHALL be preserved exactly so the database `up` history is unchanged.

#### Scenario: Existing database upgrades without destructive reset
- **WHEN** the refactored migration runner is applied to a database at the current migration level
- **THEN** no existing post data is dropped
- **AND** already-applied migration identities are not re-executed

### Requirement: Post service is deployable as a standalone microservice
`domain-posts` SHALL be runnable as a standalone binary (its own `bin` target) that boots its own Axum server, opens its own database connection, applies its own auth/CORS/cookie/body-limit/OpenTelemetry layers, runs its own migrations through `cargo run -p domain-posts -- migrate`, and exposes `/health`, `/healthz`, `/posts/**`, `/posts/{post_id}/translate`, `/posts/{post_id}/translate/background`, `/posts/{post_id}/translate/jobs/**` from its own Axum router. The same `domain-posts` crate SHALL be composable as a `Box<dyn DomainService>` inside the gateway without duplicating routes, handlers, or migrations.

#### Scenario: Standalone domain serves the same route set
- **WHEN** `cargo run -p domain-posts` is run with the same env-var surface as the current `my-cms-api`
- **THEN** the post-related routes return the same status codes, envelopes, and authorization decisions as the composed gateway
- **AND** the domain's own `/health` returns 200 with the post service's health descriptor