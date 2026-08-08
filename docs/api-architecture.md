> **Phase A and migration cleanup completed:** `apps/api/application_core/`, the legacy `apps/api/src/**` runtime, and `apps/api/migration/` were removed by [`purge-legacy-cms-and-application-core`](../openspec/changes/purge-legacy-cms-and-application-core/). Canonical migrations remain in `domain_posts`; operators use `domain_posts migrate up`, and `test_helpers` imports `domain_posts::migrations` directly.

 (Implemented + In-Progress)

This document captures the **as-built** state of the API architecture after `refactor-api-into-pluggable-domain-libraries`, `consolidate-category-ai-translate-into-domain-posts`, `merge-graphql-into-posts-domain`, `migrate-legacy-to-domain-posts`, and `split-media-and-user-domains-merge-tags-into-posts`. It is the visual companion to `pluggable-domain-refactor.md`.

> **Current state:** `legacy_bootstrap`, `cms`, `application_core`, and `migration` have been removed. The remaining domain-adapter work is to register media and user services in `gateway::manifest()`; until that follow-up lands, the gateway serves the route surface shown in §10.

> **Note on retired legacy crates:** `application_core`, `migration`, `cms`, and `legacy_bootstrap` are no longer workspace members or runtime artifacts. Post, AI translation, vector-store, pgvector, media, user, and tag business logic remains in the domain crates. The operator migration workflow is `domain_posts migrate up`; canonical migration identities live under `domain_posts::migrations`.

## 1. Cargo Workspace

```mermaid
graph LR
    subgraph ws["apps/api/  Cargo workspace"]
        DI["<b>domain_interface</b><br/>(publishable contract)<br/>DomainService trait<br/>DomainContext<br/>Mount, RouteRegistration<br/>HealthDescriptor<br/>MigrationDescriptor<br/>DomainConfigError<br/>AuthenticatedActor (actor value type)"]
        DA["<b>domain_auth</b><br/>cross-cutting infrastructure crate<br/>SupabaseAuthLayer,<br/>SupabaseAuthConfig,<br/>SupabaseClaims, SupabaseToken<br/>auth_layer_from_env<br/>DomainAuthService impl<br/>(empty routes, default startup_health,<br/>no sea-orm, no business deps)"]
        DP["<b>domain_posts</b><br/>lib + bin + canonical migrations<br/>api/{post,category,ai}/* (HTTP adapters)<br/>handlers/{post,tag_helper,<br/>category,ai,vector_store,<br/>translation_jobs}/* (commands)<br/>domain/{response,error,<br/>layers,graphql,postgres}<br/>entities/* (canonical)<br/>migrations/* (4 identities)<br/>migrations_cli.rs<br/>service.rs (DomainPostService)"]
        DM["<b>domain_media</b><br/>lib<br/>handlers/{bucket,create,<br/>delete,list,read,<br/>supabase_storage}/*<br/>api/{media/{read,list,create,<br/>delete},bucket/{create,list,<br/>get,update,delete,empty}}<br/>/*_handler.rs (HTTP adapters)<br/>api/{state.rs,routes.rs,mod.rs}<br/>service.rs (DomainMediaService)<br/>domain/{error,extensions,<br/>response (ApiResponseWith/<br/>Error, ErrorCode,<br/>AxumResponse)}<br/>entities/media (re-export)<br/>observability"]
        DU["<b>domain_user</b><br/>lib<br/>dto.rs (AppUserModel,<br/>BAN_DURATION,<br/>is_recognised_role)<br/>handlers/{create,modify,<br/>read_one,read_list,delete,<br/>reset_password,<br/>supabase_admin_client}/*<br/>domain::error<br/>observability"]
        GW["<b>gateway</b><br/>bin: my-cms-api<br/>manifest() → Box&lt;dyn<br/>DomainService&gt;<br/>current composition:<br/>DomainPostService<br/>+ DomainAuthService<br/>compose_routers<br/>(applies domain_auth layer to<br/>protected + administrator)"]
        TH["<b>test_helpers</b><br/>testcontainers + Postgres +<br/>pgvector<br/>imports canonical<br/>domain_posts::migrations"]
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
    GW --> DM
    GW --> DU
    GW --> TH

    DP --> DI
    DP --> TH

    DM --> DI
    DM --> TH

    DU --> DI
    DU --> TH

    DA --> DI

    DP -- "SeaORM, OpenAI,<br/>pgvector" --> DB
    DP -- "OpenAI / pgvector" --> OAI
    DP -- "SupabaseStorage" --> STORE
    DM -- "SupabaseStorage" --> STORE
    DU -- "SupabaseAdminClient" --> AUTH
     DA -- "SupabaseAuthLayer<br/>(auth_layer_from_env)" --> AUTH
     GW -- "OTLP / tracing" --> OAI


    classDef contract fill:#e6f3ff,stroke:#1f6feb,stroke-width:2px,color:#0b3d91
    classDef domain fill:#e8f5e9,stroke:#2e7d32,stroke-width:2px,color:#1b5e20
    classDef infra fill:#fff7e6,stroke:#b07000,stroke-width:2px,color:#6b4500
    classDef gateway fill:#f3e8ff,stroke:#6f42c1,stroke-width:2px,color:#3a1d63
    classDef ext fill:#fde7ef,stroke:#bf2c7e,stroke-width:1px,color:#7a1148
    classDef helper fill:#f5f5f5,stroke:#6c7575,stroke-width:1px,color:#495057

    class DI contract
    class DP,DM,DU domain
    class DA infra
    class GW gateway
    class TH helper
    class DB,STORE,AUTH,OAI ext
```

**Workspace member legend:**
- **Pluggable domain architecture:** `domain_interface`, `domain_auth`, `domain_posts`, `domain_media`, `domain_user`, `gateway`, `test_helpers`.
- **Canonical migrations and operator CLI:** `domain_posts::migrations` and `domain_posts migrate`.
- **Retired:** `cms`, `legacy_bootstrap`, `application_core`, and `migration` are no longer workspace members or runtime artifacts.

## 2. Deployment Modes — API and Migration Binaries

The workspace produces the gateway API binary and the domain-owned migration/operator binary:

```mermaid
graph TB
    subgraph traefik["Traefik / Listener"]
        LB["Reverse Proxy<br/>(routes by path prefix)"]
    end

    subgraph binA["Binary: my-cms-api  (gateway composition)"]
        direction TB
        GA["gateway/src/main.rs<br/>• env + tracing init<br/>• connect_database()<br/>• run_orchestrator()<br/>• build schemas<br/>• compose_routers()<br/>• bind listener"]
        MA["Manifest (target after relocate-legacy-api-adapters-to-domains):<br/>vec![<br/>  Box::new(DomainPostService::new()),<br/>  Box::new(DomainAuthService::new()),<br/>  Box::new(DomainMediaService::new(...)),<br/>  Box::new(DomainUserService::new(...)),<br/>  // + gateway-owned /administrator/database/migration Mount::Administrator route<br/>]"]
        GA --> MA
    end

    subgraph binB["Binary: domain_posts  (operator migration CLI)"]
        direction TB
        LA["apps/api/domain_posts/src/main.rs<br/>• domain_posts migrate up<br/>• domain_posts migrate --list<br/>• canonical migrations from<br/>domain_posts::migrations"]
    end

    subgraph dp["domain_posts (standalone)"]
        DA["domain_posts/src/main.rs<br/>• env + tracing init<br/>• connect_database()<br/>• build schemas<br/>• build DomainPostService<br/>• register_routes(&ctx)<br/>• bind listener"]
    end

    LB --> binA
    LB --> binB
    DA -. "operator migration command<br/>(domain_posts migrate up)" .-> DB

    classDef gateway fill:#f3e8ff,stroke:#6f42c1,stroke-width:2px,color:#3a1d63
    classDef domain fill:#e8f5e9,stroke:#2e7d32,stroke-width:2px,color:#1b5e20
    classDef shim fill:#fff4e6,stroke:#d97706,stroke-width:1px,color:#7c2d12
    classDef lb fill:#f5f5f5,stroke:#6c757d,stroke-width:1px,color:#495057

    class binA gateway
    class dp domain
    class binB domain
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
        PR6["GET  /media/images/{*path}"]
        PR7["GET  /media/{*path}"]
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
        PPR9["DELETE /tags"]
        PPR10["GET/POST/DELETE /media<br/>GET /media/info/{*path}<br/>DELETE /media/delete/{*path}"]
    end

    subgraph administrator_routes["Administrator Router (admin auth)"]
        AR1["POST /administrator/database/migration<br/>(gateway-owned adapter)"]
        AR2["GET/POST /users<br/>GET/PUT/DELETE /users/{user_id}<br/>POST /users/{user_id}/reset-password"]
        AR3["GET/POST /media/buckets<br/>GET/PUT/DELETE /media/buckets/{name}<br/>POST /media/buckets/{name}/empty"]
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

    subgraph dms["domain_media::api::routes() (target)"]
        DMSR1["RouteRegistration { mount: Public, prefix: /media }"]
        DMSR2["RouteRegistration { mount: Protected, prefix: /media }"]
        DMSR3["RouteRegistration { mount: Administrator, prefix: /media/buckets }"]
    end

    subgraph dus["domain_user::api::routes() (planned)"]
        DUSR1["RouteRegistration { mount: Administrator, prefix: /users }"]
    end

    GW --> CTX
    CTX --> ORCH
    ORCH --> COMP
    COMP --> SERV
    COMP --> PR1 & PR2 & PR3 & PR4 & PR5 & PR6 & PR7
    COMP --> PPR1 & PPR2 & PPR3 & PPR4 & PPR5 & PPR6 & PPR7 & PPR8 & PPR9 & PPR10
    COMP --> AR1 & AR2 & AR3
    MAN --> DPSR1 & DPSR2 & DPSR3 & DPSR4 & DPSR5 & DPSR6 & DPSR7
    MAN --> AUTH["domain_auth::DomainAuthService<br/>(registered; empty routes,<br/>validate_config only)"]
    MAN --> DMSR1 & DMSR2 & DMSR3
    MAN --> DUSR1
    DPSR1 --> PR1
    DPSR2 --> PPR1 & PPR2 & PPR3 & PPR4 & PPR5 & PPR6
    DPSR3 --> PPR7
    DPSR4 --> PPR8
    DPSR5 --> AR1
    DPSR6 --> PR4
    DPSR7 --> PR5
    DMSR1 --> PR6 & PR7
    DMSR2 --> PPR10
    DMSR3 --> AR3
    DUSR1 --> AR2
    AR1 -. "gateway-owned<br/>(no domain)" .-> AR1

    classDef domain fill:#e8f5e9,stroke:#2e7d32,stroke-width:2px,color:#1b5e20
    classDef infra fill:#fff7e6,stroke:#b07000,stroke-width:2px,color:#6b4500
    classDef gateway fill:#f3e8ff,stroke:#6f42c1,stroke-width:2px,color:#3a1d63
    classDef route fill:#fff,stroke:#888,stroke-width:1px,color:#333
    classDef shim fill:#fff4e6,stroke:#d97706,stroke-width:1px,color:#7c2d12

    class CTX,ORCH,COMP,SERV gateway
    class PR1,PR2,PR3,PR4,PR5,PR6,PR7,PPR1,PPR2,PPR3,PPR4,PPR5,PPR6,PPR7,PPR8,PPR9,PPR10,AR1,AR2,AR3 route
    class MAN,DPSR1,DPSR2,DPSR3,DPSR4,DPSR5,DPSR6,DPSR7,DMSR1,DMSR2,DMSR3,DUSR1 domain
    class AUTH infra
```

## 4. Retired Legacy Runtime

The historical diagram below documents the retired topology only; it is not part of the current workspace or deployment.

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

    DAU["<b>domain_auth</b><br/>cross-cutting Supabase JWT layer<br/>SupabaseAuthLayer<br/>SupabaseAuthConfig<br/>SupabaseClaims<br/>SupabaseToken<br/>auth_layer_from_env<br/>DomainAuthService impl<br/>(empty routes, default startup_health)"]

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
    subgraph before["Historical (cross-domain import via application_core)"]
        direction TB
        PCH_OLD["PostCreateHandler<br/>(application_core::commands::post::create)"]
        TCH_OLD["TagCreateHandler<br/>(application_core::commands::tag::create)"]
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

    classDef historical fill:#fff4e6,stroke:#d97706,stroke-width:1px,color:#7c2d12
    classDef good fill:#e8f5e9,stroke:#2e7d32,stroke-width:2px,color:#1b5e20

    class PCH_OLD,TCH_OLD historical
    class PCH_NEW,TH_NEW,THR_NEW good
```

> `domain_posts` no longer depends on `application_core::commands::tag::*`. The tag create + read handlers are owned by `domain_posts::handlers::tag_helper::*`. The `application_core` references in this diagram are **historical** (pre-`refactor-api-into-pluggable-domain-libraries`); the `application_core` crate was retired by [`purge-legacy-cms-and-application-core`](../openspec/changes/purge-legacy-cms-and-application-core/) (see §11).

## 10. What Each Binary Actually Serves Today

| Route prefix | `my-cms-api` (gateway) | `domain_posts` (standalone) |
|---|---|---|
| `/`, `/health`, `/healthz` | ✅ | ✅ |
| `/posts/graphql/immutable`, `/posts/graphql/mutable` | ✅ | ✅ |
| `/posts/**`, `/posts/{post_id}` | ✅ | ✅ |
| `/posts/{post_id}/translate{,/background}` | ✅ | ✅ |
| `/posts/{post_id}/translate/jobs{,/**}` | ✅ | ✅ |
| `/categories/**` | ✅ | ✅ |
| `/ai/models` | ✅ | ✅ |
| `/tags` | 🟡 | ❌ |
| `/media/**`, `/media/buckets/**`, `/media/info/**`, `/media/delete/**`, `/media/images/**` | 🟡 | ❌ |
| `/users/**` | 🟡 | ❌ |
| `/administrator/database/migration` | 🟡 | ❌ |
| `domain_posts migrate up` | operator CLI | operator CLI |

> 🟡 = domain adapters exist but the corresponding service is not yet registered in `gateway::manifest()`. No legacy runtime serves these routes.


> **Note (`merge-graphql-into-posts-domain`):** The GraphQL HTTP surface moved from `/graphql/{immutable,mutable}` to `/posts/graphql/{immutable,mutable}`. The mutable mount now accepts the Supabase app roles `my-headless-cms-writer` and `my-headless-cms-administrator` (the gateway's pre-change administrator-only gate was widened to writer + administrator so all three deployment modes expose identical authorization behaviour). The post domain is the sole owner of the GraphQL playground handlers and the `Arc<Schema>` wiring.

> **Historical note (`split-media-and-user-domains-merge-tags-into-posts`):** media, user, and tag business logic was extracted into domain crates. Their gateway registration remains follow-up work; the former `apps/api/src/api/**` adapters and `legacy_bootstrap` runtime were deleted.

## 11. Retired Compatibility Crates

`application_core` and `migration` have been deleted. Their former responsibilities now live in the domain crates:

- canonical SeaORM entities and migrations: `apps/api/domain_posts/src/entities/` and `apps/api/domain_posts/src/migrations/`;
- operator migration CLI: `apps/api/domain_posts/src/main.rs` via `domain_posts migrate`;
- test migration access: `apps/api/test_helpers/src/lib.rs` imports `domain_posts::migrations` directly.

No compatibility re-export or standalone legacy migration crate remains.

## 12. Domain Cutover — In Progress

The legacy runtime purge is complete. The remaining follow-up is registering the already-extracted media and user services in the gateway; migration ownership and operator execution are already domain-owned.

```mermaid

graph TB
    subgraph now["Today (post split-media-and-user-domains-merge-tags-into-posts)"]
        N1["my-cms-api serves post + categories + AI + translation routes"]
        N2["media and user routes await gateway service registration"]
    end

    subgraph done["Completed extractions"]
        D1["✅ domain_media extracted<br/>(apps/api/domain_media/)"]
        D2["✅ domain_users extracted<br/>(apps/api/domain_user/)"]
        D3["✅ tags merged into domain_posts::handlers::tag_helper"]
    end

    subgraph inprog["In progress — relocate-legacy-api-adapters-to-domains  (1/21 tasks done)"]
        I1["✅ domain_media/src/api/{media,bucket}/** HTTP adapters + DomainMediaService written; cargo check -p domain_media --all-targets exits 0"]
        I2["⏳ Mirror structure for domain_user (api/user/** adapters + DomainUserService) — task group 3"]
        I3["⏳ Register DomainMediaService + DomainUserService in gateway::manifest() — task 4.1"]
        I4["⏳ Add gateway-owned /administrator/database/migration Mount::Administrator adapter — task 4.2"]
        I5["⏳ Middleware/auth/role/CORS/body-limit/telemetry parity tests — task 4.3"]
        I6["⏳ Full route/contract parity matrix (release gate) — task 4.4"]
        I1 --> I2 --> I3 --> I4 --> I5 --> I6
    end

    subgraph future["Next — complete domain registration"]
        F1["Register DomainMediaService and DomainUserService"]
        F2["Add route parity and authorization tests"]
        F3["Single my-cms-api deployment image serves all registered routes"]
        F1 --> F2 --> F3
    end

    now --> inprog
    inprog --> future
    done -. "completed in split-media-and-user-domains-merge-tags-into-posts" .- now

    classDef current fill:#fff,stroke:#888,stroke-width:1px
    classDef completed fill:#e8f5e9,stroke:#2e7d32,stroke-width:1px,color:#1b5e20
    classDef nextStage fill:#fff4e6,stroke:#d97706,stroke-width:1px,color:#7c2d12
    classDef futureStage fill:#e8f5e9,stroke:#2e7d32,stroke-width:2px,color:#1b5e20

    class N1,N2 current
    class D1,D2,D3 completed
    class I1,I2,I3,I4,I5,I6 nextStage
    class F1,F2,F3,F4 futureStage
```

> **Remaining work:** Register `DomainMediaService` and `DomainUserService` in `gateway::manifest()`, then add route, authorization, and deployment parity tests. The migration CLI is already `domain_posts migrate up`; no legacy migration crate remains.


See `docs/adding-a-domain.md` for the recipe and `docs/pluggable-domain-refactor.md` for the full architectural overview.
