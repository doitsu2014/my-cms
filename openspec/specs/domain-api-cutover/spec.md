# domain-api-cutover Specification

## Purpose
The `domain-api-cutover` capability owns the contract that the single-binary
`my-cms-api` gateway exposes every CMS HTTP route through one registered
`DomainService` per owning domain. It also owns the operator-facing migration
CLI surface (`my-cms-api migrate <verb>`), the per-domain migration
dispatch contract, and the deployment surface (single shipped binary).

## Requirements

### Requirement: Single domain-owned API runtime
The system SHALL expose every supported CMS API route through the `my-cms-api` gateway using an owning domain service. The gateway's composition root (`gateway::manifest()`) SHALL register exactly one `DomainService` instance per owning domain: `DomainPostService` (post/category/tag/translation/GraphQL), `DomainMediaService` (public media delivery + administrator bucket lifecycle), `DomainUserService` (administrator user CRUD + password reset), and `DomainAuthService` (auth middleware only). After the change the gateway MUST depend on every workspace member that owns routes (`domain_posts`, `domain_media`, `domain_user`) plus `domain_auth` and `domain_interface`; the gateway MUST NOT depend on `domain_*` crates that own no routes without also registering them.

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

#### Scenario: manifest() registers four domain services
- **WHEN** `gateway::manifest()` is invoked after the change
- **THEN** the returned `Vec<Box<dyn DomainService>>` has length 4
- **AND** the four entries are `DomainPostService`, `DomainAuthService`, `DomainMediaService`, `DomainUserService` in that order
- **AND** every entry's `health().name` is one of `domain-posts`, `domain-auth`, `domain-media`, `domain-user`

### Requirement: domain_user is composed in the gateway
The `domain_user` crate SHALL expose a `DomainUserService` that implements `domain_interface::DomainService`, contributes the existing user-handler routes through a new `domain_user::api::routes` aggregator, validates required environment variables (`SUPABASE_URL`, `SUPABASE_SERVICE_ROLE_KEY`), exposes a `HealthDescriptor { name: "domain-user", version: env!("CARGO_PKG_VERSION") }`, and returns an empty `migrations()` vector. The gateway SHALL register this service in `manifest()`. The `domain_user` library surface MUST NOT export a `main` function.

#### Scenario: DomainUserService is registered and reachable
- **WHEN** `gateway::manifest()` is invoked
- **THEN** it contains `Box::new(DomainUserService::from_state(...))`
- **AND** the startup banner reports four registered domains
- **AND** `cargo run -p gateway` serves the user-handler routes on the same mount classification as `domain_auth` (administrator-only)

#### Scenario: DomainUserService implements DomainService
- **WHEN** `DomainUserService` is type-checked
- **THEN** it satisfies `domain_interface::DomainService`
- **AND** `health()` returns `name = "domain-user"`
- **AND** `migrations()` returns `Vec::new()`
- **AND** `register_routes()` returns the existing user handler routes aggregated into `Vec<RouteRegistration>` with `Mount::Administrator`

#### Scenario: domain_user::api::routes wraps the user handler modules
- **WHEN** `domain_user::api::routes::routes(state)` is invoked with a `UserApiState`
- **THEN** it returns exactly one `RouteRegistration` with `mount = Mount::Administrator`
- **AND** that registration covers `POST /users`, `GET /users`, `GET /users/:id`, `PUT /users/:id`, `DELETE /users/:id`, and `POST /users/:id/reset-password`
- **AND** the prefix is `"users"`

#### Scenario: domain_user has no standalone binary
- **WHEN** `apps/api/domain_user/Cargo.toml` is inspected
- **THEN** it does not declare a `[[bin]]` block
- **AND** no `apps/api/domain_user/src/main.rs` file exists
- **AND** the crate produces a `lib` target only

### Requirement: MediaConfig is constructible from environment variables
The `domain_media` crate SHALL expose `MediaConfig::from_env()` that reads `SUPABASE_URL`, `SUPABASE_SERVICE_ROLE_KEY`, `MEDIA_BUCKET`, and `MEDIA_BASE_URL` and returns `Result<MediaConfig, DomainConfigError>`. The factory is consumed by `gateway::main` to construct `MediaConfig` once at startup; the factory MUST fail-fast on a missing or invalid env value with `DomainConfigError::MissingEnv(<var>)`. Existing callers that construct `MediaConfig` inline continue to compile.

#### Scenario: from_env reads the four env vars
- **WHEN** all four env vars are set
- **THEN** `MediaConfig::from_env()` returns `Ok(MediaConfig { storage, bucket, media_base_url })`
- **AND** `storage` is a `SupabaseStorage` constructed from the two Supabase env vars
- **AND** `bucket` equals `MEDIA_BUCKET`
- **AND** `media_base_url` equals `MEDIA_BASE_URL`

#### Scenario: from_env fails fast on missing env
- **WHEN** any of `SUPABASE_URL`, `SUPABASE_SERVICE_ROLE_KEY`, `MEDIA_BUCKET`, `MEDIA_BASE_URL` is unset
- **THEN** `MediaConfig::from_env()` returns `Err(DomainConfigError::MissingEnv(<var>))` identifying the first missing var
- **AND** the gateway exits with `ExitCode::FAILURE` before binding the HTTP listener

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

### Requirement: DomainService supports a per-domain migration runner
The `domain_interface::DomainService` trait SHALL expose `async fn run_migrations(&self, conn: &DatabaseConnection, descriptors: &[MigrationDescriptor]) -> Result<(), DomainConfigError>` with a default no-op implementation. Domains that own migrations (currently `domain_posts`) SHALL override the method and delegate to their `migrations_cli::run` helper. Domains that own no migrations (currently `domain_media`, `domain_user`, `domain_auth`) SHALL inherit the default `Ok(())` implementation without overriding.

#### Scenario: Default run_migrations is a no-op
- **WHEN** a stub `DomainService` invokes the default `run_migrations`
- **THEN** the call returns `Ok(())` without inspecting `conn` or `descriptors`
- **AND** `domain_interface` compiles with the existing service implementors (`DomainPostService`, `DomainMediaService`, `DomainAuthService`, `DomainUserService`) without modification

#### Scenario: DomainPostService overrides run_migrations
- **WHEN** `DomainPostService::run_migrations` is invoked with any `conn` and the four post-domain descriptors
- **THEN** it delegates to `domain_posts::migrations_cli::run(conn)`
- **AND** any error returned by `migrations_cli::run` is mapped to `DomainConfigError::MigrationExecution("domain-posts: <cause>")`

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

#### Scenario: Docker migrate service uses the gateway binary
- **WHEN** the Docker Swarm `migrate` service starts
- **THEN** its `entrypoint` is `["/app/my-cms-api"]`
- **AND** its `command` is `["migrate", "up"]`
- **AND** no `/app/domain_posts` binary is referenced anywhere in `deployments/`

### Requirement: Gateway is the sole deployed API binary
The workspace SHALL NOT publish a per-domain standalone binary that exposes CMS routes. The container image SHALL contain exactly one HTTP-serving binary (`my-cms-api`). Migrations are NOT considered HTTP-serving and remain an in-process operator command of the gateway binary.

#### Scenario: No standalone domain binary is shipped
- **WHEN** `cargo build --release --workspace --bins` runs from `apps/api/`
- **THEN** exactly one HTTP-serving binary (`my-cms-api`) is produced
- **AND** `target/release/domain_posts`, `target/release/domain_media`, `target/release/domain_user`, `target/release/domain_auth` are absent
- **AND** `apps/api/Dockerfile` builds the workspace (the gateway depends on every `domain_*` crate) and the runtime stage `COPY --from=build` line copies exactly one binary (`my-cms-api`)
- **AND** the runtime image `ls /app` contains only `my-cms-api`

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

### Requirement: Operator-facing documentation matches the deployed CLI surface
Every operator-facing doc file that names a `domain_posts` migration CLI invocation SHALL be updated to name the gateway-binary equivalent. Historical references in `openspec/changes/archive/` and `docs/superpowers/plans/2026-08-08-remove-legacy-migration-crate.md` are exempt (decision archaeology).

#### Scenario: docs/api-architecture.md names the gateway binary
- **WHEN** the file is inspected after the change
- **THEN** no live operator reference to `domain_posts migrate up`, `cargo run -p domain_posts -- migrate`, or `/app/domain_posts` remains outside explicitly-labelled historical sections

#### Scenario: docs/pluggable-domain-refactor.md marks the standalone bin removed
- **WHEN** the file is inspected after the change
- **THEN** Stage 4 description marks the standalone-binary removal as completed
- **AND** every `cargo run -p domain_posts` reference is replaced with `cargo run -p gateway`
- **AND** no live reference to `cargo run -p domain_posts -- migrate --list` remains

#### Scenario: docs/ai-platform.md names the gateway binary
- **WHEN** the file is inspected after the change
- **THEN** line 58 names `cargo run -p gateway -- migrate [--list]`
- **AND** no live reference to `cargo run -p domain_posts -- migrate [--list]` remains

#### Scenario: .opencode/agents/product-owner.md names the gateway binary
- **WHEN** the file is inspected after the change
- **THEN** line 72 names `/app/my-cms-api migrate up`
- **AND** no live reference to `/app/domain_posts` remains outside historical notes

#### Scenario: .opencode/agents/software-architect.md points at the gateway binary
- **WHEN** the file is inspected after the change
- **THEN** the migration-CLI row (around line 75-97) points at `apps/api/gateway/src/migrate_cli.rs`
- **AND** no live reference to `apps/api/domain_posts/src/main.rs` remains outside historical notes
