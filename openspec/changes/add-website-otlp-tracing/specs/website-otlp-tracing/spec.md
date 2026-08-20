## ADDED Requirements

### Requirement: Opt-in server-side website trace export

The production Ducth website process SHALL support OpenTelemetry trace export only when `ENABLED_OTLP_EXPORTER` is exactly `true`. When enabled, the process SHALL use `OTEL_SERVICE_NAME`, `OTEL_EXPORTER_OTLP_TRACES_ENDPOINT`, and `OTEL_TRACES_SAMPLER` as startup-time, server-only telemetry configuration. The Docker Compose website service SHALL map website-specific deployment inputs (`WEBSITE_ENABLED_OTLP_EXPORTER`, `WEBSITE_OTEL_SERVICE_NAME`, `WEBSITE_OTEL_EXPORTER_OTLP_TRACES_ENDPOINT`, and `WEBSITE_OTEL_TRACES_SAMPLER`) to those runtime variables; its documented default service identity SHALL be `ducth-dev-website`, and its documented Jaeger endpoint SHALL use the OTLP/HTTP receiver at `http://jaeger:4318/v1/traces`.

When tracing is disabled, absent, invalid, or cannot initialize its exporter, the website SHALL retain its existing startup, HTTP, SSR, GraphQL, error-response, and logging behavior without requiring a collector. Telemetry configuration and exporter failures SHALL be reported without including secret values or request/content payloads, and SHALL NOT expose telemetry configuration to browser runtime configuration.

#### Scenario: Enabled SSR process exports under the website identity
- **WHEN** the website starts with `ENABLED_OTLP_EXPORTER=true`, a valid OTLP/HTTP traces endpoint, and `OTEL_SERVICE_NAME=ducth-dev-website`
- **THEN** a normal server-rendered reader request produces website spans eligible for export with `service.name` equal to `ducth-dev-website`
- **AND** the browser runtime configuration does not contain the exporter endpoint, sampler, or any telemetry credential

#### Scenario: Disabled or unusable telemetry fails open
- **WHEN** `ENABLED_OTLP_EXPORTER` is absent, is not exactly `true`, or the configured exporter cannot initialize or send traces
- **THEN** the website continues to serve `/healthz` and its existing reader routes with their established behavior
- **AND** it does not fail process startup or return telemetry details to a reader

### Requirement: SSR request and GraphQL trace-context propagation

For every traced non-health website request that enters the Express SSR process, the website SHALL create server-side HTTP trace data and preserve a valid incoming W3C trace context when one is supplied. The SSR Apollo GraphQL transport SHALL create or participate in a child outbound HTTP operation and inject the current W3C `traceparent` context into its request to `WEBSITE_PUBLIC_GRAPHQL_API_URL`, including the public immutable posts endpoint. The website SHALL not alter the GraphQL method, URL, document, variables, response, authorization behavior, or cache policy.

When the existing My-CMS API accepts that W3C context with its deployed OpenTelemetry request middleware, the website and API portions SHALL be searchable as one trace containing distinct website and API service segments. The website SHALL remain compatible with an API deployment that does not accept or export the context; such a deployment SHALL not change website response behavior.

#### Scenario: SSR GraphQL request carries trace context
- **WHEN** a sampled request for `/en` triggers server-side rendering and an Apollo GraphQL request
- **THEN** the GraphQL request retains its existing request contract and includes a syntactically valid W3C `traceparent` header derived from the active website request context
- **AND** the website server span and outbound GraphQL client span share the same trace identifier

#### Scenario: Existing caller context is continued
- **WHEN** a non-health reader request arrives with a valid W3C `traceparent` header
- **THEN** the website continues that trace context for its server-side spans and downstream GraphQL request
- **AND** it does not create a disconnected replacement trace solely because the request crossed the website boundary

#### Scenario: API telemetry availability does not affect reading
- **WHEN** the downstream API does not export or accept the propagated context
- **THEN** the website preserves its existing SSR response and error behavior
- **AND** the website does not retry, alter, or fail the GraphQL request because of telemetry

### Requirement: Shallow health probing and restricted website telemetry

The exact unauthenticated `GET /healthz` endpoint SHALL remain a shallow availability check and SHALL not create website telemetry spans, initiate SSR, invoke Apollo, or contact GraphQL or another external dependency. For every other traced SSR request, the server span SHALL record the approved raw client IP as `client.address`, raw User-Agent as `user_agent.original`, and bounded derived `user_agent.browser.name`, `user_agent.browser.version`, `os.name`, `os.version`, `device.type`, and `device.model` attributes. It SHALL use the first syntactically valid IP address in Traefik-normalized `x-forwarded-for`, then the direct TCP peer address, and SHALL ignore `x-real-ip`.

Traefik's `web` entry point SHALL trust inbound `X-Forwarded-*` only from the current Cloudflare CIDRs through `TRAEFIK_ENTRYPOINTS_WEB_FORWARDEDHEADERS_TRUSTEDIPS`; it SHALL NOT enable insecure forwarded-header mode. The website's published host port SHALL be bound to loopback only, preventing public traffic from bypassing the Traefik trust boundary. The raw IP and User-Agent are restricted operational data; Jaeger access and retention SHALL be limited to authorized operators.

Website telemetry SHALL NOT record or export request or response bodies, rendered HTML, GraphQL documents, GraphQL variables, cookies, authorization headers, other authentication material, telemetry configuration secrets, or complete query-string values. Failed SSR or GraphQL work SHALL mark the relevant span as failed without adding those prohibited values.

The change SHALL instrument server-side website execution only. It SHALL NOT add browser telemetry, analytics, browser exporter configuration, or browser-to-API trace propagation.

#### Scenario: SSR span identifies the exact requested path component
- **WHEN** a reader requests a non-health SSR path, including a nested localized page path
- **THEN** the website server span name is `METHOD /requested/path` and it records that requested path component in `url.path` and `http.route`
- **AND** query-string values are not recorded

#### Scenario: Health check remains untraced and dependency-free
- **WHEN** Docker or another caller sends `GET /healthz` to an enabled website process
- **THEN** the endpoint returns its established HTTP 200 shallow response without a website span or downstream GraphQL request
- **AND** the container health-check behavior remains independent of the API and collector

#### Scenario: Visitor client attributes identify the SSR requester
- **WHEN** a non-health traced SSR request arrives through Traefik from a Cloudflare CIDR with `x-forwarded-for` and a User-Agent
- **THEN** its website server span contains the first valid forwarded raw client IP, the raw User-Agent, and bounded derived browser, OS, and device attributes
- **AND** it does not attach those visitor attributes to the GraphQL client span or browser bundle

#### Scenario: Untrusted forwarding headers do not control client identity
- **WHEN** a request reaches Traefik from an address outside its Cloudflare CIDR trust list with a supplied `X-Forwarded-For` or `X-Real-IP`
- **THEN** Traefik does not trust the supplied forwarded value when constructing the request forwarded to the website
- **AND** the website obtains its client address from Traefik-normalized `X-Forwarded-For` or the direct peer address only

#### Scenario: Sensitive request and GraphQL data are excluded
- **WHEN** a traced reader request contains cookies, authorization material, URL query values, or a GraphQL request has a document and variables
- **THEN** exported website span attributes do not contain those values or rendered response content
- **AND** a failure records only bounded error/status information needed for operations

#### Scenario: Browser behavior remains uninstrumented
- **WHEN** a reader receives and hydrates an SSR response
- **THEN** the browser bundle does not initialize an OpenTelemetry SDK or exporter
- **AND** any browser-side GraphQL behavior retains its current direct-client contract
