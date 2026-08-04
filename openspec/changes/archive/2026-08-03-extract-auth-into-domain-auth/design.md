## Context

The `refactor-api-into-pluggable-domain-libraries` change shipped `domain-posts` as a self-contained crate and placed the Supabase authentication layer inside `domain_posts::domain::auth` (`SupabaseAuthLayer`, `SupabaseAuthConfig`, `SupabaseClaims`, `SupabaseToken`, 537 lines + 13 tests). The legacy shim `cms::src/common::supabase_auth.rs` re-exports the same types so `cms::api::*` handlers and the `legacy_bootstrap` binary can use `SupabaseToken` without depending on `domain-posts`. The auth layer was placed in `domain-posts` because that was the only consumer at the time of the refactor.

The upcoming `consolidate-category-ai-translate-into-domain-posts` change keeps auth in `domain_posts::domain::auth` (because categories/ai/translate move into `domain-posts`, where auth already lives). This is acceptable for the consolidation change because auth + categories + AI + translation are all post-related concerns. But **the auth layer is not post-related**: it is cross-cutting infrastructure consumed by every protected and administrator route across the system. After the consolidation change, every protected route in the system is in `domain-posts` (post CRUD, categories, AI, translation); every protected route in the legacy bootstrap is in `cms::api::{media,user,administrator}::*`. Once `domain-media`, `domain-users`, and `domain-administrator` are extracted in future changes, every one of those new domains will need to extract `AuthenticatedActor` from `Extension<AuthenticatedActor>` — meaning each new domain either depends on `domain_posts` for the auth types (cycle risk) or duplicates the auth layer (DRY violation). Pulling auth out as `domain-auth` before the next round of domain extractions avoids both problems.

Stakeholders:
- **Backend engineering**: needs the auth layer to be testable in isolation (the existing 13 tests in `domain_posts::domain::auth::tests` become `domain_auth::tests` and run without compiling the entire post crate). Currently `cargo test -p domain_posts` builds the post domain to exercise auth; after the change, `cargo test -p domain_auth` builds only the auth crate.
- **Security**: needs the auth-layer integration (Supabase GoTrue, JWT validation, role checking) to be reviewable in a focused crate without the noise of business-logic handlers. Pulling auth out shrinks the auth-only review surface from "everything in `domain_posts::domain::auth`" to "the entire `domain-auth` crate".
- **Future domain authors**: every new business domain crate (`domain-media`, `domain-users`, `domain-administrator`, etc.) extracts `Extension<AuthenticatedActor>` in its HTTP adapters. After this change, those domains depend only on `domain_interface` for the actor type — they never import `SupabaseToken` or `domain_auth`.
- **DevOps**: needs the auth-layer boot validation (JWT secret present, audience configured) to be observable. The new `domain_auth::domain::env::validate()` helper is called by `DomainAuthService::validate_config` and surfaces missing env vars through `DomainConfigError::MissingEnv`.

Constraints:
- `domain_auth` depends only on `domain_interface` (plus its own infrastructure dependencies — `axum`, `tower`, `jsonwebtoken`, `serde`, `tokio`, `reqwest`, `async-trait`). It SHALL NOT depend on any concrete business domain.
- The JWT-level public API (`SupabaseAuthLayer`, `SupabaseAuthConfig`, `SupabaseClaims`, `SupabaseToken`) is preserved bit-for-bit inside `domain_auth`. Every call site that compiles today continues to compile after the change (with an updated import path) — but only `domain_auth` consumes these types; everything else uses `domain_interface::AuthenticatedActor`.
- The Supabase env-var surface (`SUPABASE_URL`, `SUPABASE_INTERNAL_URL`, `SUPABASE_JWT_SECRET`, `SUPABASE_ANON_KEY`, `SUPABASE_SERVICE_ROLE_KEY`, `AUTHORIZATION_AUDIENCE`) is unchanged.
- The HTTP request flow is unchanged: every protected and administrator route continues to extract `Extension<AuthenticatedActor>` and read `actor.user_id`, `actor.email`, `actor.app_roles`. The auth layer's `Layer::call` middleware logic is unchanged except for the post-validation insertion (was `SupabaseToken { claims }`, becomes `AuthenticatedActor { ... }`).
- No new migrations. Auth has no DB schema.
- The published `domain_interface` contract stays binary-compatible for all existing types. Two purely additive changes: a new value type `AuthenticatedActor` and a default body for `DomainService::startup_health`.

## Goals / Non-Goals

**Goals:**

- Stand up `domain_auth` as a self-contained Cargo crate that owns the Supabase JWT validation layer and exposes it through `DomainService` for composition-time health and validation.
- Eliminate the current "auth is in `domain_posts::domain::auth`" coupling. After this change, every business domain imports `AuthenticatedActor` from `domain_interface`, never imports `SupabaseToken` directly, and never depends on `domain_auth` (only the gateway depends on `domain_auth` to apply the layer).
- Introduce `domain_interface::AuthenticatedActor` so every future domain (`domain-media`, `domain-users`, `domain-administrator`) is Supabase-agnostic in its handler code.
- Preserve the existing 13 auth tests (valid token, missing header, wrong role, expired token, role-list semantics, etc.) in the new crate. They pass unchanged.
- Wire `domain_auth::DomainAuthService` into the gateway's `Vec<Box<dyn DomainService>>` manifest so its `validate_config` is part of the gateway's readiness check (without forcing a DB probe — `startup_health` uses the default `Ok(())`).
- Change `DomainService::startup_health` to provide a default `Ok(())` impl so infrastructure-only domains (`domain_auth`, future auth-like infra crates) are first-class.
- Keep the existing route surface (paths, methods, auth roles, response envelopes, error mappings) bit-for-bit compatible. `cargo test --workspace` continues to pass.
- Keep the legacy `legacy_bootstrap` binary working. It continues to call `construct_supabase_auth_layer` (now from `domain_auth`) and applies the layer per its own composition pattern.
- Establish this change as the **reference migration template** that future domain extractions copy (see "Template value for future domains" at the end of this document).

**Non-Goals:**

- Replacing Supabase GoTrue with a different auth provider. The layer is still Supabase-specific.
- Adding OAuth flows, social login, multi-factor authentication, or other auth capabilities beyond what `SupabaseAuthLayer` already does. New auth capabilities belong in a follow-up change.
- Publishing `domain_auth` to crates.io. It remains path-only during the staged refactor.
- Moving `SupabaseToken` claims extraction into a per-claim method set; instead, `AuthenticatedActor` exposes `user_id`, `email`, `primary_role`, `app_roles`, and `has_any_role(&[String])` — the minimum surface every downstream handler needs.
- Replacing the JWKS-based ES256 verification (currently a fallback in `validate_supabase_token`) with a pure HS256 secret-based verification. The current implementation handles both; no change.

## Decisions

### Decision 1 — `domain_auth` is a `DomainService` impl, not just a library

`domain_auth::DomainAuthService` implements `DomainService` (`apps/api/domain_interface/src/lib.rs:131-154`). Its `register_routes` returns an empty `Vec<RouteRegistration>` because auth is HTTP-middleware, not routes. Its `migrations` returns an empty `Vec<MigrationDescriptor>` because auth has no schema. Its `startup_health` uses the default `Ok(())` impl from `DomainService` (see Decision 8 below) because auth does not probe the database. Its `health` returns `HealthDescriptor { name: "domain-auth", version: env!("CARGO_PKG_VERSION") }`. Its `required_env` returns `&["SUPABASE_URL", "SUPABASE_JWT_SECRET", "AUTHORIZATION_AUDIENCE"]`.

Why a `DomainService` impl: it gives the gateway composition root a uniform way to validate auth config at startup, and gives the gateway health aggregator a uniform way to report auth-domain readiness. The empty `Vec<RouteRegistration>` makes it clear that auth is HTTP-middleware, not a route group. The default `startup_health` makes it clear that auth is infrastructure-only, not DB-coupled.

**Rejected alternative:** `domain_auth` as a plain library crate without `DomainService`. Rejected because the gateway composition root needs `validate_config` validation at boot, and putting it inside `gateway::main` directly couples the gateway to `domain_auth` internals.

### Decision 2 — Auth layer is applied as an Axum `Layer`, not a `RouteRegistration`

The `SupabaseAuthLayer` continues to be applied to the protected and administrator routers via `Router::layer(SupabaseAuthLayer::new(config))`. It is NOT exposed through `DomainService::register_routes`. The auth layer is HTTP-middleware that operates on every request below the mount prefix; the `RouteRegistration` model is designed for path-specific routes.

The `DomainAuthService::register_routes` method returns an empty vector. The gateway's `compose_routers` (in `apps/api/gateway/src/main.rs:155-187`) is updated to apply the auth layer after merging the public/protected/administrator routers.

**Rejected alternative:** Add a new `Mount::Authenticated` variant to `domain_interface::Mount` (defined at `apps/api/domain_interface/src/lib.rs:49-58`). Rejected because the auth layer's role is to wrap protected routes, not to define new mount groups. The current `Mount::Protected` / `Mount::Administrator` semantics already encode "auth required".

### Decision 3 — Mechanical import update; no re-export shim

**Decision.** Every `use crate::domain::auth::SupabaseToken` (8 files in `apps/api/domain_posts/src/api/...`) and every `use crate::common::supabase_auth::SupabaseToken` (22 files in `apps/api/src/api/...`) is mechanically rewritten to `use domain_interface::AuthenticatedActor`. Every `Extension<SupabaseToken>` extractor becomes `Extension<AuthenticatedActor>`. There is **no** `pub use domain_auth::SupabaseToken` (or `pub use domain_auth::AuthenticatedActor`) re-export shim in `apps/api/domain_posts/src/domain/mod.rs`.

The full list of 30 handler files (plus `apps/api/src/bin/legacy_bootstrap.rs` for the type imports) is in `tasks.md` task group 6. The change is one-line-per-file for the `use` statement and one-line-per-file for the `Extension<...>` extractor.

**Rationale (template value).** The goal of this change is to be the reference migration for every future domain extraction. Future domains (`domain-media`, `domain-users`, `domain-administrator`) start with no legacy crate to shim from — they update their imports directly. Establishing the mechanical-update pattern here makes the future migrations a copy-paste exercise from `tasks.md` task group 6.

A re-export shim would contradict the architectural goal "auth lives in `domain_auth`, not in `domain_posts::domain::auth`": a reader searching for the canonical home of auth types would find both `domain_auth` (source) and `domain_posts::domain::auth` (alias), and the alias would survive indefinitely because removing it is another mechanical sweep. Worse, future domains would need to update their imports either way (they have no legacy shim), so the shim saves zero work in the long run while obscuring the type's true location during code review.

**Rejected alternative.** Keep `pub use domain_auth::{...}` in `domain_posts::domain::mod.rs` so `use crate::domain::auth::SupabaseToken` continues to compile. Rejected for the reasons above: contradicts the architectural goal, no net work savings, obscures the type's home.

### Decision 4 — `construct_supabase_auth_layer` lives in `domain_auth`

The `construct_supabase_auth_layer(audience: String, required_roles: Vec<String>) -> SupabaseAuthLayer` factory function moves from `cms::src/bin/legacy_bootstrap.rs::construct_supabase_auth_layer` (lines 287–302) into `apps/api/domain_auth::legacy_bootstrap::construct_supabase_auth_layer` (or `apps/api/domain_auth::factory::for_bootstrap`). The function reads `SUPABASE_URL` (defaulting to `SUPABASE_INTERNAL_URL` if missing), `SUPABASE_JWT_SECRET`, and constructs `SupabaseAuthLayer::new(SupabaseAuthConfig { ... })`.

The gateway also calls this function (the gateway composition root applies the layer to the protected router). The legacy bootstrap also calls this function (it applies the layer per its own composition). Both call sites import from `domain_auth`.

**Rejected alternative.** Have the gateway and the legacy bootstrap each define their own `construct_supabase_auth_layer` (duplicating the env-reading logic). Rejected because the env-reading logic is identical and centralizing it in `domain_auth` lets future changes (Supabase key rotation, JWKS caching, audience expansion) happen in one place.

### Decision 5 — `domain_auth::domain::env::validate()` centralizes env-var validation

`domain_auth::domain::env::validate() -> Result<(), String>` validates `SUPABASE_URL`, `SUPABASE_JWT_SECRET`, `AUTHORIZATION_AUDIENCE` and returns a list of missing env vars as an error message. `DomainAuthService::validate_config` calls this helper and converts the error to `DomainConfigError::MissingEnv`.

**Rejected alternative.** Have the gateway and the legacy bootstrap each define their own env-var validation. Rejected for the same reason as Decision 4: the env-var surface is auth-specific and centralizing it in `domain_auth` keeps the validation in one place.

### Decision 6 — Add `domain_interface::AuthenticatedActor` as the domain-agnostic actor contract

**Decision.** Add a new value type `domain_interface::AuthenticatedActor` to `apps/api/domain_interface/src/lib.rs`. `domain_auth::SupabaseAuthMiddleware::call` constructs an `AuthenticatedActor` from validated `SupabaseClaims` and inserts it into the request `Extension` map. Every business domain crate (current `domain_posts`, future `domain-media`, `domain-users`, `domain-administrator`) extracts `Extension<AuthenticatedActor>` instead of `Extension<SupabaseToken>`.

**Sketch (additive to `apps/api/domain_interface/src/lib.rs`).**

```rust
/// Domain-agnostic authenticated actor identity extracted from the
/// authenticated request by an auth-domain layer (e.g.
/// `domain_auth::SupabaseAuthLayer`).
///
/// Constructed by the auth layer from validated JWT claims and inserted
/// into the request `Extension` map. Every business domain reads actor
/// info via `Extension<AuthenticatedActor>` without importing Supabase
/// types or depending on `domain_auth`. The auth domain owns the
/// construction; downstream domains own only the consumption.
#[derive(Clone, Debug)]
pub struct AuthenticatedActor {
    /// Stable user identifier (from the JWT `sub` claim).
    pub user_id: String,
    /// Optional email (from the JWT `email` claim).
    pub email: Option<String>,
    /// Primary role (from the JWT `role` claim).
    pub primary_role: String,
    /// Application roles (flattened from the JWT `app_metadata.roles` claim).
    pub app_roles: Vec<String>,
}

impl AuthenticatedActor {
    /// Returns `true` when `required` is empty (no role gate) or when at
    /// least one of the actor's `app_roles` appears in `required`.
    pub fn has_any_role(&self, required: &[String]) -> bool {
        required.is_empty() || self.app_roles.iter().any(|r| required.contains(r))
    }
}
```

**Construction site (in `domain_auth::SupabaseAuthMiddleware::call`).**

```rust
let actor = AuthenticatedActor {
    user_id: claims.sub.clone(),
    email: claims.email.clone(),
    primary_role: claims.role.clone(),
    app_roles: extract_app_roles_from_claims(&claims),
};
req.extensions_mut().insert(actor);
```

Where `extract_app_roles_from_claims` is a private helper that flattens `claims.app_metadata.roles` (a JSON array of strings) into `Vec<String>`, returning `Vec::new()` when the array is absent or malformed.

**Rationale (template value).** Future domain extractions (`domain-media`, `domain-users`, `domain-administrator`) must extract actor info in their handlers. Without `AuthenticatedActor`, each future domain would `use domain_auth::SupabaseToken` — coupling every business domain to the Supabase-specific auth crate. With `AuthenticatedActor`, each future domain imports only from `domain_interface` and stays Supabase-agnostic. Auth-layer evolution (JWKS refresh, audience expansion, future auth providers like AWS Cognito or Auth0) is centralized in `domain_auth`.

**Rejected alternative.** Keep `SupabaseToken` as the public extraction type and let every domain import `domain_auth::SupabaseToken`. Rejected because (a) it couples every business domain to a Supabase-specific crate, defeating the "domains don't know about each other" goal of the pluggable-domain architecture; (b) future auth-provider changes require updating every handler, not just `domain_auth`; (c) `AuthenticatedActor` is strictly smaller than `SupabaseToken` (it omits `aud`, `exp`, `iat`, `user_metadata`, and the raw `app_metadata` blob), which is a feature: handlers can't accidentally depend on JWT internals.

### Decision 7 — `domain_auth` is path-only (not published)

`domain_auth::Cargo.toml` does not set `publish = true`. The crate remains a workspace member with path-only dependencies. Publishing `domain_auth` (and `domain_posts`, `domain_interface`) to crates.io is a separate concern tracked by the overall pluggable-domain refactor; out of scope for this change.

**Rejected alternative.** Set `publish = true` on `domain_auth`. Rejected because the staged refactor keeps all path-only dependencies until the team is ready to publish.

### Decision 8 — `DomainService::startup_health` has a default `Ok(())` impl; infrastructure-only domains use the default

**Decision.** Change `apps/api/domain_interface/src/lib.rs:151-153` from:

```rust
async fn startup_health(&self, ctx: &DomainContext) -> Result<(), DomainConfigError>;
```

to:

```rust
/// Async startup check. Domains that own database state MUST override
/// this to perform a `SELECT 1` probe (or equivalent). Infrastructure-only
/// domains (auth, observability, rate-limiting, …) MAY use the default
/// `Ok(())` implementation. The gateway calls this for every registered
/// domain after constructing the domain.
async fn startup_health(&self, _ctx: &DomainContext) -> Result<(), DomainConfigError> {
    Ok(())
}
```

`apps/api/domain_posts::service::DomainPostService::startup_health` keeps its `SELECT 1` override (existing behavior preserved). `apps/api/domain_auth::service::DomainAuthService::startup_health` uses the default `Ok(())`.

**Rationale.** `domain_auth` is infrastructure-only and does not need to probe the database. The previous design forced a `SELECT 1` probe on `domain_auth` to "share the database readiness check with the business domains", but that conflates DB readiness (which `domain_posts` covers) with auth-readiness (which the JWT secret format check in `validate_config` covers). Splitting the two responsibilities:
- makes `domain_auth` leaf-only (`cargo tree -p domain_auth | grep sea-orm` returns nothing except for the transitive `domain_interface` dep);
- prevents future auth-like infra crates (e.g. `domain-observability`, `domain-rate-limit`) from inheriting the same DB coupling;
- makes the contract semantics explicit in the doc-comment (DB-backed MUST override; infra-only MAY use the default).

The contract change is backward-compatible: existing impls (`domain_posts`) override `startup_health` and continue to work; new infra-only impls (`domain_auth`, future ones) use the default.

**Rejected alternative.** Keep `SELECT 1` probe on `domain_auth` for "consistency with `domain_posts`". Rejected because it imposes a transitive `sea-orm` dependency on `domain_auth` and conflates two unrelated readiness signals.

**Rejected alternative.** Validate JWT secret format without DB round-trip, but keep the contract signature requiring DB. Rejected because the signature implies the implementation should use the DB; the default-impl change makes the semantics explicit.

## Risks / Trade-offs

- **[Risk]** Every business domain and the legacy bootstrap must update their import statements and extractor types. → **Mitigation:** The change is mechanical and one-line-per-file. `tasks.md` task group 6 enumerates all 30 handler files plus `apps/api/src/bin/legacy_bootstrap.rs:18-25` for the legacy-shim imports. `rg "use crate::domain::auth::SupabaseToken|use crate::common::supabase_auth::SupabaseToken|use cms::common::supabase_auth"` returns zero matches after the change.
- **[Risk]** The legacy bootstrap's `construct_supabase_auth_layer` lives in `apps/api/src/bin/legacy_bootstrap.rs:287-302`. Moving it to `domain_auth` requires the legacy bootstrap to depend on `domain_auth` and import the function from there. → **Mitigation:** The import change is one line. The legacy bootstrap already depends on `cms`, which depends on `domain_posts`, which (after this change) depends on `domain_auth`. The transitive dependency is already there.
- **[Risk]** `SupabaseAuthLayer` has tests that exercise the JWT validation logic against actual tokens (encoded with `jsonwebtoken`). Moving the tests to `domain_auth::tests` requires the test fixture (the secret) to be available in the new crate. → **Mitigation:** The test fixture (`TEST_JWT_SECRET = "test-secret-key-at-least-32-characters-long!!"`) is duplicated as `domain_auth::tests::TEST_JWT_SECRET`. The tests are verbatim copies; they pass unchanged.
- **[Risk]** The `Application<Role>` and `Application<Aud>` types used by Supabase's `with_application_audience` configuration are not present in every version of `jsonwebtoken`. If `domain_posts::Cargo.toml` is using a newer version than `domain_auth::Cargo.toml`, the API mismatch could break compilation. → **Mitigation:** Both crates depend on `jsonwebtoken = { version = "9.3.1" }` (the version currently in `domain_posts::Cargo.toml:35`). The workspace `[workspace.dependencies]` block (added in this change) ensures both crates use the same version. The current implementation does not actually use `Application<Role>` or `Application<Aud>` types — `Validation::new(Algorithm::HS256)` (line 497 of `domain_posts/src/domain/auth.rs`) uses the default audience/role validation, which is sufficient.
- **[Risk]** The auth layer's `startup_health` was previously planned as a `SELECT 1` probe via `DatabaseConnection::execute_unprepared`. The new default-impl change means `domain_auth` does NOT probe the DB. If the gateway assumes `startup_health` covers DB readiness for every registered domain, removing the probe from `domain_auth` could leave a gap. → **Mitigation:** `domain_posts::DomainPostService::startup_health` keeps its `SELECT 1` override, so DB readiness is still checked by the post domain. The gateway's `for service in &services { service.startup_health(&ctx).await }` loop in `apps/api/gateway/src/main.rs:118-127` covers the post domain's DB probe. Document the split in the contract's doc-comment.
- **[Risk]** Future domain extractions (`domain-media`, `domain-users`, `domain-administrator`) need to follow the mechanical update pattern from `tasks.md` task group 6. Without strict adherence, drift will re-introduce cross-domain coupling. → **Mitigation:** The "Domain implementation checklist" added to `docs/adding-a-domain.md` (task 8.3) copy-pastes the five-phase pattern. Each future extraction change is reviewed against this checklist before merge.

## Migration Plan

### Phase 1 — Scaffold `domain_auth` and extend `domain_interface`

1. Extend `apps/api/domain_interface/src/lib.rs` with the `AuthenticatedActor` value type (additive — does not change existing API). Add a new unit test `authenticated_actor_has_any_role_returns_true_when_role_matches` in `apps/api/domain_interface/src/lib.rs:156-212`'s `tests` module.
2. Change `apps/api/domain_interface/src/lib.rs:151-153` so `DomainService::startup_health` has a default `Ok(())` impl and an updated doc-comment. The existing `_assert_object_safe` test continues to pass because the default impl preserves object-safety.
3. Add `apps/api/domain_auth/{Cargo.toml, src/{lib,observability,legacy_bootstrap,service}.rs}` as a workspace member. Depend only on `domain_interface` (plus `axum`, `tower`, `jsonwebtoken`, `serde`, `tokio`, `async-trait`, `reqwest`). Do NOT depend on `sea-orm`.
4. Move `apps/api/domain_posts/src/domain/auth.rs` (537 lines + 13 tests) into `apps/api/domain_auth/src/lib.rs`. Update `SupabaseAuthMiddleware::call` to construct a `domain_interface::AuthenticatedActor` and insert it (replacing the `SupabaseToken { claims }` insertion at the current line 154). The `SupabaseToken`/`SupabaseClaims` types stay in `domain_auth` as JWT-level DTOs.
5. Add `domain_auth::legacy_bootstrap::construct_supabase_auth_layer(audience: String, required_roles: Vec<String>) -> SupabaseAuthLayer`.
6. Add `domain_auth::domain::env::validate()`.
7. Add `domain_auth::DomainAuthService` with `health`, `required_env`, `validate_config`, `migrations` (empty), `register_routes` (empty), `startup_health` (uses the default `Ok(())` from `DomainService`).
8. Add `apps/api/domain_auth/Cargo.toml` with the workspace dependencies. Add `domain_auth` to `apps/api/Cargo.toml:2` `[workspace] members`.
9. Verify: `cargo check -p domain_auth`; `cargo test -p domain_auth` (13 JWT tests pass); `cargo tree -p domain_auth | grep -E "(domain-posts|application_core|cms|sea-orm)"` returns no result (only `domain_interface` is in the tree).

### Phase 2 — Update `domain_posts` to import auth from `domain_interface`

1. Add `domain_auth = { path = "../domain_auth" }` to `apps/api/domain_posts/Cargo.toml`.
2. Update `apps/api/domain_posts/src/domain/mod.rs:9` to remove `pub mod auth;`.
3. Delete `apps/api/domain_posts/src/domain/auth.rs`.
4. Update `apps/api/domain_posts/Cargo.toml:35` to remove `jsonwebtoken = { version = "9.3.1" }`.
5. Verify: `cargo check -p domain_posts`; `cargo test -p domain_posts`; `cargo tree -p domain_posts | grep jsonwebtoken` returns no result; `cargo tree -p domain_posts | grep domain_auth` shows the dependency.

### Phase 3 — Mechanical update of every `Extension<SupabaseToken>` extractor

Every handler that currently does `use crate::domain::auth::SupabaseToken` (8 files in `domain_posts`) or `use crate::common::supabase_auth::SupabaseToken` (22 files in `apps/api/src/api/`) is updated to `use domain_interface::AuthenticatedActor`. Every `Extension<SupabaseToken>` extractor becomes `Extension<AuthenticatedActor>`. Every `token.email()` call becomes `actor.email.as_deref().unwrap_or("")` (or equivalent, depending on the handler's existing usage). The full file list with current line numbers is in `tasks.md` task group 6.

After this phase, `rg "use crate::domain::auth::SupabaseToken|use crate::common::supabase_auth::SupabaseToken|use cms::common::supabase_auth|Extension<SupabaseToken>" apps/api` returns zero matches.

### Phase 4 — Update the legacy bootstrap

1. Add `domain_auth = { path = "../domain_auth" }` to `apps/api/Cargo.toml:73-75` (`[dependencies]` block of the root `cms` crate).
2. Update `apps/api/src/bin/legacy_bootstrap.rs:18-25` to replace `use cms::common::supabase_auth::{SupabaseAuthConfig, SupabaseAuthLayer}` with `use domain_auth::{SupabaseAuthConfig, SupabaseAuthLayer}`.
3. Update `apps/api/src/bin/legacy_bootstrap.rs:172` and `:230` (the two `.layer(construct_supabase_auth_layer(...))` call sites) to use the imported factory.
4. Update `apps/api/src/bin/legacy_bootstrap.rs:287-302` to replace the local `construct_supabase_auth_layer` definition with `use domain_auth::legacy_bootstrap::construct_supabase_auth_layer;`.
5. Update `apps/api/src/common/mod.rs` to remove `pub mod supabase_auth;`. Delete `apps/api/src/common/supabase_auth.rs`.
6. Verify: `cargo build --bin legacy_bootstrap` succeeds; `cargo test --workspace` passes; `cargo tree -p cms | grep domain_auth` shows the dependency.

### Phase 5 — Verify `DomainService` contract compliance

Add the unit and integration tests enumerated in `tasks.md` task group 5. These tests pin down the contract behavior:
- Object-safety: `let _: Box<dyn DomainService> = Box::new(DomainAuthService::new());` compiles.
- `required_env`: returns `&["SUPABASE_URL", "SUPABASE_JWT_SECRET", "AUTHORIZATION_AUDIENCE"]`.
- `migrations`: returns empty `Vec<MigrationDescriptor>`.
- `register_routes`: returns empty `Vec<RouteRegistration>`.
- `validate_config`: returns `Ok(())` when env is set; returns `Err(DomainConfigError::MissingEnv(...))` for each missing variable.
- `startup_health`: returns `Ok(())` with the default impl (no DB probe).

These tests are the **template for every future domain's contract-compliance suite**. `domain-media`, `domain-users`, and `domain-administrator` copy this task group and adapt the `required_env` / `validate_config` assertions to their own env-var surface.

### Phase 6 — Wire the auth layer at the gateway

1. Add `domain_auth = { path = "../domain_auth" }` to `apps/api/gateway/Cargo.toml`.
2. Update `apps/api/gateway/src/main.rs:30-32` to add `Box::new(DomainAuthService::new())` after `Box::new(DomainPostService::new())`.
3. Update `apps/api/gateway/src/main.rs:155-187` so `compose_routers` applies the auth layer to the protected and administrator merged routers via `domain_auth::legacy_bootstrap::construct_supabase_auth_layer(...)`.
4. Verify: `cargo build -p gateway`; `cargo run -p gateway` boots, registers 2 domain services, validates both, and applies the auth layer.

### Phase 7 — End-to-end verification

1. Run the full repository verification gate: `cargo check`, `cargo test`, `cargo fmt -- --check`, `cargo clippy --all-targets -- -D warnings`, `pnpm --dir apps/web build`.
2. Boot the gateway with a live testcontainer database. Verify:
   - `GET /health` returns 200 with both services' health descriptors
   - `GET /posts` without an Authorization header returns 401 (auth layer enforces auth)
   - `GET /posts` with a valid Bearer token returns 200 (auth layer validates JWT, extracts `AuthenticatedActor`)
   - `GET /posts` with an expired Bearer token returns 401 (auth layer rejects expired tokens)
   - `GET /posts` with a valid token but no required role returns 403 (auth layer enforces role list)
   - `POST /posts` (write-protected) requires the `my-headless-cms-writer` role
   - `GET /admin/database/migration` (admin-protected) requires the `my-headless-cms-administrator` role
3. Boot the legacy bootstrap. Verify:
   - `GET /media/**` returns 200 with a valid token
   - `GET /media/**` returns 401 without a token
   - `GET /users/**` returns 200 with the admin role
   - `GET /ai/models` returns 200 with the writer or admin role (now served by the gateway)
   - `GET /categories/**` returns 200 with the writer or admin role (now served by the gateway)
4. Verify `cargo tree -p domain_auth | grep -E "(domain-posts|application_core|cms|sea-orm)"` returns no result (only `domain_interface` is in the tree).
5. Verify `cargo tree -p domain_posts | grep domain_auth` returns one entry.
6. Verify `cargo tree -p domain_posts | grep jsonwebtoken` returns no result.
7. Run `cargo run -p domain_auth -- health` standalone. Verify the auth domain boots, reads env vars, and reports health.
8. Run `openspec verify --change "extract-auth-into-domain-auth"` and resolve every CRITICAL finding.
9. Run `openspec sync --change "extract-auth-into-domain-auth"` to publish the new `domain-auth-service` spec into `openspec/specs/`.
10. Run `openspec archive "extract-auth-into-domain-auth"` after the sync step succeeds.

### Rollback strategy

Each phase is independently revertible:
- Phase 1: revert by deleting `apps/api/domain_auth/`, removing the `domain_auth` entry from `apps/api/Cargo.toml:2`, reverting `AuthenticatedActor` addition in `apps/api/domain_interface/src/lib.rs`, and reverting the `startup_health` default-impl change.
- Phase 2: revert by restoring `apps/api/domain_posts/src/domain/auth.rs`, restoring `pub mod auth;` in `domain_posts::domain::mod.rs:9`, and restoring `jsonwebtoken` in `domain_posts::Cargo.toml:35`.
- Phase 3: revert by restoring `use crate::domain::auth::SupabaseToken` / `use crate::common::supabase_auth::SupabaseToken` and `Extension<SupabaseToken>` in every handler listed in `tasks.md` task group 6.
- Phase 4: revert by restoring `apps/api/src/common/supabase_auth.rs`, the local `construct_supabase_auth_layer` in `apps/api/src/bin/legacy_bootstrap.rs:287-302`, and the `cms::common::supabase_auth` import in `legacy_bootstrap.rs:18-25`.
- Phase 5: revert by removing the contract-compliance tests from `apps/api/domain_auth/src/service.rs`.
- Phase 6: revert by removing `Box::new(DomainAuthService::new())` from `gateway::manifest()` and removing the auth-layer application from `gateway::compose_routers`.

The database `up` history is unchanged (no migrations were added).

## Template value for future domains

This change is the **reference migration** for extracting any future business domain out of the legacy `cms` tree (or out of `domain_posts` after the consolidation change). Future authors of `extract-media-into-domain-media`, `extract-users-into-domain-users`, and `extract-administrator-into-domain-administrator` should copy the following checklist (also added to `docs/adding-a-domain.md` by task 8.3):

### Phase 1 — Scaffold the new crate

- [ ] Add `apps/api/domain_<name>/{Cargo.toml, src/{lib,service}.rs}` as a workspace member in `apps/api/Cargo.toml:2`.
- [ ] In `apps/api/domain_<name>/Cargo.toml`, depend on `domain_interface = { path = "../domain_interface" }`.
- [ ] If the new domain has auth-protected routes, also depend on `domain_auth = { path = "../domain_auth" }`.
- [ ] If the new domain reads actor info in handlers, depend on `domain_interface` only (use `AuthenticatedActor` — do NOT depend on `domain_auth`).

### Phase 2 — Move source files

- [ ] Move every source file from the legacy location (`cms::api::<name>` or `domain_posts::src/domain/<name>.rs`) into the new crate.
- [ ] Preserve existing tests verbatim. The 13 JWT-layer tests in `domain_auth` are the template for isolated test suites.

### Phase 3 — Implement `DomainService`

- [ ] Implement `DomainService` for `Domain<Name>Service`:
  - `health` returns `HealthDescriptor { name: "domain-<name>", version: env!("CARGO_PKG_VERSION") }`.
  - `required_env` returns the env vars the new domain needs.
  - `validate_config` checks env vars and returns `DomainConfigError::MissingEnv` on missing.
  - `migrations` returns the migration descriptors (empty if no schema).
  - `register_routes(&ctx)` returns the bare Axum routers.
  - `startup_health(&ctx)` uses the default `Ok(())` for infra-only domains; overrides with `SELECT 1` (or equivalent) for DB-backed domains.
- [ ] If the new domain has auth-protected routes, the legacy `Extension<SupabaseToken>` extractors in those routes become `Extension<AuthenticatedActor>` (imported from `domain_interface`, not from `domain_auth`).

### Phase 4 — Mechanical import update

- [ ] Enumerate every file that needs its `use` statement and `Extension<...>` extractor updated. See `tasks.md` task group 6 of `extract-auth-into-domain-auth` for the exhaustive file-list pattern.
- [ ] Update every file mechanically. `rg "use crate::domain::auth::SupabaseToken"` (or the equivalent legacy pattern) returns zero matches after the update.
- [ ] Update every `Extension<SupabaseToken>` extractor to `Extension<AuthenticatedActor>` and every `token.email()` call to `actor.email.as_deref().unwrap_or("")`.

### Phase 5 — Wire the new domain into the gateway

- [ ] Add `Box::new(Domain<Name>Service::new())` to `gateway::manifest()` in `apps/api/gateway/src/main.rs:30-32`.
- [ ] Update `gateway::compose_routers` if the new domain contributes routers.

### Phase 6 — Verify

- [ ] Add the contract-compliance test group (see `tasks.md` task group 5 of `extract-auth-into-domain-auth` for the template).
- [ ] Run the repository verification gate: `cargo check`, `cargo test`, `cargo fmt -- --check`, `cargo clippy --all-targets -- -D warnings`, `pnpm --dir apps/web build`.
- [ ] Boot the gateway with a live testcontainer database. Verify the new domain's routes enforce the expected auth roles.
- [ ] Verify `cargo tree -p domain_<name> | grep -E "(other_domain|cms|sea-orm)"` returns no result (only `domain_interface` and `domain_auth` are in the tree if needed).
- [ ] Run `openspec verify --change "extract-<name>-into-domain-<name>"`, `openspec sync`, then `openspec archive`.

### Phase 7 — Document

- [ ] Update `docs/pluggable-domain-refactor.md` to add the new domain to the workspace table.
- [ ] Update `docs/api-architecture.md` diagrams.
- [ ] Update `docs/adding-a-domain.md` with a domain-specific checklist derived from this template.

## Resolved Open Questions

### Recommendation (was Open Question 1) — `DomainService::startup_health` gets a default `Ok(())` impl

Resolved as **Decision 8** above. `domain_auth` uses the default `Ok(())`; `domain_posts` keeps its `SELECT 1` override. The contract gets a default body and an updated doc-comment.

### Recommendation (was Open Question 2) — Shared factory, per-binary application

Resolved by combining **Decision 4** (`construct_supabase_auth_layer` lives in `domain_auth`) and **Decision 2** (the layer is applied as an Axum `Layer`, not a `RouteRegistration`).

Construction is shared: `domain_auth::legacy_bootstrap::construct_supabase_auth_layer(audience: String, required_roles: Vec<String>) -> SupabaseAuthLayer` is called from both the gateway composition root (`apps/api/gateway/src/main.rs`) and the legacy bootstrap (`apps/api/src/bin/legacy_bootstrap.rs`).

Application is per-binary:
- **Gateway**: applies the layer to the merged `protected` and `administrator` routers once, after `compose_routers` collects `RouteRegistration`s from every domain.
- **Legacy bootstrap**: applies the layer per-router in `protected_router` (line 172) and `protected_administrator_router` (line 230).

### Recommendation (was Open Question 3) — Keep raw `jsonwebtoken` types inside `domain_auth`; the domain-agnostic boundary is `AuthenticatedActor`

Resolved by **Decision 6**. The `jsonwebtoken::Validation::new(Algorithm::HS256)` usage in `validate_supabase_token` (lines 492–537 of `apps/api/domain_posts/src/domain/auth.rs`) stays as-is. No newtype wraps `Validation` or `Application<Role>`/`Application<Aud>` — those types are not actually used by the current implementation. The domain-agnostic boundary for downstream consumers is `domain_interface::AuthenticatedActor` (Decision 6); the JWT-level types stay raw inside `domain_auth` because no other crate consumes them.

A newtype wrapping `SupabaseClaims` (`domain_auth::types::AuthClaims`) was considered and rejected: `SupabaseClaims` already exists as the JWT-level DTO, and an extra newtype adds no value when no other crate consumes it.
