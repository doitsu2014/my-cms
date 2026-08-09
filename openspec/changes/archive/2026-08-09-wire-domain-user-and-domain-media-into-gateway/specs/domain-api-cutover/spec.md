## MODIFIED Requirements

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

## ADDED Requirements

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

#### Scenario: domain_user::api::routes wraps the seven handler modules
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
