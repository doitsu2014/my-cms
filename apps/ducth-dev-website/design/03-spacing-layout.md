# 03 — Spacing & Layout

## Source and unit

The app uses Tailwind’s default spacing scale; `tailwind.config.ts:5` does not extend it. Base unit is `0.25rem` / `4px`.

| Utility | Value | Use | Source |
|---|---:|---|---|
| `px-1` | 4px | Horizontal menu padding. | `Header.tsx:43`. |
| `p-2` | 8px | Language dropdown padding. | `Header.tsx:55`. |
| `px-4` | 16px | Header/main gutter. | `Header.tsx:36`; `Layout.tsx:13`. |
| `py-2` | 8px | Compact header vertical padding. | `Header.tsx:36`. |
| `py-6` | 24px | Hero description vertical padding. | `HomePage.tsx:93`. |
| `py-8` | 32px | Main content vertical padding. | `Layout.tsx:13`. |
| `p-10` | 40px | Footer padding. | `Footer.tsx:10`. |
| `gap-4` | 16px | Header controls, metadata, social links. | `Header.tsx:42`; `PostDetailPage.tsx:176,218`; `Footer.tsx:18`. |
| `gap-6` | 24px | Card grids. | `HomePage.tsx:117`; `CategoriesPage.tsx:99`; `CategoryDetailPage.tsx:158`; `PostDetailPage.tsx:255`. |
| `space-y-4` | 16px | Intro/article-header rhythm. | `CategoriesPage.tsx:71`; `PostDetailPage.tsx:174`. |
| `space-y-8` | 32px | Page section rhythm. | All four page components. |
| `mt-8` / `mt-12` | 32px / 48px | Categories grid, stats, related section. | `CategoriesPage.tsx:99,132`; `PostDetailPage.tsx:253`. |

## Container and grid

- Header and main use `container mx-auto px-4` (`Header.tsx:36`, `Layout.tsx:13`). The main has no explicit custom max width; it inherits Tailwind `container`.
- Hero content is `max-w-2xl` (`HomePage.tsx:86`); article content is intentionally `max-w-none` (`PostDetailPage.tsx:212`).
- Standard collections are `grid-cols-1 md:grid-cols-2 lg:grid-cols-3 gap-6`; related content is `grid-cols-1 md:grid-cols-3`.
- Stats are vertical by default and `lg:stats-horizontal` at wide screens (`CategoriesPage.tsx:132`).

## Breakpoints

No custom screens exist. Inherited Tailwind defaults are `md = 48rem / 768px` and `lg = 64rem / 1024px`; verify installed framework output when porting because these are not app-owned constants.

## Layout rules

- Narrow: one card per row, 16px shell gutters, wrapping titles/previews.
- Medium: two standard cards per row; three related cards.
- Wide: three standard cards per row; horizontal category stats.
- Preserve `space-y-8` and main `py-8` before introducing page-specific spacing.
- Do not add a fixed shell width without resolving the inherited-container question.
