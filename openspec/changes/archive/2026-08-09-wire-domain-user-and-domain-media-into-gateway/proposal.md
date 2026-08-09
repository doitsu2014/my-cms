## Why

The `my-cms-api` gateway's composition root at `apps/api/gateway/src/main.rs:44-49`
currently registers only two domain services — `DomainPostService` and
`DomainAuthService`. The `domain_media` crate ships a fully-implemented
`DomainMediaService` (`apps/api/domain_media/src/service.rs:60-93`) that is never
registered, and `domain_user` does not yet implement `DomainService` at all
(`apps/api/domain_user/src/lib.rs` only re-exports `AppError`; there is no
`service.rs` and no `api/` module). Media routes and administrator user routes
are therefore unreachable through the deployed gateway, in violation of the
design intent of the `domain-api-cutover` capability
(`openspec/specs/domain-api-cutover/spec.md` → "Single domain-owned API runtime").

This slice is the first of three that together realise the parent change
`openspec/changes/wire-all-domains-and-collapse-to-gateway-binary/`. It wires
the two outstanding route-owning domains (`domain_media`, `domain_user`) into
the gateway composition so that every CMS API route becomes reachable through
the gateway. It deliberately does NOT modify the migration orchestrator, the
`domain_posts` standalone binary, the Dockerfile, or any documentation — those
concerns are owned by Slice 2 and Slice 3 respectively.

## What Changes

- **ADDED** `apps/api/domain_user/src/service.rs` — `pub struct DomainUserService`
  implementing `domain_interface::DomainService`. Mirrors
  `apps/api/domain_media/src/service.rs:60-93`.
- **ADDED** `apps/api/domain_user/src/api/{mod,state,routes}.rs` — a thin
  Axum adapter aggregator that wraps the seven existing handler modules
  (`create`, `delete`, `modify`, `read_list`, `read_one`, `reset_password`) into
  a single `Vec<RouteRegistration>` on `Mount::Administrator`. Mirrors
  `apps/api/domain_media/src/api/routes.rs`.
- **ADDED** `apps/api/domain_user/src/api/state.rs` — `UserApiState { supabase_admin_client: Arc<SupabaseAdminClient> }`,
  plus a `MediaConfig::from_env` factory (in `apps/api/domain_media/src/handlers/mod.rs`)
  so the gateway can build `MediaConfig` at startup from `SUPABASE_URL`,
  `SUPABASE_SERVICE_ROLE_KEY`, `MEDIA_BUCKET`, `MEDIA_BASE_URL` without
  duplicating the existing `SupabaseStorage::new` constructor.
- **MODIFIED** `apps/api/domain_user/src/lib.rs` — adds `pub mod api;` and
  `pub use service::DomainUserService;`.
- **MODIFIED** `apps/api/gateway/Cargo.toml` — adds `domain_media = { path = "../domain_media" }`
  and `domain_user = { path = "../domain_user" }` to `[dependencies]`;
  updates the `description` to mention the four-domain composition.
- **MODIFIED** `apps/api/gateway/src/main.rs:44-49` — `manifest()` returns four
  entries (`DomainPostService`, `DomainAuthService`, `DomainMediaService`,
  `DomainUserService`). Construct `MediaConfig` and `SupabaseAdminClient` from
  env vars at the top of `main` before `manifest()` is called; fail-fast on
  missing env (`DomainConfigError::MissingEnv`).
- **MODIFIED** `apps/api/domain_media/src/handlers/mod.rs` — adds
  `MediaConfig::from_env()` factory reading `SUPABASE_URL`,
  `SUPABASE_SERVICE_ROLE_KEY`, `MEDIA_BUCKET`, `MEDIA_BASE_URL`. The factory
  returns `Result<MediaConfig, DomainConfigError>` so the gateway surfaces
  configuration failures through the standard envelope.

## Capabilities

### Modified Capabilities

- `domain-api-cutover`: The "Single domain-owned API runtime" requirement is
  extended with three new scenarios — one for `DomainUserService` registration
  and reachability, one for the four-domain manifest, and one for
  `MediaConfig::from_env` fail-fast behaviour. The "domain_user is composed
  in the gateway" ADDED requirement from the parent change lands here.

## Impact

- Affected code (all ADDED unless noted MODIFIED):
  - `apps/api/domain_user/src/service.rs` — new.
  - `apps/api/domain_user/src/api/mod.rs` — new.
  - `apps/api/domain_user/src/api/state.rs` — new.
  - `apps/api/domain_user/src/api/routes.rs` — new.
  - `apps/api/domain_user/src/lib.rs` — MODIFIED: add `pub mod api;`,
    `pub mod service;`, re-exports.
  - `apps/api/domain_media/src/handlers/mod.rs` — MODIFIED: add
    `MediaConfig::from_env`.
  - `apps/api/gateway/Cargo.toml` — MODIFIED: add `domain_media`,
    `domain_user` path-deps; update description.
  - `apps/api/gateway/src/main.rs` — MODIFIED: extend `manifest()`; construct
    `MediaConfig` + `SupabaseAdminClient` from env at top of `main`.
- Affected tests: new module-level tests in `domain_user::service`,
  `domain_user::api::routes`, and (a small one) in `gateway::main` that
  asserts `manifest().len() == 4`. Existing wiremock tests for
  `SupabaseAdminClient` continue to cover the handler layer.
- No new HTTP routes, no new env vars, no new database migrations, no new
  SeaORM entities. The four canonical migration identities are unchanged.
- No Dockerfile change. No documentation change. No CLI change. Those are
  the scope of Slice 2 (CLI) and Slice 3 (Docker + docs).
- **Non-breaking.** All existing wiremock + testcontainer tests in
  `domain_user`, `domain_media`, and `gateway` continue to pass because the
  seven user handlers are wrapped (not re-implemented) and the existing
  `DomainMediaService` already implements the trait.

## Traceability to parent change

This slice is `tasks.md` §§1-2 of the parent change
`wire-all-domains-and-collapse-to-gateway-binary`. The parent change's
`proposal.md` (lines 28-37, 64-67), `design.md` Decisions 4-5, and
`specs/domain-api-cutover/spec.md` "domain_user is composed in the gateway"
ADDED Requirement (lines 100-119) all apply verbatim to this slice.
