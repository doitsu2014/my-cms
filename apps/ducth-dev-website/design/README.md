# Ducth Dev Website — Design Language

Implementation reference for the visual and interaction language observed in this app. It separates app-authored decisions from inherited Tailwind/DaisyUI behavior and cites evidence paths.

## Contents

| File | Summary |
|---|---|
| `00-overview.md` | Visual summary, principles, and usage of this reference. |
| `01-colors.md` | Semantic palette, compiled light/dark values, and color rules. |
| `02-typography.md` | Roboto stack, observed type scale, and rhythm. |
| `03-spacing-layout.md` | Spacing, containers, breakpoints, grids, and rhythm. |
| `04-borders-shadows.md` | Radii, shadows, borders, rings, and blur findings. |
| `05-iconography.md` | Icon sources, sizing, fill/stroke, and naming. |
| `06-imagery-media.md` | Avatar, thumbnails, featured image, and media rules. |
| `07-motion.md` | Transitions, hover behavior, and motion gaps. |
| `08-components.md` | Recurring UI patterns and implementation contracts. |
| `09-responsive.md` | Breakpoint matrix and responsive adaptations. |
| `10-accessibility.md` | Observed posture, target criteria, and gaps. |
| `tokens.json` | Machine-readable token map with source paths. |
| `open-questions.md` | Ambiguities for the original designer/product owner. |

## Evidence scope

Audited `tailwind.config.ts`, `postcss.config.mjs`, `src/App.css`, `index.html`, `package.json`, all application `.tsx` files under `src/`, and the absence of `public/` assets. Compiled DaisyUI values are identified separately from source declarations and come from `dist/client/static/css/index.6528c7ae.css`.
