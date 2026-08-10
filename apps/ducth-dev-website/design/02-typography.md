# 02 — Typography

## Font stack

| Token | Value | Source |
|---|---|---|
| Primary family | `Roboto` | Google Fonts import and body declaration, `src/App.css:2,17`. |
| Fallbacks | `Inter, Avenir, Helvetica, Arial, sans-serif` | `src/App.css:17`. |
| Available weights | `100..900`, normal and italic | `src/App.css:2`. |
| Code font | Not app-authored; code styling comes from highlight.js GitHub Dark. | `src/pages/PostDetailPage.tsx:5-6`. |

There are no local fonts, `@font-face`, letter-spacing declarations, or typography CSS variables.

## Observed type scale

These are Tailwind default utility sizes; `tailwind.config.ts:5` has an empty `theme.extend`.

| Role | Size | Treatment/use | Source |
|---|---:|---|---|
| Body/default | Framework default; not explicitly sized | Normal body on all pages. | `src/App.css:14-18`. |
| `text-sm` metadata | `0.875rem` / `14px` | Dates, breadcrumbs, article metadata; often `opacity-70`. | `CategoryDetailPage.tsx:116,176`; `PostDetailPage.tsx:154,176`. |
| `text-lg` body | `1.125rem` / `18px` | Hero description. | `HomePage.tsx:93`. |
| `text-xl` subtitle | `1.25rem` / `20px` | Listing description/count, subdued. | `CategoriesPage.tsx:73`; `CategoryDetailPage.tsx:131`. |
| `text-3xl` section | `1.875rem` / `30px` | Featured/related headings, bold. | `HomePage.tsx:101`; `PostDetailPage.tsx:254`. |
| `text-5xl` page | `3rem` / `48px` | Hero, category, and post headings, bold. | `HomePage.tsx:92`; `CategoriesPage.tsx:72`; `CategoryDetailPage.tsx:130`; `PostDetailPage.tsx:175`. |
| related title | `text-lg` / `18px` | DaisyUI `card-title` override. | `PostDetailPage.tsx:275`. |

## Line-height and letter spacing

No explicit `leading-*`, `line-height`, `tracking-*`, or `letter-spacing` is present in app source. Preserve framework defaults; do not claim numeric custom metrics without confirmation.

## Typographic rhythm

| Layer | Size | Spacing | Rule |
|---|---:|---|---|
| Page title | 48px | Intro `space-y-4` = 16px | Bold; centered on listings, left on article. |
| Intro subtitle | 20px | 16px below title | `opacity-70`. |
| Section heading | 30px | `mb-6` = 24px before collection | Bold, left-aligned. |
| Hero copy | 18px | `py-6` = 24px vertical | Center in `max-w-2xl`. |
| Article | `.article-prose` (custom) | `max-width: 68ch` | Hand-rolled contract in `src/App.css` styles every TipTap node and mark; highlight.js styles code. Tailwind Typography is intentionally not used. |

Keep localized headings short enough to wrap gracefully at 48px, and treat metadata/counts as secondary information. The only observed uppercase transformation is the language trigger (`Header.tsx:53`).
