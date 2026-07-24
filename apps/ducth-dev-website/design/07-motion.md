# 07 — Motion & Micro-interactions

## Authored motion

Only category cards explicitly animate: `transition-shadow` changes `shadow-xl` to `shadow-2xl` on hover (`src/pages/CategoriesPage.tsx:105-108`). No duration/easing is specified, so timing is framework default, not a bespoke token.

| Interaction | Behavior | Timing | Source |
|---|---|---|---|
| Category card hover | `shadow-xl` → `shadow-2xl` | Framework default; no `duration-*`/`ease-*` | `CategoriesPage.tsx:107`. |
| Button/link hover | DaisyUI `btn` and `link-hover` states | Framework default | `Header.tsx:37,52`; `Footer.tsx:23,31,39`; page CTAs. |
| Dropdown | DaisyUI dropdown behavior | Framework-defined | `Header.tsx:51-67`. |
| Loading | DaisyUI spinner | Framework-defined | Loading branches in all pages. |

There are no scroll-triggered, entrance, parallax, transform, route, or custom-keyframe animations in source.

## Reduced motion

No `prefers-reduced-motion`, `motion-reduce:*`, or app-level reduced-motion handling is present (`src/App.css` and `src/**/*.tsx`). The current language is low-motion by default. Any new animation must provide a reduced-motion fallback; at minimum disable non-essential shadow/transform effects while retaining understandable loading feedback.

## Guidance

Keep transitions limited to the changed property. If a numeric duration or easing is added, document it as a new decision rather than attributing it to this site. Ensure keyboard focus has an equally visible state; hover-only affordances are insufficient.
