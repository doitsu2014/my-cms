## Context

Operators can presently search Jaeger only for the API portion of an SSR reader request. The website has no OpenTelemetry dependency, SDK initialization, exporter, resource identity, or context-propagating GraphQL transport. The observed production request path is:

```text
Traefik (ducth.dev)
  -> ducth-dev-website:3001
     -> server.prod.mjs catch-all
        -> render(req.path)
           -> Apollo getDataFromTree
              -> createHttpLink / Node 20 fetch
                 -> POST WEBSITE_PUBLIC_GRAPHQL_API_URL
                    -> my-cms-api /posts/graphql/immutable
                       -> existing Axum OpenTelemetry middleware
                          -> Jaeger
```

`GET /healthz` is registered before the SSR catch-all and deliberately avoids the render and GraphQL path. The Docker runner probes that endpoint every 30 seconds. The existing API service already exposes OTLP gRPC on `jaeger:4317` and Jaeger's OTLP/HTTP receiver on `jaeger:4318`; the API's Rust runtime uses the gRPC setting. This Node service needs its own OTLP/HTTP endpoint and resource identity rather than inheriting the API values.

The graph gate completed before drafting: 1,933 nodes and 18,757 edges, low risk (0.40), current at `7e28b923`. It identified generic application entry flows but did not resolve the Express catch-all or Apollo SSR edge. The design therefore supplements graph evidence with direct source and test inspection of `server.prod.mjs`, `src/index.server.tsx`, `src/infrastructure/graphql/graphql-client.ts`, `src/test/production-server.test.ts`, the API gateway telemetry boundary, and Docker Compose configuration.

Canonical-spec note: `openspec/specs/website-deployment/spec.md` still describes `/en` as the health-check target, while the completed `add-website-healthz-probe` delta and the current Dockerfile correctly use `/healthz`. That completed change must be synced separately; this change preserves the source/current-delta behavior and does not reintroduce the stale contract.

## Goals / Non-Goals

**Goals:**

- Emit a distinct `ducth-dev-website` server-side trace segment for sampled SSR requests.
- Make the server-side Apollo request a child operation and inject W3C `traceparent` to link it to the existing API gateway trace when API OTLP is deployed.
- Keep telemetry opt-in, fail-open, bounded, restricted to server-side operational use, and invisible to browser runtime configuration.
- Keep `/healthz` shallow, dependency-free, and untraced.
- Provide deterministic local tests plus a deployment-time Jaeger acceptance check.

**Non-Goals:**

- Browser/RUM tracing, analytics, browser exporter configuration, consent UX, or browser-to-API context propagation.
- API, Rust gateway, GraphQL schema/operation, database, SeaORM, authentication, Traefik routing, or Jaeger collector changes.
- Metrics/log export, telemetry retries, a collector health check, or automatic instrumentation of unrelated Node libraries.
- Changing the existing API sampling policy, service name, or the `restore-gateway-otlp-tracing` scope.

## Decisions

### 1. Use an opt-in Node SDK with a website-specific deployment contract

**Driver:** Website telemetry must not silently reuse the API's gRPC endpoint or service identity, and unavailable telemetry must not reduce reader availability.

**Current state:** The website has no OTel packages or config. Compose passes generic `ENABLED_OTLP_EXPORTER`, `OTEL_SERVICE_NAME`, `OTEL_EXPORTER_OTLP_TRACES_ENDPOINT`, and `OTEL_TRACES_SAMPLER` only to `my-cms-api`; the Jaeger container exposes both port 4317 and 4318.

**Decision:** Add server-only OpenTelemetry dependencies (`@opentelemetry/api`, `@opentelemetry/sdk-node`, and `@opentelemetry/exporter-trace-otlp-http`, plus only the supporting packages required by the selected SDK version). A new ESM instrumentation bootstrap will run before the website application entry (`node --import ./instrumentation.mjs server.prod.mjs`). It will initialize exactly when `ENABLED_OTLP_EXPORTER` parses as `true`, set an explicit W3C propagator, create a `NodeSDK` with `service.name`, OTLP/HTTP trace exporter, and the configured sampler, and retain the SDK until process termination so shutdown can flush the batch exporter.

Compose will map website-scoped deployment variables to standard runtime OTel variables:

| Deployment input | Runtime variable | Default / constraint |
|---|---|---|
| `WEBSITE_ENABLED_OTLP_EXPORTER` | `ENABLED_OTLP_EXPORTER` | `false` |
| `WEBSITE_OTEL_SERVICE_NAME` | `OTEL_SERVICE_NAME` | `ducth-dev-website` |
| `WEBSITE_OTEL_EXPORTER_OTLP_TRACES_ENDPOINT` | `OTEL_EXPORTER_OTLP_TRACES_ENDPOINT` | `http://jaeger:4318/v1/traces` |
| `WEBSITE_OTEL_TRACES_SAMPLER` | `OTEL_TRACES_SAMPLER` | documented, operator-selected sampling policy |

The bootstrap validates only what it needs to avoid unsafe telemetry behavior. Any invalid/unsupported configuration, SDK construction failure, or exporter failure is fail-open: emit a bounded local warning without values and continue with the existing website runtime. The implementation must not add these variables to the JSON configuration embedded for the browser.

**Alternatives considered:**

- Reuse the API's generic Compose values and `jaeger:4317`: rejected because the Node implementation uses OTLP/HTTP and would collapse website and API traces under one service identity or use the wrong transport endpoint.
- Require a collector and fail startup on misconfiguration: rejected because observability is not a reader-availability dependency.
- Enable the SDK unconditionally: rejected because it changes local/production behavior and creates overhead where tracing is not requested.
- Use the broad `@opentelemetry/auto-instrumentations-node` bundle: rejected because it introduces a larger dependency graph and instruments unrelated operations without a bounded-attribute review.

**Consequences:** Deployment gains an explicit website observability contract and an operational toggle. The reader starts normally with tracing disabled. The image command and any production-server test launch helper must use the same preloading arrangement so instrumentation occurs before application modules initialize.

### 2. Instrument only the SSR request boundary and Apollo transport explicitly

**Driver:** The website needs an end-to-end trace without duplicate spans or accidental capture of HTML, query values, GraphQL documents, variables, headers, or credentials.

**Current state:** `server.prod.mjs` calls `render(req.path)` from one SSR catch-all. `index.server.tsx` constructs a new Apollo client per render. `graphql-client.ts` delegates HTTP transport to Apollo's `createHttpLink`, which uses Node 20 global `fetch` on the server.

**Decision:** Implement a small server-only telemetry module with two explicit boundaries:

1. The catch-all SSR route extracts W3C context from the request and runs the existing render/template/response work in a bounded server span named `METHOD /requested/path`. Its attributes include the requested path component as both `http.route` and `url.path` (not its query string), plus method, protocol, and response/error outcome. The trace ID remains the OpenTelemetry-generated identifier required for cross-service correlation. It records an error status without an exception message or body.
2. The server-side Apollo `HttpLink` receives a telemetry-aware `fetch` wrapper. That wrapper starts a child client span, injects W3C context into outgoing headers, and records only the GraphQL HTTP method, endpoint origin/path, response status, and bounded error outcome. Browser client creation continues to use normal fetch with no SDK or propagation wrapper.

The `/healthz` route remains before this SSR boundary; therefore it has no server span, no client span, and no GraphQL work. The bootstrap installs Node async context management and a W3C trace-context propagator before these modules load. Termination handlers shut down the SDK once without changing Express's established response behavior.

**Alternatives considered:**

- HTTP + Express + Undici auto-instrumentation: rejected for this scope because it can create multiple framework/transport spans for one request and requires a separate, library-specific review to strip potentially unbounded URL/header attributes.
- Add a span only around `render`: rejected because it misses a stable inbound request boundary, cannot continue a caller's W3C parent cleanly, and makes response status/error outcome harder to record.
- Instrument browser Apollo as well: rejected because browser telemetry is experimental in the current JavaScript ecosystem and needs an explicit product/privacy/consent decision; it also changes the user-facing data collection boundary.
- Modify the API GraphQL handler to accept a custom correlation ID: rejected because Axum's existing OpenTelemetry middleware already accepts standard trace context, and API behavior is outside scope.

**Consequences:** One conceptual website server span and one client span are created per sampled SSR GraphQL flow, with trace linkage driven by the standard W3C header. Exact implementation must avoid a second competing automatic HTTP instrumentation. The website remains compatible if an API instance ignores context: reading works, but Jaeger will show a disconnected API segment.

### 3. Treat trace data as restricted operational metadata, with approved visitor identifiers

**Driver:** Operators need to identify the browser, device, and source IP behind slow or failing SSR requests. The public reader's URL, other inbound headers, SSR content, and GraphQL requests may nevertheless contain user or content data unsuitable for Jaeger.

**Decision:** Explicit instrumentation writes an allow-list only. On the non-health SSR server span it additionally records the user-approved raw visitor identifiers: `client.address` and `user_agent.original`, each bounded to prevent untrusted oversized header values. It derives bounded, non-raw dimensions from that User-Agent: `user_agent.browser.name`, `user_agent.browser.version`, `os.name`, `os.version`, `device.type`, and `device.model`. The address selection order is the first syntactically valid value in `x-forwarded-for`, then the direct TCP peer address. `x-real-ip` is intentionally ignored because the Traefik edge normalizes `X-Forwarded-For` and only trusts that family of incoming forwarded headers from Cloudflare CIDRs.

Traefik receives the Cloudflare CIDR list through its native `TRAEFIK_ENTRYPOINTS_WEB_FORWARDEDHEADERS_TRUSTEDIPS` environment setting, with `forwardedHeaders.insecure` left disabled. The published Cloudflare ranges must be refreshed before production rollout. The website's host-port mapping is loopback-only, so public traffic cannot bypass Traefik and inject a forwarded value directly into Express.

It must never set `url.full`, query-string values, request/response body values, rendered HTML, GraphQL document/operation text, GraphQL variables, cookies, `Authorization`, other auth headers, exporter headers, or environment-variable values. Error handling records a stable error category/status and response status only; it must not attach a caught error message, stack, or upstream response body. The exporter must receive no custom headers unless a separately reviewed credential contract is introduced. Jaeger access and retention must be limited to authorized operators because raw IP and User-Agent values are personal data.

**Alternatives considered:**

- Rely on instrumentation-library defaults and document that operators must avoid sensitive requests: rejected because defaults are version-dependent and do not meet the explicit privacy guarantee.
- Record operation names/documents to ease debugging: rejected because GraphQL documents and variables can carry content or identifiers; the route/status relationship is sufficient for this change.
- Hash or truncate the IP/User-Agent: rejected because the requested incident workflow requires the full raw values; bounded storage limits only prevent malformed oversized inputs from expanding trace size.
- Sample every request permanently: rejected because trace volume and cost must remain operator-controlled through `OTEL_TRACES_SAMPLER`.

**Consequences:** Jaeger can answer which route called which backend endpoint, duration, status, trace relationship, and which client/browser/device initiated it, but cannot reproduce user content or GraphQL input. Raw visitor identifiers increase access-control and retention obligations; if richer diagnostics are needed later, they require a new data-classification design.

### 4. Verify transport locally and linkage operationally

**Driver:** A unit test can prove context and emitted spans but cannot prove the deployed collector, network, and API instrumentation all ingest one trace.

**Decision:** Add deterministic test seams for enabled/disabled telemetry and an in-memory span exporter. The production-server integration test's local GraphQL stub will assert a syntactically valid `traceparent`; in-memory spans will assert server/client span identity, parentage, raw forwarded IP/User-Agent selection, parsed browser/OS/device fields, bounded attributes, health suppression, and no browser initialization. An enabled local fake OTLP/HTTP receiver may additionally assert export payload receipt without depending on Jaeger. Existing reader-route and health-check tests remain compatibility coverage.

After deployment, the Release Engineer will use an SSR request through the deployed reader and confirm Jaeger presents `ducth-dev-website` plus `my-cms-headless-api` in a shared trace, with `POST /posts/graphql/immutable` beneath the website request. This step depends on the existing `restore-gateway-otlp-tracing` change being deployed and healthy; it does not authorize a gateway change here.

**Alternatives considered:**

- Require a live Jaeger in the normal Vitest suite: rejected because it makes developer/CI tests environment-dependent and does not isolate propagation failures.
- Assert only that OTel packages are imported: rejected because it cannot protect initialization order, data redaction, active context, or outbound injection.
- Declare cross-service linkage complete based only on `traceparent` at the mock API: rejected because that does not prove deployed collector ingestion and API parent extraction.

**Consequences:** Tests remain fast and deterministic, while production acceptance has a concrete, authorization-bounded observable outcome.

## Contracts and affected flow

```text
Docker/Node command
  -> --import instrumentation.mjs
     -> parse opt-in configuration
     -> NodeSDK + OTLP/HTTP exporter + W3C propagator (or no-op fail-open)
  -> server.prod.mjs
     -> GET /healthz: constant 200, no trace, no SSR
     -> reader catch-all: extract parent -> bounded server span
        -> index.server.tsx -> Apollo client
           -> server-only traced fetch: child span + inject traceparent
              -> POST /posts/graphql/immutable
                 -> existing API OtelAxumLayer extracts parent
                    -> Jaeger displays linked service segments
```

There is no public REST/GraphQL schema, database, migration, authorization, caching, or browser-runtime contract change. The new operational contract is the startup-only website telemetry configuration table above. It follows API-side `ENABLED_OTLP_EXPORTER` semantics but is sourced from website-specific Compose inputs to avoid collision with the API configuration.

## Risks / Trade-offs

- **Bootstrap runs after an instrumented module or test bypasses `--import`** → keep all telemetry setup in the preload module, make Docker/start/test launch paths use it, and assert context propagation in an end-to-end local process test.
- **Wrong gRPC/HTTP endpoint or endpoint path prevents ingestion** → use `http://jaeger:4318/v1/traces` for the Node OTLP/HTTP exporter, validate compose rendering, and verify a local receiver plus post-rollout Jaeger.
- **Exporter SDK/config failure affects availability** → catch telemetry-only initialization/export errors, install a no-op path, and preserve existing server startup/error behavior.
- **Spoofed forwarded IP reaches Express** → Traefik trusts `X-Forwarded-*` only from Cloudflare CIDRs, the application uses only its normalized `X-Forwarded-For`, and the website host port is loopback-only; refresh Cloudflare CIDRs before rollout and keep origin firewalling restricted to Cloudflare.
- **Raw visitor identifiers have broader access/retention impact** → the user has explicitly approved raw IP and User-Agent export; constrain attributes to server spans, bound their size, restrict Jaeger access/retention, and inspect finished spans for both the approved fields and forbidden content/credentials.
- **Sampling creates too much traffic or hides incident traces** → expose the sampler as an operator-controlled website setting; monitor export volume and request latency during rollout.
- **The API tracing restoration is not deployed** → website spans and outgoing `traceparent` still work, but the Jaeger cross-service acceptance criterion is deferred until `restore-gateway-otlp-tracing` is deployed.
- **Health checks reappear in Jaeger** → keep the exact route ahead of telemetry middleware/handler and add a regression assertion of zero spans and zero GraphQL calls.

## Migration Plan

No database schema, SeaORM generation, backfill, API compatibility window, data migration, or Jaeger collector deployment change is required.

1. Add test seams and failing website tracing/privacy/propagation tests.
2. Add the server-only dependency set, preload bootstrap, explicit spans, Apollo fetch wrapper, and lifecycle handling.
3. Add website-specific Compose environment mappings and `.env.example` documentation; verify rendered Compose config uses `ducth-dev-website` and OTLP/HTTP `jaeger:4318/v1/traces` without exposing telemetry values to browser configuration.
4. Run targeted website tests and full build/typecheck/lint, then a Docker image build and Compose config validation.
5. With release authorization, roll out only the website image/config using the normal rolling procedure. Confirm `/healthz` remains 200/untraced, `/en` renders, exporter warnings are absent, and Jaeger shows the linked trace after old website tasks drain.

**Rollback:** disable `WEBSITE_ENABLED_OTLP_EXPORTER` and redeploy the website service, or redeploy the immediately preceding reader image. Both remove only website telemetry; they leave `/healthz`, API, Jaeger, GraphQL data, and database state untouched. Roll back if website startup fails, reader error/latency rises, sensitive attributes are observed, collector volume is unacceptable, or the endpoint configuration cannot export as expected.

## Open Questions

None block implementation. Operators must choose a production sampling value appropriate to trace-volume and retention budgets before enabling the feature outside local development; the documented default is deliberately disabled.
