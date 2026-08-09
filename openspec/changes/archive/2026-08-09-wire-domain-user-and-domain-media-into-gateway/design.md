## Context

### Source-derived current state (revalidated 2026-08-09)

The API workspace (`apps/api/Cargo.toml:1-2`) is a Cargo workspace with seven
members: `domain_auth`, `domain_interface`, `domain_media`, `domain_posts`,
`domain_user`, `gateway`, `test_helpers`. The `gateway` crate produces the
`my-cms-api` binary and is documented as the sole runtime surface for
production deployments, but its composition root does not yet register the
media and user domains.

**Gateway composition root** (`apps/api/gateway/src/main.rs:44-49`): the
manifest currently registers two domain services. There is no
`domain_media` or `domain_user` import anywhere in the gateway crate
(`apps/api/gateway/Cargo.toml:13-16` lists only `domain_interface`,
`domain_posts`, `domain_auth`). The gateway therefore cannot compile a
registration that includes `DomainMediaService` or `DomainUserService`.

**`DomainMediaService` already exists** (`apps/api/domain_media/src/service.rs:24-93`).
It owns `MediaApiState`, exposes `migrations() -> Vec::new()` (line 79-82),
and `register_routes()` returns the public/protected/administrator routers
from `apps/api/domain_media/src/api/routes.rs`. The service is constructed via
`DomainMediaService::new(Arc<MediaConfig>)`; the missing piece is a
`MediaConfig::from_env()` factory (no such factory exists today; every
caller constructs `MediaConfig` inline).

**`domain_user` lacks a service** (`apps/api/domain_user/src/lib.rs:1-21`):
the crate exposes `domain`, `dto`, `handlers`, `observability`. There is no
`service.rs`, no `api/` aggregator, and no re-export of a `DomainUserService`.
The seven handler modules in `apps/api/domain_user/src/handlers/{create,delete,modify,read_list,read_one,reset_password}/mod.rs`
expose command-handler structs + traits (e.g. `CreateUserHandler`,
`CreateUserHandlerTrait`) but no HTTP adapter functions — the HTTP layer is
introduced by this slice via a thin aggregator.

**Handler adapter shape precedent** — `domain_posts::api::post::create::create_handler.rs:13`
exposes `pub async fn api_create_post(State(ctx), Extension(actor), Json(body)) -> impl IntoResponse`
that constructs a `PostCreateHandler` from `ctx.conn` and calls the
`PostCreateHandlerTrait::handle_create_post` method. The user-domain adapter
mirrors this: each route is a thin Axum function that constructs the handler
from `UserApiState.supabase_admin_client` and calls the trait method.

### Constraints (from AGENTS.md, the parent change, and pre-existing findings)

- Honour the layered architecture (gateway / domain_* / domain_interface).
- Use the existing `domain_interface::DomainService` contract verbatim —
  do not modify `apps/api/domain_interface/src/lib.rs` in this slice.
- Use `DomainConfigError::MissingEnv(<var>)` for env failures
  (`apps/api/domain_interface/src/lib.rs:107`).
- `MediaConfig::from_env` MUST fail-fast via `DomainConfigError::MissingEnv`,
  not `unwrap`/`expect`.
- Migration orchestrator and `domain_posts` bin are out of scope for this
  slice (owned by `gateway-migrate-cli-and-delete-domain-posts-bin`).
- Dockerfile and docs are out of scope (owned by
  `single-binary-docker-image-and-docs`).
- Pre-existing `async_std::test` failures in `domain_posts` / `domain_media`
  / `domain_user` are unrelated; do not regress them.

### Active overlap

`purge-legacy-cms-and-application-core` (41/47 tasks) explicitly defers media
and user wiring to a separate follow-up change
(`openspec/changes/purge-legacy-cms-and-application-core/proposal.md:49`).
This slice is that follow-up; the two changes touch **non-overlapping files**
on the Rust side and the in-progress change is finalising doc-only tasks.

## Goals / Non-Goals

**Goals**

- `gateway::manifest()` returns exactly four domain services.
- `DomainUserService` implements `domain_interface::DomainService` with the
  same surface shape as `DomainMediaService`.
- The seven user routes (one `POST /users`, one `GET /users`, one
  `GET /users/:id`, one `PUT /users/:id`, one `DELETE /users/:id`, one
  `POST /users/:id/reset-password`) are mounted on `Mount::Administrator`.
- `MediaConfig::from_env()` is the single canonical env-to-`MediaConfig`
  factory; it returns `Result<MediaConfig, DomainConfigError>`.
- The `domain_media` and `domain_user` crates are added to
  `gateway/Cargo.toml` `[dependencies]` in alphabetical order.
- `domain_user` does NOT gain a `[[bin]]` or `src/main.rs` (lib-only).
- All existing tests in `domain_user`, `domain_media`, and `gateway`
  continue to pass.
- Verification gate (per AGENTS.md §"Verify Before Commit"):
  `cargo check`, `cargo test -p domain_user -p domain_media -p gateway`,
  `cargo fmt -- --check`, `cargo clippy --all-targets -- -D warnings`.

**Non-Goals**

- No change to `domain_interface::DomainService`. The trait gains NO new
  methods in this slice.
- No change to the migration orchestrator. The hard-coded
  `if d.id.starts_with("m2024") || d.id.starts_with("m2026")` branch in
  `apps/api/gateway/src/main.rs:77-85` stays for now; Slice 2 refactors it.
- No CLI subcommand on `my-cms-api`. Slice 2 adds `migrate`.
- No Dockerfile change. Slice 3 retargets the container image.
- No docs change. Slice 3 updates the five doc files.
- No new HTTP routes. The seven user routes are already declared in the
  pre-existing handler modules; this slice only wires them.
- No new env vars. The factory reads `SUPABASE_URL`,
  `SUPABASE_SERVICE_ROLE_KEY`, `MEDIA_BUCKET`, `MEDIA_BASE_URL` — all of
  which `domain_media` and `domain_user` already depend on at runtime.

## Decisions

### Decision 1: Inline Axum adapters in `domain_user::api::routes`

**Driver.** The parent change's `tasks.md:1.4` assumes each user handler
module exports a free `pub(super) fn handler(...)` symbol. Direct inspection
shows the modules expose only `*Handler` structs and `*HandlerTrait`s
(`apps/api/domain_user/src/handlers/create/create_handler.rs:18-19`,
`:delete/delete_handler.rs:17-18`, `:modify/modify_handler.rs:25-26`,
`:read_list/read_list_handler.rs:19-20`, `:read_one/read_one_handler.rs:18-19`,
`:reset_password/reset_password_handler.rs:18-19`). The HTTP layer is
deliberately separated from the command-handler layer in this codebase (see
`apps/api/domain_posts/src/api/post/create/create_handler.rs:13-28` for the
canonical pattern).

**Decision.** Add the six HTTP adapter functions inline in
`apps/api/domain_user/src/api/routes.rs` (one per route, mirroring
`domain_posts::api::post::create::create_handler`). Each function:
- Takes `State<UserApiState>`, `Extension(AuthenticatedActor)`, and the typed
  request body / path param.
- Constructs the corresponding `*Handler { supabase: state.supabase_admin_client.clone() }`.
- Calls the `*HandlerTrait::handle_*` method.
- Wraps the `Result<_, AppError>` in the existing
  `ApiResponseWith` / `ApiResponseError` envelope (the same envelope used by
  every other domain's HTTP adapter).

**Alternatives considered.**
- (a) **Add `api_create_user` / `api_get_user_list` / ... one-per-handler
  files in submodules**, matching `domain_posts::api::post::create`. *Rejected*:
  more files than routes (6 extra files), each containing ~15 lines. The
  inline approach is equivalent semantically and ~75% smaller.
- (b) **Re-implement the handlers inside the Axum functions.** *Rejected*:
  duplicates `domain_user::handlers::*`; violates the layered architecture.

**Consequences.** The new `routes.rs` is ~110 lines. Every HTTP adapter is a
six-line Axum function that constructs a handler struct from `UserApiState`
and calls the trait method. The command-handler layer (in
`apps/api/domain_user/src/handlers/`) is untouched.

**Contracts.** No new HTTP contract — the seven routes preserve the same
paths, methods, and envelope shape as the legacy `legacy_bootstrap` binary
served (per `domain-api-cutover` requirement "Observable API compatibility").

### Decision 2: `MediaConfig::from_env` factory

**Driver.** `MediaConfig` (`apps/api/domain_media/src/handlers/mod.rs:29-34`)
holds `storage: SupabaseStorage`, `bucket: String`, `media_base_url: String`.
The fields map 1:1 onto env vars (`SUPABASE_URL` + `SUPABASE_SERVICE_ROLE_KEY`
construct `SupabaseStorage`; `MEDIA_BUCKET` and `MEDIA_BASE_URL` are strings).
Today every caller (test fixtures, the gateway when it eventually wires
media) constructs `MediaConfig` inline.

**Decision.** Add `pub fn from_env() -> Result<MediaConfig, DomainConfigError>`
to `apps/api/domain_media/src/handlers/mod.rs`. The factory reads four env
vars, returns `DomainConfigError::MissingEnv(<var>)` on the first missing one,
and uses `SupabaseStorage::new(supabase_url, service_role_key)` to build the
storage client. The factory is the only canonical env-to-`MediaConfig`
converter; the gateway's `main` calls it once at startup.

**Alternatives considered.**
- (a) **Inline `MediaConfig { ... }` construction in `gateway::main`.**
  *Rejected*: scatters env-reading across the workspace; violates the
  pattern in `domain_posts::service::validate_config` where the env
  validation lives next to the service implementation.
- (b) **Move `MediaConfig` to `domain_media::service` and add
  `from_env` there.** *Rejected*: `MediaConfig` is a handler-layer concern
  (it owns `SupabaseStorage` and the bucket string); the `handlers::mod.rs`
  location matches its current home.

**Consequences.** `domain_media::handlers::mod.rs` grows by ~15 lines. The
factory is a pure function (no async, no I/O); a module-level test
`media_config_from_env_returns_ok_when_all_vars_set` and
`media_config_from_env_returns_missing_env_for_unset_var` cover it without
needing a testcontainer.

**Verification.** `cargo test -p domain_media --lib handlers::mod::tests`
passes.

### Decision 3: `UserApiState` mirrors `MediaApiState`

**Driver.** `domain_media::api::state::MediaApiState` holds the per-process
media state (`Arc<MediaConfig>`, two moka caches). `domain_user` only needs
the `Arc<SupabaseAdminClient>` (the seven handler structs all take
`supabase: Arc<SupabaseAdminClient>`).

**Decision.** Add `apps/api/domain_user/src/api/state.rs` with
`pub struct UserApiState { pub supabase_admin_client: Arc<SupabaseAdminClient> }`,
plus `pub fn new(client: SupabaseAdminClient) -> Self` and the `Debug`
impl that redacts the service-role key (matching the redaction in
`SupabaseAdminClient::fmt::Debug` at
`apps/api/domain_user/src/handlers/supabase_admin_client.rs:37-44`).

**Consequences.** `UserApiState` is a 1-field wrapper; the `Debug` impl
defers to `SupabaseAdminClient`'s redaction. The gateway constructs
`UserApiState::new(supabase_client)` once at startup; the resulting `Arc`
is shared across every per-request handler.

### Decision 4: Gateway startup order — env → manifest → service

**Driver.** The gateway's current `main` (`apps/api/gateway/src/main.rs:91-174`)
binds env, observability, the database connection, then `manifest()`. To
construct `MediaConfig` and `SupabaseAdminClient` we need env vars resolved
before `manifest()` is called.

**Decision.** Reorder the startup sequence:

1. `dotenv::dotenv()` (existing).
2. `init_observability()` (existing).
3. **NEW**: read `SUPABASE_URL` + `SUPABASE_SERVICE_ROLE_KEY` →
   `SupabaseAdminClient::new(...)`. Fail-fast on missing env
   (`eprintln!` + `ExitCode::FAILURE`).
4. **NEW**: `MediaConfig::from_env()` → `Arc<new(MediaConfig)>`. Fail-fast
   via the same envelope.
5. **NEW**: construct `UserApiState` from the `Arc<SupabaseAdminClient>`.
6. `manifest()` now takes the three constructed values
   (`Arc<MediaConfig>`, `Arc<SupabaseAdminClient>`) as arguments so the
   four services can be constructed with their dependencies.

Function signature change:

```rust
pub fn manifest(
    media_config: Arc<MediaConfig>,
    user_state: UserApiState,
) -> Vec<Box<dyn DomainService>>
```

Inside `manifest`:

```rust
vec![
    Box::new(DomainPostService::new()),
    Box::new(DomainAuthService::new()),
    Box::new(DomainMediaService::new(media_config)),
    Box::new(DomainUserService::from_state(user_state)),
]
```

**Alternatives considered.**
- (a) **Keep `manifest()` zero-arg; have each service read its own env.**
  *Rejected*: violates the current pattern where the gateway constructs
  dependencies once and passes them in; spreads env-reading across crates.
- (b) **Construct services in `main`, pass them in.** *Rejected*: changes
  `manifest()`'s return type from `Vec<Box<dyn DomainService>>` to
  `impl Iterator<Item = Box<dyn DomainService>>`, which forces the
  composition site to be the gateway rather than `manifest()` — this is the
  inverse of the design intent (a swappable composition root).

**Consequences.** `manifest()` gains two parameters; `main` constructs the
two dependencies once before the call. No other gateway code is affected.

### Decision 5: Test strategy

**Driver.** Per AGENTS.md §"Verify Before Commit" and the existing
testcontainers + wiremock pattern. The repository already has wiremock tests
for `SupabaseAdminClient` (`apps/api/domain_user/src/handlers/...`).

**Decision.** Three test layers:

1. **Module-level tests** (no testcontainer, no wiremock):
   - `domain_user::service::tests` — five tests:
     `health_descriptor_is_domain_user`,
     `required_env_lists_supabase_url_and_service_role_key`,
     `migrations_is_empty`,
     `validate_config_returns_missing_env_for_first_unset_var`,
     `domain_user_service_is_object_safe`.
   - `domain_user::api::routes::tests` — one test:
     `routes_returns_administrator_mount_only_and_seven_prefixes`.
   - `domain_media::handlers::mod::tests` — two tests:
     `media_config_from_env_returns_ok_when_all_vars_set`,
     `media_config_from_env_returns_missing_env_for_first_unset_var`.
   - `gateway::main::tests` — one test: `manifest_with_four_services_returns_four_entries`.
2. **Existing wiremock tests** (already present, unchanged):
   - `domain_user` GoTrue contract tests against `SupabaseAdminClient`.
   - `domain_media` Supabase Storage contract tests.
3. **No testcontainer tests added in this slice.** The migration orchestrator
   is the subject of Slice 2's testcontainer tests; this slice does not
   touch the database.

**Alternatives considered.**
- (a) **Add an Axum-level integration test against `tower::ServiceExt`.**
  *Considered but deferred*: the seven user routes are wrapped via the
  standard command-handler pattern that already has wiremock coverage at
  the handler layer; an additional router-level test would be incremental,
  not blocking. Add as follow-up if a future change regresses the
  aggregator shape.

**Verification.** All three layers run in
`cargo test --workspace`. The gateway-level tests use only module-level
factories and need no database.

## Risks / Trade-offs

**[Risk]** `domain_user` handler modules expose `*Handler` / `*HandlerTrait`
symbols, not free `handler()` functions. A misreading of the parent change's
`tasks.md:1.4` could lead to wiring `create::handler` that does not exist.
→ **Mitigation:** task 1.1 explicitly reads each handler module and
records the exact `pub` symbol in the new `routes.rs` design notes;
`cargo check` will fail immediately on a wrong path.

**[Risk]** Adding `MediaConfig::from_env` could break existing inline
callers in `domain_media` tests if the test fixtures rely on a specific
ordering of `SupabaseStorage::new` arguments. → **Mitigation:** the factory
calls the existing `SupabaseStorage::new(supabase_url, service_role_key)`
constructor verbatim; tests that already pass continue to pass.

**[Risk]** The gateway startup reordering (env → manifest → service) changes
`main` and is a behaviour-preserving surface. A typo in the env var name
would cause a startup failure that did not exist before. → **Mitigation:**
the new `main` path is exercised by every `cargo run -p gateway` smoke; the
fail-fast exit code is unchanged.

**[Risk]** The graph server snapshot is stale (built ~10 minutes ago, branch
matches but minor drift from incremental commits). → **Mitigation:** every
cited file path and symbol was revalidated against current source via
`read`/`grep`. The graph is used only as a navigation aid; no graph-derived
claim is unsupported by direct source inspection.

**[Risk]** The active `purge-legacy-cms-and-application-core` change
(41/47 tasks) is mid-flight. → **Mitigation:** the two changes touch
non-overlapping Rust files (this slice only edits `domain_user`, `domain_media`,
and `gateway`; the in-progress change is on `cms` removal and doc-only tasks).
Coordinate via the software-engineer review step before merging.

## Migration Plan

### Code
1. Add `domain_user::service` (task 1.5) — no existing code is touched.
2. Add `domain_user::api::{mod,state,routes}` (tasks 1.2-1.4) — no existing
   code is touched.
3. Add `MediaConfig::from_env` (task 2.0) — additive on
   `domain_media::handlers::mod`.
4. Update `gateway/Cargo.toml` (task 2.1) — additive.
5. Reorder `gateway::main` and extend `manifest()` (task 2.2).

### Deployment
None. This slice does not change the container image or any operator-facing
command. Slice 3 owns the deployment migration.

### Rollback
Single-commit rollback: revert the merge commit. `manifest()` reverts to
two services; media and user routes become unreachable through the gateway
(returned to the pre-slice behaviour). No data loss.

### Order of operations (suggested commit chain)
1. Add `MediaConfig::from_env` + its two module-level tests. Verify
   `cargo test -p domain_media --lib handlers::mod::tests`. **Independent.**
2. Add `domain_user::service` + its five module-level tests. Verify
   `cargo test -p domain_user --lib service`. **Independent of 1.**
3. Add `domain_user::api::{mod,state,routes}` + the routes aggregator
   tests. Verify `cargo test -p domain_user --lib api::routes`.
   **Depends on 2.**
4. Update `gateway/Cargo.toml` to add `domain_media` + `domain_user`.
   Verify `cargo check -p gateway`. **Depends on 1, 3.**
5. Reorder `gateway::main` startup; extend `manifest()`; add the
   four-services test. Verify `cargo test -p gateway --lib main`.
   **Depends on 4.**
6. Run the full verification gate (task 3).

## Open Questions

1. **Should `MediaConfig::from_env` log a startup banner listing the four
   env vars it reads?** *Current default:* no banner; the existing
   `init_observability()` handles logging. **No change recommended.**

2. **Should `DomainUserService::new` take a `SupabaseAdminClient` or
   `Arc<SupabaseAdminClient>`?** *Decision 3 selected `Arc<...>` (matching
   `MediaConfig: Arc<...>` for `DomainMediaService::new`).* **No PO action
   needed; documented in Decision 3.**

3. **Should `gateway::manifest()` parameters become a struct
   (`ManifestDeps { media_config, user_state }`) or stay as two positional
   args?** *Decision 4 selected positional for clarity with the four-service
   list.* A struct is a trivial follow-up if a third dependency appears.
   **No action.**
