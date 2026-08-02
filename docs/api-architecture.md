# My-CMS API Architecture (Implemented)

This document captures the **as-built** state of the API architecture after the `refactor-api-into-pluggable-domain-libraries` change. It is the visual companion to `pluggable-domain-refactor.md`.

## 1. Cargo Workspace

```mermaid
graph LR
    subgraph ws["apps/api/  Cargo workspace"]
        DI["<b>domain_interface</b><br/>(publishable)<br/>DomainService trait<br/>DomainContext<br/>Mount, RouteRegistration<br/>HealthDescriptor<br/>MigrationDescriptor<br/>DomainConfigError"]
        DP["<b>domain_posts</b><br/>lib + bin<br/>api/{post,category,ai}/* (HTTP adapters)<br/>handlers/{post,tag_helper,<br/>category,ai,vector_store,<br/>translation_jobs}/* (commands)<br/>handlers/post::translate (pipeline)<br/>domain/{auth,response,error,<br/>layers,graphql,postgres}<br/>entities/* (canonical)<br/>migrations/* (4 identities)<br/>service.rs (DomainPostService)"]
        GW["<b>gateway</b><br/>bin: my-cms-api<br/>manifest() → Box&lt;dyn<br/>DomainService&gt;<br/>orchestrator<br/>compose_routers"]
        AC["<b>application_core</b><br/>(transitional shim)<br/>commands/{tag,media,user,ai::translate,<br/>ai::vector_store_pg}<br/>entities/* = re-export<br/>from domain_posts<br/>graphql/query_root<br/>common/{app_error,<br/>datetime_generator,extensions}"]
        MIG["<b>migration</b><br/>(transitional shim)<br/>re-exports Migrator from<br/>domain_posts::migrations"]
        TH["<b>test_helpers</b><br/>testcontainers + Postgres +<br/>pgvector"]
        CMS["<b>cms</b><br/>(legacy bootstrap lib)<br/>api/{tag,media,user,<br/>administrator}/*<br/>api/post/* (legacy post translate)<br/>common::supabase_auth<br/>lib.rs → AppState (legacy)<br/>bin: legacy_bootstrap"]
    end

    subgraph ext["External / Platform"]
        DB[("PostgreSQL + pgvector<br/>(Supabase)")]
        STORE[("Supabase Storage<br/>(S3-compatible)")]
        AUTH[("Supabase GoTrue / JWT")]
        OAI["OpenAI API<br/>(translation + embeddings)"]
    end

    GW --> DI
    GW --> DP
    GW --> AC
    GW --> TH

    DP --> DI
    DP --> TH

    AC --> MIG
    AC --> TH
    AC --> DP

    MIG --> DP

    CMS --> AC
    CMS --> MIG
    CMS --> TH

    DP -- "SeaORM, OpenAI,<br/>pgvector" --> DB
    DP -- "OpenAI / pgvector" --> OAI
    DP -- "SupabaseStorage" --> STORE
    DP -- "SupabaseAuthLayer" --> AUTH
    GW -- "OTLP / tracing" --> OAI
    CMS -- "all integrations<br/>(legacy)" --> DB
    CMS -- "all integrations<br/>(legacy)" --> STORE
    CMS -- "all integrations<br/>(legacy)" --> AUTH
    CMS -- "all integrations<br/>(legacy)" --> OAI

    classDef contract fill:#e6f3ff,stroke:#1f6feb,stroke-width:2px,color:#0b3d91
    classDef domain fill:#e8f5e9,stroke:#2e7d32,stroke-width:2px,color:#1b5e20
    classDef gateway fill:#f3e8ff,stroke:#6f42c1,stroke-width:2px,color:#3a1d63
    classDef shim fill:#fff4e6,stroke:#d97706,stroke-width:1px,color:#7c2d12
    classDef ext fill:#fde7ef,stroke:#bf2c7e,stroke-width:1px,color:#7a1148
    classDef helper fill:#f5f5f5,stroke:#6c757d,stroke-width:1px,color:#495057

    class DI contract
    class DP domain
    class GW gateway
    class AC,MIG,CMS shim
    class TH helper
    class DB,STORE,AUTH,OAI ext
```

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
        MA["Manifest:<br/>vec![<br/>  Box::new(DomainPostService::new())<br/>]"]
        GA --> MA
    end

    subgraph binB["Binary: legacy_bootstrap  (transitional)"]
        direction TB
        LA["apps/api/src/bin/legacy_bootstrap.rs<br/>• env + tracing init<br/>• construct_app_state()<br/>• public_router()<br/>• protected_router()<br/>• protected_administrator_router()<br/>• bind listener"]
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
        PR4["GET  /graphql/immutable  (playground)<br/>POST /graphql/immutable  (handler)"]
        PR5["GET  /graphql/mutable    (playground)<br/>POST /graphql/mutable    (handler)"]
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
    end

    GW --> CTX
    CTX --> ORCH
    ORCH --> COMP
    COMP --> SERV
    COMP --> PR1 & PR2 & PR3 & PR4 & PR5
    COMP --> AR1
    MAN --> DPSR1 & DPSR2 & DPSR3 & DPSR4 & DPSR5
    DPSR1 --> PR1
    DPSR2 --> PPR1 & PPR2 & PPR3 & PPR4 & PPR5 & PPR6
    DPSR3 --> PPR7
    DPSR4 --> PPR8
    DPSR5 --> AR1

    classDef domain fill:#e8f5e9,stroke:#2e7d32,stroke-width:2px,color:#1b5e20
    classDef gateway fill:#f3e8ff,stroke:#6f42c1,stroke-width:2px,color:#3a1d63
    classDef route fill:#fff,stroke:#888,stroke-width:1px,color:#333
    classDef shim fill:#fff4e6,stroke:#d97706,stroke-width:1px,color:#7c2d12

    class CTX,ORCH,COMP,SERV gateway
    class PR1,PR2,PR3,PR4,PR5,PPR1,PPR2,PPR3,PPR4,PPR5,PPR6,PPR7,PPR8,AR1 route
    class MAN,DPSR1,DPSR2,DPSR3,DPSR4,DPSR5 domain
```

## 4. Legacy Bootstrap — `legacy_bootstrap`

```mermaid
graph TB
    subgraph lb["apps/api/src/bin/legacy_bootstrap.rs"]
        direction TB
        LSTATE["AppState {<br/>  conn, media_config,<br/>  media_cache, bucket_visibility_cache,<br/>  graphql_immutable_schema,<br/>  graphql_mutable_schema,<br/>  supabase_admin_client<br/>}"]
        LPR["public_router()<br/>+ cors_layer + otel_layers"]
        LPRR["protected_router()<br/>+ auth + cookie + body-limit<br/>+ cors + otel"]
        LPAR["protected_administrator_router()<br/>+ auth + cookie<br/>+ cors + otel"]
        LSERV["axum::serve(listener, app)"]
    end

    subgraph lpublic["Public"]
        LP1["GET /"]
        LP2["GET /health"]
        LP3["GET /healthz"]
        LP4["GET /media/images/{*path}"]
        LP5["GET /media/{*path}"]
        LP6["GET /graphql/immutable<br/>POST /graphql/immutable"]
        LP7["GET /graphql/mutable<br/>POST /graphql/mutable"]
    end

    subgraph lprot["Protected (writer or admin)"]
        LPR2["GET/POST/PUT/DELETE /posts<br/>GET /posts/{post_id}"]
        LPR3["POST /posts/{post_id}/translate<br/>POST /posts/{post_id}/translate/background<br/>GET /posts/{post_id}/translate/jobs/{job_id}<br/>GET /posts/{post_id}/translate/jobs"]
        LPR5["DELETE /tags"]
        LPR6["GET/POST/DELETE /media<br/>GET /media/info/{*path}<br/>DELETE /media/delete/{*path}"]
        LPR7["GET/POST /graphql/mutable"]
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
            DAU["auth.rs<br/>SupabaseAuthLayer<br/>SupabaseAuthConfig<br/>SupabaseClaims<br/>SupabaseToken"]
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

    APIC --> HC
    APID --> HD
    APIM --> HM
    APIR --> HR
    APIT --> HT
    APIJ --> HTT
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
    SVC --> DRES & DAU & DERR & DLA & DPG & DGQ
    MCL --> MM

    classDef domain fill:#e8f5e9,stroke:#2e7d32,stroke-width:2px,color:#1b5e20
    classDef reexport fill:#fff4e6,stroke:#d97706,stroke-width:1px,color:#7c2d12

    class APIC,APID,APIM,APIR,APIT,APIJ,APICC,APII,HC,HD,HM,HR,HT,HV,HTG,HTT,HCA,HAI,DERR,DRES,DAU,DLA,DPG,DGQ,DVS,DEX,DEV,MM,M1,M2,M3,M4,SVC,MCL,OBS,ENT domain
    class ERE reexport
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
    GW->>Auth: validate JWT (writer or admin)
    Auth-->>GW: SupabaseToken Extension
    GW->>Router: dispatch Mount::Protected
    Router->>DP: api_create_post
    DP->>Api: route to api::post::create
    Api->>Handler: PostCreateHandler::handle_create_post(body, actor_email)
    Handler->>TagHelper: tag_helper::create_tags_in_transaction(tags, actor_email, tx)
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
    DP->>Auth: validate JWT
    Auth-->>DP: SupabaseToken Extension
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

> `domain_posts` no longer depends on `application_core::commands::tag::*`. The tag create + read handlers are owned by `domain_posts::handlers::tag_helper::*`.

## 10. What Each Binary Actually Serves Today

| Route prefix | `my-cms-api` (gateway) | `legacy_bootstrap` | `domain_posts` (standalone) |
|---|---|---|---|
| `/`, `/health`, `/healthz` | ✅ | ✅ | ✅ |
| `/graphql/immutable`, `/graphql/mutable` | ✅ | ✅ | ❌ |
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

## 11. Future Staged Cutover

```mermaid
graph TB
    subgraph now["Today (after consolidation)"]
        N1["my-cms-api serves post + categories + AI + translation routes"]
        N2["legacy_bootstrap serves<br/>tags/media/users/administrator"]
    end

    subgraph next["After domain_media, domain_users, domain_administrator extraction"]
        X1["Box::new(DomainMediaService),<br/>Box::new(DomainUsersService),<br/>Box::new(DomainAdministratorService)<br/>appended to gateway::manifest()"]
        X2["legacy_bootstrap loses /media/**, /users/**, /administrator/**<br/>gateway gains those routes"]
    end

    subgraph future["After domain_tags extraction"]
        F1["domain_tags registered in manifest()"]
        F2["legacy_bootstrap deleted"]
        F3["single my-cms-api deployment image<br/>serves ALL routes via DomainService composition"]
    end

    now --> next --> future

    classDef current fill:#fff,stroke:#888,stroke-width:1px
    classDef nextStage fill:#fff4e6,stroke:#d97706,stroke-width:1px,color:#7c2d12
    classDef futureStage fill:#e8f5e9,stroke:#2e7d32,stroke-width:2px,color:#1b5e20

    class N1,N2 current
    class X1,X2 nextStage
    class F1,F2,F3 futureStage
```

See `docs/adding-a-domain.md` for the recipe and `docs/pluggable-domain-refactor.md` for the full architectural overview.