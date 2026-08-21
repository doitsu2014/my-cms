## ADDED Requirements

### Requirement: Administrators can manage Ducth global head assets

The system SHALL persist a collection of independently named global head assets for the Ducth public website. Each asset SHALL have an immutable identifier, label, source HTML, enabled state, positive sort order, optimistic-concurrency row version, and creation/update audit timestamps and actor identifiers. Labels SHALL be unique within this collection. The system SHALL expose administrator-only REST operations at `GET /seo/head-assets`, `GET /seo/head-assets/{id}`, `POST /seo/head-assets`, `PUT /seo/head-assets/{id}`, and `DELETE /seo/head-assets/{id}`. Collection results SHALL be ordered by `sortOrder` ascending and then identifier ascending.

The create request SHALL contain `label`, `html`, `enabled`, and `sortOrder`. The update request SHALL contain the same fields plus the currently read `rowVersion`; updates replace the editable fields rather than merging them. Successful create and update responses SHALL return the saved asset, including its incremented `rowVersion`; successful delete SHALL return HTTP 204. Administrator responses SHALL use the existing API success envelope and camelCase fields.

#### Scenario: Administrator creates a Google Analytics asset

- **WHEN** an authenticated administrator submits a unique label, allowed Google tag HTML, `enabled: true`, and a positive sort order to `POST /seo/head-assets`
- **THEN** the system returns HTTP 201 with the persisted asset and `rowVersion: 1`
- **AND** the asset records the administrator as its creator and updater

#### Scenario: Administrator lists and reads assets in publication order

- **WHEN** an authenticated administrator requests `GET /seo/head-assets` or `GET /seo/head-assets/{id}` for an existing asset
- **THEN** the system returns HTTP 200 with the requested asset data
- **AND** the collection result orders equal sort orders by identifier so its order is deterministic

#### Scenario: Administrator updates an asset with its current version

- **WHEN** an authenticated administrator sends `PUT /seo/head-assets/{id}` with the asset's current `rowVersion` and valid replacement fields
- **THEN** the system returns HTTP 200 with the replacement values and an incremented `rowVersion`
- **AND** the asset records the administrator as its updater

#### Scenario: Administrator removes an asset

- **WHEN** an authenticated administrator sends `DELETE /seo/head-assets/{id}` for an existing asset
- **THEN** the system returns HTTP 204
- **AND** the asset is not returned by later administrator or public collection reads

#### Scenario: Duplicate label is rejected

- **WHEN** an administrator creates or updates an asset to a label already held by a different asset
- **THEN** the system returns HTTP 409 with the existing conflict error code

#### Scenario: Invalid field values are rejected

- **WHEN** an administrator submits a blank or oversized label, empty or oversized HTML source, or a non-positive sort order
- **THEN** the system returns HTTP 400 with the validation error code
- **AND** no asset is created or changed

#### Scenario: Stale update is rejected without overwriting a later update

- **WHEN** an administrator updates an asset with a `rowVersion` that no longer matches the stored version
- **THEN** the system returns HTTP 409 with the optimistic-concurrency error code
- **AND** the later stored values remain unchanged

#### Scenario: Unknown asset is not changed

- **WHEN** an administrator reads, updates, or deletes an identifier that does not exist
- **THEN** the system returns HTTP 404

### Requirement: SEO head asset authoring is administrator-only and non-executing in the CMS

All management endpoints SHALL be mounted behind the existing `my-headless-cms-administrator` role. Missing or invalid authentication SHALL return HTTP 401; a valid writer-only token SHALL return HTTP 403. The administration application SHALL expose the management UI only through `AdminOnlyRoute` at `/admin/seo/head-assets` and shall show its navigation entry only to administrators.

The admin interface SHALL provide a responsive list plus create and edit flows with required label, enabled, positive sort order, and source textarea fields. It SHALL display source only as inert text/value and SHALL NOT use `dangerouslySetInnerHTML`, an iframe, or any other live-preview execution path. It SHALL cover loading, empty, request-failure with retry, field validation, save failure, delete confirmation, keyboard/focus behavior, and responsive controls. A non-administrator who reaches the URL SHALL see the existing access-denied state rather than an authoring interface.

#### Scenario: Writer cannot use the API or authoring screen

- **WHEN** a user whose roles contain only `my-headless-cms-writer` calls a management endpoint or opens `/admin/seo/head-assets`
- **THEN** the API returns HTTP 403 for the endpoint
- **AND** the administration application shows no SEO navigation entry and no authoring UI

#### Scenario: Submitted source remains inert in administration

- **WHEN** an administrator views or edits an asset containing a script element
- **THEN** the browser displays the source as textarea/plain text content
- **AND** that script does not execute in the administration application

#### Scenario: Delete requires deliberate confirmation

- **WHEN** an administrator chooses to delete an asset
- **THEN** the administration application asks for confirmation before issuing `DELETE /seo/head-assets/{id}`
- **AND** cancelling the confirmation leaves the asset unchanged

### Requirement: Only validated head-safe trusted markup can be stored

The system SHALL treat source HTML as trusted executable content for the public site, never as untrusted user content. It SHALL validate source server-side before create or update and SHALL persist and publish the exact submitted source only when it is valid. The validation limit SHALL be 32 KiB UTF-8 source per asset and SHALL reject malformed markup, elements outside `script`, `meta`, and `link`, event-handler attributes, `style`, `base`, `title`, `iframe`, and elements or attributes that can close or escape the document head.

`script` elements SHALL permit inline JavaScript or JSON-LD and optional HTTPS `src`, with only `async`, `defer`, `src`, `type`, `integrity`, `crossorigin`, `referrerpolicy`, and `data-*` attributes. `meta` elements SHALL permit only `name` or `property` plus `content`, and SHALL reject typed metadata names/properties owned by the existing metadata capability (`description`, `robots`, `og:*`, and `twitter:*`). `link` elements SHALL permit only HTTPS `href`, `rel` values `preconnect`, `dns-prefetch`, `preload`, `modulepreload`, `stylesheet`, or `icon`, and the compatible attributes `as`, `type`, `media`, `integrity`, `crossorigin`, `referrerpolicy`, and `sizes`. The validator SHALL reject JavaScript URLs, non-HTTPS external URLs, duplicate attributes, and a script with both `src` and non-whitespace inline content.

#### Scenario: Permitted Google tag source is accepted

- **WHEN** an administrator submits the supplied Google tag snippet containing an HTTPS external async script and an inline script
- **THEN** validation accepts it and the source is eligible for persistence

#### Scenario: Search verification and JSON-LD source are accepted

- **WHEN** an administrator submits a permitted verification `meta` element or an inline `script type="application/ld+json"`
- **THEN** validation accepts the source if its allowed attributes and size satisfy the contract

#### Scenario: Dangerous or head-breaking source is rejected

- **WHEN** an administrator submits a body element, event handler, `javascript:` URL, HTTP external source, a typed metadata override, or malformed/oversized HTML
- **THEN** the system returns HTTP 400 with the validation error code
- **AND** the source is neither persisted nor published

### Requirement: Enabled assets are available to Ducth SSR without changing typed metadata

The system SHALL expose unauthenticated `GET /seo/head-assets/ducth-dev` for the Ducth SSR service. It SHALL return HTTP 200 with only enabled, validated assets, ordered by `sortOrder` ascending then identifier ascending, using a public response model that contains only `id`, `label`, `html`, `sortOrder`, and `updatedAt`. It SHALL return an empty collection when no assets are enabled. This endpoint SHALL not expose disabled assets, audit actor identifiers, row versions, or administration-only fields.

The Ducth SSR service SHALL inject the returned HTML verbatim into the existing server-rendered document `<head>` for every non-redirect public route, alongside and without replacing the existing typed metadata and runtime configuration insertions. It SHALL not fetch or inject the assets in the browser. A disabled or deleted asset SHALL be absent after the next successful cache refresh; an enabled new or updated asset SHALL appear in stable order after that refresh, without a website deployment.

#### Scenario: Enabled assets render for a crawler response

- **WHEN** Ducth SSR receives enabled public assets and renders a public route
- **THEN** the HTTP response contains each enabled asset exactly once in the document `<head>` in public API order
- **AND** the existing title, canonical, robots, social metadata, and runtime-config state remain present and non-duplicated

#### Scenario: Disabled assets are withheld from public delivery

- **WHEN** an asset is disabled and the public delivery cache refreshes successfully
- **THEN** `GET /seo/head-assets/ducth-dev` excludes it
- **AND** newly rendered Ducth documents do not contain its source

#### Scenario: No enabled assets does not change page rendering

- **WHEN** the public endpoint returns an empty collection
- **THEN** Ducth SSR produces its normal document head without dynamic assets

### Requirement: Public delivery is bounded, observable, and fails open

The Ducth SSR service SHALL obtain public assets through a server-only configured endpoint with a request timeout of at most two seconds. It SHALL cache the last successful public response for 60 seconds per SSR process and SHALL make at most one refresh request while a refresh is in flight. When no successful response is available, a timeout, invalid response, or non-200 response SHALL not fail the SSR request: the service SHALL render the existing page head without dynamic assets. When a previously successful response exists, the service MAY retain that response for a bounded maximum of five minutes while the endpoint is unavailable; it SHALL emit a warning whenever it serves stale data and SHALL drop it after that bound.

The API SHALL emit structured, secret-free audit information for each successful create, update, and delete, including action, asset identifier, and actor identifier but never source HTML. The SSR service SHALL emit cache refresh success/failure and stale-fallback telemetry without logging source HTML, full third-party URLs, or runtime configuration values.

#### Scenario: Source service timeout does not take down SSR

- **WHEN** the public asset endpoint times out or is unavailable and Ducth has no usable cached response
- **THEN** Ducth returns its normal SSR response rather than HTTP 500
- **AND** the response contains existing typed metadata but no dynamic head assets

#### Scenario: Bounded stale response preserves availability

- **WHEN** the public asset endpoint fails after a successful response is cached but before the five-minute stale bound expires
- **THEN** Ducth may render the cached asset set and emits a stale-fallback warning
- **AND** it stops using that set once it exceeds the stale bound

#### Scenario: Secret-free audit and operational logging

- **WHEN** an asset mutation, public read failure, or SSR cache failure occurs
- **THEN** logs and tracing identify the operation outcome without including the asset HTML or secrets
