# 08 — Components & UI Patterns

## Layout shell

**Source:** `src/components/layout/Layout.tsx:9-17`, `Header.tsx:34-70`, `Footer.tsx:9-45`.

- Root: `flex flex-col min-h-screen bg-base-100`; header; `main.flex-grow container mx-auto px-4 py-8`; centered footer.
- Header: `bg-base-200 shadow-lg`, compact `py-2 px-4`, avatar home link, horizontal menu, end-aligned language dropdown.
- Footer: `footer footer-center p-10 bg-base-200 text-base-content`; bold label, dynamic copyright, three text social links.
- **Do:** preserve surface roles and flex growth. **Don’t:** add competing navigation or a fixed width without resolving the container question.

## Hero

**Source:** `src/pages/HomePage.tsx:83-97`.

`hero min-h-[60vh] bg-base-200 rounded-lg` contains centered `hero-content`, `max-w-2xl` copy, 128px avatar/ring, 48px bold heading, 18px description, and `btn btn-primary`. Static; no custom hero states. Keep it framed, not full-bleed photographic.

## Post card

**Source:** `HomePage.tsx:123-147`; `CategoryDetailPage.tsx:164-189`; related variant `PostDetailPage.tsx:261-289`.

Anatomy: `card bg-base-200 shadow-xl`; optional 192px `figure` thumbnail; `card-body`; `card-title`; optional badge/date; preview; `card-actions justify-end`; `btn btn-primary btn-sm`. Home previews truncate at 100 chars, category at 120, related at 80. Keep CTA right-aligned; do not add hover lift by default.

## Category card and stats

**Source:** `src/pages/CategoriesPage.tsx:104-166`.

Category cards omit imagery and add a rotating semantic badge (`primary`, `secondary`, `accent`, `info`, `success`, `warning`), title, slug-derived description, and primary small CTA. They use `hover:shadow-2xl transition-shadow`. Stats use `stats stats-vertical lg:stats-horizontal shadow w-full mt-12`, with `stat-title`, `stat-value`, and `stat-desc`; popular value is `text-primary`.

## Article detail

**Source:** `src/pages/PostDetailPage.tsx:153-293`.

A `text-sm breadcrumbs` trail precedes the left-aligned 48px title and subdued metadata. Optional featured image is full-width and 384px high. The article body uses the custom `.article-prose` contract (no Tailwind Typography); highlight.js applies GitHub Dark to code after render. Sharing is a divider plus centered gap-4 circular outline buttons with 24px filled inline SVGs. Related posts reuse a responsive card grid.

## Feedback and primitives

- Loading: centered `loading loading-spinner loading-lg`.
- Error: `alert alert-error` with localized prefix/message.
- Empty/not-found: `alert alert-info`.
- Breadcrumbs: DaisyUI `breadcrumbs` with list items and plain anchors.
- Badges: `badge-primary` or six-class semantic rotation.
- Divider: `divider` with localized share label.
- Language menu: `dropdown dropdown-end`; `btn btn-sm btn-ghost` trigger; `dropdown-content z-[1] menu p-2 shadow bg-base-100 rounded-box w-32`.
- No skeleton, retry CTA, toast, inline validation, or offline state is present.

## Code blocks

Tailwind Typography is enabled in `src/App.css` but is intentionally not used by the article body; the `.article-prose` contract in the same file is the source of truth. highlight.js GitHub Dark is imported (`PostDetailPage.tsx:5-6`). No app-owned code radius, padding, font, or copy control exists; retain dependency delegation until approved.
