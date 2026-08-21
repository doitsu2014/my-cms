## Why

The public website has a crawler policy, but it does not publish a sitemap that helps search engines and AI crawlers discover localized reader pages and published CMS content. The existing production server also needs to serve both crawler resources before the SPA fallback so crawler requests never trigger SSR or depend on JavaScript.

## What Changes

- Add a production `sitemap.xml` endpoint containing localized static routes, published post routes, and published category routes.
- Add a sitemap declaration to `robots.txt` while preserving the existing crawler policy.
- Serve crawler resources with the correct content types, caching headers, and graceful fallback behavior when the public GraphQL API is unavailable.
- Add focused tests for XML escaping, published-content filtering, route coverage, and production HTTP responses.

## Capabilities

### New Capabilities

- `website-crawler-discovery`: Defines the public sitemap and robots discovery contract for search engines and AI crawlers.

### Modified Capabilities

- None.

## Impact

- Public website server: `apps/ducth-dev-website/server.prod.mjs` and a small sitemap serialization/fetch module.
- Public static assets: `apps/ducth-dev-website/public/robots.txt`.
- Production image: copy the sitemap server module into the runtime image.
- Website unit and production-server tests. No GraphQL schema, database, authentication, or dependency changes are required.
