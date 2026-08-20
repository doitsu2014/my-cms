## Context

The Ducth reader runner image currently executes `wget ... http://localhost:3001/en` every 30 seconds. The Express catch-all handler reads the SSR template and calls `render(req.path)`; `render` runs Apollo `getDataFromTree`, which executes the public posts GraphQL query. With gateway OpenTelemetry export enabled, the resulting `POST /posts/graphql/immutable` is correctly visible in Jaeger as recurring API traffic.

Source evidence:

- `apps/ducth-dev-website/Dockerfile` contains the 30-second `/en` health check.
- `apps/ducth-dev-website/server.prod.mjs` registers static assets and then a catch-all SSR handler; no health route precedes it.
- `apps/ducth-dev-website/src/index.server.tsx` constructs the Apollo client and calls `getDataFromTree` on every SSR render.
- `apps/ducth-dev-website/package.json`, the website source, and the website Compose environment have no OpenTelemetry SDK/exporter initialization, tracer configuration, or propagation middleware. The `@opentelemetry/api` lockfile entry is transitive rather than an application dependency.
- The graph gate completed for `add-website-healthz-probe` (1,918 nodes, 18,636 edges; medium risk 0.50; graph build matches HEAD). It identifies the website application entry flow as affected but does not resolve the Express production-server route as a graph node, so the conclusions above come from targeted source and canonical-spec inspection.

## Goals / Non-Goals

**Goals:**

- Make Docker health checking prove only that the configured website process is alive and accepting HTTP requests.
- Prevent scheduled Docker probes from performing SSR or generating downstream public GraphQL requests/traces.
- Preserve the reader-facing `/en` response, health-check cadence, container startup configuration validation, and deployment topology.
- Provide deterministic regression coverage for the shallow endpoint and the Docker health-check target.

**Non-Goals:**

- Do not make `/healthz` a dependency-readiness check for GraphQL, media, or the API.
- Do not change API routes, gateway tracing, GraphQL behavior, Traefik, Docker Compose, environment variables, image base, or container privileges.
- Do not add Ducth website OpenTelemetry instrumentation in this incident repair.

## Decisions

### 1. Register `/healthz` before the SSR catch-all

**Decision:** The production Express server will register an exact unauthenticated `GET /healthz` route before static and catch-all SSR behavior. It will return a small fixed successful response and will not enter the render pipeline. The server continues to validate required website configuration at process startup, so a malformed or missing required environment value still prevents the process from becoming healthy.

**Rationale:** This is a liveness endpoint: it separates process availability from the reader content and dependency path that was creating the incident signal. An exact route avoids the current fallback handler, which calls `fs.readFile`, SSR rendering, Apollo, and GraphQL.

**Alternatives considered:**

- Keep `/en`: rejected because every probe performs the expensive SSR/Apollo flow and creates misleading periodic API traffic.
- Probe a static asset: rejected because it couples health to artifact presence and static middleware rather than proving the intended application endpoint dispatch, while adding no explicit operational contract.
- Make `/healthz` query GraphQL/API readiness: rejected because a content/dependency outage would mark a healthy website process unhealthy and reintroduce the traffic being removed. A future readiness endpoint, if needed, needs separate operational requirements.

### 2. Retarget only the Docker health-check command

**Decision:** Change only the URL in the existing `HEALTHCHECK` command to `http://localhost:3001/healthz`; retain the 30-second interval, timeout, start period, retry count, `wget` behavior, port, and non-root runner design.

**Consequences:** The reader still requires valid configuration on boot and remains reachable through the unchanged Traefik service. API/Jaeger will no longer receive probe-triggered SSR GraphQL traffic; normal reader visits continue to create GraphQL traffic as today.

### 3. Keep website tracing as a separate observability change

**Decision:** Do not add tracing in this hotfix. The website can be instrumented in a later, explicit change, but it currently cannot emit its own server-side traces because it neither initializes an OTel SDK/exporter nor configures service identity, sampling, propagation, or safe request attributes. That future change must establish a distinct website service name and OTLP endpoint, create an outbound HTTP/Apollo instrumentation boundary, propagate W3C trace context to the API, and avoid recording GraphQL documents, variables, cookies, authorization headers, or rendered content.

**Rationale:** Adding this cross-service observability contract would expand the incident repair beyond the smallest safe scope and requires product/operational decisions about sampling, trace retention, and sensitive-data policy.

## Contracts and verification design

`GET /healthz` returns HTTP 200 with a fixed non-sensitive response. It has no auth requirement, request body, external call, persistence, cache, or retry semantics. It does not change the public reader URL contracts.

Implementation must first add a focused Vitest regression seam for the production Express app. The endpoint test shall use render/template collaborators that fail if called, then verify `GET /healthz` succeeds and neither collaborator nor a local GraphQL test server is contacted. This may require extracting a minimal app-construction function from the listener bootstrap; it must preserve startup configuration validation and use no new production dependency. A separate source/artifact test shall assert the Dockerfile health-check command targets `/healthz` and retains its existing health-check parameters. Existing SSR coverage for `/en` remains the compatibility check; add a focused production-server test if the current suite does not exercise it.

## Risks / Trade-offs

- **A shallow liveness result does not prove GraphQL readiness** → This is intentional. Monitor reader request failures and API health separately; introduce a distinct readiness contract only with explicit requirements.
- **An unauthenticated public endpoint can be probed externally through the website router** → Return no state, version, dependency, or configuration details; use a constant response.
- **A server-module extraction for testability can alter boot timing** → Keep configuration validation and `app.listen` in the runtime bootstrap, cover the missing-config exit behavior, and review the startup diff.
- **Old image versions continue to send `/en` probe traffic during rollout** → Use a rolling/recreate deployment and observe API GraphQL traces until all reader tasks use the new image.
- **Future tracing could expose request/content data or cause duplicate spans** → Treat instrumentation as a separately reviewed cross-service change with privacy constraints and trace-propagation tests.

## Migration Plan

No database, entity, data backfill, API migration, or configuration migration is required.

1. Build the reader image with the new endpoint and Dockerfile probe.
2. Run the focused server and Dockerfile regression tests, then the website typecheck/lint/build gates.
3. Deploy only the `ducth-dev-website` image using the existing Compose/Swarm procedure; do not change the API or Jaeger services.
4. Confirm the container becomes healthy through `/healthz`, `/en` continues to render, and recurring health-check-caused `POST /posts/graphql/immutable` traces disappear after the old image is gone.

**Rollback:** redeploy the preceding known-good website image. If source rollback is required, revert the health-route addition and restore the Docker health-check URL to `/en`; no data or compatibility cleanup is necessary. This restores the prior probe traffic, so rollback approval must accept that operational cost.

## Open Questions

None for this hotfix. Website-owned outbound tracing is deferred to a separately approved observability proposal.
