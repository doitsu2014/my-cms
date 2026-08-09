## Why

The `refactor-api-into-pluggable-domain-libraries` change isolated `domain-posts` but left three post-related capabilities sitting in legacy code: **category CRUD** (in `application_core::commands::category::*` + `cms::src/api/category::*`), **the AI model registry** (in `application_core::commands::ai::models::*` + `cms::src/api/ai/models::*`), and **the post translation pipeline** (in `domain_posts::handlers::post::translate::*` + `domain_posts::handlers::vector_store::*`). These are not standalone verticals — they exist to serve posts. Categories are foreign-keyed from `posts.category_id`; translation writes `post_translations` and `translation_jobs` rows whose lifecycle is owned by the post aggregate; the AI model registry exists only to enumerate the models the post translation pipeline can call. Folding them all into `domain_posts` keeps the post vertical slice cohesive, avoids creating extra domain crates for capabilities that are integral to posts, and shrinks the `legacy_bootstrap` binary so the gateway composition serves the full post surface from one Cargo crate.

## What Changes

- Move `application_core::commands::category::{create,read,modify,delete}::*` (1,100 lines) into `domain_posts::handlers::category::*`. Update `domain_posts::handlers::post::create::PostCreateHandler` (which already uses `crate::handlers::tag_helper::TagCreateHandler`) to call `crate::handlers::category::CategoryCreateHandler` directly when creating posts with a default category.
- Move `cms::src/api/category::*` HTTP adapters into `domain_posts::api::category::*`. Update `domain_posts::api::routes(ctx)` to include the category routes in its `Mount::Protected` and `Mount::Administrator` routers.
- Move `application_core::commands::ai::models::*` (128 lines) into `domain_posts::handlers::ai::models::*`. Move `domain_posts::domain::ai::openai_client_from_env` (the existing factory) into `domain_posts::handlers::ai::openai_client_from_env` so the AI subsystem is co-located.
- Move `cms::src/api/ai::models::*` HTTP adapter into `domain_posts::api::ai::models::*`. Add the `/ai/models` route to `domain_posts::api::routes`.
- The post translation pipeline (`domain_posts::handlers::post::translate::*`, `domain_posts::handlers::vector_store::*`) stays where it is — it is already in `domain_posts`. This change just clarifies its ownership in the design and updates the public surface (the post translate module's `mod.rs` becomes the canonical documentation entry point).
- Delete `application_core::commands::category::*`, `application_core::commands::ai::models::*`, `application_core::commands::ai::translate::*`, `application_core::commands::ai::vector_store_pg.rs`, and `application_core::commands::ai::mod.rs`. `application_core::commands::post::*` keeps re-exports from `domain_posts::handlers::post::*` for backward compatibility with `legacy_bootstrap`.
- Delete `cms::src/api/category::*`, `cms::src/api/ai/*`, `cms::src/api/post/*` (already mirrored in `domain_posts::api::post::*`). The legacy `cms::api::*` modules for media, user, and administrator remain (they are out of scope for this change).
- Update `apps/api/src/bin/legacy_bootstrap.rs` to drop the `categories` and `ai/models` routes from `protected_router`. The binary continues to serve `/media/**`, `/users/**`, `/administrator/database/migration`, `/healthz`, and `/graphql/**`. The gateway composition (already serving `domain-posts` routes) gains `/categories/**` and `/ai/models`.
- Update `domain_posts::Cargo.toml`: remove the unused `domain_foundation` and `application_core` workspace entries if present; the crate continues to depend only on `domain_interface` (plus its own infrastructure dependencies). No new Cargo dependencies are introduced.

### Migration CLI surface after the change

```
cargo run -p domain_posts -- migrate --list   # unchanged — 4 migration identities
cargo run -p gateway    -- migrate             # unchanged — runs domain_posts migrations
```

No new migrations are introduced. The four existing migration identities (`m20240409_151952_release_100`, `m20250330_151455_release_110`, `m20260126_040610_release_300`, `m20260531_000001_pgvector`) are preserved exactly.

## Capabilities

### Modified Capabilities

- **`domain-post-service`**: `domain-posts` becomes the single canonical owner of every post-related capability. In addition to post CRUD, the post-translation pipeline, the `VectorStore` pgvector adapter, and the tag helper, it now owns the category CRUD vertical slice (category CRUD transaction, `CategoryType` enum, `category_tags` and `category_translations` join tables), the AI model registry (`ModelsHandler`, `OpenAIModelInfo`, `ModelsListResponse`), and the post translation pipeline. `DomainPostService::register_routes` returns routes covering `/posts/**`, `/posts/{post_id}/translate{,/background}`, `/posts/{post_id}/translate/jobs{,/**}`, `/categories/**`, `/categories/{category_id}`, and `/ai/models`. The post service exposes a single `health()`, `required_env`, `validate_config`, `migrations`, and `startup_health` entry point.

  The capability text in `openspec/changes/refactor-api-into-pluggable-domain-libraries/specs/domain-post-service/spec.md` is updated to reflect the broader ownership. The contract is otherwise unchanged: same routes, same auth roles, same response envelopes, same migration identities.

## Impact

- Affected crates: `apps/api/Cargo.toml`, `apps/api/domain_posts/{Cargo.toml, src/**}`, `apps/api/application_core/src/commands/{category,ai,post}/**`, `apps/api/src/{api/{category,ai,post}/*, bin/legacy_bootstrap.rs, lib.rs}`.
- Affected routes: `/categories/**`, `/categories/{category_id}`, `/ai/models` move from the `legacy_bootstrap` binary to the `my-cms-api` gateway binary. All other routes are unchanged. Traefik rules continue to match the same paths.
- Affected entities: `categories`, `category_tags`, `category_translations`, `sea_orm_active_enums::CategoryType` are physically moved from `application_core/src/entities/*` to `domain_posts/src/entities/*`. `application_core::entities::*` becomes a re-export shim that forwards to `domain_posts::entities::*` for backward compatibility.
- Affected migrations: none. The four migration identities stay where they are (in `domain_posts::migrations::*`). The database `up` history is unchanged.
- Affected tests: `application_core::commands::category::tests::*` and `application_core::commands::ai::models::tests::*` move into `domain_posts::handlers::{category, ai}::tests::*`. The `cargo test --workspace` command continues to pass.
- Affected documentation: `docs/pluggable-domain-refactor.md` updates the "Per-Domain Ownership" table to reflect the consolidated post domain. `docs/api-architecture.md` updates the diagrams to show `/categories/**` and `/ai/models` flowing through the gateway (not the legacy bootstrap). `docs/adding-a-domain.md` is unchanged (the recipe still applies to future domains like `domain_media`, `domain_users`, `domain_administrator`).
- Affected deployment image: the `my-cms-api` binary gains two routes (`/categories/**`, `/ai/models`) and the `legacy_bootstrap` binary loses the same two routes. The total route surface served by `my-cms-api` grows from "post routes + health + GraphQL" to "post routes + category routes + AI routes + health + GraphQL". The `legacy_bootstrap` binary still exists for the not-yet-extracted media, user, and administrator domains.