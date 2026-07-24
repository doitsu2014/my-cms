# 04 — Borders, Radii & Shadows

| Utility | Concrete value/behavior | Use | Source |
|---|---|---|---|
| `rounded-lg` | `0.5rem` / `8px` | Hero panel and article image clipping. | `HomePage.tsx:84`; `PostDetailPage.tsx:202`. |
| `rounded-full` | Full radius | Header and hero avatars. | `Header.tsx:38`; `HomePage.tsx:88`. |
| `rounded-box` | DaisyUI component radius | Language dropdown. | `Header.tsx:55`; compiled DaisyUI CSS. |
| `shadow-lg` | Framework large-shadow preset | Header. | `Header.tsx:35`. |
| `shadow-xl` | Framework extra-large-shadow preset | Cards. | `HomePage.tsx:123`; `CategoriesPage.tsx:107`; `CategoryDetailPage.tsx:164`; `PostDetailPage.tsx:263`. |
| `shadow-2xl` | Framework 2XL preset | Category hover. | `CategoriesPage.tsx:107`. |
| `ring` | Framework ring | Hero avatar. | `HomePage.tsx:88`. |
| `ring-offset-2` | 8px offset | Hero avatar ring separation. | `HomePage.tsx:88`. |

No explicit border, border-color, box-shadow, backdrop-filter, blur, or glass CSS exists in `src/App.css`. `btn-outline` delegates borders to DaisyUI (`PostDetailPage.tsx:219-239`).

## Guidance

The language is soft but not heavily rounded: use `rounded-lg` for panels/media, `rounded-full` only for avatars, and DaisyUI radii for DaisyUI components. Use `shadow-lg` for persistent chrome, `shadow-xl` for cards, and `shadow-2xl` only for an interactive elevation state. Do not add gradients or glassmorphism as if they were existing tokens.

The hero avatar ring (`ring-primary ring-offset-base-100 ring-offset-2`) is decorative, not a documented focus style. Focus styling is inherited and needs verification; see `10-accessibility.md`.
