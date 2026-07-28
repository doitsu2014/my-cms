## ADDED Requirements

### Requirement: Domain services expose a stable dyn-compatible contract
The workspace SHALL provide a `domain-interface` library defining a dyn-compatible `DomainService` contract. The contract SHALL expose a stable name and semantic version, mount prefix, route registration against a shared `DomainContext`, health checking, configuration validation, and migration registration. Registration SHALL return a result and SHALL not expose domain implementation types to the gateway.

#### Scenario: Gateway registers a service through trait dispatch
- **WHEN** the gateway holds `Vec<Box<dyn DomainService>>`
- **THEN** it can register routes and query metadata without importing a domain's commands, entities, or DTOs
- **AND** the service remains object-safe for dynamic dispatch

### Requirement: Domain context is composed once and shared
The interface SHALL provide a cloneable context containing the shared `Arc<DatabaseConnection>`, foundation services/configuration, and GraphQL contribution registry. Domain-owned state SHALL be added behind domain service construction rather than duplicated gateway connections.

#### Scenario: Multiple domains share one database lifecycle
- **WHEN** two registered domains handle requests
- **THEN** both receive the same connection pool/context instance
- **AND** startup creates the database connection once

### Requirement: Domain migrations are ordered and owned
Each service SHALL return its migration set and dependency metadata. The gateway SHALL execute all selected migration sets in deterministic dependency order, and SHALL fail startup or the administrator migration operation with an `AppError`-mapped failure when ordering or execution fails.

#### Scenario: Migration dependency order is deterministic
- **WHEN** domain-post depends on foundation migrations
- **THEN** foundation migrations run before post migrations
- **AND** a cycle or duplicate migration identity prevents execution with a diagnostic error

### Requirement: Domain configuration and health are observable
Each service SHALL validate its configuration before routes are served and SHALL expose a name, version, and health result suitable for a gateway health aggregation. Health failures SHALL be reported without leaking secrets.

#### Scenario: Invalid domain configuration stops unsafe startup
- **WHEN** a registered domain lacks a required configuration value
- **THEN** gateway startup returns a configuration error
- **AND** no listener is advertised as ready
