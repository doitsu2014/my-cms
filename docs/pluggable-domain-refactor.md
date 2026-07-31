# Pluggable Domain Refactor — Architecture Overview

My-CMS has been refactored into a **pluggable domain library** architecture. Each domain is a self-contained Cargo crate that owns its REST adapters, application-layer command handlers, generated entities, migrations, GraphQL contribution, and migration runner.

## Workspace Members

```
apps/api/
├── Cargo.toml              # workspace root — declares all members
├── domain_interface/       # publishable contract crate (no domain deps)
├── domain_posts/           # self-contained Blog Post Service (lib + bin)
├── gateway/                # thin composition root (bin: my-cms-api)
├── application_core/       # transitional shim — re-exports from domain_posts
├── migration/              # transitional shim — re-exports from domain_posts
├── test_helpers/          # shared test utilities
└── src/                    # transitional shim — legacy bootstrap (bin: legacy_bootstrap)
```

## Deployment Modes

Two binaries are produced:

| Binary | Backing crate | Routes served | When to use |
|---|---|---|---|
| `my-cms-api` | `gateway` | `/`, `/health`, `/healthz`, `/graphql/**`, `/posts/**`, `/posts/{post_id}/translate*` | Composed gateway (post domain only, for now) |
| `legacy_bootstrap` | `cms` | `/`, `/health`, `/healthz`, `/categories/**`, `/tags`, `/media/**`, `/users/**`, `/administrator/**`, `/graphql/**` | Staged cutover — covers non-migrated domains |
| `domain_posts` | `domain_posts` | `/posts/**`, `/posts/{post_id}/translate*` | Standalone post microservice |

## Staged Cutover

The cutover is **staged**:

1. **Stage 1 (DONE)** — `domain_posts` is fully extracted. The gateway serves post-domain routes via `DomainPostService`. The legacy bootstrap (`legacy_bootstrap` binary) continues to serve the remaining categories/tags/media/users/administrator routes.

2. **Stage 2 (next)** — Extract `domain_categories`, `domain_tags`, `domain_media`, `domain_users`, `domain_ai` as self-contained crates (per `docs/adding-a-domain.md`). Each new domain's `Domain<Name>Service` is appended to `gateway::manifest()`.

3. **Stage 3** — The `LegacyShimService` (Task 8.2 from the change plan) integrates the remaining legacy routes into the gateway composition. Once all non-post domains are extracted, the `legacy_bootstrap` binary is removed and only `my-cms-api` survives.

4. **Stage 4** — `application_core` and `migration` crates become pure re-export shims and are eventually removed.

## Why Two Binaries Today

The gateway composition uses `Vec<Box<dyn DomainService>>` where each domain registers its own routes via `DomainService::register_routes(&ctx) -> Vec<RouteRegistration>`. The `RouteRegistration.router` is `Router<DomainContext>`.

The legacy handlers in `apps/api/src/api/{category,tag,media,user,administrator}/*` use `Router<AppState>` (the legacy state type with `media_config`, `supabase_admin_client`, etc.). Bridging `Router<AppState>` to `Router<DomainContext>` requires either:

- **Option A**: A `LegacyShimService` that wraps the legacy routers with a middleware translating `DomainContext` → `AppState` and re-exposes the result as `Router<DomainContext>`. Each legacy handler would receive `AppState` via `Extension` instead of `State`. **Status: implementation requires touching ~25 handler files; documented in Task 8.2**.

- **Option B**: Make `RouteRegistration.router` generic over `Router<S>` so each domain can use its own state type. The gateway composition adapts the resulting routers to a common state. **Status: requires a breaking change to `domain_interface`.**

- **Option C**: Run two binaries behind Traefik — the gateway for new routes, the legacy bootstrap for non-migrated routes. **Status: implemented; this is the current approach.** Once all domains are extracted, the legacy bootstrap is removed.

## Per-Domain Ownership

Each domain owns its complete lifecycle:

```
domain_<name>/
├── Cargo.toml             # own infrastructure dependencies
└── src/
    ├── lib.rs             # pub use Domain<Name>Service, ...
    ├── main.rs            # own bin — standalone microservice
    ├── api/               # own HTTP adapters (Axum handlers)
    ├── handlers/          # own domain core / application logic
    ├── domain/            # own infrastructure / integrations
    │   ├── auth.rs        # Supabase auth layer
    │   ├── response.rs    # ApiResponseWith/Error envelope
    │   ├── error.rs       # AppError
    │   ├── layers.rs      # cors/cookie/body-limit/otel layers
    │   ├── postgres.rs    # DatabaseConnection factory
    │   ├── graphql.rs     # Seaography schema contribution
    │   ├── env.rs         # required env-var surface
    │   └── extensions.rs  # StringExtension, datetime_generator
    ├── entities/          # own SeaORM-generated entities (or re-export)
    ├── migrations/        # own migrations + own Migrator
    ├── service.rs         # impl DomainService trait
    ├── observability.rs   # own tracing + OTLP init
    └── migrations_cli.rs  # own migration runner
```

The canonical reference is `domain_posts`. See `docs/adding-a-domain.md` for the recipe.

## Migration Identities

`domain_posts::migrations::Migrator` owns:

- `m20240409_151952_release_100`
- `m20250330_151455_release_110`
- `m20260126_040610_release_300`
- `m20260531_000001_pgvector`

Each identity is preserved exactly. The database `up` history is unchanged.

## Verification

```bash
cargo check --workspace
cargo test --workspace          # 220 tests pass
cargo fmt --check
cargo build --bin my-cms-api   # gateway
cargo build --bin legacy_bootstrap   # legacy bootstrap
cargo run -p domain_posts -- migrate --list   # 4 IDs in original order
cargo run --bin my-cms-api      # composed gateway
cargo run --bin legacy_bootstrap  # legacy bootstrap
```