# Public site, design, and SEO

Validate: `apps/ducth-dev-website/`, `apps/web` document head,
`apps/api/domain_seo/`, `packages/editor-prose/`.

Locales are `en` and `vi` on `/:lang/...`. The server owns runtime config.
The browser reads `window.__APP_CONFIG__` only. SSR injects that JSON
without breaking out of the script. SSR errors do not leak internals.
Media URL joins use exactly one slash. Public GraphQL is the immutable
posts endpoint. Missing translations fall back to base content.

## Shell and states

Every route has a skip link, a header with the active item, a mobile drawer,
a segmented language control, breadcrumbs where the route has a trail, and
a three-column footer. `document.documentElement.lang` follows the locale.

Page-level loading, error, empty, and not-found states keep the shell, use
shared translation and date helpers, and do not show raw GraphQL errors.
Images have a fallback. Transient feedback uses a polite live region.
Compact controls and share actions are at least 44×44 CSS px.

## Design

The public site uses the Ink & Tide tokens with light and dark parity, the
documented type scale, spacing and container primitives, and a motion
contract that honors reduced motion (no entrance motion, no smooth scroll).
Focus styles are visible. Article text uses `.article-prose`, not Tailwind
Typography `prose`. Do not add a new icon library for this site.

## Reader

Home is a writing index with asymmetric compositions. Category index uses
numbered editorial rows. Category detail uses article rows. Related posts
are deterministic and exclude the current post. Share actions work.
Thumbnail-less cards use deterministic CSS art at the requested ratio, with
no network fetch.

Post detail (`/:lang/posts/:slug`) shows category, date, reading time when
known, title, deck, byline, featured image, a body capped at 68ch, share,
and related posts. The body wrapper is `.article-prose` around the full
TipTap document. Ordinary code blocks stay Highlight.js and scroll
horizontally.

WHEN a featured image exists, THEN it uses the content width and its
intrinsic aspect ratio. It is not cropped to 21:9 and not stretched. Alt
text is a localized string derived from the title, not empty.

WHEN there is no featured image, THEN the slug-based fallback keeps its
layout ratio. The old 21:9 crop applies only to surfaces that still request
that frame (cards), not to the detail image.

Mermaid in the localized body follows the same renderer rules as
`admin-editor.md`, including diagram-only translations and stale-render
protection.

## About

`/:lang/about` exists in the nav. Without verified copy for that locale the
page shows a localized "Coming soon" state and no placeholder facts.

## Metadata

Every public route and locale has one title, description, canonical URL,
Open Graph, Twitter card, and language. Home, category list, category,
article, about, loading, error, and not-found each have a deterministic
localized profile. Empty fields use configured site defaults. Client
navigation replaces managed tags and does not leave duplicates or the
previous route's tags. SSR puts the resolved head in the first HTML
response.

Canonical URLs are the configured site origin plus a normalized locale path,
with no query or fragment. CMS and config strings are escaped. Unpublished,
missing, and error states are not indexable articles or categories.

Admin documents set a task-specific title for login, dashboard, list,
create, and edit, include `robots` `noindex, nofollow`, and emit no public
canonical, Open Graph, or Twitter tags. Resolver and SSR/client/admin
regressions are tested.

## SEO head assets

`domain_seo` is the trusted store. Only administrators manage Ducth global
head assets. The CMS UI does not execute asset source. Only validated
head-safe markup can be stored. Enabled assets are injected by public SSR
in stable order and MUST NOT replace typed metadata names or properties.
Delivery is bounded. If the source is unavailable, SSR fails open: it may
use a successful cache only inside the stale bound, and logs omit source
HTML and secrets.

## Crawlers

`GET /sitemap.xml` is UTF-8 XML: localized home, categories, and about for
`en` and `vi`, plus one localized URL per published post and published
top-level category. No root redirect URL, admin route, API route,
unpublished post, query, or fragment. XML-significant characters are
escaped.

`GET /robots.txt` and `GET /sitemap.xml` are served before the SPA
fallback. `robots.txt` is `User-agent: *` / `Allow: /`, still disallows
`ClaudeBot`, and advertises `Sitemap: <site>/sitemap.xml`. It does not
depend on GraphQL. If the content API fails, times out, or is malformed,
the sitemap still returns the static localized routes, hides upstream
errors, and uses a short public cache.
