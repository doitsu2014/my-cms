# My-CMS API Architecture (Implemented)

This document captures the **as-built** state of the API architecture after `refactor-api-into-pluggable-domain-libraries`, `consolidate-category-ai-translate-into-domain-posts`, `merge-graphql-into-posts-domain`, and `migrate-legacy-to-domain-posts`. It is the visual companion to `pluggable-domain-refactor.md`.

> **Note on legacy shims (post `migrate-legacy-to-domain-posts`):** `apps/api/application_core/` and `apps/api/migration/` are transitional crates retained only to keep `legacy_bootstrap` compiling. Post, AI translation, vector-store, and pgvector code are **no longer** duplicated inside `application_core` — they live exclusively in `domain_posts`. `application_core::entities` and `apps/api/migration/` are pure re-export shims; `application_core::commands::{post,ai}` have been deleted. Neither crate has architectural responsibility in the current picture. Both remain slated for removal as legacy handlers in tags/media/users/administrator are extracted (see §12).

## 1. Cargo Workspace

```mermaid
graph LR
    subgraph ws["apps/api/  Cargo workspace"]
        DI["<b>domain_interface</b><br/>(publishable contract)<br/>DomainService trait<br/>DomainContext<br/>Mount, RouteRegistration<br/>HealthDescriptor<br/>MigrationDescriptor<br/>DomainConfigError<br/>AuthenticatedActor (actor value type)"]
        DA["<b>domain_auth</b><br/>cross-cutting infrastructure crate<br/>SupabaseAuthLayer,<br/>SupabaseAuthConfig,<br/>SupabaseClaims, SupabaseToken<br/>construct_supabase_auth_layer<br/>DomainAuthService impl<br/>(empty routes, default startup_health,<br/>no sea-orm, no business deps)"]
        DP["<b>domain_posts</b><br/>lib + bin<br/>api/{post,category,ai}/* (HTTP adapters)<br/>handlers/{post,tag_helper,<br/>category,ai,vector_store,<br/>translation_jobs}/* (commands)<br/>handlers/post::translate (pipeline)<br/>domain/{response,error,<br/>layers,graphql,postgres}<br/>entities/* (canonical)<br/>migrations/* (4 identities)<br/>service.rs (DomainPostService)"]
        GW["<b>gateway</b><br/>bin: my-cms-api<br/>manifest() → Box&lt;dyn<br/>DomainService&gt;<br/>orchestrator<br/>compose_routers<br/>(applies domain_auth layer to<br/>protected + administrator)"]
        TH["<b>test_helpers</b><br/>testcontainers + Postgres +<br/>pgvector"]
        CMS["<b>cms</b><br/>(legacy bootstrap root package)<br/>src/api/{tag,media,user,<br/>administrator}/*<br/>src/api/post/* (HTTP adapters<br/>importing from domain_posts)<br/>src/api/{delete,tag/delete}/*<br/>src/lib.rs → AppState (legacy)<br/>bin: legacy_bootstrap<br/>(depends on legacy shims; see §11)"]
    end

    subgraph ext["External / Platform"]
        DB[("PostgreSQL + pgvector<br/>(Supabase)")]
        STORE[("Supabase Storage<br/>(S3-compatible)")]
        AUTH[("Supabase GoTrue / JWT")]
        OAI["OpenAI API<br/>(translation + embeddings)"]
    end

    GW --> DI
    GW --> DP
    GW --> DA
    GW --> TH

    DP --> DI
    DP --> TH

    DA --> DI

    CMS --> DA
    CMS --> TH
    CMS --> DP

    DP -- "SeaORM, OpenAI,<br/>pgvector" --> DB
    DP -- "OpenAI / pgvector" --> OAI
    DP -- "SupabaseStorage" --> STORE
    DA -- "SupabaseAuthLayer<br/>(construct_supabase_auth_layer)" --> AUTH
    GW -- "OTLP / tracing" --> OAI
    CMS -- "all integrations<br/>(legacy)" --> DB
    CMS -- "all integrations<br/>(legacy)" --> STORE
    CMS -- "all integrations<br/>(legacy)" --> AUTH
    CMS -- "all integrations<br/>(legacy)" --> OAI

    classDef contract fill:#e6f3ff,stroke:#1f6feb,stroke-width:2px,color:#0b3d91
    classDef domain fill:#e8f5e9,stroke:#2e7d32,stroke-width:2px,color:#1b5e20
    classDef infra fill:#fff7e6,stroke:#b07000,stroke-width:2px,color:#6b4500
    classDef gateway fill:#f3e8ff,stroke:#6f42c1,stroke-width:2px,color:#3a1d63
    classDef shim fill:#fff4e6,stroke:#d97706,stroke-width:1px,color:#7c2d12
    classDef ext fill:#fde7ef,stroke:#bf2c7e,stroke-width:1px,color:#7a1148
    classDef helper fill:#f5f5f5,stroke:#6c757d,stroke-width:1px,color:#495057

    class DI contract
    class DP domain
    class DA infra
    class GW gateway
    class CMS shim
    class TH helper
    class DB,STORE,AUTH,OAI ext
```

**Workspace member legend:**
- **Pluggable domain architecture** (the architecture): `domain_interface`, `domain_auth`, `domain_posts`, `gateway`, `test_helpers`.
- **Legacy bootstrap** (transitional): `cms` root package — retains the `legacy_bootstrap` binary until tags/media/users/administrator are extracted into their own domains.
- **Legacy shims** (no architectural responsibility; see §11): `application_core`, `migration`.

## 2. Deployment Modes — Two Binaries

The cutover is staged. Two binaries are produced today:

```mermaid
graph TB
    subgraph traefik["Traefik / Listener"]
        LB["Reverse Proxy<br/>(routes by path prefix)"]
    end

    subgraph binA["Binary: my-cms-api  (gateway composition)"]
        direction TB
        GA["gateway/src/main.rs<br/>• env + tracing init<br/>• connect_database()<br/>• run_orchestrator()<br/>• build schemas<br/>• compose_routers()<br/>• bind listener"]
        MA["Manifest:<br/>vec![<br/>  Box::new(DomainPostService::new()),<br/>  Box::new(DomainAuthService::new()),<br/>]"]
        GA --> MA
    end

    subgraph binB["Binary: legacy_bootstrap  (transitional)"]
        direction TB
        LA["apps/api/src/bin/legacy_bootstrap.rs<br/>• env + tracing init<br/>• construct_app_state()<br/>• public_router()<br/>• protected_router()<br/>• protected_administrator_router()<br/>• bind listener<br/>• applies domain_auth::<br/>legacy_bootstrap::<br/>construct_supabase_auth_layer"]
    end

    subgraph dp["domain_posts (standalone)"]
        DA["domain_posts/src/main.rs<br/>• env + tracing init<br/>• connect_database()<br/>• build schemas<br/>• build DomainPostService<br/>• register_routes(&ctx)<br/>• bind listener"]
    end

    LB --> binA
    LB --> binB
    DA -. "alternative deployment<br/>(not behind Traefik)" .-> LB

    classDef gateway fill:#f3e8ff,stroke:#6f42c1,stroke-width:2px,color:#3a1d63
    classDef domain fill:#e8f5e9,stroke:#2e7d32,stroke-width:2px,color:#1b5e20
    classDef shim fill:#fff4e6,stroke:#d97706,stroke-width:1px,color:#7c2d12
    classDef lb fill:#f5f5f5,stroke:#6c757d,stroke-width:1px,color:#495057

    class binA gateway
    class dp domain
    class binB shim
    class traefik lb
```

## 3. Gateway Composition — `my-cms-api`

```mermaid
graph LR
    subgraph gw["gateway::main()"]
        direction TB
        CTX["DomainContext {<br/>  conn: Arc&lt;DatabaseConnection&gt;,<br/>  graphql_immutable: Arc&lt;Schema&gt;,<br/>  graphql_mutable: Arc&lt;Schema&gt;<br/>}"]
        ORCH["orchestrator::run_orchestrator()<br/>collects MigrationDescriptors,<br/>dedupes by id,<br/>dispatches to per-domain runner"]
        COMP["compose_routers()<br/>merges RouteRegistration per Mount"]
        SERV["axum::serve(<br/>  listener,<br/>  app.with_state(ctx)<br/>)"]
    end

    subgraph public_routes["Public Router (no auth)"]
        PR1["GET  /"]
        PR2["GET  /health"]
        PR3["GET  /healthz"]
        PR4["GET  /posts/graphql/immutable  (playground)<br/>POST /posts/graphql/immutable  (handler)"]
        PR5["GET  /posts/graphql/mutable    (playground)<br/>POST /posts/graphql/mutable    (handler)"]
    end

    subgraph protected_routes["Protected Router (auth)"]
        PPR1["GET    /posts<br/>POST   /posts<br/>PUT    /posts<br/>DELETE /posts"]
        PPR2["GET    /posts/{post_id}"]
        PPR3["POST   /posts/{post_id}/translate"]
        PPR4["POST   /posts/{post_id}/translate/background"]
        PPR5["GET    /posts/{post_id}/translate/jobs/{job_id}"]
        PPR6["GET    /posts/{post_id}/translate/jobs"]
        PPR7["GET/POST/PUT/DELETE /categories<br/>GET /categories/{category_id}"]
        PPR8["GET /ai/models"]
    end

    subgraph administrator_routes["Administrator Router (admin auth)"]
        AR1["(placeholder —<br/>post domain contributes<br/>no administrator routes)"]
    end

    MAN["DomainPostService.register_routes(&ctx)"]
    subgraph dps["domain_posts::api::routes()"]
        DPSR1["RouteRegistration { mount: Public }"]
        DPSR2["RouteRegistration { mount: Protected, prefix: /posts }"]
        DPSR3["RouteRegistration { mount: Protected, prefix: /categories }"]
        DPSR4["RouteRegistration { mount: Protected, prefix: /ai }"]
        DPSR5["RouteRegistration { mount: Administrator, prefix: /posts-admin }"]
        DPSR6["RouteRegistration { mount: Public, prefix: /posts/graphql }"]
        DPSR7["RouteRegistration { mount: Protected, prefix: /posts/graphql }"]
    end

    GW --> CTX
    CTX --> ORCH
    ORCH --> COMP
    COMP --> SERV
    COMP --> PR1 & PR2 & PR3 & PR4 & PR5
    COMP --> AR1
    MAN --> DPSR1 & DPSR2 & DPSR3 & DPSR4 & DPSR5 & DPSR6 & DPSR7
    MAN --> AUTH["domain_auth::DomainAuthService<br/>(registered; empty routes,<br/>validate_config only)"]
    DPSR1 --> PR1
    DPSR2 --> PPR1 & PPR2 & PPR3 & PPR4 & PPR5 & PPR6
    DPSR3 --> PPR7
    DPSR4 --> PPR8
    DPSR5 --> AR1
    DPSR6 --> PR4
    DPSR7 --> PR5

    classDef domain fill:#e8f5e9,stroke:#2e7d32,stroke-width:2px,color:#1b5e20
    classDef infra fill:#fff7e6,stroke:#b07000,stroke-width:2px,color:#6b4500
    classDef gateway fill:#f3e8ff,stroke:#6f42c1,stroke-width:2px,color:#3a1d63
    classDef route fill:#fff,stroke:#888,stroke-width:1px,color:#333
    classDef shim fill:#fff4e6,stroke:#d97706,stroke-width:1px,color:#7c2d12

    class CTX,ORCH,COMP,SERV gateway
    class PR1,PR2,PR3,PR4,PR5,PPR1,PPR2,PPR3,PPR4,PPR5,PPR6,PPR7,PPR8,AR1 route
    class MAN,DPSR1,DPSR2,DPSR3,DPSR4,DPSR5,DPSR6,DPSR7 domain
    class AUTH infra
```

## 4. Legacy Bootstrap — `legacy_bootstrap`

```mermaid
graph TB
    subgraph lb["apps/api/src/bin/legacy_bootstrap.rs"]
        direction TB
        LSTATE["AppState {<br/>  conn, media_config,<br/>  media_cache, bucket_visibility_cache,<br/>  graphql_immutable_schema,<br/>  graphql_mutable_schema,<br/>  supabase_admin_client<br/>}"]
        LPR["public_router()<br/>+ cors_layer + otel_layers"]
        LPRR["protected_router()<br/>+ auth (writer or admin)<br/>+ cookie + body-limit<br/>+ cors + otel"]
        LPAR["protected_administrator_router()<br/>+ auth (admin only)<br/>+ cookie + cors + otel"]
        LSERV["axum::serve(listener, app)"]
    end

    subgraph lpublic["Public"]
        LP1["GET /"]
        LP2["GET /health"]
        LP3["GET /healthz"]
        LP4["GET /media/images/{*path}"]
        LP5["GET /media/{*path}"]
        LP6["GET /posts/graphql/immutable<br/>POST /posts/graphql/immutable"]
        LP7["GET /posts/graphql/mutable<br/>POST /posts/graphql/mutable"]
    end

    subgraph lprot["Protected (writer or admin)"]
        LPR2["GET/POST/PUT/DELETE /posts<br/>GET /posts/{post_id}"]
        LPR3["POST /posts/{post_id}/translate<br/>POST /posts/{post_id}/translate/background<br/>GET /posts/{post_id}/translate/jobs/{job_id}<br/>GET /posts/{post_id}/translate/jobs"]
        LPR5["DELETE /tags"]
        LPR6["GET/POST/DELETE /media<br/>GET /media/info/{*path}<br/>DELETE /media/delete/{*path}"]
        LPR7["GET/POST /posts/graphql/mutable"]
    end

    subgraph ladmin["Administrator (admin only)"]
        LA1["POST /administrator/database/migration"]
        LA2["GET/POST /users<br/>GET/PUT/DELETE /users/{user_id}<br/>POST /users/{user_id}/reset-password"]
        LA3["GET/POST /media/buckets<br/>GET/PUT/DELETE /media/buckets/{name}<br/>POST /media/buckets/{name}/empty"]
    end

    LSTATE --> LPR & LPRR & LPAR
    LPR --> LP1 & LP2 & LP3 & LP4 & LP5 & LP6 & LP7
    LPRR --> LPR2 & LPR3 & LPR5 & LPR6 & LPR7
    LPAR --> LA1 & LA2 & LA3
    LPR & LPRR & LPAR --> LSERV

    classDef shim fill:#fff4e6,stroke:#d97706,stroke-width:1px,color:#7c2d12
    classDef route fill:#fff,stroke:#888,stroke-width:1px,color:#333

    class LSTATE,LPR,LPRR,LPAR,LSERV shim
    class LP1,LP2,LP3,LP4,LP5,LP6,LP7,LPR2,LPR3,LPR5,LPR6,LPR7,LA1,LA2,LA3 route
```

## 5. Domain Ownership — `domain_posts` Internals

```mermaid
graph LR
    subgraph dps["apps/api/domain_posts/"]
        direction TB

        subgraph api["src/api/"]
            APIC["post/create/create_handler.rs"]
            APID["post/delete/delete_handler.rs"]
            APIM["post/modify/modify_handler.rs"]
            APIR["post/read/read_handler.rs"]
            APIT["post/translate/translate_handler.rs"]
            APIJ["post/translate/job_handler.rs"]
            APIG["post/graphql/mod.rs<br/>(playground_immutable,<br/>playground_mutable)"]
            APICC["category/{create,read,modify,delete}/*"]
            APII["ai/models/models_handler.rs"]
        end

        subgraph handlers["src/handlers/"]
            HC["post/create/*<br/>(PostCreateHandler)"]
            HD["post/delete/*<br/>(PostDeleteHandler)"]
            HM["post/modify/*<br/>(PostModifyHandler)"]
            HR["post/read/*<br/>(PostReadHandler)"]
            HT["post/translate/*<br/>(PostTranslateHandler)"]
            HV["vector_store/*<br/>(VectorStore)"]
            HTG["tag_helper/{create,read}/*<br/>(TagCreateHandler,<br/>TagReadHandler)"]
            HTT["translation_jobs/*"]
            HCA["category/{create,read,modify,delete}/*<br/>(CategoryCreateHandler,<br/>CategoryReadHandler,<br/>CategoryModifyHandler,<br/>CategoryDeleteHandler)"]
            HAI["ai/models/* + openai_client_from_env<br/>(ModelsHandler,<br/>OpenAIClient factory)"]
        end

        subgraph domain["src/domain/"]
            DERR["error.rs<br/>AppError"]
            DRES["response.rs<br/>ApiResponseWith/Error<br/>ErrorCode, AxumResponse"]
            DLA["layers.rs<br/>cors_layer()<br/>body_limit_layer()<br/>otel_layers()<br/>cookie_layer()"]
            DPG["postgres.rs<br/>connect_database()"]
            DGQ["graphql.rs<br/>contribute_post_schema()<br/>(Seaography schema<br/>with all post entities)"]
            DVS["vector_store.rs<br/>(pgvector adapter)"]
            DEX["extensions.rs<br/>StringExtension<br/>generate_vietnam_now"]
            DEV["env.rs<br/>POST_REQUIRED_ENV"]
        end

        subgraph entities["src/entities/"]
            ENT["posts, post_tags,<br/>post_translations,<br/>translation_jobs,<br/>tags, test_fulltext,<br/>categories, category_tags,<br/>category_translations,<br/>sea_orm_active_enums<br/>(canonical; physically here)"]
        end

        subgraph migrations["src/migrations/"]
            MM["mod.rs<br/>Migrator<br/>POST_MIGRATION_IDS<br/>migration_descriptors()"]
            M1["m20240409_151952_release_100"]
            M2["m20250330_151455_release_110"]
            M3["m20260126_040610_release_300"]
            M4["m20260531_000001_pgvector"]
        end

        subgraph glue["glue"]
            SVC["service.rs<br/>DomainPostService impl<br/>DomainService trait"]
            MCL["migrations_cli.rs<br/>run(), handle_args()"]
            OBS["observability.rs<br/>init()"]
        end
    end

    DAU["<b>domain_auth</b><br/>cross-cutting Supabase JWT layer<br/>SupabaseAuthLayer<br/>SupabaseAuthConfig<br/>SupabaseClaims<br/>SupabaseToken<br/>construct_supabase_auth_layer<br/>DomainAuthService impl<br/>(empty routes, default startup_health)"]

    APIC --> HC
    APID --> HD
    APIM --> HM
    APIR --> HR
    APIT --> HT
    APIJ --> HTT
    APIG --> DGQ
    APICC --> HCA
    APII --> HAI
    HC -->|uses tag_helper<br/>for tag create-in-tx| HTG
    HT --> HV
    HCA -->|uses tag_helper<br/>for category create-in-tx| HTG
    HT -->|calls openai_client_from_env| HAI
    ENT -. "owns" .-> HC & HD & HM & HR & HT & HTG & HCA & HAI
    MM --> M1 & M2 & M3 & M4
    SVC --> MM
    SVC --> api
    SVC --> DRES & DERR & DLA & DPG & DGQ
    SVC -->|consumes via<br/>domain_auth| DAU
    MCL --> MM

    classDef domain fill:#e8f5e9,stroke:#2e7d32,stroke-width:2px,color:#1b5e20
    classDef infra fill:#fff7e6,stroke:#b07000,stroke-width:2px,color:#6b4500

    class APIC,APID,APIM,APIR,APIT,APIJ,APIG,APICC,APII,HC,HD,HM,HR,HT,HV,HTG,HTT,HCA,HAI,DERR,DRES,DLA,DPG,DGQ,DVS,DEX,DEV,MM,M1,M2,M3,M4,SVC,MCL,OBS,ENT domain
    class DAU infra
```

## 6. Migration Identities — `domain_posts`

```mermaid
graph LR
    subgraph m["domain_posts::migrations::Migrator"]
        direction TB
        M1["m20240409_151952_release_100<br/>(initial schema:<br/>categories, posts, tags,<br/>category_tags, post_tags,<br/>category_translations,<br/>post_translations,<br/>test_fulltext,<br/>translation_jobs)"]
        M2["m20250330_151455_release_110<br/>(schema evolution)"]
        M3["m20260126_040610_release_300<br/>(schema evolution)"]
        M4["m20260531_000001_pgvector<br/>(pgvector extension)"]
    end

    subgraph mt["MigrationDescriptor"]
        MD1["id='m20240409_151952_release_100'<br/>depends_on=&[]"]
        MD2["id='m20250330_151455_release_110'<br/>depends_on=&[]"]
        MD3["id='m20260126_040610_release_300'<br/>depends_on=&[]"]
        MD4["id='m20260531_000001_pgvector'<br/>depends_on=&[]"]
    end

    M1 --> MD1
    M2 --> MD2
    M3 --> MD3
    M4 --> MD4

    classDef domain fill:#e8f5e9,stroke:#2e7d32,stroke-width:2px,color:#1b5e20

    class M1,M2,M3,M4,MD1,MD2,MD3,MD4 domain
```

> Identities preserved exactly. Database `up` history unchanged.

## 7. Data Flow — POST /posts (Composed Gateway)

```mermaid
sequenceDiagram
    autonumber
    participant Client
    participant LB as Traefik
    participant GW as gateway::my-cms-api
    participant Auth as SupabaseAuthLayer
    participant Router as Axum Router
    participant DP as domain_posts::DomainPostService
    participant Api as api/post/create/create_handler.rs
    participant Handler as handlers::PostCreateHandler
    participant TagHelper as handlers::tag_helper::TagCreateHandler
    participant DB as PostgreSQL

    Client->>LB: POST /posts (Bearer JWT)
    LB->>GW: forward request
    GW->>Auth: validate JWT (writer or admin)<br/>(domain_auth::SupabaseAuthLayer)
    Auth-->>GW: AuthenticatedActor Extension
    GW->>Router: dispatch Mount::Protected
    Router->>DP: api_create_post
    DP->>Api: route to api::post::create
    Api->>Handler: PostCreateHandler::handle_create_post(body, actor_email)
    Handler->>TagHelper: tag_helper::create_tags_in_transaction(tags, actor_email, tx)<br/>(actor_email from AuthenticatedActor)
    TagHelper->>DB: INSERT new tags
    DB-->>TagHelper: tag ids
    TagHelper-->>Handler: CreateTagsResponse { new_tag_ids, existing_tag_ids }
    Handler->>DB: INSERT post row
    Handler->>DB: INSERT post_tags
    Handler->>DB: INSERT post_translations (if any)
    Handler-->>Api: Uuid
    Api-->>Router: ApiResponseWith::new(uuid)
    Router-->>GW: 200 OK envelope
    GW-->>LB: 200 OK
    LB-->>Client: response body
```

## 8. Data Flow — POST /posts (Standalone `domain_posts` Bin)

```mermaid
sequenceDiagram
    autonumber
    participant Client
    participant LB as Traefik
    participant DP as domain_posts bin
    participant Auth as SupabaseAuthLayer
    participant Router as Axum Router
    participant Handler as handlers::PostCreateHandler
    participant TagHelper as handlers::tag_helper
    participant DB as PostgreSQL

    Client->>LB: POST /posts (Bearer JWT)
    LB->>DP: forward request
    DP->>Auth: validate JWT<br/>(domain_auth::SupabaseAuthLayer)
    Auth-->>DP: AuthenticatedActor Extension
    DP->>Router: dispatch
    Router->>Handler: handle_create_post
    Handler->>TagHelper: create_tags_in_transaction
    TagHelper->>DB: INSERT new tags
    DB-->>TagHelper: tag ids
    TagHelper-->>Handler: CreateTagsResponse
    Handler->>DB: INSERT post, post_tags, post_translations
    Handler-->>Router: Uuid
    Router-->>DP: ApiResponseWith::new(uuid)
    DP-->>LB: 200 OK
    LB-->>Client: response body
```

> Identical request path; only the listener differs. The standalone `domain_posts` bin does not compose other domains.

## 9. Cross-Domain Call — TagHelper Resolution

```mermaid
graph LR
    subgraph before["Before (cross-domain import)"]
        direction TB
        PCH_OLD["PostCreateHandler<br/>(application_core)"]
        TCH_OLD["TagCreateHandler<br/>(application_core)"]
        PCH_OLD -. "TagCreateHandler::<br/>:TagCreateHandlerTrait" .-> TCH_OLD
    end

    subgraph after["After (in-domain helper)"]
        direction TB
        PCH_NEW["PostCreateHandler<br/>(domain_posts/handlers/post/create)"]
        TH_NEW["tag_helper::create_handler<br/>(domain_posts/handlers/tag_helper)"]
        THR_NEW["tag_helper::read_handler<br/>(domain_posts/handlers/tag_helper)"]
        PCH_NEW -->|uses| TH_NEW
        TH_NEW -->|uses| THR_NEW
    end

    classDef bad fill:#ffe6e6,stroke:#cc0000,stroke-width:1px,color:#660000
    classDef good fill:#e8f5e9,stroke:#2e7d32,stroke-width:2px,color:#1b5e20

    class PCH_OLD,TCH_OLD bad
    class PCH_NEW,TH_NEW,THR_NEW good
```

> `domain_posts` no longer depends on `application_core::commands::tag::*`. The tag create + read handlers are owned by `domain_posts::handlers::tag_helper::*`. The `application_core` references in this diagram are **historical** (pre-`refactor-api-into-pluggable-domain-libraries`) and only exist today as legacy re-exports for the `legacy_bootstrap` path (see §11).

## 10. What Each Binary Actually Serves Today

| Route prefix | `my-cms-api` (gateway) | `legacy_bootstrap` | `domain_posts` (standalone) |
|---|---|---|---|
| `/`, `/health`, `/healthz` | ✅ | ✅ | ✅ |
| `/posts/graphql/immutable`, `/posts/graphql/mutable` | ✅ | ✅ | ✅ |
| `/posts/**`, `/posts/{post_id}` | ✅ | ✅ | ✅ |
| `/posts/{post_id}/translate{,/background}` | ✅ | ✅ | ✅ |
| `/posts/{post_id}/translate/jobs{,/**}` | ✅ | ✅ | ✅ |
| `/categories/**` | ✅ | ❌ | ✅ |
| `/ai/models` | ✅ | ❌ | ✅ |
| `/tags` | ❌ | ✅ | ❌ |
| `/media/**`, `/media/buckets/**`, `/media/info/**`, `/media/delete/**`, `/media/images/**` | ❌ | ✅ | ❌ |
| `/users/**` | ❌ | ✅ | ❌ |
| `/administrator/database/migration` | ❌ | ✅ | ❌ |

> `/categories/**` and `/ai/models` moved from `legacy_bootstrap` to the gateway (`my-cms-api`) and to the standalone `domain_posts` bin per the `consolidate-category-ai-translate-into-domain-posts` change. The `legacy_bootstrap` binary still serves the not-yet-extracted tags/media/users/administrator routes.

> **Note (`merge-graphql-into-posts-domain`):** The GraphQL HTTP surface moved from `/graphql/{immutable,mutable}` to `/posts/graphql/{immutable,mutable}`. The mutable mount now accepts the Supabase app roles `my-headless-cms-writer` and `my-headless-cms-administrator` (the gateway's pre-change administrator-only gate was widened to writer + administrator so all three deployment modes expose identical authorization behaviour). The post domain is the sole owner of the GraphQL playground handlers and the `Arc<Schema>` wiring.

## 11. Legacy Shims — `application_core` & `migration`

Two workspace members exist **only** to keep `legacy_bootstrap` compiling. They have no architectural responsibility in the pluggable domain picture above. They are slated for deletion as the first concrete step in the staged cutover (§12).

### `apps/api/application_core/`

- **Workspace member:** `application_core` (path-dep of the `cms` root package).
- **What it still exposes:**
  - `commands/{tag,media,user}` — handler structs and traits that the legacy handlers in `apps/api/src/api/...` import.
  - `entities/*` — **re-exports** of `domain_posts::entities::*` (the canonical entities live in `domain_posts`; `application_core` only re-exports them for the legacy path).
  - `common/{app_error, datetime_generator, extensions}` — small utilities the legacy handlers depend on.
  - `commands::{post,ai}` were deleted in `migrate-legacy-to-domain-posts`; their canonical replacements live under `domain_posts::handlers::post::*` and `domain_posts::handlers::vector_store::*`.
- **Where it's referenced:** `apps/api/src/bin/legacy_bootstrap.rs`, every legacy handler under `apps/api/src/api/{tag,media,user,...}/*`, and `apps/api/src/api/{delete,tag/delete}/*` (which import `PostDeleteHandler` from `domain_posts`). The legacy `apps/api/src/api/post/*` handlers still exist as thin HTTP adapters but their command-handler imports now point at `domain_posts` (no `application_core::commands::post::*` import remains).
- **Why it still exists:** to avoid a sweeping rewrite of the legacy `apps/api/src/api/...` tree during the pluggable-domain cutover. Each handler will be replaced wholesale as tags/media/users/administrator are extracted into their own domains.

### `apps/api/migration/`

- **Workspace member:** `migration` (path-dep of the `cms` root package and of `test_helpers`).
- **What it still exposes (post `migrate-legacy-to-domain-posts`):**
  - `lib.rs` — `pub use domain_posts::migrations::*; pub use domain_posts::migrations::Migrator;` re-export shim. No migration source files live here.
  - `main.rs` — `cli::run_cli(migration::Migrator).await` entry point for the `migration` standalone binary.
  - The four previously-duplicate migration source files (`m20240409_151952_release_100.rs`, `m20250330_151455_release_110.rs`, `m20260126_040610_release_300.rs`, `m20260531_000001_pgvector.rs`) and `constants.rs` were deleted in `migrate-legacy-to-domain-posts`. The canonical, authoritative copies live in `domain_posts::migrations::*`, so `cargo run -p migration -- migrate --list` still prints the same four migration IDs in the same order.
- **Where it's referenced:** `apps/api/src/bin/legacy_bootstrap.rs` (none), `apps/api/test_helpers/src/lib.rs` (`use migration::{Migrator, MigratorTrait}`), the `migration` standalone binary, and the legacy `/administrator/database/migration` handler.
- **Why it still exists:** the legacy bootstrap ships a `migration` binary that operators can invoke to run schema migrations, and `test_helpers` resolves `Migrator` through this shim. The canonical migrator is `domain_posts::migrations::Migrator`; `migration` is a thin re-export. Once `legacy_bootstrap` and the `/administrator/database/migration` route are removed, the `migration` binary also goes.

### Removal order (see §12)

1. `domain_media` extracted → `application_core::commands::media::*` becomes empty.
2. `domain_users` extracted → `application_core::commands::user::*` becomes empty.
3. `domain_tags` extracted → `application_core::commands::tag::*` becomes empty.
4. `domain_administrator` extracted → the legacy `/administrator/database/migration` route is removed; the `migration` standalone binary is no longer needed and `apps/api/migration/` can be deleted.
5. `apps/api/application_core/` deleted from workspace; `legacy_bootstrap` deleted.
6. `domain_posts` migrator remains the only migrator; the `migration` binary is replaced by `cargo run -p domain_posts --bin migrations_cli` (or the orchestrator's `run_orchestrator()` handles migrations at gateway startup).

## 12. Future Staged Cutover

```mermaid
graph TB
    subgraph now["Today (after consolidation)"]
        N1["my-cms-api serves post + categories + AI + translation routes"]
        N2["legacy_bootstrap serves<br/>tags/media/users/administrator<br/>(depends on application_core & migration shims; see §11)"]
    end

    subgraph step0["Step 0 — Delete legacy shims (prerequisite)"]
        S0["Once legacy handlers are gone,<br/>delete apps/api/application_core/<br/>and apps/api/migration/<br/>from the workspace.<br/>Unblocks clean domain extractions."]
    end

    subgraph next["Step 1 — domain_media, domain_users, domain_administrator extraction"]
        X1["Box::new(DomainMediaService),<br/>Box::new(DomainUsersService),<br/>Box::new(DomainAdministratorService)<br/>appended to gateway::manifest()"]
        X2["legacy_bootstrap loses /media/**, /users/**, /administrator/**<br/>gateway gains those routes"]
    end

    subgraph future["Step 2 — domain_tags extraction + legacy bootstrap removal"]
        F1["domain_tags registered in manifest()"]
        F2["legacy_bootstrap deleted;<br/>apps/api/src/ tree removed"]
        F3["single my-cms-api deployment image<br/>serves ALL routes via DomainService composition"]
    end

    now --> step0 --> next --> future

    classDef current fill:#fff,stroke:#888,stroke-width:1px
    classDef prereq fill:#ffe6e6,stroke:#cc0000,stroke-width:1px,color:#660000
    classDef nextStage fill:#fff4e6,stroke:#d97706,stroke-width:1px,color:#7c2d12
    classDef futureStage fill:#e8f5e9,stroke:#2e7d32,stroke-width:2px,color:#1b5e20

    class N1,N2 current
    class S0 prereq
    class X1,X2 nextStage
    class F1,F2,F3 futureStage
```

> **Prerequisite note:** Step 0 must be repeated for each domain extraction — `application_core::commands::<domain>_*` becomes empty as each handler migrates into its new domain crate. The workspace deletion of `application_core` and `migration` only happens after **all** legacy handler trees are gone. The `migration` binary in particular can be deleted once `/administrator/database/migration` and the `legacy_bootstrap` migrator CLI are both removed.

See `docs/adding-a-domain.md` for the recipe and `docs/pluggable-domain-refactor.md` for the full architectural overview.
