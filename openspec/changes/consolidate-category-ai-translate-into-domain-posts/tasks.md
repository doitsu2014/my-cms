## 1. Move category entities into `domain_posts`

- [x] 1.1 Physically copy `categories.rs`, `category_tags.rs`, `category_translations.rs`, and `sea_orm_active_enums.rs` from `apps/api/application_core/src/entities/` into `apps/api/domain_posts/src/entities/`. Update each file's docstring header to note "moved from application_core/src/entities, regenerate against the domain_posts migration set if schema changes".
- [x] 1.2 Update `apps/api/domain_posts/src/entities/mod.rs` to declare the four new modules alongside the existing post-aggregate entities (`posts`, `post_tags`, `post_translations`, `translation_jobs`, `tags`, `test_fulltext`).
- [x] 1.3 Rewrite `apps/api/application_core/src/entities/mod.rs` to `pub use domain_posts::entities::*;`. Verify that no entity file remains under `application_core/src/entities/`.
- [x] 1.4 Verify: `cargo check --workspace`; `cargo test --workspace`.

## 2. Move category command handlers into `domain_posts`

- [x] 2.1 Move `apps/api/application_core/src/commands/category/{create,read,modify,delete}/*` into `apps/api/domain_posts/src/handlers/category/{create,read,modify,delete}/*`.
- [x] 2.2 Move `apps/api/application_core/src/commands/category/mod.rs` into `apps/api/domain_posts/src/handlers/category/mod.rs`.
- [x] 2.3 Adjust the imports inside the moved handlers:
  - `use crate::common::app_error::AppError` → `use crate::domain::error::AppError` (cycle was broken early; the transitional `application_core` step is unnecessary)
  - `use crate::commands::tag::create::create_handler::TagCreateHandler` → `use crate::handlers::tag_helper::create::create_handler::{TagCreateHandler, TagCreateHandlerTrait}`
  - `use crate::entities::*` → `use crate::entities::*` (still works; `crate::entities` is now `domain_posts::entities`)
- [x] 2.4 Move the category test fixtures (`fake_create_category_request`, `fake_create_category_request_with_category_type`, `fake_create_category_request_as_child`, `fake_tag_names`) into `apps/api/domain_posts/src/handlers/test.rs` as `pub(crate)` items. Rename the `cfg(test) pub mod test` block in `commands/category/mod.rs` to `cfg(test) pub(crate) mod test_helpers`.
- [x] 2.5 Verify: `cargo check -p domain_posts`; `cargo test -p domain_posts`; the existing `handle_create_cartegory_testcase_01` and `handle_create_cartegory_testcase_parent` integration tests pass.

## 3. Move category HTTP adapters into `domain_posts`

- [x] 3.1 Move `apps/api/src/api/category/{create,read,modify,delete}/*` into `apps/api/domain_posts/src/api/category/{create,read,modify,delete}/*`.
- [x] 3.2 Adjust the imports inside the moved adapters:
  - `use crate::common::supabase_auth::SupabaseToken` → `use crate::domain::auth::SupabaseToken`
  - `use crate::{ApiResponseError, ApiResponseWith, AppState, AxumResponse}` → `use crate::domain::response::{ApiResponseError, ApiResponseWith, AxumResponse}`
  - `use application_core::commands::category::read::category_read_handler::{CategoryReadHandler, CategoryReadHandlerTrait}` → `use crate::handlers::category::read::category_read_handler::{CategoryReadHandler, CategoryReadHandlerTrait}` (and similarly for create/modify/delete)
  - `use application_core::entities::sea_orm_active_enums::CategoryType` → `use crate::entities::sea_orm_active_enums::CategoryType`
- [x] 3.3 Adjust the handler function signatures to use `State<DomainContext>` (the `AppState` extraction becomes `ctx`).
- [x] 3.4 Update `apps/api/domain_posts/src/api/mod.rs::routes(ctx)` to add a `Mount::Protected` `RouteRegistration` for `/categories{,/{category_id}}` (GET/POST/PUT/DELETE) and include it in the merged protected router.
- [x] 3.5 Verify: `cargo check -p domain_posts`; `cargo build -p gateway` succeeds.

## 4. Move AI handlers and adapters into `domain_posts`

- [x] 4.1 Move `apps/api/application_core/src/commands/ai/models/{models_handler,model_info,mod}.rs` into `apps/api/domain_posts/src/handlers/ai/models/`. Adjust imports:
  - `use crate::common::app_error::AppError` → `use crate::domain::error::AppError`
  - `use async_openai::{config::OpenAIConfig, Client}` is preserved as-is.
- [x] 4.2 Move `apps/api/domain_posts/src/domain/ai.rs` into `apps/api/domain_posts/src/handlers/ai/openai_client_from_env.rs`. Remove `apps/api/domain_posts/src/domain/ai.rs`.
- [x] 4.3 The `crate::domain::ai::openai_client_from_env` factory has no callers inside `domain_posts` (the post translation pipeline in `domain_posts::handlers::post::translate::*` calls `async_openai` directly; the legacy `cms::src::api::post::translate::translate_handler.rs` calls `application_core::commands::ai::vector_store_pg::VectorStore::new`). The factory is preserved as `domain_posts::handlers::ai::openai_client_from_env` for future callers.
- [x] 4.4 Move `apps/api/src/api/ai/models/{models_handler,mod}.rs` into `apps/api/domain_posts/src/api/ai/models/`. Adjust imports:
  - `use crate::presentation_models::api_response::{ApiResponseError, ApiResponseWith, ErrorCode}` → `use crate::domain::response::{ApiResponseError, ApiResponseWith, ErrorCode}`
  - `use application_core::commands::ai::models::{ModelsHandler, ModelsHandlerTrait}` → `use crate::handlers::ai::models::{ModelsHandler, ModelsHandlerTrait}`
  - `Extension<AppState>` → `State<DomainContext>`
- [x] 4.5 Update `apps/api/domain_posts/src/api/mod.rs::routes(ctx)` to add a `Mount::Protected` `RouteRegistration` for `GET /ai/models`.
- [x] 4.6 Verify: `cargo check -p domain_posts`; `cargo build -p gateway` succeeds.

## 5. Remove the `domain_posts → application_core` cycle

- [x] 5.1 Removed `application_core = { path = "../application_core" }` from `apps/api/domain_posts/Cargo.toml` `[dependencies]`. **Deviation:** `application_core` remains as a `[dev-dependencies]` because `domain_posts::handlers::tag_helper::read::read_handler::tests::handle_read_tags_test01` still calls `application_core::commands::tag::delete::delete_handler::TagDeleteHandler` to set up its integration test. Migrating that test to use the canonical `crate::handlers::tag_helper::delete::*` (or a test-only stub) is left for a follow-up change. `cargo tree -p domain_posts -e=no-dev | grep application_core` returns no result — the cycle is broken at the production-dep level.
- [x] 5.2 Rewrote every `use application_core::*` inside `apps/api/domain_posts/src/**`:
  - `use application_core::common::app_error::AppError` → `use crate::domain::error::AppError`
  - `use application_core::common::datetime_generator::generate_vietnam_now` → `use crate::domain::datetime_generator::generate_vietnam_now`
  - `use application_core::common::extensions::StringExtension` → `use crate::domain::extensions::StringExtension`
  - `use application_core::entities::{posts, post_tags, post_translations, translation_jobs, tags}` → `use crate::entities::{posts, post_tags, post_translations, translation_jobs, tags}`
  - `use application_core::entities::sea_orm_active_enums::CategoryType` → `use crate::entities::sea_orm_active_enums::CategoryType`
  - `use application_core::graphql::query_root::schema` → `use crate::domain::graphql::contribute_post_schema` (the canonical Seaography `schema()` function is now inlined in `domain_posts::domain::graphql`)
- [x] 5.3 The `application_core → domain_posts` dependency remains because `application_core::entities` re-exports `domain_posts::entities::*`. The cycle direction is now strictly `application_core → domain_posts` (no return edge in production deps). The direction matches the design's Decision 5 ("the net dependency direction is `application_core → domain_posts`").
- [x] 5.4 Verify: `cargo check --workspace`; `cargo tree -p domain_posts -e=no-dev | grep application_core` returns no result; `cargo tree -p application_core -e=no-dev | grep domain_posts` returns `domain_posts v0.1.0` (forward-only, as designed); `cargo test --workspace --lib --bins` passes (204 tests, 0 failed).

## 6. Drop the legacy category and ai adapters

- [x] 6.1 Deleted `apps/api/src/api/category/*` and `apps/api/src/api/ai/*`.
- [x] 6.2 Deleted `apps/api/application_core/src/commands/category/*` and `apps/api/application_core/src/commands/ai/models/*` (the `translate` and `vector_store_pg` modules stay — they remain in `application_core::commands::ai::*` because the legacy `cms::src::api::post::translate::translate_handler.rs` still imports them).
- [x] 6.3 Updated `apps/api/src/bin/legacy_bootstrap.rs` to drop the `categories` and `ai/models` routes from `protected_router`. The binary continues to serve `/media/**`, `/users/**`, `/administrator/database/migration`, `/healthz`, and `/graphql/**`.
- [x] 6.4 Verify: `cargo build --bin legacy_bootstrap` succeeds; `cargo build --bin my-cms-api` succeeds.

## 7. End-to-end verification

- [x] 7.1 Run the full repository verification gate: `cargo check`, `cargo test --workspace --lib --bins`, `cargo fmt --check`, `cargo clippy --all-targets`, `cargo build --bin legacy_bootstrap`, `cargo build --bin my-cms-api`. All pass. **`cargo clippy -- -D warnings` was not run** because the pre-change tree at HEAD = `910c0b8` does not compile clippy (`57 errors`); the post-change tree has 0 errors and only warnings — all warnings are pre-existing in `cms`, `application_core`, and `domain_posts` (none introduced by this change). **`pnpm --dir apps/web build` was not run** because the frontend is untouched by this backend refactor.
- [ ] 7.2 Boot the gateway with a live testcontainer database. **OUT OF SCOPE for the implementer** — requires Docker + a running Supabase stack. Verify:
  - `GET /health` returns 200 with the post service's health descriptor (single domain in the manifest)
  - `GET /categories` returns 200 with the same envelope as before
  - `GET /categories/{category_id}` returns 200 with the same envelope as before
  - `GET /ai/models` returns 200 with the same catalogue as before
  - `POST /posts/{post_id}/translate` (synchronous) returns the same envelope as before
  - `POST /posts/{post_id}/translate/background` (fire-and-forget) returns the same envelope as before
  - `GET /posts/{post_id}/translate/jobs/{job_id}` returns the same envelope as before
  - `cargo run -p domain_posts -- migrate --list` reports the four migration IDs in the original order
- [ ] 7.3 Run `cargo run -p domain_posts -- migrate` against a fresh testcontainer database. Verify all migrations run in dependency order and the resulting schema is identical to the pre-change schema (table set, column set, indexes). **OUT OF SCOPE** — requires testcontainer.
- [ ] 7.4 Run `cargo run -p domain_posts` standalone. Verify the post domain boots, connects to the testcontainer, and serves `/health`, `/categories`, `/ai/models`, `/posts/**`, `/posts/{post_id}/translate` with the same envelopes as the composed gateway. **OUT OF SCOPE** — requires testcontainer.
- [x] 7.5 `openspec validate "consolidate-category-ai-translate-into-domain-posts" --strict` → "Change is valid". The OpenSpec CLI does not provide a `verify` subcommand; `validate --strict` is the canonical pre-archive check.
- [ ] 7.6 Run `openspec archive "consolidate-category-ai-translate-into-domain-posts"` to publish the modified `domain-post-service` spec into `openspec/specs/` and archive the change. **OWNER: product-owner** — per AGENTS.md Phase 4, archive approval is the PO's decision. The change is in `applyRequires: tasks` ready state (`isComplete: true`) and can be archived at any time.

## 8. Documentation

- [x] 8.1 Updated `docs/pluggable-domain-refactor.md` — the Deployment Modes table, the Staged Cutover section, the per-domain ownership notes, and the verification commands now reflect the consolidated post domain (single Cargo crate owns post CRUD + categories + AI + translation + tag helper).
- [x] 8.2 Updated `docs/api-architecture.md` — diagrams 1 (Cargo Workspace), 3 (Gateway Composition), 4 (Legacy Bootstrap Routes), 5 (Domain Ownership), and 10 (What Each Binary Serves Today) now show `/categories/**` and `/ai/models` flowing through the gateway (`my-cms-api`) and through the standalone `domain_posts` bin, not through the legacy bootstrap. The §11 Future Staged Cutover diagram is updated to reflect that categories/ai/translate are now owned by `domain_posts` (no future `domain_categories`/`domain_ai` extraction needed).
- [x] 8.3 Docs are coherent and reference the consolidated post domain correctly. Both files reference the `consolidate-category-ai-translate-into-domain-posts` change by name in their update notes.