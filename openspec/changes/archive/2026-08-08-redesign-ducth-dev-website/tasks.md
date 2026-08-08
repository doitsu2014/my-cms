## 1. Foundation and design tokens

- [ ] 1.1 Confirm `migrate-ducth-dev-website` is at a stable baseline (all 95 tasks complete and verified) before starting this change
- [ ] 1.2 Add Google Fonts `<link>` tags for Noto Serif Display, Inter, and JetBrains Mono in the SSR-injected HTML shell
- [ ] 1.3 Replace `src/App.css` Roboto/emerald setup with CSS custom properties for the `ink-tide` theme (parchment, fresh-paper, ink, muted ink, hairline, cinnabar, accent wash, ink ground, feedback trio)
- [ ] 1.4 Update `tailwind.config.ts` to disable `daisyui.themes` and supply a single `ink-tide` theme block
- [ ] 1.5 Remove the forced `data-theme="emerald"` assignment from `src/index.client.tsx`
- [ ] 1.6 Add `@media (prefers-reduced-motion: reduce)` rules that disable entrance animations, smooth scrolling, and hover transforms
- [ ] 1.7 Add a global focus-visible outline (2px outline, 3px offset using the accent color)
- [ ] 1.8 Add `.article-prose` styles for headings, paragraphs, links, blockquotes, lists, inline code, code blocks, separators, pull quotes, and figures that preserve Highlight.js code styling

## 2. Shared shell and layout

- [ ] 2.1 Add `src/components/layout/Container.tsx` (1240px max, `clamp(20px, 4vw, 48px)` gutter)
- [ ] 2.2 Add `src/components/layout/Section.tsx` with a `tone` prop (`parchment | fresh-paper | ink`) for full-width section backgrounds
- [ ] 2.3 Refactor `src/components/layout/Layout.tsx` into `SiteLayout.tsx` that renders skip link, header, unconstrained `<main id="main">`, and footer
- [ ] 2.4 Add a skip link rendered as the first focusable element of the layout with `#main` as the target
- [ ] 2.5 Add `src/components/layout/SiteHeader.tsx` with the wordmark, primary navigation, and language switch
- [ ] 2.6 Add active-route underline styling for the header navigation links
- [ ] 2.7 Add `src/components/layout/SiteFooter.tsx` with the author summary, primary navigation summary, and external links columns
- [ ] 2.8 Add a build note rendered in the footer when `NODE_ENV !== "production"`
- [ ] 2.9 Add `src/components/navigation/MobileNavigation.tsx` with a `Menu` button, drawer, and focus management
- [ ] 2.10 Add `src/components/navigation/LanguageSwitch.tsx` as a segmented control with VI/EN buttons
- [ ] 2.11 Add `src/components/navigation/Breadcrumbs.tsx` with `<nav aria-label="Breadcrumb">` and an ordered list
- [ ] 2.12 Wire `src/AppContent.tsx` to render the new `SiteLayout` and re-export the existing routes
- [ ] 2.13 Update `index.server.tsx` and `index.client.tsx` to set `<html lang>` from the active `:lang` route segment

## 3. Editorial primitives and content components

- [ ] 3.1 Add `src/components/editorial/Eyebrow.tsx` for the 12px uppercase tracking label
- [ ] 3.2 Add `src/components/editorial/SectionHeader.tsx` with 1/3 + 2/3 split composition
- [ ] 3.3 Add `src/components/editorial/PostMeta.tsx` for category, date, and optional reading time
- [ ] 3.4 Add `src/components/editorial/CategoryLabel.tsx` for the category pill
- [ ] 3.5 Add `src/components/editorial/PullQuote.tsx` for body pull quotes
- [ ] 3.6 Add `src/components/editorial/EditorialImage.tsx` with a fixed aspect ratio and localized alt
- [ ] 3.7 Add `src/components/posts/PostCard.tsx` with `lead`, `compact`, `standard`, `wide`, and `related` variants
- [ ] 3.8 Add `src/components/posts/PostListRow.tsx` for the vertical category detail row
- [ ] 3.9 Add `src/components/posts/PostArtwork.tsx` rendering the thumbnail when present and a deterministic fallback motif when absent
- [ ] 3.10 Add `src/components/posts/RelatedPosts.tsx` that filters by shared category, excludes the current post, and sorts by date descending
- [ ] 3.11 Add `src/components/posts/ShareActions.tsx` with X intent, LinkedIn share, and Copy link controls
- [ ] 3.12 Add `src/components/posts/ArticleProse.tsx` wrapping the TipTap HTML in the `.article-prose` contract
- [ ] 3.13 Add `src/components/feedback/ContentSkeleton.tsx` with page-specific variants
- [ ] 3.14 Add `src/components/feedback/ContentError.tsx` with a localized message and retry button
- [ ] 3.15 Add `src/components/feedback/ContentEmpty.tsx` with a localized message and contextual link
- [ ] 3.16 Add `src/components/feedback/NotFoundState.tsx` for the post and category detail not-found states
- [ ] 3.17 Add `src/components/feedback/StatusMessage.tsx` exposing a `role="status" aria-live="polite"` live region

## 4. Shared helpers

- [ ] 4.1 Add `src/lib/i18n/getLocalizedPost.ts` and migrate all pages to use it
- [ ] 4.2 Add `src/lib/i18n/getLocalizedCategory.ts` and migrate all pages to use it
- [ ] 4.3 Add `src/lib/i18n/formatPublishedDate.ts` and migrate all pages to use it
- [ ] 4.4 Add `src/lib/media/getPostThumbnail.ts` and migrate all pages to use it
- [ ] 4.5 Remove duplicated translation, date, and thumbnail logic from `HomePage`, `CategoriesPage`, `CategoryDetailPage`, and `PostDetailPage`

## 5. Home page

- [ ] 5.1 Replace the centered DaisyUI hero with a recent articles section using `PostCard` variants
- [ ] 5.2 Add the category-count strip derived from the GraphQL response
- [ ] 5.3 Add the hiring/contact CTA block that renders only when verified content is provided
- [ ] 5.4 Confirm the featured/highlights section is omitted unless the PO approves a curated content contract
- [ ] 5.5 Remove the generic welcome copy and the unread button
- [ ] 5.6 Add `aria-busy` to the loading skeleton and the localized error retry path

## 6. Categories index

- [ ] 6.1 Replace the uniform card grid with a numbered editorial row per category
- [ ] 6.2 Add the three latest article previews per row using `PostCard` compact variant
- [ ] 6.3 Remove the rotating semantic color badges
- [ ] 6.4 Remove the "most popular" stat and the fixed "updated year" stat
- [ ] 6.5 Render the localized editorial description only when the API returns one; otherwise omit the description field
- [ ] 6.6 Add the locale-aware page header with the count of categories and posts

## 7. Category detail

- [ ] 7.1 Add the split intro with category title, localized description, and optional image
- [ ] 7.2 Replace the uniform card grid with `PostListRow`
- [ ] 7.3 Add localized empty state with a link back to the categories index
- [ ] 7.4 Implement pagination only if the existing GraphQL query supports it; otherwise render "Showing all N articles" with no pagination chrome
- [ ] 7.5 Confirm collapsed responsive behavior at 360px (single column) and 820px (split intro)

## 8. Post reading experience

- [ ] 8.1 Add the editorial header (category pill, date, optional reading time, title, deck, byline)
- [ ] 8.2 Add the 21:9 featured image with a localized alt
- [ ] 8.3 Constrain the article body to 68ch and apply the `.article-prose` contract
- [ ] 8.4 Replace the inert share buttons with the functional `ShareActions` component
- [ ] 8.5 Replace the related cards with `RelatedPosts` sorted by date descending and excluding the current post
- [ ] 8.6 Add the polite live region for "Link copied" feedback
- [ ] 8.7 Confirm reading progress is deferred (not implemented) unless the PO marks it essential

## 9. About page

- [ ] 9.1 Add `src/config/about.config.ts` with localized `verified` flags, hero metadata, pillars, statement, practices, and contact/social links
- [ ] 9.2 Add `src/pages/AboutPage.tsx` reading from the localized config
- [ ] 9.3 Register the `/:lang/about` route in `src/AppContent.tsx`
- [ ] 9.4 Add "About" to the primary navigation in `SiteHeader`
- [ ] 9.5 Render the "Coming soon" placeholder when the locale's `verified` flag is `false`
- [ ] 9.6 Render the contact and social links only when the corresponding field is non-empty

## 10. Assets and content

- [ ] 10.1 Move `avatar.jpg`, `architecture.jpg`, and `coast.jpg` from `apps/ducth-dev-website/design/new-design/assets/` to `apps/ducth-dev-website/public/images/`
- [ ] 10.2 Add a deterministic fallback motif for missing post thumbnails (CSS-only ink motif)
- [ ] 10.3 Confirm Google Fonts `<link>` references resolve to the typography families used in the redesign
- [ ] 10.4 Add bilingual UI copy for the new sections in `src/i18n/locales/en.json` and `src/i18n/locales/vi.json`

## 11. Accessibility and responsiveness

- [ ] 11.1 Verify the skip link is the first focusable element on every route
- [ ] 11.2 Verify focus ring is visible on every interactive element at 360px and 1440px
- [ ] 11.3 Verify the mobile drawer closes on Escape, on route change, and returns focus to the trigger
- [ ] 11.4 Verify the language switch preserves the path and updates `<html lang>` on hydration
- [ ] 11.5 Verify the share controls hit target is at least 44×44 CSS px
- [ ] 11.6 Verify the reduced-motion contract hides entrance animations and smooth scrolling
- [ ] 11.7 Verify the responsive breakpoints at 360px, 820px, 1024px, 1440px, and 1920px
- [ ] 11.8 Verify no nested anchors exist on the home, category, and post pages

## 12. Verification

- [ ] 12.1 Run `pnpm --dir apps/ducth-dev-website lint`
- [ ] 12.2 Run `pnpm --dir apps/ducth-dev-website typecheck`
- [ ] 12.3 Run `pnpm --dir apps/ducth-dev-website build`
- [ ] 12.4 Restart the SSR service and verify the home, categories, category detail, post, and about routes render
- [ ] 12.5 Verify the SSR HTML for the `<html lang>`, skip link, header, main, and footer landmarks
- [ ] 12.6 Verify light and dark mode contrast for body, hairline, eyebrow, and accent
- [ ] 12.7 Verify the loading skeleton, error retry, empty state, and not-found state on every page
- [ ] 12.8 Verify the Copy link and share actions on the post page
- [ ] 12.9 Verify the related posts exclude the current post and are sorted by date descending
- [ ] 12.10 Run `openspec verify --change "redesign-ducth-dev-website"` and resolve reported issues
