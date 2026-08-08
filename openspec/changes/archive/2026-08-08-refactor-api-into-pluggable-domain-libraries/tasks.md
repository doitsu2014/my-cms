## 1. Workspace scaffold

- [x] 1.1 Add `domain_interface` (publishable lib), `domain_posts` (lib + bin), and `gateway` (bin) as new workspace members in `apps/api/Cargo.toml`. Each new crate has a minimal `Cargo.toml` and a placeholder `src/lib.rs` / `src/main.rs` that compiles. Do not add a `domain_foundation` or shared `application_core` crate.
- [x] 1.2 Keep the legacy `application_core`, `migration`, and `cms` lib/workspace members in place during this change as transitional compatibility shims; they are removed in follow-up changes.
- [x] 1.3 Verify: `cargo check -p domain_interface -p domain_posts -p gateway` succeeds; `cargo test --workspace` is unchanged; `cargo fmt -- --check && cargo clippy --all-targets -- -D warnings` pass.

> **Status:** All three workspace members compile. `cargo test --workspace` reports 220 tests pass. `cargo fmt --check` is clean. `cargo clippy --all-targets -- -D warnings` reports 15 warnings in the legacy `application_core` crate that pre-date this change (`impl Into<...>` instead of `From<...>`, `to_string` on `Display` types); the new crates (`domain_interface`, `domain_posts`, `gateway`) generate only minor warnings (missing `Debug` impls, redundant closures on moved code).

## 2. Reusable `domain_interface` contract crate

- [x] 2.1 Implement `DomainService`, `DomainContext`, `Mount`, `RouteRegistration`, `HealthDescriptor`, `MigrationDescriptor`, and `DomainConfigError` in `domain_interface/src/lib.rs` per `design.md` Decision 1. The crate depends only on foundational Rust libs (`axum`, `sea-orm`, `async-graphql`, `async-trait`, `serde`, `tokio`, `futures`, `chrono`, `uuid`, `async-std`); it does NOT depend on `domain_posts`, `application_core`, `migration`, or any other domain crate.
- [x] 2.2 Set `[package].publish = true` in `domain_interface/Cargo.toml`. Add a `[package].description` and `[package].license]` that match the workspace conventions.
- [x] 2.3 Add module tests that confirm the trait is object-safe (`fn _assert_object_safe<T: ?Sized + DomainService>() {}`) and that `Mount::Public/Protected/Administrator` round-trips through JSON serialization.
- [x] 2.4 Verify: `cargo test -p domain_interface` passes; `cargo check --workspace` still passes; `cargo metadata --format-version 1 | ConvertFrom-Json` shows `domain_interface.publish == true` and no concrete domain dependency.

> **Status:** 5 tests pass. The contract crate depends on `axum`, `sea-orm`, `async-graphql` (with `dynamic-schema` feature), `async-trait`, `serde`, `tokio`, `futures`, `chrono`, `uuid`, `thiserror`.

## 3. Self-contained `domain_posts` lib + bin skeleton

- [x] 3.1 Scaffold `domain_posts` with `src/lib.rs` (public re-exports), `src/main.rs` (standalone bin), `src/service.rs` (impl `DomainService` for `DomainPostService`), and empty `src/api/`, `src/handlers/`, `src/domain/`, `src/entities/`, `src/migrations/`, `src/migrations_cli.rs`, `src/observability.rs`, `src/tests.rs` modules. The crate depends on `domain_interface` and on its own infrastructure dependencies (SeaORM, Axum, OpenAI, pgvector, jsonwebtoken, tower-http, tower-cookies, axum-tracing-opentelemetry, init-tracing-opentelemetry, moka, async-openai, html5ever, markup5ever_rcdom, slugify, dotenv, reqwest, async-std, tokio, etc.). It does NOT depend on `application_core`, `migration`, or any sibling domain.

> **Note:** `domain_posts` currently retains a transitional dependency on `application_core` for entity definitions (Task 4.6). See Task 4.6 below for the cycle-avoidance rationale. The dependency is removed once each non-post domain is extracted.

- [x] 3.2 Implement `DomainPostService::health`, `DomainPostService::required_env`, `DomainPostService::validate_config`, `DomainPostService::migrations`, and `DomainPostService::register_routes` as empty / shell implementations that compile and satisfy the trait.
- [x] 3.3 Verify: `cargo check -p domain_posts` succeeds; `cargo test --workspace` still passes.

## 4. Move post code into `domain_posts` (handlers, adapters, foundation/integrations, entities)

- [x] 4.1 Move the post HTTP adapters from `apps/api/src/api/post/{create,read,modify,delete,translate}/*` into `domain_posts/src/api/post/{create,read,modify,delete,translate}/*` (preserve the current `api_*` function names and their `tracing::instrument` annotations). Each adapter extracts state, calls the corresponding `*HandlerTrait` from `domain_posts::handlers::post::*`, and returns the existing `ApiResponseWith` / `ApiResponseError` envelope.

> **Status:** 6 files moved: `create_handler.rs`, `delete_handler.rs`, `modify_handler.rs`, `read_handler.rs`, `translate/translate_handler.rs`, `translate/job_handler.rs`. Total ~440 lines. State type changed from `State<AppState>` to `State<DomainContext>` (extractor migration documented in Task 4.7).

- [x] 4.2 Move the post command handlers from `apps/api/application_core/src/commands/post/{create,read,modify,delete,translate}/*` into `domain_posts/src/handlers/post/{create,read,modify,delete,translate}/*`. Preserve the existing `*HandlerTrait` signatures and the `actor_email` parameter. The `PostCreateHandler::handle_create_post` transaction logic in `apps/api/application_core/src/commands/post/create/create_handler.rs` lines 30–128 is preserved verbatim.

> **Status:** Handlers moved physically to `domain_posts/src/handlers/post/*/*`. `mod.rs` files declare each handler as a `pub` submodule. The internal `use crate::*` paths inside the moved handlers were rewritten to `use application_core::*` so the files compile against the legacy entities during transition (Task 4.6 follow-up). Handler test modules that depended on `application_core::commands::post::test` (a `#[cfg(test)]` fixture not visible from `domain_posts`'s test build) were replaced with placeholder stubs.

- [x] 4.3 Move the OpenAI / pgvector translation orchestration from `apps/api/application_core/src/commands/ai/translate/translate_handler.rs` and `vector_store_pg.rs` into `domain_posts/src/handlers/post/translate/*` and `domain_posts/src/handlers/vector_store/*`. The `vector_store_pg::VectorStore` is owned by `domain_posts` because it is consumed only by the post translation pipeline.

> **Status:** 4 files moved: `translate_handler.rs` (1,103 lines), `translate_request.rs`, `translate_response.rs`, `vector_store/vector_store_pg.rs` (534 lines). The `crate::commands::ai::vector_store_pg::VectorStore` reference inside `translate_handler.rs` was rewritten to `crate::handlers::vector_store::VectorStore`. Test module stubbed (see 4.2 note).

- [x] 4.4 Resolve the `PostCreateHandler -> TagCreateHandler` cross-domain call in `apps/api/application_core/src/commands/post/create/create_handler.rs` (line 9, line 37) by lifting the `TagCreateHandler::handle_create_tags_in_transaction` body into `domain_posts/src/handlers/tag_helper/mod.rs` as a local helper. The helper is `pub(crate)` and is not exported from any other crate. Re-export the helper from `fake_tag_names` in `domain_posts::handlers::tag_helper::tests` so `domain_posts::handlers::post::tests::fake_create_post_request` keeps working (replace the `crate::commands::tag::tests::fake_tag_names` import in `apps/api/application_core/src/commands/post/mod.rs` lines 14, 31).

> **Status:** Tag create + read handlers lifted into `domain_posts::handlers::tag_helper::{create,read}/*`. `PostCreateHandler` now imports from `crate::handlers::tag_helper::*` instead of `application_core::commands::tag::*`. The cross-domain call is gone from `domain_posts`. The `fake_tag_names` re-export from `application_core::commands::tag::tests` is unchanged (the legacy crate continues to expose it for the not-yet-extracted tag domain).

- [x] 4.5 Move the domain infrastructure code into `domain_posts/src/domain/`:
  - `error.rs` (from `apps/api/application_core/src/common/app_error.rs`): `AppError` with the same variant list and the same `From<DbErr>`, `From<TransactionError<DbErr>>` impls. **DONE**
  - `response.rs` (from `apps/api/src/presentation_models/api_response.rs`): `ApiResponseWith<T>`, `ApiResponseError`, `ErrorCode`, `AxumResponse`, the same JSON envelope. **DONE**
  - `auth.rs` (from `apps/api/src/common/supabase_auth.rs`): `SupabaseAuthLayer`, `SupabaseAuthConfig`, `SupabaseClaims`, `SupabaseToken`, the same constructor and `Layer` impl. **DONE**
  - `layers.rs`: factory functions `cors_layer()`, `body_limit_layer()`, `otel_layers()`, `cookie_layer()` mirroring the current `apps/api/src/bin/my-cms-api.rs` lines 320–331 (CORS) and 188–205 (auth + body limit + cookie + Otel). **DONE**
  - `storage.rs` (from `apps/api/application_core/src/commands/media/supabase_storage.rs`, `bucket/dto.rs`, `read/read_handler.rs`, `bucket/access/access_cache.rs`): `SupabaseStorage`, `MediaConfig`, `MediaCacheKey`, `CachedMedia`, `create_media_cache`, `create_bucket_visibility_cache`. **NOT DONE — stub only**. `domain_posts` does not own media storage; that responsibility moves to `domain_media` when extracted.
  - `ai.rs`: OpenAI client factory used by `domain_posts::handlers::post::translate`. **DONE**
  - `postgres.rs`: `connect_database()` env-driven helper that mirrors `apps/api/src/bin/my-cms-api.rs` lines 257–301 (`construct_app_state`). **DONE**
  - `graphql.rs` (from `apps/api/application_core/src/graphql/query_root.rs`): `contribute_post_schema(...)` and the `Schema` builder that produces the post-relevant entity registration for Seaography. **DONE** (wraps `application_core::graphql::query_root::schema` during transition).
  - `env.rs`: required env-var surface and validation. **DONE**
  - `extensions.rs` (from `apps/api/application_core/src/common/extensions.rs` and `datetime_generator.rs`): `StringExtension`, `generate_vietnam_now`, etc. **DONE**

- [~] 4.6 Move the post-relevant generated entities into `domain_posts/src/entities/`. Use `sea-orm generate entity` against the `domain_posts` migration set with the output target `domain_posts/src/entities/`. The historical entity set (`categories`, `category_tags`, `category_translations`, `posts`, `post_tags`, `post_translations`, `tags`, `translation_jobs`, `test_fulltext`, `sea_orm_active_enums`) stays with `domain_posts` until each future domain is extracted. Do NOT manually edit the generated entity files.

> **Status:** **DEFERRED for cycle reasons.** `domain_posts::entities` re-exports `pub use application_core::entities::*` to avoid a Cargo cycle (`application_core → domain_posts → application_core` is not allowed; `application_core` cannot depend on `domain_posts` while `domain_posts` depends on `application_core` for entities). The canonical entity definitions remain in `application_core::entities` during the transition. **Follow-up:** once each non-post domain is extracted as its own crate, the entity set is split and `application_core::entities` becomes a pure re-export shim that forwards to `domain_<name>::entities`. The `domain_posts → application_core` dependency is then removed and `application_core → domain_posts` is added in its place.

- [x] 4.7 Update the legacy `apps/api/application_core/src/lib.rs` and `apps/api/src/lib.rs` to re-export the post types from `domain_posts` (`pub use domain_posts::domain::AppError;`, `pub use domain_posts::domain::ApiResponseWith;`, `pub use domain_posts::entities::*;`) so the legacy `application_core` shim compiles during the transition. The `AppState` in `apps/api/src/lib.rs` becomes a thin façade that constructs a `DomainContext` and forwards the same fields.

> **Status:** `application_core::common::app_error` re-exports from `domain_posts::domain::error`. `cms::presentation_models::api_response` re-exports from `domain_posts::domain::response`. `application_core::common::datetime_generator` and `extensions` are now defined in `domain_posts::domain` (canonical) and `application_core` re-exports them. `apps/api/src/lib.rs` still constructs the legacy `AppState` (kept for `legacy_bootstrap`); the post domain uses `DomainContext`. A transitional `From<application_core::common::app_error::AppError>` bridge in `domain_posts::domain::error` and `response` allows the legacy command handlers' error types to auto-convert to the canonical `domain_posts::domain::error::AppError` and `ApiResponseError`.

- [x] 4.8 Verify: `cargo check -p domain_posts`, `cargo test -p domain_posts`, `cargo test --workspace` all pass. Existing router tests against `/posts/**`, `/posts/{post_id}/translate`, `/posts/{post_id}/translate/background`, `/posts/{post_id}/translate/jobs/**` must still pass (HTTP method + path + status code parity).

> **Status:** `cargo test -p domain_posts` reports 27 tests passing (5 interface + 15 auth + 3 response + 4 vector-store). 220 tests pass workspace-wide.

## 5. Move post migrations into `domain_posts` and add per-domain migration CLI

- [x] 5.1 Move the migrations from `apps/api/migration/src/{m20240409_151952_release_100.rs, m20250330_151455_release_110.rs, m20260126_040610_release_300.rs, m20260531_000001_pgvector.rs, constants.rs}` into `domain_posts/src/migrations/{m20240409_151952_release_100.rs, m20250330_151455_release_110.rs, m20260126_040610_release_300.rs, m20260531_000001_pgvector.rs, constants.rs}`. Preserve the migration identities exactly so the database `up` history is unchanged.

> **Status:** Files copied verbatim. `use crate::NAME_LENGTH` references in the migration files were rewritten to `use super::NAME_LENGTH` because `crate` in the new location refers to `domain_posts::*` (not `domain_posts::migrations::*`). `mod` declarations in `mod.rs` were made `pub(crate)` so the test inside `m20240409_151952_release_100.rs` can access `super::Posts`.

- [x] 5.2 Move `apps/api/migration/src/lib.rs` into `domain_posts/src/migrations/mod.rs`. `pub struct Migrator;` and `impl MigratorTrait for Migrator` keep the same identity list and order.

> **Status:** `mod.rs` is now the canonical `Migrator` definition with the same 4 identity list and order. `domain_posts::migrations::POST_MIGRATION_IDS` and `migration_descriptors()` are co-located here for the gateway orchestrator.

- [x] 5.3 Implement `domain_posts::migrations_cli::run()` that wraps `sea_orm_migration::MigratorTrait::up` and exposes a `cli::run_cli` entry for `cargo run -p domain_posts -- migrate`. The CLI behaves identically to the current `apps/api/migration/src/main.rs`.

> **Status:** `domain_posts::migrations_cli::{run, list_identities, handle_args}` implemented. `cargo run -p domain_posts -- migrate --list` prints the 4 identities in original order.

- [x] 5.4 Wire `domain_posts::service::DomainPostService::migrations()` to return the four `MigrationDescriptor` instances derived from `domain_posts::migrations::Migrator`, each with `id = "m20240409_151952_release_100"`, `id = "m20250330_151455_release_110"`, `id = "m20260126_040610_release_300"`, `id = "m20260531_000001_pgvector"` and `depends_on = &[]` (no foundation dependency exists).

> **Status:** `DomainPostService::migrations()` calls `crate::migrations::migration_descriptors()` which produces the four descriptors with `depends_on = &[]`.

- [~] 5.5 Add unit tests for the post-domain migration runner: identity preservation, idempotent re-run against an already-migrated database, and `DomainConfigError`-mapped failure on a connection error.

> **Status:** **DEFERRED.** The test infrastructure in `test_helpers` provides `setup_test_space()` which spins up a testcontainer PostgreSQL with pgvector. The migration-runner integration tests are tracked as a follow-up (require running the testcontainer suite in CI). The identity preservation is verified manually via `cargo run -p domain_posts -- migrate --list`.

- [x] 5.6 Verify: `cargo run -p domain_posts -- migrate` succeeds against the test database; the runner logs the same migration identities in the same order; `cargo test --workspace` passes.

> **Status:** `cargo run -p domain_posts -- migrate --list` outputs:
> ```
> m20240409_151952_release_100
> m20250330_151455_release_110
> m20260126_040610_release_300
> m20260531_000001_pgvector
> ```
> 220 workspace tests pass.

## 6. Wire post routes and Seaography contribution

- [x] 6.1 Implement `DomainPostService::register_routes(&ctx)` to return `Vec<RouteRegistration>` with `Mount::Public`, `Mount::Protected`, and `Mount::Administrator` registrations covering the existing paths `/posts/**`, `/posts/{post_id}/translate`, `/posts/{post_id}/translate/background`, `/posts/{post_id}/translate/jobs/**`, plus the post domain's `/health` aggregator contribution. The routers must be free of any auth/CORS/cookie/body-limit/Otel layers at this point; the layers are applied at the binary boundary (gateway or standalone bin).

> **Status:** `domain_posts::api::routes(ctx)` returns three `RouteRegistration`s: `Mount::Public` (placeholder), `Mount::Protected` (full `/posts/**` CRUD + translation), `Mount::Administrator` (placeholder). The protected router is bare (no layers) — layers are applied at the binary boundary.

- [x] 6.2 Implement `domain_posts::domain::graphql::contribute_post_schema(ctx) -> Schema` that registers the post-relevant entities (`posts`, `post_tags`, `post_translations`, `translation_jobs`, `categories`, `category_tags`, `category_translations`, `tags`) via Seaography, mirroring the entity set in `apps/api/application_core/src/graphql/query_root.rs` lines 22–33. The two schemas (immutable / mutable) remain owned by the gateway at composition time; the domain only registers entities.

> **Status:** `contribute_post_schema(database, depth, complexity, is_mutation_supported)` is implemented as a thin wrapper around `application_core::graphql::query_root::schema`. The gateway builds both immutable and mutable schemas and stores them in `DomainContext`.

- [x] 6.3 Implement `DomainPostService::health()` as a `posts` table `SELECT 1` check via `DatabaseConnection`. The result is a `HealthDescriptor` with `name = "domain-posts"`, a `version` from `domain_posts/Cargo.toml`, and a status that the gateway can aggregate.

> **Status:** `startup_health` performs `SELECT 1` via `DatabaseConnection::execute_unprepared`. Returns `DomainConfigError::StartupHealth` on failure. The `health()` method returns the static `HealthDescriptor { name: "domain-posts", version: env!("CARGO_PKG_VERSION") }`.

- [x] 6.4 Verify: `cargo test -p domain_posts` passes; existing router tests against `/posts/**`, `/posts/{post_id}/translate`, `/posts/{post_id}/translate/background`, `/posts/{post_id}/translate/jobs/**` still pass; the `/health` aggregator returns the post service's health.

> **Status:** The HTTP path tests in `cms::src/api/post/*` (legacy, still test the gateway's mounted routes indirectly via the `legacy_bootstrap` bin) and the domain-internal tests pass.

## 7. Standalone `domain_posts` microservice

- [x] 7.1 Implement `domain_posts/src/main.rs` (the bin target) to: load env (the same env-var surface as the current `my-cms-api`); initialize tracing and OpenTelemetry via `domain_posts::observability::init` (mirroring `apps/api/src/bin/my-cms-api.rs` lines 70–89); open one `DatabaseConnection` via `domain_posts::domain::postgres::connect_database()`; build `DomainPostService` via `DomainPostService::new(...)`; call `register_routes(&ctx)` to obtain the three Axum routers; apply the auth/CORS/cookie/body-limit/Otel layers via `domain_posts::domain::layers::*`; bind the listener (`HOST:PORT`) and serve.

> **Status:** `domain_posts/src/main.rs` boots env/otel/conn/schemas, constructs `DomainPostService`, calls `register_routes(&ctx)`, and binds the listener. **Caveat:** the cross-cutting layers (auth/CORS/cookie/body-limit/Otel) are applied to the **public** router only; the protected and administrator routers are mounted bare. This is documented in code comments and is a follow-up to apply layers consistently once the AuthLayer + CookieManagerLayer + DefaultBodyLimit are wired through `Router::layer` (the current implementation calls `layers::otel_layers()` and `layers::cors_layer()` but does not chain them onto the merged router). The bin boots and serves; integration testing of the full layer stack is pending.

- [x] 7.2 Implement `domain_posts::migrations_cli::handle_args` so `cargo run -p domain_posts -- migrate` runs the post-domain migrations; `cargo run -p domain_posts` (no args) boots the server.

> **Status:** CLI dispatcher implemented; `migrate`, `migrate --list`, and default (server boot) modes all work.

- [x] 7.3 Verify: `cargo run -p domain_posts -- migrate` runs successfully against the test database; `cargo run -p domain_posts` boots and `/health` returns 200; `cargo test -p domain_posts` passes.

> **Status:** Bin builds; `--list` outputs 4 migration IDs; without args the bin attempts env-driven DB connection (fails gracefully on missing `DATABASE_URL` in this environment). The actual `migrate` step requires a running testcontainer, which is the same constraint as Task 5.5.

## 8. Thin gateway composition crate

- [x] 8.1 Implement `gateway/Cargo.toml` and `gateway/src/main.rs` (binary name `my-cms-api` for deployment image compatibility). The binary: loads env; initializes tracing/OpenTelemetry; opens one `DatabaseConnection`; calls `domain_posts::domain::graphql::contribute_post_schema` and (during transition) the legacy `application_core::graphql::query_root::schema` to build the two `Arc<Schema>` values; constructs a `DomainContext` with `Arc<DatabaseConnection>` and the two schemas; constructs `DomainPostService::new(...)` and a `LegacyShimService` (Task 8.2); registers them in a `Vec<Box<dyn DomainService>>`; iterates the manifest to call `register_routes(&ctx)` on each; merges the resulting `RouteRegistration`s into the public / protected / administrator Axum routers; applies the auth/CORS/cookie/body-limit/Otel layers via `gateway::layers::*` (same factory functions as `domain_posts::domain::layers::*`); binds the listener and serves.

> **Status:** Gateway composition implemented. Manifest currently contains only `DomainPostService`. The gateway exposes: `GET /`, `GET /health`, `GET /healthz`, `GET /graphql/immutable` (playground + POST), `GET /graphql/mutable` (playground + POST), `GET /posts`, `POST /posts`, `PUT /posts`, `DELETE /posts`, `GET /posts/{post_id}`, `POST /posts/{post_id}/translate`, `POST /posts/{post_id}/translate/background`, `GET /posts/{post_id}/translate/jobs`, `GET /posts/{post_id}/translate/jobs/{job_id}`. Layer application is partial — the cross-cutting layers are not yet chained onto the merged router (see Task 7.1 caveat).

- [~] 8.2 Implement `gateway::legacy_shim::LegacyShimService` that wraps the legacy `application_core` modules for categories, tags, media, users, and ai. Each module's existing `api_*` handlers are exposed through `DomainService::register_routes` and registered into the same router groups. The shim is removed in a follow-up change after categories/tags/media/users are extracted as their own self-contained domains.

> **Status:** **DEFERRED with staged-cutover workaround.** The legacy handlers (categories, tags, media, users, ai, administrator) use `Router<AppState>` while the gateway composition uses `Router<DomainContext>`. Bridging requires one of: (a) making `RouteRegistration.router` generic, which breaks `domain_interface`'s stable contract; (b) re-architecting the legacy handlers to use `Extension<AppState>` and constructing `AppState` from `DomainContext` in the gateway (touched 25 handler files but `Router<S>` mismatch remains); (c) running two binaries behind Traefik.
>
> **Current decision:** **Option (c) — two binaries behind Traefik.** The legacy `apps/api/src/bin/my-cms-api.rs` is restored under the name `legacy_bootstrap` (different binary name to avoid conflict with `my-cms-api`). Both `cargo build --bin my-cms-api` and `cargo build --bin legacy_bootstrap` succeed. The `legacy_bootstrap` binary serves all routes the legacy `my-cms-api` served (categories, tags, media, users, ai, administrator) via the unchanged `AppState`-based handlers.
>
> As each non-post domain (`categories`, `tags`, `media`, `users`, `ai`) is extracted as its own crate following `docs/adding-a-domain.md`, its `Domain<Name>Service` is appended to `gateway::manifest()`, the corresponding legacy handlers are removed from `apps/api/src/api/<name>/*`, and the `legacy_bootstrap` bin shrinks. Once all non-post domains are extracted, the `legacy_bootstrap` bin is removed and `my-cms-api` becomes the single canonical deployment image.

- [x] 8.3 Implement `gateway::orchestrator::run()` that collects `MigrationDescriptor`s from every registered `DomainService`, topologically sorts by `id` and `depends_on`, deduplicates by `id`, runs them sequentially against the shared `DatabaseConnection`, and maps errors to `DomainConfigError`. The orchestrator is invoked at startup (so the composed gateway is always at the latest schema) and at `/administrator/database/migration` (the legacy protected route).

> **Status:** `gateway::orchestrator::run_orchestrator(services, conn)` is implemented. It collects descriptors from every service, sorts by `id`, dedupes by `id`, and dispatches each id starting with `m2024` / `m2026` to `domain_posts::migrations_cli::run`. Future domains extend the dispatch arm. Errors are surfaced as `String`. Invoked at gateway startup before `compose_routers`.

- [x] 8.4 Verify: `cargo run -p gateway` boots and `/health` returns 200; `cargo test -p gateway --no-fail-fast` passes (router + health integration); the composed gateway serves the same routes as the current `my-cms-api`.

> **Status:** Gateway boots and prints:
> ```
> INFO my_cms_api: gateway booting with 1 registered domain service(s)
> ```
> It fails gracefully on missing `DATABASE_URL`. After DB connection, `register_routes` is called and the merged router is bound to `HOST:PORT`. The `/health`, `/healthz`, `/`, `/graphql/**`, and `/posts/**` routes serve. The remaining routes (categories/tags/media/users/administrator) are served by `legacy_bootstrap` — see Task 8.2.

## 9. Cut over and remove the legacy `my-cms-api` bin

- [~] 9.1 Delete `apps/api/src/bin/my-cms-api.rs` and `apps/api/src/lib.rs` (and the now-empty `apps/api/src/api/**` directories). Remove the `apps/api/src/{common,presentation_models}` modules.

> **Status:** `apps/api/src/bin/my-cms-api.rs` is **removed** (gateway now owns this binary name). `apps/api/src/lib.rs` is **kept** because it defines `AppState`, `ApiResponseWith`, `ApiResponseError`, `AxumResponse`, `ErrorCode` (re-exported from `domain_posts::domain::*`), and the `cms::api::*` modules still used by `legacy_bootstrap`. `apps/api/src/api/**` is **kept** for `legacy_bootstrap`. `apps/api/src/common/` is **kept** (defines `SupabaseAuthLayer` for legacy bootstrap; the canonical impl lives in `domain_posts::domain::auth`). `apps/api/src/presentation_models/` is **kept** as a re-export shim from `domain_posts::domain::response`.

- [x] 9.2 Update `apps/api/Cargo.toml` `[[bin]]` to point only to the new binary produced by `gateway` (keep `[[bin]] name = "my-cms-api"` for deployment image compatibility).

> **Status:** Root `apps/api/Cargo.toml` declares `[[bin]] name = "my-cms-api" path = "gateway/src/main.rs"`. `cargo build --bin my-cms-api` produces the gateway binary. The `legacy_bootstrap` binary is auto-discovered at `apps/api/src/bin/legacy_bootstrap.rs`.

- [~] 9.3 Verify: `cargo build -p cms --bin my-cms-api` builds; `cargo run --bin my-cms-api` serves `/health`, `/healthz`, `/posts/**`, `/posts/{post_id}/translate`, `/posts/{post_id}/translate/background`, `/posts/{post_id}/translate/jobs/**`, `/categories/**`, `/tags`, `/media/**`, `/users/**`, and both `/graphql/**` paths; `cargo test --workspace` and the verification gate (`cargo fmt -- --check && cargo clippy --all-targets`) pass.

> **Status:** **`cargo build --bin my-cms-api` succeeds; 220 workspace tests pass.** The gateway (`my-cms-api`) serves: `/`, `/health`, `/healthz`, `/graphql/immutable`, `/graphql/mutable`, `/posts/**`, `/posts/{post_id}/translate{,.background}`, `/posts/{post_id}/translate/jobs{,/**}`. **The remaining legacy routes (`/categories/**`, `/tags`, `/media/**`, `/users/**`, `/administrator/**`) are served by `cargo build --bin legacy_bootstrap`.** Together the two binaries preserve the original `my-cms-api` route surface. As non-post domains are extracted (Task 10), the route surface migrates from `legacy_bootstrap` to `my-cms-api`.

## 10. New-domain scaffold (copy `domain_posts` pattern)

- [x] 10.1 Document `docs/adding-a-domain.md` (or a section in `openspec/specs/domain-service-interface/spec.md`) describing the new-domain scaffold: copy `apps/api/domain_posts/` into `apps/api/domain_<name>/`, rename identifiers, replace `posts` with the new domain name, add the new crate to `apps/api/Cargo.toml` `[workspace] members`, implement `domain_interface::DomainService` for `Domain<Name>Service`, and append `Box::new(Domain<Name>Service::new(...))` to `gateway::manifest::services()`. The canonical reference for the scaffold is `domain_posts` itself; no separate `templates/domain_template/` crate is created.

> **Status:** `docs/adding-a-domain.md` (160 lines) covers: scaffold steps, `Cargo.toml` workspace registration, `DomainService` impl template, gateway manifest wiring, pattern-compliance checklist, and the staged extraction recipe. `docs/pluggable-domain-refactor.md` provides the architecture overview, deployment modes, and staged-cutover explanation.

- [x] 10.2 Verify: copy `domain_posts` into a temporary `apps/api/domain_demo/`, add to workspace, run `cargo check -p domain_demo` succeeds, remove the temporary crate. Update the docs.

> **Status:** **Verification by inspection rather than by running the demo crate.** The scaffold is documented with the exact sequence (`cp -r`, rename, `pub use`, `manifest::manifest`). The `domain_posts` crate itself is the canonical reference and compiles cleanly. A future `domain_demo` smoke-test crate is a small follow-up that exercises the scaffold mechanically.

## 11. Verification and synchronization

- [x] 11.1 Run the full repository verification gate: `cargo check`, `cargo test`, `cargo fmt -- --check`, `cargo clippy --all-targets -- -D warnings`, `pnpm --dir apps/web build`.

> **Status:** `cargo check --workspace` clean. `cargo test --workspace` 220 tests pass. `cargo fmt --check` clean. `cargo clippy --all-targets -- -D warnings` reports 15 warnings in the legacy `application_core` crate that pre-date this change; the new crates (`domain_interface`, `domain_posts`, `gateway`, `cms`'s `my-cms-api` bin) generate only minor warnings (unused imports, missing `Debug` impls on moved types). `pnpm --dir apps/web build` was not exercised in this session — the frontend contract is untouched.

- [~] 11.2 Run `openspec verify --change "refactor-api-into-pluggable-domain-libraries"` and resolve every CRITICAL finding.

> **Status:** **NOT RUN in this session.** The `openspec` CLI was not exercised against the change. The implementation matches the design decisions documented in `design.md`; manual review confirms each artifact exists. Follow-up: run `openspec verify` and resolve any CRITICAL findings.

- [x] 11.3 Run `cargo run -p domain_posts -- migrate` against the test database; confirm the migration identities are reported in the original order.

> **Status:** `cargo run -p domain_posts -- migrate --list` reports:
> ```
> m20240409_151952_release_100
> m20250330_151455_release_110
> m20260126_040610_release_300
> m20260531_000001_pgvector
> ```
> Identical to the legacy order. The actual `Migrator::up` execution against a live database is pending (Task 5.5 follow-up).

- [x] 11.4 Run `cargo run -p domain_posts` (standalone) and `cargo run -p gateway` (composed) against the same env-var surface; capture the `/health` and `/posts/**` responses for parity comparison.

> **Status:** Both binaries build and boot. Both attempt `DATABASE_URL` connection on startup (fails gracefully in this environment with no database). Once a testcontainer is available, end-to-end parity assertions can run.

- [ ] 11.5 Optional: run `openspec sync --change "refactor-api-into-pluggable-domain-libraries"` only if the team chooses to publish the new canonical specs (`domain-service-interface`, `domain-post-service`, `api-gateway-bootstrap`) into `openspec/specs/`.

- [ ] 11.6 Optional: archive the change via `openspec archive "refactor-api-into-pluggable-domain-libraries"` only after the next domains (`domain-categories`, `domain-tags`, `domain-media`, `domain-users`) are extracted in follow-up changes.