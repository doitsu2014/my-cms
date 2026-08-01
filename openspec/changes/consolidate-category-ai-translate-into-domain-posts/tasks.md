## 1. Move category entities into `domain_posts`

- [ ] 1.1 Physically copy `categories.rs`, `category_tags.rs`, `category_translations.rs`, and `sea_orm_active_enums.rs` from `apps/api/application_core/src/entities/` into `apps/api/domain_posts/src/entities/`. Update each file's docstring header to note "moved from application_core/src/entities, regenerate against the domain_posts migration set if schema changes".
- [ ] 1.2 Update `apps/api/domain_posts/src/entities/mod.rs` to declare the four new modules alongside the existing post-aggregate entities (`posts`, `post_tags`, `post_translations`, `translation_jobs`, `tags`, `test_fulltext`).
- [ ] 1.3 Rewrite `apps/api/application_core/src/entities/mod.rs` to `pub use domain_posts::entities::*;`. Verify that no entity file remains under `application_core/src/entities/`.
- [ ] 1.4 Verify: `cargo check --workspace`; `cargo test --workspace`.

## 2. Move category command handlers into `domain_posts`

- [ ] 2.1 Move `apps/api/application_core/src/commands/category/{create,read,modify,delete}/*` into `apps/api/domain_posts/src/handlers/category/{create,read,modify,delete}/*`.
- [ ] 2.2 Move `apps/api/application_core/src/commands/category/mod.rs` into `apps/api/domain_posts/src/handlers/category/mod.rs`.
- [ ] 2.3 Adjust the imports inside the moved handlers:
  - `use crate::common::app_error::AppError` → `use application_core::common::app_error::AppError` (transitional; replaced in Phase 5)
  - `use crate::commands::tag::create::create_handler::TagCreateHandler` → `use crate::handlers::tag_helper::TagCreateHandler`
  - `use crate::entities::*` → `use application_core::entities::*` (transitional)
- [ ] 2.4 Move the category test fixtures (`fake_create_category_request`, `fake_create_category_request_with_category_type`, `fake_create_category_request_as_child`, `fake_tag_names`) into `apps/api/domain_posts/src/handlers/test.rs` as `pub(crate)` items. Rename the `cfg(test) pub mod test` block in `commands/category/mod.rs` to `cfg(test) pub(crate) mod test_helpers`.
- [ ] 2.5 Verify: `cargo check -p domain_posts`; `cargo test -p domain_posts`; the existing `handle_create_cartegory_testcase_01` and `handle_create_cartegory_testcase_parent` integration tests pass.

## 3. Move category HTTP adapters into `domain_posts`

- [ ] 3.1 Move `apps/api/src/api/category/{create,read,modify,delete}/*` into `apps/api/domain_posts/src/api/category/{create,read,modify,delete}/*`.
- [ ] 3.2 Adjust the imports inside the moved adapters:
  - `use crate::common::supabase_auth::SupabaseToken` → `use crate::domain::auth::SupabaseToken`
  - `use crate::{ApiResponseError, ApiResponseWith, AppState, AxumResponse}` → `use crate::domain::response::{ApiResponseError, ApiResponseWith, AxumResponse}`
  - `use application_core::commands::category::read::category_read_handler::{CategoryReadHandler, CategoryReadHandlerTrait}` → `use crate::handlers::category::read::read_handler::{CategoryReadHandler, CategoryReadHandlerTrait}` (and similarly for create/modify/delete)
  - `use application_core::entities::sea_orm_active_enums::CategoryType` → `use crate::entities::sea_orm_active_enums::CategoryType`
- [ ] 3.3 Adjust the handler function signatures to use `State<DomainContext>` (the `AppState` extraction becomes `ctx`).
- [ ] 3.4 Update `apps/api/domain_posts/src/api/mod.rs::routes(ctx)` to add a `Mount::Protected` `RouteRegistration` for `/categories{,/{category_id}}` (GET/POST/PUT/DELETE) and include it in the merged protected router.
- [ ] 3.5 Verify: `cargo check -p domain_posts`; `cargo build -p gateway`; `cargo run -p domain-posts` boots and `curl /categories` returns 200 with the same envelope as before.

## 4. Move AI handlers and adapters into `domain_posts`

- [ ] 4.1 Move `apps/api/application_core/src/commands/ai/models/{models_handler,model_info,mod}.rs` into `apps/api/domain_posts/src/handlers/ai/models/`. Adjust imports:
  - `use crate::common::app_error::AppError` → `use application_core::common::app_error::AppError` (transitional; replaced in Phase 5)
  - `use async_openai::{config::OpenAIConfig, Client}` is preserved as-is.
- [ ] 4.2 Move `apps/api/domain_posts/src/domain/ai.rs` into `apps/api/domain_posts/src/handlers/ai/openai_client_from_env.rs`. Remove `apps/api/domain_posts/src/domain/ai.rs` (it becomes an empty placeholder until removed in Phase 5).
- [ ] 4.3 Update the call site in `apps/api/domain_posts/src/api/post/translate/translate_handler.rs` to use `crate::handlers::ai::openai_client_from_env` instead of `crate::domain::ai::openai_client_from_env`.
- [ ] 4.4 Move `apps/api/src/api/ai/models/{models_handler,mod}.rs` into `apps/api/domain_posts/src/api/ai/models/`. Adjust imports:
  - `use crate::presentation_models::api_response::{ApiResponseError, ApiResponseWith, ErrorCode}` → `use crate::domain::response::{ApiResponseError, ApiResponseWith, ErrorCode}`
  - `use application_core::commands::ai::models::{ModelsHandler, ModelsHandlerTrait}` → `use crate::handlers::ai::models::{ModelsHandler, ModelsHandlerTrait}`
  - `Extension<AppState>` → `State<DomainContext>`
- [ ] 4.5 Update `apps/api/domain_posts/src/api/mod.rs::routes(ctx)` to add a `Mount::Protected` `RouteRegistration` for `GET /ai/models`.
- [ ] 4.6 Verify: `cargo check -p domain_posts`; `cargo build -p gateway`; `curl /ai/models` returns 200 with the same JSON catalogue.

## 5. Remove the `domain_posts → application_core` cycle

- [ ] 5.1 Remove `application_core = { path = "../application_core" }` from `apps/api/domain_posts/Cargo.toml`. Verify no other path dependency on `application_core` remains.
- [ ] 5.2 Rewrite every `use application_core::*` inside `apps/api/domain_posts/src/**`:
  - `use application_core::common::app_error::AppError` → `use crate::domain::error::AppError`
  - `use application_core::common::datetime_generator::generate_vietnam_now` → `use crate::domain::datetime_generator::generate_vietnam_now`
  - `use application_core::common::extensions::StringExtension` → `use crate::domain::extensions::StringExtension`
  - `use application_core::entities::{posts, post_tags, post_translations, translation_jobs, tags}` → `use crate::entities::{posts, post_tags, post_translations, translation_jobs, tags}`
  - `use application_core::entities::sea_orm_active_enums::CategoryType` → `use crate::entities::sea_orm_active_enums::CategoryType`
  - `use application_core::graphql::query_root::schema` → `use crate::domain::graphql::contribute_post_schema` (already the wrapper; calls go through the post-domain entry point)
- [ ] 5.3 Remove the `application_core → domain_posts` dependency (currently unused) from `apps/api/application_core/Cargo.toml`.
- [ ] 5.4 Verify: `cargo check --workspace`; `cargo tree -p domain_posts | grep application_core` returns no result; `cargo tree -p application_core | grep domain_posts` returns no result; `cargo test --workspace` passes.

## 6. Drop the legacy category and ai adapters

- [ ] 6.1 Delete `apps/api/src/api/category/*` and `apps/api/src/api/ai/*`.
- [ ] 6.2 Delete `apps/api/application_core/src/commands/category/*` and `apps/api/application_core/src/commands/ai/*` (already emptied in earlier phases).
- [ ] 6.3 Update `apps/api/src/bin/legacy_bootstrap.rs` to drop the `categories` and `ai/models` routes from `protected_router`. The binary continues to serve `/media/**`, `/users/**`, `/administrator/database/migration`, `/healthz`, and `/graphql/**`.
- [ ] 6.4 Verify: `cargo build --bin legacy_bootstrap` succeeds; `cargo build --bin my-cms-api` succeeds; `curl localhost:8989/categories` against the legacy binary returns 404; `curl localhost:8989/categories` against the gateway binary returns 200.

## 7. End-to-end verification

- [ ] 7.1 Run the full repository verification gate: `cargo check`, `cargo test`, `cargo fmt --check`, `cargo clippy --all-targets -- -D warnings`, `pnpm --dir apps/web build`.
- [ ] 7.2 Boot the gateway with a live testcontainer database. Verify:
  - `GET /health` returns 200 with the post service's health descriptor (single domain in the manifest)
  - `GET /categories` returns 200 with the same envelope as before
  - `GET /categories/{category_id}` returns 200 with the same envelope as before
  - `GET /ai/models` returns 200 with the same catalogue as before
  - `POST /posts/{post_id}/translate` (synchronous) returns the same envelope as before
  - `POST /posts/{post_id}/translate/background` (fire-and-forget) returns the same envelope as before
  - `GET /posts/{post_id}/translate/jobs/{job_id}` returns the same envelope as before
  - `cargo run -p domain_posts -- migrate --list` reports the four migration IDs in the original order
- [ ] 7.3 Run `cargo run -p domain_posts -- migrate` against a fresh testcontainer database. Verify all migrations run in dependency order and the resulting schema is identical to the pre-change schema (table set, column set, indexes).
- [ ] 7.4 Run `cargo run -p domain_posts` standalone. Verify the post domain boots, connects to the testcontainer, and serves `/health`, `/categories`, `/ai/models`, `/posts/**`, `/posts/{post_id}/translate` with the same envelopes as the composed gateway.
- [ ] 7.5 Run `openspec verify --change "consolidate-category-ai-translate-into-domain-posts"` and resolve every CRITICAL finding.
- [ ] 7.6 Run `openspec sync --change "consolidate-category-ai-translate-into-domain-posts"` to publish the modified `domain-post-service` spec into `openspec/specs/`.

## 8. Documentation

- [ ] 8.1 Update `docs/pluggable-domain-refactor.md` to reflect the consolidated post domain (single Cargo crate owns post CRUD + categories + AI + translation + tag helper).
- [ ] 8.2 Update `docs/api-architecture.md` to draw `/categories/**` and `/ai/models` flowing through the gateway (not the legacy bootstrap) in diagrams 1, 5, and 10.
- [ ] 8.3 Verify: docs are coherent and reference the consolidated post domain correctly.