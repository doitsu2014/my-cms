## ADDED Requirements

### Requirement: Shared layout shell

The system SHALL render a single `SiteLayout` for every route composed of a skip link, a `<header>`, an unconstrained `<main id="main">`, and a `<footer>`. The `<main>` SHALL NOT impose a single constrained container; pages SHALL own their section-level containers. The shell SHALL render on the server and on the client without hydration mismatch.

#### Scenario: Layout renders on every route

- **WHEN** any reader route is loaded
- **THEN** the HTML contains exactly one `<header>`, one `<main id="main">`, and one `<footer>`

#### Scenario: Main is unconstrained

- **WHEN** a page renders a full-width section
- **THEN** the section spans the viewport width and the inner content is constrained by the page's own container

### Requirement: Skip link on every route

The system SHALL render a `<a href="#main">Skip to content</a>` as the first focusable element of every route. The link SHALL become visible on focus and SHALL move keyboard focus to the `<main>` element when activated.

#### Scenario: Skip link is focused first

- **WHEN** the user presses Tab on a fresh page load
- **THEN** the skip link receives focus and is visually distinct from the page background

#### Scenario: Skip link moves focus to main

- **WHEN** the skip link is activated
- **THEN** the `<main>` element receives focus and the next Tab resumes the document's content order

### Requirement: Header with active navigation

The header SHALL display the author portrait wordmark, the primary navigation links (Home, Categories, About), and the VI/EN language switch. The link corresponding to the active route SHALL show an active underline. The header SHALL be sticky, hairline-bordered, and lifted without a drop shadow.

#### Scenario: Active route is indicated

- **WHEN** the user is on `/:lang/categories`
- **THEN** the "Categories" navigation link shows an underline and the other links do not

#### Scenario: Header is sticky

- **WHEN** the user scrolls past the header
- **THEN** the header remains pinned to the top of the viewport with a 1px bottom hairline

### Requirement: Mobile navigation drawer

The header SHALL expose a `<button aria-expanded aria-controls="mobile-nav">` labelled "Menu" when the primary navigation links do not fit alongside the wordmark and language switch. The drawer SHALL contain the same primary links as the desktop navigation. The drawer SHALL close on Escape, on route change, and when focus returns to the trigger button.

#### Scenario: Trigger opens the drawer

- **WHEN** the user activates the Menu button on a narrow viewport
- **THEN** the drawer becomes visible, focus moves to the first link, and `aria-expanded` is `true`

#### Scenario: Escape closes the drawer

- **WHEN** the drawer is open and the user presses Escape
- **THEN** the drawer closes, focus returns to the Menu button, and `aria-expanded` is `false`

#### Scenario: Route change closes the drawer

- **WHEN** the drawer is open and the user activates a link
- **THEN** the navigation proceeds and the drawer is closed on the new route

### Requirement: Language switch as segmented control

The header SHALL expose a VI/EN segmented control with two buttons. The active language SHALL be the one matching the current `:lang` route segment. Activating the other button SHALL rewrite the URL to the same page under the other locale while preserving the rest of the path.

#### Scenario: Active language is shown

- **WHEN** the user is on `/vi/categories`
- **THEN** the VI button is visually marked as active and the EN button is not

#### Scenario: Activating the other locale preserves the path

- **WHEN** the user activates the EN button on `/vi/categories/foo`
- **THEN** the browser navigates to `/en/categories/foo` and the page content renders in the new locale

### Requirement: Breadcrumbs

The system SHALL render breadcrumbs above the page title on category detail and post detail pages. Breadcrumbs SHALL use `<nav aria-label="Breadcrumb">` and an ordered list of links. The current page SHALL be the last item and SHALL NOT be a link.

#### Scenario: Breadcrumbs show the path

- **WHEN** a post page renders
- **THEN** the breadcrumb shows "Home / Categories / {Category} / {Post Title}" with the post title as the non-link current item

#### Scenario: Breadcrumb is announced as a landmark

- **WHEN** a screen reader reads the page
- **THEN** the breadcrumb is announced as a navigation landmark labelled "Breadcrumb"

### Requirement: Three-column footer

The footer SHALL render three columns: an author summary, a primary navigation summary, and a list of external links (GitHub, LinkedIn, email when provided). The footer SHALL also render a build note rendered when a non-production build is detected.

#### Scenario: Footer columns are present

- **WHEN** any reader route renders
- **THEN** the footer contains the author summary, the primary navigation links, and the external links list

#### Scenario: Footer collapses on narrow viewports

- **WHEN** the viewport width is 360px
- **THEN** the footer columns stack vertically and the external links wrap without overflow

### Requirement: Document language synchronization

The system SHALL set `<html lang>` to match the active `:lang` route segment on both the server-rendered HTML and the post-hydration DOM. Inline mixed-language spans SHALL use `lang="en"` or `lang="vi"` where pronunciation materially benefits.

#### Scenario: HTML lang matches the route

- **WHEN** the user is on `/vi/posts/foo`
- **THEN** the `<html>` element has `lang="vi"` on the SSR HTML and after hydration

#### Scenario: English brand labels announce in English

- **WHEN** the footer contains a brand label that is intentionally English
- **THEN** the span has `lang="en"` and the surrounding document remains in the route's locale
