# 05 — Iconography

## Findings

`package.json` has no Lucide, Heroicons, React Icons, or other icon-library dependency. Navigation/footer use text labels (`Footer.tsx:17-43`). Post sharing uses three inline SVG brand marks with `viewBox="0 0 24 24"`, `h-6 w-6` / `24px`, and `fill="currentColor"` (`PostDetailPage.tsx:219-247`).

| Element | Size | Fill/stroke | Source |
|---|---:|---|---|
| Social share SVG | 24px | Filled paths, `currentColor`, no stroke | `PostDetailPage.tsx:219-247`. |
| Avatar/media | Image, not icon | `<img>` | `Header.tsx:39`; `HomePage.tsx:89`. |
| Loading indicator | DaisyUI `loading-lg` | Framework-defined | Loading branches in all pages. |

## Reuse rules

- The app has no current icon-system token. Choose and document one before introducing a broad icon vocabulary.
- To match current visuals, use filled social marks in `btn btn-circle btn-outline`, with visible graphic size 24px and `currentColor`.
- Do not replace the visible text navigation/footer labels with decorative icons.
- Every icon-only control needs an accessible name; the current share buttons have no `aria-label` and are a known gap.

## Naming

No reusable icon component or naming convention exists. Future semantic names such as `ShareFacebook`, `ShareTwitter`, and `ShareLinkedIn` should be introduced only once the share actions are confirmed functional.
