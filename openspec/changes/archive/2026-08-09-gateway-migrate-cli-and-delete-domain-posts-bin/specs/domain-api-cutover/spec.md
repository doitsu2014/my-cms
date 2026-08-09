## ADDED Requirements

### Requirement: Generic migration orchestrator dispatch
The gateway's migration orchestrator SHALL dispatch migrations per-domain via a generic interface (a `run_migrations` method on `DomainService` or an equivalent registry keyed by domain) instead of hard-coding per-domain dispatch arms. The orchestrator SHALL collect `MigrationDescriptor`s from every registered `DomainService`, deduplicate by id preserving first occurrence, sort by `(id, depends_on)`, and invoke each domain's runner for the descriptors it owns. Hard-coded id-prefix dispatch (e.g. `if d.id.starts_with("m2024") || d.id.starts_with("m2026")`) is forbidden after this change.

#### Scenario: Orchestrator dispatches via the trait surface
- **WHEN** `gateway::run_orchestrator` is invoked against the registered services
- **THEN** it iterates every `DomainService` and calls `service.run_migrations(conn, &descriptors)`
- **AND** `gateway::main.rs` does NOT contain a hard-coded `if d.id.starts_with(...)` branch
- **AND** the only `domain_posts::migrations_cli::*` reference in the gateway is the call into the runner, not a conditional dispatch

#### Scenario: New domain with no migrations is a no-op
- **WHEN** a registered `DomainService` returns an empty `migrations()` vector (e.g. `DomainMediaService`, `DomainUserService`, `DomainAuthService`)
- **THEN** the orchestrator skips the domain without invoking its runner
- **AND** no error is produced

#### Scenario: Descriptor dependency order is respected
- **WHEN** two descriptors declare `depends_on`, the orchestrator runs the dependency first
- **THEN** a future migration that depends on `m20240409_151952_release_100` runs after that identity
- **AND** if a cycle is detected the orchestrator returns `DomainConfigError::MigrationPlan` with the cycle path

#### Scenario: Migration execution failure surfaces a single error
- **WHEN** a domain's runner returns `Err(...)`
- **THEN** the orchestrator returns `DomainConfigError::MigrationExecution` with the failing descriptor id
- **AND** subsequent descriptors are not run
- **AND** the CLI subcommand path exits with `ExitCode::FAILURE`

### Requirement: Gateway exposes migration CLI subcommand
The `my-cms-api` binary SHALL accept a first positional argument `migrate` followed by a subcommand verb (`up`, `down`, `status`, or `--list`). When invoked as `my-cms-api migrate <verb>` the binary SHALL run the gateway's migration orchestrator against the shared `DatabaseConnection`, exit with `ExitCode::SUCCESS` on success or `ExitCode::FAILURE` on failure, and MUST NOT bind the HTTP listener. When invoked without arguments the binary SHALL continue to bind the HTTP listener (existing behaviour).

#### Scenario: migrate up applies pending migrations and exits
- **WHEN** the binary is invoked as `my-cms-api migrate up` against a database with pending migrations
- **THEN** the gateway's migration orchestrator runs every registered domain's pending migrations in topological order
- **AND** the binary exits with status `0`
- **AND** the HTTP listener is never bound

#### Scenario: migrate down reverts the last migration
- **WHEN** the binary is invoked as `my-cms-api migrate down` against a database with at least one applied migration
- **THEN** the gateway's migration orchestrator invokes `Migrator::down(conn, None)` for every domain that owns migrations
- **AND** the binary exits with status `0`
- **AND** the HTTP listener is never bound

#### Scenario: migrate status prints applied/pending state
- **WHEN** the binary is invoked as `my-cms-api migrate status`
- **THEN** every registered domain's migration status is printed to stdout, one descriptor per line, in the order produced by the orchestrator
- **AND** the four canonical post-domain identities are present in that order
- **AND** the binary exits with status `0`

#### Scenario: migrate --list prints migration identities
- **WHEN** the binary is invoked as `my-cms-api migrate --list`
- **THEN** every registered domain's migration identity is printed to stdout, one per line, in the order produced by the orchestrator
- **AND** the four canonical post-domain identities (`m20240409_151952_release_100`, `m20250330_151455_release_110`, `m20260126_040610_release_300`, `m20260531_000001_pgvector`) are present in that order
- **AND** the binary exits with status `0`

#### Scenario: migrate with unknown verb exits with usage
- **WHEN** the binary is invoked as `my-cms-api migrate <unknown-verb>`
- **THEN** the usage banner is printed to stderr
- **AND** the binary exits with status `1`
- **AND** the HTTP listener is never bound
- **AND** no database connection is opened

#### Scenario: Default invocation still boots the HTTP server
- **WHEN** the binary is invoked with no arguments
- **THEN** the HTTP listener binds and serves traffic
- **AND** the migration orchestrator still runs at boot (idempotent against the sea-orm-migration tracking table)

### Requirement: DomainService supports a per-domain migration runner
The `domain_interface::DomainService` trait SHALL expose `async fn run_migrations(&self, conn: &DatabaseConnection, descriptors: &[MigrationDescriptor]) -> Result<(), DomainConfigError>` with a default no-op implementation. Domains that own migrations (currently `domain_posts`) SHALL override the method and delegate to their `migrations_cli::run` helper. Domains that own no migrations (currently `domain_media`, `domain_user`, `domain_auth`) SHALL inherit the default `Ok(())` implementation without overriding.

#### Scenario: Default run_migrations is a no-op
- **WHEN** a stub `DomainService` invokes the default `run_migrations`
- **THEN** the call returns `Ok(())` without inspecting `conn` or `descriptors`
- **AND** `domain_interface` compiles with the existing three service implementors (`DomainPostService`, `DomainMediaService`, `DomainAuthService`) without modification

#### Scenario: DomainPostService overrides run_migrations
- **WHEN** `DomainPostService::run_migrations` is invoked with any `conn` and the four post-domain descriptors
- **THEN** it delegates to `domain_posts::migrations_cli::run(conn)`
- **AND** any error returned by `migrations_cli::run` is mapped to `DomainConfigError::MigrationExecution("domain-posts: <cause>")`
