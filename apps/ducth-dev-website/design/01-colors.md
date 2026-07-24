# 01 — Colors

## Color architecture

`src/App.css:9-11` declares DaisyUI themes `emerald --default, dark`; `index.html:2` and `src/index.client.tsx:7` set `data-theme="emerald"`. Components use semantic classes rather than literal colors. The hex values below are compiled output for the current build, clearly marked as generated evidence.

### Emerald/default compiled palette

| Role | Hex swatch | Current use | Source |
|---|---|---|---|
| `primary` | `#66cc8a` | CTAs, category badges, avatar ring, highlighted stat. | `src/App.css:9-11`; `src/pages/HomePage.tsx:88,94,136,142`; `src/pages/CategoriesPage.tsx:153`; `src/pages/PostDetailPage.tsx:185`; compiled CSS. |
| `primary-content` | `#223d30` | DaisyUI paired foreground. | `dist/client/static/css/index.6528c7ae.css`; not directly authored. |
| `secondary` | `#377cfb` | Semantic category-color rotation. | `src/pages/CategoriesPage.tsx:65-67`; compiled CSS. |
| `accent` | `#f68067` | Semantic category-color rotation. | `src/pages/CategoriesPage.tsx:65-67`; compiled CSS. |
| `neutral` | `#333c4d` | DaisyUI neutral role; no direct app class observed. | Compiled CSS. |
| `base-100` | `#ffffff` | Root page and dropdown surface. | `src/components/layout/Layout.tsx:11`; `Header.tsx:55`; compiled CSS. |
| `base-200` | `#e8e8e8` | Header, footer, hero, and cards. | `Header.tsx:35`; `Footer.tsx:10`; `HomePage.tsx:84,123`; `CategoryDetailPage.tsx:164`; compiled CSS. |
| `base-300` | `#d1d1d1` | Available surface tier; no direct class observed. | Compiled CSS. |
| `base-content` | `#333c4d` | Default content role. | Compiled CSS. |
| `info` | `#00b4fa` | Empty/no-data alerts. | `src/pages/HomePage.tsx:112-115`; `CategoriesPage.tsx:90-93`; `CategoryDetailPage.tsx:150-153`; `PostDetailPage.tsx:145-148`; compiled CSS. |
| `success` | `#00a96e` | Category-color rotation option. | `src/pages/CategoriesPage.tsx:65-67`; compiled CSS. |
| `warning` | `#ffc100` | Category-color rotation option. | `src/pages/CategoriesPage.tsx:65-67`; compiled CSS. |
| `error` | `#ff676a` | Query/API error alerts. | `src/pages/HomePage.tsx:107-110`; `CategoriesPage.tsx:82-87`; `CategoryDetailPage.tsx:142-147`; `PostDetailPage.tsx:137-143`; compiled CSS. |

### Dark theme compiled palette

Dark is declared and compiled but not exposed by current bootstrap.

| Role | Dark hex | Source |
|---|---|---|
| `primary` / `primary-content` | `#605dff` / `#edf1fe` | Compiled CSS; theme declaration `src/App.css:10`. |
| `secondary` / `secondary-content` | `#f43098` / `#f9e4f0` | Compiled CSS; theme declaration. |
| `accent` / `accent-content` | `#00d1bb` / `#084d49` | Compiled CSS; theme declaration. |
| `neutral` / `neutral-content` | `#09090b` / `#e4e4e7` | Compiled CSS. |
| `base-100` / `base-200` / `base-300` | `#1d232a` / `#191e24` / `#15191e` | Compiled CSS. |
| `base-content` | `#f2f8ff` | Compiled CSS. |
| `info` / `info-content` | `#00bafc` / `#042e49` | Compiled CSS. |
| `success` / `success-content` | `#00d193` / `#004c39` | Compiled CSS. |
| `warning` / `warning-content` | `#f9b800` / `#793205` | Compiled CSS. |
| `error` / `error-content` | `#ff657f` / `#4d0218` | Compiled CSS. |

## Gradients and transparency

No gradient, alpha color, custom opacity color, or CSS variable is authored in `src/App.css` or application TSX. Secondary text uses `opacity-70`/`opacity-80` (`CategoriesPage.tsx:73,114`; `CategoryDetailPage.tsx:131,176`; `PostDetailPage.tsx:176`).

## Usage rules

- `base-100` is for page/dropdown surfaces; `base-200` is for chrome, hero, and cards.
- Use `primary` for main actions and content-category emphasis; semantic alternatives are feedback or category rotation, not arbitrary decoration.
- Pair a semantic surface with its DaisyUI `*-content` role; do not manually invent foreground colors.
- Test subdued opacity text in both themes before reuse.
