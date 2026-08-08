# Design: Migrate legacy post code to `domain_posts`

## Context

The `my-cms` Rust backend has been progressively carved into pluggable
domain crates since the `refactor-api-into-pluggable-domain-libraries`
change landed `domain_posts` as a self-contained Cargo crate
(`apps/api/domain_posts/`). The `consolidate-category-ai-translate-into-domain-posts`
change subsequently folded categories, the AI model registry, and
`apps/api/src/api/category/*` / `apps/api/src/api/ai/*` into the same
crate. The canonical post-related code — post CRUD command handlers,
post translation pipeline, pgvector `VectorStore`, the four post
migrations, the post-relevant SeaORM entities, the post HTTP
adapters, the post GraphQL contribution, and a `DomainPostService`
registered with the gateway — already lives under `domain_posts::*`.

What remains is a thin layer of *legacy duplicates* that
`application_core` and `migration` (and the `cms::api::post::*` HTTP
adapters) still host. The duplicates are byte-for-byte or
near-byte-for-byte copies of the canonical code; no consumer uses
them in preference to the canonical paths. The
`application_core::entities` module is a `pub use
domain_posts::entities::*;` shim, and the `migration` crate's
`lib.rs` is a `pub use domain_posts::migrations::*; pub use
domain_posts::migrations::Migrator;` shim. The shims are the only
"live" parts of the legacy crates; everything else in them is a
dead-weight duplicate that has to be removed before the
`application_core` and `migration` crates can themselves be deleted
in a future change.

This change is the code-organization cleanup that removes the
duplicates and re-points the remaining `cms::api::post::*`,
`cms::api::delete::*`, and `cms::api::tag::delete::*` HTTP
adapters to the canonical `domain_posts` paths.

## Goals / Non-Goals

**Goals:**

- Single source of truth for every post-related code path: command
  handlers, request/response DTOs, HTTP adapters, the post
  translation pipeline, the pgvector `VectorStore`, the four
  migration files, the SeaORM entities, the GraphQL playground
  handlers, and the `DomainPostService` registration. After the
  change, all of these live in `domain_posts::*`.
- The `application_core::commands::post` and
  `application_core::commands::ai` modules are removed in their
  entirety. The `application_core::entities` re-export shim is
  retained (and its docstring is updated) so that the
  `cms::api::{media, user, administrator}::*` modules — which still
  import `application_core::entities::*` — keep compiling.
- The `apps/api/migration/src/` crate contains only the
  re-export-shim `lib.rs` and the `main.rs` CLI entry point; the
  five duplicate migration files are removed.
- The `cargo run -p migration -- migrate` and
  `cargo run -p migration -- migrate --list` CLIs continue to work
  without any consumer-visible change.
- The `cargo build --bin legacy_bootstrap` binary continues to
  serve every post route with the same path, method, body, and
  envelope as before.
- The `cargo build --bin my-cms-api` gateway composition continues
  to compose `domain_posts::DomainPostService` and
  `domain_auth::DomainAuthService` without any change to its source.
- The four migration identities
  (`m20240409_151952_release_100`, `m20250330_151455_release_110`,
  `m20260126_040610_release_300`, `m20260531_000001_pgvector`) are
  preserved exactly. The database `up` history is unchanged.

**Non-Goals:**

- No behavior change. No HTTP route is added, removed, or
  re-shaped. No OpenAI prompt is altered. No translation chunking
  heuristic is changed. No GraphQL field is added or removed. No
  error envelope is altered. No authorization role is widened or
  narrowed.
- No entity schema change. No new table, column, index, or
  constraint. No SeaORM entity is hand-edited.
- No migration is added, removed, or reordered.
- No new `DomainService` is registered; no new `RouteRegistration`
  is added; no new `Mount` classification is introduced.
- No new Cargo workspace member is added; the `members` list in
  `apps/api/Cargo.toml` is unchanged.
- No new Cargo dependency is added to any crate. The
  `application_core::Cargo.toml` and `migration::Cargo.toml`
  dependency lists are unchanged; the existing
  `domain_posts = { path = "../domain_posts" }` path dependency
  is retained (it is still needed for the
  `application_core::entities` and `migration::lib` re-export
  shims).
- The `legacy_bootstrap` binary's source file is not edited. Only
  the bodies of the `cms::api::post::*` adapter functions it calls
  are updated; the function signatures and the route table are
  unchanged.
- The `application_core` crate is *not* removed. The
  `application_core::commands::{media,user,tag}::*` and
  `application_core::common::*` modules are still needed by
  `cms::api::{media,user,tag,administrator}::*` and by
  `test_helpers`. A future change will extract them into
  `domain_media`, `domain_users`, `domain_tags`, etc.; that is
  out of scope here.

## Decisions

### Decision 1 — In-place re-import of `cms::api::post::*` (no state-type change)

The legacy HTTP adapter functions in
`apps/api/src/api/post/{create,read,modify,delete,translate}/*` are
updated *in place* — only their `use` imports change. The function
signatures stay exactly as they are: `State<AppState>`,
`Extension<AuthenticatedActor>`, `Json<...>`, `Path<...>`,
`Query<...>`. The bodies switch from constructing
`PostCreateHandler { db: state.conn.clone() }` (with the
`application_core` type) to constructing the same struct with the
same `db: state.conn.clone()` injection, but the imported type is
now `domain_posts::handlers::post::create::create_handler::PostCreateHandler`.

**Why in-place and not a 1:1 re-export of `domain_posts::api::post::*`?**
The `domain_posts::api::post::create::api_create_post` and siblings
take `State<DomainContext>`, not `State<AppState>`. A 1:1 re-export
would force the `legacy_bootstrap` binary to switch from
`Router<AppState>` to `Router<DomainContext>`. That is a much
larger refactor (every `legacy_bootstrap` router, every middleware,
every extractor must change at once). The in-place approach keeps
the `AppState` type stable so `legacy_bootstrap` keeps compiling
unchanged, while still moving the *business logic* call site to
`domain_posts::*`. The envelope shape (`ApiResponseWith<Uuid>`,
`ApiResponseError`) is also stable, so the Axum response types
stay identical.

**Rejected alternative:** delete the `cms::api::post::*` files
entirely and route the legacy bootstrap through
`domain_posts::api::post::*` via a `Router<DomainContext>` adapter.
Rejected because it requires switching the entire `legacy_bootstrap`
router stack to `Router<DomainContext>`, which is a much larger
change with no behavior benefit.

### Decision 2 — The `application_core::entities` shim is retained

`apps/api/application_core/src/entities/mod.rs` is a `pub use
domain_posts::entities::*;` re-export shim (per the
`consolidate-category-ai-translate-into-domain-posts` change). It
is retained verbatim. The docstring is updated to call out the
specific paths that flow through it (the legacy
`cms::api::{media, user, administrator}::*` modules, plus the
`test_helpers` crate) so that future readers do not assume it can
be deleted.

**Why retain?** The legacy `cms::api::{media, user,
administrator}::*` modules still import `application_core::entities::*`
in ~10 files. Deleting the shim would force a parallel
re-pointing in all of them. That is a much larger change with no
behavior benefit; the shim is a one-line `pub use` and compiles to
zero machine code at runtime.

**Rejected alternative:** delete the `application_core` crate
entirely and force every `cms::api::{media, user, administrator}::*`
consumer to switch to `domain_posts::entities::*` (or to the future
`domain_media::entities::*`, `domain_users::entities::*`). Rejected
because the `cms::api::{media, user, administrator}::*` modules
themselves are slated for extraction into their own domain crates
in follow-on changes; updating the entity path now would duplicate
work.

### Decision 3 — The `application_core::commands::ai` module is removed in its entirety

`apps/api/application_core/src/commands/ai/` currently contains:

- `mod.rs` (re-exports `translate` and `vector_store_pg`),
- `translate/` (the legacy `PostTranslateHandler` and DTOs),
- `vector_store_pg.rs` (the legacy `VectorStore`),
- `README.md` (a module description).

The `consolidate-category-ai-translate-into-domain-posts` change
already removed `application_core::commands::ai::models::*` (the
AI model registry moved into `domain_posts::handlers::ai::models::*`).
After this change, the only remaining contents of the `ai` module
are the `translate` and `vector_store_pg` subtrees. Both are
duplicates of `domain_posts::handlers::post::translate::*` and
`domain_posts::handlers::vector_store::*` respectively. The
`mod.rs` `pub mod translate; pub mod vector_store_pg;` declarations
are removed; the directory and its `README.md` are deleted; the
`pub mod ai;` line in `apps/api/application_core/src/commands/mod.rs`
is removed.

**Rejected alternative:** keep `application_core::commands::ai`
around as an empty module (e.g. with just a `// moved to
domain_posts` docstring). Rejected because an empty module adds
zero value; future readers should be able to grep
`application_core::commands::ai::*` and get no matches.

### Decision 4 — Duplicate migration files in `apps/api/migration/src/` are deleted; the crate becomes a pure shim

The `apps/api/migration/src/` crate's `lib.rs` is a `pub use
domain_posts::migrations::*; pub use
domain_posts::migrations::Migrator;` re-export shim. The
`main.rs` calls `cli::run_cli(migration::Migrator).await`. Both
remain unchanged.

The five duplicate files
(`m20240409_151952_release_100.rs`,
`m20250330_151455_release_110.rs`,
`m20260126_040610_release_300.rs`,
`m20260531_000001_pgvector.rs`,
`constants.rs`) are deleted. They are byte-for-byte duplicates of
the canonical `domain_posts::migrations::*` files (verified by
diffing the `Posts`, `Categories`, `PostTranslations`,
`TranslationJobs`, `Embeddings`, `NameLength`, etc. definitions);
the canonical copies already declare the migrations in the same
order with the same identities.

**Rejected alternative:** keep the duplicate files as
`pub use domain_posts::migrations::m20240409_151952_release_100::*;`
re-export shims. Rejected because the duplicates are
*implementation* files (they define `Migration` structs and
`#[derive(DeriveIden)]` enums), not just re-exports; making them
re-export shims would require lifting the `Migration` struct and
`Iden` enum definitions into a `pub` namespace in
`domain_posts::migrations`. That is a more invasive change to
`domain_posts` than simply deleting the duplicates. Deletion is
simpler and matches the `refactor-api-into-pluggable-domain-libraries`
design's intent (the legacy `migration` crate is a transitional
shim that goes away in Stage 4 of the cutover plan documented in
`docs/pluggable-domain-refactor.md`).

### Decision 5 — Order of operations: update consumers first, then delete sources

The codebase must compile after every task group, not just at the
end. The task ordering is therefore:

1. **Re-point consumers** (`cms::api::post::*`, `cms::api::delete::*`,
   `cms::api::tag::delete::*`) to `domain_posts::*` — this changes
   the `use` statements only; the legacy modules still exist, so
   the old code still compiles alongside the new.
2. **Delete the legacy `application_core::commands::post::*` tree** —
   the consumers have already been moved off, so nothing references
   the deleted files.
3. **Delete the legacy `application_core::commands::ai::*` tree** —
   the consumers have already been moved off.
4. **Delete the duplicate migration files** — the consumers
   (`test_helpers`, `cms::api::administrator::migration::*`) read
   only `migration::Migrator` (and `MigratorTrait`), which the
   `lib.rs` shim re-exports from `domain_posts::migrations::*`.
5. **Run the repository verification gate** — `cargo check --workspace`,
   `cargo test --workspace --lib --bins`, `cargo fmt --check`,
   `cargo clippy --all-targets`, `cargo build --bin legacy_bootstrap`,
   `cargo build --bin my-cms-api`.

Each task group leaves the workspace in a state where
`cargo check --workspace` succeeds.

**Rejected alternative:** delete sources first, then update
consumers. Rejected because the workspace would not compile
between the two steps, which makes the change harder to review and
to bisect if a regression is introduced.

## Source-Derived File Migration Map

Every path below is observed in the current working tree at
`branch = refactor/my-cms-api`, `HEAD = dbc56ceaf1363c5c848dd4c54f6a3a08ba2d4a6e`.

### Post command handlers (legacy → canonical)

| Legacy path                                                            | Canonical path                                                          | Action   |
| ---------------------------------------------------------------------- | ----------------------------------------------------------------------- | -------- |
| `apps/api/application_core/src/commands/post/mod.rs`                   | `apps/api/domain_posts/src/handlers/post/mod.rs`                        | delete   |
| `apps/api/application_core/src/commands/post/create/mod.rs`            | `apps/api/domain_posts/src/handlers/post/create/mod.rs`                 | delete   |
| `apps/api/application_core/src/commands/post/create/create_handler.rs`  | `apps/api/domain_posts/src/handlers/post/create/create_handler.rs`       | delete   |
| `apps/api/application_core/src/commands/post/create/create_request.rs` | `apps/api/domain_posts/src/handlers/post/create/create_request.rs`      | delete   |
| `apps/api/application_core/src/commands/post/read/mod.rs`               | `apps/api/domain_posts/src/handlers/post/read/mod.rs`                    | delete   |
| `apps/api/application_core/src/commands/post/read/read_handler.rs`     | `apps/api/domain_posts/src/handlers/post/read/read_handler.rs`          | delete   |
| `apps/api/application_core/src/commands/post/modify/mod.rs`            | `apps/api/domain_posts/src/handlers/post/modify/mod.rs`                 | delete   |
| `apps/api/application_core/src/commands/post/modify/modify_handler.rs` | `apps/api/domain_posts/src/handlers/post/modify/modify_handler.rs`      | delete   |
| `apps/api/application_core/src/commands/post/modify/modify_request.rs` | `apps/api/domain_posts/src/handlers/post/modify/modify_request.rs`      | delete   |
| `apps/api/application_core/src/commands/post/delete/mod.rs`            | `apps/api/domain_posts/src/handlers/post/delete/mod.rs`                 | delete   |
| `apps/api/application_core/src/commands/post/delete/delete_handler.rs` | `apps/api/domain_posts/src/handlers/post/delete/delete_handler.rs`      | delete   |

### Post translation pipeline (legacy → canonical)

| Legacy path                                                                   | Canonical path                                                  | Action   |
| ----------------------------------------------------------------------------- | --------------------------------------------------------------- | -------- |
| `apps/api/application_core/src/commands/ai/mod.rs`                            | n/a (whole `ai` module removed)                                 | delete   |
| `apps/api/application_core/src/commands/ai/README.md`                         | n/a                                                             | delete   |
| `apps/api/application_core/src/commands/ai/translate/mod.rs`                  | `apps/api/domain_posts/src/handlers/post/translate/mod.rs`      | delete   |
| `apps/api/application_core/src/commands/ai/translate/translate_handler.rs`    | `apps/api/domain_posts/src/handlers/post/translate/translate_handler.rs` | delete |
| `apps/api/application_core/src/commands/ai/translate/translate_request.rs`    | `apps/api/domain_posts/src/handlers/post/translate/translate_request.rs` | delete |
| `apps/api/application_core/src/commands/ai/translate/translate_response.rs`   | `apps/api/domain_posts/src/handlers/post/translate/translate_response.rs` | delete |
| `apps/api/application_core/src/commands/ai/vector_store_pg.rs`                | `apps/api/domain_posts/src/handlers/vector_store/vector_store_pg.rs` | delete |

### Post HTTP adapters (re-point, do not delete)

| File                                                                | Change                                                                                                |
| ------------------------------------------------------------------- | ----------------------------------------------------------------------------------------------------- |
| `apps/api/src/api/post/create/create_handler.rs`                   | `use application_core::commands::post::create::{...}` → `use domain_posts::handlers::post::create::{...}` |
| `apps/api/src/api/post/read/read_handler.rs`                       | `use application_core::commands::post::read::read_handler::{...}` → `use domain_posts::handlers::post::read::read_handler::{...}` |
| `apps/api/src/api/post/modify/modify_handler.rs`                   | `use application_core::commands::post::modify::{...}` → `use domain_posts::handlers::post::modify::{...}` |
| `apps/api/src/api/post/delete/delete_handler.rs`                   | `use application_core::commands::post::delete::delete_handler::{...}` → `use domain_posts::handlers::post::delete::delete_handler::{...}` |
| `apps/api/src/api/post/translate/translate_handler.rs`             | `use application_core::commands::ai::translate::{...}` + `application_core::commands::ai::vector_store_pg::VectorStore` → `use domain_posts::handlers::post::translate::{...}` + `domain_posts::handlers::vector_store::VectorStore` |
| `apps/api/src/api/post/translate/job_handler.rs`                   | `use application_core::entities::translation_jobs;` → `use domain_posts::entities::translation_jobs;` (and the test module's `use application_core::entities::{categories, posts, sea_orm_active_enums::CategoryType};` → `use domain_posts::entities::{categories, posts, sea_orm_active_enums::CategoryType};`) |
| `apps/api/src/api/delete/delete_handler.rs`                         | `use application_core::commands::post::delete::delete_handler::{...}` → `use domain_posts::handlers::post::delete::delete_handler::{...}` |
| `apps/api/src/api/tag/delete/delete_handler.rs`                    | `use application_core::commands::post::delete::delete_handler::{...}` → `use domain_posts::handlers::post::delete::delete_handler::{...}` |

### Application-core entry-point edits (re-point, do not delete)

| File                                                                | Change                                                                                                |
| ------------------------------------------------------------------- | ----------------------------------------------------------------------------------------------------- |
| `apps/api/application_core/src/commands/mod.rs`                     | Remove `pub mod post;` and `pub mod ai;` lines                                                          |
| `apps/api/application_core/src/entities/mod.rs`                     | Update docstring to call out that the shim is purely for the legacy `cms::api::{media, user, administrator}::*` and `test_helpers` callers; the canonical entities are in `domain_posts::entities::*` |

### Duplicate migration files (delete; the lib.rs shim is the only consumer)

| File                                                                | Action   |
| ------------------------------------------------------------------- | -------- |
| `apps/api/migration/src/m20240409_151952_release_100.rs`            | delete   |
| `apps/api/migration/src/m20250330_151455_release_110.rs`            | delete   |
| `apps/api/migration/src/m20260126_040610_release_300.rs`            | delete   |
| `apps/api/migration/src/m20260531_000001_pgvector.rs`               | delete   |
| `apps/api/migration/src/constants.rs`                               | delete   |
| `apps/api/migration/src/lib.rs`                                     | retain (already a re-export shim) |
| `apps/api/migration/src/main.rs`                                    | retain (CLI entry point)         |
| `apps/api/migration/Cargo.toml`                                     | retain unchanged                 |

### Files unaffected by this change

| Path                                                                                | Why                                                                                                                            |
| ----------------------------------------------------------------------------------- | ------------------------------------------------------------------------------------------------------------------------------ |
| `apps/api/domain_posts/**/*`                                                         | Already canonical; no change                                                                                                   |
| `apps/api/domain_auth/**/*`                                                          | Out of scope (auth domain)                                                                                                     |
| `apps/api/domain_interface/**/*`                                                     | Out of scope (contract crate)                                                                                                  |
| `apps/api/gateway/**/*`                                                              | Already composes `domain_posts::DomainPostService`; no change                                                                  |
| `apps/api/src/bin/legacy_bootstrap.rs`                                               | Unchanged at the source level; the routes it wires still resolve through the (re-pointed) `cms::api::post::*` adapter functions |
| `apps/api/src/lib.rs`                                                                | `AppState` type is unchanged; `application_core::commands::{media,user}::*` imports are unaffected                          |
| `apps/api/src/api/{media,user,administrator,public,delete,tag}/**/*` (except the two `delete_handler.rs` files) | Out of scope; these are media/user/administrator/tag concerns that are part of a future extraction                 |
| `apps/api/src/api/post/{create,read,modify,delete,translate}/*` HTTP adapter files   | Function bodies are updated (Decision 1); function signatures, route table, and Axum types are unchanged                     |
| `apps/api/test_helpers/**/*`                                                         | Already uses `migration::{Migrator, MigratorTrait}` (which resolves through the shim); no change                               |

## Risks / Trade-offs

- **[Risk] Missed `use` site** — a stale `use
  application_core::commands::post::*` or
  `use application_core::commands::ai::*` import could be left
  behind if a downstream file (e.g. an integration test, a
  `wiremock` fixture, or a `dev-dependencies` test) is not
  covered by the workspace-wide search in the task list. →
  **Mitigation:** every task group runs `cargo check --workspace`
  after the imports are re-pointed; `cargo check` fails on a stale
  import. The repository verification gate
  (`cargo test --workspace --lib --bins`) at the end of the change
  catches any test-side import that escapes the per-file sweep.
- **[Risk] Test fixture `fake_create_post_request` is removed but a
  consumer was missed** — the test fixture is gated by
  `#[cfg(test)]` on the `application_core` crate, so it is only
  visible to `application_core`'s own test build. The migrated
  `domain_posts::handlers::post::create::create_handler.rs` test
  block is a placeholder with no test bodies. → **Mitigation:**
  workspace-wide search for `fake_create_post_request` confirms
  zero remaining references outside the deletion target. The
  `domain_posts` test build does not import the legacy fixture.
- **[Risk] `application_core::Cargo.toml` still declares the
  `domain_posts` path dependency but no `application_core` source
  file imports it** — `application_core::entities` is a re-export
  shim, so the dependency is still needed. → **Mitigation:** the
  dependency is retained; the `Cargo.toml` is unchanged.
- **[Risk] `legacy_bootstrap` silently changes the runtime
  behavior of a route** because the new command handler resolves
  through a different `AppError` type path or a different entity
  set. → **Mitigation:** the canonical
  `domain_posts::handlers::post::create::create_handler` (etc.) is
  byte-for-byte equivalent to the legacy
  `application_core::commands::post::create::create_handler` (etc.)
  (verified by direct diff). The `AppError` type is the
  `domain_posts::domain::error::AppError` (a `thiserror::Error`
  enum with the same variants the legacy handler returns); the
  entities are the same `domain_posts::entities::{posts,
  post_tags, post_translations, translation_jobs}` types. No
  variant of `AppError` is added or removed by the move.
- **[Risk] The `migration` binary
  (`cargo run -p migration -- migrate`) silently skips a
  migration** because the `lib.rs` re-export shim misses one of
  the four identities. → **Mitigation:** the shim uses
  `pub use domain_posts::migrations::*;` which re-exports the
  `Migrator` struct and all four migration modules;
  `domain_posts::migrations::Migrator::migrations()` returns all
  four in order. The post-change
  `cargo run -p migration -- migrate --list` (verifiable against
  any testcontainer) prints the four IDs in the same order as the
  pre-change CLI.

## Migration Plan

This change is a code-organization refactor; there is no schema
migration, no data migration, and no deployment ordering concern.
The change is applied as a single PR.

**Rollback:** if a regression is detected after the change lands,
the rollback is `git revert <merge-sha>`; the pre-change state
restores the legacy command handlers, the legacy `cms::api::post::*`
imports, the legacy `application_core::commands::ai` module, and
the duplicate `apps/api/migration/src/*.rs` files.

**Deployment ordering:** none. The two binaries that consume the
post-domain code (`legacy_bootstrap` and `my-cms-api` via the
gateway) are deployed in the same way they are deployed today; no
canary or feature flag is required because the change has zero
observable behavior.

## Open Questions

None. The change is fully constrained by the source-derived file
migration map above. Every legacy path has a canonical
counterpart (or is removed wholesale), every consumer has a
re-pointed import, and every Cargo dependency is preserved or
removed explicitly.
