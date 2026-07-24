# 10 — Accessibility

## Observed strengths

- Document `lang="en"`; routes and translations support `en`/`vi` (`index.html:2`; `AppContent.tsx:15-19`).
- Semantic landmarks are present: header, main, footer, section, article (`Header.tsx:34`; `Layout.tsx:13`; `Footer.tsx:9`; page components).
- Content images have meaningful `alt` text (`Header.tsx:39`; `HomePage.tsx:89,127-130`; `PostDetailPage.tsx:204-207`).
- Navigation uses native anchors; external footer links use `noopener noreferrer` (`Footer.tsx:19-40`).
- Loading, error, empty, and not-found states are visually distinct DaisyUI components.

These observations are not a WCAG conformance claim.

## Target criteria for reuse

| Area | Evidence/gap | Requirement |
|---|---|---|
| Keyboard | Native links/buttons and focusable dropdown label (`Header.tsx:52,55`). | Preserve DOM order and verify dropdown open/navigate/close. Target WCAG 2.2.1, 2.4.3. |
| Focus | No app-authored focus ring; avatar ring is decorative. | Provide a persistent focus indicator with at least 3:1 focus-versus-adjacent contrast. Target WCAG 2.4.7/2.4.11. |
| Control names | SVG-only share buttons have no accessible name (`PostDetailPage.tsx:219-247`). | Add localized `aria-label` or visible labels. Target WCAG 4.1.2. |
| Images | Post/avatar alts exist. | Keep meaningful alternatives; empty alt only for decorative images. Target WCAG 1.1.1. |
| Contrast | DaisyUI content roles exist; metadata uses opacity. | Test localized text in both compiled themes; do not assume opacity passes. Target WCAG 1.4.3. |
| Structure | Main landmark and prominent page heading are observed. | Keep one logical h1, ordered levels, and labeled article/sections. Target WCAG 1.3.1, 2.4.6. |
| Touch | 24px SVGs in DaisyUI buttons; no explicit touch token. | Verify icon-only share controls reach 44×44 CSS px including padding. Target WCAG 2.5.8. |
| Motion | No reduced-motion handling. | Gate future non-essential motion with `prefers-reduced-motion: reduce`. Target WCAG 2.3.3. |
| Feedback | Conditional states have no live-region attributes. | Announce loading completion/errors when adding dynamic updates. Target WCAG 4.1.3. |

## Theme/language guidance

`emerald` is active; `dark` is compiled but not selectable. Test `base-200` and `opacity-70` text in both. Meaning must not rely on color alone. The HTML root remains `lang="en"` even for Vietnamese routes; future implementation should synchronize it with `currentLang`.
