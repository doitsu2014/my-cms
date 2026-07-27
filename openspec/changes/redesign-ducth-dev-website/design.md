## Context

The migrated reader at `apps/ducth-dev-website/` renders the generic DaisyUI `emerald` blog from the source repository. A complete redesign — Ink & Tide — has been supplied under `apps/ducth-dev-website/design/new-design/` with five static HTML page templates, a brand spec, a manifest, and a handoff document. The redesign is editorial and bilingual (English + Vietnamese), built around a parchment/cinnabar palette, serif display type, asymmetric layouts, and a new `/:lang/about` route.

The in-flight `migrate-ducth-dev-website` change (62/95 tasks complete) explicitly excludes visual redesign, copy changes, and new routes (`openspec/changes/migrate-ducth-dev-website/proposal.md:7-9`). This change is therefore sequenced after migration verification and modifies a stable, verbatim-preserved baseline.

The current frontend is a React 19 + Rsbuild + Express 5 SSR app: `apps/ducth-dev-website/src/App.tsx`, `AppContent.tsx`, `index.server.tsx`, `index.client.tsx`, `rsbuild.config.ts`. Routing is locale-prefixed (`/:lang`, `/:lang/categories`, `/:lang/categories/:slug`, `/:lang/posts/:slug`) and the data layer is Apollo against `GET /graphql/immutable`. There is no backend dependency change in this proposal.

The supplied HTML/CSS prototype is the visual source of truth — `DESIGN-HANDOFF.md:3-10` states "pixels and behavior win over prose." However, the prototype has known accessibility failures (no mobile navigation below 720px, nested anchors, no skip link, share buttons without accessible names), so the design must protect the editorial intent while correcting them.

## Goals / Non-Goals

**Goals:**

- Replace the `emerald` theme with an app-owned `ink-tide` theme defined once in CSS custom properties, surfaced to Tailwind/DaisyUI through a single semantic mapping.
- Establish a token-driven, light/dark-parity foundation: typography scale, spacing scale, container primitives, prose styles, motion, focus, reduced-motion rules.
- Rebuild the shell (header, footer, skip link, mobile menu, language switch, breadcrumb) as reusable components in `src/components/layout/` and `src/components/navigation/`.
- Rewrite the four existing reader pages against the new design system and add the new `/:lang/about` page.
- Extract shared helpers (`getLocalizedPost`, `getLocalizedCategory`, `formatPublishedDate`, `getPostThumbnail`) currently duplicated across pages.
- Preserve SSR/hydration, runtime-config injection, locale routing, and Apollo contract from `migrate-ducth-dev-website`.
- Correct accessibility regressions implicit in the static prototype (mobile navigation, nested links, share controls, focus, document language, reduced motion, target size).

**Non-Goals:**

- No new GraphQL fields, no backend changes, no media pipeline changes.
- No new third-party icon library (the production dependency set has no Lucide — `apps/ducth-dev-website/package.json:18-56`); reuse inline SVGs and text arrows.
- No real reading-time computation, no real pagination unless the existing GraphQL query can support it without backend change; otherwise the design defers those and the page renders the complete list.
- No invented content facts (years of experience, post counts, contact details, hiring status); these are sourced from the product-owner as approved copy.
- No changes to deployment wiring, environment contract, or release pipeline beyond what `migrate-ducth-dev-website` already added.
- No migration of the legacy `my-blogs-rsbuild/client_side` repository; that happens in a separate change.

## Decisions

### 1. App-owned `ink-tide` theme over DaisyUI theme variables

**Decision:** Define Ink & Tide as CSS custom properties in `src/App.css` and map them onto a single DaisyUI theme block (`data-theme="ink-tide"`). Keep semantic class compatibility (`base-100`, `base-200`, `neutral`, `primary`, `primary-content`) so future DaisyUI components still work, but disable `daisyui.themes` so DaisyUI does not emit `emerald`/`light`/`dark` blocks.

**Rationale:** The redesign brand spec uses OKLCH values with explicit light/dark parity (`apps/ducth-dev-website/design/new-design/brand-spec.md:17-44`). Forcing DaisyUI themes adds an extra layer of indirection and risks CSS-variable collisions. Owning the tokens directly gives a single source of truth and keeps font-face and focus rules in the same file.

**Alternatives considered:**
- Extend the existing `emerald` theme variables — rejected because the redesign drops the green palette entirely.
- Replace DaisyUI entirely — rejected because the redesign still benefits from utility classes and would expand the rebuild surface.

### 2. Self-hosted fonts via `<link>` to Google Fonts initially, with a self-hosted cutover path

**Decision:** Load Noto Serif Display, Inter, and JetBrains Mono via Google Fonts `<link>` references in the SSR-injected HTML shell. Keep the variable definitions in `App.css` so a self-hosted swap is a one-file change.

**Rationale:** Self-hosting requires font subsetting and unicode-range tuning for Vietnamese diacritics; the migration is out of scope. Google Fonts is acceptable for the redesign scope and matches the existing pattern of declaring Roboto (`apps/ducth-dev-website/src/App.css:1-18`).

**Alternatives considered:**
- Add `@fontsource/*` packages — deferred; the SE may swap if a follow-up change prefers offline font delivery.

### 3. Unconstrained `<main>` with section-level containers

**Decision:** Make `SiteLayout` render an unconstrained `<main id="main">` and let each page own its full-width section blocks with internal `Container` (1240px max, `clamp(20px, 4vw, 48px)` gutter). The current layout forces a single `<main className="container px-4 py-8">` (`apps/ducth-dev-website/src/components/layout/Layout.tsx:11-17`), which prevents the new full-bleed design.

**Rationale:** The redesign needs full-width sections (hero strips, photography bands) with internally constrained content. Page-level control over section background and content width is the simplest model.

**Alternatives considered:**
- Keep a constrained `<main>` and pass a `bleed` prop — rejected because it forces every page to opt into full-width and complicates the layout API.

### 4. New `Container` primitives and `Section` wrappers

**Decision:** Add `src/components/layout/Container.tsx` (1240px cap, fluid gutter) and a `Section` wrapper that takes a `tone` (`parchment | fresh-paper | ink`) for the background. Pages compose `Section` + `Container` rather than building one-off wrappers.

**Rationale:** The redesign uses the same full-width-with-internal-cap pattern on every page (`apps/ducth-dev-website/design/new-design/index.html:43-45`). Centralizing the pattern prevents drift.

### 5. `PostCard` variants over per-page bespoke cards

**Decision:** Extract a single `PostCard` component with five variants: `lead`, `compact`, `standard`, `wide`, `related`. Each variant owns its own typography scale and metadata layout. Pages compose variants rather than rendering their own card markup.

**Rationale:** The home page uses three different post layouts (lead, secondary, recent row) and the post page uses a fourth (`related`). A single component with variants is easier to keep visually consistent than four bespoke cards. This also concentrates the missing-thumbnail fallback in one place.

**Alternatives considered:**
- One card with size props — rejected because the lead/secondary variant pair relies on a 1.4fr/1fr grid that doesn't compose cleanly with a size prop.
- Bespoke cards per page — rejected because it duplicates thumbnail/date/category rendering across files.

### 6. Mobile navigation drawer with focus management

**Decision:** Implement a `MobileNavigation` drawer that triggers from a `<button aria-expanded aria-controls="mobile-nav">` in the header. The drawer closes on Escape, on route change, and when focus returns to the trigger. Anchors inside the drawer use the same primary links as the desktop nav.

**Rationale:** The static prototype hides all navigation below 720px (`apps/ducth-dev-website/design/new-design/index.html:292-294`), which is a navigation failure. The reader cannot reach Categories or About on mobile. A drawer is the lightest modal-like pattern that preserves the three-link structure and remains keyboard-accessible.

**Alternatives considered:**
- Always-visible links that wrap — rejected because the portrait wordmark + locale pill already crowd the bar at 360px.
- Multi-level disclosure — rejected because the IA is three links deep at most.

### 7. Skip link on every route, not just the home

**Decision:** Render `<a href="#main" className="skip-link">Skip to content</a>` inside `SiteLayout` so it appears on every route. The current prototype only includes the skip link on `index.html:691`.

**Rationale:** Skip links are a site-wide expectation, not a homepage affordance. Putting the skip link in the layout means dynamic routes don't drift.

### 8. Synchronize `<html lang>` with the active `:lang` segment

**Decision:** Update `index.server.tsx` and `index.client.tsx` to set `<html lang>` from the resolved `:lang` route parameter. Use `lang="en"` or `lang="vi"` on inline mixed-language spans where pronunciation benefits.

**Rationale:** The current template leaves `<html lang>` at English on Vietnamese routes (`apps/ducth-dev-website/design/10-accessibility.md:27-29`). Screen readers ignore content whose language is mis-declared.

### 9. Functional share actions with a polite live region

**Decision:** Implement share actions as three real controls: an X intent link with `window.open` and proper `rel="noopener"`, a LinkedIn share link, and a Copy link button. The Copy link button writes to the clipboard and announces "Link copied" through a polite live region (`role="status"`).

**Rationale:** The current share buttons are `<button>` elements with no handlers and no accessible names (`apps/ducth-dev-website/src/pages/PostDetailPage.tsx:216-249`). The prototype's 40px share circles are below the 44px target (`apps/ducth-dev-website/design/new-design/post.html:333-345`); production uses 44px buttons with text labels for the screen-reader announcement.

### 10. Deterministic post thumbnail fallback

**Decision:** Add a `PostArtwork` component that renders the CMS post's media thumbnail when present, otherwise renders a deterministic fallback motif keyed by post slug. The fallback is a CSS-only ink motif (no random images).

**Rationale:** The redesign expects real photography; the prototype has three images and assumes coverage. The current code returns no fallback when the thumbnail is missing. Deterministic keying avoids shift on re-render and matches the editorial tone.

**Alternatives considered:**
- Use one of the three prototype images as a universal fallback — rejected because it implies a content role that the image doesn't have.
- Random placeholder service — rejected because it contradicts the editorial tone and breaks offline.

### 11. Featured/curated content contract

**Decision:** This change does NOT repurpose the "six newest posts" as featured. If a separate curated flag is not introduced, the home renders only the recent articles section and the hiring/contact CTA. The PO must decide whether featured content is a real product feature or an artefact of the prototype.

**Rationale:** The product-designer exploration explicitly flagged this as a content contract gap. Building "featured" on top of "newest" silently misrepresents curation. The redesign's home reads as a writing index without a featured section if featured is absent.

### 12. About page reads from a localized static config, not the API

**Decision:** Author the About page content as a TypeScript-localized config (`src/config/about.config.ts`) keyed by `:lang`. The config holds narrative copy, career-pillar labels, and contact/social links, each with a `verified` flag the PO sets to true.

**Rationale:** About is a one-page site artifact that updates infrequently. Adding a CMS model for it is a separate architectural decision; the SA owns that scope. The static config also documents the source of truth for product copy and forces product-owner review.

### 13. Reading progress is deferred unless the PO marks it essential

**Decision:** The fixed reading-progress dot from the prototype is not implemented in the first cut. The post page renders the article, share actions, and related posts without a scroll-bound UI.

**Rationale:** Reading progress is decorative, adds client behaviour, and the README permits it as optional (`apps/ducth-dev-website/design/new-design/README.md:73`). The SA can re-introduce it in a follow-up slice with reduced-motion and `aria-hidden` semantics.

### 14. Pagination only when the existing query supports it

**Decision:** Pagination on the category detail page is conditional. If the existing `GET /graphql/immutable` query returns a paginated connection shape, the SA implements it. Otherwise the page renders the complete list with a "Showing all N articles" line and no pagination chrome.

**Rationale:** The current query fetches all posts (`apps/ducth-dev-website/src/infrastructure/graphql/queries.ts:1-169`); implanting fake page numbers is forbidden by the design rules. The explore report already flags this as a backend concern that the SA determines.

### 15. Component decomposition for readability

**Decision:** Decompose the redesigned pages into `editorial/`, `posts/`, `feedback/`, and `navigation/` folders under `src/components/`. Each page is a thin composition layer (~150 lines) and delegates to the shared components.

**Rationale:** Four pages with bespoke components will each grow past 300 lines without a decomposition. The folders mirror the design-system boundaries (typography, content, feedback, navigation) and make code review tractable.

## Risks / Trade-offs

- **`<main>` restructure breaks SSR hydration** → Mitigation: keep the `Layout` component the SSR entry point; render the same `<a className="skip-link">` and `<main id="main">` once at the layout level; verify hydration by diffing the SSR HTML and the post-hydration DOM for the skip link and main landmark.
- **Theme variable collisions with DaisyUI** → Mitigation: disable `daisyui.themes` in `tailwind.config.ts` and supply a single `ink-tide` block; verify by inspecting computed styles for `--p`, `--b1`, `--bc` on `data-theme="ink-tide"`.
- **Featured content contract unaddressed** → Mitigation: scope the home page to recent articles + category strip + CTA until the PO defines curation; mark the omission as a follow-up decision in `design.md` Open Questions.
- **About content must be PO-approved** → [Risk] → Mitigation: gate the About page behind a per-locale `verified: true` flag; render a "Coming soon" placeholder when unverified so unverified copy never reaches production.
- **Mobile drawer focus trap** → [Risk] → Mitigation: use a `dialog` element with `showModal()` semantics (or a custom dialog with `inert` on background) so the focus trap is enforced; verify by keyboard-only navigation through every route.
- **Shared self-hosted font swap is a follow-up** → [Risk] → Mitigation: ensure the Google Fonts `<link>` is removable without component changes; document the swap in `design.md` Open Questions.
- **GraphQL pagination not supported** → [Risk] → Mitigation: render the complete list with a "Showing all N" line and ask the SA to open a follow-up change if real pagination is required.
- **Prototype visual mismatches with current content** → [Risk] → Mitigation: in the absence of category descriptions, use a slug-derived fallback only after the PO approves; otherwise render only the empty state and avoid fabricated prose.
- **Reading time, author byline, and category descriptions may not be in the API** → [Risk] → Mitigation: render the section only when the data is present; otherwise hide the field cleanly so the page never shows "—" or empty metadata.
- **SSR + new theme + new fonts increase first paint** → [Risk] → Mitigation: keep the `ink-tide` theme attribute on the SSR HTML element alongside a critical inline CSS block so the page renders parchment/cinnabar before the stylesheet finishes loading.
- **Local dev needs to reflect the new theme** → [Risk] → Mitigation: the existing `migrate-ducth-dev-website` Compose service is reused unchanged; the only dev-side change is restarting the SSR after the asset copy.
- **Content residue from the prototype** → [Risk] → Mitigation: add a content sweep checklist under Slice 6 that lists every prototype phrase and requires PO sign-off before publication.
