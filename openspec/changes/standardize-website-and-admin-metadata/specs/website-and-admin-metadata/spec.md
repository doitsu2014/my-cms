## ADDED Requirements

### Requirement: Public routes have complete, localized document metadata
The public website SHALL produce exactly one document title, meta description, canonical URL, Open Graph metadata, Twitter card metadata, and language declaration for every supported public route and locale.  Home, category listing, category detail, article detail, About, loading, error, and not-found states SHALL receive a deterministic localized metadata profile.  Profiles without route-specific content SHALL fall back to the configured default title and description.

#### Scenario: Localized article metadata is rendered
- **WHEN** a reader requests a published article at `/{lang}/posts/{slug}` and localized article content is available
- **THEN** the document title and description use the localized article title and preview content with the configured site identity
- **AND** the canonical URL identifies that locale-specific article route
- **AND** the Open Graph and Twitter metadata describe the same article without using the article's raw HTML body

#### Scenario: Public fallback metadata is rendered
- **WHEN** a public route has no route-specific content, content is loading, or its optional metadata fields are blank
- **THEN** the document has one valid title and description derived from the configured public-site defaults
- **AND** the page does not retain metadata from a previously visited route

### Requirement: Public SSR documents expose the resolved metadata
The public website's SSR response SHALL contain the resolved metadata in its HTML `<head>` before client-side JavaScript runs.  After a client-side route transition, the browser document head SHALL converge on the same metadata profile and SHALL not contain duplicate title, description, canonical, or social tags.

#### Scenario: Crawler receives article metadata without JavaScript
- **WHEN** a crawler requests a published article URL from the public website
- **THEN** the HTTP response HTML head contains the article's resolved title, description, canonical URL, and social metadata
- **AND** the crawler does not need to execute client-side JavaScript to discover those values

#### Scenario: Browser navigation replaces metadata
- **WHEN** a reader navigates from one public route to another without a full page load
- **THEN** the document title and managed metadata tags are replaced with the destination route's profile
- **AND** each managed metadata tag is present at most once

### Requirement: Public metadata is safe and canonical
Public metadata SHALL use the configured public site URL plus a normalized supported-locale route path for canonical URLs, excluding query strings and fragments.  Values derived from CMS content or runtime configuration SHALL be escaped before insertion into HTML.  The public metadata policy SHALL not expose unpublished, missing, or error-state content as an indexable article or category.

#### Scenario: CMS text cannot break head markup
- **WHEN** a title or preview contains markup-significant characters
- **THEN** the rendered metadata encodes those characters so the document head remains valid
- **AND** no additional element or script can be introduced through the metadata value

#### Scenario: Missing or unpublished article is not canonicalized as public content
- **WHEN** an article route does not resolve to a published article
- **THEN** the metadata falls back to the appropriate not-found or error profile
- **AND** it does not claim a canonical public article title or description for unavailable content

### Requirement: The administration application has private, route-aware metadata
The administration application SHALL set a meaningful title for login, dashboard, list, create, and edit routes using a centrally maintained route-metadata policy.  Every administration document SHALL contain a `robots` directive of `noindex, nofollow` and SHALL not emit public-site canonical, Open Graph, or Twitter metadata.

#### Scenario: Admin route names its current task
- **WHEN** an authenticated operator navigates to an admin list, create, or edit route
- **THEN** the document title identifies the current admin task and the administration application
- **AND** the title is updated when the operator navigates to another admin route

#### Scenario: Login and protected routes stay out of search indexes
- **WHEN** a user opens the admin login route or any protected admin route
- **THEN** the HTML document includes `robots` with `noindex, nofollow`
- **AND** no public Open Graph, Twitter card, or canonical tag is emitted

### Requirement: Metadata behavior is covered by regression tests
The applications SHALL test the metadata resolver and its integration at the public SSR boundary, public client-route transition boundary, and representative admin routes.  The tests SHALL verify values, escaping, fallbacks, noindex behavior, and absence of duplicate managed tags.

#### Scenario: A metadata regression is detected before release
- **WHEN** a change removes, duplicates, or incorrectly derives a managed metadata field for a representative route
- **THEN** the relevant website or admin test fails
- **AND** the applications' production builds remain part of the verification gate
