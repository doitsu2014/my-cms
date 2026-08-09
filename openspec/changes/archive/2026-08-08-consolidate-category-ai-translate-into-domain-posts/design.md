## Context

The `refactor-api-into-pluggable-domain-libraries` change shipped `domain-posts` as a self-contained crate and moved the post translation pipeline into `domain_posts::handlers::post::translate::*` and `domain_posts::handlers::vector_store::*`. The category CRUD command handlers stayed in `application_core::commands::category::*` (1,100 lines), and the AI model registry stayed in `application_core::commands::ai::models::*` (128 lines). The category and AI HTTP adapters stayed in `cms::src/api/category::*` and `cms::src/api/ai/models::*`, served by the `legacy_bootstrap` binary.

The user's product decision is that **categories, ai, and translation are integral to the post vertical slice, not separate domains**:

- `categories`, `category_tags`, and `category_translations` exist solely to be referenced from `posts.category_id` and from post translations. The category tree is a post taxonomy. Extracting it into `domain_categories` would force `domain-posts` to depend on `domain_categories` for the `CategoryType` enum (cycle risk) and would split a single cohesive aggregate into two crates.
- The post translation pipeline (OpenAI orchestration, pgvector embeddings, similarity-reuse threshold) is a post-pipeline capability. It writes `post_translations` rows (owned by the post aggregate) and `translation_jobs` rows (lifecycle of post translations). There is no demand for translating non-post content.
- The AI model registry (`/ai/models` endpoint) is a thin catalogue of OpenAI models. Its only consumer is the post translation pipeline. There is no demand for "AI" as a standalone capability.

The original `refactor-api-into-pluggable-domain-libraries/design.md` lists `categories`, `tags`, `media`, `users`, `ai` as future domains to be extracted in follow-up changes. This change revises that plan for `categories`, `ai`, and (the translation pipeline already partially lifted into `domain-posts`) by folding them back into the post domain. The remaining future extractions are `domain_media`, `domain_users`, `domain_administrator`, and `domain_tags` — these have non-post use cases (media is referenced by post thumbnails but also by user avatars; users and administrator routes are clearly not post concerns).

Stakeholders:
- **Content / editorial team**: depends on category CRUD, translation, and `/ai/models` continuing to work. The consolidation keeps these capabilities behind one Cargo crate boundary, simplifying the deployable surface.
- **Backend engineering**: needs the consolidated post crate to compile in isolation; the dependency graph remains acyclic (no `domain-posts → application_core` cycle remains once `application_core::commands::*` is fully emptied of post-related code).
- **AI cost / observability**: the post translation pipeline and the AI model catalogue remain co-located, so per-model cost metrics and per-translation latency metrics share the same observability boundary.
- **DevOps**: one canonical image (`my-cms-api`) serves the full post surface including categories and AI; the `legacy_bootstrap` binary shrinks by two route groups.

Constraints:
- Schema-first migrations only. Generated SeaORM entities are never hand-edited.
- `domain-posts` stays free of `application_core` and `cms` dependencies. The transitive bridges (`pub use application_core::common::app_error::AppError`) that the prior change introduced to keep the legacy shim compiling are removed in this change.
- Migration identities are append-only. The four existing identities are preserved exactly.
- The published `domain_interface` contract stays backward-compatible. No new port traits are added (the cross-domain decoupling rationale that justified port traits in the prior plan does not apply when the consumers and providers live in the same crate).
- The `domain-posts` crate's bin (`cargo run -p domain-posts`) and the gateway's composition both continue to expose `/categories/**`, `/ai/models`, and `/posts/{post_id}/translate{,/background}`.

## Goals / Non-Goals

**Goals:**

- Stand up `domain-posts` as the **single canonical owner** of the post-related vertical slice: postCRUD, post translation, post-related categories, post-related AI model registry, post-related tag helper, post-related GraphQL contribution, post-related migrations, and post-related cross-cutting layers (auth, CORS, cookies, body limit, OpenTelemetry).
- Eliminate the `domain_posts → application_core` cycle documented in the prior change as Task 4.6 deferred work. After this change: `application_core::commands::*` is empty (or holds only compatibility shims for the `legacy_bootstrap` binary's tests); `application_core::entities::*` is a pure re-export shim that forwards to `domain_posts::entities::*`.
- Wire every post-related route through `domain_posts::DomainService::register_routes`. The gateway composition continues to use a single `Box<dyn DomainService>` entry for the post domain.
- Keep the existing route surface (paths, methods, auth roles, response envelopes, error mappings) bit-for-bit compatible. `cargo test --workspace` plus the existing `domain_posts::api::post::*` HTTP path parity assertions remain green.
- Preserve the migration CLI behaviour: `cargo run -p domain_posts -- migrate --list` continues to print the four identities in the original order; `cargo run -p gateway -- migrate` continues to run them.

**Non-Goals:**

- Extracting `domain_media`, `domain_users`, `domain_administrator`, or `domain_tags`. The `legacy_bootstrap` binary remains in place for these three domains; `domain_tags` is a small enough ownership question that it can be deferred until a non-post use case emerges (the post-domain `tag_helper` already exists).
- Replacing SeaORM, Axum, the Seaography GraphQL pipeline, the Supabase auth layer, or the Supabase storage layer.
- Splitting the database. All composed domains continue to share the same PostgreSQL schema.
- Changing the public REST/GraphQL contract, response envelopes, or migration identity ordering.
- Replacing the OpenAI integration or moving the translation pipeline to a separate runtime.
- Publishing `domain_interface` to crates.io (still path-only during the staged refactor).
- Introducing port-trait abstractions (`TagWriter`, `ModelProvider`, `PostTranslationSink`) inside `domain_interface`. The rationale for port traits was cross-domain decoupling; once categories, ai, and translate live in the same crate as the post aggregate, the decoupling is unnecessary.

## Decisions

### Decision 1 — Categories live in `domain_posts::handlers::category::*`

The category CRUD command handlers (`CategoryCreateHandler`, `CategoryReadHandler`, `CategoryModifyHandler`, `CategoryDeleteHandler`) move from `application_core::commands::category::*` into `domain_posts::handlers::category::*`. The category HTTP adapters move from `cms::src/api/category::*` into `domain_posts::api::category::*`. The category entities (`categories`, `category_tags`, `category_translations`, `CategoryType` enum) move from `application_core::entities::*` into `domain_posts::entities::*`.

Why "category in posts" instead of `domain_categories`: the categories table exists only to be referenced from `posts.category_id`. The `CategoryType` enum (`Blog` | `Other`) classifies post categories. The category tree is a post taxonomy. Splitting it into `domain_categories` would force `domain-posts` to depend on `domain_categories` for the `CategoryType` enum (cycle risk) and would split a single cohesive aggregate into two crates without any deployment-time benefit.

**Rejected alternative:** Extract `domain_categories` per the prior plan. Rejected because the user's product decision is that categories are integral to posts.

### Decision 2 — AI model registry lives in `domain_posts::handlers::ai::*`

The `ModelsHandler`, `OpenAIModelInfo`, and `ModelsListResponse` types move from `application_core::commands::ai::models::*` into `domain_posts::handlers::ai::models::*`. The `OpenAIClient` factory (currently in `domain_posts::domain::ai::openai_client_from_env`) moves into `domain_posts::handlers::ai::openai_client_from_env` so the AI subsystem is co-located. The `/ai/models` HTTP adapter moves from `cms::src/api/ai/models::*` into `domain_posts::api::ai::models::*`.

Why "AI in posts" instead of `domain_ai`: the model registry has exactly one consumer — the post translation pipeline. There is no demand for AI as a standalone capability (no other domain in the codebase calls OpenAI). Splitting it into `domain_ai` would create a one-consumer crate.

**Rejected alternative:** Extract `domain_ai` per the prior plan. Rejected because the model's only consumer is the post translation pipeline.

### Decision 3 — Translation pipeline stays in `domain_posts::handlers::post::translate::*`

The post translation pipeline (`PostTranslateHandler`, `TranslatePostRequest`, `TranslatePostResponse`) and the `VectorStore` pgvector adapter stay in `domain_posts::handlers::post::translate::*` and `domain_posts::handlers::vector_store::*`. This change consolidates the documentation (the module's `mod.rs` becomes the canonical entry point) and removes the ambiguity about whether `domain_posts` owns translation or whether translation is a borrowed capability.

The translation pipeline writes to `domain_posts::entities::{post_translations, translation_jobs}` directly. No port trait is needed because the consumer (translation orchestration) and the provider (entity persistence) live in the same crate.

**Rejected alternative:** Extract `domain_translate` per the prior plan. Rejected because translation is a post-pipeline capability; there is no demand for translating non-post content.

### Decision 4 — No port traits in `domain_interface`

The prior plan introduced three port traits (`TagWriter`, `ModelProvider`, `PostTranslationSink`) in `domain_interface::ports` to allow cross-domain decoupling. This change does not introduce any port traits because the consumers and providers live in the same crate:

- The categoryCRUD handler (`CategoryCreateHandler`) and the tag helper (`TagCreateHandler`) both live in `domain_posts`. The category handler calls the tag helper directly through a Rust path import (`crate::handlers::tag_helper::TagCreateHandler`), not through a trait object.
- The AI model registry (`ModelsHandler`) and the translation pipeline (`PostTranslateHandler`) both live in `domain_posts`. The translation pipeline calls the model registry directly through `crate::handlers::ai::models::*`, not through a trait object.
- The translation pipeline and the post entity persistence both live in `domain_posts`. The translation pipeline persists through `domain_posts::entities::*`, not through a port trait.

`domain_interface` stays minimal: `DomainService`, `DomainContext`, `Mount`, `RouteRegistration`, `HealthDescriptor`, `MigrationDescriptor`, `DomainConfigError`. No new exports.

**Rejected alternative:** Introduce port traits preemptively even though everything lives in the same crate. Rejected because the abstraction has no current consumer (no future domain will call into `domain_posts`'s category handler or AI model registry).

### Decision 5 — `application_core::entities::*` becomes a pure re-export shim

`application_core::entities::mod.rs` becomes:

```rust
pub use domain_posts::entities::*;
```

This allows the legacy `cms::api::{media,user,administrator}::*` modules (which still import `application_core::entities::*`) to keep compiling without modification. As `domain_media`, `domain_users`, `domain_administrator` are extracted in follow-up changes, `application_core::entities::*` further shrinks to hold only the entity modules those domains still need (e.g., `media_config` types).

`application_core::commands::mod.rs` becomes empty. The `cms::src/lib.rs` and the `legacy_bootstrap` binary continue to use `cms::api::{media,user,administrator}::*` directly (these modules already exist and are out of scope for this change).

**Rejected alternative:** Delete `application_core` and `migration` entirely. Rejected because `cms::api::{media,user,administrator}::*` and `test_helpers` still depend on `application_core::common::*` (e.g., `app_error::AppError`, `datetime_generator::generate_vietnam_now`).

### Decision 6 — `legacy_bootstrap` loses `/categories/**` and `/ai/models`

The `legacy_bootstrap` binary's `protected_router` and `protected_administrator_router` drop the categoryCRUD routes and the `/ai/models` route. The binary retains `/media/**`, `/users/**`, `/administrator/database/migration`, `/healthz`, `/graphql/**`, and the public routes. The gateway's composed router now serves `/categories/**` and `/ai/models` through `domain_posts`.

**Rejected alternative:** Keep `legacy_bootstrap` as a fallback for `/categories/**` and `/ai/models`. Rejected because keeping two services for the same route violates the gateway composition contract — there should be one canonical handler per route.

### Decision 7 — Test fixtures move to `domain_posts::handlers::test::*`

`fake_create_post_request`, `fake_create_category_request`, `fake_create_category_request_as_child`, `fake_create_category_request_with_category_type`, and `fake_tag_names` move from `application_core::commands::post::test::*` and `application_core::commands::category::test::*` into `domain_posts::handlers::test::*`. The `cfg(test)` `pub mod test` is replaced with `#[cfg(test)] pub(crate) mod test_helpers` so the helper is `pub(crate)` within `domain_posts` but not exported.

The existing `fake_create_post_request` test helper is referenced by `domain_posts::handlers::test::fake_create_post_request` (moved from `application_core::commands::post::test::fake_create_post_request`). The category test helper (`fake_create_category_request`) is referenced by `domain_posts::handlers::category::tests::*` (moved from `application_core::commands::category::test::*`).

**Rejected alternative:** Keep helpers in `application_core` and have each domain depend on it. Rejected because that re-creates the shared-kernel pattern the prior change explicitly rejected.

### Decision 8 — No new migrations

The change is code-only. No new tables, no new columns, no new indexes. The four existing migrations (`m20240409_151952_release_100`, `m20250330_151455_release_110`, `m20260126_040610_release_300`, `m20260531_000001_pgvector`) stay where they are. The database `up` history is unchanged.

**Rejected alternative:** Introduce a `m2026_categories_release_<n>` migration that declares ownership of the categories table family. Rejected because the categories table was created by `m20240409_151952_release_100`; declaring ownership in a later migration is a no-op (the table already exists) and adds noise to the migration history.

## Risks / Trade-offs

- **[Risk]** `domain_posts` becomes a large crate (~6,000 lines across postCRUD, translation, category, AI, tag helper, vector store). Build time for `cargo check -p domain_posts` grows. → **Mitigation:** The crate's internal layout uses one module per concern (`api/post`, `api/category`, `api/ai`, `handlers/post`, `handlers/category`, `handlers/ai`, `handlers/post::translate`, `handlers/vector_store`, `handlers/tag_helper`). Compile times remain proportional to the modules that change; the workspace test suite is unchanged. Long-term: when a non-post use case emerges (e.g., translation for media metadata), the relevant module can be split into its own crate.
- **[Risk]** The `CategoryCreateHandler → TagCreateHandler` cross-module call (within the same crate, not cross-crate) needs the same care as the prior cross-crate call. → **Mitigation:** The existing `domain_posts::handlers::tag_helper::TagCreateHandler` is the canonical tag owner; `domain_posts::handlers::category::create::CategoryCreateHandler` calls it through `crate::handlers::tag_helper::TagCreateHandler`. Both live in the same crate, so the test fixtures (`setup_test_space` etc.) compile. The existing `handle_create_cartegory_testcase_01` and `handle_create_cartegory_testcase_parent` integration tests move into `domain_posts::handlers::category::create::tests` and pass unchanged.
- **[Risk]** The `legacy_bootstrap` binary's tests (which use `cms::api::*` and `application_core::entities::*`) need to keep passing. → **Mitigation:** `application_core::entities::mod.rs` becomes a re-export of `domain_posts::entities::*`. `cms::src/lib.rs` keeps `pub use api::*;`. The legacy binary continues to serve `/media/**`, `/users/**`, `/administrator/database/migration` through the unchanged `cms::api::{media,user,administrator}::*` modules. The legacy binary's tests are integration tests that hit the testcontainer; they continue to pass.
- **[Risk]** The `OpenAIClient` factory is currently in `domain_posts::domain::ai::openai_client_from_env`. Moving it into `domain_posts::handlers::ai::openai_client_from_env` changes the import path used by `domain_posts::api::post::translate::translate_handler.rs`. → **Mitigation:** The file move is mechanical. The call site `crate::domain::ai::openai_client_from_env` becomes `crate::handlers::ai::openai_client_from_env`. Both `domain_posts::handlers::post::translate::*` and `domain_posts::handlers::ai::models::*` consume the factory through the same path import.
- **[Risk]** The `domain-posts::entities::*` module gains the category entities (`categories`, `category_tags`, `category_translations`, `CategoryType`). The module already re-exports from `application_core::entities::*` (per the prior change's Task 4.6). The cycle documentation in `tasks.md` notes that the cycle is broken when `domain_posts → application_core` is removed. → **Mitigation:** This change removes `domain_posts → application_core` from `domain_posts::Cargo.toml`. The `pub use application_core::entities::*;` line in `domain_posts::entities/mod.rs` is replaced by the canonical entity module declarations. The cycle is broken at compile time. The `application_core::entities` module becomes a re-export shim that itself imports from `domain_posts::entities`, which is forward-referencing the consuming crate. To avoid the compile-time cycle, `application_core` is removed from `application_core/Cargo.toml`'s `[dependencies]` section (it does not depend on itself), and `domain_posts/Cargo.toml` is updated to remove the `application_core` path dependency. The net dependency direction is: `application_core → domain_posts` (with `application_core::entities` being a shim that imports `domain_posts::entities`).
- **[Risk]** The `Vec<Box<dyn DomainService>>` gateway manifest does not grow (it still has one entry: `DomainPostService`). The `RouteRegistration.router: Router<DomainContext>` contract is unchanged. The post domain's `register_routes` method is updated to return six `RouteRegistration`s instead of three (one per `Mount` per capability). → **Mitigation:** The `RouteRegistration` API is unchanged. The gateway's `compose_routers` function is unchanged.
- **[Risk]** The `sea-orm generate entity` regeneration is now impossible for `domain_posts::entities::*` (the entities span multiple aggregates: posts, categories, translations). → **Mitigation:** The entities are **not regenerated**; they are physically moved from `application_core::entities::*` to `domain_posts::entities::*` and the file headers are updated to note "moved from application_core, regenerate as a single entity set against the post-domain migration history". When future schema changes need new entities, the regeneration runs against the same `domain_posts::migrations::*` set, producing an identical entity set.

## Migration Plan

### Phase 1 — Move entities into `domain_posts`

1. Physically copy `categories.rs`, `category_tags.rs`, `category_translations.rs`, and `sea_orm_active_enums.rs` from `application_core/src/entities/` into `domain_posts/src/entities/`.
2. Update `domain_posts::src/entities/mod.rs` to declare the four new modules alongside the existing post-aggregate entities (`posts`, `post_tags`, `post_translations`, `translation_jobs`, `tags`, `test_fulltext`).
3. Rewrite `application_core::src/entities/mod.rs` to be a pure re-export shim:
   ```rust
   pub use domain_posts::entities::*;
   ```
4. Remove the `application_core → domain_posts` (currently unused) and `domain_posts → application_core` dependencies from their respective `Cargo.toml` files.
5. Verify: `cargo check --workspace` passes; `cargo test --workspace` passes.

### Phase 2 — Move category command handlers into `domain_posts`

1. Move `application_core::src/commands/category/{create,read,modify,delete}/*` into `domain_posts::src/handlers/category/{create,read,modify,delete}/*`. Adjust `use crate::*` → `use application_core::*` (still valid because `application_core::entities::*` re-exports from `domain_posts::entities::*`, but the path resolves back to the same crate). When `domain_posts → application_core` is removed in Phase 5, rewrite the imports to `crate::domain::error::AppError` and `crate::entities::*`.
2. Move `application_core::src/commands/category/mod.rs` into `domain_posts::src/handlers/category/mod.rs`. Add `pub mod tests { ... }` block (renamed from `mod test`) with the existing test fixtures.
3. Move the category test fixtures (`fake_create_category_request`, `fake_create_category_request_with_category_type`, `fake_create_category_request_as_child`, `fake_tag_names`) into `domain_posts::src/handlers/test.rs`.
4. Verify: `cargo check -p domain_posts`; `cargo test -p domain_posts`; the existing `handle_create_cartegory_testcase_*` integration tests pass with `setup_test_space`.

### Phase 3 — Move category HTTP adapters into `domain_posts`

1. Move `cms::src/api/category/{create,read,modify,delete}/*` into `domain_posts::src/api/category/{create,read,modify,delete}/*`.
2. Adjust the imports inside the moved adapters:
   - `use crate::common::supabase_auth::SupabaseToken` → `use crate::domain::auth::SupabaseToken`
   - `use crate::{ApiResponseError, ApiResponseWith, AppState, AxumResponse}` → `use crate::domain::response::{ApiResponseError, ApiResponseWith, AxumResponse}`
   - `use application_core::commands::category::read::category_read_handler::{CategoryReadHandler, CategoryReadHandlerTrait}` → `use crate::handlers::category::read::read_handler::{CategoryReadHandler, CategoryReadHandlerTrait}` (and similarly for create/modify/delete)
   - `use application_core::entities::sea_orm_active_enums::CategoryType` → `use crate::entities::sea_orm_active_enums::CategoryType`
3. Adjust the handler function signatures to use `State<DomainContext>` (the `AppState` extraction becomes `ctx`).
4. Update `domain_posts::src/api/mod.rs::routes(ctx)` to add a `Mount::Protected` `RouteRegistration` for `/categories{,/{category_id}}` (GET/POST/PUT/DELETE) and include it in the merged protected router.
5. Verify: `cargo check -p domain_posts`; `cargo build -p gateway`; the gateway serves `/categories/**` at port 8989 with the same envelopes as before.

### Phase 4 — Move AI handlers and adapters into `domain_posts`

1. Move `application_core::src/commands/ai/models/{models_handler,model_info,mod}.rs` into `domain_posts::src/handlers/ai/models/`. Adjust imports:
   - `use crate::common::app_error::AppError` → `use application_core::common::app_error::AppError` (transitional; replaced in Phase 5)
   - `use async_openai::{config::OpenAIConfig, Client}` is preserved as-is.
2. Move `domain_posts::src/domain/ai.rs` into `domain_posts::src/handlers/ai/openai_client_from_env.rs`. Remove `domain_posts::src/domain/ai.rs` (it becomes an empty placeholder until removed in Phase 5).
3. Update the call site in `domain_posts::src/api/post/translate/translate_handler.rs` to use `crate::handlers::ai::openai_client_from_env` instead of `crate::domain::ai::openai_client_from_env`.
4. Move `cms::src/api/ai/models/{models_handler,mod}.rs` into `domain_posts::src/api/ai/models/`. Adjust imports:
   - `use crate::presentation_models::api_response::{ApiResponseError, ApiResponseWith, ErrorCode}` → `use crate::domain::response::{ApiResponseError, ApiResponseWith, ErrorCode}`
   - `use application_core::commands::ai::models::{ModelsHandler, ModelsHandlerTrait}` → `use crate::handlers::ai::models::{ModelsHandler, ModelsHandlerTrait}`
   - `Extension<AppState>` → `State<DomainContext>`
5. Update `domain_posts::src/api/mod.rs::routes(ctx)` to add a `Mount::Protected` `RouteRegistration` for `GET /ai/models`.
6. Verify: `cargo check -p domain_posts`; `cargo build -p gateway`; `curl /ai/models` returns 200 with the same JSON catalogue.

### Phase 5 — Remove the `domain_posts → application_core` cycle

1. Remove `application_core = { path = "../application_core" }` from `domain_posts::Cargo.toml`. Verify no other path dependency on `application_core` remains.
2. Rewrite every `use application_core::*` inside `domain_posts::src/**`:
   - `use application_core::common::app_error::AppError` → `use crate::domain::error::AppError`
   - `use application_core::common::datetime_generator::generate_vietnam_now` → `use crate::domain::datetime_generator::generate_vietnam_now`
   - `use application_core::common::extensions::StringExtension` → `use crate::domain::extensions::StringExtension`
   - `use application_core::entities::{posts, post_tags, post_translations, translation_jobs, tags}` → `use crate::entities::{posts, post_tags, post_translations, translation_jobs, tags}`
   - `use application_core::entities::sea_orm_active_enums::CategoryType` → `use crate::entities::sea_orm_active_enums::CategoryType`
   - `use application_core::graphql::query_root::schema` → `use crate::domain::graphql::contribute_post_schema` (already the wrapper; calls go through the post-domain entry point)
3. Remove the `application_core → domain_posts` dependency (currently unused) from `application_core/Cargo.toml`.
4. Verify: `cargo check --workspace`; `cargo tree -p domain_posts | grep application_core` returns no result; `cargo tree -p application_core | grep domain_posts` returns no result; `cargo test --workspace` passes.

### Phase 6 — Drop the legacy category and ai adapters

1. Delete `cms::src/api/category/*` and `cms::src/api/ai/*`.
2. Delete `application_core::src/commands::category/*` and `application_core::src/commands::ai/*` (already emptied in earlier phases).
3. Update `apps/api/src/bin/legacy_bootstrap.rs` to drop the `categories` and `ai/models` routes from `protected_router`. The binary continues to serve `/media/**`, `/users/**`, `/administrator/database/migration`, `/healthz`, and `/graphql/**`.
4. Verify: `cargo build --bin legacy_bootstrap` succeeds; `cargo build --bin my-cms-api` succeeds; `curl localhost:8989/categories` against the legacy binary returns 404; `curl localhost:8989/categories` against the gateway binary returns 200.

### Phase 7 — End-to-end verification

1. Run the full repository verification gate: `cargo check`, `cargo test`, `cargo fmt --check`, `cargo clippy --all-targets -- -D warnings`, `pnpm --dir apps/web build`.
2. Boot the gateway with a live testcontainer database. Verify:
   - `GET /health` returns 200 with the post service's health descriptor (single domain in the manifest)
   - `GET /categories` returns 200 with the same envelope as before
   - `GET /categories/{category_id}` returns 200 with the same envelope as before
   - `GET /ai/models` returns 200 with the same catalogue as before
   - `POST /posts/{post_id}/translate` (synchronous) returns the same envelope as before
   - `POST /posts/{post_id}/translate/background` (fire-and-forget) returns the same envelope as before
   - `GET /posts/{post_id}/translate/jobs/{job_id}` returns the same envelope as before
   - `cargo run -p domain_posts -- migrate --list` reports the four migration IDs in the original order
3. Run `cargo run -p domain_posts -- migrate` against a fresh testcontainer database. Verify all migrations run in dependency order and the resulting schema is identical to the pre-change schema (table set, column set, indexes).
4. Run `cargo run -p domain_posts` standalone. Verify the post domain boots, connects to the testcontainer, and serves `/health`, `/categories`, `/ai/models`, `/posts/**`, `/posts/{post_id}/translate` with the same envelopes as the composed gateway.
5. Run `openspec verify --change "consolidate-category-ai-translate-into-domain-posts"` and resolve every CRITICAL finding.
6. Run `openspec sync --change "consolidate-category-ai-translate-into-domain-posts"` to publish the modified `domain-post-service` spec into `openspec/specs/`.
7. Run `openspec archive "consolidate-category-ai-translate-into-domain-posts"` after the sync step succeeds.

### Rollback strategy

Each phase is independently revertible:
- Phases 1–5: revert by deleting the moved files from `domain_posts/src/**` and restoring them in `application_core/src/**` + `cms::src/api/**`. The dependency direction is restored.
- Phase 6: revert by restoring the deleted `cms::src/api/{category,ai}/*` modules and the corresponding routes in `legacy_bootstrap`.
- The database `up` history is unchanged (no migrations were added). Rollback does not require a database rollback.

## Open Questions

- **Should the `domain_posts::api::category::*` and `domain_posts::api::ai::models::*` HTTP adapters continue to use `Extension<AppState>` (as the legacy handlers do after the prior change's session-4 cleanup), or should they migrate to `State<DomainContext>`?** The latter is consistent with the post-CRUD adapters. Confirm before implementation.
- **Should `domain_posts::handlers::category::create::CategoryCreateHandler` call `crate::handlers::tag_helper::TagCreateHandler` directly, or through a `DomainContext`-injected trait object?** The direct call is simpler and matches the in-crate pattern. The trait object matches the prior plan's port-trait design. Recommend the direct call (no port trait needed inside the same crate) — confirm before implementation.
- **Should the AI model catalogue (`ModelsHandler::get_hardcoded_models`) be moved to a `domain_posts::handlers::ai::models::HARDCODED_MODELS` constant (separate from the handler logic), or stay inline in the handler?** The current code has it inline. Refactoring to a constant is a separate cleanup; out of scope for this change.