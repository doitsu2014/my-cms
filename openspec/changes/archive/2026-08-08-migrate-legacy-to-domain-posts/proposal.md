## Why

The `domain_posts` crate is already the canonical home for post-related Rust
code: it owns the post CRUD handlers, the post translation pipeline, the
pgvector `VectorStore`, the post GraphQL contribution, the post-relevant
SeaORM entities, the four post migrations, and a `DomainPostService`
registration with the gateway. The `refactor-api-into-pluggable-domain-libraries`
change introduced `domain_posts` and the `consolidate-category-ai-translate-into-domain-posts`
change folded categories and the AI model registry into the same crate.

What is left is a thin layer of *legacy* post-related code in the original
`application_core`, `migration`, and `cms` (legacy bootstrap) trees that
*duplicates* the canonical code. Concretely:

- `apps/api/application_core/src/commands/post/{create,read,modify,delete}/*` —
  the four legacy post command handlers (~570 lines) duplicate the
  canonical `domain_posts::handlers::post::{create,read,modify,delete}::*`.
- `apps/api/application_core/src/commands/ai/translate/*` and
  `apps/api/application_core/src/commands/ai/vector_store_pg.rs` — the
  legacy translation pipeline and pgvector `VectorStore` (~1,600 lines)
  duplicate `domain_posts::handlers::post::translate::*` and
  `domain_posts::handlers::vector_store::*`.
- `apps/api/src/api/post/{create,read,modify,delete,translate}/*` — the
  legacy `cms` HTTP adapters still import the legacy `application_core`
  command handlers, not `domain_posts`.
- `apps/api/src/api/delete/delete_handler.rs` and
  `apps/api/src/api/tag/delete/delete_handler.rs` — the legacy
  bulk-delete handlers import `application_core::commands::post::delete::*`.
- `apps/api/migration/src/{m20240409_151952_release_100,m20250330_151455_release_110,m20260126_040610_release_300,m20260531_000001_pgvector,constants}.rs`
  — five *duplicate* migration files. The canonical copies are already in
  `domain_posts::migrations::*`; the `migration` crate's `lib.rs` is a
  `pub use domain_posts::migrations::*;` shim that re-exports them.

The duplication is technically dead code (every consumer is still wired
through `application_core::commands::post::*` instead of
`domain_posts::handlers::post::*`), but it blocks three follow-on changes:

1. `extract-media-into-domain-media` and `extract-users-into-domain-users`
   would need to delete `application_core::commands::post::*` to break the
   cycle that currently forces them to keep `application_core` as a path
   dependency. The cycle is already broken at the production-dep level for
   `domain_posts` (per `consolidate-category-ai-translate-into-domain-posts`
   Task 5.1), but the legacy `application_core::commands::post::*` tree
   keeps the import surface alive.
2. `merge-graphql-into-posts-domain` and the `legacy_bootstrap` cutover
   are blocked on every `cms::api::post::*` HTTP adapter being
   re-pointed to `domain_posts::handlers::post::*`.
3. The `migration` binary (a `migration::cli::run_cli(migration::Migrator)`
   in `apps/api/migration/src/main.rs`) cannot be removed until the
   duplicate migration files are deleted; the shim `lib.rs` is already in
   place to re-route `migration::Migrator` to `domain_posts::migrations::Migrator`.

This change is a pure, no-behavior-change re-organization: move the import
surface to `domain_posts::*` and delete the legacy duplicates. No HTTP route
is added, removed, or re-shaped. No entity column, index, or constraint is
added, removed, or re-shaped. No migration identity is renamed or reordered.
No OpenAI prompt, no translation pipeline invariant, no GraphQL schema, no
error envelope is altered. The change strictly targets *where* the post code
lives, not *what it does*.

## What Changes

- **Update** `apps/api/src/api/post/create/create_handler.rs` to import
  `PostCreateHandler`, `PostCreateHandlerTrait`, and `CreatePostRequest` from
  `domain_posts::handlers::post::create::{...}` instead of
  `application_core::commands::post::create::{...}`. The function signature
  (`State<AppState>`, `Extension<AuthenticatedActor>`, `Json<CreatePostRequest>`)
  is preserved exactly.
- **Update** `apps/api/src/api/post/read/read_handler.rs` to import
  `PostReadHandler` / `PostReadHandlerTrait` from
  `domain_posts::handlers::post::read::read_handler` instead of
  `application_core::commands::post::read::read_handler`.
- **Update** `apps/api/src/api/post/modify/modify_handler.rs` to import
  `PostModifyHandler` / `PostModifyHandlerTrait` / `ModifyPostRequest` from
  `domain_posts::handlers::post::modify::{...}` instead of
  `application_core::commands::post::modify::{...}`.
- **Update** `apps/api/src/api/post/delete/delete_handler.rs` to import
  `PostDeleteHandler` / `PostDeleteHandlerTrait` from
  `domain_posts::handlers::post::delete::delete_handler` instead of
  `application_core::commands::post::delete::delete_handler`.
- **Update** `apps/api/src/api/post/translate/translate_handler.rs` to
  import `PostTranslateHandler` / `PostTranslateHandlerTrait` /
  `TranslatePostRequest` from
  `domain_posts::handlers::post::translate::{...}` and the `VectorStore`
  from `domain_posts::handlers::vector_store::VectorStore` instead of
  `application_core::commands::ai::translate::{...}` and
  `application_core::commands::ai::vector_store_pg::VectorStore`.
- **Update** `apps/api/src/api/post/translate/job_handler.rs` to import
  `translation_jobs` from `domain_posts::entities::translation_jobs` (and
  `categories` / `posts` / `CategoryType` from `domain_posts::entities::*`
  in the test module) instead of the `application_core::entities` re-export
  shim.
- **Update** `apps/api/src/api/delete/delete_handler.rs` and
  `apps/api/src/api/tag/delete/delete_handler.rs` to import
  `PostDeleteHandler` / `PostDeleteHandlerTrait` from
  `domain_posts::handlers::post::delete::delete_handler` instead of
  `application_core::commands::post::delete::delete_handler`.
- **Delete** the legacy `application_core` post command modules:
  - `apps/api/application_core/src/commands/post/mod.rs`
  - `apps/api/application_core/src/commands/post/{create,read,modify,delete}/`
    (the four whole subtrees including the legacy
    `cfg(test) pub mod test { fn fake_create_post_request(...) }` fixture,
    which is no longer referenced by any test in the workspace).
  - The `pub mod post;` declaration in
    `apps/api/application_core/src/commands/mod.rs` is removed.
- **Delete** the legacy `application_core` AI translation modules:
  - `apps/api/application_core/src/commands/ai/mod.rs`
  - `apps/api/application_core/src/commands/ai/translate/`
  - `apps/api/application_core/src/commands/ai/vector_store_pg.rs`
  - The `pub mod ai;` declaration in
    `apps/api/application_core/src/commands/mod.rs` is removed.
  - The `README.md` next to the directory is removed.
  - The `ai` module in `application_core::commands` is *fully* removed
    after this change; the `models` subtree it previously contained was
    already deleted by the `consolidate-category-ai-translate-into-domain-posts`
    change, so no `application_core::commands::ai::*` submodule remains
    after the deletion.
- **Delete** the duplicate migration files in `apps/api/migration/src/`:
  - `apps/api/migration/src/m20240409_151952_release_100.rs`
  - `apps/api/migration/src/m20250330_151455_release_110.rs`
  - `apps/api/migration/src/m20260126_040610_release_300.rs`
  - `apps/api/migration/src/m20260531_000001_pgvector.rs`
  - `apps/api/migration/src/constants.rs`
  - The `apps/api/migration/src/lib.rs` re-export shim and
    `apps/api/migration/src/main.rs` binary entry point are *retained*;
    the shim already re-exports `domain_posts::migrations::*` and
    `domain_posts::migrations::Migrator`. The `cargo run -p migration -- migrate`
    CLI continues to work without any consumer-visible change.

No new capabilities are introduced at the HTTP layer. No new
`DomainService` is registered. No new `MigrationDescriptor` is added.
No new entity is added. The change is strictly a deletion + re-import
refactor.

## Capabilities

### New Capabilities

- `domain-posts`: The self-contained, single-source-of-truth post-domain
  module that owns the post CRUD command handlers, the post translation
  pipeline, the pgvector `VectorStore`, the post-relevant SeaORM entities,
  the four post migrations, the post HTTP adapters, the post GraphQL
  contribution, and the `DomainPostService` registered with the gateway.
  This capability captures the migration invariants: every post-related
  code path lives under `domain_posts::*`; the four migration identities
  are preserved; no behavior change is introduced; the legacy duplicates
  in `application_core::commands::post::*`,
  `application_core::commands::ai::{translate,vector_store_pg}::*`, and
  the duplicate `apps/api/migration/src/m{...}*.rs` files are removed.

### Modified Capabilities

None. The change is a code-organization refactor; no requirement changes
the observable behavior of any existing capability. The existing
canonical specs in `openspec/specs/` (`posts-graphql-mount`,
`pgvector-vector-search`, etc.) are not modified by this change.

## Impact

- **Affected source files (legacy to be deleted):**
  - `apps/api/application_core/src/commands/post/mod.rs`
  - `apps/api/application_core/src/commands/post/{create,read,modify,delete}/*`
    (16 files including `mod.rs` + `_handler.rs` + `_request.rs` per
    sub-action, plus the `test` fixture in `mod.rs`)
  - `apps/api/application_core/src/commands/ai/mod.rs`
  - `apps/api/application_core/src/commands/ai/translate/{mod,translate_handler,translate_request,translate_response}.rs`
  - `apps/api/application_core/src/commands/ai/vector_store_pg.rs`
  - `apps/api/application_core/src/commands/ai/README.md`
  - `apps/api/migration/src/m20240409_151952_release_100.rs`
  - `apps/api/migration/src/m20250330_151455_release_110.rs`
  - `apps/api/migration/src/m20260126_040610_release_300.rs`
  - `apps/api/migration/src/m20260531_000001_pgvector.rs`
  - `apps/api/migration/src/constants.rs`
  - The `pub mod post;` and `pub mod ai;` lines in
    `apps/api/application_core/src/commands/mod.rs`.
- **Affected source files (consumers to be re-pointed):**
  - `apps/api/src/api/post/create/create_handler.rs`
  - `apps/api/src/api/post/read/read_handler.rs`
  - `apps/api/src/api/post/modify/modify_handler.rs`
  - `apps/api/src/api/post/delete/delete_handler.rs`
  - `apps/api/src/api/post/translate/translate_handler.rs`
  - `apps/api/src/api/post/translate/job_handler.rs`
  - `apps/api/src/api/delete/delete_handler.rs`
  - `apps/api/src/api/tag/delete/delete_handler.rs`
- **Affected Cargo files:**
  - `apps/api/application_core/Cargo.toml` — the `application_core` crate
    no longer owns any post-related code. The `domain_posts = { path = "../domain_posts" }`
    dependency is retained (it is still needed for the
    `application_core::entities::*` re-export shim).
  - `apps/api/migration/Cargo.toml` — no change; the `domain_posts` path
    dependency is retained.
  - `apps/api/Cargo.toml` (workspace members) — no change; `domain_posts`
    and `migration` are already declared as workspace members.
- **Affected entities:** none at the schema level. The `posts`,
  `post_tags`, `post_translations`, `translation_jobs` entities remain
  defined canonically in `domain_posts::entities::*`. The
  `application_core::entities` module is *retained* as a re-export shim
  (`pub use domain_posts::entities::*;`) so that the legacy
  `cms::api::{media, user, administrator}::*` modules — which still
  import `application_core::entities::*` — keep compiling. The
  `application_core::entities::mod.rs` docstring is updated to call out
  that the post entities now flow through `domain_posts::entities` and
  that the shim is purely for the `cms::api::*` legacy tree.
- **Affected migrations:** none. The four migration identities
  `m20240409_151952_release_100`, `m20250330_151455_release_110`,
  `m20260126_040610_release_300`, `m20260531_000001_pgvector` remain
  registered in `domain_posts::migrations::Migrator` (which
  `migration::Migrator` re-exports). The database `up` history is
  unchanged. The `cargo run -p migration -- migrate` and
  `cargo run -p migration -- migrate --list` CLIs continue to work.
- **Affected tests:**
  - The `cfg(test) pub mod test { fn fake_create_post_request(...) }`
    fixture inside `apps/api/application_core/src/commands/post/mod.rs`
    is removed. A workspace-wide search confirms it is not referenced
    by any test outside the post-handler tests (the test block in
    `domain_posts::handlers::post::create::create_handler.rs` already
    contains only a `// placeholder` comment and no test bodies).
  - The `cfg(test) #[allow(unused_imports, dead_code)] mod tests {}`
    placeholder blocks in
    `domain_posts::handlers::post/{create,read,modify,delete,translate}::*`
    remain as-is; no test removal is implied by this change.
  - `apps/api/src/api/post/translate/job_handler.rs` test code that
    imports `application_core::entities::{categories, posts,
    sea_orm_active_enums::CategoryType}` and
    `application_core::entities::translation_jobs` is re-pointed to
    `domain_posts::entities::{categories, posts,
    sea_orm_active_enums::CategoryType}` and
    `domain_posts::entities::translation_jobs` directly. No test is
    removed.
- **Affected binaries:** the `legacy_bootstrap` binary
  (`apps/api/src/bin/legacy_bootstrap.rs`) is unchanged at the source
  level — it imports `cms::api::post::create::create_handler::api_create_post`
  etc. as before. The function bodies of those handlers are
  re-implemented to call the `domain_posts::*` command handlers instead
  of the `application_core::commands::post::*` ones, so the legacy
  binary's runtime behavior is unchanged but its dependency surface
  becomes `cms::api::post::*` → `domain_posts::handlers::post::*` →
  `domain_posts::entities::*` instead of
  `cms::api::post::*` → `application_core::commands::post::*` →
  `application_core::entities::*` (re-export shim) →
  `domain_posts::entities::*`.
- **Affected `Cargo.toml` workspace:** the `members` list in
  `apps/api/Cargo.toml` is unchanged: `["application_core", "domain_auth",
  "domain_interface", "domain_posts", "gateway", "migration",
  "test_helpers"]`.
