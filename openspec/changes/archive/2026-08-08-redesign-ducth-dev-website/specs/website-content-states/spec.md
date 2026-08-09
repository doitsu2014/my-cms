## ADDED Requirements

### Requirement: Page-level loading state

Every reader page SHALL render a layout-matched skeleton while the GraphQL query is in flight. The skeleton SHALL preserve the final layout's section structure and SHALL NOT collapse the page to a single spinner. The skeleton's container SHALL expose `aria-busy="true"` while the data is loading.

#### Scenario: Skeleton preserves section structure

- **WHEN** the home page is loading
- **THEN** the page renders placeholder sections whose dimensions match the loaded page, and the page does not collapse to a centered spinner

#### Scenario: Loading container is marked busy

- **WHEN** the query is in flight
- **THEN** the loading container has `aria-busy="true"` and the screen reader announces the page as loading

### Requirement: Page-level error state

When the GraphQL query fails, the page SHALL render a localized error message, a retry button, and the shared layout shell. The page SHALL NOT render the raw GraphQL error message. The retry button SHALL refetch the current query.

#### Scenario: Error is localized and actionable

- **WHEN** the query fails
- **THEN** the page renders a localized error message and a Retry button that refetches the query

#### Scenario: Raw error is not shown

- **WHEN** the query fails
- **THEN** the rendered text does not contain the raw GraphQL error message

### Requirement: Page-level empty state

When the GraphQL query returns no items, the page SHALL render a localized empty message and a contextual link back to the categories index or the home page. The page SHALL preserve the page shell and the section header when one is already rendered.

#### Scenario: Empty category still navigable

- **WHEN** the category detail page has no published posts
- **THEN** the page renders the category intro and a localized empty message with a link back to the categories index

### Requirement: Page-level not-found state

When a slug does not resolve to a post or category, the page SHALL render a "Not found" state with a localized message and a link back to the categories index. The page SHALL preserve the shared layout shell.

#### Scenario: Unknown post slug shows not-found

- **WHEN** the user navigates to `/:lang/posts/unknown-slug`
- **THEN** the page renders a localized "Not found" message with a link back to the categories index

### Requirement: Shared translation and date helpers

The system SHALL expose `getLocalizedPost`, `getLocalizedCategory`, `formatPublishedDate`, and `getPostThumbnail` helpers for use across pages. The helpers SHALL be the only consumers of the GraphQL translation fields. Pages SHALL NOT reimplement translation, date formatting, or thumbnail resolution logic.

#### Scenario: Localized post title is selected

- **WHEN** a post has a Vietnamese translation
- **THEN** `getLocalizedPost` returns the Vietnamese title and content and the page renders the translated values

#### Scenario: Missing translation falls back to default

- **WHEN** a post has no Vietnamese translation
- **THEN** `getLocalizedPost` returns the default-locale title and content and the page renders the default values without a placeholder label

### Requirement: Image fallback pattern

The system SHALL expose a `PostArtwork` component that renders a post's thumbnail when present and a deterministic fallback motif when absent. The fallback SHALL be CSS-only, SHALL preserve the layout aspect ratio, and SHALL NOT make a network request.

#### Scenario: Thumbnail renders when present

- **WHEN** a post has a thumbnail URL
- **THEN** the `PostArtwork` renders an `<img>` with the thumbnail and a localized alt

#### Scenario: Fallback renders when absent

- **WHEN** a post has no thumbnail
- **THEN** the `PostArtwork` renders the deterministic fallback motif and the layout dimensions are preserved

### Requirement: Polite live region for transient feedback

The system SHALL expose a shared `StatusMessage` component using `role="status"` and `aria-live="polite"`. The Copy link action and the Retry button SHALL route announcements through this component rather than mutating focus.

#### Scenario: Copy link announces via live region

- **WHEN** the user activates the Copy link button
- **THEN** the live region announces "Link copied" without moving focus

### Requirement: Target size compliance

All icon-only and compact interactive controls (share buttons, language switch buttons, mobile menu trigger, breadcrumb links) SHALL have a hit target of at least 44×44 CSS px. Inline text links in prose are exempt.

#### Scenario: Share buttons meet target size

- **WHEN** the share row renders
- **THEN** each control's hit target is at least 44×44 CSS px

#### Scenario: Language switch buttons meet target size

- **WHEN** the language switch renders
- **THEN** each segment has a hit target of at least 44×44 CSS px
