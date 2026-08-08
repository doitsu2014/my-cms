## Context

The public reader for `ducth.dev` currently lives in a sibling repository at `my-blogs-rsbuild/client_side/` as a React 19 + Rsbuild + Express 5 SSR application. Its source files (`App.tsx:1-18`, `AppContent.tsx:1-44`, `index.client.tsx:1-16`, `index.server.tsx:1-39`, `server.prod.mjs:1-175`, `pages/*.tsx`, `infrastructure/graphql/queries.ts:1-169`) and its three-stage `node:20-alpine` `Dockerfile:1-57` already implement SSR + hydration, locale-prefixed routing, public GraphQL queries against the same Rust API, and media URL resolution. What the sibling repo lacks is (a) environment-backed runtime configuration (it hardcodes `https://my-cms-api.ducth.dev/graphql/immutable` in `src/config/api.config.ts:7-8`, `https://my-cms-api.ducth.dev/media/{path}` across all four pages, `https://my-blogs.ducth.dev` in `src/config/site.config.ts:8`, and a favicon URL in `index.html:7-8`); (b) safe SSR HTML injection (the existing `server.prod.mjs:140-142` `JSON.stringify(apolloState).replace(/</g, '\\u003c')` only escapes `<`); and (c) co-location with the API and admin it talks to.

The target repo `my-cms` already runs the admin at `apps/web/` using nginx + runtime-injected `config.js` (see `apps/web/src/config/runtime-config.ts:32-57`, `apps/web/entrypoint.sh:9-31`, `apps/web/Dockerfile:1-37`, `apps/web/nginx.conf:1-40`). It also runs the same Rust API on port 8989 with a public GraphQL endpoint at `/graphql/immutable` (`apps/api/src/bin/my-cms-api.rs:108-113`) and a permissive CORS layer (`apps/api/src/bin/my-cms-api.rs:320-330`). The local Docker Swarm stack at `deployments/docker-swarm/` is composed of three sub-stacks (Supabase, apps, Traefik) on a shared external `supabase_network`, with Traefik routing via labels in `deployments/docker-swarm/apps/docker-compose.yaml` for the admin and a file-based dynamic config in `deployments/docker-swarm/traefik/dynamic/my-cms.yml:13-143` for Host-rule routing.

The Phase 1 handoff confirmed: identity decision (standalone pnpm app at `apps/ducth-dev-website/`, pnpm package name `ducth-dev-website`, Compose service + container name `ducth-dev-website`, Traefik router `website`, Traefik service `website-service`, Traefik Host `ducth.dev`); preserve SSR (no nginx, no CSR-only rewrite); parameterize hardcoded URLs via a single server-controlled runtime configuration injected into SSR HTML and consumed by the browser bundle via `window.__APP_CONFIG__`. The code-review-graph MCP gate was unavailable in this session; all evidence above was gathered by direct repository inspection (no findings were fabricated).

## Goals / Non-Goals

**Goals:**
- Move the public reader source from `my-blogs-rsbuild/client_side/` to `apps/ducth-dev-website/` with **no visual, route, query, or i18n changes**.
- Replace every hardcoded URL or brand string in the migrated source with a single server-controlled runtime configuration.
- Add a three-stage `node:20-alpine` Dockerfile that produces a non-root Express runner on port 3001 with a `wget /en` healthcheck.
- Add a Docker Compose service in `deployments/docker-swarm/apps/docker-compose.yaml` on `supabase_network`, depending on `my-cms-api`.
- Add a Traefik Host(`ducth.dev`) router + service in `deployments/docker-swarm/traefik/dynamic/my-cms.yml` and extend the existing `api-cors` allow-list with `https://ducth.dev`.
- Provide a small unit-test surface for the runtime-config contract (URL joining, JSON escaping, required-env validation).

**Non-Goals:**
- Kubernetes / Helm chart for the reader (the existing `deployments/k8s/charts/my-cms-admin/` and `deployments/charts/my-blogs/` are not extended).
- CI image publication to Docker Hub (`doitsu2014/ducth-dev-website` is reserved as a future deployment convention only; no GitHub release workflow is added in this change).
- Visual redesign, copy changes, route renames, or i18n locale additions.
- Replacement or merge of the admin app at `apps/web/`. The admin remains untouched.
- Backend feature changes (no GraphQL schema edits, no new my-cms-api endpoints, no Supabase migrations).
- Deletion of the `my-blogs-rsbuild` repository. That repo remains the source of truth for the reader until this migration is verified end-to-end; deletion is explicitly out of scope.
- Traefik dashboard, basic-auth, or Kong changes. Only the reader's router and the `api-cors` allow-list are touched.

## Decisions

### D1. Standalone pnpm app at `apps/ducth-dev-website/`, mirroring `apps/web/`

**Choice**: create a fresh standalone pnpm app at `apps/ducth-dev-website/` with its own `package.json`, `pnpm-workspace.yaml`, `pnpm-lock.yaml`, `tsconfig.json`, `eslint.config.mjs`, `postcss.config.mjs`, `tailwind.config.ts`, `.env.example`, `.dockerignore`, `.gitignore`. The pnpm package `name` SHALL be `ducth-dev-website`.

**Rationale**: this is the same pattern already proven by `apps/web/` (which carries its own `pnpm-workspace.yaml:1-2` and is consumed by `deployments/docker-swarm/apps/docker-compose.yaml:144` as `context: ../../../apps/web`). Standalone apps keep lockfiles independent, let the reader redeploy without rebuilding the admin, and avoid dragging `apps/api` Cargo deps into a pnpm resolution graph.

**Alternatives considered**:
- *Top-level pnpm workspace with `apps/*` peers* — would couple admin and reader lifecycles; rejected because of the cross-cutting blast radius for redeploys.
- *Sub-app of `apps/web/`* — would require either two Dockerfiles in one app (nginx + node) or a forced CSR conversion; rejected because it breaks the SSR contract and forces the admin to redeploy for reader-only changes.
- *Replace `apps/web/` with `apps/ducth-dev-website/`* — removes the admin; rejected because it would orphan `openspec/specs/supabase-auth/`, `media-bucket-management/`, `user-management/`, etc.

### D2. Server is the single source of truth for runtime configuration

**Choice**: the Express server reads the `WEBSITE_*` env contract once at startup, validates each value, and inlines the resolved payload into every SSR HTML response as `<script id="app-config" type="application/json">…</script>`. The browser bundle reads `window.__APP_CONFIG__` only and never reads `process.env` at runtime.

**Rationale**: the existing admin pattern (`apps/web/src/config/runtime-config.ts:32-57`) and the SSR pattern in `my-blogs-rsbuild/client_side/server.prod.mjs:37-43` already encode this direction. Compose env vars cannot mutate an already-built browser bundle (`pnpm run build` happens before `docker compose up`); only the server is alive at request time and can resolve per-host URLs without a rebuild.

**Alternatives considered**:
- *Build-time `PUBLIC_*` envs baked into the rsbuild bundle (admin style)* — forces one image per host; rejected because it removes the ability to ship one image and rotate env per environment.
- *Hardcoded production URLs (current state)* — rejected for the reasons that motivated this change.
- *Both build-time and runtime, with runtime overriding build-time* — adds surface area without a need; rejected as over-engineering for the first migration.

### D3. SSR HTML injection uses seven-character JSON escaping

**Choice**: the SSR handler escapes `<`, `>`, `&`, `'`, `"`, U+2028, and U+2029 in the JSON payload before injecting it. The script element uses `type="application/json"` so the browser does not execute it. The client reads the script tag's **text content** with `JSON.parse`.

**Rationale**: the existing `server.prod.mjs:140-142` `JSON.stringify(apolloState).replace(/</g, '\\u003c')` only escapes `<`. The OWASP JSON-in-HTML guidance (and RFC 8259 §7) recommends the seven-character set because each character can break the embedded JSON in HTML parsing, XSS, or strict parsers (notably U+2028/U+2029 break JavaScript parsers and were never valid in JSON until ES2019). Using `type="application/json"` matches the WHATWG HTML spec for "data blocks" so the browser will not execute it and the inline payload cannot be interpreted as script.

**Alternatives considered**:
- *Escape only `<`* (current behavior) — insufficient; vulnerable to `</script>` injection if a value contains that sequence.
- *Base64-encode the payload* — works but requires a client decoder and obscures debugging; rejected as unnecessary complexity.

### D4. Required-env validation exits non-zero with named variable

**Choice**: at startup, the server iterates the required variable list and exits non-zero with a single log line naming the variable if it is missing or `new URL(value)` throws. Optional variables fall back to documented defaults.

**Rationale**: a misconfigured image that boots, binds 3001, and 500s on every request is harder to diagnose than an image that refuses to start. The healthcheck (`wget http://localhost:3001/en`) doubles as the liveness signal, so an exited container is visible immediately.

**Alternatives considered**:
- *Lazy validation at first request* — masks configuration errors; rejected because compose restart loops are the right place to surface them.
- *Best-effort defaults for required values* — unsafe; the reader depends on these URLs to function.

### D5. SSR error handler logs method + path + correlation id only

**Choice**: the SSR `try`/`catch` blocks log a single structured line: HTTP method, URL path, and a generated UUID-shaped correlation id. The response body is a generic HTML 500 page that omits the original error message. Stack traces, request bodies, and error messages are not logged.

**Rationale**: the existing `server.prod.mjs:159-162` already returns a generic `Server Error` body but its `console.error('SSR Error:', err)` line in `:160` writes the full error to stdout, including any Apollo error messages that may contain server-side data. Container logs are typically shipped to centralised log storage where they outlive the request; sanitising at the source is the only safe pattern.

**Alternatives considered**:
- *Log the full error for debugging* — rejected as a privacy and security risk (logs may be retained for months and may leak data through backend error messages).
- *No logging at all* — rejected because operators need a way to correlate client reports with server-side state.

### D6. Static Traefik dynamic config is the single source of routing truth

**Choice**: the Host rule, the entrypoint, and the load-balancer URL for the `website` router live exclusively in `deployments/docker-swarm/traefik/dynamic/my-cms.yml`. The `ducth-dev-website` Compose service adds **only** the `traefik.http.routers.website.middlewares=website-compress` label. No `Host(...)` label, no `entrypoints` label, no `loadbalancer.server.port` label on the website service.

**Rationale**: the existing admin router is split this way too (`deployments/docker-swarm/apps/docker-compose.yaml:162-168` adds only the `middlewares` and `compress` labels, while the router definition lives in the dynamic file `deployments/docker-swarm/traefik/dynamic/my-cms.yml:15-23`). Duplicating the rule in both places would produce a duplicate-router validation error from Traefik or, worse, drift silently. Using the static dynamic file as truth means all four Host-based routers (`admin`, `api`, `studio`, `supabase-api`, `website`) are defined next to each other in a single YAML file that can be reviewed and linted.

**Alternatives considered**:
- *Compose labels for everything (admin style pre-split)* — would create a duplicate-router error because both label sets register a router named `website`; rejected.
- *Compose labels for the website router only* — drifts from the existing pattern; rejected for consistency.

### D7. `api-cors` allow-list extended, not replaced

**Choice**: add `https://ducth.dev` to the existing `api-cors.accessControlAllowOriginList` in `deployments/docker-swarm/traefik/dynamic/my-cms.yml:88-105`. Do not introduce a separate `website-cors` middleware.

**Rationale**: the API already accepts any origin at the Axum layer (`apps/api/src/bin/my-cms-api.rs:329` `.allow_origin(Any)`); the Traefik-level `api-cors` exists mainly to add CORS headers to proxied responses so browser preflights succeed. Extending the existing list is the minimum-touch change.

**Alternatives considered**:
- *New `website-cors` middleware* — duplicate surface; rejected.
- *Disable Traefik-level CORS and rely on Axum* — would change behavior for the admin; rejected as out of scope.

### D8. Healthcheck on `/en`

**Choice**: `HEALTHCHECK CMD wget --no-verbose --tries=1 --spider http://localhost:3001/en || exit 1`.

**Rationale**: `/en` is reachable regardless of configuration because the SSR handler's locale-prefixed routing short-circuits to `/en` content for the default locale, and `wget` is present in `node:20-alpine` (matches `my-blogs-rsbuild/client_side/Dockerfile:53-55`).

**Alternatives considered**:
- *`/health` endpoint added to the Express server* — would duplicate the `/health` contract already in `apps/web/nginx.conf:35-39`; rejected because it requires new code that is not strictly necessary for the migration.
- *TCP-port check* — less informative than an HTTP probe.

### D9. Three-stage Dockerfile, no nginx

**Choice**: the runner stage is `node:20-alpine` running `node server.prod.mjs`. No nginx is added.

**Rationale**: the SSR contract requires a Node.js process (Express + `renderToString` + Apollo `getDataFromTree`). Adding nginx would either be a no-op (if it just reverse-proxies to localhost:3001) or a behaviour change (if it terminates the HTTP connection). The non-root pattern (`expressjs:1001`) matches `my-blogs-rsbuild/client_side/Dockerfile:33-34`.

**Alternatives considered**:
- *nginx → Express upstream* — adds a network hop without value; rejected.
- *Single-stage `node:20-alpine` build* — would ship `devDependencies` and `src/` to production; rejected as insecure and bloat-heavy.

### D10. Compose depends_on uses `service_started` (not `service_healthy`)

**Choice**: `depends_on: my-cms-api { condition: service_started }`.

**Rationale**: the existing admin uses `service_started` (`deployments/docker-swarm/apps/docker-compose.yaml:157-158`). Switching the reader to `service_healthy` would require adding a Docker healthcheck to the API image, which is out of scope. The Express server's startup-time GraphQL URL validation (D4) means the reader will fail fast and restart-loop if the API is unreachable.

**Alternatives considered**:
- *`service_healthy`* — better signal but requires an API Dockerfile change; deferred.

### D11. One image, multiple env contracts (no per-host rebuild)

**Choice**: the Dockerfile builds a single image whose runtime behavior is fully controlled by `WEBSITE_*` env vars. No rebuild is required to switch from `ducth.dev` to a staging hostname.

**Rationale**: matches the admin pattern (`apps/web/Dockerfile:1-37` produces one image consumed by every environment) and aligns with the Phase 1 user decision to avoid per-host rebuilds.

**Alternatives considered**:
- *Per-host Dockerfiles* — operational overhead; rejected.

### D12. Tests target the runtime-config contract, not the SSR rendering

**Choice**: vitest unit tests cover (a) `getMediaUrl` joining, (b) the seven-character JSON escaping, (c) the required-env validation. SSR end-to-end behaviour is verified through the Compose smoke gate (`curl /en` returns 200).

**Rationale**: the existing `my-blogs-rsbuild/client_side` has zero tests (`client_side/.gitignore` and `package.json:33-48` show no vitest). Adding SSR integration tests in this change would expand scope significantly. The configuration contract is the highest-leverage area to test because that is where the most security- and correctness-sensitive logic lives.

**Alternatives considered**:
- *Render-to-string integration tests with vitest + jsdom* — useful but out of scope for the migration.
- *Snapshot tests on the SSR HTML* — brittle; rejected for this change.

## Risks / Trade-offs

- **SSR HTML injection XSS** → mitigated by seven-character JSON escaping (D3) and `type="application/json"`. Verified by a vitest scenario for `&`, `</script>`, and U+2028.
- **Hardcoded URL drift after the migration** → mitigated by an `rg` scan in CI that fails if `https://my-cms-api.ducth.dev` or `https://my-blogs.ducth.dev` reappears under `apps/ducth-dev-website/src/`. Captured as a scenario in `specs/website-frontend/spec.md` ("Hardcoded strings are absent from src").
- **Duplicate Traefik router** → mitigated by D6 (single source of truth in the dynamic file). The Compose file adds only middlewares, not Host rules.
- **Required env missing at startup** → mitigated by D4 (fail-fast exit with named variable).
- **Server-side error leak** → mitigated by D5 (correlation-id logging, generic 500 body).
- **i18n locale route colliding with API/admin on the same Host** → mitigated by Host-based routing only (`Host(ducth.dev)` ≠ `Host(cms-admin.ducth.dev)`). Path-based multiplexing is not used.
- **Seaography `parentId.is_null: "true"` shape** → flagged for verification with a one-query smoke test during implementation. If the GraphQL layer rejects the string form, the query becomes `isNull: true`; either way the contract on `apps/ducth-dev-website/src/infrastructure/graphql/queries.ts` is preserved verbatim.
- **Container memory** → matches existing `clientSide` Helm limits of 200m/128Mi (`deployments/charts/my-blogs/values.yaml:148-156`); not enforced in Compose but documented in the deployment spec.
- **Source-repo deletion premature** → mitigated by explicit non-goal; the migration can be rolled back by reverting the PR without touching `my-blogs-rsbuild`.
- **Graph MCP unavailable** → direct repository inspection substituted; this is recorded here and in the proposal so the change is reproducible without the MCP server.

## Migration Plan

The migration proceeds in five verification-gated steps. Each step ends at a checkpoint the SE can re-run from a clean checkout.

1. **Scaffold the empty app** — create `apps/ducth-dev-website/` with the boilerplate (`package.json`, `pnpm-workspace.yaml`, `pnpm-lock.yaml`, `tsconfig.json`, `eslint.config.mjs`, `postcss.config.mjs`, `tailwind.config.ts`, `.env.example`, `.dockerignore`, `.gitignore`) and a placeholder `src/index.tsx`. **Checkpoint**: `pnpm --dir apps/ducth-dev-website install --frozen-lockfile && pnpm --dir apps/ducth-dev-website build` succeeds with the empty placeholder.
2. **Migrate source verbatim** — copy `App.tsx`, `AppContent.tsx`, `index.client.tsx`, `index.server.tsx`, `server.prod.mjs`, `pages/*`, `components/layout/*`, `i18n/*`, `infrastructure/graphql/*`, `config/*`, `rsbuild.config.ts`, `index.html` from `my-blogs-rsbuild/client_side/` into `apps/ducth-dev-website/`. **Checkpoint**: `pnpm lint && pnpm typecheck && pnpm build` succeeds, both `dist/client/index.html` and `dist/server/index.mjs` are produced. The hardcoded URLs are still present and not yet resolved at runtime; this is expected.
3. **Introduce the runtime-configuration contract** — add `src/config/runtime-config.ts`, `src/config/escape-json.ts`, `src/config/validate-env.ts`, `getMediaUrl` helper, and update the page components to read `window.__APP_CONFIG__`. Wire the SSR handler to read `process.env` and inject the script tag with seven-character escaping. Add vitest scenarios for the helpers. **Checkpoint**: `pnpm test` passes, the `rg` scan returns no hits for `https://my-cms-api.ducth.dev` or `https://my-blogs.ducth.dev` under `apps/ducth-dev-website/src/`.
4. **Add Dockerfile and Compose service** — write `apps/ducth-dev-website/Dockerfile` (D9), add the `ducth-dev-website` block to `deployments/docker-swarm/apps/docker-compose.yaml` (D2, D10), update `deployments/docker-swarm/apps/.env.example` and `deployments/docker-swarm/traefik/.env.example` (D6, D7). **Checkpoint**: `docker build -f apps/ducth-dev-website/Dockerfile apps/ducth-dev-website` succeeds; `docker compose -f deployments/docker-swarm/apps/docker-compose.yaml --env-file deployments/docker-swarm/apps/.env build ducth-dev-website` succeeds.
5. **Add Traefik router and CORS extension** — extend `deployments/docker-swarm/traefik/dynamic/my-cms.yml` (D6, D7). **Checkpoint**: full stack starts, `curl -i -H "Host: ducth.dev" http://localhost/` returns the reader HTML with `Content-Encoding: gzip`; `curl -i http://localhost:8989/graphql/immutable` smoke gate returns 200; the existing admin and API are unaffected.

**Rollback** (independent per the spec):
- Delete the `ducth-dev-website` block from `deployments/docker-swarm/apps/docker-compose.yaml`.
- Delete the `website` router, `website-service`, and `website-compress` middleware from `deployments/docker-swarm/traefik/dynamic/my-cms.yml`; remove `https://ducth.dev` from `api-cors.accessControlAllowOriginList`.
- Delete `CMS_WEBSITE_HOST` from `deployments/docker-swarm/traefik/.env.example` and the `WEBSITE_*` entries from `deployments/docker-swarm/apps/.env.example`.
- Optionally delete `apps/ducth-dev-website/`.

No other service depends on the reader. The Supabase stack, the API, the admin, and Jaeger remain untouched.

## Open Questions

- (Resolved by Phase 1 decisions) Identity tuple: directory = `apps/ducth-dev-website/`, package = `ducth-dev-website`, Compose service + container = `ducth-dev-website`, Traefik router = `website`, Traefik service = `website-service`, Host = `ducth.dev`, image = `doitsu2014/ducth-dev-website` (future only, no CI deliverable in this change).
- (Resolved) Compose env vars cannot mutate a built bundle → server-side source of truth with SSR HTML injection (D2).
- (Resolved) Compose labels vs static dynamic config → static dynamic config owns Host rules; Compose labels own only middlewares (D6).
- (Deferred, flag for follow-up) Rate limit defaults — keep existing 100 req / 15 min from `server.prod.mjs:55-59`; document the env override once traffic warrants it.
- (Deferred) Adding a CI release-image workflow for `doitsu2014/ducth-dev-website` is explicitly out of scope per the user constraints; revisit when the reader is promoted to production deployment.
- (Deferred) Whether to publish a Kubernetes Helm chart for the reader — explicitly out of scope; revisit if k8s deployment becomes a requirement.
