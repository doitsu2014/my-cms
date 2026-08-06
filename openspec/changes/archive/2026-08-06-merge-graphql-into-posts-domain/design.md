## Context

The My-CMS API exposes a GraphQL surface alongside its REST CRUD. Today
the GraphQL HTTP wiring is split across four locations and the route
paths sit at the top of the URL hierarchy:

```
apps/api/src/api/graphql/mod.rs                                 17 lines  — playground HTML only
apps/api/src/bin/legacy_bootstrap.rs                            295 lines — wires /graphql/{immutable,mutable}
apps/api/gateway/src/main.rs                                    260 lines — wires /graphql/{immutable,mutable}
apps/api/application_core/src/graphql/query_root.rs             50 lines  — schema builder (legacy duplicate)
apps/api/domain_posts/src/domain/graphql.rs                     77 lines  — schema builder (canonical)
apps/api/domain_posts/src/api/mod.rs                            132 lines — does NOT yet register /graphql/**
apps/api/domain_posts/src/main.rs                               121 lines — builds the two schemas standalone
apps/api/src/lib.rs::AppState                                   38 lines  — owns two Arc<Schema> fields
apps/api/domain_interface/src/lib.rs                            267 lines — owns the same two Arc<Schema> on DomainContext
apps/web/src/infrastructure/graphQL/graphql-client.ts           51 lines  — reads config().graphqlApiUrl
```

The standalone `apps/api/src/api/graphql/` module is **only the
GraphiQL playground HTML**. The GraphQL POST handlers are wired
inline in the routers via
`async_graphql_axum::GraphQL::new(app_state.graphql_*.schema)` — they
do not live in `apps/api/src/api/graphql/`. There are **no other
REST resources** in that module; it is purely a UI helper.

The seven entities exposed via the Seaography schema
(`categories`, `category_tags`, `posts`, `post_tags`, `tags`,
`category_translations`, `post_translations`) are owned by
`domain_posts` (per the active
`consolidate-category-ai-translate-into-domain-posts` change). The
schema-builder therefore lives in `domain_posts` and the only
remaining uses of `application_core::graphql::query_root::schema` are
the `legacy_bootstrap` binary and a transitive `application_core::graphql`
module re-export. The new gateway composition never imports the
legacy builder.

The current route mount is **breaking the per-domain ownership**
principle the rest of the refactor enforces. `/graphql/...` sits at
the URL root while every other post-domain route sits under
`/posts/**` (read, create, modify, delete, translate, jobs). The
GraphQL surface serves the post aggregate, so it belongs under
`/posts/graphql/...` next to its peers.

**Confirmed test gaps** (`code-review-graph_query_graph_tool::tests_for`
on the canonical paths):
- `apps/api/src/api/graphql/mod.rs` — **0 tests**
- `apps/api/domain_posts/src/domain/graphql.rs` — **0 tests**

These gaps will be closed by the new playground-handler tests and the
new route-registration tests under
`apps/api/domain_posts/src/api/post/graphql/tests.rs`.

**Stakeholders:**
- **Content / editorial team**: uses the GraphQL POST surface for
  bulk reads (`{ posts(filter: ...) { nodes { ... } } }`); needs the
  URL change to be communicated through the `PUBLIC_GRAPHQL_API_URL`
  env var.
- **Backend engineering**: needs `cargo check --workspace` to stay
  green; needs the cycle introduced by
  `application_core::graphql::query_root::schema` (now unused) to
  become deletable.
- **Frontend engineering**: the Apollo client in
  `apps/web/src/infrastructure/graphQL/graphql-client.ts` reads the
  base URL from `config().graphqlApiUrl`. The default flips from
  `/graphql` to `/posts/graphql`. Existing deploys that override the
  env var keep working without code changes.

**Constraints:**
- Schema-first migrations only. No new migrations.
- The published `domain_interface` contract stays
  backward-compatible. No new port traits are introduced.
- The Seaography schema's seven-entity registration and the
  `is_mutation_supported` flag are preserved exactly.
- The legacy `legacy_bootstrap` binary keeps working — its
  router rewires to `/posts/graphql/...` but every other route is
  untouched.
- **BREAKING:** the GraphQL HTTP route path changes from
  `/graphql/{immutable,mutable}` to `/posts/graphql/{immutable,mutable}`.
  The schema itself (operations, types, entities) does NOT change.
  Operators can pin the old path during a deploy window by setting
  `PUBLIC_GRAPHQL_API_URL` to the new value or by using a reverse
  proxy rewrite.

## Goals / Non-Goals

**Goals:**
- Make `domain_posts` the single canonical owner of every
  post-related HTTP surface, including the GraphQL HTTP surface.
  After this change `apps/api/src/api/graphql/` does not exist.
- Re-mount the GraphQL endpoints at `/posts/graphql/{immutable,mutable}`
  in every binary that currently exposes them (`legacy_bootstrap`,
  gateway, standalone `domain_posts`).
- Delete the transitional duplicate
  `apps/api/application_core/src/graphql/query_root::schema`. The
  schema-builder lives in exactly one place:
  `apps/api/domain_posts/src/domain/graphql::contribute_post_schema`.
- Preserve the existing Seaography schema (seven entities,
  `CategoryType` enum, mutation toggle, depth/complexity
  `None`-default) bit-for-bit.
- Preserve the existing auth boundary at the new mount point —
  immutable stays public, mutable stays protected with the same
  Supabase JWT role gate.
- Update the frontend default URL surface so a fresh checkout that
  runs `pnpm --dir apps/web build` and the gateway in parallel
  resolves the new path automatically.
- Close the documented test gaps for the playground handlers and
  the schema builder.

**Non-Goals:**
- Splitting `domain_posts` further. Categories and AI stay in
  `domain_posts` (per the active
  `consolidate-category-ai-translate-into-domain-posts` change).
- Adding depth / complexity limiters to the schema (existing
  behaviour is `None` for both). A follow-up change can introduce
  per-environment limits.
- Introducing schema versioning, persisted queries, persisted
  fragments, or schema directives. Pure refactor.
- Removing the `legacy_bootstrap` binary. Categories, AI, translation,
  GraphQL, etc. that the binary still serves keep working; only the
  GraphQL route paths are rewritten.
- Changing the Supabase auth layer, the auth roles, or the role-gate
  policy.
- Publishing `domain_interface` or `domain_posts` to crates.io.
- Introducing port traits in `domain_interface`. The schema-builder
  consumer (the gateway and the `legacy_bootstrap` binary) and the
  provider (`domain_posts::domain::graphql`) are wired directly
  through Rust paths.

## Decisions

### Decision 1 — Mount the GraphQL endpoint at `/posts/graphql/{immutable,mutable}`

**Driver.** Per-domain ownership, route hierarchy alignment with the
rest of the post aggregate (`/posts/**`), and consistency with the
`domain_posts::api::routes(ctx)` registration pattern.

**Current state.** Routes are wired at `/graphql/immutable` (public)
and `/graphql/mutable` (protected) in
`apps/api/src/bin/legacy_bootstrap.rs` (lines 105–109 and 164–168) and
in `apps/api/gateway/src/main.rs` (lines 169–177, inside
`compose_routers`).

**Selected design.** Rewrite both routers so the routes become
`/posts/graphql/immutable` (public) and `/posts/graphql/mutable`
(protected). The `domain_posts::api::routes(ctx)` function gains two
new `RouteRegistration`s:

```rust
RouteRegistration {
    mount: Mount::Public,
    router: Router::new().route(
        "/posts/graphql/immutable",
        get(post::graphql::playground_immutable)
            .post_service(GraphQL::new(ctx.graphql_immutable.as_ref().clone())),
    ),
    prefix: "/posts/graphql",
},
RouteRegistration {
    mount: Mount::Protected,
    router: Router::new().route(
        "/posts/graphql/mutable",
        get(post::graphql::playground_mutable)
            .post_service(GraphQL::new(ctx.graphql_mutable.as_ref().clone())),
    ),
    prefix: "/posts/graphql",
},
```

The router literal re-uses the same `ctx.graphql_*` `Arc<Schema>`
fields that `domain_interface::DomainContext` already exposes (the
gateway already builds them at startup in `gateway::main`). The
standalone `domain_posts` binary already builds both schemas
(`domain_posts::main::main` lines 42–55) and just needs to wire them
into the same router literal via `ctx`.

**Rejected alternative 1.** Mount at a single `/posts/graphql`
endpoint and let auth gate mutations inside the resolver. Rejected
because the existing two-mount-point design (`immutable` public,
`mutable` protected) is what the rest of the system relies on for
read-only caching and for unauthenticated read-only consumers.
Collapsing the two mounts would break the cache helper
(`buildCacheGraphQLClient`) and any public read-only consumer.

**Rejected alternative 2.** Mount at `/posts/graphql/{immutable,mutable}`
but keep the legacy `/graphql/{immutable,mutable}` paths as
compatibility aliases. Rejected because the user request explicitly
calls for the re-route and because leaving aliases in the gateway
would re-introduce the duplication the refactor is removing. A
reverse-proxy rewrite is the documented mitigation for operators
that need a deploy window.

### Decision 2 — Move the playground handlers into the post domain

**Driver.** The `apps/api/src/api/graphql/` module is a 17-line
adapter whose only purpose is to return the GraphiQL HTML. It has
no data dependencies and exists purely to serve the UI. It belongs
next to the other post HTTP adapters.

**Current state.** `apps/api/src/api/graphql/mod.rs` exposes
`graphql_mutable()` and `graphql_immutable()`, each returning
`playground_source(GraphQLPlaygroundConfig::new("/graphql/..."))`.

**Selected design.** Create `apps/api/domain_posts/src/api/post/graphql/mod.rs`
(new tree) with `pub async fn playground_immutable()` and
`pub async fn playground_mutable()`, each pointing at the new path
string (`/posts/graphql/immutable` and `/posts/graphql/mutable`
respectively). The `apps/api/src/api/graphql/` directory and the
`pub mod graphql;` declaration in `apps/api/src/api/mod.rs` are
deleted. **No `apps/api/src/api/post/graphql/` directory is created
in the legacy `cms` tree** — the new tree is the single source of
truth, and the `legacy_bootstrap` binary imports the handlers
directly from `domain_posts::api::post::graphql`. The legacy
`cms::api::post::mod.rs` SHALL NOT declare `pub mod graphql;`. The
post-domain `register_routes` output SHALL be the canonical mount.

**Rejected alternative.** Keep the playground handlers in
`apps/api/src/api/graphql/` and just rename the module to
`apps/api/src/api/playground/`. Rejected because it perpetuates the
misleading "graphql module" abstraction (the data path is wired
inline elsewhere) and the playground is conceptually paired with
the GraphQL POST endpoint that this change moves into the post
domain.

### Decision 3 — Delete the legacy `application_core::graphql` duplicate

**Driver.** `apps/api/application_core/src/graphql/query_root::schema`
is a verbatim duplicate of `apps/api/domain_posts/src/domain/graphql::contribute_post_schema`
(50 lines vs. 77 lines — the difference is the `POST_GRAPHQL_ENTITIES`
constant comment that is now redundant). It is imported only by
`apps/api/src/bin/legacy_bootstrap.rs::construct_app_state`.

**Current state.** `application_core/src/graphql/mod.rs` declares
`pub mod query_root;`. `application_core/src/lib.rs` declares
`pub mod graphql;`. `legacy_bootstrap` imports the schema via
`use application_core::graphql::query_root::schema;`.

**Selected design.** Delete `application_core::src/graphql/` entirely.
Drop `pub mod graphql;` from `application_core::src/lib.rs`. Update
`legacy_bootstrap.rs::construct_app_state` to import from
`use domain_posts::domain::graphql::contribute_post_schema;`. The
`domain_posts::Cargo.toml` already exposes
`domain_posts::domain::graphql`; the dependency direction
`legacy_bootstrap → domain_posts` is acceptable because the
`legacy_bootstrap` binary is already a transitional compatibility
shim that imports from `domain_posts` for the category/AI handlers
introduced by the active
`consolidate-category-ai-translate-into-domain-posts` change.

**Rejected alternative.** Keep the legacy duplicate as a
re-export shim that forwards to
`contribute_post_schema`. Rejected because the legacy duplicate
is a transitive source of confusion (two identical 50-line
functions, one of which is never imported by any new code) and
because deleting it is what makes the new builder the single
canonical owner.

### Decision 4 — `application_core::entities` keeps its re-export shim

**Driver.** `application_core::entities` is a pure re-export shim
that forwards to `domain_posts::entities::*` (per Decision 5 of the
active `consolidate-category-ai-translate-into-domain-posts` change).
The `legacy_bootstrap` binary's `cms::api::{media,user,administrator}::*`
modules still import from `application_core::entities::*`. Deleting
`application_core::graphql` does not affect this shim; the shim
keeps working as-is.

**Current state.** `application_core::entities::mod.rs` is
`pub use domain_posts::entities::*;` (per the active consolidation
change's Task 1.3). `application_core::entities` re-exports posts,
post_tags, post_translations, translation_jobs, tags, categories,
category_tags, category_translations, sea_orm_active_enums.

**Selected design.** No change to `application_core::entities`. The
`cargo tree -p application_core | grep domain_posts` output
continues to show `domain_posts v0.1.0` (forward-only), as it does
after the active consolidation change.

**Rejected alternative.** Move `application_core::entities` into
`domain_posts` and have `application_core` depend on `domain_posts`
directly for the re-export. Rejected because this is exactly what
the active consolidation change's Task 5.3 already established as
the canonical layout; re-doing it here would conflict with the
active change.

### Decision 5 — Frontend default URL flips; env-var override wins

**Driver.** A fresh checkout that runs the gateway on
`http://localhost:8989` and the web app on `localhost:3000` without
setting `PUBLIC_GRAPHQL_API_URL` needs to resolve to the new path.
Existing deploys that already set `PUBLIC_GRAPHQL_API_URL` keep
working.

**Current state.** `apps/web/src/config/runtime-config.ts` defaults
`graphqlApiUrl` to `import.meta.env.PUBLIC_GRAPHQL_API_URL ||
'http://localhost:4000/graphql'`. `apps/web/.env.example` ships
`PUBLIC_GRAPHQL_API_URL=http://localhost:8989/graphql`. The
Apollo client (`apps/web/src/infrastructure/graphQL/graphql-client.ts`)
appends `/immutable` or `/mutable` via the playground's own
auto-detection.

**Selected design.** Update the default and the env-example to the
new path:

```ts
// apps/web/src/config/runtime-config.ts (line ~42)
graphqlApiUrl: import.meta.env.PUBLIC_GRAPHQL_API_URL ||
  'http://localhost:8989/posts/graphql',
```

```dotenv
# apps/web/.env.example (line ~32)
PUBLIC_GRAPHQL_API_URL=http://localhost:8989/posts/graphql
PUBLIC_GRAPHQL_CACHE_API_URL=http://localhost:8989/posts/graphql
```

The frontend's `getGraphqlApiUrl()` helper
(`apps/web/src/config/api.config.ts`) is unchanged. The cache helper
in `apps/web/src/infrastructure/graphQL/graphql-client.ts` keeps
falling back to `graphqlApiUrl` when `graphqlCacheApiUrl` is unset,
matching the existing behaviour.

**Rejected alternative.** Add a runtime check that probes the old
path and falls back to the new path. Rejected because it adds a
runtime round-trip and a permanent compat branch for a one-time
URL change. Operators that need the old path can set
`PUBLIC_GRAPHQL_API_URL` explicitly.

### Decision 6 — Auth-layer ordering widens to writer+administrator at every deployment mode

**Driver.** The legacy `protected_router` applies the Supabase auth
layer with the role set
`["my-headless-cms-writer", "my-headless-cms-administrator"]`
(legacy_bootstrap.rs lines 169–175). The gateway's pre-change
`compose_routers` applies an administrator-only auth layer to the
`/graphql/**` routes. The product owner has decided that the three
deployment modes (gateway, standalone `domain_posts`, `legacy_bootstrap`)
MUST expose identical authorization behaviour at the mutable mount,
so the gateway's role set is widened from administrator-only to
writer + administrator as part of this change. This is a behaviour
change, not a constraint preservation.

**Current state.** Legacy:
- `/graphql/immutable` (public_router) — no auth layer.
- `/graphql/mutable` (protected_router) — writer + administrator.
Gateway:
- `/graphql/immutable` + `/graphql/mutable` (both inside
  `compose_routers::protected` and merged into the admin-protected
  router) — administrator only.
Standalone `domain_posts`:
- The current standalone binary does not yet register `/graphql/**`
  routes (the post domain's `register_routes` was not yet wired with
  GraphQL). The role gate for the future mount is a fresh decision.

**Selected design.** The mutable mount's role gate SHALL be
`["my-headless-cms-writer", "my-headless-cms-administrator"]`
identically in the gateway composition, the standalone `domain_posts`
binary, and the `legacy_bootstrap` binary. The
`apps/api/gateway/src/main.rs::compose_routers` function applies the
Supabase auth layer with the role vector
`vec!["my-headless-cms-writer".to_string(), "my-headless-cms-administrator".to_string()]`
to the `Mount::Protected` GraphQL registrations (in addition to the
existing administrator-only layer applied to administrator-only
routes). The standalone `domain_posts` binary's auth layer is updated
to the same role vector when it wires `Mount::Protected` registrations.
The `legacy_bootstrap` binary's role vector is unchanged (it already
applies writer + administrator). The `Mount::Public` immutable mount
remains public (no auth layer) in all three modes.

**Rejected alternative.** Preserve the gateway's administrator-only
gate and tighten the legacy bootstrap to match. Rejected because it
silently regresses the writer role at the gateway, breaking any
existing writer-typed client that talks to the gateway's GraphQL
endpoint. The PO has explicitly directed the writer+administrator
role set so the three modes are aligned.

**Rejected alternative.** Leave the legacy bootstrap unchanged and
only widen the gateway. Rejected because it produces three different
authorization policies for the same endpoint shape, which is exactly
the inconsistency the PO asked this change to fix.

### Decision 7 — No new GraphQL POST handler in the API tree

**Driver.** The GraphQL POST endpoint is wired via
`async_graphql_axum::GraphQL::new(arc_schema)` which provides a
`post_service` handler. The standalone `apps/api/src/api/graphql/`
module never owned the POST handler; the POST handler lives inline
in the router literal. Keeping that pattern avoids re-introducing
a `apps/api/src/api/post/graphql/post_handler.rs` file that would
be a one-line pass-through.

**Current state.** `legacy_bootstrap.rs` does
`get(api::graphql::graphql_immutable).post_service(GraphQL::new(...))`
on a single route line. There is no separate
`graphql_immutable_post` function.

**Selected design.** The `domain_posts::api::post::graphql::mod.rs`
module exposes only the two playground handlers
(`playground_immutable`, `playground_mutable`). The POST service is
attached at the route literal in `domain_posts::api::mod.rs::routes(ctx)`,
matching the existing pattern.

**Rejected alternative.** Create `apps/api/domain_posts/src/api/post/graphql/post_handlers.rs`
exposing `graphql_immutable_post(Schema)` and `graphql_mutable_post(Schema)`
functions and call them from the route literal. Rejected because
the inline `post_service(GraphQL::new(...))` pattern is what every
other GraphQL mount in the codebase uses; introducing handler
functions would create a new abstraction that exists nowhere else.

## Risks / Trade-offs

- **[Risk — Breaking URL change]** The GraphQL HTTP route changes from
  `/graphql/{immutable,mutable}` to `/posts/graphql/{immutable,mutable}`.
  Any client that hard-codes the old URL breaks. → **Mitigation:** the
  Apollo client defaults to the new path; operators pin
  `PUBLIC_GRAPHQL_API_URL` during a deploy window; a reverse-proxy
  rewrite (`traefik.my-cms.yml` `my-cms-api` service) is documented
  in the deployment README as the recommended rollout. The schema
  itself is unchanged so introspection and operation names carry over.
- **[Risk — Legacy duplicate deletion cascades]** Deleting
  `application_core::graphql::query_root::schema` could surface
  hidden imports in `test_helpers` or in the legacy
  `cms::src/{api,presentation_models,common}::*` modules. → **Mitigation:**
  `rg "application_core::graphql" apps/` was run as part of this
  design and returns only `legacy_bootstrap.rs` (the importer we are
  rewiring) and `application_core::graphql::query_root` itself
  (the importer we are deleting). No other consumer exists. The
  `cargo tree` verification step (`cargo tree -p application_core |
  grep graphql` returns no production-dep result) confirms the
  deletion is self-contained.
- **[Risk — Test-gap regression]** The current `apps/api/src/api/graphql/`
  has zero tests, so we have no regression baseline for the
  playground handlers. → **Mitigation:** the new
  `apps/api/domain_posts/src/api/post/graphql/tests.rs` adds the
  following cases:
  1. `GET /posts/graphql/immutable` returns 200 with `Content-Type:
     text/html; charset=utf-8` and the body contains
     `/posts/graphql/immutable`.
  2. `GET /posts/graphql/mutable` returns 200 with the same shape
     and the body contains `/posts/graphql/mutable`.
  3. `POST /posts/graphql/immutable` accepts a GraphQL introspection
     query and returns 200 with the seven-entity type list.
  4. `POST /posts/graphql/mutable` without a JWT returns 401.
  5. `POST /posts/graphql/mutable` with a writer JWT accepts a
     mutation and returns 200.
- **[Risk — Frontend default URL race]** Operators that pin the
  default in a reverse proxy rather than via the env var could see
  the new path forwarded before they update the proxy. → **Mitigation:**
  Traefik rules route by service hostname (`Host(...)`), not by
  GraphQL path, so no Traefik rule change is required. The
  `my-cms-api` service in `deployments/docker-swarm/traefik/dynamic/my-cms.yml`
  routes all paths under the hostname to the gateway container, and
  the gateway decides internally which mount handles which path.
- **[Risk — Schema duplication recurs]** A future contributor could
  re-introduce a `graphql` module somewhere else (e.g. resurrect
  the legacy `cms::api::graphql::` module or create a parallel
  `apps/api/src/api/post/graphql/` directory). → **Mitigation:**
  `apps/api/domain_posts/src/api/post/graphql/mod.rs` is the
  single canonical home; `apps/api/src/api/post/mod.rs` SHALL NOT
  declare `pub mod graphql;` (asserted by the spec's "Legacy API
  tree does not carry the post-domain graphql handlers" requirement
  and the corresponding `rg` verification step in tasks Phase 1).
  The standalone `domain_posts` binary's role set is also captured
  in a dedicated test, so a future regression that reverts the role
  set to administrator-only would fail the test.

- **[Risk — Role-set behavior change at the gateway mutable mount]** Widening the gateway's mutable mount role gate from administrator-only to writer + administrator is a behavior change visible to existing writer-typed clients that currently talk to the gateway. The change improves consistency with the legacy bootstrap but is a security-policy expansion that must be called out explicitly. → **Mitigation:** the change is gated on PO approval (recorded in the design Open Questions section as resolved); the role set is asserted by a dedicated test (`mutable_mount_accepts_writer_jwt` and `mutable_mount_accepts_administrator_jwt`); the design documents the rejected alternative of preserving the admin-only gate so future contributors understand why the gate is writer+administrator.

## Migration Plan

### Phase 1 — Move the playground handlers (single new-tree home)

1. Create `apps/api/domain_posts/src/api/post/graphql/mod.rs` (the
   single canonical home) with two handlers, `playground_immutable()`
   and `playground_mutable()`, that render the GraphiQL HTML pointing
   at `/posts/graphql/{immutable,mutable}`.
2. Add `pub mod graphql;` to
   `apps/api/domain_posts/src/api/post/mod.rs`.
3. Delete `apps/api/src/api/graphql/` and remove `pub mod graphql;`
   from `apps/api/src/api/mod.rs`. Verify that NO
   `apps/api/src/api/post/graphql/` directory is created and that
   `apps/api/src/api/post/mod.rs` does NOT declare `pub mod graphql;`.
4. Verify: `rg "post/graphql" apps/api/src` returns no results;
   `rg "post/graphql" apps/api` returns only paths under
   `apps/api/domain_posts/src/api/post/graphql/`.
5. Verify: `cargo check --workspace`; `cargo build --workspace`;
   `cargo test --workspace --lib --bins`.

### Phase 2 — Re-mount the routes in the legacy binary

1. Update `apps/api/src/bin/legacy_bootstrap.rs::public_router` to
   route `/posts/graphql/immutable` instead of `/graphql/immutable`.
   Update the handler reference to
   `domain_posts::api::post::graphql::playground_immutable` (NOT a
   legacy `api::post::graphql::*` path — the legacy tree does not
   carry the handler).
2. Update `apps/api/src/bin/legacy_bootstrap.rs::protected_router` to
   route `/posts/graphql/mutable` instead of `/graphql/mutable`. Update
   the handler reference to
   `domain_posts::api::post::graphql::playground_mutable`.
3. Update `apps/api/src/bin/legacy_bootstrap.rs::construct_app_state`
   to import `contribute_post_schema` from
   `domain_posts::domain::graphql` instead of
   `application_core::graphql::query_root`.
4. Verify: `cargo build --bin legacy_bootstrap`; `cargo test -p cms`;
   the legacy binary's auth layer still applies the
   `["my-headless-cms-writer", "my-headless-cms-administrator"]` role
   set (already correct — no change needed).

### Phase 3 — Re-mount the routes in the gateway (widened role set)

1. Update `apps/api/gateway/src/main.rs::compose_routers` so the
   `public` router literal routes `/posts/graphql/immutable` and the
   `protected` router literal routes `/posts/graphql/mutable`
   (instead of `/graphql/...`). The mutable endpoint stays inside
   the `protected` router; the immutable endpoint is split into the
   `public` router literal.
2. **Widen the mutable-mount auth layer** from administrator-only
   to writer + administrator: add
   `domain_posts::api::post::graphql::playground_immutable` /
   `playground_mutable` references where appropriate, and update the
   role vector applied to the GraphQL-protected route so it carries
   BOTH `my-headless-cms-writer` and `my-headless-cms-administrator`.
   The cleanest implementation is to introduce a new auth layer
   bound to the GraphQL registrations that uses
   `vec!["my-headless-cms-writer".to_string(), "my-headless-cms-administrator".to_string()]`,
   leaving the existing administrator-only layer in place for the
   rest of the protected router. Confirm the precise layer
   composition with the SE before implementation; the spec requires
   the writer role to be sufficient on its own at the mutable mount.
3. Update the `graphql_immutable_playground` /
   `graphql_mutable_playground` helpers to point at
   `"/posts/graphql/immutable"` and `"/posts/graphql/mutable"`
   respectively (the GraphiQL config string).
4. Verify: `cargo build -p gateway`; `cargo test -p gateway`;
   `cargo test -p domain_posts` (the dedicated role-set test
   added in Phase 7 covers this behaviour).

### Phase 4 — Register the routes in `domain_posts::api::routes`

1. Update `apps/api/domain_posts/src/api/mod.rs::routes` to add a
   `Mount::Public` `RouteRegistration` for `/posts/graphql/immutable`
   and a `Mount::Protected` `RouteRegistration` for
   `/posts/graphql/mutable`, both reading `ctx.graphql_*` from the
   `DomainContext`.
2. Update `apps/api/domain_posts/src/main.rs::main` to attach the
   `protected_router` from the routes registration to the auth layer
   (the existing `domain_posts::domain::layers::cors_layer` /
   `otel_layers` plumbing is unchanged).
3. Verify: `cargo check -p domain_posts`; `cargo build --bin
   domain_posts`; `cargo test -p domain_posts`.

### Phase 5 — Delete the legacy schema-builder duplicate

1. Delete `apps/api/application_core/src/graphql/mod.rs` and
   `apps/api/application_core/src/graphql/query_root.rs`.
2. Remove `pub mod graphql;` from
   `apps/api/application_core/src/lib.rs`.
3. Verify: `cargo check --workspace`; `cargo tree -p
   application_core | grep graphql` returns no production-dep
   result.

### Phase 6 — Update the frontend default URL surface

1. Update `apps/web/src/config/runtime-config.ts` so
   `graphqlApiUrl` defaults to
   `http://localhost:8989/posts/graphql`.
2. Update `apps/web/.env.example` to ship the new default URL.
3. Update `apps/web/public/config.js` to reflect the new default
   (the file is generated from `apps/web/public/config.js.template`
   at build time, so verify the template is unchanged and update
   the generated file directly).
4. Update `apps/web/src/config/api.config.ts` if it embeds the old
   suffix (per source inspection, it returns
   `config().graphqlApiUrl || 'http://localhost:8989/graphql'` —
   update the fallback to `'http://localhost:8989/posts/graphql'`).
5. Verify: `pnpm --dir apps/web build`; the runtime config dump
   shows the new path.

### Phase 7 — Close the test gaps

1. Add `apps/api/domain_posts/src/api/post/graphql/tests.rs` with
   the five scenarios from the Risks / Trade-offs section.
2. Add a smoke test in
   `apps/api/domain_posts/src/api/post/graphql/tests.rs` for the
   `RouteRegistration` shape (`prefix == "/posts/graphql"`,
   `Mount::Public` for the immutable registration, `Mount::Protected`
   for the mutable registration).
3. Verify: `cargo test -p domain_posts --lib`; the new tests pass
   against a live testcontainer database.

### Phase 8 — End-to-end verification

1. Run the full repository verification gate: `cargo check`,
   `cargo test --workspace --lib --bins`, `cargo fmt -- --check`,
   `cargo clippy --all-targets`, `cargo build --bin legacy_bootstrap`,
   `cargo build --bin my-cms-api`, `cargo build --bin domain_posts`,
   `pnpm --dir apps/web build`.
2. Boot the gateway with a live testcontainer database. Verify:
   - `GET /posts/graphql/immutable` returns 200 with the playground
     HTML embedding `/posts/graphql/immutable`.
   - `POST /posts/graphql/immutable` with an introspection query
     returns 200 with the seven-entity type list.
   - `GET /posts/graphql/mutable` returns 200 with the playground
     HTML embedding `/posts/graphql/mutable`.
   - `POST /posts/graphql/mutable` without a JWT returns 401.
   - `POST /posts/graphql/mutable` with an admin JWT and a sample
     mutation returns 200.
   - `GET /graphql/immutable` and `GET /graphql/mutable` return 404.
3. Boot `cargo run -p domain_posts` standalone. Verify the same
   five checks succeed.
4. Run `openspec verify --change "merge-graphql-into-posts-domain"`
   and resolve every CRITICAL finding.
5. Run `openspec sync --change "merge-graphql-into-posts-domain"` to
   publish the new `posts-graphql-mount` spec into `openspec/specs/`
   and update the modified `domain-post-service` spec.
6. Run `openspec archive "merge-graphql-into-posts-domain"` after
   the sync step succeeds (Phase 4 approval — owner: product-owner).

### Rollback strategy

Each phase is independently revertible:
- **Phase 1** — restore `apps/api/src/api/graphql/` from the deleted
  commit (this change never creates `apps/api/src/api/post/graphql/`;
  there is nothing in the new tree to remove as part of the
  rollback).
- **Phases 2–4** — revert the route path string changes; the
  `post_service(GraphQL::new(...))` wiring is unchanged.
- **Phase 5** — restore `apps/api/application_core/src/graphql/`
  and re-add `pub mod graphql;` to
  `application_core::src/lib.rs`.
- **Phase 6** — revert the URL default in
  `apps/web/src/config/runtime-config.ts` and `.env.example`.
- **Phases 7–8** — the new tests are additive and only fail if the
  change is partially applied. A revert restores the previous
  test-passing state.
- No database migration is added. Rollback does not require a
  database rollback.

## Open Questions

- **Should the playground HTML body be served with `Cache-Control:
  no-store`?** The current `/graphql/...` endpoints do not set
  this header. Out of scope for this change; a follow-up can add
  it if the GraphiQL playground starts embedding sensitive
  operation suggestions.
- **Should the `POST_GRAPHQL_ENTITIES` constant in
  `apps/api/domain_posts/src/domain/graphql.rs` stay as a
  documentation comment, or move to a `tests::expected_entities`
  test fixture so the entity list is asserted on every test run?**
  Recommend the test-fixture form (catches accidental entity
  additions/removals) — confirm before implementation.

### Resolved by the product owner

The following questions were raised in the initial draft of this
design and have been resolved by the product owner before the
artifacts were finalised. The resolutions are recorded here so the
rationale is preserved in the change history:

1. **Gateway mutable-mount role set** — **RESOLVED: widen from
   administrator-only to writer + administrator** to align with the
   legacy bootstrap's pre-existing role gate. The three deployment
   modes (gateway, standalone `domain_posts`, `legacy_bootstrap`)
   MUST expose identical authorization behaviour at the mutable
   mount. See Decision 6 and the spec's "Authorization boundary at
   the new mount point" requirement. The implementation is captured
   in tasks Phase 3 (gateway) and Phase 4 (standalone binary).
2. **Legacy tree (`apps/api/src/api/post/graphql/`) fate** —
   **RESOLVED: delete entirely.** The legacy tree MUST NOT carry a
   parallel copy of the playground handlers and MUST NOT be a
   re-export shim. The new tree
   (`apps/api/domain_posts/src/api/post/graphql/mod.rs`) is the
   single source of truth, and the `legacy_bootstrap` binary
   imports the handlers directly from `domain_posts::api::post::graphql`.
   See Decision 2 and the spec's "Legacy API tree does not carry
   the post-domain graphql handlers" requirement.
