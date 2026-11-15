# Public site, reader, and SEO

[Glossary](glossary.md) · Related: [admin editor](admin-editor.md), [deployment operations](deployment-operations.md)

## Evidence boundary

Scope paths: `apps/ducth-dev-website/`, `apps/web` document head, `apps/api/domain_seo/`, and `packages/editor-prose/`. The partial CodeKB describes route composition but does not deeply verify these requirements. Treat detailed behaviors below as **Unverified intended contract** pending source/test inspection. Maintained docs and project knowledge support, but do not supersede current source.

## Localized reader

The intended locales are `en` and `vi` at `/:lang/...`; public GraphQL uses the immutable posts endpoint and missing translations fall back to base content. The shell should provide skip link, navigation, mobile drawer, language control, route breadcrumbs, localized document language, and consistent loading/error/empty/not-found states without exposing raw GraphQL errors. Verify all route and state behavior in current app source.

Article rendering uses `.article-prose` from the shared package. Featured images retain intrinsic ratio; missing-image cards use deterministic CSS artwork, while the detail-page fallback preserves its layout ratio. Article body width is capped at 68ch. Related posts are deterministic and exclude the current post. Mermaid uses the explicit shared renderer contract in [admin-editor](admin-editor.md). These presentation details are **Unverified**.

## Metadata, SEO, and crawlers

Intended public metadata includes localized title, description, canonical URL, Open Graph and Twitter tags, with configured defaults for empty values and no stale duplicates after client navigation. Canonicals use configured origin plus normalized locale path without query/fragment. CMS/config values are escaped; unpublished, missing and error states are not indexable content. Admin pages use task-specific titles and `noindex, nofollow`, with no public canonical or social-card tags. **Unverified:** current resolver, SSR output and escaping behavior.

`domain_seo` is the intended trusted store for administrator-managed global head assets. Only validated head-safe markup may be stored; the UI does not execute asset source. Public SSR injects enabled assets in stable order without replacing typed metadata. Delivery bounds, cache/stale behavior and failure policy remain **Unverified** pending domain and SSR source review.

Intended crawler endpoints: `/sitemap.xml` includes localized home, categories, about, published posts and published top-level categories, excludes admin/API/unpublished/query/fragment URLs, and escapes XML. `/robots.txt` allows general crawling, disallows `ClaudeBot`, and advertises the sitemap. Sitemap failure should retain static localized routes and hide upstream errors. Current output and behavior are **Unverified**.

## Open evidence items

Verify locale routes, SSR/client parity, metadata, SEO asset validation and delivery, crawler endpoint output, and reader image/content states. Record unresolved source/contract conflicts with both claims and references; request owner decision where source cannot determine intended policy.
