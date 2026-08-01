## Context

The `refactor-api-into-pluggable-domain-libraries` change shipped `domain-posts` as a self-contained crate and placed the Supabase authentication layer inside `domain_posts::domain::auth` (`SupabaseAuthLayer`, `SupabaseAuthConfig`, `SupabaseClaims`, `SupabaseToken`, 537 lines + 13 tests). The legacy shim `cms::src/common::supabase_auth.rs` re-exports the same types so `cms::api::*` handlers and the `legacy_bootstrap` binary can use `SupabaseToken` without depending on `domain-posts`. The auth layer was placed in `domain-posts` because that was the only consumer at the time of the refactor.

The upcoming `consolidate-category-ai-translate-into-domain-posts` change keeps auth in `domain_posts::domain::auth` (because categories/ai/translate move into `domain-posts`, where auth already lives). This is acceptable for the consolidation change because auth + categories + AI + translation are all post-related concerns. But **the auth layer is not post-related**: it is cross-cutting infrastructure consumed by every protected and administrator route across the system. After the consolidation change, every protected route in the system is in `domain-posts` (post CRUD, categories, AI, translation); every protected route in the legacy bootstrap is in `cms::api::{media,user,administrator}::*`. Once `domain-media`, `domain-users`, and `domain-administrator` are extracted in future changes, every one of those new domains will need to extract `SupabaseToken` from `Extension<SupabaseToken>` — meaning each new domain either depends on `domain-posts` for the auth types (cycle risk) or duplicates the auth layer (DRY violation). Pulling auth out as `domain-auth` before the next round of domain extractions avoids both problems.

Stakeholders:
- **Backend engineering**: needs the auth layer to be testable in isolation (the existing 13 tests in `domain_posts::domain::auth::tests` become `domain_auth::tests` and run without compiling the entire post crate). Currently `cargo test -p domain_posts` builds the post domain to exercise auth; after the change, `cargo test -p domain_auth` builds only the auth crate.
- **Security**: needs the auth-layer integration (Supabase GoTrue, JWT validation, role checking) to be reviewable in a focused crate without the noise of business-logic handlers. Pulling auth out shrinks the auth-only review surface from "everything in `domain_posts::domain::auth`" to "the entire `domain-auth` crate".
- **Future domain authors**: every new business domain crate (`domain-media`, `domain-users`, `domain-administrator`, etc.) extracts `Extension<SupabaseToken>` in its HTTP adapters. After this change, those domains depend on `domain-auth` (a small, focused crate) instead of `domain-posts` (a large crate that pulls in OpenAI, pgvector, html5ever, etc.).
- **DevOps**: needs the auth-layer boot validation (JWT secret present, audience configured) to be observable. The new `domain_auth::domain::env::validate()` helper is called by `DomainAuthService::validate_config` and surfaces missing env vars through `DomainConfigError::MissingEnv`.

Constraints:
- `domain_auth` depends only on `domain_interface` (plus its own infrastructure dependencies — `axum`, `tower`, `jsonwebtoken`, `serde`, `tokio`). It SHALL NOT depend on any concrete business domain.
- The `SupabaseAuthLayer`, `SupabaseAuthConfig`, `SupabaseClaims`, `SupabaseToken` public API is preserved bit-for-bit. Every call site that compiles today continues to compile after the change (with an updated import path).
- The Supabase env-var surface (`SUPABASE_URL`, `SUPABASE_INTERNAL_URL`, `SUPABASE_JWT_SECRET`, `SUPABASE_ANON_KEY`, `SUPABASE_SERVICE_ROLE_KEY`, `AUTHORIZATION_AUDIENCE`) is unchanged.
- The HTTP request flow is unchanged: every protected and administrator route continues to extract `Extension<SupabaseToken>` and read `claims.email`, `claims.role`, etc. The auth layer's `Layer::call` middleware logic is unchanged.
- No new migrations. Auth has no DB schema.
- The published `domain-interface` contract stays unchanged. `domain_auth` does not introduce new port traits; it is an infrastructure crate, not a business domain.

## Goals / Non-Goals

**Goals:**

- Stand up `domain_auth` as a self-contained Cargo crate that owns the Supabase JWT validation layer and exposes it through `DomainService` for composition-time health and validation.
- Eliminate the current "auth is in `domain_posts::domain::auth`" coupling. After this change, every business domain imports `SupabaseToken` from `domain_auth`, not from `domain_posts`.
- Preserve the existing 13 auth tests (valid token, missing header, wrong role, expired token, role-list semantics, etc.) in the new crate. They pass unchanged.
- Wire `domain_auth::DomainAuthService` into the gateway's `Vec<Box<dyn DomainService>>` manifest so its `startup_health` (a `SELECT 1` probe to verify the database is reachable through the same connection as the business domains) is part of the gateway's readiness check.
- Keep the existing route surface (paths, methods, auth roles, response envelopes, error mappings) bit-for-bit compatible. `cargo test --workspace` continues to pass.
- Keep the legacy `legacy_bootstrap` binary working. It continues to call `construct_supabase_auth_layer` (now from `domain_auth`) and applies the layer per its own composition pattern.

**Non-Goals:**

- Replacing Supabase GoTrue with a different auth provider. The layer is still Supabase-specific.
- Adding OAuth flows, social login, multi-factor authentication, or other auth capabilities beyond what `SupabaseAuthLayer` already does. New auth capabilities belong in a follow-up change.
- Publishing `domain_auth` to crates.io. It remains path-only during the staged refactor.
- Implementing the auth layer as a port trait in `domain_interface`. The current architecture has only one auth implementation; a trait would be premature.
- Moving `SupabaseToken` claims extraction (the `actor_email` parameter on every handler) into a `domain_interface::AuthenticatedActor` value type. This is a follow-up cleanup; out of scope for this change.
- Replacing the JWKS-based ES256 verification (currently a fallback in `validate_supabase_token`) with a pure HS256 secret-based verification. The current implementation handles both; no change.

## Decisions

### Decision 1 — `domain_auth` is a `DomainService` impl, not just a library

`domain_auth::DomainAuthService` implements `DomainService`. Its `register_routes` returns an empty `Vec<RouteRegistration>` because auth is HTTP-middleware, not routes. Its `migrations` returns an empty `Vec<MigrationDescriptor>` because auth has no schema. Its `startup_health` performs a `SELECT 1` probe to verify the database connection is reachable (the auth layer doesn't depend on the DB at runtime, but the `DomainContext` is shared with business domains, and a database outage must abort gateway startup). Its `health` returns `HealthDescriptor { name: "domain-auth", version: env!("CARGO_PKG_VERSION") }`. Its `required_env` returns `&["SUPABASE_URL", "SUPABASE_JWT_SECRET", "AUTHORIZATION_AUDIENCE"]`.

Why a `DomainService` impl: it gives the gateway composition root a uniform way to validate auth config at startup, and gives the gateway health aggregator a uniform way to report auth-domain readiness. The empty `Vec<RouteRegistration>` makes it clear that auth is HTTP-middleware, not a route group.

**Rejected alternative:** `domain_auth` as a plain library crate without `DomainService`. Rejected because the gateway composition root needs `startup_health` validation, and putting it inside `gateway::main` directly couples the gateway to `domain_auth` internals.

### Decision 2 — Auth layer is applied as an Axum `Layer`, not a `RouteRegistration`

The `SupabaseAuthLayer` continues to be applied to the protected and administrator routers via `Router::layer(SupabaseAuthLayer::new(config))`. It is NOT exposed through `DomainService::register_routes`. The auth layer is HTTP-middleware that operates on every request below the mount prefix; the `RouteRegistration` model is designed for path-specific routes.

The `DomainAuthService::register_routes` method returns an empty vector. The gateway's `compose_routers` is updated to apply the auth layer after merging the public/protected/administrator routers:

```rust
let protected = compose_protected_router(&services, &ctx);
let protected = protected.layer(construct_supabase_auth_layer(
    env::var("AUTHORIZATION_AUDIENCE").unwrap_or_else(|_| "authenticated".to_string()),
    vec!["my-headless-cms-writer".to_string(), "my-headless-cms-administrator".to_string()],
));
```

The legacy `legacy_bootstrap` binary continues to apply the auth layer per its own composition (lines 188–205 in the prior change).

**Rejected alternative:** Add a new `Mount::Authenticated` variant to `domain_interface::Mount`. Rejected because the auth layer's role is to wrap protected routes, not to define new mount groups. The current `Mount::Protected` / `Mount::Administrator` semantics already encode "auth required".

### Decision 3 — `SupabaseToken`, `SupabaseClaims`, `SupabaseAuthConfig` are re-exported from `domain_posts` during the transition

The `domain_posts::Cargo.toml` adds `domain_auth = { path = "../domain_auth" }` as a dependency. `domain_posts::api::post::*` HTTP adapters continue to do `use crate::domain::auth::SupabaseToken` (the import path is preserved) by having `domain_posts::domain::mod.rs` re-export `domain_auth::SupabaseToken`. This way, the call sites that previously said `use crate::domain::auth::SupabaseToken` continue to work without modification.

```rust
// domain_posts/src/domain/mod.rs
pub use domain_auth::{
    SupabaseAuthConfig, SupabaseAuthLayer, SupabaseClaims, SupabaseToken,
};
```

After the consolidation change (which removes `domain_posts → application_core`), this re-export becomes the only auth path. Future domain extractions (`domain-media`, `domain-users`, `domain-administrator`) directly depend on `domain_auth` instead of `domain_posts` for the auth types.

**Rejected alternative:** Update every `use crate::domain::auth::SupabaseToken` to `use domain_auth::SupabaseToken` in this change. Rejected because the larger surface area (every `domain_posts::api::post::*` handler, every `cms::api::*` handler) is updated mechanically and creates churn unrelated to the architectural goal.

### Decision 4 — `construct_supabase_auth_layer` lives in `domain_auth`

The `construct_supabase_auth_layer(audience: String, required_roles: Vec<String>) -> SupabaseAuthLayer` factory function moves from `cms::src/bin/legacy_bootstrap.rs` (lines 303–318) into `domain_auth::legacy_bootstrap::construct_supabase_auth_layer` (or `domain_auth::factory::for_bootstrap`). The function reads `SUPABASE_URL` (defaulting to `SUPABASE_INTERNAL_URL` if missing), `SUPABASE_JWT_SECRET`, and constructs `SupabaseAuthLayer::new(SupabaseAuthConfig { ... })`.

The gateway also calls this function (the gateway composition root applies the layer to the protected router). The legacy bootstrap also calls this function (it applies the layer per its own composition). Both call sites import from `domain_auth`.

**Rejected alternative:** Have the gateway and the legacy bootstrap each define their own `construct_supabase_auth_layer` (duplicating the env-reading logic). Rejected because the env-reading logic is identical and centralizing it in `domain_auth` lets future changes (Supabase key rotation, JWKS caching, audience expansion) happen in one place.

### Decision 5 — `domain_auth::domain::env::validate()` centralizes env-var validation

`domain_auth::domain::env::validate() -> Result<(), String>` validates `SUPABASE_URL`, `SUPABASE_JWT_SECRET`, `AUTHORIZATION_AUDIENCE` and returns a list of missing env vars as an error message. `DomainAuthService::validate_config` calls this helper and converts the error to `DomainConfigError::MissingEnv`.

**Rejected alternative:** Have the gateway and the legacy bootstrap each define their own env-var validation. Rejected for the same reason as Decision 4: the env-var surface is auth-specific and centralizing it in `domain_auth` keeps the validation in one place.

### Decision 6 — No port traits in `domain_interface` for auth

`domain_interface::ports` stays unchanged. Auth is consumed by every business domain through `use domain_auth::SupabaseToken` (a direct type import), not through a trait object. The auth layer is applied as an Axum `Layer` at composition time (gateway / legacy bootstrap), and the auth-layer logic itself is in `SupabaseAuthLayer::call` (Tower middleware). No port trait is needed.

**Rejected alternative:** Add `AuthProvider` and `AuthenticatedActor` port traits to `domain_interface::ports`. Rejected because the only auth implementation is Supabase, and the auth layer is HTTP-middleware — not a domain-port-style collaboration. The trait abstraction has no current consumer; the current Supabase implementation is the only one.

### Decision 7 — `domain_auth` is path-only (not published)

`domain_auth::Cargo.toml` does not set `publish = true`. The crate remains a workspace member with path-only dependencies. Publishing `domain_auth` (and `domain_posts`, `domain_interface`) to crates.io is a separate concern tracked by the overall pluggable-domain refactor; out of scope for this change.

**Rejected alternative:** Set `publish = true` on `domain_auth`. Rejected because the staged refactor keeps all path-only dependencies until the team is ready to publish.

## Risks / Trade-offs

- **[Risk]** Every business domain and the legacy bootstrap add `domain_auth = { path = "../domain_auth" }` to their `Cargo.toml`. This is a small but pervasive dependency. → **Mitigation:** `domain_auth` is small (~600 lines + 600 lines of tests) and compiles quickly. The dependency is justified by the architecture: every HTTP handler that touches a protected or administrator route extracts `SupabaseToken`.
- **[Risk]** The legacy bootstrap's `construct_supabase_auth_layer` lives in `apps/api/src/bin/legacy_bootstrap.rs` (lines 303–318). Moving it to `domain_auth` requires the legacy bootstrap to depend on `domain_auth` and import the function from there. → **Mitigation:** The import change is one line. The legacy bootstrap already depends on `cms`, which depends on `domain_posts`, which (after this change) depends on `domain_auth`. The transitive dependency is already there.
- **[Risk]** `SupabaseAuthLayer` has tests that exercise the JWT validation logic against actual tokens (encoded with `jsonwebtoken`). Moving the tests to `domain_auth::tests` requires the test fixture (the secret) to be available in the new crate. → **Mitigation:** The test fixture (`TEST_JWT_SECRET = "test-secret-key-at-least-32-characters-long!!"`) is duplicated as `domain_auth::tests::TEST_JWT_SECRET`. The tests are verbatim copies; they pass unchanged.
- **[Risk]** The `Application<Role>` and `Application<Aud>` types used by Supabase's `with_application_audience` configuration are not present in every version of `jsonwebtoken`. If `domain_posts::Cargo.toml` is using a newer version than `domain_auth::Cargo.toml`, the API mismatch could break compilation. → **Mitigation:** Both crates depend on `jsonwebtoken = { version = "9.3.1" }` (the version currently in `domain_posts::Cargo.toml`). The workspace `[workspace.dependencies]` block (added in this change) ensures both crates use the same version.
- **[Risk]** The auth layer's `startup_health` performs a `SELECT 1` probe via `DatabaseConnection::execute_unprepared`. This means `domain_auth` transitively depends on `sea-orm`. If `domain_auth` is meant to be a pure infrastructure crate, it should not pull in `sea-orm`. → **Mitigation:** The `DomainService` contract requires `startup_health` to perform an async startup check (the prior change's design Decision 4). The `SELECT 1` probe is the canonical implementation. If we ever want `domain_auth` to be infrastructure-only (no DB), we can change the contract to allow `startup_health` to be a no-op for infrastructure-only services. Track as a follow-up.
- **[Risk]** The `domain_posts::domain::mod.rs` re-export `pub use domain_auth::{...}` adds a transitive dependency from `domain_posts` to `domain_auth`. After the consolidation change, `domain_posts` already depends on `application_core` (transitional); after the auth extraction, it additionally depends on `domain_auth`. This is fine — `domain_auth` has no transitive dependencies on `domain_posts`, so no cycle. → **Mitigation:** Verify with `cargo tree -p domain_posts | grep domain_auth` shows the dependency; `cargo tree -p domain_auth | grep domain_posts` returns no result. Both are part of the post-implementation verification gate.
- **[Risk]** Future domain extractions (`domain-media`, `domain-users`, `domain-administrator`) need to update their `Cargo.toml` to depend on `domain_auth`. Without this dependency, they cannot import `SupabaseToken`. → **Mitigation:** The future extraction changes (each tracked as its own OpenSpec change) include `domain_auth = { path = "../domain_auth" }` as a `[dependencies]` entry. Document this in the post-extraction recipe.

## Migration Plan

### Phase 1 — Scaffold `domain_auth`

1. Add `apps/api/domain_auth/{Cargo.toml, src/{lib,observability,legacy_bootstrap,service}.rs}` as a workspace member. Depend only on `domain_interface` (plus `axum`, `tower`, `jsonwebtoken`, `serde`, `tokio`, `async-trait`).
2. Move `apps/api/domain_posts/src/domain/auth.rs` (537 lines + 13 tests) into `apps/api/domain_auth/src/lib.rs`. The `SupabaseAuthLayer`, `SupabaseAuthConfig`, `SupabaseClaims`, `SupabaseToken`, `Layer` impl, and `validate_supabase_token` function move verbatim.
3. Add `domain_auth::legacy_bootstrap::construct_supabase_auth_layer(audience: String, required_roles: Vec<String>) -> SupabaseAuthLayer` (the function currently in `cms::src/bin/legacy_bootstrap.rs` lines 303–318).
4. Add `domain_auth::domain::env::validate()` (validates `SUPABASE_URL`, `SUPABASE_JWT_SECRET`, `AUTHORIZATION_AUDIENCE`).
5. Add `domain_auth::DomainAuthService` with `health`, `required_env`, `validate_config`, `migrations` (empty), `register_routes` (empty), `startup_health` (performs `SELECT 1` probe).
6. Verify: `cargo check -p domain_auth`; `cargo test -p domain_auth` (13 tests pass); `cargo metadata -p domain_auth` shows no concrete business domain dependency.

### Phase 2 — Update `domain_posts` to import auth from `domain_auth`

1. Add `domain_auth = { path = "../domain_auth" }` to `apps/api/domain_posts/Cargo.toml`.
2. Update `apps/api/domain_posts/src/domain/mod.rs` to remove `pub mod auth;` and add `pub use domain_auth::{SupabaseAuthConfig, SupabaseAuthLayer, SupabaseClaims, SupabaseToken};`.
3. Delete `apps/api/domain_posts/src/domain/auth.rs` (now in `domain_auth`).
4. Verify: `cargo check -p domain_posts`; `cargo test -p domain_posts`; `cargo tree -p domain_posts | grep domain_auth` shows the dependency.

### Phase 3 — Update the legacy bootstrap

1. Add `domain_auth = { path = "../domain_auth" }` to `apps/api/Cargo.toml` (root) and to `apps/api/src/lib.rs` (or wherever `cms::lib.rs` lives).
2. Update `apps/api/src/bin/legacy_bootstrap.rs`:
   - Replace the local `construct_supabase_auth_layer` (lines 303–318) with `use domain_auth::legacy_bootstrap::construct_supabase_auth_layer;`
   - Replace the auth-layer construction call sites with the imported function.
3. Update `apps/api/src/common/mod.rs` to remove `pub mod supabase_auth;`. Delete `apps/api/src/common/supabase_auth.rs`.
4. Verify: `cargo build --bin legacy_bootstrap` succeeds; `cargo test --workspace` passes.

### Phase 4 — Wire the auth layer at the gateway

1. Add `domain_auth = { path = "../domain_auth" }` to `apps/api/gateway/Cargo.toml`.
2. Update `apps/api/gateway/src/main.rs` to construct `DomainAuthService`, add it to the manifest, and apply the auth layer to the protected router:

   ```rust
   pub fn manifest() -> Vec<Box<dyn DomainService>> {
       vec![
           Box::new(DomainPostService::new()),
           Box::new(DomainAuthService::new()),
       ]
   }
   ```

   And in `compose_routers`:
   ```rust
   let protected = compose_protected_router(...);
   let protected = protected.layer(domain_auth::legacy_bootstrap::construct_supabase_auth_layer(
       env::var("AUTHORIZATION_AUDIENCE").unwrap_or_else(|_| "authenticated".to_string()),
       vec!["my-headless-cms-writer".to_string(), "my-headless-cms-administrator".to_string()],
   ));
   ```

3. Verify: `cargo build -p gateway`; `cargo run -p gateway` boots and the auth layer is applied to the protected router.

### Phase 5 — End-to-end verification

1. Run the full repository verification gate: `cargo check`, `cargo test`, `cargo fmt --check`, `cargo clippy --all-targets -- -D warnings`, `pnpm --dir apps/web build`.
2. Boot the gateway with a live testcontainer database. Verify:
   - `GET /health` returns 200 with the post service's health descriptor (the auth service contributes `name: "domain-auth"` to the manifest but does not register routes)
   - `GET /posts` without an Authorization header returns 401 (auth layer enforces auth)
   - `GET /posts` with a valid Bearer token returns 200 (auth layer validates JWT, extracts `SupabaseToken`)
   - `GET /posts` with an expired Bearer token returns 401 (auth layer rejects expired tokens)
   - `GET /posts` with a valid token but no required role returns 403 (auth layer enforces role list)
   - `POST /posts` (write-protected) requires the `my-headless-cms-writer` role
   - `GET /admin/database/migration` (admin-protected) requires the `my-headless-cms-administrator` role
3. Boot the legacy bootstrap. Verify:
   - `GET /media/**` returns 200 with a valid token
   - `GET /media/**` returns 401 without a token
   - `GET /users/**` returns 200 with the admin role
   - `GET /ai/models` returns 200 with the writer or admin role (now served by the gateway, not the legacy bootstrap)
   - `GET /categories/**` returns 200 with the writer or admin role (now served by the gateway)
4. Verify `cargo tree -p domain_auth | grep -E "(domain-posts|application_core|cms)"` returns no result. `domain_auth` is a leaf crate.
5. Verify `cargo tree -p domain_posts | grep domain_auth` returns one entry (the `domain_auth` path dependency).
6. Run `openspec verify --change "extract-auth-into-domain-auth"` and resolve every CRITICAL finding.
7. Run `openspec sync --change "extract-auth-into-domain-auth"` to publish the new `domain-auth-service` spec into `openspec/specs/`.
8. Run `openspec archive "extract-auth-into-domain-auth"` after the sync step succeeds.

### Rollback strategy

Each phase is independently revertible:
- Phase 1: revert by deleting `apps/api/domain_auth/` and removing the `domain_auth` entry from `apps/api/Cargo.toml`.
- Phase 2: revert by restoring `apps/api/domain_posts/src/domain/auth.rs` and removing the `domain_auth` re-export from `domain_posts::domain::mod.rs`.
- Phase 3: revert by restoring `apps/api/src/common/supabase_auth.rs` and the local `construct_supabase_auth_layer` in `apps/api/src/bin/legacy_bootstrap.rs`.
- Phase 4: revert by removing the `DomainAuthService` entry from `gateway::manifest()` and the auth-layer application from `gateway::main`.

The database `up` history is unchanged (no migrations were added).

## Open Questions

- **Should `domain_auth::DomainAuthService::startup_health` perform a `SELECT 1` probe via the shared `DatabaseConnection`, or should it validate the JWT secret format (length, encoding) without a DB round-trip?** The current design uses `SELECT 1` to share the database readiness check with the business domains. The alternative is to validate the JWT secret format only. The first is consistent with the existing `DomainService` pattern; the second is faster but requires a contract change. Recommend `SELECT 1` for now (consistent with `domain_posts::DomainPostService::startup_health`). Confirm before implementation.
- **Should the gateway and the legacy bootstrap share the auth-layer construction logic via `domain_auth::legacy_bootstrap::construct_supabase_auth_layer`, or should each define its own construction pattern (gateway: applied to the merged router via `Router::layer`; legacy: applied per-router in `protected_router` and `protected_administrator_router`)?** The shared factory function (Decision 4) handles construction. The application pattern (when to apply the layer) is different per binary. Recommend keeping the factory shared but the application per-binary. Confirm before implementation.
- **Should the `Application<Role>` and `Application<Aud>` types from `jsonwebtoken` be wrapped in a `domain_auth::types::AuthClaims` newtype, or kept as the raw `jsonwebtoken` types?** The raw types are simpler; the newtype adds an adapter layer. Recommend raw types for now (no abstraction needed). Confirm before implementation.