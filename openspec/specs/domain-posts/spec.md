# domain-posts Specification

## Purpose
TBD - created by archiving change migrate-legacy-to-domain-posts. Update Purpose after archive.
## Requirements
### Requirement: Post CRUD command handlers live exclusively in `domain_posts`
The system SHALL own the post CRUD command handlers — `PostCreateHandler`,
`PostReadHandler`, `PostModifyHandler`, `PostDeleteHandler` and their
`*HandlerTrait` companions — exclusively in
`domain_posts::handlers::post::{create,read,modify,delete}::*`. The
`application_core::commands::post` module SHALL NOT exist after this change
takes effect.

#### Scenario: No legacy post command module remains
- **WHEN** the workspace is built with `cargo check --workspace`
- **THEN** `application_core::commands::post` does not appear in any
  `use` statement, in any `pub mod post;` declaration, or in the
  `application_core::Cargo.toml` dependency graph
- **AND** the `apps/api/application_core/src/commands/post/{create,read,modify,delete}/`
  directory tree is empty or removed
- **AND** `apps/api/application_core/src/commands/post/mod.rs` no longer
  declares the `pub mod test` block that contained
  `fake_create_post_request`

#### Scenario: Existing post handler call sites still resolve
- **WHEN** a caller invokes any of
  `domain_posts::handlers::post::create::create_handler::PostCreateHandler`,
  `domain_posts::handlers::post::read::read_handler::PostReadHandler`,
  `domain_posts::handlers::post::modify::modify_handler::PostModifyHandler`,
  or `domain_posts::handlers::post::delete::delete_handler::PostDeleteHandler`
- **THEN** the handler trait method (`handle_create_post`,
  `handle_get_all_posts`, `handle_get_posts_with_filtering`,
  `handle_get_post`, `handle_modify_post`, `handle_delete_posts`) is
  callable with the same signature, the same `Arc<DatabaseConnection>`
  injection, and the same `Result<_, AppError>` return type as the
  pre-change legacy handler

### Requirement: Post translation pipeline lives exclusively in `domain_posts`
The system SHALL own the post translation pipeline — `PostTranslateHandler`,
the `TranslatePostRequest` and `TranslatePostResponse` request/response
DTOs, the 3-tier lookup (DB → pgvector → OpenAI), the `VectorStore`
helper, and the translation-job lifecycle — exclusively in
`domain_posts::handlers::post::translate::*` and
`domain_posts::handlers::vector_store::*`. The
`application_core::commands::ai::translate` module and the
`application_core::commands::ai::vector_store_pg` module SHALL NOT exist
after this change takes effect, and the `application_core::commands::ai`
module SHALL be removed in its entirety.

#### Scenario: No legacy AI translation module remains
- **WHEN** the workspace is built with `cargo check --workspace`
- **THEN** `application_core::commands::ai` is not present in any `use`
  statement, in any `pub mod ai;` declaration, or in the
  `application_core::Cargo.toml` dependency graph
- **AND** the files
  `apps/api/application_core/src/commands/ai/{mod.rs,translate/*,vector_store_pg.rs,README.md}`
  are removed

#### Scenario: Existing translation call sites still resolve
- **WHEN** a caller invokes
  `domain_posts::handlers::post::translate::translate_handler::PostTranslateHandler::handle_translate_post`
  or `::handle_translate_post_background`, or
  `domain_posts::handlers::vector_store::VectorStore::{new, initialize_collection, store_translation, search_similar_translations, find_translation}`
- **THEN** the trait methods are callable with the same signature, the
  same `Arc<DatabaseConnection>` injection, the same OpenAI model
  selection, the same chunking behavior (HTML / plain text), the same
  pgvector `ON CONFLICT (post_id, language_code) DO UPDATE` upsert
  semantics, and the same `Result<_, AppError>` return type as the
  pre-change legacy handler

### Requirement: Post HTTP adapters in `cms::api::post` SHALL import from `domain_posts`
The legacy HTTP adapter modules under `apps/api/src/api/post/{create,read,modify,delete,translate}/*` SHALL import their command-handler dependencies from `domain_posts::handlers::post::*` and `domain_posts::handlers::vector_store::*` rather than from `application_core::commands::post::*` or `application_core::commands::ai::*`. The function signatures of the `api_*` functions SHALL remain unchanged: they continue to take `State<AppState>` (the legacy state type with `conn: Arc<DatabaseConnection>`) and `Extension<AuthenticatedActor>` so that the `legacy_bootstrap` binary keeps serving the same routes with the same envelope.

#### Scenario: Legacy HTTP adapter files no longer import the deleted modules
- **WHEN** a workspace-wide search is run for
  `use application_core::commands::post::` and
  `use application_core::commands::ai::` inside
  `apps/api/src/api/post/**`
- **THEN** no match is found in
  `apps/api/src/api/post/{create,read,modify,delete,translate}/**/*.rs`
- **AND** each `api_*` function continues to accept the same `State<AppState>`,
  `Extension<AuthenticatedActor>`, `Json<...>`, `Path<...>`, and
  `Query<...>` extractors and returns the same
  `ApiResponseWith<_>` / `ApiResponseError` envelope

#### Scenario: The `legacy_bootstrap` binary routes the same way
- **WHEN** the `legacy_bootstrap` binary is built with
  `cargo build --bin legacy_bootstrap`
- **THEN** the binary compiles
- **AND** the routes wired in `apps/api/src/bin/legacy_bootstrap.rs`
  (`/posts` GET/POST/PUT/DELETE, `/posts/{post_id}` GET,
  `/posts/{post_id}/translate` POST,
  `/posts/{post_id}/translate/background` POST,
  `/posts/{post_id}/translate/jobs` GET,
  `/posts/{post_id}/translate/jobs/{job_id}` GET) all serve the same
  HTTP method, path, and authorization role as before

#### Scenario: The `delete` and `tag::delete` HTTP adapters use the domain_posts delete handler
- **WHEN** a workspace-wide search is run for
  `use application_core::commands::post::delete::delete_handler::` inside
  `apps/api/src/api/**`
- **THEN** no match is found
- **AND** `apps/api/src/api/delete/delete_handler.rs` and
  `apps/api/src/api/tag/delete/delete_handler.rs` import
  `PostDeleteHandler` and `PostDeleteHandlerTrait` from
  `domain_posts::handlers::post::delete::delete_handler`

#### Scenario: The `translate::job_handler` reads entities from `domain_posts`
- **WHEN** the `apps/api/src/api/post/translate/job_handler.rs` source
  is read
- **THEN** the production code imports `translation_jobs` from
  `domain_posts::entities::translation_jobs`
- **AND** the test code imports `categories`, `posts`,
  `sea_orm_active_enums::CategoryType`, and `translation_jobs` from
  `domain_posts::entities::*` (no `application_core::entities::*` import
  remains in this file)

### Requirement: Migration duplicates are removed; `migration` crate is a pure re-export shim
The `apps/api/migration/src/` crate SHALL contain only `lib.rs` (the
`pub use domain_posts::migrations::*; pub use domain_posts::migrations::Migrator;`
shim) and `main.rs` (the `cli::run_cli(migration::Migrator).await`
binary entry point). The five duplicate migration files
(`m20240409_151952_release_100.rs`, `m20250330_151455_release_110.rs`,
`m20260126_040610_release_300.rs`, `m20260531_000001_pgvector.rs`,
`constants.rs`) SHALL NOT exist after this change takes effect.

#### Scenario: Duplicate migration files are removed
- **WHEN** `ls apps/api/migration/src/` is run
- **THEN** the output contains only `lib.rs` and `main.rs`
- **AND** `apps/api/migration/Cargo.toml` retains
  `domain_posts = { path = "../domain_posts" }` in `[dependencies]`
  so the re-export shim can resolve

#### Scenario: `cargo run -p migration -- migrate --list` still prints four IDs in order
- **WHEN** the migration CLI is invoked against any reachable database
  (including the testcontainer used by `test_helpers::setup_test_space`)
- **THEN** it prints the four migration identities
  `m20240409_151952_release_100`, `m20250330_151455_release_110`,
  `m20260126_040610_release_300`, `m20260531_000001_pgvector` in this
  exact order
- **AND** it does NOT print any new migration identity

#### Scenario: `test_helpers` and `administrator::migration` resolve `Migrator` from the shim
- **WHEN** `apps/api/test_helpers/src/lib.rs` is read
- **THEN** the line `use migration::{Migrator, MigratorTrait};` resolves
  against the shim and produces the same `Migrator` type as
  `domain_posts::migrations::Migrator`
- **AND** `apps/api/src/api/administrator/migration/migration_handler.rs`
  continues to import `Migrator` and `MigratorTrait` from `migration::*`
  and call `Migrator::up(...)` with the same arguments

### Requirement: The four post-domain migration identities SHALL be preserved
The four migration identities `m20240409_151952_release_100`, `m20250330_151455_release_110`, `m20260126_040610_release_300`, and `m20260531_000001_pgvector` SHALL remain registered in `domain_posts::migrations::Migrator::migrations()` in this exact order, and the database `up` history produced by running `Migrator::up` SHALL be identical to the pre-change history (no new table, no new column, no new index, no new constraint, no renamed constraint).

#### Scenario: `DomainPostService::migrations()` returns the same four descriptors
- **WHEN** `domain_posts::service::DomainPostService::migrations()` is
  called
- **THEN** it returns a `Vec<MigrationDescriptor>` of length 4 with the
  `id` values `"m20240409_151952_release_100"`,
  `"m20250330_151455_release_110"`,
  `"m20260126_040610_release_300"`,
  `"m20260531_000001_pgvector"` in this order
- **AND** every `MigrationDescriptor::depends_on` is the empty slice
  (no foundation dependency exists)

#### Scenario: Re-running migrations against a fresh testcontainer is idempotent
- **WHEN** `Migrator::up(conn, None).await` is run twice in sequence
  against a fresh PostgreSQL testcontainer
- **THEN** the first run applies all four migrations and the second run
  is a no-op
- **AND** the resulting schema is identical to the pre-change schema
  produced by running the four legacy migration files in order

### Requirement: No observable behavior change
The change SHALL NOT alter any observable behavior of the system. Every
HTTP route served by `legacy_bootstrap` (and by the `domain_posts`
standalone bin, and by the `gateway` composition root) SHALL accept
the same methods, paths, request bodies, response envelopes, status
codes, and authorization roles as before. Every public entity, every
public command-handler struct, and every public HTTP handler function
SHALL remain callable with the same name and signature.

#### Scenario: HTTP route contract is preserved
- **WHEN** a regression test suite enumerates the routes wired by
  `legacy_bootstrap` (or the gateway or the `domain_posts` bin)
- **THEN** the set of route prefixes, methods, and mount classifications
  is identical to the pre-change set: `/`, `/health`, `/healthz`,
  `/posts/graphql/{immutable,mutable}`, `/posts`, `/posts/{post_id}`,
  `/posts/{post_id}/translate{,/background,/jobs,/jobs/{job_id}}`,
  `/tags`, `/media`, `/media/info/{*path}`, `/media/delete/{*path}`,
  `/users`, `/users/{user_id}`, `/users/{user_id}/reset-password`,
  `/media/buckets`, `/media/buckets/{name}`,
  `/media/buckets/{name}/empty`, `/administrator/database/migration`

#### Scenario: GraphQL schema, scopes, and roles are preserved
- **WHEN** the gateway or the `domain_posts` bin boots
- **THEN** the GraphQL immutable and mutable schemas are produced by
  `domain_posts::domain::graphql::contribute_post_schema(...)` exactly
  as before the change
- **AND** the `/posts/graphql/mutable` mount accepts
  `["my-headless-cms-writer", "my-headless-cms-administrator"]` roles
  exactly as before
- **AND** the `/posts/graphql/immutable` mount remains public

#### Scenario: Existing repository verification gate passes
- **WHEN** the repository verification gate is run after this change
- **THEN** `cargo check --workspace` succeeds
- **AND** `cargo test --workspace --lib --bins` succeeds (the same set
  of tests that passed pre-change, with the same pass/fail counts)
- **AND** `cargo fmt --check` reports no formatting changes
- **AND** `cargo clippy --all-targets` reports no new warnings introduced
  by this change
- **AND** `cargo build --bin legacy_bootstrap` and
  `cargo build --bin my-cms-api` both succeed

