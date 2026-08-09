# website-reader-experience Specification

## Purpose
TBD - created by archiving change redesign-ducth-dev-website. Update Purpose after archive.
## Requirements
### Requirement: Home as a writing index

The home page at `/:lang` SHALL render four sections in order: a recent articles section, a category-count strip, and a hiring/contact CTA. A featured/highlights section SHALL be rendered only when a separate curated contract is approved by the product-owner; otherwise the home page SHALL NOT render a section mislabelled as featured. The page SHALL NOT render prototype placeholder facts such as "38 articles" or "11 years" without an approved source.

#### Scenario: Recent articles lists the latest published posts

- **WHEN** the home page renders
- **THEN** the recent articles section lists the six most recently published posts in date-descending order

#### Scenario: Category strip reflects real category counts

- **WHEN** the home page renders
- **THEN** the category strip shows each category name and the count of published posts in that category, both derived from the GraphQL response

#### Scenario: Featured section is not mislabelled

- **WHEN** no curated content contract is approved
- **THEN** the home page does not render a section labelled "Featured" or "Highlights" and the most recent posts are not additionally presented as curated

### Requirement: Asymmetric home compositions

The home page SHALL render the recent articles using a single asymmetric layout grid (lead, compact, standard, wide variants) at viewport widths of 1024px and above. Below 1024px the grid SHALL collapse to a single column without reordering the DOM.

#### Scenario: Desktop uses the asymmetric grid

- **WHEN** the viewport width is 1440px
- **THEN** the recent articles section uses the lead-then-compact-then-standard-then-wide composition and the DOM order matches the visual order

#### Scenario: Narrow viewport collapses to one column

- **WHEN** the viewport width is 360px
- **THEN** the recent articles section renders as a single column and no visual reordering shuffles the DOM

### Requirement: Category index with numbered editorial rows

The categories index page at `/:lang/categories` SHALL render a numbered editorial row per category. Each row SHALL show the ordinal, the bilingual category name, the localized slug, the editorial description (when available), the post count, a CTA to the category detail, and the three latest article previews. The page SHALL NOT render rotating semantic color badges.

#### Scenario: Each row renders the latest three previews

- **WHEN** the categories index renders
- **THEN** each row shows three preview cards of the most recent posts in that category in date-descending order

#### Scenario: Description is not derived from the slug

- **WHEN** no editorial description is available for a category
- **THEN** the row omits the description field rather than rendering slug-derived prose

#### Scenario: Numbered rows are visually ordered

- **WHEN** the categories index renders
- **THEN** each row shows an ordinal `01`, `02`, … matching its position in the list

### Requirement: Category detail with article rows

The category detail page at `/:lang/categories/:slug` SHALL render a split intro (portrait image + bilingual prose) and a list of vertical article rows (image, metadata, title, excerpt, CTA). Rows SHALL collapse to a single column under 800px. Pagination SHALL be rendered only if the GraphQL contract supports it; otherwise the page renders the complete list with a "Showing all N articles" line and no pagination chrome.

#### Scenario: Article rows render real posts

- **WHEN** the category detail page renders
- **THEN** the article rows list the published posts in that category in date-descending order

#### Scenario: No fake pagination

- **WHEN** the GraphQL query does not return a paginated connection
- **THEN** the page does not render a numbered pagination chrome and instead shows "Showing all N articles"

#### Scenario: Empty category remains navigable

- **WHEN** the category has no published posts
- **THEN** the page renders the category intro and a localized empty message with a link back to the categories index

### Requirement: Post reading experience

The post page at `/:lang/posts/:slug` SHALL render an editorial header (category pill, date, optional reading time when data is available, title, deck, byline), a featured image at 21:9, an article body constrained to 68ch, a share row, and a related posts section. The article body SHALL use the `.article-prose` contract. The Highlight.js code highlighting integration SHALL be preserved.

#### Scenario: Article body stays within 68ch

- **WHEN** the post page renders
- **THEN** the article body's computed width is at most 68ch

#### Scenario: Featured image renders at 21:9

- **WHEN** the post has a featured image
- **THEN** the image renders at a 21:9 aspect ratio with a stable crop and the empty alt is avoided by using a localized alt derived from the title

#### Scenario: Code blocks remain highlighted

- **WHEN** the post body contains a TipTap code block
- **THEN** Highlight.js applies the existing syntax classes and the code block scrolls horizontally on overflow

### Requirement: Functional share actions

The share row SHALL expose three controls: an X intent link, a LinkedIn share link, and a Copy link button. All three controls SHALL have an accessible name and a hit target of at least 44×44 CSS px. The Copy link button SHALL write the canonical URL to the clipboard and SHALL announce "Link copied" through a polite live region.

#### Scenario: Copy link announces success

- **WHEN** the user activates the Copy link button
- **THEN** the canonical URL is written to the clipboard and a `role="status"` live region announces "Link copied"

#### Scenario: Share controls are at least 44px

- **WHEN** the share row renders
- **THEN** each control's hit target is at least 44×44 CSS px

#### Scenario: X and LinkedIn links navigate correctly

- **WHEN** the user activates the X link
- **THEN** the browser opens an X intent compose page in a new tab with the canonical URL and title pre-filled, and the link has `rel="noopener"`

### Requirement: Related posts are deterministic and exclude the current post

The related posts section SHALL list three posts that share at least one category with the current post, excluding the current post itself. The list SHALL be sorted by date in descending order before slicing. When fewer than three related posts exist, the section SHALL render the available posts without placeholder cards.

#### Scenario: Related posts exclude the current post

- **WHEN** the related posts section renders
- **THEN** the current post is not listed and the remaining posts are sorted by date descending

#### Scenario: Fewer than three related posts

- **WHEN** fewer than three related posts are available
- **THEN** the section renders only the available posts and does not pad with placeholder cards

### Requirement: Deterministic thumbnail fallback

When a post has no thumbnail, the post card and the article row SHALL render a deterministic fallback motif keyed by the post slug. The fallback SHALL be a CSS-only ink motif (no random image service) and SHALL preserve the layout aspect ratio.

#### Scenario: Missing thumbnail falls back deterministically

- **WHEN** a post has no thumbnail
- **THEN** the card renders the fallback motif and the same post renders the same motif on every render

### Requirement: Locale-prefixed routes are preserved

The reader routes SHALL remain `/:lang`, `/:lang/categories`, `/:lang/categories/:slug`, `/:lang/posts/:slug`, and the new `/:lang/about`. The catch-all behavior SHALL be preserved. Hardcoded URLs in the static template SHALL be replaced with the runtime-configured base URL.

#### Scenario: All routes resolve

- **WHEN** the user navigates to `/en`, `/en/categories`, `/en/categories/foo`, `/en/posts/bar`, or `/en/about`
- **THEN** each route renders the corresponding page

#### Scenario: Legacy URLs redirect

- **WHEN** the user navigates to `/`
- **THEN** the reader redirects to `/en`

