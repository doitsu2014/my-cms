## Why

The migrated reader at `apps/ducth-dev-website/` still renders the generic DaisyUI `emerald` blog from the source repository: uniform cards, a hollow hero, a category index with rotating semantic colors, and a post page with inert share buttons. A complete redesign — codenamed **Ink & Tide** — has been supplied under `apps/ducth-dev-website/design/new-design/` with brand tokens, page templates, and a handoff document. Implementing it shifts the public reader from a generic blog into a bilingual, editorial, career-oriented site that better serves hiring managers, technical readers, and returning bilingual visitors.

The active `migrate-ducth-dev-website` change explicitly excludes "no visual redesign," copy changes, and new routes (`openspec/changes/migrate-ducth-dev-website/proposal.md:7-9`), so this work must ship as a separate follow-up change sequenced after migration verification.

## What Changes

- Replace the `emerald` DaisyUI theme with an app-owned `ink-tide` theme: parchment surface, deep ink foreground, single cinnabar accent, hairline borders, and explicit light/dark token parity (see `apps/ducth-dev-website/design/new-design/brand-spec.md:17-44`).
- Adopt the typography system: Noto Serif Display (display), Inter (body/UI), JetBrains Mono (code), with fluid display sizes and a 68ch article measure (`apps/ducth-dev-website/design/new-design/brand-spec.md:69-106`, `apps/ducth-dev-website/design/new-design/post.html:28`).
- Replace the constrained `<main className="container">` wrapper with an unconstrained layout shell that supports full-width sections with internal 1240px containers and fluid gutters (`apps/ducth-dev-website/src/components/layout/Layout.tsx:11-17`, `apps/ducth-dev-website/design/new-design/index.html:43-45`).
- Add a new route `/:lang/about` with a career hero, career pillars, practice habits, and verified contact/social links (`apps/ducth-dev-website/design/new-design/about.html`).
- Add an active-route-aware sticky hairline header with wordmark, primary navigation (Home, Categories, About), and a VI/EN segmented control (`apps/ducth-dev-website/design/new-design/index.html:210-294`).
- Add a mobile menu trigger with focus management, Escape-to-close, and route-change closing; the exported prototype hides all navigation below 720px with no replacement, which is not acceptable (`apps/ducth-dev-website/design/new-design/index.html:292-294`).
- Replace the existing centered DaisyUI footer with a three-column editorial footer (`apps/ducth-dev-website/design/new-design/index.html:595-631`).
- Add a skip link on every route and synchronize `<html lang>` with the active `:lang` segment (current behavior leaves `<html lang>` at English on Vietnamese routes — `apps/ducth-dev-website/design/10-accessibility.md:27-29`).
- Rewrite `HomePage` as a writing index: featured lead + two secondary, six recent articles in asymmetric layouts, a category-count strip, and a hiring/contact CTA (`apps/ducth-dev-website/design/new-design/index.html:713-1026`). **BREAKING** of the current "six newest as featured" model: featured is a distinct contract, not "newest six."
- Rewrite `CategoriesPage` as a numbered editorial row list with three latest-article previews per category and remove the rotating semantic color badges (`apps/ducth-dev-website/design/new-design/categories.html:310-400`).
- Rewrite `CategoryDetailPage` with a split intro, vertical article rows replacing the uniform card grid, and real pagination only if the GraphQL contract supports it (`apps/ducth-dev-website/design/new-design/category.html:259-432`).
- Rewrite `PostDetailPage` with an editorial header, 21:9 featured image, custom 68ch prose styles, functional share actions (X, LinkedIn, copy link), and deterministically sorted related posts (`apps/ducth-dev-website/design/new-design/post.html:456-738`). The current inert `<button>` share elements are removed.
- Extract shared editorial components and helpers: `PostCard` (lead/compact/standard/wide/related variants), `PostListRow`, `SectionHeader`, `Breadcrumbs`, `LanguageSwitch`, `ContentSkeleton`, `ContentError`, `ContentEmpty`, `NotFoundState`, plus shared `getLocalizedPost`, `getLocalizedCategory`, `formatPublishedDate`, and `getPostThumbnail` helpers currently duplicated across pages.
- Replace `Roboto` with the new self-hosted font stack and remove the forced `data-theme="emerald"` assignment (`apps/ducth-dev-website/src/App.css:1-18`, `apps/ducth-dev-website/src/index.client.tsx:6-10`).
- Move the three prototype images (`avatar.jpg`, `architecture.jpg`, `coast.jpg`) into a production-served asset path and define a deterministic fallback motif for missing post thumbnails.
- Apply the reduced-motion contract: no entrance translations, no smooth scroll, no hover transforms; keep instantaneous state changes visible (`apps/ducth-dev-website/design/new-design/index.html:64-65`, `apps/ducth-dev-website/design/new-design/brand-spec.md:77`).
- Real, verified content only: no prototype placeholder claims (e.g., "11 years," "38 articles," fake contact details) reach production. Approved copy comes from product-owner; the change does not invent facts.

## Capabilities

### New Capabilities

- `website-design-system`: Ink & Tide design tokens (color, typography, spacing, container), light/dark parity, motion contract, prose styles, and reduced-motion rules.
- `website-shell`: Shared layout (header, footer, skip link, mobile navigation, language switch, breadcrumb), 1240px container primitive, and active-route semantics.
- `website-reader-experience`: Bilingual home (featured + recent + categories strip + CTA), categories index (numbered editorial rows with latest previews), category detail (split intro + article rows + optional pagination), and post reading experience (editorial header, 68ch prose, share actions, related posts).
- `website-about`: New `/:lang/about` page with career hero, career pillars, practice habits, and verified contact/social links.
- `website-content-states`: Loading, error, empty, not-found, partial-data, and offline patterns plus shared translation/date/media helpers used across all reader pages.

### Modified Capabilities

None. The canonical `openspec/specs/` capabilities (`user-management`, `supabase-auth`, `supabase-storage`, `media-bucket-management`, `image-transformation`, `pgvector-vector-search`, `local-dev-environment`) are not affected by behaviour changes. The in-flight `website-frontend` and `website-deployment` delta specs from `migrate-ducth-dev-website` are not modified either; deployment wiring is reused unchanged.

## Impact

- **Code**: `apps/ducth-dev-website/src/` (rewrite/add `pages/AboutPage.tsx`, restructure `components/layout/`, add `components/editorial/`, `components/posts/`, `components/feedback/`, `components/navigation/`, update `App.css`, `index.client.tsx`, `index.html` template). Touch `tailwind.config.ts` only if an app-owned font/container mapping improves consistency.
- **Assets**: New `apps/ducth-dev-website/public/images/{avatar,architecture,coast}.jpg`; add a deterministic fallback motif path. No new third-party icon library; the production dependency set has no Lucide (`apps/ducth-dev-website/package.json:18-56`), so inline SVGs or text arrows are reused.
- **Dependencies**: No new runtime packages. Self-hosted fonts (or `<link>` to Google Fonts) are the only added asset cost.
- **GraphQL**: No new fields are required for the core redesign. Reading time, pagination, and category descriptions are handled by client-side fallback or by enhancement-only routes that explicitly detect missing data; the SA must confirm whether pagination is supported by the existing query before deferring it.
- **APIs consumed**: Unchanged — `GET /graphql/immutable` and `GET /media/*` from `apps/api/src/bin/my-cms-api.rs:99-113`.
- **Deployment**: Unchanged. The `ducth-dev-website` service, Traefik router, and env contract added by `migrate-ducth-dev-website` are reused as-is.
- **CI**: No new release pipeline. Verification gate is `pnpm --dir apps/ducth-dev-website lint`, `pnpm --dir apps/ducth-dev-website build`, plus visual checks at 360 / 820 / 1024 / 1440 / 1920 widths.
- **Accessibility**: Skip link, document-language sync, focus-visible ring, 44px target size for icon-only controls, fix nested links in the prototype, and `prefers-reduced-motion` contract.
- **Risks**: Carries the same SSR/route boundaries and `data-theme` hydration surface as the migration; sequencing after `migrate-ducth-dev-website` verification avoids contaminating an in-flight change.
