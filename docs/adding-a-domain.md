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