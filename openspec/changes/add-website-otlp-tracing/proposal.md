## Why

Operators can currently see only the API portion of a Ducth website SSR request in Jaeger. This leaves page-render latency and failures disconnected from their downstream GraphQL work, slowing diagnosis of user-facing incidents.

## What Changes

- Add server-side OpenTelemetry tracing for Ducth website SSR requests with a distinct website service identity.
- Correlate each traced SSR request with its downstream My-CMS API GraphQL work so operators can inspect the end-to-end request in Jaeger.
- Keep the `/healthz` probe intentionally quiet: it must remain a shallow availability check and must not produce website spans or downstream GraphQL traffic.
- Record only bounded, operationally useful trace attributes and avoid sensitive request, authentication, and GraphQL payload data.
- Preserve page availability when tracing is disabled, misconfigured, or unable to export telemetry.

## Capabilities

### New Capabilities

- `website-otlp-tracing`: Server-side SSR trace creation, safe export, and trace-context propagation from the Ducth website to its My-CMS API GraphQL requests.

### Modified Capabilities

None.

## Impact

- Affects the Ducth website server runtime, SSR request handling, and its GraphQL transport configuration.
- Introduces website-side OpenTelemetry runtime dependencies and deployment-time tracing configuration that must align with the existing internal OTLP/Jaeger observability path.
- Changes the operational troubleshooting experience by adding a distinct website service to Jaeger and linking website and API trace segments.
- Does not change API behavior, collector or Jaeger deployment, public website functionality, browser telemetry, analytics, or health-check semantics.
