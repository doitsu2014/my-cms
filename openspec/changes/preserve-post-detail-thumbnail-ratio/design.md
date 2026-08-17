## Context

The public reader is implemented in `apps/ducth-dev-website`. `PostDetailPage` passes `aspect="21 / 9"` to `PostArtwork`. When a thumbnail exists, `PostArtwork` delegates to `EditorialImage`, which writes the aspect ratio onto the `<figure>`. The shared stylesheet then sets the image to `width: 100%; height: 100%; object-fit: cover`, so source images that are taller than 21:9 are cropped inside an overflow-hidden frame.

The same artwork components are also used by home, category, and related-post cards, where fixed editorial crops are intentional. The change must therefore be scoped to the detail-page presentation and must not alter media URLs, API contracts, storage, or server-side transformations.

## Goals / Non-Goals

**Goals:**

- Show the complete detail-page thumbnail at the available content width.
- Preserve the source image's intrinsic aspect ratio without stretching or cropping.
- Keep existing cover-cropped aspect ratios for post cards, article rows, related posts, and fallback motifs.
- Make the presentation distinction explicit in the component API and regression-test it.

**Non-Goals:**

- Changing stored media dimensions or introducing a new image transformation endpoint.
- Changing thumbnail URL resolution, GraphQL fields, API responses, or Supabase Storage behavior.
- Redesigning the detail-page header, article body, or other editorial artwork.

## Decisions

### Use an explicit natural-image presentation mode

Extend `PostArtwork` and `EditorialImage` with a presentation option whose default remains the current cover behavior. The detail page opts into a natural mode. In natural mode, a real thumbnail does not receive a forced aspect-ratio style; its image remains `width: 100%` with `height: auto`. This keeps the API explicit and prevents the detail-page fix from silently changing card layouts.

When the detail page has no thumbnail, the CSS fallback continues to use the requested fixed aspect ratio and deterministic slug-based variant. The natural mode applies only to a real source image.

### Keep the current cover behavior as the default

The existing card variants depend on stable aspect ratios for grid alignment and editorial composition. Their call sites should not change, and the shared cover styling should remain the default so this bug fix has a narrow visual blast radius.

### Treat the existing media URL as the original source

The frontend will continue to use the URL returned by `getPostThumbnail`. No resize query or API change is needed: CSS width scaling is sufficient once the forced 21:9 crop is removed.

### Prefer intrinsic sizing over `object-fit: contain`

Using `contain` inside the existing 21:9 frame would preserve pixels but leave letterboxed space because a typical thumbnail is closer to 3:2. Removing the forced frame for natural images provides full-width rendering without artificial bands.

## Risks / Trade-offs

- [Variable image heights] → Detail pages may become taller or differ in height across posts; this is the expected consequence of preserving source proportions and is preferable to hiding image content.
- [CSS regression] → A shared selector could accidentally affect cards; keep natural styling behind a dedicated modifier class and add DOM-level component tests for both modes.
- [Visual verification gap] → JSDOM cannot calculate rendered image dimensions; supplement unit/component tests with the website typecheck, build, and a desktop/mobile browser smoke check.

## Migration Plan

No data migration or deployment sequencing is required. Ship the frontend component/style/test change with the normal website build. Rollback is a frontend-only revert if the natural presentation causes an unacceptable editorial regression.

## Open Questions

None. The approved direction is to preserve the full thumbnail on detail pages while retaining fixed crops elsewhere.
