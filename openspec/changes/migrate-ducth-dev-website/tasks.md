
# migrate-ducth-dev-website — Implementation Plan

> Source of truth: `openspec/changes/migrate-ducth-dev-website/{proposal.md, design.md, specs/website-frontend/spec.md, specs/website-deployment/spec.md, specs/local-dev-environment/spec.md}`.
> This plan does **not** commit, push, or archive without explicit user approval. Each `- [ ]` task is independently verifiable. RED-GREEN-REFACTOR applies to every behavioral change.

## Execution Conventions

- **Path layout**: every new file lives under `apps/ducth-dev-website/` (mirror of `apps/web/`). Every deployment edit touches only `deployments/docker-swarm/apps/docker-compose.yaml`, `deployments/docker-swarm/apps/.env.example`, `deployments/docker-swarm/traefik/dynamic/my-cms.yml`, `deployments/docker-swarm/traefik/.env.example`, and `deployments/docker-swarm/README.md`. `apps/web/**` and `apps/api/**` are not modified.
- **Identity tokens** (must appear verbatim in created files):
  - pnpm `name` and Compose service + container: `ducth-dev-website`
  - Traefik router: `website`, Traefik service: `website-service`, middleware: `website-compress`
  - Traefik Host rule: `ducth.dev`
  - Env prefix: `WEBSITE_*` (with `CMS_WEBSITE_HOST` consumed from the traefik env file)
- **Graph gate** (from `AGENTS.md`): before each task group that touches source, run `get_minimal_context(task="migrate-ducth-dev-website")` on the `code-review-graph` MCP server. After each task group, run `detect_changes`, `get_affected_flows`, `tests_for` (for high-risk functions), and `get_impact_radius`. If the MCP server is unavailable, substitute `git diff --stat` against the previous group, then run `pnpm --dir apps/ducth-dev-website build` and `docker build -f apps/ducth-dev-website/Dockerfile apps/ducth-dev-website` as the equivalent verification. **Never fabricate graph results.**
- **Checkpoint**: at the end of each group, request a code review before continuing. Do not move to the next group with a failing review.
- **No commit**: tasks end at "report done to the user and await commit/PR approval". Repository policy requires an explicit user request before `git add`, `git commit`, `git push`, or any PR creation.

## 1. Scaffold the empty `apps/ducth-dev-website/` pnpm app

Establish the standalone pnpm app directory mirroring `apps/web/` exactly. No source migration yet; this group only proves the build pipeline works on an empty placeholder.

- [x] 1.1 Create the directory `apps/ducth-dev-website/` with subdirectories `src/`, `src/components/layout/`, `src/pages/`, `src/i18n/locales/`, `src/infrastructure/graphql/`, `src/config/`, `public/`.
- [x] 1.2 Create `apps/ducth-dev-website/package.json` with `"name": "ducth-dev-website"`, `"private": true`, `"type": "module"`, `"packageManager": "pnpm@10.33.0"` (matching `apps/web/package.json:6`), and the migrated runtime deps from `my-blogs-rsbuild/client_side/package.json:15-32` (`react@^19.1.1`, `react-dom@^19.1.1`, `@apollo/client@^3.14.0`, `react-router-dom@^7.11.0`, `react-i18next@^16.5.1`, `i18next@^25.7.4`, `daisyui@^5.5.14`, `tailwindcss@^4.1.18`, `@tailwindcss/postcss@^4.1.18`, `@tailwindcss/typography@^0.5.19`, `autoprefixer@^10.4.23`, `postcss@^8.5.6`, `express@^5.2.1`, `express-rate-limit@^8.2.1`, `highlight.js@^11.11.1`, `graphql@^16.12.0`). Dev deps from `client_side/package.json:33-48` plus `vitest@^4.1.8`, `@vitest/ui@^4.1.8`, `jsdom@^29.1.1`, `@testing-library/react@^16.3.2`, `@testing-library/jest-dom@^6.9.1`, `@testing-library/user-event@^14.6.1`, plus a `vitest` script entry.
- [x] 1.3 Create `apps/ducth-dev-website/pnpm-workspace.yaml` mirroring `apps/web/pnpm-workspace.yaml:1-2` (the `allowBuilds.core-js` quirk: `allowBuilds: core-js: set this to true or false`).
- [x] 1.4 Create `apps/ducth-dev-website/tsconfig.json` mirroring `my-blogs-rsbuild/client_side/tsconfig.json:1-25` and extend `include` to `["src", "src/**/*.test.ts", "src/**/*.test.tsx"]`. Also export vitest globals (`"types": ["vitest/globals"]`).
- [x] 1.5 Create `apps/ducth-dev-website/eslint.config.mjs` mirroring `my-blogs-rsbuild/client_side/eslint.config.mjs:1-23`.
- [x] 1.6 Create `apps/ducth-dev-website/.prettierrc` and `apps/ducth-dev-website/.prettierignore` mirroring `my-blogs-rsbuild/client_side/.prettierrc` and `my-blogs-rsbuild/client_side/.prettierignore`.
- [x] 1.7 Create `apps/ducth-dev-website/postcss.config.mjs` mirroring `my-blogs-rsbuild/client_side/postcss.config.mjs:1-9`.
- [x] 1.8 Create `apps/ducth-dev-website/tailwind.config.ts` mirroring `my-blogs-rsbuild/client_side/tailwind.config.ts:1-23`.
- [x] 1.9 Create `apps/ducth-dev-website/.env.example` documenting every `WEBSITE_*` variable with example values: `WEBSITE_SITE_NAME`, `WEBSITE_SITE_URL`, `WEBSITE_AVATAR_URL`, `WEBSITE_DEFAULT_TITLE`, `WEBSITE_DEFAULT_DESCRIPTION`, `WEBSITE_DEFAULT_LOCALE=en`, `WEBSITE_PUBLIC_GRAPHQL_API_URL`, `WEBSITE_PUBLIC_GRAPHQL_CACHE_API_URL`, `WEBSITE_PUBLIC_MEDIA_BASE_URL`, `WEBSITE_PORT=3001`.
- [x] 1.10 Create `apps/ducth-dev-website/.dockerignore` with: `node_modules`, `dist`, `.git`, `.gitignore`, `*.md`, `.env*`, `.vscode`, `.idea`, `*.log`, `npm-debug.log*`, `yarn-debug.log*`, `yarn-error.log*`, `.DS_Store`, `Thumbs.db`, `coverage`, `**/*.test.ts`, `**/*.test.tsx`, `vitest.config.ts`.
- [x] 1.11 Create `apps/ducth-dev-website/.gitignore` mirroring `my-blogs-rsbuild/client_side/.gitignore:1-16` plus `coverage/`.
- [x] 1.12 Create `apps/ducth-dev-website/vitest.config.ts` with `environment: "jsdom"`, `globals: true`, `setupFiles: ["./src/test/setup.ts"]`.
- [x] 1.13 Create `apps/ducth-dev-website/rsbuild.config.ts` with two environments — `web` (entry `src/index.client.tsx`, output `dist/client`) and `node` (entry `src/index.server.tsx`, output `dist/server/index.mjs`, `library.type: module`, `experiments.outputModule: true`) — mirroring `my-blogs-rsbuild/client_side/rsbuild.config.ts:1-62`.
- [x] 1.14 Create `apps/ducth-dev-website/index.html` with `<html lang="en" data-theme="emerald">`, `<div id="root"><!--app-content--></div>`, and an empty `<head>` (no favicon link — favicon will be served from media at runtime).
- [x] 1.15 Create placeholder source files: `src/index.client.tsx` exporting a no-op React component, `src/index.server.tsx` exporting `default async function render() { return { html: "<div id=\"root\"></div>", apolloState: null }; }`, `src/server.prod.mjs` (Express scaffold reading `process.env.PORT`), `src/App.tsx`, `src/AppContent.tsx`, `src/config/runtime-config.ts`, `src/test/setup.ts` (empty), `src/i18n/locales/en.json` and `src/i18n/locales/vi.json` as empty objects.
- [x] 1.16 Run `pnpm --dir apps/ducth-dev-website install --frozen-lockfile` and let `pnpm-lock.yaml` be written. Expected success: `node_modules/` populated, `pnpm-lock.yaml` written.
- [x] 1.17 Run `pnpm --dir apps/ducth-dev-website build`. Expected success: `apps/ducth-dev-website/dist/client/index.html` and `apps/ducth-dev-website/dist/server/index.mjs` exist. Run `node -e "import('./apps/ducth-dev-website/dist/server/index.mjs').then(m => console.log(typeof m.default))"` and assert the output is `function`.

▶ CHECKPOINT: request code review (`requesting-code-review` skill). Do not continue until the scaffold review passes.

## 2. TDD: runtime env validation (server side)

`apps/ducth-dev-website/src/config/validate-env.ts`. RED tests first; GREEN implementation next.

- [x] 2.1 Write failing tests in `apps/ducth-dev-website/src/config/validate-env.test.ts` covering: (a) missing required variable throws with message containing the variable name; (b) malformed URL throws with message containing variable name and first 64 chars of value; (c) valid URL returns the value trimmed; (d) optional `WEBSITE_PUBLIC_GRAPHQL_CACHE_API_URL` falls back to `WEBSITE_PUBLIC_GRAPHQL_API_URL`; (e) optional `WEBSITE_AVATAR_URL` returns `undefined` when unset; (f) optional `WEBSITE_PORT` falls back to `"3001"`; (g) optional `WEBSITE_DEFAULT_LOCALE` falls back to `"en"`. Run `pnpm --dir apps/ducth-dev-website test` and confirm all seven tests fail with `Cannot find module './validate-env'` or `parseRequiredEnv is not a function`.
- [x] 2.2 Implement `apps/ducth-dev-website/src/config/validate-env.ts` exporting `parseRequiredEnv(name: string, raw: string | undefined): string` (throws on missing; runs `new URL(raw)` on present values; throws with name + first 64 chars on invalid URL) and `resolveRuntimeConfig(env: NodeJS.ProcessEnv): RuntimeConfig`. Run `pnpm --dir apps/ducth-dev-website test` and confirm all seven tests pass.
- [x] 2.3 Add the matching scenario coverage from `specs/website-frontend/spec.md` requirement "Runtime configuration is owned by the server" by also testing the `Server` exit-message contract: include an assertion that the thrown error string includes the literal `WEBSITE_PUBLIC_GRAPHQL_API_URL is required` and that the malformed-URL message includes both the variable name and `not-a-url`.

▶ CHECKPOINT: review tests + implementation. The env validation is the security-critical contract; no implementation work may proceed past this group until the review passes.

## 3. TDD: safe JSON bootstrap escaping

`apps/ducth-dev-website/src/config/escape-json.ts`. The seven characters that break inline JSON in HTML: `<`, `>`, `&`, `'`, `"`, U+2028, U+2029.

- [x] 3.1 Write failing tests in `apps/ducth-dev-website/src/config/escape-json.test.ts` covering the spec-mandated escape set as defined in `specs/website-frontend/spec.md` requirement "SSR HTML injection is safe against inline-JSON pitfalls". Tests must assert: (a) `escapeJsonForScript('</script>')` round-trips through `JSON.parse` and never contains the literal substring `</script>`; (b) `escapeJsonForScript('a&b')` produces a JSON string whose parsed value is `'a&b'`; (c) `escapeJsonForScript('\u2028')` produces a JSON string whose parsed value is `'\u2028'`; (d) `escapeJsonForScript('<>&\u0027\u2028\u2029')` round-trips through `JSON.parse`; (e) the produced output for any input does not contain the literal `</script>` sequence. Run `pnpm test` and confirm five failing tests.
- [x] 3.2 Implement `apps/ducth-dev-website/src/config/escape-json.ts` exporting `escapeJsonForScript(value: unknown): string` that wraps `JSON.stringify` and post-processes the output to escape the seven characters per `specs/website-frontend/spec.md` (`<` → `\u003c`, `>` → `\u003e`, `&` → `\u0026`, `'` → `\u0027`, `"` handled by `JSON.stringify` natively, U+2028 → `\u2028`, U+2029 → `\u2029`). Run `pnpm --dir apps/ducth-dev-website test` and confirm all five tests pass.
- [x] 3.3 Add a regression test asserting the OWASP-mandated character set: write `escapeJsonForScript({ url: 'https://example.com/?a=1&b=2' })` and assert the output contains `\u0026` (the JSON-safe representation of `&`) and that `JSON.parse(escapeJsonForScript({ url: 'https://example.com/?a=1&b=2' })).url` equals the original URL.

## 4. TDD: media URL helper

`apps/ducth-dev-website/src/config/get-media-url.ts`.

- [x] 4.1 Write failing tests in `apps/ducth-dev-website/src/config/get-media-url.test.ts` covering the three scenarios from `specs/website-frontend/spec.md` requirement "Media URL helper joins exactly one slash": (a) `getMediaUrl('x.png', 'https://x.test/media')` returns `https://x.test/media/x.png`; (b) `getMediaUrl('/x.png', 'https://x.test/media/')` returns `https://x.test/media/x.png`; (c) `getMediaUrl('https://cdn.example.test/x.png', 'https://x.test/media')` returns `https://cdn.example.test/x.png` unchanged. Run `pnpm test` and confirm three failing tests.
- [x] 4.2 Implement `apps/ducth-dev-website/src/config/get-media-url.ts` exporting `getMediaUrl(path: string, mediaBaseUrl: string): string` that returns the path verbatim when it starts with `http://` or `https://`, otherwise joins `mediaBaseUrl.replace(/\/$/, '')` + `/` + `path.replace(/^\//, '')`. Confirm all three tests pass.

▶ CHECKPOINT: review the runtime-config helpers (env validation, JSON escape, media URL). These three modules are the foundation of every later group.

## 5. TDD: browser config reader

`apps/ducth-dev-website/src/config/read-browser-config.ts`. Reads `<script id="app-config" type="application/json">…</script>` text content and parses with `JSON.parse`.

- [x] 5.1 Write failing tests in `apps/ducth-dev-website/src/config/read-browser-config.test.ts` using jsdom that cover: (a) given a `document` with the script tag injected, `readBrowserConfig()` returns the parsed object; (b) when the script tag is absent, the function throws a clear error containing `app-config`; (c) when the script tag's text content is malformed JSON, the function throws the underlying `SyntaxError` (not a generic error). Run `pnpm test` and confirm three failing tests.
- [x] 5.2 Implement `apps/ducth-dev-website/src/config/read-browser-config.ts` exporting `readBrowserConfig(doc: Document = document): RuntimeConfig`. Confirm all three tests pass.

## 6. TDD: GraphQL client reads from window config

`apps/ducth-dev-website/src/infrastructure/graphql/graphql-client.ts`.

- [ ] 6.1 Write failing tests in `apps/ducth-dev-website/src/infrastructure/graphql/graphql-client.test.ts` that cover the scenario "GraphQL client uses injected URL" from `specs/website-frontend/spec.md`: mock `window.__APP_CONFIG__` with `graphqlApiUrl: "https://example.test/graphql"`, build the client, capture the HTTP link URI through an Apollo `ApolloLink` interceptor, assert the URI equals `"https://example.test/graphql"`. Run `pnpm test` and confirm the test fails (current client reads from `api.config.ts` hardcoded URL).
- [ ] 6.2 Refactor `apps/ducth-dev-website/src/infrastructure/graphql/graphql-client.ts` so `httpLink` reads from `window.__APP_CONFIG__.graphqlApiUrl` via `readBrowserConfig()` (lazy import to keep SSR bundle from importing browser-only code at top level). Confirm the test passes.
- [ ] 6.3 Add a static check: `rg "https://my-cms-api.ducth.dev|https://my-blogs.ducth.dev" apps/ducth-dev-website/src/` returns zero matches. If matches are found in `src/`, refactor until zero matches remain.

## 7. Migrate source verbatim from `my-blogs-rsbuild/client_side/`

Copy source files without functional changes. Hardcoded URLs are tolerated at this stage; the next group replaces them.

- [ ] 7.1 Run `get_minimal_context(task="migrate-ducth-dev-website")` on the `code-review-graph` MCP server. If unavailable, substitute `git status --porcelain` and `git diff --stat HEAD -- apps/ apps/api deployments` to confirm a clean working tree.
- [ ] 7.2 Copy verbatim from `../my-blogs-rsbuild/client_side/src/` into `apps/ducth-dev-website/src/`: `App.tsx`, `AppContent.tsx`, `i18n/i18n.ts`, `i18n/locales/en.json`, `i18n/locales/vi.json`, `components/layout/Footer.tsx`, `components/layout/Header.tsx`, `components/layout/Layout.tsx`, `pages/HomePage.tsx`, `pages/CategoriesPage.tsx`, `pages/CategoryDetailPage.tsx`, `pages/PostDetailPage.tsx`, `infrastructure/graphql/queries.ts`, `App.css`. Do **not** yet copy `config/api.config.ts`, `config/site.config.ts`, `index.html`, `index.client.tsx`, `index.server.tsx`, `env.d.ts`, or `server.prod.mjs` — those are replaced/rewritten in groups 8 and 9.
- [ ] 7.3 Run `pnpm --dir apps/ducth-dev-website typecheck` and `pnpm --dir apps/ducth-dev-website lint`. Expected success: zero errors. If imports reference `@/config/api.config` or `@/config/site.config`, stub those modules (returning empty objects) so the build can complete — they will be replaced in groups 8 and 9.
- [ ] 7.4 Run `pnpm --dir apps/ducth-dev-website build`. Expected success: both `dist/client/index.html` and `dist/server/index.mjs` produced.
- [ ] 7.5 Run `detect_changes` and `get_impact_radius` on the `code-review-graph` MCP server. If unavailable, substitute `git diff --stat` to confirm only `apps/ducth-dev-website/**` changed.

## 8. Wire SSR contract + replace hardcoded URLs

The migrated pages currently hardcode `https://my-cms-api.ducth.dev/media/{path}` and `https://my-blogs.ducth.dev`. Replace via the runtime-config helpers and the SSR script-injection contract.

- [ ] 8.1 Replace `apps/ducth-dev-website/index.html` so the `<head>` ends with `</head>` (no inline config). Update `apps/ducth-dev-website/src/index.client.tsx` to call `hydrateRoot` using `App` and to use `readBrowserConfig()` for any browser-side config access.
- [ ] 8.2 Replace `apps/ducth-dev-website/src/index.server.tsx` so its default export is `async function render(url: string): Promise<{ html: string; apolloState: object }>` that builds the app tree with `StaticRouter` + `ApolloProvider(buildGraphQLClient())` and runs `getDataFromTree(App)` then `renderToString(App)` then `client.extract()`. Use `resolveRuntimeConfig(process.env)` to obtain the GraphQL URL for the SSR-time Apollo client.
- [ ] 8.3 Replace `apps/ducth-dev-website/src/server.prod.mjs` (carried over from `my-blogs-rsbuild/client_side/server.prod.mjs:1-175`) so that: (a) `CONFIG` is populated from `resolveRuntimeConfig(process.env)` via a small CJS shim or via direct `process.env` reads that mirror the validated env contract; (b) the Express handler serves `GET /{*path}` and replaces `<!--app-content-->` with rendered HTML; (c) the handler injects the resolved config as `<script id="app-config" type="application/json">{escapeJsonForScript(runtimeConfig)}</script>` immediately before `</head>`; (d) `GET /static/*` continues to serve `dist/client/static`; (e) every `try`/`catch` logs only `method`, `path`, and a generated UUID correlation id (e.g. `crypto.randomUUID()`) and returns a generic HTML 500 page; (f) missing required env at startup throws with the named variable and the process exits non-zero (no `listen()` happens).
- [ ] 8.4 Replace `apps/ducth-dev-website/src/config/api.config.ts` so it re-exports `getMediaUrl`, `getGraphqlApiUrl`, and the legacy `API_CONFIG` shape by reading from `window.__APP_CONFIG__` via `readBrowserConfig()`. Delete any literal `https://my-cms-api.ducth.dev` strings.
- [ ] 8.5 Replace `apps/ducth-dev-website/src/config/site.config.ts` so it re-exports `SITE_CONFIG` from `window.__APP_CONFIG__`. Delete any literal `https://my-blogs.ducth.dev` strings.
- [ ] 8.6 In `src/pages/HomePage.tsx`, `src/pages/CategoriesPage.tsx`, `src/pages/CategoryDetailPage.tsx`, `src/pages/PostDetailPage.tsx`, replace every `https://my-cms-api.ducth.dev/media/{path}` literal with `getMediaUrl(path, getMediaBaseUrl())`.
- [ ] 8.7 Run `rg "https://my-cms-api.ducth.dev|https://my-blogs.ducth.dev" apps/ducth-dev-website/src/` and confirm zero matches.
- [ ] 8.8 Run `pnpm --dir apps/ducth-dev-website typecheck && pnpm lint && pnpm test && pnpm build`. Expected: zero TypeScript errors, zero lint errors, all vitest scenarios pass, both bundles produced.
- [ ] 8.9 Run `tests_for` and `get_affected_flows` on the `code-review-graph` MCP server for the modified files (`src/server.prod.mjs`, `src/index.server.tsx`, `src/index.client.tsx`, `src/pages/*`). If unavailable, substitute `pnpm test -- --reporter=verbose` to confirm full coverage.

▶ CHECKPOINT: review the SSR contract and URL replacement. This is the highest-blast-radius group (security-relevant: JSON injection + error sanitization).

## 9. Live Seaography query-shape verification

The client uses `parentId: { is_null: "true" }` (string-typed). Verify the GraphQL schema accepts the literal string form before declaring group 8 complete.

- [ ] 9.1 Start the local stack: `cd deployments/docker-swarm && ./apps/reset.sh` (or the equivalent documented bring-up). Confirm `my-cms-api` is `running`.
- [ ] 9.2 From a host terminal, run: `curl -i http://localhost:8989/graphql/immutable -H 'content-type: application/json' -d '{"query":"{ categories(filters:{categoryType:{eq:Blog},parentId:{is_null:\"true\"}}){ nodes { displayName slug } } }"}'`. Expected: HTTP 200 with `data.categories.nodes` as an array (possibly empty if not seeded).
- [ ] 9.3 If the response is HTTP 400 with an error mentioning `is_null`, change `apps/ducth-dev-website/src/infrastructure/graphql/queries.ts` so `parentId: { is_null: "true" }` becomes `parentId: { is_null: true }` (boolean). Re-run 9.2. Document the chosen form (string vs boolean) in the change's `openspec/changes/migrate-ducth-dev-website/decisions.md` or inline in `queries.ts`.
- [ ] 9.4 Re-run `pnpm --dir apps/ducth-dev-website build` after any query shape change and confirm both bundles rebuild.

## 10. SSR + hydration smoke

- [ ] 10.1 Run `pnpm --dir apps/ducth-dev-website build`. Confirm `dist/client/index.html` and `dist/server/index.mjs` exist.
- [ ] 10.2 Run `node -e "import('./apps/ducth-dev-website/dist/server/index.mjs').then(m => console.log(typeof m.default, m.default.constructor.name))"` and assert the output is `function async` (the default export is an async function).
- [ ] 10.3 Run `node -e "import('./apps/ducth-dev-website/dist/server/index.mjs').then(async m => { const r = await m.default('/en'); console.log(typeof r.html === 'string' && r.html.length > 0); })"` and assert the output is `true` when a temporary `WEBSITE_PUBLIC_GRAPHQL_API_URL=http://localhost:8989/graphql WEBSITE_PUBLIC_MEDIA_BASE_URL=http://localhost:8989/media WEBSITE_SITE_URL=http://localhost:3001 WEBSITE_SITE_NAME=test WEBSITE_DEFAULT_TITLE=t WEBSITE_DEFAULT_DESCRIPTION=d` env is supplied and the Supabase stack is up.

## 11. Three-stage Node 20 Alpine Dockerfile

`apps/ducth-dev-website/Dockerfile`. No nginx.

- [ ] 11.1 Create `apps/ducth-dev-website/Dockerfile` with first line `# syntax=docker.io/docker/dockerfile:1`, then three stages: `deps` (`FROM node:20-alpine`, `RUN apk add --no-cache libc6-compat`, `WORKDIR /app`, `COPY package.json pnpm-lock.yaml pnpm-workspace.yaml ./`, `RUN corepack enable pnpm && pnpm install --frozen-lockfile`); `builder` (`FROM node:20-alpine`, `WORKDIR /app`, `COPY --from=deps /app/node_modules ./node_modules`, `COPY . .`, `RUN corepack enable pnpm && pnpm run build`); `runner` (`FROM node:20-alpine`, `WORKDIR /app`, `RUN addgroup --system --gid 1001 nodejs && adduser --system --uid 1001 expressjs`, `COPY --from=builder /app/dist ./dist`, `COPY --from=builder /app/server.prod.mjs ./server.prod.mjs`, `COPY --from=builder /app/package.json ./package.json`, `COPY --from=deps /app/node_modules ./node_modules`, `RUN chown -R expressjs:nodejs /app`, `USER expressjs`, `ENV NODE_ENV=production`, `ENV PORT=3001`, `EXPOSE 3001`, `HEALTHCHECK --interval=30s --timeout=10s --start-period=5s --retries=3 CMD wget --no-verbose --tries=1 --spider http://localhost:3001/en || exit 1`, `CMD ["node", "server.prod.mjs"]`).
- [ ] 11.2 Run `docker build -f apps/ducth-dev-website/Dockerfile -t ducth-dev-website:dev apps/ducth-dev-website`. Expected: build completes with exit zero.
- [ ] 11.3 Run `docker run --rm ducth-dev-website:dev sh -c 'id && ls /app/dist/client | head -5 && ls /app/dist/server && cat /app/package.json | head -5'` and assert: `uid=1001(expressjs) gid=1001(nodejs)`; `dist/client` listed; `index.mjs` listed under `dist/server`; package `name` is `ducth-dev-website`.
- [ ] 11.4 Run a healthy container: `docker run --rm -d --name ducth-dev-website-smoke -p 3001:3001 -e WEBSITE_PUBLIC_GRAPHQL_API_URL=http://host.docker.internal:8989/graphql -e WEBSITE_PUBLIC_MEDIA_BASE_URL=http://host.docker.internal:8989/media -e WEBSITE_SITE_URL=http://localhost:3001 -e WEBSITE_SITE_NAME=Test -e WEBSITE_DEFAULT_TITLE=T -e WEBSITE_DEFAULT_DESCRIPTION=D ducth-dev-website:dev` (only when the local stack is up). Wait for healthcheck. Run `curl -i http://localhost:3001/en` and assert HTTP 200 with body containing `<!--app-content-->` replaced (no literal marker).
- [ ] 11.5 Run a misconfigured container: `docker run --rm ducth-dev-website:dev` (no env). Expected: container exits with non-zero status within 5 seconds; last log line contains `WEBSITE_PUBLIC_GRAPHQL_API_URL`.
- [ ] 11.6 `docker rm -f ducth-dev-website-smoke` (cleanup if it was started).

## 12. Docker Compose service on `supabase_network`

`deployments/docker-swarm/apps/docker-compose.yaml`. Touch only the `my-cms-frontend` block area; do **not** reorder existing services.

- [ ] 12.1 Add a new `ducth-dev-website` service to `deployments/docker-swarm/apps/docker-compose.yaml` after the `my-cms-frontend:` block (line ~168). Body: `container_name: ducth-dev-website`; `build: { context: ../../../apps/ducth-dev-website, dockerfile: Dockerfile }`; `restart: unless-stopped`; `depends_on: { my-cms-api: { condition: service_started } }`; `networks: [supabase_network]`; `environment: { NODE_ENV: production, PORT: 3001, SITE_NAME: ${WEBSITE_SITE_NAME}, SITE_URL: ${WEBSITE_SITE_URL}, AVATAR_URL: ${WEBSITE_AVATAR_URL}, DEFAULT_TITLE: ${WEBSITE_DEFAULT_TITLE}, DEFAULT_DESCRIPTION: ${WEBSITE_DEFAULT_DESCRIPTION}, PUBLIC_GRAPHQL_API_URL: ${WEBSITE_PUBLIC_GRAPHQL_API_URL}, PUBLIC_GRAPHQL_CACHE_API_URL: ${WEBSITE_PUBLIC_GRAPHQL_CACHE_API_URL}, PUBLIC_MEDIA_BASE_URL: ${WEBSITE_PUBLIC_MEDIA_BASE_URL} }`; `labels: [ "traefik.enable=true", "traefik.http.routers.website.middlewares=website-compress" ]` (no Host-rule label, no entrypoint label, no loadbalancer-server-port label — those live in the static dynamic config).
- [ ] 12.2 Append a `WEBSITE_*` block to `deployments/docker-swarm/apps/.env.example` with header comment `# ---------- Website (ducth-dev-website) ----------` and entries: `WEBSITE_SITE_NAME`, `WEBSITE_SITE_URL`, `WEBSITE_AVATAR_URL`, `WEBSITE_DEFAULT_TITLE`, `WEBSITE_DEFAULT_DESCRIPTION`, `WEBSITE_PUBLIC_GRAPHQL_API_URL`, `WEBSITE_PUBLIC_GRAPHQL_CACHE_API_URL`, `WEBSITE_PUBLIC_MEDIA_BASE_URL`.
- [ ] 12.3 Run `docker compose -f deployments/docker-swarm/apps/docker-compose.yaml --env-file deployments/docker-swarm/apps/.env.example config` (with example values substituted into the `WEBSITE_*` lines) and assert the command exits 0. If `docker compose config` fails on the example file, fix the YAML.

## 13. Traefik static dynamic config + env example

`deployments/docker-swarm/traefik/dynamic/my-cms.yml`. This file is the **single source of routing truth** for the `website` router.

- [ ] 13.1 Edit `deployments/docker-swarm/traefik/dynamic/my-cms.yml` to: (a) add `website:` router under `http.routers` with `rule: "Host(\`ducth.dev\`)"`, `entryPoints: [web]`, `service: website-service`, `middlewares: [website-compress, security-headers]` (mirror the `admin` router's middleware list at lines 21-23 to preserve the security posture); (b) add `website-service:` under `http.services` with `loadBalancer.servers: [{ url: "http://ducth-dev-website:3001" }]`; (c) add `website-compress: { compress: {} }` under `http.middlewares`; (d) append `https://ducth.dev` to `api-cors.accessControlAllowOriginList` and to `cors-everything.accessControlAllowOriginList` at lines 90-92 and 125-127.
- [ ] 13.2 Append `CMS_WEBSITE_HOST=ducth.dev` to `deployments/docker-swarm/traefik/.env.example` under a `# ---------- Website ----------` header.
- [ ] 13.3 Run `docker compose -f deployments/docker-swarm/traefik/docker-compose.yaml --env-file deployments/docker-swarm/traefik/.env.example up -d` and assert the traefik container starts with no YAML parse errors. Check the traefik log for `level=error` lines — expected zero new ones.

## 14. Compose smoke gate

Bring up the full stack and verify the reader is reachable through Traefik.

- [ ] 14.1 `cd deployments/docker-swarm && ./supabase/reset.sh --restart` (idempotent) followed by `./apps/reset.sh` (full bring-up including the new website service).
- [ ] 14.2 `docker compose -f deployments/docker-swarm/apps/docker-compose.yaml ps` and assert `ducth-dev-website` is in `running` or `healthy` state.
- [ ] 14.3 `docker exec ducth-dev-website wget -q -O - http://my-cms-api:8989/health` and assert the response body contains `CMS is running successfully`.
- [ ] 14.4 `docker exec ducth-dev-website sh -c 'wget -q -O - http://localhost:3001/en | head -c 500'` and assert the response starts with `<!DOCTYPE html>` (SSR-rendered HTML, not the static template).
- [ ] 14.5 From the host: `curl -i -H "Host: ducth.dev" http://localhost/` and assert: HTTP 200; `Content-Encoding: gzip` header; body contains `<!--app-content-->` replaced.
- [ ] 14.6 From the host: `curl -i -H "Host: cms-admin.ducth.dev" http://localhost/` and assert: HTTP 200; body is the admin SPA (unchanged behavior — confirms no Traefik router collision).
- [ ] 14.7 Run the public GraphQL smoke gate: `curl -i http://localhost:8989/graphql/immutable -H 'content-type: application/json' -d '{"query":"{ categories(filters:{categoryType:{eq:Blog}}){ nodes { displayName slug } } }"}'`. Expected: HTTP 200; response JSON has `data.categories.nodes` as an array.
- [ ] 14.8 Run CORS preflight: `curl -i -X OPTIONS -H "Origin: https://ducth.dev" -H "Access-Control-Request-Method: POST" http://localhost/graphql/immutable`. Expected: response includes `Access-Control-Allow-Origin: https://ducth.dev`.
- [ ] 14.9 Run `get_affected_flows` and `get_impact_radius` on the `code-review-graph` MCP server for the changed deployment files. If unavailable, substitute `git diff --stat` and the eight curl/docker assertions above.

▶ CHECKPOINT: review the deployment surface. After this checkpoint the reader is live in the local stack.

## 15. Documentation update

- [ ] 15.1 Edit `deployments/docker-swarm/README.md` "Access points" table (lines 67-73) to add a row `https://ducth.dev` → `Public reader (via Traefik Host rule)`. Edit the "Daily workflow" section (lines 49-63) to add a note: "the `ducth-dev-website` container joins the same stack once `my-cms-api` is healthy."
- [ ] 15.2 Verify the final `deployments/docker-swarm/README.md` renders without broken Markdown by reading it back and confirming the table and code blocks still parse.

## 16. Rollback dry-run (independent rollback contract)

Per `specs/website-deployment/spec.md` requirement "Independent rollback".

- [ ] 16.1 Take a snapshot of the changed files: `git status --porcelain > /tmp/pre-rollback.txt` (do not commit — this is just a working-tree diff for verification).
- [ ] 16.2 Temporarily undo the website edits: remove the `ducth-dev-website` block from `docker-compose.yaml`; remove the `website` router, `website-service`, `website-compress` middleware, and `https://ducth.dev` CORS entries from the dynamic config; remove `CMS_WEBSITE_HOST` from traefik `.env.example`; remove the `WEBSITE_*` block from apps `.env.example`. **Do not** remove `apps/ducth-dev-website/` or modify `apps/web/**` or `apps/api/**`.
- [ ] 16.3 `docker compose -f deployments/docker-swarm/apps/docker-compose.yaml --env-file deployments/docker-swarm/apps/.env.example config` — must exit 0. `docker compose -f deployments/docker-swarm/apps/docker-compose.yaml up -d` (without the website service) must bring up `my-cms-api`, `my-cms-frontend`, `migrate`, `init-wait`, `jaeger` unchanged.
- [ ] 16.4 `curl -i http://localhost:8989/health` → 200; `curl -i http://localhost:3002/` → 200 (admin still works).
- [ ] 16.5 Restore the edits from `/tmp/pre-rollback.txt` (re-apply the group 12-13 changes).
- [ ] 16.6 Run `docker compose -f deployments/docker-swarm/apps/docker-compose.yaml up -d` and re-verify group 14.

## 17. Repository-wide verification gate

- [ ] 17.1 `cargo check --manifest-path apps/api/Cargo.toml` — must exit 0.
- [ ] 17.2 `cargo test --manifest-path apps/api/Cargo.toml --no-fail-fast` — must exit 0.
- [ ] 17.3 `cargo fmt --manifest-path apps/api/Cargo.toml -- --check` — must exit 0.
- [ ] 17.4 `cargo clippy --manifest-path apps/api/Cargo.toml --all-targets -- -D warnings` — must exit 0.
- [ ] 17.5 `pnpm --dir apps/web build` — must exit 0.
- [ ] 17.6 `pnpm --dir apps/ducth-dev-website install --frozen-lockfile && pnpm --dir apps/ducth-dev-website lint && pnpm --dir apps/ducth-dev-website typecheck && pnpm --dir apps/ducth-dev-website test && pnpm --dir apps/ducth-dev-website build` — all five subcommands must exit 0.
- [ ] 17.7 Run `rg "https://my-cms-api.ducth.dev|https://my-blogs.ducth.dev" apps/ducth-dev-website/src/` — must return zero matches.
- [ ] 17.8 Re-run group 14 (Compose smoke) end-to-end.

▶ FINAL CHECKPOINT: report done to the user with the verification log. **Do not commit.** Wait for explicit user instruction.

## 18. OpenSpec handoff (unchecked — user approval required)

The following items remain unchecked and require an explicit user instruction before any of them run. The implementation engineer SHALL NOT execute them autonomously.

- [ ] 18.1 `openspec verify --change "migrate-ducth-dev-website"` — runs only after the user reviews the verification log and approves.
- [ ] 18.2 `openspec sync --change "migrate-ducth-dev-website"` — runs only after `openspec verify` returns clean and the user approves. Merges the three delta specs into `openspec/specs/{website-frontend,website-deployment,local-dev-environment}/spec.md`.
- [ ] 18.3 `openspec archive "migrate-ducth-dev-website"` — runs only after sync succeeds and the user approves. Moves the change folder under `openspec/changes/archive/YYYY-MM-DD-migrate-ducth-dev-website/`.
- [ ] 18.4 Hand off to the user for `git add`, `git commit`, `git push`, and PR creation. Repository policy forbids any of these without explicit user instruction.

## Out of Scope (explicit non-goals)

The following items MUST NOT be implemented as part of this change. They are recorded as non-goals in `design.md` and any attempt to add them is a scope violation that blocks the PR.

- Kubernetes / Helm chart for the reader. The existing `deployments/k8s/charts/my-cms-admin/` and the `deployments/charts/my-blogs/` chart are not modified.
- CI image publication workflow for `doitsu2014/ducth-dev-website`. The image name is reserved as a future deployment convention only; no `.github/workflows/release-ducth-dev-website-image.yml` is created.
- Visual redesign, copy changes, route renames, or i18n locale additions.
- Replacement or merge of the admin app at `apps/web/`.
- Backend feature work: no GraphQL schema edits, no my-cms-api route additions, no Supabase migrations.
- Deletion of the `my-blogs-rsbuild` repository. It remains the source of truth for the reader until the user confirms migration is verified end-to-end.
- Traefik dashboard, basic-auth, or Kong configuration changes.
