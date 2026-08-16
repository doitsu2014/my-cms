## Why

The public website and the administrator application do not currently own document titles or page metadata.  The public site's server-side rendering path returns a document with no route-specific title, description, canonical URL, or social metadata, while the admin remains at the build-tool default title across all routes.  This produces incorrect browser labels, weak search/social previews, and a risk that authenticated administration routes are indexed.

## What Changes

- Introduce a shared, declarative metadata policy for every public website route, including localized titles and descriptions, canonical URLs, Open Graph and Twitter tags, and an explicit robots policy.
- Make public-page metadata part of the SSR document response as well as client-side route transitions, so crawlers and users receive consistent metadata before JavaScript executes.
- Derive article and category metadata from the existing localized GraphQL content, with safe fallbacks when optional excerpts, images, or translations are unavailable.
- Introduce an admin metadata policy that gives every admin route a meaningful title and marks the application `noindex, nofollow`; it must not publish public-site SEO or social tags.
- Add focused metadata tests that assert the rendered head and route-transition behavior, preventing future pages from silently falling back to an incorrect title or metadata set.

## Capabilities

### New Capabilities

- `website-and-admin-metadata`: Defines consistent, route-aware document-title and metadata behavior for the public website and the private administration application.

### Modified Capabilities

- None.

## Impact

- Public website: `apps/ducth-dev-website/src/index.server.tsx`, `src/server.prod.mjs`, route/page components, runtime/site configuration, and tests.
- Admin application: `apps/web/src/App.tsx`, application/runtime configuration as needed, and tests.
- No API, GraphQL schema, database, authentication, or deployment contract changes are expected. A small client-side metadata dependency may be introduced only if it is necessary to keep SSR and browser navigation in one tested implementation.
