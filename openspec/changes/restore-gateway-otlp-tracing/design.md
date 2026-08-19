## Context

Production enables the existing OpenTelemetry configuration (`ENABLED_OTLP_EXPORTER=true`, service identity, OTLP trace endpoint, and sampler) and the collector is reachable, but gateway requests emit no traces. This is a gateway bootstrap regression, not a new telemetry capability or configuration change.

**Observed current state (2026-08-19):**

- `apps/api/gateway/src/main.rs` calls its private `init_observability()` before it builds runtime dependencies. That function installs only a `tracing_subscriber` text formatter; it creates no OTLP exporter and returns no exporter guard.
- `apps/api/domain_posts/src/observability.rs` exposes `observability::init() -> Option<OtelGuard>` for the existing OTLP subscriber and `init_text_logging()` for its plain-text fallback. Current repository search finds no caller of either helper from the gateway.
- `apps/api/domain_posts/src/domain/layers.rs` exposes `otel_layers()` returning `OtelInResponseLayer` and `OtelAxumLayer`; current repository search finds no caller. The gateway's `compose_routers` first merges public, protected, and administrator routes, applies per-mount auth, and then applies the cookie manager to the merged router.
- Before commit `6ef07eb`, the legacy API binary retained the OTLP guard in `main` and put both layers around every legacy router. The refactored gateway unintentionally omitted both connections.
- Docker Swarm already passes all relevant OTEL variables to `my-cms-api` and exposes the Jaeger collector. No public endpoint, data model, auth policy, or deployment-manifest change is required.

The code-review graph is unavailable for reliable analysis: its supplied index was built on `agent/add-repository-thumbnail`, while this worktree is on `hotfix/tracing_oltp`. The design therefore uses targeted current-source search, direct source reads, current gateway tests, deployment configuration inspection, and comparison with the pre-refactor binary. No graph findings are asserted.

## Goals / Non-Goals

**Goals:**

- Restore opt-in OTLP trace export for all HTTP traffic served by the gateway-composed router.
- Keep the existing OTLP guard alive until the gateway serving lifecycle ends so shutdown can flush the configured exporter.
- Preserve the existing plain-text logging behavior when OTLP export is disabled, absent, or parsed as invalid.
- Preserve route ownership, middleware behavior, Supabase authentication/401 behavior, status and response contracts, and service configuration.
- Provide deterministic automated coverage for router behavior and tracing-layer presence, then a deploy-time collector verification.

**Non-Goals:**

- Adding a telemetry backend, changing collector URLs, sampling, resource/service naming, log format, or deployment environment variables.
- Adding domain/business-operation instrumentation, metrics, retries, a durable telemetry queue, or collector health checks.
- Changing standalone-domain tracing, routes, authorization, database schema, migrations, or generated SeaORM entities.
- Turning collector reachability or exporter initialization failure into a gateway startup failure.

## Decisions

### 1. Reuse the domain-provided observability initializer at the gateway process boundary

**Driver:** `api-gateway-bootstrap` must export traces when the existing feature flag is enabled while retaining the legacy lifecycle semantics.

**Decision:** Replace the gateway's local text-only subscription setup with the existing `domain_posts::observability` helpers. Gateway startup will obtain the `Option<OtelGuard>` from `observability::init()` and retain that binding in `main` through the `axum::serve` future. When it returns `None`, the gateway will invoke `init_text_logging()` exactly once as the existing text-logging fallback. The migration CLI remains before observability initialization, as it is today.

The process boundary owns this lifetime because `OtelGuard` owns exporter shutdown/flush behavior; a temporary created in an initializer or router constructor can be dropped before the listener receives requests.

**Alternatives considered:**

- Reimplement OTLP subscriber construction in `gateway`: rejected because it duplicates the existing domain-owned initializer and risks drift in environment parsing, exporter setup, and shutdown behavior.
- Move the initializer to a new shared foundation crate: rejected because the canonical capability explicitly prohibits a new shared middleware foundation and this change needs no new architecture.
- Make OTLP mandatory and fail gateway startup when initialization fails: rejected because it changes the current optional operational contract and availability behavior.

**Consequences:** An enabled but misconfigured/unreachable exporter continues to use the existing initializer's non-fatal behavior; operators must diagnose the initialization/runtime error from existing logs and collector configuration. This change does not add a retry or health signal.

### 2. Instrument once at the fully merged gateway router boundary

**Driver:** Every route class must receive a root HTTP span without changing domain route registration or double-instrumenting per-domain routers.

**Decision:** In `compose_routers`, obtain `domain_posts::domain::layers::otel_layers()` and apply its `OtelInResponseLayer` and `OtelAxumLayer` around the router only after public, protected, and administrator mounts have been merged and the existing cookie-manager boundary has been preserved. Keep the current per-mount auth layers inside this outer telemetry boundary. This lets the request span include successful public/health traffic and rejected authentication responses, while preserving handler and auth execution order.

Axum applies `Router::layer` in reverse declaration order for inbound requests. The implementation must therefore retain the established pair ordering (`OtelInResponseLayer` then `OtelAxumLayer`) and place the pair as the outer cross-cutting wrapper in a way that preserves the cookie layer and does not alter the existing auth/response semantics. The two layers are applied regardless of exporter mode; with the plain-text subscriber, tracing remains non-exporting.

**Alternatives considered:**

- Add tracing to each `DomainService` router: rejected because health/public gateway routes and auth failures would remain uncovered, and individual domains could be double-instrumented.
- Instrument only protected routes: rejected because it omits health, public, administrator, and rejected-auth traffic required by the specification.
- Add the layers only when OTLP is enabled: rejected because it makes router behavior configuration-dependent and diverges from the existing standalone middleware shape; the subscriber controls export.

**Consequences:** The gateway has a single root HTTP span per request. Existing `#[instrument]` spans in domain handlers can become children of that root span. No request headers, credentials, tokens, or request bodies are newly recorded by this change.

### 3. Verify via a layered test and operational smoke check

**Driver:** Global tracing subscribers and real collectors are process/external state, while the regression is otherwise easy to reintroduce by removing either wiring connection.

**Decision:** Add focused gateway tests before the wiring change. They will exercise a composed router using a capture/local tracing subscriber or other deterministic test seam to prove that a public/health request and an auth-rejected protected request traverse the OpenTelemetry request layer, while retaining their existing status/handler behavior. Keep environment-mutating tests serialized using the existing `ENV_LOCK`; do not require a live Jaeger instance for unit tests. Add/retain a compilation-level test of the startup wiring sufficient to prevent the guard from being immediately dropped.

After deployment, execute a real request through the production gateway and query the configured collector for the deployed `OTEL_SERVICE_NAME`, operation, and response outcome. This operational check is the acceptance evidence for actual exporter-to-collector transport, which a unit test cannot prove.

**Alternatives considered:**

- Assert only that helper functions are called: rejected because it does not prove the merged router produces HTTP spans.
- Use a live Jaeger/OTLP collector in the standard Rust test suite: rejected because it makes CI/environment-dependent tests slow and flaky.
- Rely exclusively on production validation: rejected because wiring regressions would recur without a deterministic guardrail.

## Contracts and Affected Flow

There is no new REST, GraphQL, database, or configuration contract. The operational contract is:

```text
gateway main
  -> domain_posts::observability::init()
     -> Some(OtelGuard): retained through axum::serve; OTLP exporter is active
     -> None: domain_posts::observability::init_text_logging(); no exporter
  -> compose_routers()
     -> public + protected(auth) + administrator(auth)
     -> existing cookie-manager boundary
     -> OtelInResponseLayer + OtelAxumLayer around merged router
  -> request / health / protected / administrator / rejected auth
     -> root HTTP tracing span -> configured subscriber -> OTLP collector (enabled only)
```

`ENABLED_OTLP_EXPORTER` remains the startup-time boolean gate implemented by the existing helper: false, absent, and unparseable values use the text-logging branch. With valid OTLP configuration and sampling enabled, the existing OTLP environment variables determine collector export and service identity. The gateway shall neither inspect nor expose OTEL credentials, bearer tokens, cookies, request bodies, or other secret material as part of this restoration.

## Risks / Trade-offs

- **OTLP guard is scoped incorrectly and drops before serving** → keep its binding in `main` through the awaited server lifecycle; add focused review/tests of that lifetime.
- **Layer placement excludes a route class or changes auth/cookie behavior** → apply once only to the fully composed router; test public/health and rejected-auth flows alongside the existing cookie test.
- **Enabled exporter still yields no collector traces because deployment config/collector is invalid** → preserve non-fatal startup behavior, inspect container environment and logs, then send a post-deploy request and verify collector search using the actual service name.
- **Global subscriber tests interfere with concurrent tests** → avoid asserting a process-global exporter in parallel tests; use local capture seams and the repository environment lock.
- **New telemetry captures sensitive request data** → use the existing layer factory without adding fields or debug logging of headers, JWTs, cookies, bodies, secrets, or environment values.
- **Trace volume increases when `always_on` sampling is configured** → this restores the previously intended volume only; operators retain the existing sampling controls.

## Migration Plan

No database or SeaORM migration, backfill, entity regeneration, client compatibility window, or deployment-manifest change is required.

1. Land the focused gateway code and tests.
2. Run the API-focused and full repository verification gates.
3. Build and deploy the normal `my-cms-api` image with the existing OTLP configuration unchanged.
4. Confirm startup completes, issue a public or authenticated request through the deployed gateway, and search the configured collector for the deployed service name, request operation, and response outcome.
5. Monitor gateway logs, request error rates, and collector ingestion during the normal rollout window.

**Rollback:** redeploy the immediately preceding API image or disable `ENABLED_OTLP_EXPORTER` to return to text-only logging. Neither action changes data or public HTTP behavior. Roll back when the new image prevents startup, changes route/auth behavior, causes unacceptable request failures/latency, or produces unacceptable telemetry overhead.

## Open Questions

None blocking implementation. The existing initializer suppresses the detailed reason when OTLP subscriber initialization fails; surfacing that reason is a worthwhile follow-up observability improvement, but intentionally outside this narrowly scoped restoration.
