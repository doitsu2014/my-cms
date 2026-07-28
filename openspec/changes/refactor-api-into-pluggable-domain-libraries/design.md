## Context

The current API workspace `apps/api/` is a single crate `cms` with submodules `api`, `common`, `presentation_models`, and the `bin` `my-cms-api`, plus a workspace member `application_core` that owns all command handlers, DTOs, and SeaORM entities, and a workspace member `migration` that owns the SeaORM migrator. The `my-cms-api` bootstrap file `apps/api/src/bin/my-cms-api.rs` is 331 lines and simultaneously: loads env vars, configures OpenTelemetry, builds three `AppState` instances in `construct_app_state()`, builds three `Router` instances in `public_router()`, `protected_router()`, and `protected_administrator_router()`, applies Supabase auth + CORS + cookie + body-limit + tracing layers, and binds/serve the listener. The shared `AppState` in `apps/api/src/lib.rs` exposes `conn`, `media_config`, `media_cache`, `bucket_visibility_cache`, both Seaography schemas, and `supabase_admin_client`, so the gateway holds a domain-leakage surface for every cross-cutting concern.

`application_core` exports one giant `commands::*` tree containing all post, category, tag, media, AI, and user handler implementations. Cross-domain calls are visible: `commands::post::create::create_handler` and `commands::category::create::create_handler` call `commands::tag::create::create_handler::TagCreateHandler`; `commands::post::delete::delete_handler` is reused for tag deletion by `apps/api/src/api/tag/delete/delete_handler.rs`; `commands::category::tests` imports `commands::tag::tests::fake_tag_names`; and `apps/api/src/api/post/translate/job_handler.rs` reaches into `application_core::entities::translation_jobs`. The migration crate `apps/api/migration/src/lib.rs` declares four ordered migrations (`m20240409_release_100`, `m20250330_release_110`, `m20260126_release_300`, `m20260531_pgvector`). Generated entities live under `apps/api/application_core/src/entities/` and are imported across the whole `application_core` tree.

Existing canonical specs in `openspec/specs/` (auth, supabase storage, pgvector, media bucket management, image transformation, local dev, user management) must remain applicable after the refactor. This design is a behavior-preserving re-slicing: same routes, same methods, same auth, same response envelopes, same SeaORM schema. No GraphQL, REST, or database contract changes are introduced.

`code-review-graph` MCP was not callable in this session; the map and graph gate fell back to repository search and `cargo metadata`. No graph findings are fabricated; call sites listed in the proposal and tasks were verified via `rg` and direct file reads.

## Goals / Non-Goals

**Goals:**
- Define a `domain-interface` library that every domain implements, enabling the gateway to register routes/health/migration/GraphQL through a dyn-compatible trait.
- Make `bin/my-cms-api` a thin composition root with no domain logic and no domain imports.
- Extract a self-contained `domain-post` library that owns the Blog Post Service.
- Preserve all current public and protected route paths, auth roles, error mappings, SeaORM schema, and external integrations.
- Allow new domains to be added without gateway route edits, only composition manifest updates.
- Keep the existing `cargo check && cargo test && cargo fmt -- --check && cargo clippy` gate green at every step.
- Keep migration ordering deterministic and remain schema-first.
- Provide a domain scaffold so new services follow the same pattern.

**Non-Goals:**
- Replacing SeaORM, Axum, the Seaography GraphQL pipeline, Supabase auth, or Supabase storage.
- Splitting the database or introducing cross-service transactions.
- Changing the public REST/GraphQL contract, response envelopes, or migration identity ordering.
- Replacing the OpenAI integration or moving translation to a separate runtime.
- Implementing the refactor in this OpenSpec change (this design only).

## Decisions

### Decision 1 — Single workspace at `apps/api/`

Keep the existing top-level `apps/api/Cargo.toml` `[workspace]` and add new members. The Rust workspace boundary is the cheapest way to enforce compile-time decoupling without introducing a new super-workspace. Rejected alternatives: (a) a new top-level `workspace/` Cargo workspace that detaches `apps/api` from existing build scripts; rejected because it requires moving testcontainers, wiremock, and the Supabase testcontainers module dependencies, and the change is significantly larger. (b) feature-flagging the legacy `cms` lib; rejected because the goal is to retire the monolith, not coexist with it.

Migration contract: `apps/api/Cargo.toml` `members` becomes `["application_core", "migration", "test_helpers", "domain_interface", "domain_foundation", "domain_post", "my_cms_api"]`. `application_core` and the original `cms` lib are deprecated as soon as each replacement exists; the migration bin from `apps/api/migration/src/main.rs` continues to work as a development-time CLI for the orchestrator.

### Decision 2 — `domain-interface` library (`apps/api/domain_interface/`)

Defines `DomainService` plus shared types so domains and the gateway can communicate without an import on domain internals. Selected because a thin trait interface is dyn-compatible, gives a registry, and keeps the API contract stable while the implementation moves. Rejected: dynamic Rust plugin loading (`inventory`/`abi_stable`), rejected because the operational cost (separate compilation units, static linking) outweighs the benefit for a single binary and would require redesigning tracing and error mapping.

```rust
// apps/api/domain_interface/src/lib.rs (sketch — design only, not code to ship here)

use std::sync::Arc;
use async_trait::async_trait;
use axum::Router;
use sea_orm::DatabaseConnection;
use async_graphql::dynamic::Schema;

#[derive(Clone, Debug)]
pub struct DomainContext {
    pub conn: Arc<DatabaseConnection>,
    pub foundation: Arc<FoundationServices>,
}

pub struct FoundationServices {
    pub media_config: Arc<application_core::commands::media::MediaConfig>,
    pub media_cache: Arc<moka::future::Cache<MediaCacheKey, CachedMedia>>,
    pub bucket_visibility_cache: Arc<moka::future::Cache<String, bool>>,
    pub supabase_admin_client: Arc<application_core::commands::user::supabase_admin_client::SupabaseAdminClient>,
    pub graphql_immutable_schema: Arc<Schema>,
    pub graphql_mutable_schema: Arc<Schema>,
}

#[derive(Copy, Clone, Debug, Eq, PartialEq)]
pub enum Mount {
    Public,
    Protected,
    Administrator,
}

pub struct RouteRegistration {
    pub mount: Mount,
    pub router: Router<DomainContext>,
    pub required_roles: Vec<String>,
}

pub struct MigrationDescriptor {
    pub id: &'static str,
    pub depends_on: &'static [&'static str],
    pub run: Box<dyn Fn(Arc<DatabaseConnection>) -> futures::future::BoxFuture<'static, Result<(), AppError>> + Send + Sync>,
}

#[async_trait]
pub trait DomainService: Send + Sync {
    fn name(&self) -> &'static str;
    fn version(&self) -> &'static str;
    fn required_env(&self) -> &'static [&'static str];
    fn validate_config(&self) -> Result<(), AppError>;
    fn migrations(&self) -> Vec<MigrationDescriptor>;
    fn register_routes(&self, ctx: &DomainContext) -> Vec<RouteRegistration>;
    async fn health(&self, ctx: &DomainContext) -> Result<(), AppError>;
}
```

`Mount` and `required_roles` are explicit so the gateway can build the three existing router groups without leaking domain knowledge. `required_env` enables the gateway to validate env at startup. `MigrationDescriptor` carries dependencies so ordering stays deterministic.

### Decision 3 — `domain-foundation` library (`apps/api/domain_foundation/`)

Holds the cross-cutting capabilities that multiple domains share: `AppError` (lifted from `application_core::common::app_error`), `SupabaseAuthLayer` (lifted from `apps/api/src/common/supabase_auth.rs`), `ApiResponseWith`/`ApiResponseError` (lifted from `apps/api/src/presentation_models/api_response.rs`), CORS/Cookie/BodyLimit/Otel layer factories, and Supabase Storage/Admin client constructors. The foundation is a regular library, not a trait, so domains can `use domain_foundation::error::AppError` directly.

Rejected: a runtime of macros that injects these into every domain. Rejected because the macro approach hides the dependency rather than removing it and complicates error diagnostics.

### Decision 4 — `domain-post` library (`apps/api/domain_post/`)

Owns the Blog Post Service vertical slice: post HTTP adapters, command handlers, DTOs, post-relevant common helpers, generated entities for `posts`, `post_tags`, `post_translations`, `translation_jobs`, `tags` (only because posts depend on tag creation), categories (read-only dependency), Seaography contribution for the post aggregate, and the post/translation/pgvector migrations. Selected because it is the largest domain in the source and exercises every cross-cut (REST, GraphQL, migrations, OpenAI, pgvector, JWT-protected routes, public routes via the GraphQL mount). It is the highest-risk extraction and therefore the one that validates the interface.

Rejected: extracting `domain-media` first. Rejected because media has the most cross-domain code (binds buckets, image rendering, AI models, GraphQL), which would slow the first vertical slice and risk delaying observable benefit. The post service is still substantial and the interface is the same regardless of which domain goes first.

Cross-domain call policy: `domain-post` may import a small set of foundation helpers (error, presentation models, JWT extension) and may call into `domain-foundation` for Supabase Admin/Storage clients. It MUST NOT import another domain's commands. If a domain truly needs cross-domain logic (e.g., post creating a tag), the dependency is resolved by lifting the helper into `domain-foundation` or by event-driven design in a future change. This rule is a spec invariant (see `domain-service-interface` scenarios).

### Decision 5 — Composition root in `my_cms_api` bin (`apps/api/my_cms_api/`)

The `bin` becomes a thin composition root: load env, init tracing, build one `DatabaseConnection`, build foundation services once, construct the `DomainContext`, build the three Axum routers by iterating `Vec<Box<dyn DomainService>>`, apply the auth/CORS/cookie/tracing layers, run the orchestrator's `run_migrations` on demand or at startup, and serve. The `Cargo.toml` `[[bin]]` name remains `my-cms-api` for backward compatibility with deployment images and `deployments/docker-swarm/`. The current `apps/api/src/bin/my-cms-api.rs` and `apps/api/src/lib.rs` are deleted once the new bootstrap is in place. Rejected: a builder DSL — the gateway is small enough that a function is enough.

### Decision 6 — Migration orchestration

The `domain-interface` defines `MigrationDescriptor` and the `domain-post` exposes its own descriptors. The gateway (or a small `migration_orchestrator` inside `domain_foundation`) sorts descriptors by `depends_on`, deduplicates by `id`, and runs them against the shared `DatabaseConnection`. The legacy `apps/api/migration/src/main.rs` bin keeps working as a development tool by calling into the orchestrator with the same descriptors. The current migration identities (`m20240409_151952_release_100`, `m20250330_151455_release_110`, `m20260126_040610_release_300`, `m20260531_000001_pgvector`) are preserved exactly so the database state is unchanged. The `m20260531_000001_pgvector` migration is owned by the post domain because the only consumer is `translation_jobs` and the embeddings table.

Rejected: a single global `Migrator` (current). Rejected because the entire point of the refactor is for each domain to own its migrations. Rejected: a separate database per domain. Out of scope (see Non-Goals).

### Decision 7 — SeaORM entity generation and Seaography

`sea-orm generate entity` continues to run against the migration set as a single output target: `apps/api/application_core/src/entities/`. The `application_core` crate remains the single source of truth for generated entities during the transition. Domains depend on `application_core::entities` for the entities they need and re-export their domain-scoped prelude from their own library. The `sea-orm` codegen output path can be made a per-domain directory in a later change, but that is out of scope for this refactor because the same migration set produces the same set of entities, and Seaography still needs the full `EntityTrait` set on startup. Rejected: per-domain generated entity trees. Rejected for this change because it duplicates generation output and complicates the entity-regeneration script.

### Decision 8 — GraphQL contribution

`Seaography` already takes a `DatabaseConnection` and returns a `Schema`. To preserve the existing mounts (`/graphql/immutable`, `/graphql/mutable`) without giving the gateway access to entity internals, the foundation builds the schemas once via a callback supplied by the gateway. The foundation's `FoundationServices` exposes `Arc<Schema>` for both. The domain-post service still contributes the post-specific Seaography module (entities + filter logic) through a `pub fn contribute_post_schema(...)` exported from the domain library, called by a `graph_root_builder` function in the foundation that the gateway calls at startup. This is the smallest deviation that keeps `dyn DomainService` clean.

Rejected: building schemas entirely inside `domain-post`. Rejected because the gateway still needs to expose both endpoints and the same `Schema` value type, which means the foundation must own the wiring. Rejected: replacing Seaography. Out of scope.

### Decision 9 — Cross-cutting layers (auth, CORS, tracing, body limits, cookies)

These stay in `domain_foundation` and are applied once in the gateway's composition. The auth layer continues to read `SUPABASE_URL`, `SUPABASE_INTERNAL_URL`, `SUPABASE_JWT_SECRET`, `AUTHORIZATION_AUDIENCE`, and the role lists. The CORS layer keeps its current permissive `Any` policy, with a follow-up improvement tracked but not in scope. The `OtelAxumLayer`/`OtelInResponseLayer` continue to wrap every router. Body limit and cookie layers behave identically.

### Decision 10 — Test helpers

`test_helpers` stays in the workspace and depends on `application_core` and `domain_post` for backward compatibility. New tests for the gateway and `DomainService` implementations live next to the trait or in the foundation crate. Existing module-local unit tests stay where they are; only their `use` paths change to the new crate names.

### Decision 11 — New-domain scaffold

A new domain is created by copying the `apps/api/domain_post/` skeleton into `apps/api/domain_<name>/`, replacing `post` identifiers, registering the crate in `apps/api/Cargo.toml` `members`, and appending a `Box::new(DomainXService::new(...))` to the gateway's composition manifest. The scaffold is committed under `apps/api/templates/domain_template/` (a Cargo workspace member with a README) so it stays in sync. A `docs/adding-a-domain.md` (under the `openspec/specs/local-dev-environment/` or a new spec) documents the steps.

Rejected: a procedural macro that generates the crate. Rejected because copy-paste of an explicit, readable template is faster to understand, and codegen hides error diagnostics.

## Risks / Trade-offs

- [Risk] Three `AppState` instances today become one `DomainContext` — any divergence in env/connection lifetime could change behavior. → Mitigation: a single `connect_database()` helper and a one-shot context build in the gateway; tests assert one connection pool, three router groups, and identical health response.
- [Risk] Cross-domain call from post to tag delete today is hidden behind `apps/api/src/api/tag/delete/delete_handler.rs` calling `commands::post::delete::delete_handler::api_delete_post`. → Mitigation: the refactor replaces this with a tag-domain handler that lives in its own domain (out of scope for `domain-post`); the current behavior is preserved by keeping the same HTTP route in the tag adapter moved into `domain-tag` (follow-up change). For this change, post-domain does not delete tags.
- [Risk] Seaography entity registration currently lives in `application_core::graphql::query_root` and references every entity; splitting this without breaking the public schema. → Mitigation: the contribution callback in the foundation keeps the existing entity set and the same `schema(conn, None, None, false)` / `true()` calls. Verified by snapshotting the GraphQL SDL before/after the refactor.
- [Risk] `migration` crate has both a lib and a bin. After the refactor, the orchestrator lives in the foundation and the bin keeps its CLI. → Mitigation: keep the bin's `pub fn` surface stable; the orchestrator is a thin shim.
- [Risk] Generated entities imported by `commands::*` change path when the new foundation/error module is moved. → Mitigation: move entities as a final, mechanical pass; `rg` shows every import of `application_core::entities::*` and the changes are scripted.
- [Risk] `code-review-graph` MCP unavailable. → Mitigation: explicit fallback to `rg`, `cargo metadata`, and `openspec list --json`; documented in proposal and tasks.
- [Risk] Dev hot-reload (`cargo run` from `apps/api/`) may need a re-run after `Cargo.toml` `members` change. → Mitigation: documented in the rollout; the verification gate is the canonical completion signal.
- [Risk] Two `Seaography` schemas are built twice today; the orchestrator must still build them only once. → Mitigation: foundation owns the schemas; the gateway references them.

## Migration Plan

1. **Workspace scaffold** (no behavior change): add `domain_interface`, `domain_foundation`, `domain_post`, and `my_cms_api` members to `apps/api/Cargo.toml`. Build with `cargo check`. Old code still compiles; old bin still runs.
2. **Interface and foundation** (no behavior change): add the trait, `DomainContext`, `FoundationServices`, `MigrationDescriptor`, layer factories, and `AppError` re-export. New crates build; old code still compiles.
3. **Gateway composition root** (no behavior change): add `my_cms_api` bin that builds the same three routers via the foundation factories and the same `construct_app_state` payload, but exposes them through a `Vec<Box<dyn DomainService>>` populated by a single test domain (a no-op `Hello` domain that only registers `/`). Old `my-cms-api` is removed only after `my_cms_api` matches behavior.
4. **Post extraction** (no behavior change): move post HTTP adapters, commands, post-relevant DTOs, post-related migrations, and Seaography contribution callback into `domain-post`. Foundation provides the layers. Post routes still work. Translation and job routes still work.
5. **Decommission** (no behavior change): remove `apps/api/src/lib.rs` and `apps/api/src/api/{post,category,tag,media,...}/**` files replaced by domain libraries. Remove unused `application_core::commands::post`, `category`, `tag`, `media`, `ai`, `user` submodules after they are moved. Entity import paths in remaining files are updated.
6. **Rollout and rollback**: each step is a separate PR. The repository verification gate (`cargo check && cargo test && cargo fmt -- --check && cargo clippy`) is the go/no-go signal. Rollback is `git revert` of the latest PR. A follow-up change archives this one and adds the next domain.

Verification per step:
- `cargo check -p domain_interface -p domain_foundation -p domain_post -p my_cms_api`
- `cargo test -p domain_post --no-fail-fast`
- `cargo test -p application_core --no-fail-fast` (until decommissioned)
- `cargo test -p my_cms_api` (router + health integration)
- `cargo fmt -- --check && cargo clippy --all-targets --all-features`
- `pnpm --dir apps/web build` (frontend contract untouched)

## Open Questions

- Is `domain-foundation` the right name, or should it stay `domain-shared`? The user wrote "common" — the design uses `domain_foundation` to avoid confusion with the existing `common` module. Confirm before implementation.
- Should the `presentation_models` re-export live in `domain-foundation` or remain in the per-domain library? Default: per-domain re-export; foundation exposes only the trait + error + layers.
- Should the legacy `cms` lib and `application_core` be removed in this change, or kept as a compatibility shim that re-exports from the new crates? Default: keep `application_core` until the post domain is fully extracted, then remove the migrated submodules. Confirm before implementation.
- Should a new capability spec `api-gateway-bootstrap` be archived to `openspec/specs/` after the refactor lands, or remain only inside this change? Default: archive as a new canonical spec once verified.
- Should the deprecated `apps/api/src/bin/my-cms-api.rs` be removed in the same PR as the new `my_cms_api` bin, or two PRs? Default: two PRs — keep the old one compiling until the new one passes the verification gate.
