## 1. Scaffold `domain_auth` crate and extend `domain_interface`

- [ ] 1.1 Add `apps/api/domain_auth/{Cargo.toml, src/{lib,observability,legacy_bootstrap,service}.rs}` as a workspace member. Mirror `domain_posts` layout: `lib.rs` (public re-exports), `Cargo.toml` (dependencies), `src/service.rs` (impl `DomainService` for `DomainAuthService`). Depend only on `domain_interface` (plus `axum`, `tower`, `jsonwebtoken`, `serde`, `tokio`, `async-trait`, `reqwest`). Do NOT depend on `sea-orm`.
- [ ] 1.2 Extend `apps/api/domain_interface/src/lib.rs` with the `AuthenticatedActor` value type (additive — does not change existing API). Add a new unit test `authenticated_actor_has_any_role_returns_true_when_role_matches` and `authenticated_actor_has_any_role_returns_false_when_no_match` in the `tests` module at `apps/api/domain_interface/src/lib.rs:156-212`.
- [ ] 1.3 Change `apps/api/domain_interface/src/lib.rs:151-153` so `DomainService::startup_health` has a default `Ok(())` impl and an updated doc-comment. The existing `_assert_object_safe` test continues to pass because the default impl preserves object-safety.
- [ ] 1.4 Move `apps/api/domain_posts/src/domain/auth.rs` (537 lines + 13 tests) into `apps/api/domain_auth/src/lib.rs`. Update `SupabaseAuthMiddleware::call` to construct a `domain_interface::AuthenticatedActor` from validated `SupabaseClaims` and insert it (replacing the `SupabaseToken { claims }` insertion at the current line 154 of the moved code). The `SupabaseToken` and `SupabaseClaims` types stay in `domain_auth` as JWT-level DTOs.
- [ ] 1.5 Add `domain_auth::legacy_bootstrap::construct_supabase_auth_layer(audience: String, required_roles: Vec<String>) -> SupabaseAuthLayer` (the function currently in `apps/api/src/bin/legacy_bootstrap.rs:287-302`). The function reads `SUPABASE_URL` (defaulting to `SUPABASE_INTERNAL_URL`), `SUPABASE_JWT_SECRET`, and constructs `SupabaseAuthConfig { supabase_url, jwt_secret, expected_audience, required_roles }`.
- [ ] 1.6 Add `domain_auth::domain::env::validate() -> Result<(), String>` (validates `SUPABASE_URL`, `SUPABASE_JWT_SECRET`, `AUTHORIZATION_AUDIENCE`). Returns an error message listing missing env vars.
- [ ] 1.7 Add `domain_auth::DomainAuthService` with `health`, `required_env`, `validate_config`, `migrations` (empty), `register_routes` (empty), `startup_health`. `startup_health` uses the default `Ok(())` from `DomainService` (no DB probe — auth is infrastructure-only).
- [ ] 1.8 Add `apps/api/domain_auth/Cargo.toml` with `workspace.dependencies` resolving `axum = "0.8.9"`, `jsonwebtoken = "9.3.1"`, `tower = "0.5.3"`, `serde = { version = "1.0.228", features = ["derive"] }`, `serde_json = "1.0.150"`, `tokio = { version = "1.52.3", features = ["full"] }`, `async-trait = "0.1"`, `domain_interface = { path = "../domain_interface" }`, `reqwest = { version = "0.12.28" }`. No `sea-orm` dependency.
- [ ] 1.9 Add `domain_auth` to `apps/api/Cargo.toml:2` `[workspace] members`.
- [ ] 1.10 Verify: `cargo check -p domain_auth`; `cargo test -p domain_auth` (13 JWT tests pass); `cargo tree -p domain_auth | grep -E "(domain-posts|application_core|cms|sea-orm)"` returns no result (only `domain_interface` is in the tree).

## 2. Update `domain_posts` to import auth from `domain_interface`

- [ ] 2.1 Add `domain_auth = { path = "../domain_auth" }` to `apps/api/domain_posts/Cargo.toml`.
- [ ] 2.2 Update `apps/api/domain_posts/src/domain/mod.rs:9`: remove `pub mod auth;`.
- [ ] 2.3 Delete `apps/api/domain_posts/src/domain/auth.rs`.
- [ ] 2.4 Remove `jsonwebtoken = { version = "9.3.1" }` from `apps/api/domain_posts/Cargo.toml:35`. Keep `axum`, `tower`, etc.
- [ ] 2.5 Verify: `cargo check -p domain_posts`; `cargo test -p domain_posts`; `cargo tree -p domain_posts | grep jsonwebtoken` returns no result; `cargo tree -p domain_posts | grep domain_auth` shows the dependency.

## 3. Update the legacy bootstrap

- [ ] 3.1 Add `domain_auth = { path = "../domain_auth" }` to `apps/api/Cargo.toml:73-75` (`[dependencies]` block of the root `cms` crate).
- [ ] 3.2 Update `apps/api/src/bin/legacy_bootstrap.rs:18-25` to replace `use cms::common::supabase_auth::{SupabaseAuthConfig, SupabaseAuthLayer}` with `use domain_auth::{SupabaseAuthConfig, SupabaseAuthLayer}`.
- [ ] 3.3 Update `apps/api/src/bin/legacy_bootstrap.rs:287-302` to replace the local `construct_supabase_auth_layer` function with `use domain_auth::legacy_bootstrap::construct_supabase_auth_layer;`. The two call sites at lines 172 and 230 use the imported function unchanged.
- [ ] 3.4 Update `apps/api/src/common/mod.rs` to remove `pub mod supabase_auth;`. Delete `apps/api/src/common/supabase_auth.rs`.
- [ ] 3.5 Verify: `cargo build --bin legacy_bootstrap` succeeds; `cargo test --workspace` passes; `cargo tree -p cms | grep domain_auth` shows the dependency.

## 4. Wire the auth layer at the gateway

- [ ] 4.1 Add `domain_auth = { path = "../domain_auth" }` to `apps/api/gateway/Cargo.toml`.
- [ ] 4.2 Update `apps/api/gateway/src/main.rs:30-32` to add `Box::new(DomainAuthService::new())` after `Box::new(DomainPostService::new())`.
- [ ] 4.3 Update `apps/api/gateway/src/main.rs:155-187` (`compose_routers`) so the protected and administrator merged routers are layered with `domain_auth::legacy_bootstrap::construct_supabase_auth_layer(...)` before being returned.
- [ ] 4.4 Verify: `cargo build -p gateway`; `cargo run -p gateway` boots, registers 2 domain services, validates both, and applies the auth layer.

## 5. Verify `DomainService` contract compliance

These tests pin down the contract behavior so that every future domain extraction can copy this task group and adapt the assertions to its own env-var surface. The seven tasks below are the **template for every future domain's contract-compliance suite**.

- [ ] 5.1 Add unit test in `apps/api/domain_auth/src/service.rs`: `let _: Box<dyn DomainService> = Box::new(DomainAuthService::new());` compiles. This proves `DomainService` is object-safe for the auth impl.
- [ ] 5.2 Add unit test in `apps/api/domain_auth/src/service.rs`: `DomainAuthService::required_env()` returns `&["SUPABASE_URL", "SUPABASE_JWT_SECRET", "AUTHORIZATION_AUDIENCE"]`.
- [ ] 5.3 Add unit test in `apps/api/domain_auth/src/service.rs`: `DomainAuthService::migrations()` returns an empty `Vec<MigrationDescriptor>`.
- [ ] 5.4 Add unit test in `apps/api/domain_auth/src/service.rs`: `DomainAuthService::register_routes(&ctx)` returns an empty `Vec<RouteRegistration>`.
- [ ] 5.5 Add integration test in `apps/api/domain_auth/src/service.rs`: `DomainAuthService::validate_config()` returns `Ok(())` when `SUPABASE_URL`, `SUPABASE_JWT_SECRET`, and `AUTHORIZATION_AUDIENCE` are all set in the env (use a temp-env helper that sets and unsets the variables around the call).
- [ ] 5.6 Add integration test in `apps/api/domain_auth/src/service.rs`: `DomainAuthService::validate_config()` returns `Err(DomainConfigError::MissingEnv("SUPABASE_URL"))` when `SUPABASE_URL` is unset (and similarly for each of `SUPABASE_JWT_SECRET` and `AUTHORIZATION_AUDIENCE`). Run three sub-tests, one per variable.
- [ ] 5.7 Add integration test in `apps/api/domain_auth/src/service.rs`: `DomainAuthService::startup_health(&ctx)` returns `Ok(())` for any `DomainContext` (the default impl does not probe the DB; this test pins down that behavior so a future change to the contract does not silently regress auth to a DB-coupled impl).
- [ ] 5.8 Verify: `cargo test -p domain_auth --lib service::tests` passes all 7 contract-compliance tests plus the 13 JWT-layer tests.

## 6. Mechanical update of every `Extension<SupabaseToken>` call site

Decision 3 is "mechanical update" (no re-export shim). Every file that currently imports `SupabaseToken` is updated to import `domain_interface::AuthenticatedActor` and every `Extension<SupabaseToken>` extractor becomes `Extension<AuthenticatedActor>`. The exhaustive file list (found via `rg "use crate::domain::auth::SupabaseToken|use crate::common::supabase_auth::SupabaseToken|use cms::common::supabase_auth"`) is:

- [ ] 6.1 Update the 8 handler files in `apps/api/domain_posts/src/api/...` that import `use crate::domain::auth::SupabaseToken`:
  - `apps/api/domain_posts/src/api/category/create/create_handler.rs:4` — replace import; `Extension<SupabaseToken>` at line 15 → `Extension<AuthenticatedActor>`; `token.email().unwrap_or("").to_string()` at line 23 → `actor.email.as_deref().unwrap_or("").to_string()` (rename local `token` to `actor`).
  - `apps/api/domain_posts/src/api/category/delete/delete_handler.rs:5` — same pattern.
  - `apps/api/domain_posts/src/api/category/modify/modify_handler.rs:4` — same pattern.
  - `apps/api/domain_posts/src/api/post/create/create_handler.rs:10` — same pattern.
  - `apps/api/domain_posts/src/api/post/delete/delete_handler.rs:11` — same pattern.
  - `apps/api/domain_posts/src/api/post/modify/modify_handler.rs:10` — same pattern.
  - `apps/api/domain_posts/src/api/post/translate/job_handler.rs:21` — same pattern.
  - `apps/api/domain_posts/src/api/post/translate/translate_handler.rs:24` — same pattern.
- [ ] 6.2 Update the 22 handler files in `apps/api/src/api/...` that import `use crate::common::supabase_auth::SupabaseToken`:
  - `apps/api/src/api/administrator/migration/migration_handler.rs:1` — replace import; `Extension<SupabaseToken>` → `Extension<AuthenticatedActor>`; rename local `token` to `actor`; `token.email()` → `actor.email.as_deref()`.
  - `apps/api/src/api/delete/delete_handler.rs:9` — same pattern.
  - `apps/api/src/api/media/bucket/create/create_handler.rs:1` — same pattern.
  - `apps/api/src/api/media/bucket/delete/delete_handler.rs:1` — same pattern.
  - `apps/api/src/api/media/bucket/empty/empty_handler.rs:1` — same pattern.
  - `apps/api/src/api/media/bucket/get/get_handler.rs:1` — same pattern.
  - `apps/api/src/api/media/bucket/list/list_handler.rs:1` — same pattern.
  - `apps/api/src/api/media/bucket/update/update_handler.rs:1` — same pattern.
  - `apps/api/src/api/media/create/create_handler.rs:1` — same pattern.
  - `apps/api/src/api/media/delete/delete_handler.rs:1` — same pattern.
  - `apps/api/src/api/media/list/list_handler.rs:1` — same pattern.
  - `apps/api/src/api/media/read/metadata_handler.rs:1` — same pattern.
  - `apps/api/src/api/post/create/create_handler.rs:1` — same pattern.
  - `apps/api/src/api/post/delete/delete_handler.rs:1` — same pattern.
  - `apps/api/src/api/post/modify/modify_handler.rs:1` — same pattern.
  - `apps/api/src/api/post/translate/job_handler.rs:1` — same pattern.
  - `apps/api/src/api/post/translate/translate_handler.rs:1` — same pattern.
  - `apps/api/src/api/tag/delete/delete_handler.rs:1` — same pattern.
  - `apps/api/src/api/user/create/create_handler.rs:1` — same pattern.
  - `apps/api/src/api/user/delete/delete_handler.rs:1` — same pattern.
  - `apps/api/src/api/user/modify/modify_handler.rs:1` — same pattern.
  - `apps/api/src/api/user/reset_password/reset_password_handler.rs:1` — same pattern.
- [ ] 6.3 Update `apps/api/src/bin/legacy_bootstrap.rs:18-25`: replace `use cms::common::supabase_auth::{SupabaseAuthConfig, SupabaseAuthLayer}` with `use domain_auth::{SupabaseAuthConfig, SupabaseAuthLayer}` (the `SupabaseAuthLayer` and `SupabaseAuthConfig` types stay in `domain_auth`; only the legacy-shim path is removed). Update the two call sites at lines 172 and 230 to use the imported factory.
- [ ] 6.4 Verify: `rg "use crate::domain::auth::SupabaseToken|use crate::common::supabase_auth::SupabaseToken|use cms::common::supabase_auth|Extension<SupabaseToken>" apps/api` returns zero matches. `cargo check --workspace` succeeds.

## 7. End-to-end verification

- [ ] 7.1 Run the full repository verification gate: `cargo check`, `cargo test`, `cargo fmt -- --check`, `cargo clippy --all-targets -- -D warnings`, `pnpm --dir apps/web build`.
- [ ] 7.2 Boot the gateway with a live testcontainer database. Verify:
  - `GET /health` returns 200 with both services' health descriptors (`domain-posts` and `domain-auth`)
  - `GET /posts` without an Authorization header returns 401
  - `GET /posts` with a valid Bearer token returns 200 (handler extracts `Extension<AuthenticatedActor>`, reads `actor.email`)
  - `GET /posts` with an expired Bearer token returns 401
  - `GET /posts` with a valid token but no required role returns 403
  - `POST /posts` requires the `my-headless-cms-writer` role
  - `GET /admin/database/migration` requires the `my-headless-cms-administrator` role
- [ ] 7.3 Boot the legacy bootstrap. Verify:
  - `GET /media/**` returns 200 with a valid token
  - `GET /media/**` returns 401 without a token
  - `GET /users/**` returns 200 with the admin role
  - `GET /ai/models` (served by gateway via `domain_posts`) returns 200 with the writer or admin role
  - `GET /categories/**` (served by gateway via `domain_posts`) returns 200 with the writer or admin role
- [ ] 7.4 Verify `cargo tree -p domain_auth | grep -E "(domain-posts|application_core|cms|sea-orm)"` returns no result. `domain_auth` is leaf-only.
- [ ] 7.5 Verify `cargo tree -p domain_posts | grep domain_auth` returns one entry.
- [ ] 7.6 Verify `cargo tree -p domain_posts | grep jsonwebtoken` returns no result.
- [ ] 7.7 Run `cargo run -p domain_auth -- health` standalone. Verify the auth domain boots, reads env vars, and reports health.
- [ ] 7.8 Run `openspec verify --change "extract-auth-into-domain-auth"` and resolve every CRITICAL finding.
- [ ] 7.9 Run `openspec sync --change "extract-auth-into-domain-auth"` to publish the new `domain-auth-service` spec into `openspec/specs/`.
- [ ] 7.10 Run `openspec archive "extract-auth-into-domain-auth"` after the sync step succeeds.

## 8. Documentation

- [ ] 8.1 Update `docs/pluggable-domain-refactor.md` to add `domain-auth` to the workspace table and to the "Per-Domain Ownership" section. Note that `domain_auth` is infrastructure-only (no DB probe; uses the default `DomainService::startup_health` impl).
- [ ] 8.2 Update `docs/api-architecture.md` to draw `domain-auth` in diagrams 1 (workspace), 2 (two-binary deployment), and 7 (request flow — the auth layer sits between the gateway listener and the route handlers).
- [ ] 8.3 Update `docs/adding-a-domain.md` to add a **Domain implementation checklist** section that lists the steps from this change as a copyable template. The checklist is the "Template value for future domains" section of `design.md`. Future extractions of `domain-media`, `domain-users`, and `domain-administrator` copy this checklist and adapt it to their env-var surface.
- [ ] 8.4 Update `docs/adding-a-domain.md` to note that every new domain that has auth-protected routes extracts `Extension<AuthenticatedActor>` (imported from `domain_interface`, never from `domain_auth`), and that `domain_auth` is for HTTP-middleware construction only.
- [ ] 8.5 Verify: docs are coherent and reference the new `domain_auth` crate and `domain_interface::AuthenticatedActor` correctly. Cross-check `docs/pluggable-domain-refactor.md`, `docs/api-architecture.md`, and `docs/adding-a-domain.md` for consistency.
