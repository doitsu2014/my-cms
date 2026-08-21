## Context

`apps/ducth-dev-website` is an SSR React site whose production Express server currently serves health, images, favicon, and the SPA/SSR catch-all. `robots.txt` is a public asset and existing uncommitted server work already serves it before the catch-all. Public posts and categories are available through the unauthenticated GraphQL endpoint used by the website, while admin routes and unpublished posts must remain outside crawler discovery.

## Goals / Non-Goals

**Goals:**

- Make `/robots.txt` and `/sitemap.xml` reliable production endpoints.
- Keep the sitemap current with published CMS content without a build or deployment step for every content edit.
- Include both supported locales and use the configured `WEBSITE_SITE_URL` as the canonical origin.
- Keep discovery available when GraphQL is temporarily unavailable.

**Non-Goals:**

- Changing GraphQL pagination or content visibility rules.
- Adding CMS-managed SEO fields, image/video/news sitemap extensions, or admin sitemap output.
- Adding an AI-specific crawler allowlist; robots directives remain the site owner's policy.

## Decisions

### 1. Generate the sitemap at the production server boundary

Add a small server-only ES module that owns static route definitions, the minimal GraphQL query, published-content filtering, URL construction, and XML serialization. Express handles `/sitemap.xml` before the SSR route. A static sitemap file alone is rejected because it would become stale as posts and categories change.

### 2. Use a baseline-first fallback

Every sitemap response starts with the localized static routes. The server attempts one bounded GraphQL request and adds valid published post/category routes when the response is usable. Failures return the baseline rather than a 5xx or an upstream error body. Results are cached briefly in memory to protect the API from repeated crawler requests.

### 3. Derive routes from existing public fields

The sitemap uses `slug`, `published`, `lastModifiedAt`, and category translation slugs already exposed by the website's public GraphQL query. Post slugs are shared between locales by the existing router; category slugs use a locale translation when available and otherwise fall back to the base slug. Only top-level blog categories are included.

### 4. Keep robots policy source-controlled and runtime-aware

Extend `public/robots.txt` with a sitemap marker/default. The production route reads the built file and replaces or appends the sitemap declaration with an absolute URL from `WEBSITE_SITE_URL`, preserving the current allow/disallow rules. This keeps local/static builds understandable while satisfying crawler expectations in deployed environments.

### 5. Test pure serialization and real HTTP behavior

Unit tests cover URL normalization, filtering, deduplication, fallback, and XML escaping. The existing production-server test starts the real server and verifies both crawler endpoints bypass GraphQL/SSR, return correct media types, and expose the expected discovery content.

## Risks / Trade-offs

- [Large content sets make sitemap responses expensive] → Use a compact query, short in-memory caching, and a bounded result shape; a sitemap index/pagination can be added later if the content volume requires it.
- [GraphQL response shape changes] → Validate arrays defensively and retain static-route fallback instead of failing the endpoint.
- [A stale cached sitemap delays discovery] → Keep the cache lifetime short and send a matching short public cache lifetime.
- [Relative or malformed site URL produces invalid loc values] → Reuse the server's required URL validation and normalize the origin before joining paths.

## Migration Plan

1. Add the sitemap module and pure tests.
2. Add the production routes and update the robots asset.
3. Copy the server module into the runtime image and run targeted website tests/build.
4. Deploy normally; no data migration is needed.
5. Roll back by reverting the route/module and robots changes; existing HTML rendering remains unchanged.

## Open Questions

- If the public content volume exceeds one sitemap's practical URL limit, introduce a sitemap index and paginated child documents in a follow-up.
