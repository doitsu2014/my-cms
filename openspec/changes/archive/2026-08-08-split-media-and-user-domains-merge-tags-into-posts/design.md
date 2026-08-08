## Context

### Current-state evidence

The checked-out source already declares `commands::media`, `commands::user`, and legacy `commands::tag` in `apps/api/application_core/src/commands/mod.rs:1-3`. Media owns bucket CRUD/access, media create/list/read/delete/metadata, `SupabaseStorage`, `MediaConfig`, media DTOs, and content-type helpers (`apps/api/application_core/src/commands/media/mod.rs:4-49`). User owns CRUD, password reset, DTOs, and `SupabaseAdminClient` (`apps/api/application_core/src/commands/user/mod.rs:1-8`). Tags currently own create/read/delete and test helpers (`apps/api/application_core/src/commands/tag/mod.rs:1-22`).

The current branch also contains `apps/api/domain_posts/src/handlers/tag_helper/` and post handlers that already call tag handlers, while legacy tag source remains in `application_core`; this is a source-policy mismatch requiring reconciliation before implementation, not an assumption that both copies are runtime-active. Existing API callers include media adapters under `apps/api/src/api/media/**`, user adapters under `apps/api/src/api/user/**`, and tag adapters must be located from the current branch with a final exhaustive search because no `commands::tag` caller was found under `apps/api/src/api` in the initial scan. `AppState` directly depends on media cache/config and the user Supabase admin client (`apps/api/src/lib.rs:6-26`).

### Graph gate

`get_minimal_context(task="split-media-and-user-domains-merge-tags-into-posts")` succeeded. It reported 1,210 nodes, 11,236 edges, 295 files, medium risk 0.65, four test gaps, touched communities `media-bucket`, `components-handle`, and `read-api`, and flows `App`, `handleCheckAuth`, and `handleRefreshToken`. The graph is stale relative to HEAD (`built_on_branch=main`, built SHA differs from head), so implementation must rebuild/update it and use targeted callers/callees/imports/flow queries for the final change. The returned top entities were auth/job-test related rather than direct command handlers; therefore no unreported hub/bridge claim is made.

## Goals / Non-Goals

**Goals:**

- Make media and user ownership explicit under dedicated command domain modules.
- Make tags a post-owned submodule, preferably `commands::post::tags/` where that module exists; if the active architecture has already moved post commands to `domain_posts`, reconcile the legacy copy rather than creating two owners.
- Preserve all public contracts, layering, handler traits, tracing, errors, tests, and runtime dependencies.
- Remove stale legacy module declarations and imports without editing generated entities.

**Non-Goals:**

- No endpoint, GraphQL schema, DTO shape, auth policy, storage behavior, database schema, migration, entity, or data change.
- No new abstraction, dependency, compatibility alias, or public API version.
- No implementation code in this change.

## Decisions

### 1. Use domain modules, not compatibility aliases

**Decision:** Update module declarations and all consumers to the target paths, then remove the obsolete tag declaration.

**Rationale:** Compatibility aliases would preserve the legacy boundary and undermine the architectural objective.

**Alternative rejected:** Re-export `commands::tag` indefinitely; lower short-term churn but leaves two conceptual ownership paths and masks stale callers.

### 2. Nest tags under posts

**Decision:** Place tag commands under the active post domain's tag helper/submodule, with one canonical implementation. Prefer `commands::post::tags/` for the requested target; if the current domain extraction means `domain_posts::handlers::tag_helper` is already canonical, delete/reconcile the duplicate application-core copy and update callers to that canonical post-domain export.

**Rationale:** Existing post create/modify code already invokes tag handlers, proving post/tag coupling in source (`apps/api/domain_posts/src/handlers/post/create/create_handler.rs:10,34`; `.../post/modify/modify_handler.rs:18,47`).

**Alternative rejected:** Keep `commands::tag` and merely document it as a sub-concept; it does not satisfy the boundary refactor.

### 3. Preserve handler and adapter responsibilities

**Decision:** Relocation is path/wiring-only. API handlers continue extraction, auth, conversion, and response mapping; command handlers retain business logic, `Trait + Struct`, `#[instrument]`, and `AppError`.

**Alternative rejected:** Consolidate logic into API modules; violates strict layering and increases test coupling.

### 4. No database migration

**Decision:** Do not alter migrations, entities, tables, indexes, or data.

**Rationale:** This is an in-process Rust module relocation. SeaORM generated entities remain untouched.

### 5. Reconcile branch overlap before moving files

**Decision:** Before edits, inspect active OpenSpec changes and current crates (`application_core`, `domain_posts`, `domain_auth`) and establish one source of truth per handler. The engineer must not blindly copy legacy files over an already-migrated implementation.

**Rationale:** Recent commits and source show ongoing extraction work; duplicate handlers could otherwise compile inconsistently or be silently unused.

## Target structure

Under the requested `application_core/src/commands/` boundary, the intended declarations are:

- `category/` unchanged
- `post/` unchanged plus `post/tags/{create,read,delete}` (or the already-canonical post-domain equivalent)
- `media/` containing current media, bucket, and storage modules
- `user/` containing current user, DTO, reset-password, and admin-client modules
- `ai/` unchanged
- `common/` unchanged
- no `tag/` declaration

The exact file list must be derived by the engineer from the current branch and existing domain crates; generated entities are excluded.

## Affected flows and callers

- Media: `apps/api/src/api/media/create/create_handler.rs:1-2`; `.../list/list_handler.rs:1-2,36`; `.../read/read_handler.rs:1-8,344-388`; `.../read/metadata_handler.rs:1-5`; `.../delete/delete_handler.rs:1-5`; bucket adapters under `apps/api/src/api/media/bucket/{create,update,get,list,delete,empty}/...:2`; `apps/api/src/lib.rs:6-9`; `apps/api/src/bin/legacy_bootstrap.rs:5`.
- User: `apps/api/src/api/user/{create,modify,read_one,read_list,delete,reset_password}/**/*:1-3`; `apps/api/src/lib.rs:10`; `apps/api/src/bin/legacy_bootstrap.rs:9`.
- Tags: no legacy `commands::tag` API caller was found in the initial `apps/api/src/api` scan; current post-domain callers are `apps/api/domain_posts/src/handlers/post/create/create_handler.rs:10,34`, `.../post/modify/modify_handler.rs:18,47`, and category handlers at `.../category/create/create_handler.rs:13,36` and `.../category/modify/modify_handler.rs:21,48`. Final `rg`/graph search is mandatory before implementation because the branch contains duplicate historical paths.
- Cross-domain coupling: media uses storage and bucket access/cache internally (`media/mod.rs:4-13`; `supabase_storage.rs:1`; bucket access); user handlers share `SupabaseAdminClient` and DTO paths (`user/*`); tags call tag read from tag create (`tag/create/create_handler.rs:8,43`) and post/category handlers call tag creation. No media-to-user or post-to-media command call was established by the initial search.

## Migration / Rollout / Rollback

1. Rebuild/update the code graph and inventory current canonical handlers.
2. Add/confirm target module declarations and move files without changing contents.
3. Move/reconcile tag handlers under post ownership; update intra-module imports and tests.
4. Update API, `AppState`, bootstrap, crate re-exports, and any GraphQL/Seaography wiring only where paths require it; do not change schemas.
5. Run focused tests, graph impact review, and full verification.
6. Roll out as a normal application binary change; no database deployment step.
7. Roll back by reverting the source commit if compilation or runtime smoke tests fail. Because no data changes occur, rollback has no data-loss operation. Do not retain a compatibility alias unless a hidden consumer is found and explicitly approved.


## Media domain extraction

### Target layout

```
apps/api/domain_media/
├── Cargo.toml                          # crate = "domain_media", lib only
└── src/
    ├── lib.rs                          # pub mod domain; pub mod entities; pub mod handlers; pub mod observability;
    ├── domain/
    │   └── error.rs                    # AppError — mirrors domain_posts::domain::error::AppError
    ├── entities/
    │   ├── mod.rs                      # pub use sea_orm::entity::prelude::*;
    │   └── media.rs                    # re-export of canonical SeaORM Media entities (no regeneration)
    ├── handlers/
    │   ├── mod.rs                      # pub mod bucket; pub mod create; pub mod delete; pub mod list; pub mod read; pub mod supabase_storage;
    │   ├── bucket/...                  # moved wholesale
    │   ├── create/...                  # moved wholesale
    │   ├── delete/...                  # moved wholesale
    │   ├── list/...                    # moved wholesale
    │   ├── read/...                    # moved wholesale (incl. metadata_handler.rs)
    │   └── supabase_storage.rs         # moved whole (1500+ lines); split deferred to follow-up
    └── observability/
        └── mod.rs                      # tracing init pattern mirroring domain_auth
```

### File-to-path mapping

| Source (application_core::commands::media) | Destination (domain_media::handlers) |
|---|---|
| `mod.rs` (top-level re-exports) | `src/handlers/mod.rs` + `src/lib.rs` |
| `bucket/mod.rs`, `bucket/dto.rs` | `src/handlers/bucket/mod.rs`, `src/handlers/bucket/dto.rs` |
| `bucket/access/access_cache.rs` | `src/handlers/bucket/access/access_cache.rs` |
| `bucket/access/access_handler.rs` | `src/handlers/bucket/access/access_handler.rs` |
| `bucket/{create,update,get,list,delete,empty}/{mod.rs,*_handler.rs}` | `src/handlers/bucket/{create,update,get,list,delete,empty}/{mod.rs,*_handler.rs}` |
| `create/{mod.rs,create_handler.rs}` | `src/handlers/create/{mod.rs,create_handler.rs}` |
| `delete/{mod.rs,delete_handler.rs}` | `src/handlers/delete/{mod.rs,delete_handler.rs}` |
| `list/{mod.rs,list_handler.rs}` | `src/handlers/list/{mod.rs,list_handler.rs}` |
| `read/{mod.rs,read_handler.rs,read_response.rs,metadata_handler.rs}` | `src/handlers/read/{mod.rs,read_handler.rs,read_response.rs,metadata_handler.rs}` |
| `supabase_storage.rs` | `src/handlers/supabase_storage.rs` |
| `MediaConfig`, `MediaModel`, `MediaMetadata`, `is_supported_content_type`, `is_image_content_type` (currently in `media/mod.rs`) | `src/lib.rs` (top-level types) or `src/handlers/mod.rs` |
| `CachedMedia`, `MediaCacheKey`, `BucketAccessPolicy` (in `bucket/access/*`) | `src/handlers/bucket/access/*` |

### Wiring updates

- `apps/api/src/lib.rs:6-9` — replace `use application_core::commands::media::{...}` with `use domain_media::{MediaConfig, CachedMedia, MediaCacheKey, BucketAccessPolicy, ...};`
- `apps/api/src/lib.rs:18-26` `AppState` — fields unchanged; types resolve from `domain_media`.
- `apps/api/src/bin/legacy_bootstrap.rs:5` — retarget to `domain_media::handlers::...`.
- All `apps/api/src/api/media/**` handlers — retarget `use application_core::commands::media::*` to `use domain_media::handlers::*`.
- Remove `pub mod media;` from `apps/api/application_core/src/commands/mod.rs` after the move. `application_core::common::app_error::AppError` becomes a re-export shim.

## User domain extraction

### Target layout

```
apps/api/domain_user/
├── Cargo.toml                          # crate = "domain_user", lib only
└── src/
    ├── lib.rs                          # pub mod domain; pub mod dto; pub mod handlers; pub mod observability;
    ├── domain/
    │   └── error.rs                    # AppError — mirrors domain_posts::domain::error::AppError
    ├── dto.rs                          # AppUserModel, BAN_DURATION, is_recognised_role — moved from commands::user::dto.rs
    ├── handlers/
    │   ├── mod.rs                      # pub mod create; pub mod delete; pub mod modify; pub mod read_list; pub mod read_one; pub mod reset_password; pub mod supabase_admin_client;
    │   ├── create/...                  # moved wholesale
    │   ├── modify/...                  # moved wholesale
    │   ├── read_one/...                # moved wholesale
    │   ├── read_list/...               # moved wholesale
    │   ├── delete/...                  # moved wholesale
    │   ├── reset_password/...          # moved wholesale
    │   └── supabase_admin_client.rs    # moved whole (646 lines)
    └── observability/
        └── mod.rs                      # tracing init pattern mirroring domain_auth
```

### File-to-path mapping

| Source (application_core::commands::user) | Destination (domain_user) |
|---|---|
| `mod.rs` (declares 8 submodules + re-exports) | `src/handlers/mod.rs` + `src/lib.rs` |
| `dto.rs` | `src/dto.rs` |
| `create/{mod.rs,create_request.rs,create_handler.rs}` | `src/handlers/create/{mod.rs,create_request.rs,create_handler.rs}` |
| `modify/{mod.rs,modify_request.rs,modify_handler.rs}` | `src/handlers/modify/{mod.rs,modify_request.rs,modify_handler.rs}` |
| `read_one/{mod.rs,read_one_handler.rs}` | `src/handlers/read_one/{mod.rs,read_one_handler.rs}` |
| `read_list/{mod.rs,read_list_handler.rs}` | `src/handlers/read_list/{mod.rs,read_list_handler.rs}` |
| `delete/{mod.rs,delete_handler.rs}` | `src/handlers/delete/{mod.rs,delete_handler.rs}` |
| `reset_password/{mod.rs,reset_password_request.rs,reset_password_handler.rs}` | `src/handlers/reset_password/{mod.rs,reset_password_request.rs,reset_password_handler.rs}` |
| `supabase_admin_client.rs` | `src/handlers/supabase_admin_client.rs` |

### Wiring updates

- `apps/api/src/lib.rs:10` — replace `use application_core::commands::user::supabase_admin_client::SupabaseAdminClient;` with `use domain_user::handlers::supabase_admin_client::SupabaseAdminClient;`
- `apps/api/src/lib.rs:18-26` `AppState` — `supabase_admin_client` field type unchanged; resolves from `domain_user`.
- `apps/api/src/bin/legacy_bootstrap.rs:9` — retarget to `domain_user::handlers::supabase_admin_client::SupabaseAdminClient`.
- All `apps/api/src/api/user/**` handlers — retarget `use application_core::commands::user::*` to `use domain_user::handlers::*`.
- Remove `pub mod user;` from `apps/api/application_core/src/commands/mod.rs` after the move.

## Cross-crate dependencies (no circular deps)

- `domain_media` → `domain_interface` only
- `domain_user` → `domain_interface` only
- `application_core` → may depend on `domain_media`, `domain_user`, `domain_posts`, `domain_auth`, `domain_interface` (read model types)
- `domain_*` crates **must NOT** depend on `application_core` (except as dev-dep for tests, mirroring `domain_posts`)

## Rollback

Revert the source commit. No DB rollback needed. `application_core::commands::{media,user}` directories are restored intact.

## Risks / Trade-offs

- **[Risk] Duplicate legacy/domain_posts implementations** → establish canonical ownership before moving; compile with workspace-wide checks and remove only the obsolete copy.
- **[Risk] Missed path-qualified imports in binaries/tests** → exhaustive `rg`, graph callers/importers, and `cargo check --workspace`.
- **[Risk] Accidental public contract drift** → add route/API smoke checks and compare GraphQL schema output where applicable.
- **[Risk] Cache/config wiring breakage** → compile and run media handler tests plus AppState construction tests.
- **[Risk] Stale graph** → update graph before and after implementation; treat current graph summary as directional only.

## Testing strategy

- Unit tests: run relocated media, user, and tag command tests unchanged, repairing only module paths.
- API/route tests: cover representative media, bucket, user, and tag adapters and verify status/error mapping.
- Workspace compile: ensure all binaries, crates, tests, and feature configurations resolve imports.
- GraphQL: compare schema/introspection or existing GraphQL tests; no operation or shape changes are permitted.
- Static architecture checks: search for `commands::tag`, forbidden API business logic, and edits under generated entities.
- Final gate: `openspec verify --change "split-media-and-user-domains-merge-tags-into-posts"`, `cargo check --workspace`, `cargo test --workspace`, `cargo fmt --all -- --check`, `cargo clippy --workspace --all-targets --all-features`, and `pnpm --dir apps/web build`.

## Open Questions

- Which copy is canonical on this branch: legacy `application_core/src/commands/tag` or `domain_posts/src/handlers/tag_helper`? The implementation owner must resolve this before moving files.
- Does the requested final layout refer only to `application_core` or also require completion/cleanup of the parallel `domain_posts` extraction? Scope should remain internal unless PO approves broader crate restructuring.
- Are any external consumers compiled outside this workspace that import `application_core::commands::*` directly? If yes, provide a migration window or approve the breaking internal Rust path change.
