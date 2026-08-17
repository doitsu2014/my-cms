## 1. Add explicit artwork presentation modes

- [x] 1.1 Extend `EditorialImage` with an explicit natural-image mode that omits the forced aspect-ratio style for real images while preserving the current cover behavior by default.
- [x] 1.2 Extend `PostArtwork` with the natural-image mode, applying it only when a thumbnail source exists and retaining the supplied aspect ratio for deterministic fallback artwork.

## 2. Update detail-page presentation

- [x] 2.1 Update `PostDetailPage` to request natural thumbnail presentation for the featured article image without changing card, row, or related-post call sites.
- [x] 2.2 Add a dedicated natural-image modifier style that renders the detail thumbnail at full available width with automatic height and no cover crop, while retaining existing border, overflow, and card styles.

## 3. Add regression coverage

- [x] 3.1 Add component/page assertions that a real detail thumbnail uses natural presentation and does not receive the fixed 21:9 image frame contract.
- [x] 3.2 Add assertions that missing thumbnails still render the deterministic fallback with its requested aspect ratio and that card artwork keeps the existing cover-mode default.

## 4. Verify the website change

- [x] 4.1 Run the website test suite and typecheck; the installed local Vitest and TypeScript binaries were used because the pnpm shim could not verify its cached signature.
- [x] 4.2 Run the website production build with the installed Rsbuild binary and inspect the built website for errors; the pnpm shim could not verify its cached signature.
- [ ] 4.3 Perform a browser smoke check of an article detail page at desktop and narrow viewport widths, confirming the full thumbnail is visible, proportional, and not stretched while card crops remain unchanged.
