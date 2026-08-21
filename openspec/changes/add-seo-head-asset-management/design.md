## Context

Ducth currently derives typed title, canonical, robots, and social markup in `apps/ducth-dev-website/src/metadata/` and inserts `rendered.metadataHead` plus runtime config in `apps/ducth-dev-website/server.prod.mjs`. There is no CMS-managed injection seam. The server reads only process configuration and currently turns an SSR failure into a 500 response.

The API gateway composes four `DomainService` implementations in `apps/api/gateway/src/main.rs`: posts, auth, media, and user. `DomainService` supplies route registrations, migration descriptors, validation, and startup health. `DomainUserService` demonstrates an administrator-only mount (`apps/api/domain_user/src/api/routes.rs`); its thin Axum adapters delegate to trait-backed handlers and return the domain response envelope. `DomainPostService` owns SeaORM migration descriptors and is the current canonical migration precedent, although its generated entities still reflect the retired application-core layout. The gateway migration CLI invokes the same manifest and orchestrator.

The CMS SPA wires admin routes in `apps/web/src/App.tsx`, guards them with `AdminOnlyRoute`, hides privileged navigation in `apps/web/src/app/admin/components/left-menu.tsx`, and calls REST through `authenticatedFetch(getApiUrl(...))`. Its user-management pages demonstrate the expected DaisyUI form/dialog loading and destructive-action conventions.

**Graph evidence limitation:** `get_minimal_context(task="add-seo-head-asset-management")` returned a low 0.40 risk, historical communities `entities-handle`, `components-handle`, and `config-theme`, and flows `App`, `RichTextEditorWrapper`, and `main`; its graph was built on `refactor/my-cms` at `d133a9c`, not current HEAD `6927e3f`. It therefore cannot establish current consumers. This design uses targeted current-source search and inspection as the authoritative fallback, including the gateway manifest/route loop, domain user/media route patterns, SSR template replacement, production-server tests, app routes, and admin REST helpers.

## Goals / Non-Goals

**Goals:**

- Introduce a layered `domain_seo` that owns Ducth-global, administrator-managed head assets and its SeaORM schema/migrations.
- Publish only enabled validated assets to Ducth SSR with predictable, bounded cache/availability behavior.
- Preserve typed metadata as the sole owner of document title, canonical, robots, Open Graph, and Twitter tags.
- Make the dangerous trust boundary conspicuous through administrator-only access, server validation, audit events, and inert administration display.

**Non-Goals:**

- Visitor consent, regional privacy controls, tag-manager protocols, external analytics configuration, or secrets management.
- Any public browser fetch, client-side injection, route/content/locale/user targeting, body/footer injection, live preview, GraphQL exposure, or Seaography registration.
- Manual edits to generated SeaORM entities, a backfill, or changes to sitemap/typed metadata behavior.

## Architecture and contracts

```text
CMS admin /admin/seo/head-assets
  -> administrator router /seo/head-assets (JWT role)
  -> domain_seo Axum adapter -> trait-backed handlers -> SeaORM / seo_head_assets
                                                               |
                                                GET /seo/head-assets/ducth-dev (public)
                                                               |
                 Ducth server-only cache + <=2s fetch -> SSR <head> insertion
```

### Data model and migration

`domain_seo` owns an ordered global table `seo_head_assets`:

| Column | Contract |
| --- | --- |
| `id` | application-generated UUID primary key |
| `label` | nonblank, trimmed, unique text; maximum 128 UTF-8 characters |
| `html` | validated exact UTF-8 source; nonblank; maximum 32 KiB |
| `enabled` | non-null boolean, default true |
| `sort_order` | non-null positive integer |
| `row_version` | non-null integer, default 1, incremented on successful update |
| `created_at`, `updated_at` | non-null UTC timestamps |
| `created_by`, `updated_by` | non-null administrator UUIDs from `AuthenticatedActor` |

The migration adds the primary key, a unique label constraint, a positive-order check, and a partial/public query index on enabled assets in `(sort_order, id)` order. The `down` migration drops this isolated table and index; it intentionally discards head-asset data, which is acceptable only for deployment rollback before operators make indispensable changes. No backfill is needed. The implementer applies the migration through the gateway migration orchestrator, then runs SeaORM entity generation into `apps/api/domain_seo/src/entities/`; those generated files are reviewed but never hand-authored. The new migration descriptor has a globally unique `domain_seo/...` identity and no dependency on post-domain history.

### API contract

The new crate follows `domain_media`/`domain_user` ownership: API extraction and HTTP mapping under `domain_seo/src/api`, request validation and durable behavior under trait-backed `handlers/head_assets`, domain DTO/error/response types under `domain_seo/src/domain`, persistence entities/migrations owned by the crate, and `DomainSeoService` in `service.rs`.

| Endpoint | Mount / audience | Response |
| --- | --- | --- |
| `GET /seo/head-assets` | Administrator / administrator JWT | 200 ordered admin collection |
| `GET /seo/head-assets/{id}` | Administrator / administrator JWT | 200 asset; 404 missing |
| `POST /seo/head-assets` | Administrator / administrator JWT | 201 saved asset; 400 invalid; 409 label conflict |
| `PUT /seo/head-assets/{id}` | Administrator / administrator JWT | 200 saved asset; 404 missing; 409 label conflict or stale version |
| `DELETE /seo/head-assets/{id}` | Administrator / administrator JWT | 204; 404 missing |
| `GET /seo/head-assets/ducth-dev` | Public / none | 200 enabled public collection |

All successful API bodies use the established `ApiResponseWith` envelope and camelCase model fields. The public DTO deliberately excludes `enabled`, audit actor identifiers, and `rowVersion`. The management model includes them. Missing/invalid authentication is 401; a valid non-admin is 403 from existing gateway auth. Handler validation returns `AppError::Validation`/400, unique-label violations return conflict/409, a conditional update affecting no row returns optimistic-concurrency/409, absence returns not-found/404, and database failures use the established safe internal mapping. Create, update, and delete write one row each; update is `WHERE id = ? AND row_version = ?`, updates all editable values and `updated_*`, then increments once. No multi-row transaction is necessary; the conditional statement is the concurrency boundary.

### Trusted source validation and rendering boundary

Server validation is the security control, not React form validation. A parser-backed validator accepts only the exact source grammar in the capability spec: `script`, `meta`, and `link`; allowed attributes; HTTPS URLs; safe script-content rule; restricted metadata ownership; and 32 KiB maximum. It rejects rather than sanitizes to prevent surprising publication of modified code. It must not execute JavaScript or fetch third-party URLs. A parser/tokenizer that can ensure every node is in the allowed set is selected over regex validation; regex cannot safely prove document structure. Unit cases include the requested `gtag.js`, JSON-LD, verification meta, malformed nesting, closing-head/body markup, all forbidden tags/attributes, URL schemes, duplicate attributes, and source-size boundaries.

The raw value is deliberately inserted unchanged only by server-side `server.prod.mjs`, after the API has validated it. The UI renders it through controlled form value/text nodes only. The SSR output therefore trusts the protected storage boundary while typed metadata remains independently escaped/serialized by its existing owner. The new source must never be logged, attached to spans, returned from the public failure path, or placed in an exception message.

### Ducth SSR delivery cache

Add a required server-only `WEBSITE_SEO_HEAD_ASSETS_API_URL` validated with the existing `requiredUrl` helper but omit it from the browser `CONFIG`/`app-config` payload. Create a small module, e.g. `head-assets.mjs`, injected with `fetch`, clock, timeout, and logger seams. It parses only the public envelope/schema, retains a 60-second fresh cache, coalesces refreshes, and enforces a 2-second abort timeout. It inserts the joined ordered `html` values immediately before `</head>` beside—not inside—`rendered.metadataHead` and `app-config`.

If no usable cache exists, any fetch/parse/non-200 failure produces an empty head-asset fragment and normal SSR continues. A last known good value may be used after fresh expiry for no longer than five minutes, with a warning; after that it is discarded. This creates an explicitly bounded consistency window: a healthy change is visible on the first cache refresh, at most 60 seconds per SSR process. A deletion can remain during that same fresh/stale window if the public API is unreachable; recovery availability wins over immediate global removal. Operators needing immediate removal must restore public API reachability or restart affected SSR processes after disabling the asset.

## Decisions

### 1. New domain service and isolated migration ownership

**Decision:** Add `domain_seo` as the fifth service in the gateway manifest, with its own crate, migration descriptors, routes, handlers, error/response model, and generated entities.

**Why:** The capability owns new durable state and both public/admin APIs; placing it in posts or gateway would violate current service boundaries and make future SEO evolution coupled to content or runtime composition.

**Alternatives rejected:** A JSON runtime setting lacks lifecycle/audit/concurrency controls. Adding columns to posts makes global integrations a content-domain concern. Writing manually composed SQL in the gateway skips the migration/service interfaces.

**Consequences:** Gateway workspace/manifest and migration CLI composition change; `DomainSeoService` needs database access from `DomainContext` and a startup `SELECT 1` health probe like posts. No extra service key or external integration is needed.

### 2. Whole-asset optimistic concurrency

**Decision:** Use the existing `row_version` pattern, require it on `PUT`, and reject mismatches as a 409 optimistic-concurrency error.

**Why:** Raw executable source is high-impact and accidental overwrites are worse than an edit retry. Whole-resource replacement keeps the public set atomic.

**Alternatives rejected:** Last-write-wins loses concurrent safety. ETags add a second version contract without improving this single-row admin API. Partial updates make it unclear which raw-source version is authoritative.

### 3. Restricted trusted markup, not arbitrary HTML or a tag-manager model

**Decision:** Persist exact source only after strict parser-backed allow-list validation and administrator authorization.

**Why:** Scripts are intentionally required for gtag and JSON-LD, but arbitrary markup can corrupt the document, bypass the typed metadata policy, or introduce a broader XSS surface.

**Alternatives rejected:** Fully arbitrary HTML violates the head/document boundary. Sanitizing untrusted HTML falsely suggests it is safe and can mutate third-party code. Typed Analytics-only fields cannot support verification/structured-data assets.

**Consequences:** Some third-party snippets will be rejected until deliberately added to the allow-list; this is a security review event, not a client-side workaround. A future consent feature must own the decision to conditionally invoke a stored asset.

### 4. Server-only public read plus bounded SSR cache

**Decision:** Ducth's Node SSR process calls a public API endpoint, caches success 60 seconds, allows stale success at most five minutes on failure, and otherwise renders without dynamic assets.

**Why:** It lets changes publish without a web deployment while preventing public-page latency/availability from being coupled to an API call. The SSR path is necessary for crawler-visible tags.

**Alternatives rejected:** Fetching in React misses crawlers and enables browser-side access. Fetching every request adds latency and outages. Deployment-time environment snippets require redeployment. An indefinite stale cache makes disable/delete operationally unsafe.

**Consequences:** Multiple SSR replicas refresh independently and have eventual, rather than instantaneous, consistency. `WEBSITE_SEO_HEAD_ASSETS_API_URL` is deployment configuration, not a secret, but must be a reachable TLS API endpoint.

### 5. Admin UI duplicates server validation but never previews source

**Decision:** Add an admin-only `/admin/seo/head-assets` list/create/edit flow using React Hook Form + Zod for immediate feedback and server errors as authority; source uses a textarea and delete uses confirmation.

**Why:** Operators need usable lifecycle control without a code-execution surface.

**Alternatives rejected:** A WYSIWYG/editor preview can execute markup. A writer-visible screen conflicts with the API privilege boundary. A raw JSON console is less accessible and loses familiar status/error states.

## Security, privacy, and operations

- This grants administrators the ability to run code for every visitor. Audit events include only action, IDs, actor, enabled state, and order—not HTML. The audit fields are durable for attribution but are not an immutable, separate audit-log table.
- Analytics can collect visitor information without consent in v1. The site owner must approve each enabled integration and ensure legal/privacy obligations before enabling it. Authors must never put credentials, tokens, or personal data in markup.
- The public API has no sensitive fields, but its response remains an executable supply-chain input. Ducth validates response shape, uses HTTPS config, limits request time, and avoids source in logs.
- Add `#[instrument]` to API handlers/commands with source fields skipped. Emit info for successful admin state changes and warning/error for validation, database, fetch, parse, timeout, and stale-cache conditions; use correlation/tracing context already provided by gateway/website telemetry.
- No public GraphQL/Seaography registration, cookie, CSP, CORS policy, or rate-limit change is part of v1. Before a stricter CSP rollout, review whether inline source needs nonce/hash support; do not silently relax CSP for this feature.

## Migration, rollout, and rollback

1. Build and test the crate/migration and apply it through `my-cms-api migrate up` in the target environment. Confirm the service reports healthy and the public endpoint returns an empty list before enabling an asset.
2. Deploy gateway with `domain_seo`; existing Ducth versions continue operating because no endpoint is called yet.
3. Configure and deploy Ducth with `WEBSITE_SEO_HEAD_ASSETS_API_URL`; verify an empty public response leaves the existing head unchanged.
4. Create an initially disabled test asset, confirm admin controls/audit, then enable a reviewed Google tag or verification asset. Verify rendered HTML and cache-refresh telemetry from each replica within 60 seconds.
5. Roll back by disabling offending assets first (if public API is healthy), then roll back Ducth to remove fetch/injection, and gateway only if necessary. The migration down path destroys stored assets and is therefore a last-resort rollback with export/operator acknowledgement.

## Risks / Trade-offs

- [Administrator account compromise executes third-party code] → Existing admin role only, durable actor attribution, no preview, restricted grammar, and operational approval.
- [Third-party script harms performance/privacy/availability] → Explicit site-owner acceptance, HTTPS-only source, sorted independent disable control, cache timeout/fail-open, and no silent retries per page.
- [API outage delays removal] → Fresh and stale bounds are documented; stale data has warning telemetry and operators can restore API/restart SSR.
- [Strict allow-list excludes a legitimate integration] → Reject safely; extend only through a reviewed spec/design change and regression tests.
- [Migration/schema-first drift] → Run gateway orchestrator and regenerate entities before code is considered complete; no generated-file hand edits.
- [Existing graph is stale] → SE repeats graph impact checks after implementation task groups and falls back to source/test review if graph remains stale.

## Open Questions

- No blocking product decision remains for v1. The proposal records that enabling tracking without consent is the site owner's legal/privacy decision.
- Confirm operational deployment manifests/secret stores that will supply the new server-only endpoint before release; the repository does not reveal the live deployment configuration.
