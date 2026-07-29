## ADDED Requirements

### Requirement: Domain services expose a stable dyn-compatible contract
The workspace SHALL provide a `domain-interface` library defining a dyn-compatible `DomainService` contract. The contract SHALL expose a stable name and semantic version, mount prefix, route registration against a shared `DomainContext`, health checking, configuration validation, and migration registration. Registration SHALL return a result and SHALL NOT expose domain implementation types to the gateway. The `domain-interface` crate SHALL be `publish = true` and SHALL NOT depend on any concrete domain crate (no `domain-posts`, no `application_core`, no `migration`), so it can be a Cargo dependency of any domain.

#### Scenario: Gateway registers a service through trait dispatch
- **WHEN** the gateway holds `Vec<Box<dyn DomainService>>`
- **THEN** it can register routes and query metadata without importing a domain's commands, entities, or DTOs
- **AND** the service remains object-safe for dynamic dispatch

#### Scenario: Domain-interface is a publishable contract crate
- **WHEN** `cargo metadata --format-version 1` is run for the `domain-interface` crate
- **THEN** its `[package].publish` is `true`
- **AND** its `[dependencies]` contain no entry referencing a concrete domain crate

### Requirement: Domain context is composed once and shared
The interface SHALL provide a cloneable context containing the shared `Arc<DatabaseConnection>`, the GraphQL contribution registry, and any runtime configuration the gateway needs to expose to every domain. Domain-owned state SHALL be added behind domain service construction rather than duplicated gateway connections.

#### Scenario: Multiple domains share one database lifecycle
- **WHEN** two registered domains handle requests
- **THEN** both receive the same connection pool/context instance
- **AND** startup creates the database connection once

### Requirement: Domain migrations are ordered and owned per domain
Each service SHALL return its migration set and dependency metadata via `MigrationDescriptor`s with `id` and `depends_on` fields. The gateway SHALL execute all selected migration sets in deterministic dependency order, and SHALL fail startup or the administrator migration operation with a `DomainConfigError`-mapped failure when ordering or execution fails. The `domain-interface` crate SHALL NOT own a `Migrator` or migration orchestrator; migrations are owned by each domain crate and surfaced only as descriptors through the contract.

#### Scenario: Migration dependency order is deterministic
- **WHEN** domain-posts returns a `MigrationDescriptor` whose `depends_on` references the ID of a migration owned by another domain
- **THEN** the depending migration runs after the depended-on migration
- **AND** a cycle or duplicate migration identity prevents execution with a diagnostic error

#### Scenario: Migration identity is preserved across the refactor
- **WHEN** the orchestrator runs the post-domain migration set against a database at the current migration level
- **THEN** the migration identities `m20240409_151952_release_100`, `m20250330_151455_release_110`, `m20260126_040610_release_300`, `m20260531_000001_pgvector` are reported in the original order
- **AND** no migration is re-executed

### Requirement: Domain configuration and health are observable
Each service SHALL validate its configuration before routes are served and SHALL expose a name, version, and health result suitable for a gateway health aggregation. Health failures SHALL be reported without leaking secrets.

#### Scenario: Invalid domain configuration stops unsafe startup
- **WHEN** a registered domain lacks a required configuration value
- **THEN** gateway startup returns a configuration error
- **AND** no listener is advertised as ready