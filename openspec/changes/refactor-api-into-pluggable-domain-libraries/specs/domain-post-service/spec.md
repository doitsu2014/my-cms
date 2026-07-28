## ADDED Requirements

### Requirement: Blog Post Service owns the post vertical slice
`domain-post` SHALL expose post REST adapters, post application-core command handlers, presentation models, domain common/error composition, required generated entities, translation integration, GraphQL contribution, and post migrations through one library boundary. It SHALL preserve the existing post and translation HTTP and GraphQL contracts.

#### Scenario: Existing post route remains compatible after extraction
- **WHEN** an authenticated client calls the existing post create, read, modify, delete, or translate route
- **THEN** the same path, method, authorization role, response envelope, and error mapping are returned
- **AND** command business logic remains outside the Axum gateway

### Requirement: Post GraphQL contribution is registered by the domain
The post service SHALL contribute its Seaography schema/query registration to the gateway's immutable and mutable GraphQL mounts without the gateway importing post entities directly. Existing `/graphql/immutable` and `/graphql/mutable` behavior SHALL remain available.

#### Scenario: GraphQL schema includes post operations
- **WHEN** the gateway registers domain-post and builds both schemas
- **THEN** post query fields remain available on immutable GraphQL
- **AND** the existing mutable authorization boundary remains enforced

### Requirement: Post migration ownership preserves schema
The post service SHALL own or explicitly reference the migrations for posts, post relations, translations, and translation jobs, while preserving the current ordered database history and schema-first entity regeneration.

#### Scenario: Existing database upgrades without destructive reset
- **WHEN** the refactored migration runner is applied to a database at the current migration level
- **THEN** no existing post data is dropped
- **AND** already-applied migration identities are not re-executed
