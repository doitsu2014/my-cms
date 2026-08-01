## 1. Scaffold `domain_auth` crate

- [ ] 1.1 Add `apps/api/domain_auth/{Cargo.toml, src/{lib,observability,service}.rs}` as a workspace member. Mirror `domain_posts` layout: `lib.rs` (public re-exports), `Cargo.toml` (dependencies), `src/service.rs` (impl `DomainService` for `DomainAuthService`). Depend only on `domain_interface` (plus `axum`, `tower`, `jsonwebtoken`, `serde`, `tokio`, `async-trait`).
- [ ] 1.2 Move `apps/api/domain_posts/src/domain/auth.rs` (537 lines + 13 tests) into `apps/api/domain_auth/src/lib.rs`. Adjust the docstring header to note "moved from domain_posts/src/domain/auth.rs; canonical Supabase auth layer for the workspace". The `SupabaseAuthLayer`, `SupabaseAuthConfig`, `SupabaseClaims`, `SupabaseToken`, `Layer` impl, and `validate_supabase_token` function move verbatim. The 13 tests in `mod tests` continue to pass.
- [ ] 1.3 Add `domain_auth::legacy_bootstrap::construct_supabase_auth_layer(audience: String, required_roles: Vec<String>) -> SupabaseAuthLayer` (the function currently in `apps/api/src/bin/legacy_bootstrap.rs` lines 303–318). The function reads `SUPABASE_URL` (defaulting to `SUPABASE_INTERNAL_URL`), `SUPABASE_JWT_SECRET`, and constructs `SupabaseAuthConfig { supabase_url, jwt_secret, expected_audience, required_roles }`.
- [ ] 1.4 Add `domain_auth::domain::env::validate() -> Result<(), String>` (validates `SUPABASE_URL`, `SUPABASE_JWT_SECRET`, `AUTHORIZATION_AUDIENCE`). Returns an error message listing missing env vars.
- [ ] 1.5 Add `domain_auth::DomainAuthService` with `health`, `required_env`, `validate_config`, `migrations` (empty), `register_routes` (empty), `startup_health`. `startup_health` performs `DatabaseConnection::execute_unprepared("SELECT 1")` to verify the shared connection is reachable.
- [ ] 1.6 Add `apps/api/domain_auth/Cargo.toml` with `workspace.dependencies` resolving `axum = "0.8.9"`, `jsonwebtoken = "9.3.1"`, `tower = "0.5.3"`, `serde = { version = "1.0.228", features = ["derive"] }`, `serde_json = "1.0.150"`, `tokio = { version = "1.52.3", features = ["full"] }`, `async-trait = "0.1"`, `domain_interface = { path = "../domain_interface" }`, `sea-orm = { version = "1.1.20" }`.
- [ ] 1.7 Add `domain_auth` to `apps/api/Cargo.toml` `[workspace] members`.
- [ ] 1.8 Verify: `cargo check -p domain_auth`; `cargo test -p domain_auth` (13 tests pass); `cargo tree -p domain_auth | grep -E "(domain-posts|application_core|cms)"` returns no result.

## 2. Update `domain_posts` to import auth from `domain_auth`

- [ ] 2.1 Add `domain_auth = { path = "../domain_auth" }` to `apps/api/domain_posts/Cargo.toml`.
- [ ] 2.2 Update `apps/api/domain_posts/src/domain/mod.rs`:
  - Remove `pub mod auth;`
  - Add `pub use domain_auth::{SupabaseAuthConfig, SupabaseAuthLayer, SupabaseClaims, SupabaseToken};`
- [ ] 2.3 Delete `apps/api/domain_posts/src/domain/auth.rs`.
- [ ] 2.4 Update `apps/api/domain_posts/Cargo.toml` to remove dependencies that were only there because of the auth submodule: `jsonwebtoken`, `tower-http` (keep `axum` and `tower` only if still used elsewhere in `domain_posts`).
- [ ] 2.5 Verify: `cargo check -p domain_posts`; `cargo test -p domain_posts`; `cargo tree -p domain_posts | grep domain_auth` shows the dependency; `cargo tree -p domain_posts | grep jsonwebtoken` returns no result.

## 3. Update the legacy bootstrap

- [ ] 3.1 Add `domain_auth = { path = "../domain_auth" }` to `apps/api/Cargo.toml` (root `[dependencies]`).
- [ ] 3.2 Update `apps/api/src/bin/legacy_bootstrap.rs`:
  - Replace the local `construct_supabase_auth_layer` function (lines 303–318) with `use domain_auth::legacy_bootstrap::construct_supabase_auth_layer;`
  - Update the two call sites (`protected_router` line 188 and `protected_administrator_router` line 246) to use the imported function.
- [ ] 3.3 Update `apps/api/src/common/mod.rs` to remove `pub mod supabase_auth;`. Delete `apps/api/src/common/supabase_auth.rs`.
- [ ] 3.4 Verify: `cargo build --bin legacy_bootstrap` succeeds; `cargo test --workspace` passes; `cargo tree -p cms | grep domain_auth` shows the dependency.

## 4. Wire the auth layer at the gateway

- [ ] 4.1 Add `domain_auth = { path = "../domain_auth" }` to `apps/api/gateway/Cargo.toml`.
- [ ] 4.2 Update `apps/api/gateway/src/main.rs`:
  - Add `Box::new(DomainAuthService::new())` to `manifest()` after `Box::new(DomainPostService::new())`.
  - In `compose_routers`, after merging the public/protected/administrator sub-routers, apply the auth layer to the protected and administrator sub-routers via `Router::layer(domain_auth::legacy_bootstrap::construct_supabase_auth_layer(...))`.
  - In `main`, after constructing the `DomainContext`, call `service.validate_config()` and `service.startup_health(&ctx)` for every registered service (the auth service's `validate_config` checks env vars; its `startup_health` performs `SELECT 1`).
- [ ] 4.3 Verify: `cargo build -p gateway`; `cargo run -p gateway` boots, registers 2 domain services, validates both, and applies the auth layer to the protected router.

## 5. End-to-end verification

- [ ] 5.1 Run the full repository verification gate: `cargo check`, `cargo test`, `cargo fmt --check`, `cargo clippy --all-targets -- -D warnings`, `pnpm --dir apps/web build`.
- [ ] 5.2 Boot the gateway with a live testcontainer database. Verify:
  - `GET /health` returns 200 with the post service's health descriptor
  - `GET /posts` without an Authorization header returns 401
  - `GET /posts` with a valid Bearer token returns 200
  - `GET /posts` with an expired Bearer token returns 401
  - `GET /posts` with a valid token but no required role returns 403
  - `POST /posts` requires the `my-headless-cms-writer` role
  - `GET /admin/database/migration` requires the `my-headless-cms-administrator` role
- [ ] 5.3 Boot the legacy bootstrap. Verify:
  - `GET /media/**` returns 200 with a valid token
  - `GET /media/**` returns 401 without a token
  - `GET /users/**` returns 200 with the admin role
  - `GET /ai/models` (served by gateway via `domain_posts`) returns 200 with the writer or admin role
  - `GET /categories/**` (served by gateway via `domain_posts`) returns 200 with the writer or admin role
- [ ] 5.4 Verify `cargo tree -p domain_auth | grep -E "(domain-posts|application_core|cms)"` returns no result. `domain_auth` is a leaf crate.
- [ ] 5.5 Verify `cargo tree -p domain_posts | grep domain_auth` returns one entry (the `domain_auth` path dependency).
- [ ] 5.6 Run `cargo run -p domain_auth -- health` standalone. Verify the auth domain boots, reads env vars, and reports health.
- [ ] 5.7 Run `openspec verify --change "extract-auth-into-domain-auth"` and resolve every CRITICAL finding.
- [ ] 5.8 Run `openspec sync --change "extract-auth-into-domain-auth"` to publish the new `domain-auth-service` spec into `openspec/specs/`.
- [ ] 5.9 Run `openspec archive "extract-auth-into-domain-auth"` after the sync step succeeds.

## 6. Documentation

- [ ] 6.1 Update `docs/pluggable-domain-refactor.md` to add `domain-auth` to the workspace table and to the "Per-Domain Ownership" section.
- [ ] 6.2 Update `docs/api-architecture.md` to draw `domain-auth` in diagrams 1 (workspace), 2 (two-binary deployment), and 7 (request flow — the auth layer sits between the gateway listener and the route handlers).
- [ ] 6.3 Update `docs/adding-a-domain.md` to note that every new domain must depend on `domain_auth` for the auth types (`SupabaseToken`, `SupabaseAuthLayer`, etc.).
- [ ] 6.4 Verify: docs are coherent and reference the new `domain_auth` crate correctly.