## 1. Sitemap generation

- [x] 1.1 Add a server-only sitemap module that builds localized static URLs, fetches published public posts/categories with a bounded GraphQL request, filters/deduplicates entries, and emits escaped XML with a static-route fallback.
- [x] 1.2 Add unit tests for sitemap URL construction, locale/category translation handling, unpublished filtering, malformed upstream data, XML escaping, and fallback behavior.

## 2. Production crawler endpoints

- [x] 2.1 Update the production website server to serve `/sitemap.xml` and runtime-aware `/robots.txt` before static/SSR fallback with correct media types and short cache headers.
- [x] 2.2 Update the robots asset and production Docker image so the sitemap directive and server module are present in deployed output.
- [x] 2.3 Extend production-server tests to verify both crawler endpoints, SSR bypass, GraphQL failure fallback, sitemap content, and response headers.

## 3. Verification

- [x] 3.1 Run focused website tests, lint/typecheck/build, inspect the diff, and verify OpenSpec status reports all implementation tasks complete.
