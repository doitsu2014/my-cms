## 1. Baseline and canonical ownership

- [x] 1.1 Rebuild/update the code-review graph for the current HEAD and record callers, callees, importers, affected flows, communities, and tests for media, user, and tag handlers; prerequisite: none; verify with graph `get_minimal_context`, `detect_changes`, `get_affected_flows`, `get_impact_radius`, and targeted `query_graph` calls.
- [x] 1.2 Inventory every file, trait, struct, API caller, binary caller, test, re-export, and feature-gated reference under `application_core/src/commands/{media,user,tag}` and compare against `domain_posts`/`domain_auth`; prerequisite: 1.1; verify with `rg` and source review.
- [x] 1.3 Decide and document the single canonical tag implementation for this branch, including whether `commands::post::tags` or the existing `domain_posts::handlers::tag_helper` is authoritative; prerequisite: 1.2; verify no duplicate active owner remains.

## 2. Module relocation and post/tag merge

- [x] 2.1 Add/confirm `commands::post` and its tag submodule declarations while preserving existing `post`, `category`, `ai`, and `common` declarations; prerequisite: 1.3; test-first: add/retain module-resolution compile coverage; verify with `cargo check --workspace`.
- [x] 2.2 Relocate the canonical tag create/read/delete handlers, DTOs, test helpers, and module files under the post domain without changing handler behavior; prerequisite: 2.1; test-first: run existing tag and post tag interaction tests before and after; verify focused cargo tests.
- [x] 2.3 Update post, category, tag API adapters, and all cross-command imports/re-exports to the canonical post tag path; prerequisite: 2.2; verify `rg "commands::tag"` returns no unintended references and `cargo test --workspace` compiles.
- [x] 2.4 Remove the obsolete standalone `commands::tag` declaration/files only after all references are migrated; prerequisite: 2.3; verify no `commands/tag` module is compiled and tag routes still resolve.

## 3. Media domain wiring

- [x] 3.1 Confirm media module declarations, storage adapter, bucket modules, DTOs, cache types, and helpers are under `commands::media`; prerequisite: 1.2; test-first: preserve existing media command tests; verify module inventory.
- [x] 3.2 Update all media and bucket API adapter imports and qualified paths, including cache construction and config references; prerequisite: 3.1; verify representative route tests and `cargo check --workspace`.
- [x] 3.3 Update `AppState` and `legacy_bootstrap` imports without changing fields, initialization semantics, or cache behavior; prerequisite: 3.2; verify application/binary compile and media cache tests.

## 4. User domain wiring

- [x] 4.1 Confirm user CRUD, password reset, DTO, and Supabase admin client modules are declared under `commands::user`; prerequisite: 1.2; test-first: preserve existing user mock-server tests.
- [x] 4.2 Update user API adapter imports and in-module Supabase admin/DTO paths; prerequisite: 4.1; verify user handler tests and auth/error mapping tests.
- [x] 4.3 Update `AppState`, bootstrap, and any crate-level user re-exports while preserving client construction and secret handling; prerequisite: 4.2; verify `cargo check --workspace`.

## 5. Cross-cutting verification and cleanup

- [x] 5.1 Run exhaustive forbidden-path and architecture searches for `commands::tag`, stale `commands::media/user` paths, duplicate tag implementations, generated-entity edits, and business logic in API handlers; prerequisite: 2.4, 3.3, 4.3; verify with `rg` and diff review.
- [x] 5.2 Inspect and update GraphQL/Seaography or crate wiring only if source search proves a moved module path is referenced; preserve schema and operation snapshots; prerequisite: 5.1; verify GraphQL build/schema checks.
- [x] 5.3 Re-run graph impact analysis and confirm expected flows, callers, tests, bridge/hub risks, and no unexplained high-risk edges; prerequisite: 5.2; verify graph `detect_changes`, `get_affected_flows`, `get_impact_radius`, and `tests_for`.
- [x] 5.4 Run the complete verification gate: `openspec verify --change "split-media-and-user-domains-merge-tags-into-posts"`, `cargo check --workspace`, `cargo test --workspace`, `cargo fmt --all -- --check`, `cargo clippy --workspace --all-targets --all-features`, and `pnpm --dir apps/web build`; prerequisite: 5.3.
- [x] 5.5 Review the diff for public route/schema/request/response stability and confirm no migrations, entities, or data changes; prerequisite: 5.4; verify with `git diff --check`, targeted contract tests, and architecture review.

## 6. Scaffold `domain_media` crate

- [x] 6.1 Add `domain_media` member to workspace `apps/api/Cargo.toml` (path = "../domain_media").
- [x] 6.2 Create `apps/api/domain_media/Cargo.toml` — lib only, deps: `sea-orm`, `tokio`, `tracing`, `serde`, `wiremock` (dev), `domain_interface`, `application_core` (dev), `test_helpers` (dev). Mirror `domain_auth/Cargo.toml` shape.
- [x] 6.3 Create `apps/api/domain_media/src/lib.rs` exposing `pub mod domain; pub mod entities; pub mod handlers; pub mod observability;`.
- [x] 6.4 Create `apps/api/domain_media/src/domain/error.rs` mirroring `domain_posts/src/domain/error.rs`.
- [x] 6.5 Create `apps/api/domain_media/src/entities/{mod.rs,media.rs}` re-exporting canonical SeaORM Media entities (no regeneration; copy the re-export pattern from `domain_posts/src/entities/mod.rs`).
- [x] 6.6 Create `apps/api/domain_media/src/observability/mod.rs` with `init_tracing()` pattern from `domain_auth`.

## 7. Move media handlers into `domain_media`

Move in this dependency order (each sub-task is one directory or file move + import update). Do NOT change file contents beyond path-qualified imports.

- [x] 7.1 Move `bucket/dto.rs` → `domain_media/src/handlers/bucket/dto.rs`. Update internal `crate::commands::media::bucket::dto::*` paths.
- [x] 7.2 Move `bucket/access/access_cache.rs` → `domain_media/src/handlers/bucket/access/access_cache.rs`.
- [x] 7.3 Move `bucket/access/access_handler.rs` → `domain_media/src/handlers/bucket/access/access_handler.rs`.
- [x] 7.4 Move `bucket/{create,update,get,list,delete,empty}/...` (mod.rs + *_handler.rs per subcommand) → `domain_media/src/handlers/bucket/{create,update,get,list,delete,empty}/...`.
- [x] 7.5 Move `bucket/mod.rs` → `domain_media/src/handlers/bucket/mod.rs`. Verify `BucketAccessPolicy` re-export.
- [x] 7.6 Move `create/{mod.rs,create_handler.rs}` → `domain_media/src/handlers/create/...`.
- [x] 7.7 Move `delete/{mod.rs,delete_handler.rs}` → `domain_media/src/handlers/delete/...`.
- [x] 7.8 Move `list/{mod.rs,list_handler.rs}` → `domain_media/src/handlers/list/...`.
- [x] 7.9 Move `read/{mod.rs,read_handler.rs,read_response.rs,metadata_handler.rs}` → `domain_media/src/handlers/read/...`.
- [x] 7.10 Move `supabase_storage.rs` → `domain_media/src/handlers/supabase_storage.rs`. Confirm 1500+ lines + 25+ wiremock tests survive intact.
- [x] 7.11 Create `domain_media/src/handlers/mod.rs` declaring the seven submodules and re-exporting `MediaConfig`, `MediaModel`, `MediaMetadata`, `is_supported_content_type`, `is_image_content_type`, `CachedMedia`, `MediaCacheKey`, `BucketAccessPolicy`.
- [x] 7.12 Delete `apps/api/application_core/src/commands/media/` directory.

## 8. Wire `domain_media` into API + bootstrap

- [x] 8.1 Update `apps/api/src/lib.rs:6-9` — replace `use application_core::commands::media::{...}` with `use domain_media::{MediaConfig, CachedMedia, MediaCacheKey, BucketAccessPolicy, MediaModel, MediaMetadata, ...};` and add `domain_media` to imports.
- [x] 8.2 Confirm `apps/api/src/lib.rs:18-26` `AppState` fields resolve (`media_config`, `media_cache`, `bucket_visibility_cache`) — no edits expected.
- [x] 8.3 Update `apps/api/src/bin/legacy_bootstrap.rs:5` — retarget to `domain_media::handlers::...`.
- [x] 8.4 Retarget every `use application_core::commands::media::*` import in `apps/api/src/api/media/**` to `use domain_media::handlers::*` (run `rg "application_core::commands::media" apps/api/src/api` first to enumerate).
- [x] 8.5 Remove `pub mod media;` from `apps/api/application_core/src/commands/mod.rs`.
- [x] 8.6 `cargo check --workspace --all-targets` — must be green.

## 9. Scaffold `domain_user` crate

- [x] 9.1 Add `domain_user` member to workspace `apps/api/Cargo.toml`.
- [x] 9.2 Create `apps/api/domain_user/Cargo.toml` — lib only, deps same as 6.2.
- [x] 9.3 Create `apps/api/domain_user/src/lib.rs` exposing `pub mod domain; pub mod dto; pub mod handlers; pub mod observability;`.
- [x] 9.4 Create `apps/api/domain_user/src/domain/error.rs` mirroring `domain_posts/src/domain/error.rs`.
- [x] 9.5 Create `apps/api/domain_user/src/observability/mod.rs` with `init_tracing()` pattern.

## 10. Move user handlers into `domain_user`

- [x] 10.1 Move `dto.rs` → `domain_user/src/dto.rs` (AppUserModel, BAN_DURATION, is_recognised_role).
- [x] 10.2 Move `create/{mod.rs,create_request.rs,create_handler.rs}` → `domain_user/src/handlers/create/...`.
- [x] 10.3 Move `modify/{mod.rs,modify_request.rs,modify_handler.rs}` → `domain_user/src/handlers/modify/...`.
- [x] 10.4 Move `read_one/{mod.rs,read_one_handler.rs}` → `domain_user/src/handlers/read_one/...`.
- [x] 10.5 Move `read_list/{mod.rs,read_list_handler.rs}` → `domain_user/src/handlers/read_list/...`.
- [x] 10.6 Move `delete/{mod.rs,delete_handler.rs}` → `domain_user/src/handlers/delete/...`.
- [x] 10.7 Move `reset_password/{mod.rs,reset_password_request.rs,reset_password_handler.rs}` → `domain_user/src/handlers/reset_password/...`.
- [x] 10.8 Move `supabase_admin_client.rs` → `domain_user/src/handlers/supabase_admin_client.rs` (646 lines incl. wiremock tests).
- [x] 10.9 Create `domain_user/src/handlers/mod.rs` declaring the seven submodules.
- [x] 10.10 Delete `apps/api/application_core/src/commands/user/` directory.

## 11. Wire `domain_user` into API + bootstrap

- [x] 11.1 Update `apps/api/src/lib.rs:10` — replace `use application_core::commands::user::supabase_admin_client::SupabaseAdminClient;` with `use domain_user::handlers::supabase_admin_client::SupabaseAdminClient;`.
- [x] 11.2 Confirm `apps/api/src/lib.rs:18-26` `AppState.supabase_admin_client` resolves.
- [x] 11.3 Update `apps/api/src/bin/legacy_bootstrap.rs:9` — retarget.
- [x] 11.4 Retarget every `use application_core::commands::user::*` import in `apps/api/src/api/user/**` to `use domain_user::handlers::*`.
- [x] 11.5 Remove `pub mod user;` from `apps/api/application_core/src/commands/mod.rs`.
- [x] 11.6 `cargo check --workspace --all-targets` — must be green.

## 12. Cross-cutting verification (media + user extraction)

- [x] 12.1 `rg "application_core::commands::(media|user)" apps` — zero live matches (doc-comment historical references in `domain_posts/src/handlers/**` are tolerated).
- [x] 12.2 `rg "domain_media|domain_user" apps` — matches canonical paths only (`apps/api/domain_media/**`, `apps/api/domain_user/**`, plus retargeted API/import sites).
- [x] 12.3 `cargo test --workspace --lib --no-fail-fast` — counts ≥ baseline per crate (application_core ≥ 128, cms ≥ 16, domain_auth ≥ 28, domain_interface ≥ 7, domain_posts ≥ 35, domain_media = new count, domain_user = new count).
- [x] 12.4 `cargo fmt --all -- --check` — clean.
- [x] 12.5 `cargo clippy --workspace --all-targets --all-features` — no new errors vs. baseline (12 pre-existing `domain_posts` lints allowed).
- [x] 12.6 `pnpm --dir apps/web build` — green.
- [x] 12.7 `git diff --check` — clean.
- [x] 12.8 Code-review-graph gate: `build_or_update_graph_tool`, `get_minimal_context`, `detect_changes`, `get_affected_flows`, `tests_for` on representative media + user handlers, `get_impact_radius max_depth=3`, `find_large_functions_tool` (flag any handler > 50 lines introduced by the move). Document or resolve material findings.

## 13. OpenSpec archive prep

- [x] 13.1 `openspec status --change "split-media-and-user-domains-merge-tags-into-posts" --json` — all artifacts `done`, `isComplete: true`, `applyRequires: [tasks]`, all 60+ tasks ticked.
- [x] 13.2 `openspec validate "split-media-and-user-domains-merge-tags-into-posts" --type change --strict --json` — `valid: true`.
- [x] 13.3 Hand off to PO for: (a) commit, (b) `openspec sync` (review `tags-domain` REMOVED carefully), (c) `openspec archive`.

## 14. Final verification commands

```bash
openspec status --change "split-media-and-user-domains-merge-tags-into-posts" --json
openspec validate "split-media-and-user-domains-merge-tags-into-posts" --type change --strict --json
cargo check --workspace --all-targets
cargo test --workspace --lib --no-fail-fast
cargo fmt --all -- --check
cargo clippy --workspace --all-targets --all-features -- -D warnings   # 12 pre-existing baseline allowed
pnpm --dir apps/web build
git diff --check
rg "application_core::commands::(media|user)" apps   # zero live matches
rg "domain_media|domain_user" apps                   # canonical paths only
```

After all pass, hand off to PO for `openspec sync` and `openspec archive`.