# Backend domain architecture

Narrative: `docs/architecture/software-architecture.md`.

Validate: `apps/api/gateway/src/main.rs`, `apps/api/gateway/Cargo.toml`,
`apps/api/domain_interface/src/lib.rs`, `apps/api/domain_posts/`,
`apps/api/domain_auth/`, `apps/api/domain_media/`, `apps/api/domain_user/`,
`apps/api/domain_seo/`.

## Current shape

My-CMS is one Rust process, `my-cms-api`, from `apps/api/gateway`. The gateway
is a composition root. Feature logic stays in domain crates that implement
`domain_interface::DomainService`.

`gateway::manifest()` registers five services:

| Service | Owns |
| --- | --- |
| `DomainPostService` | Posts, categories, tags, translations, AI catalogue, jobs, GraphQL HTTP |
| `DomainAuthService` | Supabase JWT middleware only. No routes, no migrations, no DB health probe |
| `DomainMediaService` | Storage objects and administrator bucket lifecycle |
| `DomainUserService` | Administrator user CRUD and password reset. Empty migrations |
| `DomainSeoService` | Trusted SEO head assets, admin CRUD, public enabled-asset delivery, its migration |

The gateway loads env, opens one SeaORM pool, builds immutable and mutable
GraphQL schemas once, validates each domain, runs startup health, dispatches
migration descriptors, merges bare routers, then applies cookie and auth layers
once. Domains must not add their own global auth or cookie layers.

Mounts: Public (anyone), Protected (writer or administrator), Administrator
(administrator only). Mutable GraphQL is Protected and MUST accept either
`my-headless-cms-writer` or `my-headless-cms-administrator`. Do not narrow it
to administrator-only.

`/posts/graphql/immutable` is public. `/posts/graphql/mutable` is protected.
Playgrounds point at those paths. Schemas come from `DomainContext`; the post
domain does not rebuild them per request. Legacy `/graphql/...` mounts and
`apps/api/src/api/graphql` are retired.

## Invariants

- WHEN the gateway starts, THEN it refuses to bind if required configuration
  or startup health fails, and errors MUST NOT leak secrets.
- WHEN `my-cms-api migrate <verb>` runs, THEN migrations dispatch through the
  domain descriptors and the HTTP listener is not bound. Default invocation
  serves HTTP.
- WHEN a migration fails, THEN later migrations in that run do not continue.
- Post migration identities and order stay owned by `domain_posts`. There is
  no `migration` crate and no `application_core` dependency.
- Business domains read the caller as `domain_interface::AuthenticatedActor`.
  They do not depend on `domain_auth` JWT types.
- Adapters extract and serialize. Command handlers own rules. The gateway does
  not import domain entities or handlers.
- Tag create, read, and delete helpers live in
  `domain_posts::handlers::tag_helper`. Do not add a parallel tag crate or
  restore `application_core` tag commands.
- The Docker image ships `my-cms-api` only. Per-domain bins are not a
  deployment surface. The auth stub binary is not an API.

## Cross-cutting trace (gateway)

WHEN `ENABLED_OTLP_EXPORTER=true` and exporter configuration is valid, THEN
the gateway initializes the existing OTLP pipeline before serving, keeps the
exporter guard for the process lifetime, and applies OpenTelemetry layers to
the merged router so public, protected, administrator, health, and
auth-rejected requests can emit an export-eligible HTTP trace with operation
and outcome.

WHEN the flag is false, absent, or invalid, THEN the gateway uses plain-text
logging, does not require a collector, and keeps routes, status codes, and
auth decisions unchanged.

Auth, CORS, cookies, and body limits stay on the merged router. Missing
credentials on a protected route are HTTP 401 and the domain handler is not
called. A valid token without an allowed role is HTTP 403. Role match is
case-sensitive OR. An empty required-role list does not enforce roles.

## Extending

Add a domain only when it owns routes, state, migrations, or health as a
boundary. Register it in the manifest. Do not put feature code in the gateway.
A shared pool is not shared table ownership. Cross-domain transactions need an
explicit design in an AI-DLC intent, not a new OpenSpec change.
