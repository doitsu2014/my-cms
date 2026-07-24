# 09 — Responsive Behavior

## Strategy

The implementation is mobile-first: base classes define narrow layout and `md:`/`lg:` add columns or horizontal stats. No `sm:`, `xl:`, or `2xl:` behavior is observed. Breakpoints inherit Tailwind because `tailwind.config.ts:5` has no custom screens.

## Breakpoint matrix

| Range | Shell/header | Collections | Stats | Behavior | Evidence |
|---|---|---|---|---|---|
| `<48rem` / `<768px` | `container px-4`; one flex-row header; menu remains horizontal and may wrap with long translations. | One column, `gap-6`. | Vertical. | 60vh hero; 48px headings can wrap; image remains 384px high. | `Layout.tsx:13`; `Header.tsx:36-67`; page grids. |
| `48rem–<64rem` / `768–1023px` | Same shell; no nav collapse. | Standard grids become two columns; related becomes three. | Still vertical. | Two-column cards preserve 24px gap. | `HomePage.tsx:117`; `CategoriesPage.tsx:99`; `CategoryDetailPage.tsx:158`; `PostDetailPage.tsx:255`. |
| `≥64rem` / `≥1024px` | Container widens; same header pattern. | Standard grids become three columns. | `stats-horizontal`. | Three-column collection density. | `CategoriesPage.tsx:99,132`; other page grids. |

## Rules

- No hamburger/mobile menu exists; validate bilingual header labels at narrow widths.
- Hero has no breakpoint-specific classes; content remains centered in `max-w-2xl` with fixed avatar width.
- Cards collapse columns but retain anatomy and image frame height.
- Metadata uses `flex-wrap` (`PostDetailPage.tsx:176`); article body remains `max-w-none`.
- Footer `footer-center` and `grid-flow-col` are not customized for narrow screens.
- No header, breadcrumb, share, or stats element is sticky.

## QA

At narrow width verify header controls remain reachable, 48px headings do not create horizontal overflow, 384px imagery fits its container, and social buttons have adequate targets. At medium/wide widths verify grid transitions and that card actions remain right-aligned.
