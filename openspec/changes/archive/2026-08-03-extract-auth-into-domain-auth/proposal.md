## Why

The `refactor-api-into-pluggable-domain-libraries` change placed the Supabase authentication layer (`SupabaseAuthLayer`, `SupabaseAuthConfig`, `SupabaseClaims`, `SupabaseToken`) inside `domain_posts::domain::auth` and the legacy shim `cms::src/common::supabase_auth.rs`. After the upcoming `consolidate-category-ai-translate-into-domain-posts` change, every protected and administrator route across the system will live in either `domain_posts` (post CRUD, categories, AI, translation) or in the legacy `cms::api::{media,user,administrator}::*` modules — and every one of those routes extracts `SupabaseToken` via the Axum `Extension` extractor. The auth layer is **not** a post-domain concern: it is cross-cutting infrastructure consumed by every domain. Keeping it inside `domain_posts::domain::auth` forces every future domain (`domain_media`, `domain_users`, `domain_administrator`) to either depend on `domain_posts` for the auth types or duplicate the auth layer in its own crate. Pulling the auth layer out as a self-contained `domain_auth` crate lets every domain consume auth through `domain_auth` directly, lets the Supabase integration be tested in isolation, and lets `domain_auth` evolve (Supabase key rotation, JWKS refresh, audience expansion) without touching any business domain.

## What Changes

- **Decision 3 (resolved) — Mechanical import update.** Every `use crate::domain::auth::SupabaseToken` (8 files in `apps/api/domain_posts/src/api/...`) and every `use crate::common::supabase_auth::SupabaseToken` (22 files in `apps/api/src/api/...`) is mechanically rewritten to `use domain_interface::AuthenticatedActor`. Every `Extension<SupabaseToken>` extractor becomes `Extension<AuthenticatedActor>`. There is **no** `pub use domain_auth::SupabaseToken` re-export shim in `apps/api/domain_posts/src/domain/mod.rs`. The full file list is in `tasks.md` task group 6.

- **Decision 6 (resolved) — Add `domain_interface::AuthenticatedActor`.** A new domain-agnostic value type `AuthenticatedActor` is added to `apps/api/domain_interface/src/lib.rs`. `domain_auth::SupabaseAuthLayer::call` constructs an `AuthenticatedActor` from validated `SupabaseClaims` and inserts it into the request `Extension` map. Every business domain crate (current `domain_posts`, future `domain-media`, `domain-users`, `domain-administrator`) extracts `Extension<AuthenticatedActor>` without depending on Supabase types or on `domain_auth`. `SupabaseToken` and `SupabaseClaims` remain in `domain_auth` as JWT-level DTOs; `AuthenticatedActor` is the domain-level contract.

- **Open Question 1 (resolved) — `DomainService::startup_health` gets a default `Ok(())` impl.** The contract trait in `apps/api/domain_interface/src/lib.rs:151-153` is updated so `startup_health` has a default `async fn startup_health(&self, _ctx: &DomainContext) -> Result<(), DomainConfigError> { Ok(()) }` body. `domain_posts::DomainPostService` keeps its `SELECT 1` override. `domain_auth::DomainAuthService` uses the default — auth is infrastructure-only and never probes the database. This removes the transitive `sea-orm` dependency from `domain_auth` and decouples DB-readiness from auth-readiness.

- Add a new self-contained Cargo crate **`domain_auth`** as a workspace member. It owns `SupabaseAuthLayer`, `SupabaseAuthConfig`, `SupabaseClaims`, `SupabaseToken`, the `SupabaseAuthMiddleware<S>` (which constructs and inserts `AuthenticatedActor`), and the `construct_supabase_auth_layer(audience: String, required_roles: Vec<String>) -> SupabaseAuthLayer` factory. It depends only on `domain_interface` (plus its own infrastructure dependencies — `axum`, `tower`, `jsonwebtoken`, `serde`, `tokio`, `reqwest`, `async-trait`) and SHALL NOT depend on any concrete business domain (`domain-posts`, `application_core`, `cms`).

- Move `apps/api/domain_posts/src/domain/auth.rs` (537 lines + 13 tests) into `apps/api/domain_auth/src/lib.rs`. Update `SupabaseAuthMiddleware::call` to construct a `domain_interface::AuthenticatedActor` from validated `SupabaseClaims` and insert it (replacing the `SupabaseToken` insertion at the current line 154). Remove `pub mod auth;` from `apps/api/domain_posts/src/domain/mod.rs`.

- Delete `apps/api/src/common/supabase_auth.rs`. The legacy `cms::api::*` handlers no longer import from `crate::common::supabase_auth`; they import `use domain_interface::AuthenticatedActor`.

- Move `apps/api/src/bin/legacy_bootstrap.rs::construct_supabase_auth_layer` (lines 287–302) into `apps/api/domain_auth::legacy_bootstrap::construct_supabase_auth_layer`. Both the legacy bootstrap and the gateway import the factory from `domain_auth`.

- Update `apps/api/domain_posts/Cargo.toml`: remove `jsonwebtoken` (no longer needed; auth moved out). Keep `axum`, `tower`, etc.

- Update `apps/api/gateway/Cargo.toml` to add `domain_auth = { path = "../domain_auth" }`. The gateway composition root constructs `SupabaseAuthLayer` via `domain_auth::legacy_bootstrap::construct_supabase_auth_layer(...)` and applies it to the protected and administrator merged routers via `Router::layer(...)`.

- Wire `domain_auth::DomainAuthService` into the gateway's `Vec<Box<dyn DomainService>>` manifest. The gateway calls `service.validate_config()` (checks `SUPABASE_URL`, `SUPABASE_JWT_SECRET`, `AUTHORIZATION_AUDIENCE`) and `service.startup_health(&ctx)` (default `Ok(())`).

- The Supabase env-var surface (`SUPABASE_URL`, `SUPABASE_INTERNAL_URL`, `SUPABASE_JWT_SECRET`, `SUPABASE_ANON_KEY`, `SUPABASE_SERVICE_ROLE_KEY`, `AUTHORIZATION_AUDIENCE`) is unchanged. `domain_auth::domain::env::validate()` validates the auth-relevant subset at boot.

- No new migrations, no schema changes, no GraphQL changes. Auth is HTTP-layer only.

### Auth-layer CLI surface after the change

```
cargo build -p domain_auth           # builds the auth crate (no sea-orm dep)
cargo test -p domain_auth            # 13 auth tests pass + new contract-compliance tests
cargo build --bin my-cms-api         # gateway builds with auth layer
cargo run -p domain_auth -- health   # standalone auth domain boots and reports health
```

The `domain_auth` crate does not expose a CLI subcommand for migration because it owns no migrations. The auth-layer boot is gated by `validate_config()` (env-var check) and `startup_health()` (default `Ok(())` — DB readiness is delegated to `domain_posts::DomainPostService::startup_health`).

### Template for future domains

This change is the **reference migration** for extracting any future business domain out of the legacy `cms` tree or out of `domain_posts`. Future extractions of `domain-media`, `domain-users`, and `domain-administrator` should follow the same five-phase pattern documented in `tasks.md`:

1. **Scaffold the new crate** as a workspace member of `apps/api/Cargo.toml` with `domain_interface = { path = "../domain_interface" }` as a dependency. If the new domain has auth-protected routes, also add `domain_auth = { path = "../domain_auth" }`.
2. **Move source files** from the legacy location into the new crate, preserving the implementation and the existing tests verbatim.
3. **Mechanically update every `use` and `Extension<...>` extractor** that referenced the legacy path. If the legacy type is an auth-related DTO, replace it with `use domain_interface::AuthenticatedActor` (and update the `Extension<...>` extractor type). Enumerate every file path in the task group before editing.
4. **Implement `DomainService`** for the new domain against the `domain_interface::DomainService` contract. Add `Box::new(Domain<Name>Service::new())` to the gateway's `manifest()` function. Verify the contract-compliance tests in task group 5 pass for every trait method.
5. **Verify** with the repository verification gate (`cargo check`, `cargo test`, `cargo fmt --check`, `cargo clippy`, `pnpm --dir apps/web build`) and the new contract-compliance test group.

Each phase is independently revertible (see Migration Plan in `design.md`). The "Domain implementation checklist" section that this change adds to `docs/adding-a-domain.md` (task 8.3) is the copy-paste template.

## Capabilities

### New Capabilities

- **`domain-auth-service`**: Self-contained Supabase Authentication Service. Owns the JWT validation layer (`SupabaseAuthLayer`, `SupabaseAuthConfig`, `SupabaseClaims`, `SupabaseToken`, `SupabaseAuthMiddleware<S>`), the role-checking middleware, and the auth-layer construction factory `domain_auth::legacy_bootstrap::construct_supabase_auth_layer`. Provides a `DomainAuthService` impl of `DomainService` whose `register_routes` returns an empty `Vec<RouteRegistration>`, whose `migrations` returns an empty `Vec<MigrationDescriptor>`, whose `startup_health` returns the default `Ok(())` (infrastructure-only — does not probe the shared `DatabaseConnection`), whose `health` returns `HealthDescriptor { name: "domain-auth", version: env!("CARGO_PKG_VERSION") }`, and whose `required_env` returns `&["SUPABASE_URL", "SUPABASE_JWT_SECRET", "AUTHORIZATION_AUDIENCE"]`. The crate depends only on `domain_interface` and is leaf-only (`cargo tree -p domain_auth | grep -E "(domain-posts|application_core|cms|sea-orm)"` returns no result except for the transitive `domain_interface` dep).

- **`domain-actor-contract`** (extension of `domain-service-interface`): The `domain_interface::AuthenticatedActor` value type. Domain-agnostic identity extracted from the authenticated request by any auth-domain layer and consumed by every business domain via `Extension<AuthenticatedActor>`. Future domains (`domain-media`, `domain-users`, `domain-administrator`) do not depend on Supabase types or on `domain_auth` to read actor info. The contract is additive to the existing `domain_interface` API and does not change `DomainService`, `DomainContext`, `Mount`, `RouteRegistration`, `MigrationDescriptor`, `HealthDescriptor`, or `DomainConfigError`.

### Modified Capabilities

- **`domain-post-service`**: `domain_posts` no longer owns the auth layer. HTTP adapters extract `Extension<AuthenticatedActor>` (imported from `domain_interface`), not `Extension<SupabaseToken>`. The `Cargo.toml` removes `jsonwebtoken`. The post domain's behavior is unchanged otherwise. The capability text in `openspec/changes/refactor-api-into-pluggable-domain-libraries/specs/domain-post-service/spec.md` and `openspec/changes/consolidate-category-ai-translate-into-domain-posts/specs/domain-post-service/spec.md` is updated to reference `domain_interface::AuthenticatedActor` for the actor type.

- **`api-gateway-bootstrap`**: The gateway composition root adds `Box::new(DomainAuthService::new())` to `manifest()`. The gateway applies the auth layer to the protected and administrator merged routers via `domain_auth::legacy_bootstrap::construct_supabase_auth_layer(...)`. The auth-domain config validation runs at boot via `service.validate_config()`. The `DomainService::startup_health` contract gets a default `Ok(())` impl; `domain_posts::DomainPostService` keeps its `SELECT 1` override.

## Impact

- Affected crates: `apps/api/Cargo.toml` (workspace members + root `cms` deps), `apps/api/domain_interface/src/lib.rs` (additive `AuthenticatedActor` type + default `startup_health` impl), `apps/api/domain_posts/{Cargo.toml, src/{domain/{auth.rs,mod.rs},api/{category,post}/**}}`, `apps/api/src/{common/supabase_auth.rs (deleted), api/**, bin/legacy_bootstrap.rs}`, `apps/api/gateway/{Cargo.toml, src/main.rs}`, plus the new `apps/api/domain_auth/{Cargo.toml, src/{lib.rs,legacy_bootstrap.rs,service.rs}}`.
- Affected routes: none. Auth-layer wiring is HTTP-middleware; the routes served by `domain-posts`, `legacy_bootstrap`, and the gateway composition are unchanged.
- Affected env vars: none. `SUPABASE_URL`, `SUPABASE_JWT_SECRET`, `SUPABASE_ANON_KEY`, `SUPABASE_SERVICE_ROLE_KEY`, `SUPABASE_INTERNAL_URL`, `AUTHORIZATION_AUDIENCE` continue to be read by `domain_auth`.
- Affected migrations: none. Auth has no DB schema.
- Affected GraphQL: none. The `/graphql/**` endpoints continue to use the same schemas.
- Affected tests: `domain_posts::domain::auth::tests` (13 tests covering valid token, missing header, wrong role, expired token, etc.) move into `domain_auth::tests`. New contract-compliance tests are added (see `tasks.md` task group 5). The legacy `cms` shim's tests that exercise auth-protected routes continue to pass because the public API of `SupabaseAuthLayer` and the `Extension<AuthenticatedActor>` extractor is preserved.
- Affected deployment image: the gateway binary (`my-cms-api`) and the legacy bootstrap binary (`legacy_bootstrap`) both now depend on `domain_auth`. The deployable surface is unchanged — Traefik rules, image names, and route paths are unchanged.
- Affected documentation: `docs/pluggable-domain-refactor.md` adds `domain-auth` to the workspace table and to the "Per-Domain Ownership" section. `docs/api-architecture.md` updates diagrams 1 (workspace), 5 (gateway internals), and 10 (route coverage matrix) to reflect that auth is its own crate consumed by both binaries. `docs/adding-a-domain.md` adds a "Domain implementation checklist" section derived from this change's tasks (see task 8.3).
