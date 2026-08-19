## MODIFIED Requirements

### Requirement: Cross-cutting authorization and middleware are applied in every deployment mode
Auth, CORS, cookie, body-limit, and OpenTelemetry middleware SHALL be applied identically in two modes: (a) gateway-composed mode — the gateway applies the layers to the merged router; (b) standalone domain mode — the domain's own `bin` applies the same layers to its own router. In both modes, the middleware SHALL preserve Supabase JWT validation, token request extensions, writer/admin OR semantics, administrator-only routes, CORS behavior, tracing/OpenTelemetry layers, body limits, and cookie handling, and the HTTP 401 contract for unauthenticated requests SHALL remain unchanged. The cross-cutting middleware is NOT shared through a separate "foundation" crate; each domain that serves HTTP owns its own copies of the layer factories, and the gateway owns its own copies at the composition layer.

When `ENABLED_OTLP_EXPORTER=true`, the gateway-composed mode SHALL initialize the existing OTLP tracing pipeline before serving HTTP, preserve its exporter guard until the serving lifecycle ends, and apply the existing OpenTelemetry request layers to the fully merged gateway router. Every request served by that router — including public, protected, administrator, health, and auth-rejected requests — SHALL produce a request trace eligible for export using the configured OpenTelemetry environment variables. When `ENABLED_OTLP_EXPORTER` is false, absent, or invalid, the gateway SHALL start with its existing plain-text logging path and SHALL preserve route, authorization, and response behavior without requiring an OTLP collector.

#### Scenario: Protected domain route rejects missing auth in gateway-composed mode
- **WHEN** an unauthenticated request reaches a protected domain route registered through the gateway
- **THEN** the response is HTTP 401 with the existing error contract
- **AND** the domain handler is not invoked
- **AND** when OTLP export is enabled, the request produces an export-eligible HTTP trace with its outcome

#### Scenario: Protected domain route rejects missing auth in standalone mode
- **WHEN** the domain's `bin` is run standalone and an unauthenticated request reaches a protected route
- **THEN** the response is HTTP 401 with the existing error contract
- **AND** the domain handler is not invoked

#### Scenario: Gateway exports a public or health request when OTLP is enabled
- **WHEN** `ENABLED_OTLP_EXPORTER=true`, valid OTLP exporter configuration is present, and a request reaches a public or health route served by the gateway
- **THEN** the gateway emits an export-eligible HTTP request trace using the configured OpenTelemetry service identity and collector endpoint
- **AND** the trace records the request operation and response outcome

#### Scenario: Gateway preserves operational behavior when OTLP is disabled
- **WHEN** `ENABLED_OTLP_EXPORTER` is false, absent, or invalid and the gateway starts
- **THEN** the gateway installs its plain-text logging path without initializing OTLP export
- **AND** public, protected, administrator, health, and auth-rejected requests retain their existing routes, status codes, authorization decisions, and response contracts
