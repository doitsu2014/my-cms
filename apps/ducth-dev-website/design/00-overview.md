# 00 — Overview

## Summary

Ducth Dev Website is a compact bilingual (`en`/`vi`) content-first blog/portfolio site. Its visual language is a clean DaisyUI “emerald” theme: full-width base surfaces, a compact elevated header, centered hero and page introductions, generous vertical separation, semantic accent badges, responsive post-card grids, and an article view that gives TipTap HTML and dark syntax-highlighted code the strongest reading hierarchy. The implementation favors framework primitives over bespoke CSS; the only app-authored global CSS sets Roboto and basic document height.

## Core principles

- **Semantic color over hard-coded color:** use DaisyUI roles (`primary`, `base-200`, `error`, `info`).
- **Content-first hierarchy:** bold headings, readable prose, thumbnails as previews, subdued metadata.
- **Progressive disclosure:** cards summarize; detail pages reveal full content, sharing, and related posts.
- **Predictable responsive density:** one column narrow, two at `md`, three at `lg`.
- **Quiet elevation:** surface contrast plus `shadow-xl`/`shadow-lg`; no bespoke gradients.
- **International parity:** language changes content, not the visual structure.

## How to use this folder

1. Start here, then use `tokens.json` when implementing reusable components.
2. Treat values as **Observed** behavior; treat prose labeled guidance as reuse rules, not new source tokens.
3. Do not convert unresolved framework defaults into product tokens without confirming `open-questions.md`.
4. Compose the existing shell, `base-100` page surface, `base-200` elevated surface, `space-y-8` rhythm, and semantic DaisyUI components before adding CSS.
5. Check responsive and accessibility guidance together.

## Source boundary

`tailwind.config.ts` has no custom theme extensions, `src/App.css` has no CSS custom properties, and `public/` has no assets. Most visual values are inherited from Tailwind 4 and DaisyUI 5; this is evidence, not an assumed gap.
