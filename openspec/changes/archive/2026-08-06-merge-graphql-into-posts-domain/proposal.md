## Why

The `apps/api/src/api/graphql/` module is a thin, mis-scoped adapter that
sits outside every domain boundary. It owns only the two GraphiQL
playground HTML handlers (`graphql_mutable`, `graphql_immutable`) while
the actual Seaography-backed POST endpoints are wired inline in
`apps/api/src/bin/legacy_bootstrap.rs` and `apps/api/gateway/src/main.rs`.
The schema-builder lives in two parallel locations:
`apps/api/application_core/src/graphql/query_root::schema` (legacy) and
`apps/api/domain_posts/src/domain/graphql::contribute_post_schema` (new).
Routing exposes the GraphQL surface at the top level (`/graphql/immutable`,
`/graphql/mutable`) instead of under the resource it actually serves —
posts — making the API surface harder to reason about, harder to
deprecate per-resource, and inconsistent with the per-domain ownership
the rest of `domain_posts` already enforces. This change folds the
GraphQL adapter into the posts domain and re-mounts the GraphQL endpoint
under the posts route group so the schema, playground, and HTTP route
all live next to the post handlers.

## What Changes

- Move `apps/api/src/api/graphql/mod.rs` (the two playground handlers)
  into `apps/api/domain_posts/src/api/post/graphql/mod.rs`. The
  handler exposes a thin `playground_immutable()` /
  `playground_mutable()` pair that renders the GraphiQL HTML
  pointing at the **new** route paths. NO parallel copy is created
  in the legacy `cms` tree (`apps/api/src/api/post/graphql/` does
  not exist).
- Delete `apps/api/src/api/graphql/` and remove `pub mod graphql;` from
  `apps/api/src/api/mod.rs`. The module ceases to exist.
- Re-mount the GraphQL endpoint from `/graphql/{immutable,mutable}` to
  `/posts/graphql/{immutable,mutable}` in every router that currently
  exposes it:
  - `apps/api/src/bin/legacy_bootstrap.rs::public_router()` —
    `GET/POST /posts/graphql/immutable` (public).
  - `apps/api/src/bin/legacy_bootstrap.rs::protected_router()` —
    `GET/POST /posts/graphql/mutable` (writer + administrator).
  - `apps/api/gateway/src/main.rs::compose_routers()` —
    `GET/POST /posts/graphql/immutable` (public) and
    `GET/POST /posts/graphql/mutable` (protected, admin only — current
    gateway policy).
  - `apps/api/domain_posts/src/api/mod.rs::routes()` — the post domain
    contributes a `Mount::Public` registration for
    `/posts/graphql/immutable` and a `Mount::Protected` registration for
    `/posts/graphql/mutable` so both gateway and standalone
    `domain_posts` binary serve the new paths.
- Delete the legacy `apps/api/application_core/src/graphql/` module
  (currently a transitional duplicate of `contribute_post_schema`); the
  schema-builder canonical home is `apps/api/domain_posts/src/domain/graphql.rs`.
  `apps/api/application_core/src/lib.rs` drops `pub mod graphql;`.
- Update the frontend default URL surface in
  `apps/web/src/infrastructure/graphQL/graphql-client.ts`,
  `apps/web/src/config/runtime-config.ts`,
  `apps/web/src/config/api.config.ts`,
  `apps/web/.env.example`, and `apps/web/public/config.js` so the default
  `graphqlApiUrl` resolves to `http://localhost:8989/posts/graphql`
  (the `…/immutable` and `…/mutable` segments are appended by the
  GraphiQL playground and by the cache helper). The `PUBLIC_GRAPHQL_API_URL`
  / `PUBLIC_GRAPHQL_CACHE_API_URL` env vars keep working unchanged; only
  the trailing-path default changes.
- Update `apps/web/src/infrastructure/das/categories.das.ts` and any
  other data-access service that hard-codes a GraphQL URL to construct
  the URL relative to the new `graphqlApiUrl` base (no hard-coded
  `/graphql` suffixes remain).

### Migration CLI surface after the change

```text
cargo run -p domain_posts -- migrate --list   # unchanged — 4 migration identities
cargo run -p gateway    -- migrate             # unchanged — runs domain_posts migrations
```

No new migrations are introduced. The four existing migration identities
(`m20240409_151952_release_100`, `m20250330_151455_release_110`,
`m20260126_040610_release_300`, `m20260531_000001_pgvector`) are preserved
exactly. **BREAKING:** the GraphQL HTTP route path changes from
`/graphql/{immutable,mutable}` to `/posts/graphql/{immutable,mutable}`.
The schema itself (operations, types, entities) does not change; only the
URL changes.

### Per-resource decision (resources currently exposed by the standalone graphql module)

The standalone `apps/api/src/api/graphql/mod.rs` exposes **no
data resources** — it only returns the GraphiQL playground HTML. The
data resources exposed via GraphQL are determined by the
`contribute_post_schema` builder, which registers the historical
seven-entity set (`categories`, `category_tags`, `posts`, `post_tags`,
`tags`, `category_translations`, `post_translations`). The decision per
resource:

| Entity                      | Belongs to        | Decision                                                                                          |
|-----------------------------|-------------------|---------------------------------------------------------------------------------------------------|
| `posts`                     | posts             | Move alongside posts.                                                                            |
| `post_tags`                 | posts             | Move alongside posts.                                                                            |
| `post_translations`         | posts             | Move alongside posts.                                                                            |
| `categories`                | posts (per active `consolidate-category-ai-translate-into-domain-posts` change) | Stays registered by `domain_posts::contribute_post_schema` until a `domain_categories` is extracted. |
| `category_tags`             | posts             | Stays registered by `domain_posts::contribute_post_schema`.                                      |
| `category_translations`     | posts             | Stays registered by `domain_posts::contribute_post_schema`.                                      |
| `tags`                      | `application_core` (not yet extracted) | Stays registered by `domain_posts::contribute_post_schema` until a `domain_tags` is extracted. |
| `CategoryType` enum         | posts             | Stays registered by `domain_posts::contribute_post_schema`.                                      |

**No resource stays at the old `/graphql/...` path.** All seven
entities remain queryable through the new `/posts/graphql/...` endpoint;
the file move and route re-mount is the only observable change.

## Capabilities

### New Capabilities

- `posts-graphql-mount`: Owns the GraphQL HTTP surface for the post
  domain. The post domain contributes the Seaography schema, the
  GraphiQL playground handlers, and the Axum route registrations that
  expose `/posts/graphql/immutable` (public) and `/posts/graphql/mutable`
  (protected, gated by the post-domain auth roles). The capability
  subsumes the former top-level `graphql` module; the standalone
  `apps/api/src/api/graphql/` directory ceases to exist.

### Modified Capabilities

- `domain-post-service`: The capability text in
  `openspec/changes/refactor-api-into-pluggable-domain-libraries/specs/domain-post-service/spec.md`
  is updated so the post domain's `register_routes` output now includes
  the `/posts/graphql/**` registrations (both `Mount::Public` and
  `Mount::Protected`). The schema-builder requirement (currently
  satisfied by `contribute_post_schema`) is preserved verbatim. No
  public schema/contract change beyond the URL path.

## Impact

- Affected crates: `apps/api/Cargo.toml`,
  `apps/api/application_core/{Cargo.toml, src/{lib.rs, graphql/**}}`,
  `apps/api/domain_posts/{Cargo.toml, src/{api/mod.rs, api/post/mod.rs, api/post/graphql/**, domain/graphql.rs}}`,
  `apps/api/src/{api/{mod.rs, graphql/**}, bin/legacy_bootstrap.rs, lib.rs}`,
  `apps/api/gateway/src/main.rs`.
- **The legacy `cms` API tree (`apps/api/src/api/post/`) does NOT
  gain a `graphql` sub-module.** The post-domain handlers live
  exclusively under `apps/api/domain_posts/src/api/post/graphql/`,
  and the `legacy_bootstrap` binary imports them directly from
  `domain_posts::api::post::graphql`. Removing the legacy tree in
  the same change (not deferring it) is intentional — it prevents
  two parallel copies from drifting apart.
- Affected routes: `/graphql/immutable` and `/graphql/mutable` move to
  `/posts/graphql/immutable` and `/posts/graphql/mutable` in both the
  gateway and `legacy_bootstrap` binaries. The standalone
  `domain_posts` binary's standalone mode gains the same
  `/posts/graphql/...` paths when this change wires
  `register_routes`. **No other route changes.** Traefik rules
  (`deployments/docker-swarm/traefik/dynamic/my-cms.yml`) continue
  to match because they route by service hostname, not by GraphQL
  path.
- **Behaviour change at the gateway mutable mount:** the role gate
  is widened from administrator-only to writer + administrator to
  align the gateway with the legacy bootstrap's pre-existing role
  gate. This is a policy expansion (writers can now mutate via the
  gateway's GraphQL endpoint, where they previously could not) —
  recorded in the design's Decision 6 and the spec's "Authorization
  boundary at the new mount point" requirement.
- Affected entities: none. The seven entities registered by
  `contribute_post_schema` are unchanged.
- Affected migrations: none. The four migration identities stay where
  they are.
- Affected tests: new tests added under
  `apps/api/domain_posts/src/api/post/graphql/tests.rs` covering the
  playground handler (returns 200 with GraphiQL HTML, embeds the new
  path), the route registration (`Mount::Public` /
  `Mount::Protected` semantics), and the new role-set behaviour
  (writer JWT alone is sufficient, administrator JWT alone is
  sufficient, no-JWT returns 401, role-less JWT returns 403). A
  workspace-level smoke test asserts that
  `apps/api/src/api/graphql/` and `apps/api/src/api/post/graphql/`
  are absent (catches accidental recreation of the deleted trees).
  The existing test gaps for `apps/api/src/api/graphql/mod.rs` and
  `apps/api/domain_posts/src/domain/graphql.rs` (both **0 tests**
  per `code-review-graph_query_graph_tool::tests_for`) are closed.
- Affected frontend: `apps/web/src/infrastructure/graphQL/graphql-client.ts`,
  `apps/web/src/config/runtime-config.ts`,
  `apps/web/src/config/api.config.ts`,
  `apps/web/public/config.js`, and `apps/web/.env.example` get the new
  default URL suffix. The `PUBLIC_GRAPHQL_API_URL` /
  `PUBLIC_GRAPHQL_CACHE_API_URL` env vars keep working (operators can
  pin the old path during a deploy window).
- Affected deployment image: `my-cms-api` (gateway binary) and
  `legacy_bootstrap` binary now mount GraphQL under
  `/posts/graphql/...`; the standalone `domain_posts` binary mirrors
  the same paths. No image rebuild is required beyond the code change.
