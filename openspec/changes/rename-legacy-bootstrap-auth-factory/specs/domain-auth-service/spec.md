# domain-auth-service Specification — Delta

This delta modifies the existing `domain-auth-service` capability to rename
the auth-layer factory module and change its return type from
`SupabaseAuthLayer` to `Result<SupabaseAuthLayer, DomainConfigError>`.

## MODIFIED Requirements

### Requirement: Auth domain registers as a DomainService for composition-time validation

The auth factory SHALL be reachable at `domain_auth::factory::auth_layer_from_env` and SHALL return `Result<SupabaseAuthLayer, DomainConfigError>`. The factory SHALL replace the previous `domain_auth::legacy_bootstrap::construct_supabase_auth_layer` path. The gateway and the standalone `domain_posts` binary SHALL `?`-propagate the result and abort startup with `ExitCode::FAILURE` (the existing pattern in both binaries) when the factory returns `Err(DomainConfigError::MissingEnv(var))`. *(Original source: `openspec/specs/domain-auth-service/spec.md` lines 107-115; the requirement name is unchanged — only the factory path, return type, and error-propagation contract are updated.)*

#### Scenario: Gateway composes the auth domain
- **WHEN** the gateway boots with `DomainPostService` and
  `DomainAuthService` in the manifest
- **THEN** the gateway validates both domains' config before binding the
  listener
- **AND** the gateway calls `startup_health` on both domains and aborts
  if either fails
- **AND** the auth layer is applied to the protected router via
  `domain_auth::factory::auth_layer_from_env`
- **AND** the public router is unchanged (no auth layer)

#### Scenario: Standalone domain_posts binary composes the auth layer
- **WHEN** the `domain_posts` standalone binary boots
- **THEN** the binary builds the merged router
- **AND** it applies the auth layer to the protected router via
  `domain_auth::factory::auth_layer_from_env`
- **AND** it applies the auth layer to the administrator router via
  `domain_auth::factory::auth_layer_from_env` with the administrator
  role set
- **AND** a failure to construct the auth layer (missing
  `SUPABASE_URL` / `SUPABASE_JWT_SECRET`) results in
  `ExitCode::FAILURE` and an `eprintln!` line that names the missing
  variable

## ADDED Requirements

### Requirement: Auth factory returns Result instead of panicking on missing env

`domain_auth::factory::auth_layer_from_env` SHALL return `Result<SupabaseAuthLayer, DomainConfigError>`. The function SHALL NOT call `.expect()` or `.unwrap()` on `std::env::var` (the no-`expect` rule from `AGENTS.md` "Rust Backend Conventions" applies). When a required env var is missing, the function SHALL return `Err(DomainConfigError::MissingEnv("<var-name>"))`. When `SUPABASE_URL` is missing but `SUPABASE_INTERNAL_URL` is set, the function SHALL use `SUPABASE_INTERNAL_URL` (the historical fallback semantics from `legacy_bootstrap.rs:30-31`).

#### Scenario: Factory succeeds when both required env vars are set
- **WHEN** `auth_layer_from_env("authenticated", vec!["writer"])` is
  called with `SUPABASE_URL` and `SUPABASE_JWT_SECRET` both set in the
  environment
- **THEN** the function returns
  `Ok(SupabaseAuthLayer)` with the resolved `supabase_url`,
  `jwt_secret`, `expected_audience = "authenticated"`, and
  `required_roles = ["writer"]`

#### Scenario: Factory returns MissingEnv when SUPABASE_URL is absent
- **WHEN** `auth_layer_from_env` is called with `SUPABASE_URL` unset
  (and `SUPABASE_INTERNAL_URL` also unset)
- **THEN** the function returns
  `Err(DomainConfigError::MissingEnv("SUPABASE_URL"))`
- **AND** the function does not panic

#### Scenario: Factory returns MissingEnv when SUPABASE_JWT_SECRET is absent
- **WHEN** `auth_layer_from_env` is called with `SUPABASE_JWT_SECRET`
  unset
- **THEN** the function returns
  `Err(DomainConfigError::MissingEnv("SUPABASE_JWT_SECRET"))`
- **AND** the function does not panic

#### Scenario: Factory uses SUPABASE_INTERNAL_URL when SUPABASE_URL is unset
- **WHEN** `auth_layer_from_env` is called with `SUPABASE_URL` unset and
  `SUPABASE_INTERNAL_URL` set to a non-empty value
- **THEN** the function returns `Ok(SupabaseAuthLayer)`
- **AND** the constructed `SupabaseAuthConfig.supabase_url` equals the
  `SUPABASE_INTERNAL_URL` value (verified through a test helper that
  inspects the layer's config — or via the layer's behavior in an
  integration test)

### Requirement: Auth factory emits a tracing event on construction

`domain_auth::factory::auth_layer_from_env` SHALL be annotated with `#[tracing::instrument(skip_all, fields(expected_audience, required_roles_count))]` and SHALL emit a `tracing::info!` event when construction succeeds. The log payload SHALL contain only `expected_audience` (the `aud` value, not the JWT secret), `required_roles_count` (the length of the role vector), and `supabase_url` (the resolved URL — needed for debugging JWKS fallback; treated as non-sensitive per the historical record). The function SHALL NOT log `SUPABASE_JWT_SECRET`, the resolved `jwt_secret`, or any JWT contents.

#### Scenario: Tracing records construction metadata
- **WHEN** `auth_layer_from_env("authenticated", vec!["writer"])` is
  called with all required env vars set
- **THEN** a tracing subscriber capturing `info!` events observes one
  event whose message includes the resolved `expected_audience` and
  `required_roles_count` fields
- **AND** the captured event does not contain the JWT secret

### Requirement: Auth factory test helper is consolidated into test_lock

The `with_env_var<F, R>(var: &str, value: Option<&str>, f: F) -> R` test helper SHALL be defined exactly once in `apps/api/domain_auth/src/test_lock.rs`. The duplicates in `apps/api/domain_auth/src/service.rs:105-120`, `apps/api/domain_auth/src/domain/env.rs:34-49`, and the current `apps/api/domain_auth/src/legacy_bootstrap.rs:47-62` SHALL be removed and replaced with a `use crate::test_lock::with_env_var;` import.

#### Scenario: A single with_env_var helper serves every test module
- **WHEN** `rg 'fn with_env_var' apps/api/domain_auth/src` is run
- **THEN** exactly one match is reported (in
  `apps/api/domain_auth/src/test_lock.rs`)
