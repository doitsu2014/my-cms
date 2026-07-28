## Task 1 — Establish empty workspace members

- Add `domain_interface`, `domain_foundation`, `domain_post`, and `my_cms_api` as empty `[lib]`/`[[bin]]` members under `apps/api/Cargo.toml` `[workspace] members`.
- Each new crate has a minimal `Cargo.toml` and a placeholder `src/lib.rs` or `src/main.rs` that compiles.
- Do not change any existing source.
- Verify: `cargo check -p domain_interface -p domain_foundation -p domain_post -p my_cms_api` succeeds; `cargo test --workspace` is unchanged; `cargo fmt -- --check && cargo clippy --all-targets -- -D warnings` pass.

## Task 2 — Define the `DomainService` contract in `domain_interface`

- Add `DomainService`, `DomainContext`, `Mount`, `RouteRegistration`, `MigrationDescriptor`, and `FoundationServices` per `design.md` Decision 2.
- Add module tests that confirm the trait is object-safe (`fn _assert_object_safe<T: ?Sized + DomainService>() {}`) and that `Mount::Public/Protected/Administrator` round-trips to JSON.
- Verify: `cargo test -p domain_interface` passes; `cargo check --workspace` still passes.

## Task 3 — Lift `AppError`, presentation models, auth, CORS, and tracing to `domain_foundation`

- Move `apps/api/application_core/src/common/app_error.rs` into `apps/api/domain_foundation/src/error.rs`, re-exporting it as `domain_foundation::AppError`. Keep variant list unchanged.
- Move `apps/api/src/presentation_models/api_response.rs` into `apps/api/domain_foundation/src/response.rs`; keep envelope types unchanged.
- Move `apps/api/src/common/supabase_auth.rs` into `apps/api/domain_foundation/src/auth.rs`; re-export as `domain_foundation::SupabaseAuthLayer` and `SupabaseAuthConfig`. Public constructor and behavior unchanged.
- Add factory functions `cors_layer()`, `body_limit_layer()`, `otel_layers()`, `cookie_layer()` in `apps/api/domain_foundation/src/layers.rs`.
- Update `apps/api/application_core/src/lib.rs` and `apps/api/src/lib.rs` to re-export from `domain_foundation`; do not delete yet.
- Verify: `cargo check --workspace`, `cargo test --workspace`, `cargo fmt -- --check`, `cargo clippy --all-targets` all pass. `/health` smoke check via existing integration test still passes.

## Task 4 — Move foundation services (media + GraphQL) into `domain_foundation`

- Move `commands::media::{MediaConfig, SupabaseStorage, MediaCacheKey, CachedMedia, create_media_cache}` from `application_core` to `domain_foundation/src/media.rs`.
- Move the GraphQL schema builder (`commands::graphql::query_root::schema`) and the `application_core::graphql` module into `domain_foundation/src/graphql.rs`. Seaography registration callback stays as-is.
- Move `commands::user::supabase_admin_client::SupabaseAdminClient` into `domain_foundation/src/admin_client.rs`.
- Re-export from `application_core` as compatibility shims; `apps/api/src/lib.rs::AppState` becomes a thin façade that constructs a `FoundationServices` + `DomainContext`.
- Verify: `cargo check --workspace`, `cargo test --workspace` pass. One-shot `cargo test -p application_core commands::media` and `commands::graphql` pass.

## Task 5 — Add migration orchestrator

- Add `apps/api/domain_foundation/src/migrations.rs` with `MigrationDescriptor`, `MigrationSet`, and an `Orchestrator` that topologically sorts descriptors by `id` and `depends_on` and runs them via `sea-orm-migration`.
- Wire `apps/api/migration/src/main.rs` to delegate to the orchestrator, keeping the same migration identities (`m20240409_151952_release_100`, `m20250330_151455_release_110`, `m20260126_040610_release_300`, `m20260531_000001_pgvector`).
- Add unit tests for cycle detection, duplicate detection, and topological order.
- Verify: `cargo run -p migration -- up` succeeds against the test database; the orchestrator logs the same migration identities in the same order; `cargo test --workspace` passes.

## Task 6 — Create `domain-post` skeleton with empty `DomainPostService`

- Scaffold `apps/api/domain_post/` with `Cargo.toml` and `src/lib.rs` exporting `pub struct DomainPostService` implementing `DomainService`.
- Implement `name()`/`version()`/`required_env()`; leave `register_routes`, `migrations`, and `health` returning empty results.
- Register the new crate in `apps/api/Cargo.toml` `members` and `workspace.dependencies`.
- Verify: `cargo check -p domain_post` succeeds; `cargo test --workspace` still passes.

## Task 7 — Move post HTTP adapters into `domain-post`

- Move `apps/api/src/api/post/**/*.rs` (handlers + mod.rs files) into `apps/api/domain_post/src/api/` with re-exports `pub use api::*` for handler functions.
- Rewrite handlers so each still extracts state, calls the corresponding `*HandlerTrait` from `application_core::commands::post::*`, and returns the existing `ApiResponseWith`/`ApiResponseError` envelope.
- Move the corresponding `commands::post::{create,read,modify,delete,translate}` into `apps/api/domain_post/src/commands/`. Keep `commands::post::translate::translate_handler::TranslateHandler`, `JobHandler`, and the OpenAI/pgvector client wrappers.
- Verify: `cargo check -p domain_post`, `cargo test -p domain_post`, `cargo test --workspace`. Existing router tests against `/posts/**`, `/posts/{post_id}/translate`, `/posts/{post_id}/translate/background`, `/posts/{post_id}/translate/jobs/**` must still pass (HTTP method + path + status code parity).

## Task 8 — Wire post migrations and Seaography contribution

- Move post-relevant migrations (`m20260126_040610_release_300` for `posts/post_tags/post_translations/translation_jobs/categories/tags/category_tags/category_translations` and `m20260531_000001_pgvector`) into `apps/api/domain_post/src/migrations.rs`.
- Implement `DomainPostService::migrations()` returning descriptors with `depends_on` referencing foundation migration ids (none in this change; foundation does not yet own a migration, so the dependencies are empty and execution order remains the historical one).
- Implement `DomainPostService::register_routes(...)` returning `Mount::Public`, `Mount::Protected` registrations under existing paths (`/posts/**`, `/posts/{post_id}/translate`, `/posts/{post_id}/translate/background`, `/posts/{post_id}/translate/jobs/**`).
- Implement `DomainPostService::health()` as a `posts` table `SELECT 1` check via `DatabaseConnection`.
- Verify: `cargo test -p domain_post` and `cargo run -p migration -- up` both succeed against the test database; `/health` aggregator returns the post service's health.

## Task 9 — Add new `my_cms_api` gateway bin

- Create `apps/api/my_cms_api/src/main.rs` (composition root). It loads env, initializes tracing/OpenTelemetry, builds one `DatabaseConnection`, builds `FoundationServices`, builds `DomainContext`, registers `DomainPostService` and a no-op `Hello` test domain in a `Vec<Box<dyn DomainService>>`, iterates the vec to build the three Axum routers (`Mount::Public`, `Mount::Protected`, `Mount::Administrator`), applies the foundation's auth/CORS/cookie/body-limit/Otel layers, runs the migration orchestrator only when invoked via `/administrator/database/migration`, and binds the listener.
- The new bin re-uses `construct_app_state`'s current env-var surface unchanged.
- Verify: `cargo run -p my_cms_api` boots and `/health` returns 200; `cargo test --workspace` passes.

## Task 10 — Cut over and remove the legacy `my-cms-api` bin

- Delete `apps/api/src/bin/my-cms-api.rs` and `apps/api/src/lib.rs` (and the now-empty `apps/api/src/api/**` directories).
- Update `apps/api/Cargo.toml` `[[bin]]` to point only to the new bin name `my-cms-api` (the user-facing name) backed by `apps/api/my_cms_api/src/main.rs`. Keep `[[bin]] name = "my-cms-api"` for deployment image compatibility.
- Verify: `cargo build -p cms --bin my-cms-api` builds; `cargo run --bin my-cms-api` serves `/health`, `/posts/**`, `/posts/{post_id}/translate`, `/posts/{post_id}/translate/background`, `/posts/{post_id}/translate/jobs/**`, and both GraphQL paths; `cargo test --workspace` and the verification gate (`cargo fmt -- --check && cargo clippy --all-targets`) pass.

## Task 11 — Add a new-domain scaffold

- Commit `apps/api/templates/domain_template/` (a Cargo lib + README) with the standard `Cargo.toml`, `src/lib.rs`, `src/api/` placeholder, `src/commands/` placeholder, `src/migrations.rs` placeholder, `src/service.rs` implementing an empty `DomainService`.
- Add `docs/adding-a-domain.md` describing: copy template, replace crate name, add to `Cargo.toml` members, register in the gateway composition manifest.
- Verify: copy the template into a temporary `apps/api/domain_demo/`, add to workspace, run `cargo check -p domain_demo` succeeds, remove the temporary crate. Update docs.

## Task 12 — Verification and synchronization

- Run the full repository verification gate: `cargo check`, `cargo test`, `cargo fmt -- --check`, `cargo clippy --all-targets -- -D warnings`, `pnpm --dir apps/web build`.
- Run `openspec verify --change "refactor-api-into-pluggable-domain-libraries"` and resolve every CRITICAL finding.
- Optional: run `openspec sync --change "refactor-api-into-pluggable-domain-libraries"` only if the team chooses to publish the new canonical specs (`domain-service-interface`, `domain-post-service`, `api-gateway-bootstrap`) into `openspec/specs/`.
- Optional: archive the change via `openspec archive "refactor-api-into-pluggable-domain-libraries"` only after the next domain (`domain-tag`, `domain-category`, `domain-media`, `domain-user`) is approved and shipped in a follow-up change.
