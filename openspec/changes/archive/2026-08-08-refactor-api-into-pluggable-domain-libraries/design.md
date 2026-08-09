## Context

This design supersedes the previously proposed three-shared-units shape (shared `application_core` + shared `migration` + legacy `cms` lib + a previously proposed shared `domain_foundation`). The user has redirected the architecture toward three concrete outcomes:

1. The domain interface is a **standalone, reusable, publishable contract crate** that can be injected as a Cargo dependency into **any** domain — there is no concrete domain implementation dependency and no cyclic dependency.
2. The current `application_core` and `migration` responsibilities are **not** preserved as separate shared crates (not even under new names). All responsibilities required by a domain — domain core/application logic, API adapters, entities/data access, domain-specific infrastructure/integrations, migrations, migration runner/orchestration, and tests — are **folded into the domain package itself**, beginning with `domain_posts`. Every future `domain_*` follows the same self-contained ownership model.
3. Each domain is a **deployable microservice**: it owns its runtime/service boundary, its persistence, its integrations, and its configuration. The same domain can be run **standalone** or **composed behind the gateway**, and the design must show both modes explicitly.

The refactor is **design-only** in this change: the architecture, diagrams, and decisions below describe the **target** shape. The actual extraction of code from the current `application_core` / `migration` / `cms` arrangement into per-domain crates is a later implementation step (tracked in `tasks.md`). The current source baseline described next is **not** the target architecture — it is the starting point that the implementation will move away from.

The current source baseline, verified by `rg` and direct file reads (starting point, not target):

- `apps/api/Cargo.toml` `[workspace] members = ["application_core", "migration", "test_helpers"]` plus the `cms` lib/bin.
- `apps/api/src/bin/my-cms-api.rs` (331 lines) bootstraps env, OpenTelemetry, three `AppState`s, three routers (public / protected / administrator), all middleware layers, and the listener.
- `apps/api/src/lib.rs` exposes a single `AppState` holding `conn`, `media_config`, `media_cache`, `bucket_visibility_cache`, both Seaography schemas, and `supabase_admin_client` — domain-leakage through the shared gateway type.
- `apps/api/application_core/src/commands/{post,category,tag,media,ai,user}/*` houses every handler and DTO for every domain in one tree; cross-domain calls exist (`commands::post::create::create_handler::PostCreateHandler` calls `commands::tag::create::create_handler::TagCreateHandler`; `apps/api/src/api/tag/delete/delete_handler.rs` reuses `commands::post::delete::delete_handler::PostDeleteHandler`).
- `apps/api/migration/src/lib.rs` declares a single `Migrator` with four ordered migrations: `m20240409_151952_release_100`, `m20250330_151455_release_110`, `m20260126_040610_release_300`, `m20260531_000001_pgvector` (the pgvector embeddings migration is post-translation only).
- `apps/api/application_core/src/entities/*` contains SeaORM-generated entities for `categories`, `category_tags`, `category_translations`, `posts`, `post_tags`, `post_translations`, `tags`, `translation_jobs`, and `test_fulltext` — all generated together from the single migration set.
- `apps/api/application_core/src/graphql/query_root.rs::schema` registers every entity with Seaography and returns two `async_graphql::dynamic::Schema` instances (immutable / mutable).
- `apps/api/src/common/supabase_auth.rs` carries `SupabaseAuthLayer`, `SupabaseAuthConfig`, `SupabaseClaims`, `SupabaseToken` (provider of `Extension<SupabaseToken>` for protected routes).
- `apps/api/application_core/src/common/app_error.rs` defines `AppError` (the single error type returned by every handler).
- `apps/api/application_core/src/commands/commands/ai/vector_store_pg.rs` houses the pgvector `VectorStore` used by the post translation lookup.

The refactor is **behavior-preserving**: same routes, same methods, same auth roles, same response envelopes, same SeaORM schema, same migration identities. The change is a re-slicing of where the code lives, not what it does.

`code-review-graph` MCP was not callable in this environment (only the `mcp` CLI for local server management is exposed; no `code-review-graph` namespace is available). The map and graph-evidence gate therefore fell back to `rg`, `cargo metadata` (the existing `apps/api/Cargo.toml`), and direct file reads. No graph findings are fabricated; every claim is traceable to a file/line.

## Architecture

The diagrams below encode the **target** architecture described by the user-imposed decisions and the source evidence. Every node and edge is traceable to a source path under `apps/api/`. The current implementation baseline is shown separately at the top of this section so it is not accidentally read as part of the target.

Legend for diagrams:
- **Solid arrow** = compile-time Cargo dependency (allowed import direction).
- **Dotted arrow** = runtime composition / wiring / external protocol.
- **Boxes** = Cargo crates (with `lib` and/or `bin` targets) or external services.
- **Blue** = stable contract; **green** = self-contained domain; **purple** = gateway composition; **orange** = current baseline (where called out); **pink** = external platform.

### 0. Current implementation baseline (one-time reference, **not** the target)

This diagram shows the current source arrangement as a labeled reference. It is included once so reviewers can see what the implementation will move away from. The target architecture in the next sub-section does **not** contain `application_core`, `migration`, or `cms` as crates.

````mermaid
flowchart LR
    classDef baseline fill:#fff4e6,stroke:#d97706,stroke-width:1px,color:#7c2d12
    classDef helper fill:#f5f5f5,stroke:#6c757d,stroke-width:1px,color:#495057

    subgraph baseline["Current implementation baseline (NOT target — to be removed)"]
        AC["application_core<br/>commands/{post,category,tag,media,ai,user}<br/>common::app_error<br/>entities/* (generated)<br/>graphql/query_root.rs"]
        MIG["migration<br/>single Migrator<br/>m20240409_151952_release_100<br/>m20250330_151455_release_110<br/>m20260126_040610_release_300<br/>m20260531_000001_pgvector"]
        CMS["cms  (lib + bin)<br/>apps/api/src/bin/my-cms-api.rs<br/>AppState, api/, common/,<br/>presentation_models/"]
        TH["test_helpers"]
    end

    CMS --> AC
    CMS --> MIG
    CMS --> TH
    AC --> MIG
    AC --> TH

    class AC,MIG,CMS baseline
    class TH helper
````

Notes about the baseline:

- The current `application_core` crate holds every domain's commands, entities, and GraphQL schema in one tree.
- The current `migration` crate holds a single `Migrator` with four ordered migrations.
- The current `cms` lib/bin is the bootstrap that wires `application_core` + `migration` + `presentation_models` + `common` into the Axum server.
- The implementation will move responsibilities out of these crates into per-domain crates (see the target diagram below and the Migration Plan section).

### 1. Target architecture — Package / dependency view (Cargo workspace)

This is the **primary** diagram. It encodes the three user decisions:

- `domain_interface` is a stable, publishable contract crate that **may** be a Cargo dependency of any domain and contains only interfaces/types (no domain implementation imports).
- `domain_posts` is a self-contained Cargo crate (lib + bin) that owns its complete domain core/application logic, API adapters, entities/data access, domain-specific infrastructure/integrations, migrations, migration runner/orchestration, and tests. It does **not** depend on a shared `application_core` or a shared `migration` crate — there is no such shared crate in the target.
- Every future `domain_<name>` follows the same self-contained ownership model: its own core, its own migrations, its own migration runner, its own bin, independently runnable/deployable as a microservice.
- `gateway` is a thin composition root that depends on `domain_interface` and the registered domains. It does **not** own domain core and does **not** own a central migration crate.

````mermaid
flowchart LR
    classDef contract fill:#e6f3ff,stroke:#1f6feb,stroke-width:2px,color:#0b3d91
    classDef domain fill:#e8f5e9,stroke:#2e7d32,stroke-width:2px,color:#1b5e20
    classDef gateway fill:#f3e8ff,stroke:#6f42c1,stroke-width:2px,color:#3a1d63
    classDef helper fill:#f5f5f5,stroke:#6c757d,stroke-width:1px,color:#495057
    classDef ext fill:#fde7ef,stroke:#bf2c7e,stroke-width:1px,color:#7a1148

    subgraph ws["apps/api/  Cargo workspace (target architecture)"]
        direction TB

        subgraph stable["Stable contract crate (publishable)"]
            DI["domain_interface<br/>publish = true<br/>DomainService, DomainContext, Mount,<br/>RouteRegistration, HealthDescriptor,<br/>MigrationDescriptor, DomainConfigError<br/>interfaces and types only"]
        end

        subgraph domains["Self-contained domain crates (repeatable boundary)"]
            DP["domain_posts<br/>lib + bin<br/>api/  handlers/  domain/<br/>entities/  migrations/  tests/<br/>auth.rs  cors.rs  tracing.rs<br/>storage.rs  ai.rs  graphql.rs<br/>response.rs  error.rs<br/>migrations_cli.rs (own runner)<br/>SeaORM Migrator (own)"]
            DX["domain_<name><br/>same ownership pattern as domain_posts<br/>own core, own migrations,<br/>own runner, own bin<br/>independently runnable/deployable"]
        end

        subgraph composition["Thin composition crate"]
            GW["gateway  (bin: my-cms-api)<br/>main.rs owns the Vec&lt;Box&lt;dyn DomainService&gt;&gt;<br/>registration manifest<br/>no domain core, no central migration crate"]
        end

        subgraph helpers["Test helpers (no production coupling)"]
            TH["test_helpers<br/>testcontainers + Postgres + pgvector<br/>+ MigratorTrait ref"]
        end
    end

    subgraph ext["External / Platform"]
        DB[("PostgreSQL + pgvector<br/>(Supabase)")]
        STORE[("Supabase Storage<br/>(S3-compatible)")]
        AUTH[("Supabase GoTrue / JWT")]
        OAI["OpenAI API<br/>(translation + embeddings)"]
        OTEL["OpenTelemetry / Jaeger"]
    end

    GW  --> DI
    GW  --> DP
    GW  --> DX
    GW  --> TH
    DP  --> DI
    DX  --> DI
    DP  --> TH
    DX  --> TH

    DP  -- "SeaORM" --> DB
    DP  -- "SupabaseStorage" --> STORE
    DP  -- "SupabaseAuthLayer" --> AUTH
    DP  -- "OpenAI / pgvector" --> OAI
    DP  -- "Otel" --> OTEL
    DX  -- "domain-specific integrations" --> ext
    GW  -- "Otel / logs" --> OTEL

    class DI contract
    class DP,DX domain
    class GW gateway
    class TH helper
    class DB,STORE,AUTH,OAI,OTEL ext
````

Rules enforced by this diagram:

- `domain_interface` depends on **no** domain crate. It depends only on foundational Rust libs (`axum`, `sea-orm`, `async-graphql`, `async-trait`, `serde`, `tokio`, etc.). It is `publish = true` and versioned independently.
- `domain_posts` depends on `domain_interface` (to implement `DomainService`) and on its own infrastructure (SeaORM, Axum, OpenAI, jsonwebtoken, tower-http, etc.). It does **not** depend on any other domain crate, and there is no shared `application_core` or shared `migration` crate in the target for it to depend on.
- `domain_<name>` follows the same repeatable boundary: own core, own migrations, own migration runner, own bin, own integrations. No domain depends on another domain's implementation.
- `gateway` depends on `domain_interface` and on each domain it composes. It does **not** depend on any domain's commands, entities, DTOs, or storage clients — only on `domain_interface` and on the `DomainService` trait objects. It does **not** own domain core and does **not** own a central migration crate.
- The only shared crate in the target is `domain_interface`. `test_helpers` is a test-only workspace member and is not part of the production dependency graph.

### 2. Target architecture — Microservice / deployment view (standalone vs gateway-composed)

This diagram encodes the third user decision: each domain is a **deployable microservice**. The same `domain_posts` Cargo crate is used in both modes — the only difference is which binary owns the listener.

````mermaid
flowchart TB
    classDef standalone fill:#e8f5e9,stroke:#2e7d32,stroke-width:2px,color:#1b5e20
    classDef composed fill:#f3e8ff,stroke:#6f42c1,stroke-width:2px,color:#3a1d63
    classDef contract fill:#e6f3ff,stroke:#1f6feb,stroke-width:2px,color:#0b3d91
    classDef ext fill:#fde7ef,stroke:#bf2c7e,stroke-width:1px,color:#7a1148
    classDef deploy fill:#fff4e6,stroke:#d97706,stroke-width:1px,color:#7c2d12

    subgraph modeA["Deployment mode A — Standalone domain_posts microservice"]
        direction TB
        LB2["Traefik / Listener<br/>(PORT=8989)"]
        BIN2["domain_posts bin<br/>(cargo run -p domain_posts)<br/>boots Axum, owns its layers,<br/>owns its context, owns its migrations<br/>owns its migration runner"]
        DB2[("PostgreSQL + pgvector<br/>schema owned by domain_posts")]
        STORE2[("Supabase Storage")]
        AUTH2[("Supabase GoTrue")]
        OAI2["OpenAI API"]
    end

    subgraph modeB["Deployment mode B — Composed behind the gateway"]
        direction TB
        LB1["Traefik / Listener<br/>(PORT=8989)"]
        GBP["gateway bin  (my-cms-api)<br/>owns Vec&lt;Box&lt;dyn DomainService&gt;&gt;<br/>no domain core, no central migration crate"]
        REG["domain_posts::DomainPostService<br/>(registered into the gateway's<br/>Vec&lt;Box&lt;dyn DomainService&gt;&gt;)"]
        DB1[("PostgreSQL + pgvector<br/>schema shared by all composed domains")]
        STORE1[("Supabase Storage")]
        AUTH1[("Supabase GoTrue")]
        OAI1["OpenAI API"]
    end

    subgraph shared["Reusable contract crate"]
        DI["domain_interface<br/>publish = true<br/>Cargo-injectable into any domain"]
    end

    BIN2  --> DI
    BIN2  --> DB2
    BIN2  --> STORE2
    BIN2  --> AUTH2
    BIN2  --> OAI2
    LB2   --> BIN2

    GBP   --> DI
    GBP   --> REG
    REG   --> DI
    REG   --> DB1
    REG   --> STORE1
    REG   --> AUTH1
    REG   --> OAI1
    LB1   --> GBP

    class DI contract
    class BIN2,DB2,STORE2,AUTH2,OAI2 standalone
    class GBP,REG,DB1,STORE1,AUTH1,OAI1 composed
    class LB1,LB2 deploy
````

The two modes share:

- The same `domain_posts` Cargo crate (same `src/`, same `Cargo.toml`).
- The same `domain_interface` contract crate.
- The same external integrations (PostgreSQL, Supabase Storage, Supabase GoTrue, OpenAI, OpenTelemetry).
- The same env-var surface as the current `my-cms-api` bootstrap reads in `construct_app_state()` and `construct_supabase_auth_layer()`: `SUPABASE_URL`, `SUPABASE_INTERNAL_URL`, `SUPABASE_ANON_KEY`, `SUPABASE_SERVICE_ROLE_KEY`, `SUPABASE_JWT_SECRET`, `AUTHORIZATION_AUDIENCE`, `MAX_BODY_LENGTH`, `HOST`, `PORT`, `MEDIA_BASE_URL`, `OPENAI_API_KEY`, `DATABASE_URL`, `ENABLED_OTLP_EXPORTER`.

The two modes differ only in **which binary owns the listener**:

- **Standalone** — `domain_posts` boots its own Axum server, applies its own auth/CORS/cookie/body-limit/Otel layers, opens its own database connection, runs its own migrations via `cargo run -p domain_posts -- migrate`, and exposes `/health`, `/healthz`, `/posts/**`, `/posts/{post_id}/translate`, `/posts/{post_id}/translate/background`, `/posts/{post_id}/translate/jobs/**` from `domain_posts::api`.
- **Composed** — the `gateway` crate imports `domain_posts` as a Cargo dependency, constructs `Box::new(DomainPostService::new(...))` and registers it in a `Vec<Box<dyn DomainService>>`. The gateway's `main.rs` iterates the manifest, calls `register_routes(&ctx)` on each, and merges the resulting `RouteRegistration`s into the public/protected/administrator Axum routers.

The gateway also imports every other `domain_<name>` it composes (categories, tags, media, users, etc.) the same way — each registered as a `Box<dyn DomainService>`. The gateway does not import any domain's commands, entities, DTOs, or storage clients; it only imports `domain_interface` and the `DomainService` trait objects.

### 3. Representative request flow (POST /posts)

The standalone and composed paths share the same `domain_posts` handlers; only the entry point differs. The flow below shows the **composed** path (gateway dispatch); the standalone path is identical from `DomainPostService::api_create_post` onward.

````mermaid
sequenceDiagram
    autonumber
    participant Client
    participant LB as Traefik / Listener
    participant GW as gateway main.rs
    participant Otel as Otel layers
    participant Auth as SupabaseAuthLayer
    participant Router as Axum Router
    participant DP as domain_posts::DomainPostService
    participant H as PostCreateHandler
    participant Ctx as DomainContext
    participant DB as PostgreSQL

    Client->>LB: POST /posts (Bearer JWT)
    LB->>GW: forward request
    GW->>Otel: open span
    GW->>Router: dispatch by Mount::Protected
    Router->>Auth: validate JWT (Supabase)
    Auth-->>Router: AuthUser (role: writer or admin)
    Router->>DP: api_create_post
    DP->>H: PostCreateHandler::handle(req)
    H->>Ctx: borrow Arc<DatabaseConnection>
    H->>DB: INSERT post row
    DB-->>H: row
    H-->>DP: Post entity
    DP-->>Router: ApiResponseWith<Post>
    Router-->>GW: 200 OK envelope
    GW-->>LB: 200 OK
    LB-->>Client: response body
````

Source alignment for each step (pointing at the current baseline; the implementation will move the code into `domain_posts`):

- `apps/api/src/bin/my-cms-api.rs` lines 51–54 build the merged router (`public_router().merge(protected_router()).merge(protected_administrator_router())`).
- `apps/api/src/api/post/create/create_handler.rs` lines 12–31 implement `api_create_post` (today calls `PostCreateHandler`; in the target it lives in `domain_posts::api::post::create`).
- `apps/api/application_core/src/commands/post/create/create_handler.rs` lines 30–128 implement `PostCreateHandler::handle_create_post` (today; in the target it lives in `domain_posts::handlers::post::create`).
- `apps/api/src/common/supabase_auth.rs` lines 61–80 implement the `SupabaseAuthLayer` today; in the target it lives in `domain_posts::domain::auth`.
- `apps/api/src/presentation_models/api_response.rs` lines 5–48 define `AxumResponse`, `ApiResponseWith`, and the JSON envelope today; in the target it lives in `domain_posts::domain::response`.

## Goals / Non-Goals

**Goals:**

- Stand up `domain_interface` as a `publish = true` contract crate whose `DomainService` trait is dyn-compatible and stable. It exports only `DomainService`, `DomainContext`, `Mount`, `RouteRegistration`, `HealthDescriptor`, `MigrationDescriptor`, `DomainConfigError`, and the supporting trait/types. No concrete domain implementation is referenced.
- Make `domain_posts` a **self-contained Cargo crate** (lib + bin) that owns every responsibility a domain needs to operate: domain core/application logic, API adapters, entities/data access, domain-specific infrastructure/integrations, migrations, migration runner/orchestration, and tests. There is no shared `application_core` crate and no shared central `migration` crate in the target.
- Make `domain_posts` a **deployable microservice**: its `bin` target can boot a standalone Axum server with its own auth, CORS, cookies, body limits, OpenTelemetry, database connection, and migrations. The same `domain_posts` crate is also registered as a `Box<dyn DomainService>` inside the gateway.
- Make every future `domain_<name>` follow the **same self-contained ownership model**: own core, own migrations, own migration runner, own bin, independently runnable/deployable as a microservice.
- Keep the gateway as a **thin composition root** that depends on `domain_interface` and the registered domains. The gateway does **not** own domain core and does **not** own a central migration crate.
- Run an `OpenSpec` change that is **behavior-preserving** for the post and translation HTTP and GraphQL contracts (`/posts/**`, `/posts/{post_id}/translate`, `/posts/{post_id}/translate/background`, `/posts/{post_id}/translate/jobs/**`) and the migration identity ordering.
- Keep the existing verification gate (`cargo check`, `cargo test`, `cargo fmt -- --check`, `cargo clippy --all-targets -- -D warnings`, `pnpm --dir apps/web build`) green at every step.

**Non-Goals:**

- Replacing SeaORM, Axum, the Seaography GraphQL pipeline, Supabase auth, or Supabase storage.
- Splitting the database or introducing cross-service transactions. All composed domains continue to share the same PostgreSQL instance and schema.
- Changing the public REST / GraphQL contract, response envelopes, or migration identity ordering.
- Replacing the OpenAI integration or moving the translation pipeline to a separate runtime; the OpenAI / pgvector code stays inside `domain_posts`.
- Creating a `domain_foundation`, `domain_shared`, or shared `application_core` crate that other domains depend on. Shared kernel is explicitly rejected.
- Implementing the refactor in this design change. The design only; tasks are tracked in `tasks.md` and the actual code movement from the current baseline to the target is a later implementation step.

## Decisions

### Decision 1 — `domain_interface` is a publishable, dependency-free contract crate

`domain_interface` is a Cargo library with `publish = true`. It depends only on foundational Rust libraries (`axum`, `sea-orm`, `async-graphql`, `async-trait`, `serde`, `tokio`, `futures`, `chrono`, `uuid`, `async-std`). It does **not** depend on any concrete domain crate and does **not** depend on any shared-domain kernel crate. It is the only shared crate in the target architecture; every domain and the gateway depend on it, and no domain depends on another domain's implementation.

Rejected alternatives:

- **Re-using a shared `application_core` / domain kernel as the contract crate.** Rejected because it is not publishable and leaks implementation types into every consumer.
- **A dynamic Rust plugin model (`inventory`, `abi_stable`, `dylib`).** Rejected because the operational cost (separate compilation units, static linking, ABI tracking) outweighs the benefit for a single binary, and it would require redesigning tracing and error mapping.
- **A single global `Migrator` owned by `domain_interface`.** Rejected because migrations are owned by each domain, not by the contract crate.

Sketch (design only — not code to ship in this change):

```rust
// apps/api/domain_interface/src/lib.rs
use std::sync::Arc;
use async_trait::async_trait;
use axum::Router;
use sea_orm::DatabaseConnection;
use async_graphql::dynamic::Schema;

#[derive(Clone, Debug)]
pub struct DomainContext {
    pub conn: Arc<DatabaseConnection>,
    pub graphql_immutable: Arc<Schema>,
    pub graphql_mutable: Arc<Schema>,
}

#[derive(Copy, Clone, Debug, Eq, PartialEq)]
pub enum Mount { Public, Protected, Administrator }

pub struct RouteRegistration {
    pub mount: Mount,
    pub router: Router<DomainContext>,
    pub required_roles: Vec<String>,
}

pub struct MigrationDescriptor {
    pub id: &'static str,
    pub depends_on: &'static [&'static str],
}

pub struct HealthDescriptor {
    pub name: &'static str,
    pub version: &'static str,
}

#[async_trait]
pub trait DomainService: Send + Sync {
    fn health(&self) -> HealthDescriptor;
    fn required_env(&self) -> &'static [&'static str];
    fn validate_config(&self) -> Result<(), DomainConfigError>;
    fn migrations(&self) -> Vec<MigrationDescriptor>;
    fn register_routes(&self, ctx: &DomainContext) -> Vec<RouteRegistration>;
    async fn startup_health(&self, ctx: &DomainContext) -> Result<(), DomainConfigError>;
}
```

Notes:

- `DomainConfigError` is a small, stable enum defined in `domain_interface` so the gateway can return a uniform failure without depending on any domain's error type.
- `Mount` and `required_roles` are explicit so the gateway can build the three existing router groups without leaking domain knowledge.
- `MigrationDescriptor::depends_on` lets the gateway (or `domain_posts::migrations_cli`) deterministically order descriptors across domains.
- `HealthDescriptor` exposes the gateway's `/health` aggregator without coupling the gateway to a domain's `Debug` impl.

### Decision 2 — `domain_posts` is a self-contained Cargo crate (lib + bin) that owns its core, migrations, and migration runner

`domain_posts` is a Cargo crate with both `lib` and `bin` targets. It owns every responsibility a domain needs to operate and serves as the **repeatable boundary** for every future `domain_<name>`. The package layout is:

```
apps/api/domain_posts/
├── Cargo.toml                 # deps: domain_interface, axum, sea-orm, sea-orm-migration,
│                              #      seaography, async-graphql, async-graphql-axum,
│                              #      jsonwebtoken, tower-http, tower-cookies, axum-tracing-opentelemetry,
│                              #      init-tracing-opentelemetry, async-openai, hyper, moka,
│                              #      sea-query, tracing, anyhow, uuid, chrono, serde, html5ever,
│                              #      markup5ever_rcdom, slugify, dotenv, reqwest, async-std, tokio
├── src/
│   ├── lib.rs                 # pub use DomainPostService, ApiResponseWith, AppError, SupabaseAuthLayer, ...
│   ├── main.rs                # standalone bin: env, otel, ctx, axum, health, listener
│   ├── api/                   # HTTP adapters (Axum handlers)
│   │   ├── post/
│   │   │   ├── create/        # post HTTP create adapter
│   │   │   ├── read/          # post HTTP read adapter
│   │   │   ├── modify/        # post HTTP modify adapter
│   │   │   ├── delete/        # post HTTP delete adapter
│   │   │   └── translate/     # post HTTP translate adapter
│   │   ├── public/            # /health, /healthz, /  (root + readiness)
│   │   └── administrator/     # /administrator/database/migration (post domain's migration CLI)
│   ├── handlers/              # application-layer command handlers (domain core)
│   │   ├── post/
│   │   │   ├── create/        # PostCreateHandler
│   │   │   ├── read/          # PostReadHandler
│   │   │   ├── modify/        # PostModifyHandler
│   │   │   ├── delete/        # PostDeleteHandler
│   │   │   └── translate/     # PostTranslateHandler
│   │   ├── translation_jobs/  # translation job lifecycle
│   │   ├── tag_helper/        # local tag-creation helper used inside PostCreateHandler
│   │   │                      #     (not a cross-domain import; lives inside domain_posts)
│   │   └── vector_store/      # pgvector VectorStore used by post translation
│   ├── domain/                # domain infrastructure / foundation code
│   │   ├── error.rs           # AppError
│   │   ├── response.rs        # ApiResponseWith/Error/ErrorCode
│   │   ├── auth.rs            # SupabaseAuthLayer + SupabaseConfig + SupabaseToken
│   │   ├── layers.rs          # cors_layer(), cookie_layer(), body_limit_layer(), otel_layers()
│   │   ├── storage.rs         # SupabaseStorage + MediaConfig + caches
│   │   ├── ai.rs              # OpenAI config + client factory (used by translate)
│   │   ├── postgres.rs        # connect_database() (env-driven)
│   │   ├── graphql.rs         # Seaography schema builder (post-aggregate contribution)
│   │   ├── env.rs             # required env-var surface and validation
│   │   └── extensions.rs      # StringExtension, datetime_generator, etc.
│   ├── entities/              # SeaORM-generated entities (generated here, owned by domain_posts)
│   │   ├── mod.rs
│   │   ├── posts.rs
│   │   ├── post_tags.rs
│   │   ├── post_translations.rs
│   │   ├── translation_jobs.rs
│   │   ├── categories.rs
│   │   ├── category_tags.rs
│   │   ├── category_translations.rs
│   │   ├── tags.rs
│   │   └── sea_orm_active_enums.rs
│   ├── migrations/            # SeaORM migrations owned by domain_posts
│   │   ├── mod.rs             # pub struct Migrator; impl MigratorTrait { fn migrations() -> Vec<...> }
│   │   ├── m20240409_151952_release_100.rs
│   │   ├── m20250330_151455_release_110.rs
│   │   ├── m20260126_040610_release_300.rs
│   │   └── m20260531_000001_pgvector.rs
│   ├── service.rs             # impl DomainService for DomainPostService
│   ├── observability.rs       # tracing + OTLP init
│   └── migrations_cli.rs      # pub fn run_cli() invoked by `cargo run -p domain_posts -- migrate`
└── tests/
    ├── routes_post.rs          # HTTP integration tests for /posts/**
    ├── routes_translate.rs     # HTTP integration tests for translate endpoints
    ├── jobs.rs                 # translation job lifecycle
    └── health.rs               # /health + startup_health
```

This is the architectural realization of the user's three decisions. Specifically:

- Post-generated entities (`posts`, `post_tags`, `post_translations`, `translation_jobs`) live in `domain_posts::entities`.
- Post-relevant cross-domain entities (`categories`, `category_tags`, `category_translations`, `tags`) live in `domain_posts::entities` because the post aggregate references them via foreign keys and the historical `m20240409_151952_release_100` migration creates all of them. Future domains (categories, tags, media, users) can be extracted as their own crates that **depend on `domain_posts::entities`** for the entities they share, or they can be re-generated against their own per-domain migrations in a follow-up change.
- All four migrations stay with `domain_posts` for identity preservation; `domain_posts::migrations::Migrator` keeps the same identities (`m20240409_151952_release_100`, `m20250330_151455_release_110`, `m20260126_040610_release_300`, `m20260531_000001_pgvector`). Future domains add their own migrations and declare `MigrationDescriptor::depends_on` on the post-domain migrations they need.
- The migration runner/orchestration is owned by `domain_posts` in `src/migrations_cli.rs` (and a sibling helper in `gateway` for composed mode). `domain_posts` is the **only** owner of the post migration set; there is no central migration crate in the target.
- The cross-domain call inside `PostCreateHandler` (which currently reaches into `commands::tag::create::create_handler::TagCreateHandler`) becomes a **local helper** in `domain_posts::handlers::tag_helper::create_tags_in_transaction`. The helper is a code move, not a runtime invocation, so the `DatabaseTransaction` semantics are preserved. The helper is exposed only inside `domain_posts`; it is not a public API of any other crate.
- `AppError`, `ApiResponseWith`, `ApiResponseError`, `ErrorCode`, `AxumResponse`, `SupabaseAuthLayer`, `SupabaseAuthConfig`, `SupabaseClaims`, `SupabaseToken`, `MediaConfig`, `CORS`/`cookie`/`body-limit`/`Otel` layers, OpenTelemetry setup, env loading, and the GraphQL schema builder **all live inside `domain_posts`**. They are not shared with any other domain.

Rejected alternatives:

- **A shared `domain_foundation` crate.** Rejected. Sharing is not a substitute for ownership; `domain_posts` is more honest when it owns its own auth, error, response, storage, and observability.
- **Move entities into a shared `entities` crate.** Rejected. Entities are generated from the domain's migrations and live with the domain that owns the migrations.
- **Continue to live behind a shared `application_core` with re-exports.** Rejected. That keeps the shared kernel without naming it, which is the disguised shape the user forbids.
- **Macro-generated crate.** Rejected. Explicit, readable code is faster to debug and matches the existing convention.

### Decision 3 — `domain_posts` is a deployable microservice (lib + bin)

`domain_posts` has a `bin` target that boots the same Axum server the gateway otherwise would, but with the domain's own routes, layers, and context. The `bin` is a thin wrapper around the same code the gateway uses:

- `src/main.rs` (or `src/bin/domain_posts_bin.rs`) reads the same env vars as `my-cms-api`'s `construct_app_state()` and `construct_supabase_auth_layer()` (Supabase URL/keys, JWT secret, audience, role list, OpenAI key, database URL, host/port, body limit, OTLP flag).
- It initializes tracing and OpenTelemetry using the same `init_tracing_opentelemetry` code currently in `apps/api/src/bin/my-cms-api.rs` lines 70–89.
- It opens a single `DatabaseConnection` via `domain_posts::domain::postgres::connect_database()`.
- It builds `DomainPostService` via `DomainPostService::new(...)` and calls `register_routes(&ctx)` to obtain the Axum routers.
- It applies the auth / CORS / cookie / body-limit / Otel layers (via `domain_posts::domain::layers::*`).
- It runs its own migrations via `domain_posts::migrations_cli::run()` (standalone mode) or delegates to the gateway's orchestrator (composed mode).
- It binds the listener (`HOST:PORT`) and serves the merged router.

The same `DomainPostService` is also registered into the gateway's `Vec<Box<dyn DomainService>>` when the gateway chooses to compose `domain_posts`. The two modes are wired in the same code path; the only difference is which binary owns the listener.

Rejected alternatives:

- **Library-only domain, no `bin`.** Rejected — a domain must be deployable on its own.
- **Domain has a bin but no lib.** Rejected — the gateway needs to import `DomainPostService` from the crate, which requires a `lib` target.
- **Two separate crates for the same domain (one for lib, one for bin).** Rejected — duplicates code and ownership.

### Decision 4 — The gateway is a thin composition crate (no domain core, no central migration crate)

`gateway` is a Cargo crate whose only responsibilities are:

- Load env / config.
- Initialize OpenTelemetry / tracing.
- Optionally open a database connection (shared between composed domains).
- Build a `DomainContext` (one `Arc<DatabaseConnection>`, two `Arc<Schema>` for graphql).
- Construct a `DomainPostService` and register it in a `Vec<Box<dyn DomainService>>` (alongside every other `domain_<name>` it composes).
- Iterate the manifest, call `register_routes(&ctx)` on each, and merge the resulting `RouteRegistration`s into the public / protected / administrator Axum routers.
- Apply the auth / CORS / cookie / body-limit / Otel layers at the gateway level (the same layers `domain_posts` uses when standalone, so behavior is identical).
- Bind the listener (`HOST:PORT`) and serve.

The gateway does **not** import domain commands, entities, DTOs, storage clients, or business rules. It only imports `domain_interface` and the `DomainService` trait objects provided by each domain crate. The gateway does **not** own domain core and does **not** own a central migration crate.

The gateway may also import additional per-domain crates (e.g., a future `domain_categories`) the same way it imports `domain_posts`: as a Cargo dependency that contributes a `DomainService`. The gateway treats every domain identically.

Rejected alternatives:

- **A builder DSL.** Rejected — the gateway is small enough that a function is enough.
- **Feature flags in the gateway crate.** Rejected — Cargo features hide composition; an explicit `Vec<Box<dyn DomainService>>` is auditable.

### Decision 5 — Migration ownership is per-domain; the orchestrator is a small helper inside `domain_posts` and inside `gateway`

Each domain owns its migration set and its migration runner. `domain_posts` exposes its migrations as `MigrationDescriptor` instances via `DomainService::migrations()` and exposes its runner as `domain_posts::migrations_cli::run()` (used by the `bin` for standalone mode). The gateway exposes a sibling helper `gateway::orchestrator::run()` (used by the gateway for composed mode). The orchestrator:

- Collects descriptors from every registered `DomainService`.
- Topologically sorts by `MigrationDescriptor::id` and `MigrationDescriptor::depends_on`.
- Deduplicates by `id`.
- Runs them sequentially against the shared `DatabaseConnection`.
- Maps errors to `DomainConfigError` (so the gateway can fail startup) or to a 5xx HTTP response (via the gateway's `/administrator/database/migration` Administrator route).

The current migration identities are preserved exactly: `m20240409_151952_release_100`, `m20250330_151455_release_110`, `m20260126_040610_release_300`, `m20260531_000001_pgvector`. The migration `up` history in the database is unchanged.

There is no central migration crate in the target. Each domain owns its migrations and its runner; the gateway only orchestrates the runs.

Rejected alternatives:

- **A single global `Migrator` (current baseline).** Rejected because the entire point of the refactor is that each domain owns its migrations.
- **A separate database per domain.** Out of scope (see Non-Goals).
- **A migration orchestrator in `domain_interface`.** Rejected because the orchestrator is not a contract — it is a helper that depends on `sea-orm-migration`. The contract crate stays clean.

### Decision 6 — SeaORM entity generation is per-domain

`sea-orm generate entity` is run per-domain against the domain's `migrations` set. The output for `domain_posts` is `apps/api/domain_posts/src/entities/`. Each domain owns its generated entities. Seaography continues to receive the full entity set for the gateway's schemas, but the gateway obtains the entities by calling each domain's contribution helper — not by importing a domain's `entities::*` directly.

If a future domain needs to share an entity (e.g., `tags` is used by `domain_posts` and a future `domain_tags`), the dependency direction is **consumer → producer**, not bidirectional. A future `domain_tags` can depend on `domain_posts::entities` for read-only access, or both domains can be re-generated against their own migrations in a follow-up change.

Rejected alternatives:

- **Per-domain generated entity trees immediately.** Rejected for this change because the historical migration set produces a single entity set; splitting the entity generation requires splitting the migration set, which is a larger refactor.
- **A shared `entities` crate.** Rejected — entities are domain-owned.

### Decision 7 — GraphQL contribution is per-domain

`Seaography` already takes a `DatabaseConnection` and returns a `Schema`. To preserve the existing mounts (`/graphql/immutable`, `/graphql/mutable`) without giving the gateway access to entity internals, each domain that contributes to GraphQL exposes a `pub fn contribute_schema(...)` helper. `domain_posts::domain::graphql::contribute_post_schema(...)` is the post-aggregate contribution. The gateway builds both schemas by calling each domain's contribution. The gateway owns the two `Arc<Schema>` values in `DomainContext`; the domains contribute the entity registrations.

Rejected alternatives:

- **Building schemas entirely inside `domain_posts`.** Rejected because the gateway still needs to expose both endpoints and the same `Schema` value type, which means the gateway must own the orchestration.
- **Replacing Seaography.** Out of scope.

### Decision 8 — Cross-cutting layers (auth, CORS, tracing, body limits, cookies) live with each domain

The auth layer, CORS layer, cookies, body limits, and OpenTelemetry layers are needed by every domain that serves HTTP. Each domain therefore **owns its own copies** of these layer factories (in `domain_posts::domain::layers`). The gateway also owns them (because it serves the composed router). The two implementations are identical — they are just two copies of the same factory functions. Cross-cutting concerns are not abstracted into a shared crate; they are explicitly duplicated by convention.

When the domain runs standalone, the domain's `bin` applies its own layers. When the domain is composed, the gateway applies its own layers. The two layers are identical in behavior (same env vars, same builder calls).

Rejected alternatives:

- **A shared kernel crate for cross-cutting layers.** Rejected — shared kernel is explicitly forbidden.
- **A procedural macro that injects layers.** Rejected — explicit code is faster to debug.

### Decision 9 — Inter-domain communication is by shared schema only, not by direct call

In the target, every domain shares the same PostgreSQL schema. Cross-domain logic that currently lives in a shared kernel (e.g., `PostCreateHandler` calling `TagCreateHandler`) is resolved by **lifting the relevant helper into `domain_posts`** (Decision 2 — `domain_posts::handlers::tag_helper`). When a future domain entry creates a tag, the gateway registers both `domain_posts` and the new domain, and the new domain's tag handler is the canonical place.

No domain directly imports another domain's implementation. The only allowed imports are:

- `domain_X` → `domain_interface` (to implement the contract).
- `domain_X` → `domain_Y` (only when `domain_X` legitimately needs `domain_Y` for read-only access to a shared entity, and approved by the spec).
- `gateway` → `domain_interface` + `domain_X` (for each registered domain).

Rejected alternatives:

- **Inter-domain gRPC or HTTP.** Out of scope (see Non-Goals).
- **A shared kernel crate.** Rejected.

### Decision 10 — Test helpers stay at the workspace root

`test_helpers` (`apps/api/test_helpers/`) stays in the workspace and is reused by `domain_posts`, `gateway`, and every other `domain_<name>`. It depends on `sea-orm-migration` for the `MigratorTrait` import and is otherwise neutral. In the target, each domain's `tests` depend on `test_helpers` (or on the domain's own `migrations::Migrator` once the domain crate is in place).

Tests for `domain_posts` live in `domain_posts/tests/` and `domain_posts/src/handlers/**/tests` so they ship with the domain.

Rejected alternatives:

- **Move `test_helpers` into `domain_posts`.** Rejected because the gateway and every other domain also need it.
- **A shared kernel crate for tests.** Rejected.

### Decision 11 — Every future `domain_<name>` follows the same self-contained ownership model

A new domain is created by **copying the entire `domain_posts` crate** into `domain_<name>/`, renaming the crate, replacing `posts` identifiers, and registering a `Domain<Name>Service` in the gateway's `Vec<Box<dyn DomainService>>`. The new domain owns its complete domain core/application logic, its own API adapters, its own entities/data access, its own domain-specific infrastructure/integrations, its own migrations, its own migration runner, and its own tests — and is independently runnable/deployable as a microservice. The scaffold is committed under `apps/api/domain_posts/` as the canonical reference. There is no separate `templates/` directory.

The repeatable boundary is:

```
domain_<name>/
├── Cargo.toml             # own infrastructure dependencies
├── src/
│   ├── lib.rs             # pub use Domain<Name>Service, ...
│   ├── main.rs            # own bin (standalone microservice)
│   ├── api/               # own HTTP adapters
│   ├── handlers/          # own domain core / application logic
│   ├── domain/            # own infrastructure / integrations
│   ├── entities/          # own SeaORM-generated entities
│   ├── migrations/        # own migrations + own Migrator
│   ├── service.rs         # impl DomainService
│   ├── observability.rs   # own tracing + OTLP init
│   └── migrations_cli.rs  # own migration runner
└── tests/                 # own tests
```

Rejected alternatives:

- **A procedural macro that generates the crate.** Rejected — copy-paste is faster to understand and matches the existing convention.
- **A separate `templates/domain_template/` crate.** Rejected — the canonical reference is the first extracted domain (`domain_posts`); a template duplicates the canonical crate.

## Risks / Trade-offs

- [Risk] The current `application_core` and `migration` crates will be emptied/folded into per-domain crates. Any code that imports `application_core::commands::*` or `application_core::entities::*` breaks until updated. → Mitigation: the implementation phase updates import paths in a mechanical `rg`-driven pass (tracked in `tasks.md`); the migration is done in stages so every step keeps the workspace green.
- [Risk] The cross-domain call inside `PostCreateHandler` (which currently reaches into `commands::tag::create::create_handler::TagCreateHandler`) is resolved by lifting the relevant helper into `domain_posts::handlers::tag_helper`. If the lifted helper diverges from the eventual future tag-domain handler, the two implementations will drift. → Mitigation: the lifted helper is **only** the parts of tag creation that post creation needs (`create_tags_in_transaction`); no behavior change. When a future `domain_tags` is extracted, it reuses the canonical schema and the existing `tags` table; the helper is moved again at that time.
- [Risk] `sea-orm-migration::Migrator` is the same struct used by `test_helpers` (`Migrator::refresh(&conn)`). Once the migration `Migrator` moves into `domain_posts::migrations`, `test_helpers` must depend on `domain_posts::migrations` or maintain a re-export. → Mitigation: `domain_posts::migrations` exposes `pub use sea_orm_migration::prelude::*;` and `pub struct Migrator;` with `impl MigratorTrait`; `test_helpers` switches to `domain_posts::migrations::Migrator` when `domain_posts` is added to its `[dependencies]`.
- [Risk] The gateway depends on `domain_posts` (Cargo). If `domain_posts` is published or vendored, the workspace version must be pinned. → Mitigation: the workspace uses path dependencies for now; a follow-up change will add a workspace `version` policy.
- [Risk] Three `AppState` instances today become one `DomainContext` shared by every domain. Any divergence in env/connection lifetime could change behavior. → Mitigation: `domain_posts::domain::postgres::connect_database()` is the single source of truth; the gateway calls it once and passes the resulting `Arc<DatabaseConnection>` to every domain's `DomainContext`. Behavior parity tests assert one connection pool, three router groups, and identical health responses.
- [Risk] The Postgres schema is shared between domains. A future domain might modify a table owned by `domain_posts`. → Mitigation: per Decision 2, only `domain_posts` owns migrations for the tables it currently controls. Future domains add migrations with explicit `depends_on` ordering and own only their tables.
- [Risk] `code-review-graph` MCP unavailable. → Mitigation: explicit fallback to `rg`, `cargo metadata`, and `openspec list --json`; documented in proposal and tasks.
- [Risk] Dev hot-reload (`cargo run` from `apps/api/`) may need a re-run after `Cargo.toml` `members` changes. → Mitigation: documented in the rollout; the verification gate is the canonical completion signal.
- [Risk] Two `Seaography` schemas are built twice today. The orchestrator must still build them only once. → Mitigation: the gateway constructs the schemas once and stores them in `DomainContext`; every domain receives the same `Arc<Schema>` values.
- [Risk] The `applications_core/commands/ai/translate/translate_handler.rs` module is large (1400+ lines) and moves into `domain_posts::handlers::post::translate` intact. → Mitigation: the move is a code move plus import path updates; no logic changes.
- [Risk] The Pgvector `VectorStore` is intimately tied to post translation and uses `async-openai`. It moves into `domain_posts::handlers::vector_store` and depends on `OpenAI` env vars. → Mitigation: the env-var surface is unchanged; the `required_env` of `DomainPostService` reflects this.

## Migration Plan

This section describes the **implementation** steps that move from the current baseline (see the labeled "Current implementation baseline" diagram at the top of the Architecture section) to the target architecture (see the "Target architecture" diagrams above). The steps are ordered by dependency. Each step is a separate PR and keeps the verification gate green.

1. **Workspace scaffold** (no behavior change): add `domain_interface` (publishable) and `domain_posts` (lib + bin) to `apps/api/Cargo.toml` `[workspace] members`. Add `gateway` (bin) for the composed deployment. New crates build; old code still compiles.
2. **Contract crate** (no behavior change): add `DomainService`, `DomainContext`, `Mount`, `RouteRegistration`, `HealthDescriptor`, `MigrationDescriptor`, `DomainConfigError` in `domain_interface`. Module tests confirm `DomainService` is object-safe. `domain_interface` is the only dependency of `domain_posts` and `gateway`.
3. **`domain_posts` skeleton** (no behavior change): scaffold `domain_posts` with `src/lib.rs`, `src/main.rs`, `src/domain/error.rs`, `src/domain/response.rs`, `src/api/mod.rs`, `src/handlers/mod.rs`, `src/migrations/mod.rs`, `src/entities/mod.rs`, `src/service.rs`. The crate compiles; `DomainPostService::register_routes` returns an empty `Vec<RouteRegistration>`.
4. **Code move into `domain_posts`** (no behavior change): move the post HTTP adapters, command handlers, DTOs, common helpers, generated entities for post tables, and the four migrations into `domain_posts`. Resolve the `PostCreateHandler → TagCreateHandler` cross-domain call by lifting the relevant helper into `domain_posts::handlers::tag_helper`. The current `application_core` and `migration` crates are emptied of post-related code; their remaining modules (categories, tags, media, users, ai) are addressed in follow-up changes.
5. **Standalone binary** (no behavior change): implement `domain_posts::main` (the `bin` target) to boot Axum, apply layers, open the database connection, run migrations via `domain_posts::migrations_cli::run`, register the post routes, and serve. Verify `cargo run -p domain_posts` boots and `/health` returns 200.
6. **Gateway composition** (no behavior change): implement `gateway::main` to construct `DomainPostService`, wrap it in `Box::new(...)`, push it into a `Vec<Box<dyn DomainService>>`, iterate the manifest to build the three Axum routers, and serve. The gateway owns the cross-cutting layers and the orchestrator. The gateway keeps the same `[[bin]] name = "my-cms-api"` for deployment image compatibility.
7. **Cut over** (no behavior change): remove the legacy `my-cms-api` bootstrap and the legacy `cms`/`application_core`/`migration` crates. Update `apps/api/Cargo.toml` so the canonical `my-cms-api` binary is produced by `gateway`. The `domain_interface` + `domain_posts` + `gateway` + `test_helpers` workspace is the target composition.
8. **Rollout and rollback**: each step is a separate PR. The repository verification gate (`cargo check`, `cargo test`, `cargo fmt -- --check`, `cargo clippy --all-targets -- -D warnings`, `pnpm --dir apps/web build`) is the go/no-go signal. Rollback is `git revert` of the latest PR. A follow-up change archives this one and adds the next domains (`domain_categories`, `domain_tags`, `domain_media`, `domain_users`) using the same self-contained ownership model.

Verification per step:

- `cargo check -p domain_interface -p domain_posts -p gateway`
- `cargo test -p domain_interface`
- `cargo test -p domain_posts --no-fail-fast`
- `cargo test -p gateway --no-fail-fast` (router + health integration)
- `cargo run -p domain_posts -- migrate` against the test database
- `cargo run -p domain_posts` standalone serves `/health`, `/posts/**`, `/posts/{post_id}/translate`, `/posts/{post_id}/translate/background`, `/posts/{post_id}/translate/jobs/**`
- `cargo run -p gateway` composed serves the same routes plus the legacy compatibility routes
- `cargo fmt -- --check && cargo clippy --all-targets --all-features`
- `pnpm --dir apps/web build` (frontend contract untouched)

## Open Questions

- Should `domain_posts` be a single crate with both `lib` and `bin` targets, or one crate with `lib` and a sibling `bin` crate that depends on it? The default: single crate with `[lib]` and `[[bin]]` entries in the same `Cargo.toml`. Confirm before implementation.
- When should the legacy `application_core` and `migration` crates be removed? The default: remove them as part of step 7 (cut over) once posts are fully extracted. The intermediate steps keep the legacy crates as carry-over for the not-yet-extracted domains (categories, tags, media, users, ai). Confirm before implementation.
- Should the `gateway` crate be named `gateway` or `my_cms_api` or `my-cms-api`? The default: `gateway` (cleaner Cargo crate name); the binary it produces is `my-cms-api` (preserving the deployment image name). Confirm before implementation.
- Should `domain_posts::entities` keep the historical entity set (categories, tags, post_*) or be split into a post-only entity set with a follow-up re-generation? The default: keep the historical entity set to avoid requiring a regenerator pass; split in a follow-up change when each future domain is extracted. Confirm before implementation.
- Should `domain_interface` be versioned at `0.1.0` and published to crates.io now, or remain path-only until the first PR ships? The default: remain path-only until the first PR merges; publishing is a separate change. Confirm before implementation.
- Should the `Vec<Box<dyn DomainService>>` registration manifest live in a separate `gateway/manifest.rs` file, or in `gateway/src/main.rs`? The default: `gateway/manifest.rs`. Confirm before implementation.
