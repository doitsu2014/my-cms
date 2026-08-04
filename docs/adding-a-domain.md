# Adding a New Domain

My-CMS is built on a **pluggable domain** architecture. Each domain is a self-contained Cargo crate that owns its REST adapters, application-layer command handlers, generated entities, migrations, GraphQL contribution, and migration runner. The canonical reference is `domain_posts`; this guide documents how to scaffold a new domain from scratch.

## Overview

```
gateway/
└── (registers Box::new(Domain<Name>Service::new(...))) in Vec<Box<dyn DomainService>>

domain_<name>/
├── Cargo.toml                # own infrastructure dependencies
└── src/
    ├── lib.rs                # pub use Domain<Name>Service, ... re-exports
    ├── main.rs               # own bin — standalone microservice
    ├── api/                  # own HTTP adapters (Axum handlers)
    ├── handlers/             # own domain core / application logic
    ├── domain/               # own infrastructure / integrations
    ├── entities/             # own SeaORM-generated entities
    ├── migrations/           # own migrations + own Migrator
    ├── service.rs            # impl DomainService trait
    ├── observability.rs      # own tracing + OTLP init
    └── migrations_cli.rs     # own migration runner
```

Each future `domain_<name>` follows the same ownership pattern: own core, own migrations, own migration runner, own bin, independently runnable/deployable as a microservice.

## Step-by-Step

### 1. Scaffold the crate

Copy `apps/api/domain_posts/` into `apps/api/domain_<name>/`, then rename:

- `domain_posts` → `domain_<name>` in all source files
- `DomainPostService` → `Domain<Name>Service`
- `DomainPost*Handler` → `Domain<Name>*Handler`
- `POST_*` constants → `<NAME>_*` constants

### 2. Update the workspace

Add `domain_<name>` to `apps/api/Cargo.toml` `[workspace] members`:

```toml
[workspace]
members = ["application_core", "domain_<name>", "domain_interface", "domain_posts", "gateway", "migration", "test_helpers"]
```

Add `domain_<name> = { path = "domain_<name>" }` to `apps/api/gateway/Cargo.toml` `[dependencies]`.

### 3. Implement `DomainService`

In `domain_<name>/src/service.rs`:

```rust
use async_trait::async_trait;
use domain_interface::{
    DomainConfigError, DomainContext, DomainService, HealthDescriptor,
    MigrationDescriptor, RouteRegistration,
};

#[derive(Debug, Clone, Default)]
pub struct Domain<Name>Service;

impl Domain<Name>Service {
    pub fn new() -> Self { Self }
}

#[async_trait]
impl DomainService for Domain<Name>Service {
    fn health(&self) -> HealthDescriptor {
        HealthDescriptor {
            name: "domain-<name>",
            version: env!("CARGO_PKG_VERSION"),
        }
    }

    fn required_env(&self) -> &'static [&'static str] {
        &["DATABASE_URL", "..."]
    }

    fn validate_config(&self) -> Result<(), DomainConfigError> {
        for var in self.required_env() {
            if std::env::var(var).is_err() {
                return Err(DomainConfigError::MissingEnv(var));
            }
        }
        Ok(())
    }

    fn migrations(&self) -> Vec<MigrationDescriptor> {
        crate::migrations::migration_descriptors()
    }

    fn register_routes(&self, ctx: &DomainContext) -> Vec<RouteRegistration> {
        crate::api::routes(ctx)
    }

    async fn startup_health(&self, ctx: &DomainContext) -> Result<(), DomainConfigError> {
        // SELECT 1 against ctx.conn
        Ok(())
    }
}
```

### 4. Wire into the gateway

In `apps/api/gateway/src/main.rs`:

```rust
pub fn manifest() -> Vec<Box<dyn DomainService>> {
    vec![
        Box::new(DomainPostService::new()),
        Box::new(Domain<Name>Service::new()),  // <-- add
    ]
}
```

If the new domain has migrations, also extend `gateway::orchestrator::run_orchestrator` to invoke the new domain's `migrations_cli::run`.

### 5. Verify

```bash
cargo check -p domain_<name>
cargo test -p domain_<name>
cargo build -p gateway
cargo test --workspace
cargo run -p domain_<name> -- migrate --list   # confirms migration identities
cargo run -p domain_<name>                       # standalone boot test
cargo run -p gateway                             # composed boot test
```

## Pattern Compliance Checklist

- [ ] `domain_<name>` has its own `Cargo.toml` declaring `domain_interface` as a dependency
- [ ] `domain_<name>` does NOT depend on `domain_posts`, `application_core`, or any sibling domain
- [ ] `lib.rs` re-exports the canonical `Domain<Name>Service` via `pub use service::Domain<Name>Service;`
- [ ] `main.rs` boots Axum, applies its own auth/CORS/cookie/body-limit/Otel layers, and binds the listener
- [ ] `migrations_cli.rs` supports `cargo run -p domain_<name> -- migrate [--list]`
- [ ] All `api_*` handlers extract `DomainContext`, call into `handlers::*`, and return `ApiResponseWith` / `ApiResponseError`
- [ ] Migration identities are preserved exactly (no `up` history change)
- [ ] `service::register_routes` returns `Vec<RouteRegistration>` with `Mount::Public/Protected/Administrator` covering every existing route
- [ ] No `unsafe` (`#![deny(unsafe_code)]` in `lib.rs`)

## When a domain is extracted

The legacy `application_core` and `migration` crates become pure re-export shims. Each non-post domain (`categories`, `tags`, `media`, `users`, `ai`) follows the same recipe.

Once `domain_<name>` is in place:

1. `application_core::commands::<name>` becomes `pub use domain_<name>::handlers::*;`
2. `application_core::entities::<name>` re-exports from `domain_<name>::entities`
3. The legacy HTTP adapter at `apps/api/src/api/<name>/*` is removed
4. `cargo run -p gateway` now serves `<name>` from the new domain crate
5. `cargo run -p domain_<name>` standalone serves only the new domain's routes

The `domain_<name>` crate is fully removable when the gateway no longer needs to fall back to the legacy `cms::api::<name>` handlers.

## Domain implementation checklist

This checklist is the copy-paste template derived from the
`extract-auth-into-domain-auth` change. Future extractions of
`domain-media`, `domain-users`, and `domain-administrator` follow it
verbatim, adapting `required_env` / `validate_config` to the new
domain's env-var surface.

1. **Add the new crate to the workspace.** Append `domain_<name>` to
   `apps/api/Cargo.toml` `[workspace] members`.
2. **Declare the contract dependency.** In
   `apps/api/domain_<name>/Cargo.toml`, depend on
   `domain_interface = { path = "../domain_interface" }` (mandatory).
   If the new domain has auth-protected routes, also depend on
   `domain_auth = { path = "../domain_auth" }`.
3. **Implement `DomainService`.** In
   `apps/api/domain_<name>/src/service.rs`, implement the contract for
   `Domain<Name>Service`:
   - `health()` returns `HealthDescriptor { name: "domain-<name>", version: env!("CARGO_PKG_VERSION") }`.
   - `required_env()` returns the env-var names the new domain needs.
   - `validate_config()` iterates `required_env()` and returns
     `Err(DomainConfigError::MissingEnv(var))` for each unset variable;
     `Ok(())` when every var is set.
   - `migrations()` returns the new domain's `MigrationDescriptor`s
     (or an empty `Vec` if the domain owns no schema).
   - `register_routes(&ctx)` returns the bare Axum routers as
     `Vec<RouteRegistration>` with the appropriate `Mount` per route group.
   - `startup_health(&ctx)` uses the default `Ok(())` impl for
     infrastructure-only domains; overrides with `SELECT 1` (or
     equivalent) for DB-backed domains.
4. **Add the object-safety test.** In
   `apps/api/domain_<name>/src/service.rs`'s `tests` module, add a
   `#[test]` that asserts
   `let _: Box<dyn DomainService> = Box::new(Domain<Name>Service::new());`
   compiles. This pins down the object-safety contract.
5. **Add the contract-compliance test suite.** Pin down the contract
   with one test per behaviour:
   - `required_env` returns the expected set of env vars.
   - `migrations` returns the expected descriptors (empty or non-empty).
   - `register_routes` returns the expected list (empty or non-empty).
   - `validate_config` returns `Ok(())` when every required var is set.
   - `validate_config` returns
     `Err(DomainConfigError::MissingEnv("<var>"))` for each missing
     variable, one sub-test per variable.
   - `startup_health` returns `Ok(())` with the default impl, or runs
     the DB probe against a `MockDatabase` for DB-backed domains.
6. **Auth-protected routes (when applicable).** Extract
   `Extension<AuthenticatedActor>` (imported from
   `domain_interface`) in every protected/administrator handler. **Never**
   import `AuthenticatedActor` from `domain_auth`, and never extract
   `Extension<SupabaseToken>` (the JWT-level DTO stays inside
   `domain_auth`).
7. **Wire the new domain into the gateway.** In
   `apps/api/gateway/src/main.rs`, append
   `Box::new(Domain<Name>Service::new())` to the `vec![ ... ]` returned
   by `manifest()`. If the new domain has migrations, also extend
   `gateway::run_orchestrator` to invoke the new domain's
   `migrations_cli::run`.
8. **Update the gateway's dependencies.** In
   `apps/api/gateway/Cargo.toml`, add
   `domain_<name> = { path = "../domain_<name>" }` and
   `domain_auth = { path = "../domain_auth" }` if not already present
   (the gateway always consumes the contract crate and may need the
   auth crate to apply the auth layer to its merged routers).
9. **Run the verification gate.**
   - `cargo check --workspace`
   - `cargo test --workspace`
   - `cargo fmt -- --check`
   - `cargo clippy --all-targets -- -D warnings`
   - `pnpm --dir apps/web build` (only if the change touches the
     frontend)
10. **OpenSpec lifecycle.**
    - `openspec verify --change "extract-<name>-into-domain-<name>"`
      (resolve every CRITICAL finding).
    - `openspec sync --change "extract-<name>-into-domain-<name>"` to
      publish the new capability spec into `openspec/specs/`.
    - `openspec archive "extract-<name>-into-domain-<name>"` after the
      sync step succeeds.

### Auth-protected routes — extractor note

For auth-protected routes, **import `AuthenticatedActor` from
`domain_interface`, never from `domain_auth`**. `domain_auth` is for
HTTP-middleware construction only (the `SupabaseAuthLayer` and the
`construct_supabase_auth_layer` factory); the value type that flows
into business-domain handlers is `domain_interface::AuthenticatedActor`,
which every business domain crate extracts as
`Extension<AuthenticatedActor>`. This keeps the auth crate's
`SupabaseToken` / `SupabaseClaims` JWT-level DTOs sealed inside
`domain_auth` and prevents business domains from re-coupling to a
Supabase-specific type when an auth-provider swap becomes necessary.