## Why

The Ducth reader image probes its SSR route (`/en`) every 30 seconds. Each probe runs the server-rendering and Apollo data-prefetch path, causing avoidable public GraphQL calls that appear as periodic API traces in Jaeger and make health monitoring dependent on content-query availability.

## What Changes

- Add a shallow, unauthenticated `GET /healthz` endpoint to the production Ducth website server. A successful response proves that the Express process is accepting requests without rendering React, loading the SSR template, or contacting GraphQL, media, or other external services.
- Redirect the image's existing Docker `HEALTHCHECK` from `/en` to `/healthz` while retaining its timing, retry, startup-period, and non-root process behavior.
- Update the website deployment requirement and regression coverage to prove the health probe is shallow and the reader route continues to work independently.

## Capabilities

### New Capabilities

None.

### Modified Capabilities

- `website-deployment`: Replace the SSR-backed reader health-check target with the new shallow website liveness endpoint.

## Impact

- Affected website runtime and container artifact: `apps/ducth-dev-website/server.prod.mjs` and `apps/ducth-dev-website/Dockerfile`.
- Affected operational contract: Docker health reporting for `ducth-dev-website`; public reader traffic and API GraphQL contracts remain unchanged.
- Regression tests will cover the production-server health route and inspect the Docker health-check target. No schema, API, environment-variable, dependency, or deployment-manifest change is required.
- This hotfix deliberately does not add website OpenTelemetry instrumentation. Repository evidence shows the website currently has no OpenTelemetry initialization/export configuration or instrumentation dependency; tracing its outbound SSR GraphQL requests is feasible as a separately scoped observability change requiring service identity, exporter configuration, safe propagation, sampling, and privacy decisions.
