# Design: Rename `domain_auth::legacy_bootstrap` and modernize the auth-layer factory

## Context

`apps/api/domain_auth/src/legacy_bootstrap.rs` (119 lines) is a
single-function module. The function
`construct_supabase_auth_layer(expected_audience: String, required_roles: Vec<String>) -> SupabaseAuthLayer`
reads `SUPABASE_URL` (falling back to `SUPABASE_INTERNAL_URL`),
reads `SUPABASE_JWT_SECRET`, and constructs a
`SupabaseAuthLayer`. It is the only call from outside the module.

The module name `legacy_bootstrap` is a historical artifact: it
preserves the name of the now-deleted `apps/api/src/bin/legacy_bootstrap.rs`
binary. The `purge-legacy-cms-and-application-core` change archive
explicitly labels the module as a "separately labelled historical
reference" (`docs/pluggable-domain-refactor.md:88`).

The function violates two `AGENTS.md` rules from "Rust Backend
Conventions":

1. **No `unwrap`/`expect` in production code** — `legacy_bootstrap.rs:29`
   and `:32` use `env::var(...).expect(...)`; the resulting panics are
   documented by the two `#[should_panic]` tests at lines 99 and 109.
2. **Async / `Result` propagation** — every fallible function in
   `domain_auth` returns `Result<T, DomainConfigError>` (see
   `service.rs:47-54` and `domain/env.rs:12-27`), but the factory
   returns a bare `SupabaseAuthLayer`.

The function also duplicates a `with_env_var` test helper that is
already duplicated a second time in `service.rs:105-120` and a third
time in `domain/env.rs:34-49`.

The call graph (per
`code-review-graph_get_affected_flows_tool` on `legacy_bootstrap.rs`,
`gateway/src/main.rs`, `domain_posts/src/main.rs`) has **two affected
flows** with criticality 0.72+ — the gateway `main` flow (depth 3,
node_count 14) and the standalone `domain_posts::main` flow (depth 3,
node_count 11). Both flows pass through
`construct_supabase_auth_layer` to apply the auth layer. The graph
returned `head_matches_build: false`, which means the in-memory graph
is one commit behind `HEAD` (`1af4fc6`) — `d7b835d` is the build SHA.
This is informational, not a correctness problem for this change
because the rename and signature change are localized to the factory
module and its four call sites; no graph edge is removed by the
rename, only relabeled.

## Goals / Non-Goals

**Goals:**

- Rename the module and function so the call-site path
  (`domain_auth::factory::auth_layer_from_env`) describes what the
  factory does, not what binary it came from.
- Change the factory signature to return
  `Result<SupabaseAuthLayer, DomainConfigError>` and propagate the
  error to the gateway and `domain_posts` bootstrap paths via `?`.
- Replace the `#[should_panic]` tests with `Result`-asserting tests.
- Consolidate the duplicated `with_env_var` test helper into
  `test_lock.rs`.
- Annotate the factory with `#[tracing::instrument]` and emit one
  `info!` event on successful construction.

**Non-Goals:**

- Renaming the historical references under
  `openspec/changes/archive/**` — those are deliberate historical
  records and stay as-is.
- Renaming `SupabaseAuthLayer`, `SupabaseAuthConfig`,
  `SupabaseClaims`, or `SupabaseToken` (the JWT-level DTOs).
- Changing the JWT validation path (`validate_supabase_token`,
  HS256/RS256 fallback, JWKS URL) — the factory only constructs the
  layer; validation is unchanged.
- Changing the role-check semantics (the `OR` semantics already
  specified in `supabase-auth/spec.md` are preserved).
- Changing the `DomainAuthService` impl or its `validate_config`
  behavior.
- Moving the factory out of `domain_auth` — it stays in the
  cross-cutting infra crate.

## Decisions

### Decision 1 — Rename module to `factory` and function to `auth_layer_from_env`

**Driver:** The module currently does *one* thing — build a layer
from the environment. The crate's other modules are named for their
purpose (`service`, `domain`, `observability`, `lib` for the type
definitions). `factory` matches that pattern.

**Alternatives considered:**

- `domain_auth::auth_layer::from_env` — rejected because the
  `SupabaseAuthLayer` *type* already lives in `domain_auth` root
  (`lib.rs:74-96`); putting a constructor under an `auth_layer`
  submodule would shadow the type. Sticking with `factory` keeps the
  type at the crate root and the constructor in a dedicated module.
- `domain_auth::bootstrap::auth_layer` — rejected because
  `bootstrap` still implies "the old bootstrap binary" and the goal
  is to retire that name entirely.
- `domain_auth::legacy_bootstrap::auth_layer_from_env` (function-only
  rename, keep module name) — rejected because the module name is the
  bigger source of confusion in `gateway/src/main.rs:200,209` and
  `domain_posts/src/main.rs:15,134,141`; keeping the old module name
  and renaming only the function would be a half-measure that keeps
  the misleading `legacy_bootstrap` segment.

**Consequence:** every import path changes by one segment; the
function's name change also forces the two `use` statements and the
four `.layer(auth_layer_from_env(...))` call sites to update. All
six sites are listed in the proposal and tasks.

### Decision 2 — Return `Result<SupabaseAuthLayer, DomainConfigError>`

**Driver:** `DomainConfigError::MissingEnv` is the canonical
configuration-failure variant in `domain_interface/src/lib.rs:107`.
`DomainAuthService::validate_config` already returns it. The factory
should align.

**Alternative considered:** return `Result<SupabaseAuthLayer, AppError>`
(the in-app error type used in other domain crates) — rejected because
auth is configuration-only and `DomainConfigError` is the typed error
the gateway composition root already knows how to format. Returning
`AppError` would force an extra `From` impl on the caller side.

**Consequence:** callers must `?`-propagate the result. The gateway
already wraps startup logic in a function whose `Err` is rendered to
`ExitCode::FAILURE` (`apps/api/gateway/src/main.rs`), so the
propagation lands naturally. The `domain_posts::main` binary has the
same pattern at lines 36-73.

**Behavior change (breaking):** a missing `SUPABASE_URL` or
`SUPABASE_JWT_SECRET` now produces a clean `eprintln!("auth config
validation failed: missing or invalid environment variable:
SUPABASE_URL")` exit, instead of a `thread 'main' panicked at
'SUPABASE_URL must be set'` abort. The Docker Swarm `migrate`
service in `deployments/docker-swarm/apps/docker-compose.yaml` and
the Traefik health-check loop will see the same `ExitCode::FAILURE`
they saw for every other configuration error, so observability
behavior is consistent.

### Decision 3 — Tracing on construction

**Driver:** `AGENTS.md` "Tracing": "Use `#[instrument]` on important
functions; `info!` for state changes". Boot-time auth-layer
construction is a state change.

**Alternative considered:** log only on failure (no `info!` on
success) — rejected because the success log is what an operator needs
to confirm the layer came up with the right audience/role set.

**Consequence:** `apps/api/domain_auth/Cargo.toml` needs `tracing` as
a direct dependency (it is currently transitive via `domain_interface`'s
re-exports). This adds one `[dependencies]` line.

### Decision 4 — Consolidate `with_env_var` into `test_lock.rs`

**Driver:** the helper is duplicated three times across the crate.
Consolidating reduces drift risk when a new env-mutating test is
added in a fourth module.

**Alternative considered:** move the helper into
`domain_interface::test_support` (the contract crate) — rejected
because `test_lock.rs` is already a `domain_auth`-local concern and
moving it would force `domain_interface` to depend on `std::sync::Mutex`
publicly.

**Consequence:** `test_lock.rs` grows by ~14 lines; `service.rs` and
`domain/env.rs` each shrink by ~14 lines and gain one `use`
statement. Net code reduction: ~14 lines.

### Decision 5 — Test rewrite

**Driver:** the two `#[should_panic]` tests
(`legacy_bootstrap.rs:99,109`) assert the wrong behavior. They must
become `Result`-asserting tests that exercise the new error path.

**Alternatives considered:** delete the two `#[should_panic]` tests
entirely — rejected because the scenarios they cover (missing
`SUPABASE_URL`, missing `SUPABASE_JWT_SECRET`) are real boot-time
failure modes that must be regression-tested after the change. The
behavior they assert is wrong, but the scenarios they cover are
right.

**Consequence:** four tests total in the renamed
`factory.rs::tests` module (2 success-path + 2 error-path), each
≤25 lines. Coverage stays identical; assertions flip from
`#[should_panic]` to `assert!(matches!(result, Err(...)))`.

## Risks / Trade-offs

- **[Risk]** A future contributor who grep-searches for
  `construct_supabase_auth_layer` will not find it after the rename.
  → **Mitigation:** the OpenSpec proposal, design, tasks, and the
  updated `domain-auth-service` spec all carry the new name; the
  commit message includes the old name in the body
  (`refactor(domain_auth): rename legacy_bootstrap to factory (was
  construct_supabase_auth_layer)`).

- **[Risk]** A consumer outside the workspace (none today, per
  `code-review-graph_traverse_graph_tool` depth 3) imports
  `domain_auth::legacy_bootstrap::construct_supabase_auth_layer`.
  → **Mitigation:** none required — `domain_auth` is `publish = false`
  in `apps/api/domain_auth/Cargo.toml:7`; no consumer outside the
  workspace can resolve the path.

- **[Risk]** The factory now requires the caller to handle a `Result`
  and any caller that already calls `.unwrap()` on it will reintroduce
  the same panic-on-misconfig behavior at the call site.
  → **Mitigation:** the call sites in
  `apps/api/gateway/src/main.rs:200,209` and
  `apps/api/domain_posts/src/main.rs:134,141` are explicitly updated
  to use `?` propagation; `cargo clippy --all-targets` will catch any
  subsequent `.unwrap()` on the call.

- **[Risk]** Renaming a module that is referenced from doc comments in
  `apps/api/domain_media/src/api/routes.rs:4`,
  `apps/api/domain_media/src/observability/mod.rs:4-5`, and
  `apps/api/domain_user/src/observability/mod.rs:4-5` leaves
  outdated references in those files.
  → **Mitigation:** task 4.3 updates those doc comments to reference
  the new path.

- **[Risk]** `docs/pluggable-domain-refactor.md:88` is labelled
  `Historical` and **must** keep the old name in the historical
  context line, but the post-line example at `:92` must update.
  → **Mitigation:** task 5.1 keeps line 88 verbatim and updates only
  line 92.

## Migration Plan

1. Land the change as a single PR with the four artifacts + the
   source edits, atomic per task group.
2. No data migration, no DB migration, no rollback plan beyond
   `git revert` — the change is localized to one module rename, one
   function rename, one signature change, and six call-site updates.
3. Rollback: `git revert <merge-commit>` restores the previous
   module/file/signature/call-site state without any data loss.

## Open Questions

- **Should the tracing payload include `supabase_url`?** The
  decision records this as "yes (treated as non-sensitive per the
  historical record)" but the JWT secret is explicitly excluded.
  Resolved at design time; no further input required.

- **Should `factory::auth_layer_from_env` be re-exported at the crate
  root (`pub use factory::auth_layer_from_env;`)?** The other public
  symbols (`SupabaseAuthLayer`, `SupabaseAuthConfig`,
  `DomainAuthService`) are all re-exported from `lib.rs` for ergonomic
  import paths. Re-exporting the factory would change the import
  path back to `domain_auth::auth_layer_from_env(...)`. **Decision:**
  do **not** re-export; the new module path
  (`domain_auth::factory::auth_layer_from_env`) is descriptive
  enough, and not re-exporting keeps the gateway/`domain_posts` call
  sites explicit about which factory they are calling. This is a
  minor point — a reviewer who disagrees can flag during code review.
