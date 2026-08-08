# domain-api-cutover Specification

## Purpose
TBD - created by archiving change relocate-legacy-api-adapters-to-domains. Update Purpose after archive.
## Requirements
### Requirement: Single domain-owned API runtime
The system SHALL expose every supported CMS API route through the `my-cms-api` gateway using an owning domain service, and SHALL NOT require the legacy bootstrap runtime after cutover.

#### Scenario: Gateway exposes the complete route inventory
- **WHEN** the gateway is composed with post, media, user, and auth domain services
- **THEN** every route previously served by `legacy_bootstrap` is registered with the same HTTP method and path
- **AND** each route is registered in the same public, protected, or administrator authorization mount

#### Scenario: Legacy runtime is retired safely
- **WHEN** route and contract parity verification passes for the gateway
- **THEN** the legacy bootstrap binary and obsolete legacy API module tree are removed
- **AND** the workspace provides `my-cms-api` as the sole full API runtime

### Requirement: Domain ownership of adapters
The system SHALL locate API extraction and serialization adapters in the domain crate that owns the corresponding application handlers, while business behavior SHALL remain in trait-backed domain handlers.

#### Scenario: Media adapter ownership
- **WHEN** a public or authenticated media or bucket request is handled
- **THEN** the route invokes an adapter owned by `domain_media`
- **AND** the adapter delegates storage, validation, access policy, and cache behavior to existing `domain_media` handlers

#### Scenario: User adapter ownership
- **WHEN** an administrator user-management request is handled
- **THEN** the route invokes an adapter owned by `domain_user`
- **AND** the adapter delegates validation and GoTrue interaction to existing `domain_user` handlers

#### Scenario: Post adapter duplication is removed
- **WHEN** the cutover completes
- **THEN** post, category, and tag routes use the canonical `domain_posts` adapters
- **AND** no duplicate adapter implementation remains under `apps/api/src/api`

### Requirement: Observable API compatibility
The cutover SHALL preserve existing HTTP paths, methods, request and response shapes, status and error mapping, authentication audience, role authorization, GraphQL mounts, media privacy behavior, and external integration semantics.

#### Scenario: Authorized request parity
- **WHEN** a request accepted by the legacy runtime is sent to the gateway with equivalent configuration and credentials
- **THEN** the gateway returns the same contractually significant status, response envelope, and side effect

#### Scenario: Authentication and authorization failure parity
- **WHEN** a protected or administrator route receives missing, invalid, or insufficient credentials
- **THEN** the gateway preserves the existing 401 or 403 behavior
- **AND** private media remains obscured as not found where the existing contract requires it

#### Scenario: External integration failure parity
- **WHEN** Supabase Storage or GoTrue returns a mapped failure
- **THEN** the gateway returns the existing `AppError`-derived HTTP mapping
- **AND** service-role credentials and sensitive upstream content are not exposed in responses or traces

### Requirement: Dependency lifecycle and state consistency
The gateway SHALL construct each domain service dependency once per process and SHALL share the configured database connection while preserving media cache keys, bucket-visibility policy, and Supabase client configuration.

#### Scenario: Single startup composition
- **WHEN** `my-cms-api` starts successfully
- **THEN** media caches and Supabase clients are initialized once and supplied to their owning domain services
- **AND** all services receive the shared `DomainContext` database and GraphQL schemas

#### Scenario: Invalid startup configuration
- **WHEN** a required media or user environment variable is absent or invalid
- **THEN** startup fails deterministically before accepting traffic
- **AND** the failure identifies the configuration category without revealing secret values

### Requirement: Migration and rollback safety
The cutover SHALL preserve all database migration identities and schema state, and deployment SHALL support rollback to the prior two-runtime topology until gateway parity is accepted.

#### Scenario: No schema change
- **WHEN** the cutover artifacts are implemented
- **THEN** no new database migration is required
- **AND** no generated SeaORM entity file is manually edited

#### Scenario: Rollback before legacy deletion
- **WHEN** gateway parity verification fails during staged rollout
- **THEN** traffic can be returned to the previously deployed legacy bootstrap image without data rollback
- **AND** no API or schema incompatibility prevents that rollback

### Requirement: Verification evidence
The cutover SHALL be accepted only after deterministic route, auth, command-handler, external-contract, and workspace verification demonstrates parity and no forbidden legacy ownership remains.

#### Scenario: Focused verification succeeds
- **WHEN** media, bucket, user, post/category/tag, migration, and auth tests run
- **THEN** route inventory and representative success and failure contracts pass
- **AND** Storage and GoTrue tests use deterministic mocks rather than live services

#### Scenario: Cleanup verification succeeds
- **WHEN** the final cleanup is reviewed
- **THEN** source search finds no live `cms::api` or `legacy_bootstrap` consumer
- **AND** graph impact analysis finds no unexplained callers, imports, flows, or uncovered high-risk adapters

