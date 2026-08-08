# domain-auth-service Specification

## Purpose
TBD - created by archiving change extract-auth-into-domain-auth. Update Purpose after archive.
## Requirements
### Requirement: Blog Auth Service owns the Supabase authentication layer
`domain-auth` SHALL be a self-contained Cargo crate (lib + bin) that exposes the Supabase JWT validation layer (`SupabaseAuthLayer`, `SupabaseAuthConfig`, `SupabaseClaims`, `SupabaseToken`), the role-checking middleware, the `validate_supabase_token` async function, the `construct_supabase_auth_layer(audience: String, required_roles: Vec<String>) -> SupabaseAuthLayer` factory, and the env-var surface (`SUPABASE_URL`, `SUPABASE_INTERNAL_URL`, `SUPABASE_JWT_SECRET`, `SUPABASE_ANON_KEY`, `SUPABASE_SERVICE_ROLE_KEY`, `AUTHORIZATION_AUDIENCE`). It SHALL depend only on `domain_interface` (plus its own infrastructure dependencies — `axum`, `tower`, `jsonwebtoken`, `serde`, `tokio`, `async-trait`, `reqwest`). It SHALL NOT depend on any concrete business domain (`domain-posts`, `application_core`, `cms`). It SHALL publish a `DomainAuthService` that implements `DomainService` and whose `register_routes` returns an empty `Vec<RouteRegistration>` (auth is HTTP-middleware, not routes).

#### Scenario: Auth layer is reusable across all binaries
- **WHEN** the gateway composition root applies the auth layer to the merged protected router
- **THEN** every request to a `Mount::Protected` route extracts `Extension<AuthenticatedActor>`
- **AND** every request to a `Mount::Administrator` route extracts `Extension<AuthenticatedActor>` and enforces the `my-headless-cms-administrator` role
- **AND** every public route (`Mount::Public`) skips the auth layer
- **AND** the same auth types are used by the legacy `legacy_bootstrap` binary without code duplication

#### Scenario: Auth validates JWT and enforces roles
- **WHEN** an authenticated client calls a protected route with a valid Bearer token
- **THEN** the auth layer validates the JWT signature against `SUPABASE_JWT_SECRET` (HS256)
- **AND** the auth layer validates the audience claim against `AUTHORIZATION_AUDIENCE`
- **AND** the auth layer validates the role claim against `required_roles` (semantics: "any role in `required_roles` is acceptable")
- **AND** the handler receives `Extension<AuthenticatedActor>` with `actor.user_id`, `actor.email`, `actor.primary_role`, `actor.app_roles`

#### Scenario: Auth rejects invalid tokens
- **WHEN** a client calls a protected route with a missing Authorization header
- **THEN** the auth layer returns `401 Unauthorized` with body `{"error":"Missing Authorization header"}`
- **WHEN** a client calls a protected route with an invalid Bearer token
- **THEN** the auth layer returns `401 Unauthorized` with the JWT validation error message
- **WHEN** a client calls a protected route with an expired Bearer token
- **THEN** the auth layer returns `401 Unauthorized` with the JWT expiration error message
- **WHEN** a client calls a protected route with a valid token but no matching role
- **THEN** the auth layer returns `403 Forbidden` with body `{"error":"Insufficient permissions"}`

### Requirement: DomainAuthService implements the DomainService contract
`domain-auth::DomainAuthService` SHALL implement every method of `domain_interface::DomainService` (`apps/api/domain_interface/src/lib.rs:131-154`). Specifically:

- `health` SHALL return `HealthDescriptor { name: "domain-auth", version: env!("CARGO_PKG_VERSION") }`.
- `required_env` SHALL return `&["SUPABASE_URL", "SUPABASE_JWT_SECRET", "AUTHORIZATION_AUDIENCE"]`.
- `validate_config` SHALL read the three env vars above and return `DomainConfigError::MissingEnv(<var-name>)` for each missing variable; otherwise return `Ok(())`.
- `migrations` SHALL return an empty `Vec<MigrationDescriptor>` (auth has no DB schema).
- `register_routes` SHALL return an empty `Vec<RouteRegistration>` (auth is HTTP-middleware, not routes).
- `startup_health` SHALL use the default `Ok(())` impl from `DomainService` (auth is infrastructure-only and does not probe the shared `DatabaseConnection`).

#### Scenario: DomainAuthService is object-safe through the trait
- **WHEN** `DomainAuthService::new()` is constructed and boxed as `Box<dyn DomainService>`
- **THEN** the construction compiles
- **AND** the boxed instance is `Send + Sync` (matching the trait bounds)
- **AND** every method can be called through the trait object without monomorphization

#### Scenario: DomainAuthService declares exactly the auth-relevant env vars
- **WHEN** `DomainAuthService::required_env()` is called
- **THEN** it returns `&["SUPABASE_URL", "SUPABASE_JWT_SECRET", "AUTHORIZATION_AUDIENCE"]`
- **AND** no other env var is declared (env vars used by other domains are not declared here)

#### Scenario: DomainAuthService owns no migrations
- **WHEN** `DomainAuthService::migrations()` is called
- **THEN** it returns an empty `Vec<MigrationDescriptor>`
- **AND** the gateway's migration orchestrator iterates the descriptor list and finds no entries for `domain-auth`

#### Scenario: DomainAuthService registers no routes
- **WHEN** `DomainAuthService::register_routes(&ctx)` is called with any `DomainContext`
- **THEN** it returns an empty `Vec<RouteRegistration>`
- **AND** the gateway's `compose_routers` merges zero entries from `domain-auth`

#### Scenario: DomainAuthService::validate_config succeeds when all env vars are set
- **WHEN** `DomainAuthService::validate_config()` is called with `SUPABASE_URL`, `SUPABASE_JWT_SECRET`, and `AUTHORIZATION_AUDIENCE` all set in the environment
- **THEN** it returns `Ok(())`

#### Scenario: DomainAuthService::validate_config fails for each missing env var independently
- **WHEN** `DomainAuthService::validate_config()` is called with `SUPABASE_URL` unset (and the other two vars set)
- **THEN** it returns `Err(DomainConfigError::MissingEnv("SUPABASE_URL"))`
- **WHEN** `DomainAuthService::validate_config()` is called with `SUPABASE_JWT_SECRET` unset (and the other two vars set)
- **THEN** it returns `Err(DomainConfigError::MissingEnv("SUPABASE_JWT_SECRET"))`
- **WHEN** `DomainAuthService::validate_config()` is called with `AUTHORIZATION_AUDIENCE` unset (and the other two vars set)
- **THEN** it returns `Err(DomainConfigError::MissingEnv("AUTHORIZATION_AUDIENCE"))`

#### Scenario: DomainAuthService::startup_health does not probe the database
- **WHEN** `DomainAuthService::startup_health(&ctx)` is called with any `DomainContext` (healthy or otherwise)
- **THEN** it returns `Ok(())`
- **AND** the implementation does NOT call `ctx.conn.execute_unprepared("SELECT 1")` or any other DB operation
- **AND** DB readiness is delegated to `domain_posts::DomainPostService::startup_health`, which retains its `SELECT 1` probe

### Requirement: Auth extraction is the template for future domain extractions
This change SHALL serve as the **reference migration** for extracting any future business domain (`domain-media`, `domain-users`, `domain-administrator`) out of the legacy `cms` tree or out of `domain_posts`. The artifact set, the design decisions, and the task groups SHALL be reusable as a copy-paste template with minimal adaptation. Specifically:

- `openspec/changes/<future-domain>/proposal.md` SHALL follow the structure of this change's `proposal.md` (Why, What Changes, Capabilities, Impact).
- `openspec/changes/<future-domain>/design.md` SHALL include a "Template value for future domains" section with a copy-paste checklist.
- `openspec/changes/<future-domain>/tasks.md` SHALL include a task group 5 ("Verify `DomainService` contract compliance") modeled on this change's task group 5, and a task group for the mechanical update of every consumer's import path.
- `apps/api/<future-domain>/Cargo.toml` SHALL declare `domain_interface = { path = "../domain_interface" }` as a dependency.
- If the future domain has auth-protected routes, the routes SHALL extract `Extension<AuthenticatedActor>` (imported from `domain_interface`), NEVER `Extension<SupabaseToken>` (from `domain_auth`) or any other auth-specific type.
- `apps/api/<future-domain>/src/service.rs` SHALL implement `DomainService` with the same shape as `domain_auth::DomainAuthService` (object-safe, `required_env` declares the domain's env vars, `validate_config` checks them, `migrations` returns descriptors or empty, `register_routes` returns the bare Axum routers, `startup_health` uses the default `Ok(())` for infra-only domains or overrides with a DB probe for DB-backed domains, `health` returns a `HealthDescriptor` with the domain's name and version).

#### Scenario: Future domain extraction follows the same five-phase migration pattern
- **WHEN** `domain-media`, `domain-users`, or `domain-administrator` is extracted as a new crate in a future change
- **THEN** that change's `proposal.md` includes a "Template for future domains" subsection modeled on this change's
- **AND** that change's `tasks.md` has a task group 5 for `DomainService` contract compliance with seven specific test tasks (object-safety, `required_env`, `migrations`, `register_routes`, three `validate_config` cases, `startup_health` with healthy `MockDatabase` or default)
- **AND** that change's `tasks.md` has a task group for the mechanical update of every consumer's import path with the full file list enumerated
- **AND** that change's `Cargo.toml` declares `domain_interface = { path = "../domain_interface" }` as a dependency
- **AND** that change's HTTP adapters extract `Extension<AuthenticatedActor>` (never `Extension<SupabaseToken>`)
- **AND** that change's `DomainService` impl passes the seven contract-compliance tests in its own test suite

#### Scenario: Future domain extraction updates docs/adding-a-domain.md
- **WHEN** `domain-media`, `domain-users`, or `domain-administrator` is extracted in a future change
- **THEN** `docs/adding-a-domain.md` adds a domain-specific checklist derived from this change's task group 5 and task group 6
- **AND** the checklist enumerates every file that needs an import-path update, with current line numbers
- **AND** the checklist explicitly notes that `Extension<AuthenticatedActor>` is the canonical extractor type for auth-protected routes

### Requirement: Auth domain registers as a DomainService for composition-time validation
`domain-auth::DomainAuthService` SHALL implement `DomainService`. Its `validate_config` reads `SUPABASE_URL`, `SUPABASE_JWT_SECRET`, `AUTHORIZATION_AUDIENCE` from the env and returns `DomainConfigError::MissingEnv(var_name)` for each missing variable. Its `startup_health` uses the default `Ok(())` impl (auth does not probe the database). Its `migrations` returns an empty `Vec<MigrationDescriptor>` (auth has no DB schema). Its `register_routes` returns an empty `Vec<RouteRegistration>`. Its `health` returns `HealthDescriptor { name: "domain-auth", version: env!("CARGO_PKG_VERSION") }`. Its `required_env` returns `&["SUPABASE_URL", "SUPABASE_JWT_SECRET", "AUTHORIZATION_AUDIENCE"]`.

#### Scenario: Gateway composes the auth domain
- **WHEN** the gateway boots with `DomainPostService` and `DomainAuthService` in the manifest
- **THEN** the gateway validates both domains' config before binding the listener
- **AND** the gateway calls `startup_health` on both domains and aborts if either fails
- **AND** the auth layer is applied to the protected router via `domain_auth::factory::auth_layer_from_env`
- **AND** the public router is unchanged (no auth layer)

#### Scenario: Auth domain is deployable as a standalone microservice
`domain-auth` SHALL be runnable as a standalone binary (`cargo run -p domain-auth`) that boots its own Axum server (currently a no-op health-only stub), opens its own database connection (if and only if the binary needs one — pure infra-only auth does not), applies its own OpenTelemetry layers, and exposes `/health`. The standalone mode is for development and integration testing; production deployments use the auth layer through the gateway composition.

#### Scenario: Standalone auth serves the same env contract
- **WHEN** `cargo run -p domain-auth` is run with the same env-var surface as the current `my-cms-api`
- **THEN** the auth domain boots, reads `SUPABASE_URL` / `SUPABASE_JWT_SECRET` / `AUTHORIZATION_AUDIENCE` from the env, validates them, and serves `/health` with `200 OK`
- **AND** the domain's own `/health` returns `200 OK` with the auth service's health descriptor

### Requirement: Business domains consume auth through `domain_interface::AuthenticatedActor`, not through `domain_auth`
After this change, every business domain (`domain-posts`, future `domain-media`, future `domain-users`, future `domain-administrator`) SHALL import `AuthenticatedActor` from `domain_interface` (not from `domain_posts` or any other business domain, and NOT from `domain_auth`). Domain HTTP adapters SHALL extract `Extension<AuthenticatedActor>` (not `Extension<SupabaseToken>`). The `SupabaseToken` and `SupabaseClaims` types remain in `domain_auth` as JWT-level DTOs but are only consumed inside `domain_auth`.

#### Scenario: domain_posts does not own the auth layer
- **WHEN** `cargo tree -p domain_posts` runs
- **THEN** no `jsonwebtoken` or `tower-http` dependency appears in the direct dependencies of `domain-posts` (the auth layer was the only consumer of these)
- **AND** `domain_auth` appears in the dependency tree as a workspace member
- **AND** every handler in `domain_posts/src/api/` extracts `Extension<AuthenticatedActor>` (imported from `domain_interface`)

#### Scenario: Future domain extractions depend only on domain_interface for actor info
- **WHEN** `domain-media`, `domain-users`, or `domain-administrator` is extracted as a new crate in a future change
- **THEN** that crate's `Cargo.toml` declares `domain_interface = { path = "../domain_interface" }` as a `[dependencies]` entry
- **AND** that crate's HTTP adapters extract `Extension<AuthenticatedActor>` (imported from `domain_interface`)
- **AND** that crate does NOT depend on `domain-posts` for the actor type
- **AND** that crate does NOT depend on `domain-auth` for the actor type (the actor is read directly from `domain_interface`)

