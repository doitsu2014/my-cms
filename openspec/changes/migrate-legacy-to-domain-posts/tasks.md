## 1. Re-point the post HTTP adapters to `domain_posts`

- [x] 1.1 Update `apps/api/src/api/post/create/create_handler.rs` to import
      `PostCreateHandler`, `PostCreateHandlerTrait`, and `CreatePostRequest`
      from `domain_posts::handlers::post::create::{create_handler, create_request}`
      instead of `application_core::commands::post::create::{...}`. Preserve
      the function signature (`State<AppState>`, `Extension<AuthenticatedActor>`,
      `Json<CreatePostRequest>`) and the body
      (`PostCreateHandler { db: state.conn.clone() }`,
      `handler.handle_create_post(body, actor.email.clone()).await`).
      **Verify:** `cargo check -p cms` succeeds; `git diff apps/api/src/api/post/create/create_handler.rs`
      shows the `use` lines changed but the function body, signature, and
      `ApiResponseWith` / `ApiResponseError` envelope are unchanged.
- [x] 1.2 Update `apps/api/src/api/post/read/read_handler.rs` to import
      `PostReadHandler`, `PostReadHandlerTrait` from
      `domain_posts::handlers::post::read::read_handler` instead of
      `application_core::commands::post::read::read_handler`. The
      `CategoryType` import switches from
      `application_core::entities::sea_orm_active_enums::CategoryType` to
      `domain_posts::entities::sea_orm_active_enums::CategoryType`.
      Preserve both `api_get_posts_with_filtering` and `api_get_post`
      signatures and bodies.
      **Verify:** `cargo check -p cms` succeeds.
- [x] 1.3 Update `apps/api/src/api/post/modify/modify_handler.rs` to import
      `PostModifyHandler`, `PostModifyHandlerTrait`, `ModifyPostRequest`
      from `domain_posts::handlers::post::modify::{modify_handler, modify_request}`
      instead of `application_core::commands::post::modify::{...}`. Preserve
      the function signature and body.
      **Verify:** `cargo check -p cms` succeeds.
- [x] 1.4 Update `apps/api/src/api/post/delete/delete_handler.rs` to import
      `PostDeleteHandler`, `PostDeleteHandlerTrait` from
      `domain_posts::handlers::post::delete::delete_handler` instead of
      `application_core::commands::post::delete::delete_handler`. Preserve
      the function signature and body.
      **Verify:** `cargo check -p cms` succeeds.
- [x] 1.5 Update `apps/api/src/api/post/translate/translate_handler.rs` to
      import `PostTranslateHandler`, `PostTranslateHandlerTrait`,
      `TranslatePostRequest` from
      `domain_posts::handlers::post::translate::{translate_handler, translate_request}`
      instead of `application_core::commands::ai::translate::{...}`. Replace
      the `application_core::commands::ai::vector_store_pg::VectorStore`
      references with
      `domain_posts::handlers::vector_store::VectorStore`. Preserve the
      `TranslatePostRequestBody` and `TranslatePostResponse` DTOs, the
      `initialize_vector_store` helper, and the bodies of
      `api_translate_post` and `api_translate_post_background`.
      **Verify:** `cargo check -p cms` succeeds;
      `grep -n "application_core::commands::ai" apps/api/src/api/post/translate/translate_handler.rs`
      returns no matches.
- [x] 1.6 Update `apps/api/src/api/post/translate/job_handler.rs` to import
      `translation_jobs` from `domain_posts::entities::translation_jobs`
      in the production code. In the `#[cfg(test)] mod tests` block, change
      `use application_core::entities::{categories, posts, sea_orm_active_enums::CategoryType};`
      to `use domain_posts::entities::{categories, posts, sea_orm_active_enums::CategoryType};`
      and every `use application_core::entities::translation_jobs;` inside
      the test functions to
      `use domain_posts::entities::translation_jobs;`.
      **Verify:** `cargo check -p cms --tests` succeeds;
      `grep -n "application_core::entities" apps/api/src/api/post/translate/job_handler.rs`
      returns no matches.
- [x] 1.7 Update `apps/api/src/api/delete/delete_handler.rs` to import
      `PostDeleteHandler`, `PostDeleteHandlerTrait` from
      `domain_posts::handlers::post::delete::delete_handler` instead of
      `application_core::commands::post::delete::delete_handler`. Preserve
      the function signature and body.
      **Verify:** `cargo check -p cms` succeeds.
- [x] 1.8 Update `apps/api/src/api/tag/delete/delete_handler.rs` to import
      `PostDeleteHandler`, `PostDeleteHandlerTrait` from
      `domain_posts::handlers::post::delete::delete_handler` instead of
      `application_core::commands::post::delete::delete_handler`. Preserve
      the function signature and body.
      **Verify:** `cargo check -p cms` succeeds.
- [x] 1.9 Group gate: `cargo check --workspace` succeeds; the legacy
      `application_core::commands::post::*` and
      `application_core::commands::ai::*` modules are still present (no
      deletions yet) but no `cms::api::post::*`, `cms::api::delete::*`,
      or `cms::api::tag::delete::*` file imports them.

## 2. Drop the legacy command module declarations from `application_core`

- [x] 2.1 Update `apps/api/application_core/src/commands/mod.rs` to remove
      the `pub mod post;` and `pub mod ai;` lines. Retain
      `pub mod media; pub mod tag; pub mod user;`.
      **Verify:** `cargo check -p application_core` fails with
      `error[E0583]: could not find module file .../commands/post/mod.rs`
      (and similarly for `ai`) — this is the expected "broken by design"
      state. Proceed immediately to Task 3 to delete the files.
- [x] 2.2 (Documentation only, no functional change) Update
      `apps/api/application_core/src/entities/mod.rs` to refresh the
      docstring at the top of the file: explicitly call out that the
      shim is the single remaining reason `application_core` still
      depends on `domain_posts`, and that the only callers that flow
      through it are the legacy `cms::api::{media, user, administrator}::*`
      modules plus `apps/api/test_helpers`. The `pub use
      domain_posts::entities::*;` line itself is unchanged.
      **Verify:** `cargo check -p application_core` still succeeds (it
      will not, until Task 3 is complete — but the docstring is the
      only line that changes in this task, so reverting it restores
      the pre-task file if needed).

## 3. Delete the legacy `application_core::commands::post` tree

- [x] 3.1 Delete the directory
      `apps/api/application_core/src/commands/post/create/` and its
      contents (`mod.rs`, `create_handler.rs`, `create_request.rs`).
      **Verify:** `ls apps/api/application_core/src/commands/post/create/`
      returns "No such file or directory".
- [x] 3.2 Delete the directory
      `apps/api/application_core/src/commands/post/read/` and its
      contents (`mod.rs`, `read_handler.rs`).
      **Verify:** `ls apps/api/application_core/src/commands/post/read/`
      returns "No such file or directory".
- [x] 3.3 Delete the directory
      `apps/api/application_core/src/commands/post/modify/` and its
      contents (`mod.rs`, `modify_handler.rs`, `modify_request.rs`).
      **Verify:** `ls apps/api/application_core/src/commands/post/modify/`
      returns "No such file or directory".
- [x] 3.4 Delete the directory
      `apps/api/application_core/src/commands/post/delete/` and its
      contents (`mod.rs`, `delete_handler.rs`).
      **Verify:** `ls apps/api/application_core/src/commands/post/delete/`
      returns "No such file or directory".
- [x] 3.5 Delete `apps/api/application_core/src/commands/post/mod.rs`.
      **Verify:** `ls apps/api/application_core/src/commands/post/`
      returns "No such file or directory".
- [x] 3.6 Group gate: `cargo check -p application_core -p cms` succeeds;
      `cargo check -p domain_posts` succeeds; no remaining `use`
      statement references `application_core::commands::post`.

## 4. Delete the legacy `application_core::commands::ai` tree

- [x] 4.1 Delete the directory
      `apps/api/application_core/src/commands/ai/translate/` and its
      contents (`mod.rs`, `translate_handler.rs`, `translate_request.rs`,
      `translate_response.rs`).
      **Verify:** `ls apps/api/application_core/src/commands/ai/translate/`
      returns "No such file or directory".
- [x] 4.2 Delete `apps/api/application_core/src/commands/ai/vector_store_pg.rs`.
      **Verify:** `ls apps/api/application_core/src/commands/ai/vector_store_pg.rs`
      returns "No such file or directory".
- [x] 4.3 Delete `apps/api/application_core/src/commands/ai/mod.rs` and
      `apps/api/application_core/src/commands/ai/README.md`.
      **Verify:** `ls apps/api/application_core/src/commands/ai/`
      returns "No such file or directory".
- [x] 4.4 Group gate: `cargo check -p application_core -p cms -p domain_posts`
      succeeds; no remaining `use` statement references
      `application_core::commands::ai`.

## 5. Delete the duplicate migration files

- [x] 5.1 Delete `apps/api/migration/src/m20240409_151952_release_100.rs`.
      **Verify:** `ls apps/api/migration/src/m20240409_151952_release_100.rs`
      returns "No such file or directory".
- [x] 5.2 Delete `apps/api/migration/src/m20250330_151455_release_110.rs`.
      **Verify:** `ls apps/api/migration/src/m20250330_151455_release_110.rs`
      returns "No such file or directory".
- [x] 5.3 Delete `apps/api/migration/src/m20260126_040610_release_300.rs`.
      **Verify:** `ls apps/api/migration/src/m20260126_040610_release_300.rs`
      returns "No such file or directory".
- [x] 5.4 Delete `apps/api/migration/src/m20260531_000001_pgvector.rs`.
      **Verify:** `ls apps/api/migration/src/m20260531_000001_pgvector.rs`
      returns "No such file or directory".
- [x] 5.5 Delete `apps/api/migration/src/constants.rs`.
      **Verify:** `ls apps/api/migration/src/constants.rs` returns
      "No such file or directory".
- [x] 5.6 Group gate: `cargo check -p migration -p cms -p test_helpers`
      succeeds. The only remaining files in `apps/api/migration/src/`
      are `lib.rs` (the `pub use domain_posts::migrations::*; pub use
      domain_posts::migrations::Migrator;` shim) and `main.rs` (the
      `cli::run_cli(migration::Migrator).await` entry point).

## 6. Repository verification gate

- [x] 6.1 Run `cargo check --workspace` and confirm success with no
      errors. **Verify:** exit code 0; the only warnings are
      pre-existing (the three `missing_debug_implementations` warnings
      in `domain_posts::domain::response::ApiResponseError`,
      `domain_posts::migrations::Migrator`, and the
      `proc-macro-error2` future-incompat note).
- [x] 6.2 Run `cargo test --workspace --lib --bins` and confirm the same
      set of tests that passed before the change still passes (the
      placeholder test blocks in `domain_posts::handlers::post::*` are
      not counted as failures; `application_core::entities::tests` and
      the test_helpers integration tests are unaffected).
      **Verify:** exit code 0; pass count matches the pre-change count.
- [x] 6.3 Run `cargo fmt -- --check` and confirm no formatting drift.
      **Verify:** exit code 0.
- [x] 6.4 Run `cargo clippy --all-targets` and confirm no new clippy
      warnings are introduced by this change. Pre-existing warnings
      (e.g. in `domain_posts::domain::response`, in
      `domain_auth::legacy_bootstrap::tests::with_env_var`'s pattern
      usage, etc.) are tolerated.
      **Verify:** clippy warning count is the same as before the
      change.
- [x] 6.5 Run `cargo build --bin legacy_bootstrap` and
      `cargo build --bin my-cms-api`. Both must succeed.
      **Verify:** exit code 0 for both.
- [x] 6.6 Confirm no remaining `use application_core::commands::post::*`
      or `use application_core::commands::ai::*` import exists anywhere
      in the workspace (excluding `target/` and the four legacy
      files that were just deleted).
      **Verify:**
      `rg -n "use application_core::commands::(post|ai)" apps/api`
      returns no matches.
- [x] 6.7 Confirm no remaining duplicate migration file exists.
      **Verify:**
      `ls apps/api/migration/src/` returns only `lib.rs` and `main.rs`.

## 7. Handoff

- [ ] 7.1 Archive the change with `openspec archive migrate-legacy-to-domain-posts`
      (only after all of Task 6's gates pass and the change is merged
      to `main`). **Verify:** `openspec list --json` reports the change
      as archived and the next `openspec verify --change
      migrate-legacy-to-domain-posts` succeeds against the archived
      snapshot.
