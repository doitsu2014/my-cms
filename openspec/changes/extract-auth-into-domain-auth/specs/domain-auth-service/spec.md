## ADDED Requirements

### Requirement: Blog Auth Service owns the Supabase authentication layer
`domain-auth` SHALL be a self-contained Cargo crate (lib + bin) that exposes the Supabase JWT validation layer (`SupabaseAuthLayer`, `SupabaseAuthConfig`, `SupabaseClaims`, `SupabaseToken`), the role-checking middleware, the `validate_supabase_token` async function, the `construct_supabase_auth_layer(audience: String, required_roles: Vec<String>) -> SupabaseAuthLayer` factory, and the env-var surface (`SUPABASE_URL`, `SUPABASE_INTERNAL_URL`, `SUPABASE_JWT_SECRET`, `SUPABASE_ANON_KEY`, `SUPABASE_SERVICE_ROLE_KEY`, `AUTHORIZATION_AUDIENCE`). It SHALL depend only on `domain_interface` (plus its own infrastructure dependencies — `axum`, `tower`, `jsonwebtoken`, `serde`, `tokio`, `async-trait`). It SHALL NOT depend on any concrete business domain (`domain-posts`, `domain-categories`, `domain-ai`, `domain-translate`, `application_core`, `cms`). It SHALL publish a `DomainAuthService` that implements `DomainService` and whose `register_routes` returns an empty `Vec<RouteRegistration>` (auth is HTTP-middleware, not routes).

#### Scenario: Auth layer is reusable across all binaries
- **WHEN** the gateway composition root applies the auth layer to the merged protected router
- **THEN** every request to a `Mount::Protected` route extracts `Extension<SupabaseToken>`
- **AND** every request to a `Mount::Administrator` route extracts `Extension<SupabaseToken>` and enforces the `my-headless-cms-administrator` role
- **AND** every public route (`Mount::Public`) skips the auth layer
- **AND** the same auth types are used by the legacy `legacy_bootstrap` binary without code duplication

#### Scenario: Auth validates JWT and enforces roles
- **WHEN** an authenticated client calls a protected route with a valid Bearer token
- **THEN** the auth layer validates the JWT signature against `SUPABASE_JWT_SECRET` (HS256)
- **AND** the auth layer validates the audience claim against `AUTHORIZATION_AUDIENCE`
- **AND** the auth layer validates the role claim against `required_roles` (semantics: "any role in `required_roles` is acceptable")
- **AND** the handler receives `Extension<SupabaseToken>` with `claims.email`, `claims.role`, `claims.app_metadata.roles`

#### Scenario: Auth rejects invalid tokens
- **WHEN** a client calls a protected route with a missing Authorization header
- **THEN** the auth layer returns `401 Unauthorized` with body `{"error":"Missing Authorization header"}`
- **WHEN** a client calls a protected route with an invalid Bearer token
- **THEN** the auth layer returns `401 Unauthorized` with the JWT validation error message
- **WHEN** a client calls a protected route with an expired Bearer token
- **THEN** the auth layer returns `401 Unauthorized` with the JWT expiration error message
- **WHEN** a client calls a protected route with a valid token but no matching role
- **THEN** the auth layer returns `403 Forbidden` with body `{"error":"Insufficient permissions"}`

### Requirement: Auth domain registers as a DomainService for composition-time validation
`domain-auth::DomainAuthService` SHALL implement `DomainService`. Its `validate_config` reads `SUPABASE_URL`, `SUPABASE_JWT_SECRET`, `AUTHORIZATION_AUDIENCE` from the env and returns `DomainConfigError::MissingEnv(var_name)` for each missing variable. Its `startup_health` performs a `SELECT 1` probe via `DatabaseConnection::execute_unprepared` to verify the database is reachable (the auth layer shares the connection pool with business domains). Its `migrations` returns an empty `Vec<MigrationDescriptor>` (auth has no DB schema). Its `register_routes` returns an empty `Vec<RouteRegistration>`. Its `health` returns `HealthDescriptor { name: "domain-auth", version: env!("CARGO_PKG_VERSION") }`. Its `required_env` returns `&["SUPABASE_URL", "SUPABASE_JWT_SECRET", "AUTHORIZATION_AUDIENCE"]`.

#### Scenario: Gateway composes the auth domain
- **WHEN** the gateway boots with `DomainPostService` and `DomainAuthService` in the manifest
- **THEN** the gateway validates both domains' config before binding the listener
- **AND** the gateway calls `startup_health` on both domains and aborts if either fails
- **AND** the auth layer is applied to the protected router via `domain_auth::legacy_bootstrap::construct_supabase_auth_layer`
- **AND** the public router is unchanged (no auth layer)

#### Scenario: Auth domain is deployable as a standalone microservice
`domain-auth` SHALL be runnable as a standalone binary (`cargo run -p domain-auth`) that boots its own Axum server (currently a no-op health-only stub), opens its own database connection, applies its own OpenTelemetry layers, and exposes `/health`. The standalone mode is for development and integration testing; production deployments use the auth layer through the gateway composition.

#### Scenario: Standalone auth serves the same env contract
- **WHEN** `cargo run -p domain-auth` is run with the same env-var surface as the current `my-cms-api`
- **THEN** the auth domain boots, reads `SUPABASE_URL` / `SUPABASE_JWT_SECRET` / `AUTHORIZATION_AUDIENCE` from the env, validates them, performs the `SELECT 1` startup probe, and serves `/health` with `200 OK`
- **AND** the domain's own `/health` returns `200 OK` with the auth service's health descriptor

### Requirement: Business domains consume auth through `domain_auth`, not `domain_posts`
After this change, every business domain (`domain-posts`, future `domain-media`, future `domain-users`, future `domain-administrator`) SHALL import `SupabaseToken`, `SupabaseAuthLayer`, `SupabaseClaims`, `SupabaseAuthConfig` from `domain_auth` (not from `domain_posts` or any other business domain). `domain-posts` SHALL re-export the auth types from its `domain::mod.rs` for backward compatibility with call sites that say `use crate::domain::auth::SupabaseToken` (the import path is preserved by the re-export).

#### Scenario: domain_posts does not own the auth layer
- **WHEN** `cargo tree -p domain_posts` runs
- **THEN** no `jsonwebtoken` or `tower` dependency appears in the direct dependencies of `domain-posts` (the auth layer was the only consumer of these)
- **AND** `domain_auth` appears in the dependency tree as a workspace member

#### Scenario: Future domain extractions depend only on domain_auth for auth
- **WHEN** `domain-media`, `domain-users`, or `domain-administrator` is extracted as a new crate in a future change
- **THEN** that crate's `Cargo.toml` declares `domain_auth = { path = "../domain_auth" }` as a `[dependencies]` entry
- **AND** that crate's HTTP adapters extract `Extension<SupabaseToken>` (imported from `domain_auth`)
- **AND** that crate does not depend on `domain-posts` for the auth types