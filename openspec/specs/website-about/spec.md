# website-about Specification

## Purpose
TBD - created by archiving change redesign-ducth-dev-website. Update Purpose after archive.
## Requirements
### Requirement: New `/about` route

The system SHALL expose a new reader route at `/:lang/about` rendered by `AboutPage`. The route SHALL be reachable from the primary navigation and SHALL be localized according to the active `:lang` segment. The page SHALL resolve the same SSR + hydration contract as the existing reader pages.

#### Scenario: About route renders

- **WHEN** the user navigates to `/en/about` or `/vi/about`
- **THEN** the About page renders within the shared layout shell

#### Scenario: About is reachable from the navigation

- **WHEN** the user is on any reader route
- **THEN** the primary navigation shows an "About" link that navigates to `/:lang/about`

### Requirement: Verified About content

The About page SHALL render content from a localized static config (`src/config/about.config.ts`) keyed by `:lang`. Each locale SHALL have a `verified` flag set by the product-owner. When the flag is `false`, the page SHALL render a localized "Coming soon" placeholder instead of unverified copy. The page SHALL NOT render prototype placeholder facts (e.g., "11 years," "38 articles," fake contact details) unless those values are present in the verified config.

#### Scenario: Unverified locale renders a placeholder

- **WHEN** the active locale's `verified` flag is `false`
- **THEN** the page renders a localized "Coming soon" placeholder and no other content from the unverified config

#### Scenario: Verified locale renders real content

- **WHEN** the active locale's `verified` flag is `true`
- **THEN** the page renders the hero, the pillars, the statement, the practices, and the contact/social links from the config

### Requirement: About page hierarchy

The About page SHALL render, in order: a career hero with portrait and three metadata facts, career pillars, a portrait statement, practice habits, and a contact/social links block. Each section SHALL be a localized group inside the config and SHALL be rendered only when the locale's `verified` flag is true.

#### Scenario: Hero renders three metadata facts

- **WHEN** the verified About page renders
- **THEN** the hero shows the portrait and exactly three metadata facts (e.g., role, focus, location) drawn from the config

#### Scenario: Pillars render in config order

- **WHEN** the verified About page renders
- **THEN** the pillars section shows the pillars defined in the config in the order they appear in the config

### Requirement: Contact and social links

The contact section SHALL render a `mailto:` link when an email is provided, and SHALLOW `rel="noopener"` external links for each social URL provided. The page SHALL NOT render placeholder email or social URLs; when a field is absent from the config, the corresponding contact row SHALL be omitted.

#### Scenario: Email is rendered when provided

- **WHEN** the verified About page has a non-empty email
- **THEN** the contact section renders a `mailto:` link with the configured email

#### Scenario: Missing email is omitted

- **WHEN** the verified About page has no email
- **THEN** the contact section does not render an email row and no placeholder link is shown

#### Scenario: External links open in a new tab safely

- **WHEN** an external link is rendered
- **THEN** the link has `target="_blank"` and `rel="noopener noreferrer"`

