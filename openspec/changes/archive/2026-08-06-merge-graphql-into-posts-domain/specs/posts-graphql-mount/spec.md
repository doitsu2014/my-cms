## ADDED Requirements

### Requirement: Post domain owns the GraphQL HTTP surface

The post domain SHALL own the GraphQL HTTP surface end-to-end. The
`apps/api/src/api/graphql/` module SHALL NOT exist. The post domain
SHALL expose:

- Two GraphiQL playground HTML handlers (`playground_immutable`,
  `playground_mutable`) under the post domain's new API tree
  (`apps/api/domain_posts/src/api/post/graphql/mod.rs`), each pointing
  at the post domain's own GraphQL endpoint. The legacy `cms` API
  tree SHALL NOT carry a parallel copy of these handlers — the
  post-domain new tree is the single source of truth, and the
  `legacy_bootstrap` binary SHALL import the handlers from
  `domain_posts::api::post::graphql` directly.
- Two GraphQL POST handlers that delegate to the `Arc<Schema>` provided
  by `domain_posts::domain::graphql::contribute_post_schema`, mounted
  under `/posts/graphql/{immutable,mutable}`.
- The schema-builder function
  `domain_posts::domain::graphql::contribute_post_schema` SHALL remain
  the canonical owner of the Seaography schema; no other module SHALL
  re-implement it.

#### Scenario: Standalone `graphql` module is removed
- **WHEN** the change is merged and the workspace compiles
- **THEN** no `apps/api/src/api/graphql/` directory exists
- **AND** `apps/api/src/api/mod.rs` does NOT declare `pub mod graphql`
- **AND** no `apps/api/src/api/post/graphql/` directory exists (the
  legacy tree does NOT carry a parallel copy)
- **AND** `apps/api/src/api/post/mod.rs` does NOT declare
  `pub mod graphql`
- **AND** every `cargo build --workspace` invocation succeeds

#### Scenario: Playground handler points at the new route path
- **WHEN** an unauthenticated client sends `GET /posts/graphql/immutable`
- **THEN** the response body is the GraphiQL HTML playground
- **AND** the embedded playground URL is `"/posts/graphql/immutable"`
  (NOT `"/graphql/immutable"`)
- **AND** the response status is 200 with `Content-Type: text/html; charset=utf-8`

#### Scenario: Playground handler for the mutable endpoint
- **WHEN** an authenticated client (with the `my-headless-cms-writer` or
  `my-headless-cms-administrator` app role) sends
  `GET /posts/graphql/mutable`
- **THEN** the response body is the GraphiQL HTML playground
- **AND** the embedded playground URL is `"/posts/graphql/mutable"`

#### Scenario: Legacy `/graphql/...` paths are no longer served
- **WHEN** any client sends `GET /graphql/immutable` or
  `GET /graphql/mutable`
- **THEN** the gateway returns 404 (or 405 if a method is recognised)
- **AND** the `legacy_bootstrap` binary returns 404

### Requirement: Post domain registers GraphQL routes via `DomainService`

The post domain's `DomainPostService::register_routes(ctx)` SHALL return
a `Mount::Public` `RouteRegistration` for `/posts/graphql/immutable`
and a `Mount::Protected` `RouteRegistration` for `/posts/graphql/mutable`.
The `RouteRegistration::prefix` SHALL be `"/posts/graphql"`. The
`graphql_immutable` and `graphql_mutable` `Arc<Schema>` values SHALL be
read from the `DomainContext` provided by the gateway — the post domain
SHALL NOT rebuild the schema per-request.

#### Scenario: Gateway serves both immutable and mutable endpoints
- **WHEN** the gateway composition runs
- **THEN** `GET /posts/graphql/immutable` returns 200 with the playground
  HTML (no auth required)
- **AND** `POST /posts/graphql/immutable` accepts a GraphQL query body
  and returns the same shape that the legacy `/graphql/immutable` returned
  (same entity set, same field names, same errors)
- **AND** `GET /posts/graphql/mutable` requires either the
  `my-headless-cms-writer` or the `my-headless-cms-administrator` app
  role (the gateway's role gate is widened from administrator-only to
  writer + administrator as part of this change)
- **AND** `POST /posts/graphql/mutable` accepts mutation bodies when
  the caller has either of the required roles

#### Scenario: Standalone `domain_posts` binary serves the new paths
- **WHEN** `cargo run -p domain_posts` boots with a live database
- **THEN** the standalone binary registers the same `/posts/graphql/**`
  routes
- **AND** both playground handlers return 200 with the GraphiQL HTML
- **AND** both POST endpoints accept GraphQL bodies and return the same
  schema-defined types as the gateway
- **AND** the mutable mount enforces the writer + administrator role
  gate identically to the gateway

#### Scenario: Legacy bootstrap binary serves the new paths
- **WHEN** `cargo run --bin legacy_bootstrap` boots with a live database
- **THEN** the legacy binary registers the same `/posts/graphql/**`
  routes
- **AND** the immutable mount is public (no auth required)
- **AND** the mutable mount enforces the writer + administrator role
  gate identically to the gateway

### Requirement: Seaography schema content is preserved verbatim

`domain_posts::domain::graphql::contribute_post_schema` SHALL continue
to register the historical seven-entity set (`categories`,
`category_tags`, `posts`, `post_tags`, `tags`, `category_translations`,
`post_translations`) plus the `CategoryType` enum, with no field-level
changes. The `is_mutation_supported` flag SHALL continue to gate the
mutable schema's mutation surface. The depth/complexity limiters SHALL
remain `None` (unlimited) until a follow-up change introduces them.

#### Scenario: Schema introspection enumerates the historical entities
- **WHEN** an authenticated client sends an introspection query
  (`{ __schema { types { name } } }`) to `/posts/graphql/mutable`
- **THEN** the response enumerates the seven historical entities plus
  `CategoryType`
- **AND** no entity is added or removed compared to the pre-change
  schema served at `/graphql/mutable`

#### Scenario: Mutable schema includes mutations, immutable does not
- **WHEN** the gateway builds both schemas
- **THEN** `graphql_immutable` exposes no `Mutation` type
- **AND** `graphql_mutable` exposes the full mutation set produced by
  Seaography for the seven entities

#### Scenario: Schema construction is idempotent
- **WHEN** `contribute_post_schema` is called twice with identical
  arguments
- **THEN** both calls return `Ok(Schema)` and the schemas expose the
  same types, queries, and mutations

### Requirement: Authorization boundary at the new mount point

The immutable endpoint (`/posts/graphql/immutable`) SHALL be served
without any auth layer. The mutable endpoint (`/posts/graphql/mutable`)
SHALL be served behind the Supabase JWT auth layer with the role gate
`["my-headless-cms-writer", "my-headless-cms-administrator"]`. This
role set SHALL be applied identically by the gateway composition, the
standalone `domain_posts` binary, and the `legacy_bootstrap` binary —
the legacy bootstrap's pre-existing writer+admin gate is preserved,
and the gateway's pre-existing administrator-only gate is widened to
writer+administrator as part of this change so the three deployment
modes expose identical authorization behaviour at the mutable mount.

#### Scenario: Unauthenticated immutable query succeeds
- **WHEN** an unauthenticated client sends
  `POST /posts/graphql/immutable` with a valid GraphQL query body
- **THEN** the response is 200 with the query result (no auth required)

#### Scenario: Unauthenticated mutable query is rejected
- **WHEN** an unauthenticated client sends
  `POST /posts/graphql/mutable`
- **THEN** the response is 401 (or the equivalent auth-middleware error
  envelope)
- **AND** the GraphQL handler is never invoked

#### Scenario: Authenticated mutable query with the wrong role is rejected
- **WHEN** a client with a valid Supabase JWT that lacks BOTH the
  `my-headless-cms-writer` and the `my-headless-cms-administrator`
  roles sends `POST /posts/graphql/mutable`
- **THEN** the response is 403 (or the equivalent forbidden envelope)

#### Scenario: Writer JWT alone is sufficient on the mutable mount
- **WHEN** a client with a valid Supabase JWT carrying the
  `my-headless-cms-writer` role (and no other app role) sends
  `POST /posts/graphql/mutable` with a valid GraphQL body
- **THEN** the response is 200 with the query result
- **AND** the same behaviour is observed regardless of which binary
  serves the request (gateway composition, standalone
  `domain_posts`, or `legacy_bootstrap`)

#### Scenario: Administrator JWT alone is sufficient on the mutable mount
- **WHEN** a client with a valid Supabase JWT carrying the
  `my-headless-cms-administrator` role (and no other app role) sends
  `POST /posts/graphql/mutable` with a valid GraphQL body
- **THEN** the response is 200 with the query result

### Requirement: Frontend GraphQL clients resolve to the new base URL

`apps/web/src/infrastructure/graphQL/graphql-client.ts` SHALL read
`config().graphqlApiUrl` and default it to
`http://localhost:8989/posts/graphql` (replacing the previous default
`http://localhost:4000/graphql` / `http://localhost:8989/graphql`). The
`PUBLIC_GRAPHQL_API_URL` / `PUBLIC_GRAPHQL_CACHE_API_URL` env vars
SHALL continue to override the default. `apps/web/.env.example` and
`apps/web/public/config.js` SHALL be updated to reflect the new default.

#### Scenario: Default `graphqlApiUrl` resolves to the new base
- **WHEN** the web app is started without `PUBLIC_GRAPHQL_API_URL` set
- **THEN** `getGraphqlApiUrl()` returns the URL string ending in
  `/posts/graphql`
- **AND** an Apollo client built with that URL issues requests to
  `/posts/graphql/immutable` or `/posts/graphql/mutable` per the
  playground's own auto-detection

#### Scenario: Override env var wins over the default
- **WHEN** `PUBLIC_GRAPHQL_API_URL=http://localhost:8989/posts/graphql`
  is set in the web app env
- **THEN** `getGraphqlApiUrl()` returns that URL verbatim
- **AND** no `/graphql` suffix is appended

### Requirement: Legacy schema-builder duplicate is removed

The `apps/api/application_core/src/graphql/query_root::schema` function and the `apps/api/application_core/src/graphql/` module SHALL be deleted. `apps/api/application_core/src/lib.rs` SHALL NOT declare `pub mod graphql;`. The `apps/api/Cargo.toml` SHALL NOT depend on the `application_core::graphql` symbol. `domain_posts::domain::graphql::contribute_post_schema` SHALL be the only schema-builder in the workspace.

#### Scenario: Compiling the workspace no longer references `application_core::graphql`
- **WHEN** `cargo build --workspace` runs after the change
- **THEN** no symbol under `application_core::graphql::` is referenced
  by any other crate
- **AND** `cargo tree -p application_core | grep graphql` returns no
  production-dependency result

#### Scenario: The `legacy_bootstrap` binary imports the schema from the new home
- **WHEN** `legacy_bootstrap` builds the two `Arc<Schema>` instances
- **THEN** it imports `domain_posts::domain::graphql::contribute_post_schema`
  (NOT `application_core::graphql::query_root::schema`)
- **AND** the resulting schemas expose the same entities as before

### Requirement: Legacy API tree does not carry the post-domain graphql handlers

The legacy `apps/api/src/api/post/` directory SHALL NOT declare
`pub mod graphql;` and SHALL NOT contain an
`apps/api/src/api/post/graphql/` sub-directory after this change. The
`legacy_bootstrap` binary SHALL import the playground handlers directly
from `domain_posts::api::post::graphql::{playground_immutable, playground_mutable}`.
No re-export shim is permitted — the new tree is the single source of
truth.

#### Scenario: Source tree contains no `post/graphql` outside the new domain crate
- **WHEN** the change is merged
- **THEN** `rg "post/graphql" apps/api/src` returns no results
- **AND** `rg "post/graphql" apps/api` returns only paths under
  `apps/api/domain_posts/src/api/post/graphql/`

#### Scenario: Legacy bootstrap imports from the canonical new tree
- **WHEN** `legacy_bootstrap.rs::public_router` and
  `legacy_bootstrap.rs::protected_router` reference the playground
  handlers
- **THEN** they import the handlers from
  `domain_posts::api::post::graphql`
- **AND** no `use api::post::graphql::...` declaration remains in
  `legacy_bootstrap.rs`
