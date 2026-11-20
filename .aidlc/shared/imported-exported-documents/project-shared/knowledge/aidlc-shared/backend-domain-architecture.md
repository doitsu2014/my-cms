# Backend domain architecture

[Glossary](glossary.md) · Maintained narrative: [`docs/architecture/software-architecture.md`](../../../../../docs/architecture/software-architecture.md)

## Evidence boundary

This contract records intended boundaries and behavior noted in shared knowledge, with partial static scan support. Source-of-truth paths for validation are `apps/api/gateway/`, `apps/api/domain_interface/`, and `apps/api/domain_{posts,auth,media,user,seo}/`. The CodeKB scan is partial; statements below are not a claim that every path or behavior was re-inspected for this rewrite. Treat specific behavior not supported by inspected source as **Unverified** until verified.

## Process and ownership

- The shipped API is the `my-cms-api` binary, composed by `apps/api/gateway`; do not restore a standalone posts/auth binary, `application_core`, or separate migration crate. Post migrations remain owned by `domain_posts`.
- The gateway is the composition root. Domain crates implement `domain_interface::DomainService`; domain logic, entities, handlers, and migrations remain with their owning domain. A shared database pool does not confer shared table ownership.
- The intended manifest contains five domains: `domain_posts`, `domain_auth`, `domain_media`, `domain_user`, and `domain_seo`. `domain_auth` supplies authentication middleware, not an alternate API deployment.
- Gateway-level configuration, shared pool and GraphQL schema construction, migration dispatch, route merge, shared middleware, and listener startup belong at the gateway boundary. The exact current ordering and failure behavior require source confirmation (**Unverified** outside the partial scan).

| Domain | Contract responsibility |
|---|---|
| `domain_posts` | Posts, categories, tags, translations, jobs, GraphQL contribution and post-owned migrations. |
| `domain_auth` | Supabase JWT validation and role middleware. |
| `domain_media` | Media objects and bucket lifecycle. |
| `domain_user` | Administrator user operations through GoTrue. |
| `domain_seo` | SEO head-asset persistence, administration and public delivery. |

## Route and authorization boundary

The intended mount classes are **Public**, **Protected** (writer or administrator), and **Administrator**. Shared auth/cookie layers are applied at gateway composition; domains register bare routers. Posts GraphQL paths are `/posts/graphql/immutable` (public) and `/posts/graphql/mutable` (protected). Mutable GraphQL must allow either `my-headless-cms-writer` or `my-headless-cms-administrator`, not administrator only. Route registration and exact enforcement details are **Unverified** pending deep source validation.

Missing credentials on a protected route are intended to be rejected before its handler; a valid token lacking a permitted role is forbidden. Role matching, empty-role behavior, health-route policy, CORS, cookie, body-limit, and error mapping details remain **Unverified** where not confirmed in the current source scan.

## Migration and extension rules

`my-cms-api migrate <verb>` is the documented migration entry point. Migration identity and ordering are domain-owned; a migration failure should not be treated as successful completion. Exact supported verbs, rollback characteristics, and failure sequencing are **Unverified** pending source inspection. Add a domain only for a coherent ownership boundary and register it through the gateway contract. Cross-domain transactions require an explicit design.

## Observability

Gateway OTLP behavior, including enablement by `ENABLED_OTLP_EXPORTER`, exporter configuration, request coverage, and fallback behavior, is carried as intended contract from existing knowledge but is **Unverified** by the partial scan. Validate gateway configuration and middleware before relying on these details. Do not infer runtime export success from configuration alone.

## Open evidence items

- **Unverified:** exact gateway startup sequence, route/middleware coverage, status/error mappings, migration verbs and failure semantics, and current OTLP behavior.
- **Unresolved:** any disagreement between this intended boundary and current source must be recorded with both claims and paths; a project owner must decide contract changes not settled by source.
