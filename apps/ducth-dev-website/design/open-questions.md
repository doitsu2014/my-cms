# Open Questions

1. **Dark mode:** `src/App.css:10` declares `dark`, but `index.html:2` and `src/index.client.tsx:7` force `emerald`. Is a theme switch intended?
2. **Palette authority:** hex values in `01-colors.md` come from generated `dist/client/static/css/index.6528c7ae.css`, not app-authored source. Should colors be pinned in an app-owned theme file?
3. **Container width:** `container mx-auto` has no custom max width (`Layout.tsx:13`, `Header.tsx:36`). Is inherited Tailwind width intentional?
4. **Typography metrics:** no explicit body size, line-height, or tracking is authored. Should these be locked for long Vietnamese articles?
5. **Icon system:** no icon dependency exists. Should future controls adopt Lucide, another library, or inline SVG?
6. **Share actions:** circular social buttons have SVGs but no `onClick`, `href`, or accessible name (`PostDetailPage.tsx:218-248`). Are they placeholders?
7. **Narrow navigation:** header menu never collapses (`Header.tsx:42-49`). Is bilingual overflow acceptable?
8. **Runtime imagery:** no local `public/` assets; avatar/thumbnails come from runtime/API. Is a fallback/brand asset policy needed?
9. **OG/SEO:** `index.html` has an empty head and no favicon/OG image. Should these conventions be added?
10. **Article media:** `prose prose-lg` delegates TipTap media styling; there are no custom aspect ratios/captions. Is that intentional?
11. **Recovery:** spinner and alerts have no retry, skeleton, offline, or aria-live pattern. What should future content pages use?
12. **Motion timing:** `transition-shadow` has no explicit duration/easing (`CategoriesPage.tsx:107`). Should timing be formalized?
13. **Focus token:** which focus-ring color/width should be standardized after contrast testing?
14. **Hashed evidence:** compiled CSS filename changes after builds. Should a stable extracted palette artifact be checked in?
