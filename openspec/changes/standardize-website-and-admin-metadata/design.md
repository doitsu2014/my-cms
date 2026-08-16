## Context

Both products are React applications, but they need different metadata policies.  The public `apps/ducth-dev-website` application renders the body server-side through `src/index.server.tsx` and `src/server.prod.mjs`; its template has no title or SEO elements and the server currently injects only runtime config and Apollo state.  The application already validates and exposes `WEBSITE_DEFAULT_TITLE`, `WEBSITE_DEFAULT_DESCRIPTION`, `WEBSITE_SITE_URL`, locale, localized post/category values, and article thumbnails, but no component consumes them for the document head.

`apps/web` is a client-rendered Rsbuild admin SPA. Its router owns the complete login and protected admin route set, but it has no metadata policy and the generated document retains the Rsbuild default title.  Its control-plane pages must be explicitly non-indexable, rather than receiving public SEO markup.

Graph review found the public SSR render flow (`index.server.tsx` → runtime config → GraphQL client → `AppContent` → public pages) as the public integration seam, and `apps/web/src/App.tsx` as the admin route-title seam. Targeted repository inspection confirmed no existing metadata symbols or tests. The graph snapshot identifies three test gaps and the current public render flow as medium risk; metadata values must therefore have a single owner across SSR and browser navigation.

## Goals / Non-Goals

**Goals:**

- Provide accurate, localized titles and safe SEO/social metadata for all public route states, including the HTML served to non-JavaScript crawlers.
- Ensure browser navigation has the same metadata as the public SSR response and never accumulates duplicate managed tags.
- Give every admin route an intelligible, private document title and consistent `noindex, nofollow` protection.
- Establish a small, typed, testable metadata boundary instead of scattering direct `document` writes across pages.

**Non-Goals:**

- CMS-authored SEO overrides, new GraphQL fields, or database migrations.
- Changing public content visibility, publication logic, or localized content behavior.
- Creating or sourcing a new social-card image; existing eligible article imagery may be used only when it is available.
- SEO optimization of the private administration application.

## Decisions

### 1. Use metadata profiles resolved from a single typed owner

Introduce a public metadata resolver that maps the supported route state and available localized content to a structured profile: title, description, canonical URL, robots policy, Open Graph/Twitter values, and locale. Use a complementary small admin route resolver for page titles and robots directives. Components supply route/content facts; they do not manipulate individual head tags.

This makes default, loading, error, and not-found behavior explicit and testable, and lets SSR serialize exactly the same resolved profile used after browser navigation. A component-local `useEffect` per page is rejected because it risks inconsistent fallbacks and stale/duplicated tags. A framework migration is rejected because it is disproportionate to the required behavior.

### 2. Make public metadata available at the SSR seam and reconcile it on navigation

Extend the public server render result with a resolved metadata profile (or an already escaped head fragment produced from it). `server.prod.mjs` injects this only into the existing HTML template's head, alongside its current runtime-config and Apollo-state injections. The browser applies the same profile after router navigation, replacing only tags marked as managed by the metadata layer.

The server must resolve after Apollo data loading, so article/category metadata has the same data as the rendered route. Client-side-only head updates are rejected because crawlers and social preview bots would still receive the empty template head.

### 3. Derive public route metadata conservatively from existing public content

Use `WEBSITE_DEFAULT_TITLE` and `WEBSITE_DEFAULT_DESCRIPTION` as validated fallbacks. Published posts use localized title and preview text; categories use their localized display name; static routes use localized editorial defaults. Canonical URLs normalize `WEBSITE_SITE_URL` plus the supported locale route path and exclude query/hash values. The description must be plain, bounded text derived from the preview/default, never raw article HTML. If an eligible thumbnail exists, it can be supplied as the social-image value; otherwise the profile omits it rather than inventing an asset.

Allowing arbitrary post body HTML or emitting a canonical article profile before publication/data resolution is rejected because both can misrepresent or expose content. CMS-editable overrides are deferred because they require an authenticated authoring contract, validation rules, storage, and new API work.

### 4. Treat the admin SPA as private and route-aware

The admin application uses a centrally declared mapping from route patterns to title labels, with a product/admin suffix. The resolver runs at the router boundary and creates or updates exactly one robots meta tag set to `noindex, nofollow`. The build HTML default is updated so the first paint is not `Rsbuild App`; no public SEO/social/canonical tags are added.

Static one-size-fits-all admin titles are rejected because operators cannot distinguish edit/create/list tasks in tabs. Public SEO tags are rejected because the admin is a control plane and its routes may be authenticated or sensitive.

### 5. Escape once at HTML serialization and test the actual head

The public SSR insertion path must HTML-escape all profile strings before writing them into the template. Browser DOM updates must use DOM APIs/property assignment rather than HTML string concatenation. Tests cover the resolver directly and parse rendered SSR/head output plus client navigation, so they can detect both unsafe rendering and duplicates.

## Risks / Trade-offs

- [SSR data and browser metadata diverge] → The SSR and browser adapters consume the same typed resolver/profile and share fixtures for representative routes.
- [CMS content contains markup-significant text] → Normalize descriptions to text and escape all head values at the rendering boundary; include hostile-string regression tests.
- [Public error/not-found metadata accidentally describes stale content] → Model these states explicitly with default/error profiles and replace managed tags on every navigation.
- [Social image is unavailable or unsuitable] → Omit the optional image tag rather than fabricate a URL; a later CMS-override capability can provide a curated fallback.
- [Admin route titles drift as routes change] → Keep the mapping next to router ownership and add a representative test for each route family.
- [Search indexing assumptions differ by deployment] → Make the public robots policy explicit in the resolver; retain `noindex, nofollow` as the invariant for admin.

## Migration Plan

1. Add and test the pure public/admin metadata resolvers, including locale, canonicalization, fallback, and escaping cases.
2. Integrate public metadata with SSR response generation and client navigation, then verify representative HTML documents contain one managed tag set.
3. Integrate the admin title/robots policy at the router boundary and replace the generated default document title.
4. Run website/admin tests and production builds. Deploy normally; no schema or API migration is required.
5. Roll back by reverting the metadata integration and template changes. Content and API contracts remain unaffected.

## Open Questions

- Confirm the approved public title template and editorial defaults for English and Vietnamese routes.
- Confirm whether `hreflang` alternate links are required in this iteration; the initial requirement guarantees the active document language but does not require alternate-link discovery.
- Confirm the desired default social-card asset. The proposed scope emits an image only when existing eligible content has one.
- Confirm whether public metadata should start as indexable in every deployed environment or support an environment-level noindex switch.
