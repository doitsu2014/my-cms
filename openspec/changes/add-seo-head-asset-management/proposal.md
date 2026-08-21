## Why

The Ducth public website has no administrator-managed way to add global SEO and measurement assets, so adding Google Analytics or a search-verification tag currently requires a website code change and deployment. The requested need extends beyond Analytics to multiple independently managed assets that must be present in every public document head.

## What Changes

- Introduce an SEO domain that persists a collection of global Ducth website head assets, rather than a Google Analytics-specific configuration field.
- Let authorized administrators create, view, update, enable or disable, order, and remove named head assets from the CMS administration application.
- Make enabled assets available to the Ducth website and render them in the server-rendered `<head>` of every public route, including the initial crawler response.
- Treat each asset's HTML as trusted, head-safe executable content, with administrator-only authoring, validation, size limits, auditability, and no executable preview in the administration application.
- Preserve the existing typed public metadata policy (title, description, canonical, robots, and social metadata) as a separate concern.

## Goals and Non-Goals

### Goals

- Support multiple global assets, including Google Analytics, search-engine verification tags, structured-data markup, and approved third-party integrations.
- Allow an authorized administrator to manage assets without a Ducth website deployment.
- Ensure disabled or removed assets are no longer published and enabled assets appear in a stable administrator-defined order.
- Keep the capability scoped to the single Ducth public website in this release.

### Non-Goals

- Consent banners, preference management, regional privacy rules, or conditional loading based on visitor consent.
- Per-route, per-content, locale-specific, user-specific, or environment-specific assets.
- Body/footer injection, arbitrary page content, tag-manager workflows, or an in-admin live preview of submitted code.
- Replacing the existing typed metadata, sitemap, crawler-directive, authentication, or authorization capabilities.

## Capabilities

### New Capabilities

- `seo-head-asset-management`: Administrator-managed global Ducth website head assets, including their lifecycle, authorization, trusted-content safeguards, and public rendering availability.

### Modified Capabilities

- None.

## Impact

- A new API domain, its persistence/migration lifecycle, authenticated administration endpoints, and a public read path for enabled Ducth assets.
- The CMS administration application, including an administrator-only SEO management area and its normal loading, empty, validation, failure, and destructive-action states.
- The Ducth SSR server and its production-server tests, which must add enabled assets to every rendered document head while retaining current metadata behavior.
- Deployment/runtime configuration and operational documentation may need updating so the website can reach the public asset source and changes take effect predictably.
- No existing public content, GraphQL content model, sitemap, or route-metadata requirements are changed.

## Assumptions, Dependencies, and Risks

- **Assumption — consent:** The first release publishes enabled assets to every visitor immediately. This is appropriate only if the site owner has determined that the selected assets, including Google Analytics, may load without consent for the audience and jurisdictions served. Consent-aware loading is deliberately deferred and must be a separate product decision/change.
- **Assumption — trust:** Raw head HTML can execute JavaScript for all visitors. Only administrators are trusted to author it; it must not be previewed as live code in the admin UI, and authors must not store secrets in it.
- **Dependency:** The Ducth SSR deployment must be able to retrieve the enabled assets reliably at runtime; the technical design will define the availability and refresh behavior.
- **Risk:** A malicious, incorrect, or unavailable third-party asset can affect privacy, security, performance, or page rendering. The implementation needs explicit validation, controlled publication, audit evidence, and safe behavior when the asset source cannot be read.

## Acceptance Outcomes

- An administrator can manage more than one named global head asset for the Ducth website and control whether each is published.
- An enabled Google Analytics snippet can be persisted and is present in the SSR HTML `<head>` for every public Ducth page without a website deployment.
- A disabled or deleted asset is absent from newly rendered public documents, while the existing title, canonical, robots, and social metadata remain present and non-duplicated.
- Non-administrators cannot author or modify assets, and submitted code is never executed inside the administration interface.
- The release makes the above consent and trusted-code assumptions visible to the site owner before any tracking asset is enabled.
