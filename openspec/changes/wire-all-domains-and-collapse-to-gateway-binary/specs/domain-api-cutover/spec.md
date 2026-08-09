## MODIFIED Requirements

### Requirement: Single domain-owned API runtime
The system SHALL expose every supported CMS API route through the `my-cms-api` gateway using an owning domain service. The gateway's composition root (`gateway::manifest()`) SHALL register exactly one `DomainService` instance per owning domain: `DomainPostService` (post/category/tag/translation/GraphQL), `DomainMediaService` (public media delivery + administrator bucket lifecycle), `DomainUserService` (administrator user CRUD + password reset), and `DomainAuthService` (auth middleware only). After the change the gateway MUST depend on every workspace member that owns routes (`domain_posts`, `domain_media`, `domain_user`) plus `domain_auth` and `domain_interface`; the gateway MUST NOT depend on `domain_*` crates that own no routes without also registering them. Per-domain `[[bin]]` targets are forbidden as a deployment surface after this change — the only HTTP-serving binary the container ships SHALL be `my-cms-api`.

#### Scenario: Gateway exposes the complete route inventory
- **WHEN** the gateway is composed with post, media, user, and auth domain services
- **THEN** every route previously served by `legacy_bootstrap` is registered with the same HTTP method and path
- **AND** each route is registered in the same public, protected, or administrator authorization mount
- **AND** `gateway::manifest()` contains exactly four `Box<dyn DomainService>` entries: `DomainPostService`, `DomainMediaService`, `DomainUserService`, `DomainAuthService`

#### Scenario: Legacy runtime is retired safely
- **WHEN** route and contract parity verification passes for the gateway
- **THEN** the legacy bootstrap binary and obsolete legacy API module tree are removed
- **AND** the workspace provides `my-cms-api` as the sole full API runtime
- **AND** no `apps/api/domain_*/Cargo.toml` declares a `[[bin]]` target whose binary is shipped in the Docker image
- **AND** `apps/api/Dockerfile` builds exactly one binary (`my-cms-api`) and the runtime image copies exactly one binary

#### Scenario: Gateway Cargo.toml declares the full domain inventory
- **WHEN** the gateway composition is wired
- **THEN** `apps/api/gateway/Cargo.toml` `[dependencies]` includes `domain_interface`, `domain_posts`, `domain_auth`, `domain_media`, and `domain_user`
- **AND** no workspace member that owns HTTP routes is omitted from the gateway's `[dependencies]`

## ADDED Requirements

### Requirement: Gateway is the sole deployed API binary
The workspace SHALL NOT publish a per-domain standalone binary that exposes CMS routes. The container image SHALL contain exactly one HTTP-serving binary (`my-cms-api`). Migrations are NOT considered HTTP-serving and remain an in-process operator command of the gateway binary.

#### Scenario: No standalone domain binary is shipped
- **WHEN** `cargo build --release --workspace --bins` runs from `apps/api/`
- **THEN** exactly one HTTP-serving binary (`my-cms-api`) is produced
- **AND** `target/release/domain_posts`, `target/release/domain_media`, `target/release/domain_user`, `target/release/domain_auth` are absent
- **AND** `apps/api/Dockerfile` `RUN cargo build --release` line lists exactly one `--bin` argument (`--bin my-cms-api`)
- **AND** the runtime stage `COPY --from=builder` line copies exactly one binary

#### Scenario: No per-domain bin target exists in route-owning domains
- **WHEN** the workspace is inspected after the change
- **THEN** `apps/api/domain_posts/Cargo.toml` does not declare a `[[bin]]` block
- **AND** `apps/api/domain_media/Cargo.toml` does not declare a `[[bin]]` block
- **AND** `apps/api/domain_user/Cargo.toml` does not declare a `[[bin]]` block
- **AND** `apps/api/domain_auth/Cargo.toml` MAY continue to declare a placeholder `[[bin]]` only if its `main.rs` is preserved verbatim; the binary MUST NOT be shipped in the Docker image

#### Scenario: Migrations remain a library function
- **WHEN** `domain_posts::migrations_cli::run(conn)` is inspected
- **THEN** it remains callable from `gateway` without spawning a child process
- **AND** the `Migrator::up(conn, None)` semantics are unchanged
- **AND** `domain_posts::migrations::Migrator` continues to expose the four canonical migration identities in the original order

### Requirement: Gateway exposes migration CLI subcommand
The `my-cms-api` binary SHALL accept a first positional argument `migrate` followed by a subcommand verb (`up`, `down`, `status`, or `--list`). When invoked as `my-cms-api migrate <verb>` the binary SHALL run the gateway's migration orchestrator against the shared `DatabaseConnection`, exit with `ExitCode::SUCCESS` on success or `ExitCode::FAILURE` on failure, and MUST NOT bind the HTTP listener. When invoked without arguments the binary SHALL continue to bind the HTTP listener (existing behaviour). The Docker Swarm `migrate` one-shot SHALL use this subcommand.

#### Scenario: migrate up applies pending migrations and exits
- **WHEN** the binary is invoked as `my-cms-api migrate up` against a database with pending migrations
- **THEN** the gateway's migration orchestrator runs every registered domain's pending migrations in topological order
- **AND** the binary exits with status `0`
- **AND** the HTTP listener is never bound

#### Scenario: migrate --list prints migration identities
- **WHEN** the binary is invoked as `my-cms-api migrate --list`
- **THEN** every registered domain's migration identity is printed to stdout, one per line, in the order produced by the orchestrator
- **AND** the four canonical post-domain identities (`m20240409_151952_release_100`, `m20250330_151455_release_110`, `m20260126_040610_release_300`, `m20260531_000001_pgvector`) are present in that order
- **AND** the binary exits with status `0`

#### Scenario: Default invocation still boots the HTTP server
- **WHEN** the binary is invoked with no arguments
- **THEN** the HTTP listener binds and serves traffic
- **AND** the migration orchestrator still runs at boot (idempotent against the sea-orm-migration tracking table)

#### Scenario: Docker migrate service uses the gateway binary
- **WHEN** the Docker Swarm `migrate` service starts
- **THEN** its `entrypoint` is `["/app/my-cms-api"]`
- **AND** its `command` is `["migrate", "up"]`
- **AND** no `/app/domain_posts` binary is referenced anywhere in `deployments/`

### Requirement: Generic migration orchestrator dispatch
The gateway's migration orchestrator SHALL dispatch migrations per-domain via a generic interface (a `MigrationRunner` method on `DomainService` or an equivalent registry keyed by domain) instead of hard-coding per-domain dispatch arms. The orchestrator SHALL collect `MigrationDescriptor`s from every registered `DomainService`, deduplicate by `id` preserving first occurrence, topologically sort by `(id, depends_on)`, and invoke each domain's runner for the descriptors it owns. Hard-coded id-prefix dispatch (e.g. `if d.id.starts_with("m2024") || d.id.starts_with("m2026")`) is forbidden after this change.

#### Scenario: Orchestrator dispatches via the trait surface
- **WHEN** `gateway::run_orchestrator` is invoked against the registered services
- **THEN** it iterates every `DomainService` and calls a generic runner method on the trait (e.g. `service.run_migrations(conn, &descriptors)`)
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

### Requirement: domain_user is composed in the gateway
The `domain_user` crate SHALL expose a `DomainUserService` that implements `domain_interface::DomainService`, contributes the existing user-handler routes through a new `domain_user::api::routes` aggregator, validates required environment variables (`SUPABASE_URL`, `SUPABASE_SERVICE_ROLE_KEY`), exposes a `HealthDescriptor { name: "domain-user", version: env!("CARGO_PKG_VERSION") }`, and returns an empty `migrations()` vector. The gateway SHALL register this service in `manifest()`. The `domain_user` library surface MUST NOT export a `main` function.

#### Scenario: DomainUserService is registered and reachable
- **WHEN** `gateway::manifest()` is invoked
- **THEN** it contains `Box::new(DomainUserService::new(...))`
- **AND** the startup banner reports four registered domains
- **AND** `cargo run -p gateway` serves the user-handler routes on the same mount classification as `domain_auth` (administrator-only)

#### Scenario: DomainUserService implements DomainService
- **WHEN** `DomainUserService` is type-checked
- **THEN** it satisfies `domain_interface::DomainService`
- **AND** `health()` returns `name = "domain-user"`
- **AND** `migrations()` returns `Vec::new()`
- **AND** `register_routes()` returns the existing user handler routes aggregated into `Vec<RouteRegistration>` with `Mount::Administrator`

#### Scenario: domain_user has no standalone binary
- **WHEN** `apps/api/domain_user/Cargo.toml` is inspected
- **THEN** it does not declare a `[[bin]]` block
- **AND** no `apps/api/domain_user/src/main.rs` file exists
- **AND** the crate produces a `lib` target only
