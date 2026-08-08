# Pluggable Domain Refactor — Architecture Overview

My-CMS has been refactored into a **pluggable domain library** architecture. Each domain is a self-contained Cargo crate that owns its REST adapters, application-layer command handlers, generated entities, migrations, GraphQL contribution, and migration runner.

## Workspace Members

```
apps/api/
├── Cargo.toml              # workspace root — declares all members (pure virtual workspace)
├── domain_interface/       # publishable contract crate (no domain deps)
├── domain_auth/            # cross-cutting Supabase JWT validation (lib) — see `extract-auth-into-domain-auth`
├── domain_posts/           # self-contained Blog Post Service (lib + bin); owns canonical migrations and the operator CLI
├── domain_media/           # self-contained Media Service (lib) — extracted by `split-media-and-user-domains-merge-tags-into-posts`
├── domain_user/            # self-contained User Service (lib) — extracted by `split-media-and-user-domains-merge-tags-into-posts`
├── gateway/                # thin composition root (bin: my-cms-api)
└── test_helpers/           # shared test utilities; imports `domain_posts::migrations` directly
```

> **Historical (retired):** `apps/api/application_core/`, `apps/api/migration/`, and `apps/api/src/` (legacy `cms` library + `legacy_bootstrap` binary) were removed by [`purge-legacy-cms-and-application-core`](../openspec/changes/purge-legacy-cms-and-application-core/). All retired paths retain only explicitly labeled historical references.

## Deployment Modes

Two binaries are produced:

| Binary | Backing crate | Routes served | When to use |
|---|---|---|---|
| `my-cms-api` | `gateway` | `/`, `/health`, `/healthz`, `/posts/graphql/**`, `/posts/**`, `/posts/{post_id}/translate*`, `/categories/**`, `/ai/models` | Composed gateway (single domain: post + categories + AI + translation) |
| `domain_posts` | `domain_posts` | `/posts/graphql/**`, `/posts/**`, `/posts/{post_id}/translate*`, `/categories/**`, `/ai/models`; `migrate [--list]` for operator migrations | Standalone post microservice (same HTTP surface as the composed gateway) and operator-facing migration CLI |

> **Historical (retired):** `legacy_bootstrap` (the `cms`-backed binary) was retired with the `cms` library. The staged cutover is complete: media and user business logic was extracted into `domain_media` and `domain_user`; their gateway `manifest()` registration is the only remaining follow-up.

## Staged Cutover

The cutover is **staged**:

1. **Stage 1 (DONE)** — `domain_posts` is fully extracted as the single Cargo crate that owns every post-related capability: post CRUD, post translation, post-related categories, post-related AI model registry, post-related tag helper, post GraphQL contribution, post migrations, and post-related cross-cutting layers (auth, response, error, OpenTelemetry). The gateway serves the consolidated post-domain routes via `DomainPostService`.

> **Note (`merge-graphql-into-posts-domain`):** The post domain is the **sole** owner of the GraphQL HTTP surface. The playground handlers (`playground_immutable`, `playground_mutable`) and the `Arc<Schema>` wiring live exclusively under `apps/api/domain_posts/src/api/post/graphql/`.

2. **Stage 2 (DONE)** — `domain_media` and `domain_user` extracted as self-contained crates (per `docs/adding-a-domain.md`). Each new domain's `Domain<Name>Service` is ready to be appended to `gateway::manifest()`. **Categories, AI, and translation are intentionally NOT extracted** — they are integral to the post vertical slice (per the `consolidate-category-ai-translate-into-domain-posts` change). `domain_posts` is the canonical owner of these. Tags are merged into `domain_posts::handlers::tag_helper` per `split-media-and-user-domains-merge-tags-into-posts`.

3. **Stage 3 (DONE)** — The legacy `cms::api::*` adapter tree, the `cms` library, the `legacy_bootstrap` binary, and the `application_core` and `migration` transitional crates have all been removed (see `purge-legacy-cms-and-application-core`). The gateway composition is the sole production runtime.

4. **Stage 4 (DONE)** — `application_core` and `migration` crates have been deleted. `test_helpers` imports `domain_posts::migrations` directly; the operator migration CLI is `domain_posts migrate up` (or `cargo run -p domain_posts -- migrate` locally).

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

### Cross-cutting infrastructure crate: `domain_auth`

`domain_auth` is **not** a business domain — it is the cross-cutting
infrastructure crate that owns the Supabase JWT validation layer
(`SupabaseAuthLayer`, `SupabaseAuthConfig`, `SupabaseClaims`,
`SupabaseToken`) and exposes the `domain_interface::AuthenticatedActor`
value type to every business domain. It depends only on
`domain_interface` (plus its own infrastructure dependencies —
`axum`, `tower`, `jsonwebtoken`, `serde`, `tokio`, `reqwest`,
`async-trait`) and SHALL NOT depend on any concrete business domain
(`domain-posts`, `domain_media`, `domain_user`).

> **Historical:** The `domain_auth::legacy_bootstrap` module name is a legacy reference to the deleted `apps/api/src/bin/legacy_bootstrap.rs` bootstrap binary. The module is the only remaining bearer of the `legacy_bootstrap` name; it currently exposes `construct_supabase_auth_layer(...)`, which `gateway::main` and `domain_posts::main` apply to the protected/administrator routers.

Auth is HTTP-middleware, not routes: `DomainAuthService::register_routes`
returns an empty `Vec<RouteRegistration>` and the gateway applies
`domain_auth::legacy_bootstrap::construct_supabase_auth_layer(...)` to
the protected and administrator merged routers in `compose_routers`.
Auth is also infrastructure-only: it uses the default
`DomainService::startup_health` implementation (no `SELECT 1` probe —
database readiness is delegated to `domain_posts::DomainPostService`).
Every business domain reads actor info via
`Extension<AuthenticatedActor>` (imported from `domain_interface`),
never via `Extension<SupabaseToken>`.

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
cargo test --workspace
cargo fmt -- --check
cargo clippy --workspace --all-targets --all-features -- -D warnings
cargo build --release --workspace --bins   # produces my-cms-api + domain_posts
cargo run -p domain_posts -- migrate --list   # 4 IDs in original order
cargo run -p gateway                            # composed gateway
cargo run -p domain_posts                       # standalone post microservice
```