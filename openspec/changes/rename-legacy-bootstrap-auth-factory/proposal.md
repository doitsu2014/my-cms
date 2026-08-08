# Proposal: Rename `domain_auth::legacy_bootstrap` and modernize the auth-layer factory

## Why

The `domain_auth::legacy_bootstrap` module exists purely as a historical
name. The `legacy_bootstrap` **binary** at `apps/api/src/bin/legacy_bootstrap.rs`
was deleted by `purge-legacy-cms-and-application-core`; only the
**module name** survived. Every remaining reference to "legacy_bootstrap" in
the codebase is now either (a) a doc comment that points at the dead binary, or
(b) the auth-layer factory module itself.

The factory also violates three current project conventions:

1. **`expect()` on `env::var(...)`** in
   `apps/api/domain_auth/src/legacy_bootstrap.rs:29` and
   `:32` panics the process at runtime, while the rest of `domain_auth`
   surfaces configuration failures through `DomainConfigError::MissingEnv`
   (see `apps/api/domain_auth/src/service.rs:47-54` and
   `apps/api/domain_auth/src/domain/env.rs:12-27`). The two
   `#[should_panic(expected = "...")]` tests at
   `legacy_bootstrap.rs:99` and `:109` document this behavior as a feature.
2. **No tracing on a boot-time observable event** — construction of a
   Supabase auth layer is exactly the kind of side effect `#[instrument]`
   should annotate.
3. **Module/file name is misleading.** "legacy_bootstrap" describes the
   shape of a binary that no longer exists; it does not describe a
   *factory*. The module name leaks history into the call sites
   (`domain_auth::legacy_bootstrap::construct_supabase_auth_layer` appears in
   `apps/api/gateway/src/main.rs:200,209` and
   `apps/api/domain_posts/src/main.rs:15,134,141`).

This change retires the legacy name, modernizes the factory signature to
return `Result<SupabaseAuthLayer, DomainConfigError>`, and aligns the call
sites with the rest of `domain_auth`.

## What Changes

- **Rename module** `apps/api/domain_auth/src/legacy_bootstrap.rs` →
  `apps/api/domain_auth/src/factory.rs` (`pub mod legacy_bootstrap;` →
  `pub mod factory;` in `apps/api/domain_auth/src/lib.rs:17`).
- **Rename function** `construct_supabase_auth_layer` →
  `auth_layer_from_env`. Update the four call sites:
  - `apps/api/gateway/src/main.rs:200,209`
  - `apps/api/domain_posts/src/main.rs:15,134,141`
- **Change signature** from
  `pub fn construct_supabase_auth_layer(expected_audience: String, required_roles: Vec<String>) -> SupabaseAuthLayer`
  to
  `pub fn auth_layer_from_env(expected_audience: String, required_roles: Vec<String>) -> Result<SupabaseAuthLayer, DomainConfigError>`.
  Missing or invalid env values return `DomainConfigError::MissingEnv(<var>)`
  (matching the variants in `domain_interface::DomainConfigError` and the
  behavior of `DomainAuthService::validate_config`).
- **Update gateway and `domain_posts` call sites** to propagate the
  `Result`. Both binaries already return `ExitCode::FAILURE` on
  configuration failures (`apps/api/gateway/src/main.rs` and
  `apps/api/domain_posts/src/main.rs:36-73`); the factory now plugs into
  that existing pattern instead of bypassing it with a panic.
- **Update tests.** The two `#[should_panic]` tests
  (`legacy_bootstrap.rs:99,109`) are replaced with
  `Result`-asserting tests that exercise the new error path. The two
  success-path tests (`legacy_bootstrap.rs:65,80`) stay and assert
  `Ok(SupabaseAuthLayer)`.
- **Add `#[instrument]`** on `auth_layer_from_env` and a single
  `tracing::info!` line that records the resolved `expected_audience`
  and the `required_roles` length (never the secret values).
- **Delete the `with_env_var` test helper** at
  `legacy_bootstrap.rs:47-62`. The same helper is already duplicated in
  `apps/api/domain_auth/src/service.rs:105-120` and
  `apps/api/domain_auth/src/domain/env.rs:34-49`; consolidate it into
  `apps/api/domain_auth/src/test_lock.rs` so all three test modules reuse
  one implementation.
- **Update documentation** at `docs/pluggable-domain-refactor.md:88-92`,
  `docs/api-architecture.md` (`legacy_bootstrap` historical label),
  `.opencode/agents/software-architect.md:83`, and
  `.agents/skills/map-my-cms-api-architecture/references/api-architecture.md:15`
  to use the new module name. **Do not** update the historical archive
  docs under `openspec/changes/archive/**` — those are deliberately
  labelled "Historical" and must keep their original names.

**BREAKING** — every external caller of
`domain_auth::legacy_bootstrap::construct_supabase_auth_layer` must
update its import path and handle the new `Result` return type. There
are exactly two external callers (gateway, domain_posts); both are
updated in this change.

## Capabilities

### Modified Capabilities

- **`domain-auth-service`** — The existing requirement
  "Auth domain registers as a DomainService for composition-time
  validation" (in `openspec/specs/domain-auth-service/spec.md:107-114`)
  currently names `domain_auth::legacy_bootstrap::construct_supabase_auth_layer`.
  This change updates the requirement to the new module path
  (`domain_auth::factory::auth_layer_from_env`) and adds a new
  requirement that the factory returns `Result<SupabaseAuthLayer,
  DomainConfigError>` and emits a tracing event on construction.

## Impact

- **Source files** (4):
  - `apps/api/domain_auth/src/lib.rs:17` (module declaration rename)
  - `apps/api/domain_auth/src/legacy_bootstrap.rs` → renamed to
    `apps/api/domain_auth/src/factory.rs` (module rename + function rename +
    signature change + tracing + `with_env_var` consolidation)
  - `apps/api/domain_auth/src/service.rs` and
    `apps/api/domain_auth/src/domain/env.rs` (re-export `with_env_var`
    from `test_lock` to dedupe the helper)
  - `apps/api/domain_auth/src/test_lock.rs` (host the consolidated helper)
  - `apps/api/gateway/src/main.rs:200,209` (call-site rename + `?` propagation)
  - `apps/api/domain_posts/src/main.rs:15,134,141` (call-site rename + `?` propagation)
- **Tests**: 4 unit tests in `legacy_bootstrap.rs:64-118` are rewritten
  against the new signature; the existing 13 auth-layer tests in
  `lib.rs:340-630` stay unchanged because they construct
  `SupabaseAuthLayer::new(SupabaseAuthConfig { ... })` themselves.
- **Documentation**: 4 files need a name update (see above). Archive
  change docs under `openspec/changes/archive/**` are intentionally
  left untouched as historical record.
- **No DB / migration / entity / GraphQL / public-route impact.**
- **No new Cargo dependencies** — `tracing` is already a transitive
  dependency via `domain_interface` and direct through
  `domain_auth`'s downstream consumers; we add a direct
  `tracing = "0.1"` entry if the build rejects it.
