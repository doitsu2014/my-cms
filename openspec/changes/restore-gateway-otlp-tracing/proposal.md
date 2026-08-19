## Why

Production requests to `my-cms-headless-api` no longer produce searchable traces after the gateway refactor, despite OTLP export being enabled and the collector being reachable. This removes the operational visibility needed to investigate live API failures and latency.

## What Changes

- Restore gateway use of the existing OTLP observability initialization when OTLP export is enabled, while preserving plain-text logging when it is disabled.
- Ensure the gateway applies OpenTelemetry HTTP tracing to its fully composed router so inbound public, protected, administrator, health, and rejected-auth requests can produce request traces.
- Preserve trace export for the lifetime of the API process and its existing graceful shutdown behavior.

## Capabilities

### New Capabilities

None.

### Modified Capabilities

- `api-gateway-bootstrap`: Make the existing gateway-composed OpenTelemetry middleware requirement observable by requiring request traces to be emitted to the configured collector whenever OTLP export is enabled.

## Impact

- Affected runtime: the `my-cms-api` gateway startup and merged-router composition.
- Affected operational system: the configured OTLP collector and Jaeger trace search for the existing `my-cms-headless-api` service.
- No public API paths, response contracts, authorization rules, data schema, client behavior, or deployment configuration change.

## Scope and Non-Goals

In scope is restoring the already configured, opt-in production tracing path for gateway-served HTTP traffic. This change does not introduce a new telemetry backend, change sampling policy or service identity, redesign application logging, add business-operation instrumentation, or alter standalone-domain behavior.

## Assumptions and Dependencies

- Deployments that require traces continue to provide valid OTLP exporter settings and a reachable collector.
- The existing domain-provided observability initializer and OpenTelemetry router layers remain the shared behavior expected by the gateway bootstrap capability.

## Risks and Success Criteria

- Risk: applying cross-cutting tracing at the wrong router boundary could leave route classes untraced or inadvertently change middleware behavior.
- Risk: telemetry initialization failures may remain visible only through the existing logging path; no new operational configuration is introduced here.
- Success: with OTLP export enabled, a real gateway HTTP request appears in Jaeger under `my-cms-headless-api` within the collector's normal ingestion window, including its request operation and outcome.
- Success: with OTLP export disabled, the gateway continues to start and emit its existing plain-text logs; route, authentication, and response behavior remain unchanged in both modes.
