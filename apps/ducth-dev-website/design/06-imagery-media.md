# 06 — Imagery & Media

## Asset inventory

No files exist under `public/`. Avatar and thumbnail media are runtime/API values: `SITE_CONFIG.avatarUrl` is runtime-configured (`src/config/site.config.ts:3-11`), and thumbnail paths are resolved by `getMediaUrl` (`HomePage.tsx:41-50`, with equivalent helpers in category/post pages).

## Treatments

| Role | Treatment and dimensions | Source |
|---|---|---|
| Header avatar | `w-10` = 40px, inner `rounded-full`; link is `btn btn-ghost btn-circle avatar`. | `src/components/layout/Header.tsx:37-40`. |
| Hero avatar | `w-32` = 128px, `rounded-full`, `ring-primary`, `ring-offset-2`, `ring-offset-base-100`; `mb-6` = 24px. | `src/pages/HomePage.tsx:87-89`. |
| Card thumbnail | `w-full h-48` = 192px high, `object-cover`; no explicit radius/aspect-ratio. | `HomePage.tsx:124-130`; `CategoryDetailPage.tsx:165-171`; `PostDetailPage.tsx:265-271`. |
| Article featured image | `w-full h-96` = 384px high, `object-cover`; parent `rounded-lg overflow-hidden`. | `src/pages/PostDetailPage.tsx:200-208`. |
| Article HTML | `prose prose-lg max-w-none`; no custom media CSS. | `PostDetailPage.tsx:211-214`; `src/App.css:8`. |

## Behavior

The first thumbnail path is used. Full `http://`/`https://` paths bypass URL construction; other paths use the configured media base URL. Post image alts use translated titles; avatar alts are explicit (`Duc Tran` / `Duc Tran’s Blog`). `object-cover` prioritizes consistent card rhythm over complete image display.

No image gradients, overlays, lazy-loading attributes, aspect-ratio utilities, captions, OG image files, or local brand assets are present.

## Hero convention

The home hero is a DaisyUI `hero min-h-[60vh] bg-base-200 rounded-lg` with centered `hero-content` and `max-w-2xl`. It stacks the 128px avatar, 48px welcome heading, 18px description, and primary CTA (`HomePage.tsx:83-97`). It is a framed introduction, not a full-bleed image hero.

## OG/SEO

`index.html` has an empty `<head>` and no favicon, OG image, or image metadata (`index.html:1-7`). Runtime config has site metadata, but no visual asset standard is authored; confirm before adding one.

