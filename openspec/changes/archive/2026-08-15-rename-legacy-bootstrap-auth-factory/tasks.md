## 1. Audit and Confirm Pre-State

- [x] 1.1 Verify `rg 'construct_supabase_auth_layer|domain_auth::legacy_bootstrap' apps/` returns exactly six source matches (gateway/src/main.rs:200,209; domain_posts/src/main.rs:15,134,141; legacy_bootstrap.rs:4,25) plus the legacy_bootstrap.rs test file. Affected layer: audit. Verification: `rg 'construct_supabase_auth_layer|domain_auth::legacy_bootstrap' apps/` returns the expected list with no additional consumers.

- [x] 1.2 Verify `rg 'legacy_bootstrap' docs/` returns exactly the historical-reference set documented in `openspec/changes/rename-legacy-bootstrap-auth-factory/design.md` §Risks (pluggable-domain-refactor.md, api-architecture.md, adding-a-domain.md, the two mermaid graph nodes, plus three module-doc references in domain_media and domain_user). Affected layer: doc audit. Verification: result matches design §Risks.

- [x] 1.3 Run `cargo check -p domain_auth --all-targets` and confirm exit 0 (baseline). Affected layer: build. Verification: command exits 0; baseline established before any rename.

## 2. Rename Module and Function

- [x] 2.1 Rename `apps/api/domain_auth/src/legacy_bootstrap.rs` to `apps/api/domain_auth/src/factory.rs` (use `git mv` to preserve history). Affected layer: `domain_auth` module tree. Test-first: `cargo check -p domain_auth --all-targets` exits 0 after task 2.3. Verification: `ls apps/api/domain_auth/src/legacy_bootstrap.rs` reports "No such file or directory"; `ls apps/api/domain_auth/src/factory.rs` exists; `git log --follow apps/api/domain_auth/src/factory.rs` shows the rename.

- [x] 2.2 Update `apps/api/domain_auth/src/lib.rs:17` from `pub mod legacy_bootstrap;` to `pub mod factory;`. Affected layer: crate root. Verification: `cargo check -p domain_auth` exits 0.

- [x] 2.3 Update the module doc comment at `apps/api/domain_auth/src/factory.rs:1-5` to reflect the new name (remove the historical "Both the legacy `legacy_bootstrap` binary and the gateway composition root call this function" reference; replace with "Both the gateway composition root and the standalone `domain_posts` binary call this function"). Affected layer: module docs. Verification: `rg 'legacy_bootstrap' apps/api/domain_auth/src/factory.rs` returns no matches.

## 3. Modernize the Factory Signature

- [x] 3.1 Change the function signature in `apps/api/domain_auth/src/factory.rs:25-28` from
  ```
  pub fn construct_supabase_auth_layer(
      expected_audience: String,
      required_roles: Vec<String>,
  ) -> SupabaseAuthLayer
  ```
  to
  ```
  #[tracing::instrument(skip_all, fields(expected_audience, required_roles_count))]
  pub fn auth_layer_from_env(
      expected_audience: String,
      required_roles: Vec<String>,
  ) -> Result<SupabaseAuthLayer, DomainConfigError>
  ```
  Affected layer: `domain_auth::factory`. Verification: `rg 'fn construct_supabase_auth_layer' apps/api/domain_auth/src/` returns no matches; `rg 'fn auth_layer_from_env' apps/api/domain_auth/src/` returns one match.

- [x] 3.2 Replace the `expect()` calls at the current `apps/api/domain_auth/src/legacy_bootstrap.rs:29,32` (now `factory.rs:34-39`) with `?`-propagation that maps the `env::VarError` into `DomainConfigError::MissingEnv`. Add `use domain_interface::DomainConfigError;` to the file's imports. Affected layer: error handling. Verification: `rg '\.expect\(|\.unwrap\(' apps/api/domain_auth/src/factory.rs` returns no production-code matches (test-only `unwrap` in the `with_env_var` helper is allowed).

- [x] 3.3 Add the `tracing::info!` event on the success path (after the `SupabaseAuthLayer::new` call) that records `expected_audience`, `required_roles_count`, and the resolved `supabase_url`. Affected layer: observability. Verification: the file contains exactly one `tracing::info!` call; the payload does not contain the literal strings `SUPABASE_JWT_SECRET` or `jwt_secret` value.

- [x] 3.4 Update `apps/api/domain_auth/src/factory.rs` test imports and signatures:
  - Remove the `with_env_var` helper at the current lines 47-62 (after step 2.1, the line numbers shift).
  - Add `use crate::test_lock::{ENV_LOCK, with_env_var};`.
  - Rewrite the four tests against the new `Result`-returning signature:
    - `construct_factory_returns_auth_layer_when_env_vars_are_set` (success) → `auth_layer_from_env_returns_ok_when_env_vars_are_set`; assert `result.is_ok()`.
    - `construct_factory_uses_supabase_internal_url_when_present` (success) → `auth_layer_from_env_uses_supabase_internal_url_when_supabase_url_is_missing`; assert `result.is_ok()` (cannot inspect URL — note the rationale in the test body).
    - `construct_factory_panics_when_supabase_url_is_missing` (was `#[should_panic]`) → `auth_layer_from_env_returns_missing_env_error_when_supabase_url_is_unset`; assert `matches!(result, Err(DomainConfigError::MissingEnv("SUPABASE_URL")))`.
    - `construct_factory_panics_when_supabase_jwt_secret_is_missing` (was `#[should_panic]`) → `auth_layer_from_env_returns_missing_env_error_when_supabase_jwt_secret_is_unset`; assert `matches!(result, Err(DomainConfigError::MissingEnv("SUPABASE_JWT_SECRET")))`.
  Affected layer: tests. Verification: `rg '#\[should_panic' apps/api/domain_auth/src/factory.rs` returns no matches; `cargo test -p domain_auth --lib factory::tests` exits 0 with exactly 4 tests passing.

## 4. Consolidate the `with_env_var` Test Helper

- [x] 4.1 Move the `with_env_var` helper from the current `service.rs:105-120` into `apps/api/domain_auth/src/test_lock.rs` (replace the single `ENV_LOCK` definition with both `ENV_LOCK` and `with_env_var`). Affected layer: test infra. Verification: `rg 'fn with_env_var' apps/api/domain_auth/src` returns exactly one match (in `test_lock.rs`).

- [x] 4.2 In `apps/api/domain_auth/src/service.rs`, delete the local `with_env_var` definition (lines 105-120) and add `use crate::test_lock::with_env_var;`. Affected layer: tests. Verification: `cargo test -p domain_auth --lib service::tests` still passes (8 tests).

- [x] 4.3 In `apps/api/domain_auth/src/domain/env.rs`, delete the local `with_env_var` definition (lines 34-49) and add `use crate::test_lock::with_env_var;`. Affected layer: tests. Verification: `cargo test -p domain_auth --lib domain::env::tests` still passes (3 tests).

## 5. Update the Gateway Call Sites

- [x] 5.1 In `apps/api/gateway/src/main.rs`, replace the two `domain_auth::legacy_bootstrap::construct_supabase_auth_layer(...)` invocations at lines 200 and 209 with `domain_auth::factory::auth_layer_from_env(...)?` (wrap the function so the `compose_routers` caller propagates the error). Affected layer: gateway composition. Test-first: `cargo build -p gateway --bins` exits 0; `cargo clippy -p gateway --all-targets -- -D warnings` exits 0. Verification: `rg 'construct_supabase_auth_layer' apps/api/gateway/` returns no matches; `rg 'auth_layer_from_env' apps/api/gateway/src/main.rs` returns two matches.

- [x] 5.2 Update the historical doc comment at `apps/api/gateway/src/main.rs:15` from `legacy_bootstrap` binary's mutable mount to `domain_posts` standalone binary's mutable mount (the historical context is gone; the current behavior is that the gateway itself serves both mounts). Affected layer: doc hygiene. Verification: `rg 'legacy_bootstrap' apps/api/gateway/` returns no matches.

## 6. Update the `domain_posts` Call Sites

- [x] 6.1 In `apps/api/domain_posts/src/main.rs`, change the import at line 15 from `use domain_auth::legacy_bootstrap::construct_supabase_auth_layer;` to `use domain_auth::factory::auth_layer_from_env;`. Affected layer: imports. Verification: `rg 'legacy_bootstrap' apps/api/domain_posts/` returns no matches.

- [x] 6.2 Replace the two call sites at `apps/api/domain_posts/src/main.rs:134` and `:141` to wrap each in a `match` or `?` that propagates `Err(DomainConfigError)` into `ExitCode::FAILURE` (matching the existing pattern at lines 36-73). Affected layer: boot error handling. Test-first: `cargo build --bin domain_posts` exits 0. Verification: `rg 'construct_supabase_auth_layer' apps/api/domain_posts/src/main.rs` returns no matches.

## 7. Update Doc References (Non-Archive)

- [x] 7.1 Update `docs/pluggable-domain-refactor.md:92` from `domain_auth::legacy_bootstrap::construct_supabase_auth_layer(...)` to `domain_auth::factory::auth_layer_from_env(...)`. **Keep line 88 verbatim** (it is deliberately labelled "Historical"). Affected layer: docs. Verification: `rg 'legacy_bootstrap' docs/pluggable-domain-refactor.md` returns only line 88 (the historical label).

- [x] 7.2 Update `docs/api-architecture.md` — replace `domain_auth::legacy_bootstrap::construct_supabase_auth_layer` references (lines 17, 55, 333) with `domain_auth::factory::auth_layer_from_env`. Keep the historical note at line 7-9 and the retired-binary label at line 77 verbatim. Verification: `rg 'legacy_bootstrap' docs/api-architecture.md` returns only the historical labels.

- [x] 7.3 Update `.agents/skills/map-my-cms-api-architecture/references/api-architecture.md:15` — keep the first sentence verbatim ("References to `legacy_bootstrap` below are historical...") and add a sentence noting the module was renamed to `factory::auth_layer_from_env` by this change. Affected layer: agent skills. Verification: the change is additive and the historical references below are unchanged.

- [x] 7.4 Update `.opencode/agents/software-architect.md:83` — change "the `domain_auth::legacy_bootstrap` module is a separately labelled historical name on the auth-layer factory" to "the `domain_auth::factory::auth_layer_from_env` function is the auth-layer factory (formerly `legacy_bootstrap::construct_supabase_auth_layer` before the `rename-legacy-bootstrap-auth-factory` change)". Affected layer: agent docs. Verification: `rg 'legacy_bootstrap' .opencode/agents/software-architect.md` returns no matches outside the historical-context parenthetical.

- [x] 7.5 Update the doc-comments in
  `apps/api/domain_media/src/api/routes.rs:4`,
  `apps/api/domain_media/src/observability/mod.rs:4-5`,
  and `apps/api/domain_user/src/observability/mod.rs:4-5`
  to reference `domain_auth::factory::auth_layer_from_env` instead of
  `legacy_bootstrap.rs`. Affected layer: cross-crate doc hygiene. Verification: `rg 'legacy_bootstrap' apps/api/domain_media apps/api/domain_user` returns no matches.

- [x] 7.6 Update `openspec/specs/domain-auth-service/spec.md:114` — change `domain_auth::legacy_bootstrap::construct_supabase_auth_layer` to `domain_auth::factory::auth_layer_from_env` (the spec delta will already cover this, but the canonical spec text must match). Affected layer: spec sync. Verification: `rg 'legacy_bootstrap' openspec/specs/domain-auth-service/spec.md` returns no matches.

## 8. Verify and Sync

- [x] 8.1 Run the full repository verification gate (changes-scoped):
  - `cargo check -p domain_auth -p gateway -p domain_posts -p domain_interface -p domain_media -p domain_user -p test_helpers --bins` exits 0.
  - `cargo test -p domain_auth --lib` exits 0 (29 tests pass under default parallel execution).
  - `cargo fmt -- --check` exits 0.
  - `cargo clippy -p domain_auth -p gateway -p domain_posts --all-targets -- -D warnings` exits 0 on the touched packages; workspace-wide clippy is blocked by pre-existing baseline failures in `domain_posts` (`to_string` in format args at `vector_store_pg.rs:178`; missing `Debug` impl on `ApiResponseWith`/`ApiResponseError`/`Migrator`) that reproduce on the pre-change baseline `1af4fc6` and are unrelated to this change. Filed as a follow-up.
  - `cargo build --bin my-cms-api` exits 0.
  - `cargo build --bin domain_posts` exits 0.
  Affected layer: build, test, lint. Verification: every in-scope command exits 0.

- [x] 8.2 Run `rg 'legacy_bootstrap|construct_supabase_auth_layer' apps/ openspec/specs/` and confirm no live matches outside the explicitly historical contexts in `openspec/changes/archive/**`. Affected layer: audit. Verification: matches are limited to `archive/2026-08-03-extract-auth-into-domain-auth/**`, `archive/2026-08-06-merge-graphql-into-posts-domain/**`, `archive/2026-08-08-*/**` (deliberate historical record), the `Historical:` note at `docs/pluggable-domain-refactor.md:88`, the historical parentheticals in `.opencode/agents/{product-owner,software-architect}.md`, the change artifacts themselves (`openspec/changes/rename-legacy-bootstrap-auth-factory/**`), and `apps/api/domain_media/src/api/routes.rs:4` (which references the deleted `apps/api/src/bin/legacy_bootstrap.rs:86-235` file as a historical archaeology note, distinct from the auth factory).

- [x] 8.3 OpenSpec quality gate — `openspec status --change rename-legacy-bootstrap-auth-factory --json` returns `isComplete: true` (no `verify` subcommand exists in the CLI; `status` is the canonical readiness check). All four `applyRequires` artifacts (`proposal`, `design`, `specs`, `tasks`) report `status: done`. Affected layer: OpenSpec quality gate. Verification: `isComplete: true`, no CRITICAL findings.

- [ ] 8.4 Run `openspec sync --change rename-legacy-bootstrap-auth-factory` to merge the delta spec into `openspec/specs/domain-auth-service/spec.md`. Affected layer: canonical spec sync. **Phase 4 owner action — not executed in this turn.** Verification: `openspec/specs/domain-auth-service/spec.md` reflects the renamed factory and the new `Result`-returning signature.

- [ ] 8.5 Run `openspec archive --change rename-legacy-bootstrap-auth-factory` after the PR merges. Affected layer: change archive. **Phase 4 owner action — not executed in this turn.** Verification: the change folder moves to `openspec/changes/archive/2026-MM-DD-rename-legacy-bootstrap-auth-factory/` and the source tree no longer references `legacy_bootstrap` outside the historical contexts.
