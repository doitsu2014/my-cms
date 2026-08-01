## Why

The `refactor-api-into-pluggable-domain-libraries` change placed the Supabase authentication layer (`SupabaseAuthLayer`, `SupabaseAuthConfig`, `SupabaseClaims`, `SupabaseToken`) inside `domain_posts::domain::auth` and the legacy shim `cms::src/common::supabase_auth.rs`. After the upcoming `consolidate-category-ai-translate-into-domain-posts` change, every protected and administrator route across the system will live in either `domain_posts` (post CRUD, categories, AI, translation) or in the legacy `cms::api::{media,user,administrator}::*` modules — and every one of those routes extracts `SupabaseToken` via the Axum `Extension` extractor. The auth layer is **not** a post-domain concern: it is cross-cutting infrastructure consumed by every domain. Keeping it inside `domain_posts::domain::auth` forces every future domain (`domain_media`, `domain_users`, `domain_administrator`) to either depend on `domain_posts` for the auth types or duplicate the auth layer in its own crate. Pulling the auth layer out as a self-contained `domain_auth` crate lets every domain consume auth through `domain_auth` directly, lets the Supabase integration be tested in isolation, and lets `domain_auth` evolve (Supabase key rotation, JWKS refresh, audience expansion) without touching any business domain.

## What Changes

- Add a new self-contained Cargo crate **`domain_auth`** as a workspace member. It owns `SupabaseAuthLayer`, `SupabaseAuthConfig`, `SupabaseClaims`, `SupabaseToken`, and the `construct_supabase_auth_layer(audience: String, required_roles: Vec<String>)` factory function. It depends only on `domain_interface` (plus its own infrastructure dependencies — `axum`, `tower`, `jsonwebtoken`, `serde`, `tokio`) and SHALL NOT depend on any concrete business domain (`domain-posts`, `domain-categories`, `domain-ai`, `domain-translate`, `application_core`, `cms`).
- Move `apps/api/domain_posts/src/domain/auth.rs` into `apps/api/domain_auth/src/lib.rs`. Remove the file from `domain_posts::domain::auth` (the `domain` module loses the `auth` submodule).
- Move `apps/api/src/common/supabase_auth.rs` into `apps/api/domain_auth/src/legacy_shim.rs` and re-export the same types so the legacy `cms::api::*` modules continue to compile unchanged. Delete `apps/api/src/common/supabase_auth.rs`.
- Move `apps/api/src/bin/legacy_bootstrap.rs::construct_supabase_auth_layer` (lines 303–318) into `apps/api/domain_auth/src/legacy_bootstrap.rs::construct_supabase_auth_layer`. The legacy bootstrap imports the helper from `domain_auth`.
- Update every handler that currently does `use crate::domain::auth::SupabaseToken` (or `use crate::common::supabase_auth::SupabaseToken`) to do `use domain_auth::SupabaseToken`. Affected files:
  - `apps/api/domain_posts/src/api/post/{create,read,modify,delete,translate}/*`
  - `apps/api/src/api/{category,ai/models,media/{create,delete,list,read},user/{create,delete,modify,read_list,read_one,reset_password}}/*` (legacy bootstrap handlers)
  - `apps/api/src/bin/legacy_bootstrap.rs`
- Update `apps/api/domain_posts/src/domain/mod.rs` to remove `pub mod auth;`. Add a documentation note that `SupabaseToken`, `SupabaseAuthLayer`, etc. now live in `domain_auth`.
- Update `apps/api/domain_posts/Cargo.toml`: remove any auth-related dependencies (`jsonwebtoken`, `tower`, `tower-http`) that are no longer used after the `auth` submodule is removed. Keep `axum` (still needed for `State<DomainContext>` and `Extension` extractors).
- Update `apps/api/gateway/Cargo.toml` to add `domain_auth = { path = "../domain_auth" }` (the gateway's main.rs may apply the auth layer to its composed router).
- Wire the auth layer at the gateway composition root: `gateway::main.rs` constructs `SupabaseAuthLayer` via `domain_auth::construct_supabase_auth_layer("authenticated".to_string(), vec!["my-headless-cms-writer".to_string(), "my-headless-cms-administrator".to_string()])` and applies it to the protected router before serving. The legacy `legacy_bootstrap.rs` continues to apply the writer/admin/auth layers per its own construction.
- The Supabase env-var surface (`SUPABASE_URL`, `SUPABASE_INTERNAL_URL`, `SUPABASE_JWT_SECRET`, `SUPABASE_ANON_KEY`, `SUPABASE_SERVICE_ROLE_KEY`, `AUTHORIZATION_AUDIENCE`) is unchanged. The `domain_auth::domain::env::validate()` helper validates the auth-relevant subset (`SUPABASE_URL`, `SUPABASE_JWT_SECRET`, `AUTHORIZATION_AUDIENCE`) at boot.
- No new migrations, no schema changes, no GraphQL changes. Auth is HTTP-layer only.

### Auth-layer CLI surface after the change

```
cargo build -p domain_auth           # builds the auth crate
cargo test -p domain_auth            # 13 auth tests pass (valid token, missing header, wrong role, etc.)
cargo build --bin my-cms-api         # gateway builds with auth layer
cargo run -p domain_auth -- health   # standalone auth domain boots and reports health
```

The `domain_auth` crate does not expose a CLI subcommand for migration because it owns no migrations. The auth-layer boot is gated by `validate_config()` and `startup_health()` in the gateway.

## Capabilities

### New Capabilities

- **`domain-auth-service`**: Self-contained Supabase Authentication Service. Owns the JWT validation layer, the `SupabaseClaims`/`SupabaseToken` DTOs, the role-checking middleware, and the auth-layer construction factory. Provides a `DomainAuthService` impl of `DomainService` whose `register_routes` returns an empty `Vec<RouteRegistration>` (auth is HTTP-middleware, not routes), whose `migrations` returns an empty `Vec<MigrationDescriptor>`, and whose `startup_health` performs a `SELECT 1` probe to verify the JWT secret is configured (without exposing the secret). The crate depends only on `domain_interface`. Auth is composed by the gateway (and by `legacy_bootstrap`) as an Axum `Layer` applied to the protected router. Every business domain crate extracts `Extension<SupabaseToken>` and reads `claims.email`, `claims.role`, etc. without depending on `domain_auth`'s internals beyond the public types.

### Modified Capabilities

- **`domain-post-service`**: `domain_posts` no longer owns the auth layer. `SupabaseToken`, `SupabaseAuthLayer`, `SupabaseAuthConfig`, `SupabaseClaims` are imported from `domain_auth`. The post domain's HTTP adapters extract `Extension<SupabaseToken>` (unchanged behaviour) but the type lives in a different crate. The post domain's `Cargo.toml` removes the `jsonwebtoken`, `tower`, and any other auth-specific dependencies that were only there because of the `auth` submodule. The capability text in `openspec/changes/refactor-api-into-pluggable-domain-libraries/specs/domain-post-service/spec.md` and `openspec/changes/consolidate-category-ai-translate-into-domain-posts/specs/domain-post-service/spec.md` is updated to reference `domain_auth` for the auth types.

- **`api-gateway-bootstrap`**: The gateway composition root builds the auth layer from `domain_auth::construct_supabase_auth_layer(audience, required_roles)` and applies it to the protected router. The auth layer is applied as an Axum `Layer` (not as a `RouteRegistration`) because it operates on every route below the mount prefix, not on a specific path. The gateway's `Vec<Box<dyn DomainService>>` manifest grows by one entry: `Box::new(DomainAuthService::new())`. The `DomainAuthService::register_routes` returns an empty vector — the layer is applied by `gateway::main` directly.

## Impact

- Affected crates: `apps/api/Cargo.toml`, `apps/api/domain_posts/{Cargo.toml, src/{domain/{auth.rs,mod.rs},api/post/**}}`, `apps/api/src/{common/supabase_auth.rs, api/**, bin/legacy_bootstrap.rs}`, `apps/api/gateway/{Cargo.toml, src/main.rs}`, plus the new `apps/api/domain_auth/{Cargo.toml, src/{lib.rs,legacy_shim.rs,legacy_bootstrap.rs,observability.rs,service.rs}}`.
- Affected routes: none. Auth-layer wiring is HTTP-middleware; the routes served by `domain-posts`, `legacy_bootstrap`, and the gateway composition are unchanged.
- Affected env vars: none. `SUPABASE_URL`, `SUPABASE_JWT_SECRET`, `SUPABASE_ANON_KEY`, `SUPABASE_SERVICE_ROLE_KEY`, `SUPABASE_INTERNAL_URL`, `AUTHORIZATION_AUDIENCE` continue to be read by `domain_auth` (canonical) and by the legacy shim (re-export).
- Affected migrations: none. Auth has no DB schema.
- Affected GraphQL: none. The `/graphql/**` endpoints continue to use the same schemas.
- Affected tests: `domain_posts::domain::auth::tests` (13 tests covering valid token, missing header, wrong role, expired token, etc.) move into `domain_auth::tests`. The legacy `cms` shim's tests that exercise auth-protected routes continue to pass unchanged because the public API of `SupabaseAuthLayer` and `SupabaseToken` is preserved.
- Affected deployment image: the gateway binary (`my-cms-api`) and the legacy bootstrap binary (`legacy_bootstrap`) both now depend on `domain_auth`. The deployable surface is unchanged — Traefik rules, image names, and route paths are unchanged.
- Affected documentation: `docs/pluggable-domain-refactor.md` adds `domain-auth` to the workspace table and to the "Per-Domain Ownership" section. `docs/api-architecture.md` updates diagrams 1 (workspace), 5 (gateway internals), and 10 (route coverage matrix) to reflect that auth is its own crate consumed by both binaries.