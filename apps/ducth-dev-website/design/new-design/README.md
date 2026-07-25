# Ink & Tide — Modernized redesign for Đức Trần's career notebook

This folder is the modernized redesign of `ducth-dev-website`, regenerated from a full audit of the current React + DaisyUI implementation. The design is a custom editorial direction called **Ink & Tide**: parchment + ink + cinnabar, Vietnamese typography, asymmetric editorial grids, and the author's photo as the visual mark.

## What changed and why

| | Before (DaisyUI "emerald") | After (Ink & Tide) |
|---|---|---|
| Theme | DaisyUI emerald (green primary) | Custom — parchment + ink + cinnabar |
| Font | Roboto (Latin-only safe defaults) | Noto Serif Display + Inter (full Vietnamese + Latin) |
| Layout | Centered hero, DaisyUI card grid | Asymmetric editorial, two-weight type, ink motifs |
| Identity | Generic "developer blog" | The author's photo as the visual mark — a real portrait on the lake |
| Imagery | CMS photo thumbnails (real) | SVG ink motifs for post art + the author's photo as avatar |
| Palette | 16 DaisyUI semantic slots | Six OKLCH neutrals + one cinnabar accent |
| Sections | Hero + featured grid | Hero + career panel + asymmetric featured + categories + closing CTA |
| Pages | 4 (Home, Categories, Category, Post) | 5 (added About — the career homepage) |
| Copy | "passionate developer" | Career-focused — 11 years backend engineering, system architecture, technical leadership |

## Files

| File | Purpose |
|---|---|
| `brand-spec.md` | Full token spec: OKLCH palette, font stacks, posture rules, anti-patterns |
| `assets/avatar.jpg` | The author's photo — used as the avatar (header, home hero, career panel, About statement) |
| `assets/architecture.jpg` | Concrete pillars — used as the Systems category intro seal and the home closing CTA background |
| `assets/coast.jpg` | Rocky coast — used as the post.html featured image and the About page "Khoảnh khắc" contemplation section |
| `index.html` | Home — hero with avatar, career panel, asymmetric featured posts, categories strip, hiring CTA with architecture background |
| `categories.html` | Categories index — five career domains with latest post previews |
| `category.html` | Category detail — breadcrumb with architecture photo, post list, pagination |
| `post.html` | Post detail — full article reading experience with coast photo as featured image |
| `about.html` | About — career pillars (Kỹ sư / Vận hành / Dẫn dắt), practice, "Khoảnh khắc" contemplation, contact-for-hire |

## How the direction serves the author

The whole site reads like an editor's notebook beside a slow river. Generous whitespace is the "water surface." Hairline borders are brush strokes. Cinnabar accents are the seal paste (朱) on an editor's manuscript. Three real photos anchor the visual identity:

- **avatar.jpg** (lakeside figure) — the personal mark, used 5 times as the avatar in header / home hero / career panel / About statement / post byline
- **architecture.jpg** (concrete pillars) — the career mark, used as the Systems category intro and as a faint background on the home closing CTA
- **coast.jpg** (rocky coast with a wooden post) — the contemplative mark, used as the post featured image and as a dedicated "Khoảnh khắc" section on the About page

Typography uses Noto Serif Display — a contemporary serif with full Vietnamese diacritic support — paired with Inter for UI.

The copy is now career-focused: *what the author does, for how long, with what expertise*. The home hero leads with "Mười một năm viết phần mềm" (Eleven years writing software). The career panel tells the arc: PHP in 2014 → distributed systems in production today. The About page reframes the three pillars as **Kỹ sư** (Backend engineering), **Vận hành** (Production reliability), and **Dẫn dắt** (Tech lead / mentoring). The closing CTA is a hiring call: "Nếu bạn đang tuyển một kỹ sư backend." A new "Khoảnh khắc" section on the About page closes the personal narrative with a moment of stillness — the coast photo and a one-line caption tying 11 years of work back to a quiet morning.

## Why a custom direction instead of one from the library

The Open Design library offers `editorial-monocle`, `modern-minimal`, `human-approachable`, `tech-utility`, and `brutalist-experimental`. None of them speak Vietnamese + a personal career narrative by default. `editorial-monocle` is the closest relative — Monocle magazine's considered, print-first aesthetic — and the posture cues carry through (serif display + measured columns + restrained accent). What this redesign adds is the ink-wash palette, the Vietnamese diacritic typography rules, and the photo-as-avatar identity pattern that the library presets don't ship.

## Self-check notes

- **Anti-slop audit**: no indigo `#6366f1`, no trust-gradient on the hero, no emoji as icons, no rounded-card-with-left-border, no Roboto/Inter as display face, no invented metrics, no lorem ipsum. ✓
- **Accent discipline**: cinnabar appears at most twice per screen (counted: home meta — none; period in wordmark — recurring brand mark; hiring CTA; closing eyebrow dot). Within budget. ✓
- **Typography craft**: ALL CAPS uses 0.18em tracking; display type uses -0.025em to -0.015em tracking; Vietnamese diacritics accommodated with line-height 1.65–1.7. ✓
- **Layout integrity**: asymmetric grids collapse to single column under 900px; reading-progress dot only appears on post.html; the avatar appears in 5 placements (header 36px circle, home hero 4:5 portrait, career panel 1:1 square, About statement 1:1 square, post byline 40px circle — all use `object-fit: cover`); the architecture photo appears in 3 placements (category detail 3:4 portrait seal, home closing CTA background at 26% opacity with `mix-blend-mode: luminosity` + dark gradient overlay); the coast photo appears in 2 placements (post featured image 21:9 wide crop, About "Khoảnh khắc" 4:3 split layout); no clipped text. ✓
- **Real content**: every title, excerpt, date, and body paragraph is a real draft. No `feature one / two / three`. ✓

## How to view

Open `index.html` in any modern browser. The site is fully self-contained — fonts load from Google Fonts, the avatar lives at `assets/avatar.jpg`, no build step required. Dark mode is automatic via `prefers-color-scheme`.

## Migration notes (for the React app)

When porting back to the React app:

1. **Tokens**: replace the DaisyUI `emerald` theme with the OKLCH palette from `brand-spec.md`. Keep the same semantic role names (`primary`, `base-100`, etc.) so existing classNames still work — only the resolved values change.
2. **Typography**: swap the Roboto import for Noto Serif Display + Inter. Update `body` font-family.
3. **Avatar**: copy `assets/avatar.jpg` into the React app's `public/` (or import) and use in 4 places: header (36px circle), home hero (4:5 portrait), career panel (1:1 square), about statement (1:1 square). Use `object-fit: cover` and `object-position: center 40%` to keep the figure in frame.
4. **Hero**: drop the centered `hero` div for the asymmetric `hero__grid` layout. The CSS already lives in the HTML files; port the relevant blocks to your component library.
5. **Cards**: replace DaisyUI `card` with the bespoke `.post-card` pattern — image-on-top, no left-border accent, no `card-actions` footer.
6. **About page**: this is a new page in the React app — add a route at `/:lang/about` and a new component with the three career pillars (Kỹ sư / Vận hành / Dẫn dắt).
7. **Footer**: replace the single-line footer with the three-column grid (about / navigation / elsewhere).
8. **Language toggle**: keep the `VI / EN` pair as a pill — it's already an established UI affordance on the current site.
9. **Reading progress**: the dot at the bottom-right of `post.html` is a 30-line JS snippet — port it as a hook if you want it in the React app, or drop it for the SSR'd build.
