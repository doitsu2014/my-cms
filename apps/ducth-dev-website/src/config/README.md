# Public runtime metadata

The website metadata policy reads these validated runtime values:

- `WEBSITE_SITE_NAME` and `WEBSITE_DEFAULT_TITLE` identify the public site.
- `WEBSITE_DEFAULT_DESCRIPTION` is the bounded fallback description.
- `WEBSITE_SITE_URL` is the canonical URL base.
- `WEBSITE_PUBLIC_MEDIA_BASE_URL` resolves relative article thumbnails for social cards.
- `WEBSITE_DEFAULT_LOCALE` selects the fallback locale (`en` when omitted).

Published article and category translations can refine the title and preview. Missing,
unpublished, loading, and error states use a safe fallback and are marked
`noindex, nofollow`; raw article HTML is never used as a description. The public
website is indexable only for resolved, published routes.

The admin application is a private control plane. Its built document and every route
use `noindex, nofollow` and do not emit public canonical, Open Graph, or Twitter tags.
CMS-editable SEO overrides are intentionally out of scope; metadata remains derived
from the existing runtime configuration and public content fields.
