## Why

Article detail pages currently force featured thumbnails into a 21:9 frame and crop them with `object-fit: cover`. Many stored thumbnails are closer to 3:2, so important content at the top and bottom is hidden and the image appears incorrectly resized. The detail page should show the complete thumbnail at the available content width while preserving its source aspect ratio.

## What Changes

- Change the article detail featured-image behavior to preserve the source image aspect ratio and render the complete image without vertical cropping.
- Keep fixed-aspect-ratio, cover-cropped artwork for home, category, related, and compact post cards.
- Preserve the deterministic fallback artwork behavior when a post has no thumbnail.
- Add regression coverage for the detail-image presentation contract and verify the layout at desktop and narrow viewports.
- Do not change media URLs, API responses, storage, or server-side image transformation behavior.

## Capabilities

### New Capabilities

None.

### Modified Capabilities

- `website-reader-experience`: change the post reading experience so a featured thumbnail on `/:lang/posts/:slug` preserves the source aspect ratio instead of being required to render at 21:9; card and fallback artwork aspect-ratio behavior remains unchanged.

## Impact

- Frontend components and styles under `apps/ducth-dev-website/src/`, primarily `PostDetailPage`, `PostArtwork`, `EditorialImage`, and `App.css`.
- Frontend tests for the post reading surface and artwork presentation.
- No API, database, media-storage, dependency, or deployment changes.
