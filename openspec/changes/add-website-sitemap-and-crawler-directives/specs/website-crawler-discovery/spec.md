## ADDED Requirements

### Requirement: The public website exposes a valid sitemap

The public website SHALL serve `GET /sitemap.xml` as UTF-8 XML with an XML sitemap `<urlset>` containing every supported localized static reader route, every published post route, and every published top-level blog category route. Sitemap URLs SHALL use the configured public site URL and SHALL be absolute, canonical, and free of query strings or fragments.

#### Scenario: Static routes are discoverable in every supported locale

- **WHEN** a crawler requests `/sitemap.xml`
- **THEN** the response includes localized home, categories, and about URLs for English and Vietnamese
- **AND** it does not include the redirect-only root URL or admin/API routes

#### Scenario: Published content is included and unpublished content is excluded

- **WHEN** the public content query returns published posts and categories plus an unpublished post
- **THEN** the sitemap includes one route per supported locale for each published post and category
- **AND** the sitemap omits the unpublished post

#### Scenario: Sitemap content is safe XML

- **WHEN** a published slug or configured site URL contains XML-significant characters
- **THEN** the sitemap escapes XML text and remains parseable
- **AND** no content value can introduce an additional XML element

### Requirement: Crawler resources bypass the SPA fallback

The production website SHALL serve `robots.txt` and `sitemap.xml` before the SSR catch-all route with `200` responses and crawler-appropriate content types. `robots.txt` SHALL preserve its existing public allow policy and ClaudeBot disallow policy and SHALL advertise the absolute sitemap URL.

#### Scenario: Robots points crawlers to the sitemap

- **WHEN** a crawler requests `/robots.txt`
- **THEN** the response is plain text and contains `Sitemap: <configured-site-url>/sitemap.xml`
- **AND** the response contains `User-agent: *` with `Allow: /`
- **AND** the response continues to disallow `ClaudeBot`

#### Scenario: Crawler requests do not invoke SSR

- **WHEN** a crawler requests `/robots.txt` or `/sitemap.xml` while the GraphQL API is unavailable
- **THEN** the server does not render the SPA fallback
- **AND** `robots.txt` still returns the crawler policy
- **AND** the sitemap returns the static public routes with a successful XML response

### Requirement: Sitemap generation handles API failures safely

The sitemap endpoint SHALL filter to published public content, bound its upstream request, and fall back to the localized static route set when the GraphQL API fails, times out, or returns malformed data. The endpoint SHALL use cache headers appropriate for a short-lived public discovery document.

#### Scenario: Temporary content API failure retains baseline discoverability

- **WHEN** the GraphQL request fails or exceeds its timeout
- **THEN** `/sitemap.xml` returns a valid sitemap containing the localized static routes
- **AND** the response does not expose upstream error details
- **AND** the response includes a short public cache lifetime
