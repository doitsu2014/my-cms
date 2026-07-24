# Hotfix: ducth-dev-website launch (2026-07-24)

## Context
Pulled `main` to `468971b` and deployed the new `apps/ducth-dev-website/`
service (pnpm + Rsbuild + React 19 SSR + Express 5 + Apollo GraphQL +
Tailwind 4 + i18next). Two issues surfaced during verification.

## Issue 1 — RESOLVED ✅
**Traefik dynamic config YAML bugs**
File: `deployments/docker-swarm/traefik/dynamic/my-cms.yml`

Two bugs in the committed file:

- `api-cors` middleware block had misaligned indentation (parent at 6 spaces,
  children at inconsistent 8–10 spaces), which crashed YAML parsing on
  file-watcher reload.
- The four `rule:` lines (admin, api, studio, supabase-api) and several
  `accessControlAllowOriginList` entries used double-quoted YAML with `\"`
  escapes for inner Go-template quotes (e.g. `{{ env "X" | default "Y" }}`).
  Traefik's file-provider parser didn't strip the backslashes cleanly,
  raising `template: :17: unexpected "\\" in operand` and rejecting the file.

**Fixed in commit `81b4da4`** on this branch: rewrote the file with consistent
2-space indentation and switched the Go-template-bearing strings to YAML
single-quoted form (literal — no escape processing required). All 5 file
routers now load (`admin`, `api`, `studio`, `supabase-api`, `website`).
Regression-tested `cms-admin.ducth.dev` and `cms-api.ducth.dev/healthz` → 200.

## Issue 2 — OPEN ❌ (to resolve locally)
**SSR `ReferenceError: document is not defined`**

File: `apps/ducth-dev-website/src/config/read-browser-config.ts:19`

```ts
export function readBrowserConfig(doc: Document = document): RuntimeConfig {
```

The default param `doc = document` is evaluated **each call**. Node has no
`document` global, so the very first component that touches any of the
five `SITE_CONFIG` getters crashes SSR with `ReferenceError`.

### Stack trace
Captured by exec'ing `render('/en')` inside the running container:

```
ReferenceError: document is not defined
    at ak (file:///app/dist/server/index.mjs:35:199034)
    at get avatarUrl [as avatarUrl] (file:///app/dist/server/index.mjs:35:251319)
    at iG (file:///app/dist/server/index.mjs:35:252243)
    at rt (file:///app/dist/server/index.mjs:2:43825)
    at rn (file:///app/dist/server/index.mjs:2:45626)
    at ro (file:///app/dist/server/index.mjs:2:66046)
    at ri (file:///app/dist/server/index.mjs:2:63522)
    at r_ (file:///app/dist/server/index.mjs:2:70930)
    at rs (file:///app/dist/server/index.mjs:2:68079)
    at ro (file:///app/dist/server/index.mjs:2:66324)
```

### Affected getters
All five getters in `apps/ducth-dev-website/src/config/site.config.ts`:

- `siteName`
- `siteUrl`
- `avatarUrl`
- `seo.defaultTitle`
- `seo.defaultDescription`

Each calls `readBrowserConfig()` with no args → triggers default-param
evaluation of `document` → ReferenceError → 500.

### Reproduce
```bash
# direct via host port
curl -i http://localhost:3001/en

# via Traefik
curl -i -H 'Host: ducth.dev' http://localhost/en
```

Both return `HTTP/1.1 500 Internal Server Error`. Container healthcheck
failing streak 35+, status `unhealthy`.

### Suggested fix — minimal
Guard each getter in `src/config/site.config.ts` with a `typeof document`
check so the default param never evaluates on the server:

```ts
// src/config/site.config.ts
import { readBrowserConfig } from './read-browser-config';

const isBrowser = typeof document !== 'undefined';

export const SITE_CONFIG = {
  get siteName() {
    return isBrowser ? readBrowserConfig().siteName : '';
  },
  get siteUrl() {
    return isBrowser ? readBrowserConfig().siteUrl : '';
  },
  get avatarUrl() {
    return isBrowser ? readBrowserConfig().avatarUrl : undefined;
  },
  socialLinks: {
    github:  'https://github.com',
    twitter: 'https://twitter.com',
    linkedin: 'https://linkedin.com',
  },
  seo: {
    get defaultTitle() {
      return isBrowser ? readBrowserConfig().defaultTitle : '';
    },
    get defaultDescription() {
      return isBrowser ? readBrowserConfig().defaultDescription : '';
    },
  },
};
```

The SSR HTML shell will render with empty/`undefined` placeholders and the
client will hydrate with real values from `<script id="app-config">` once
the JS bundle runs. The consuming components (`Header.tsx`, `HomePage.tsx`)
already tolerate `undefined`/`''` for these fields.

### Suggested fix — cleaner, longer-term
Inject the server's `CONFIG` (built from env in `server.prod.mjs`) into
the React tree via a context provider, so SSR uses the server-side config
and the client uses `readBrowserConfig()`. Then SSR HTML shows real
title/avatar/etc. instead of empty placeholders. Bigger refactor.

### Rebuild + redeploy after fix
```bash
docker compose --env-file .env build ducth-dev-website
docker compose --env-file .env up -d ducth-dev-website
docker inspect ducth-dev-website --format '{{.State.Health.Status}}'  # expect: healthy
curl -i http://localhost:3001/en                                       # expect: 200
curl -i -H 'Host: ducth.dev' http://localhost/en                      # expect: 200
```

## Deploy artifacts applied but not committed
- `deployments/docker-swarm/apps/.env` — appended `WEBSITE_*` block:
  - `WEBSITE_SITE_NAME=Duc Tran's Blog`
  - `WEBSITE_SITE_URL=https://ducth.dev`
  - `WEBSITE_AVATAR_URL=https://cms-api.ducth.dev/media/avatar.png`
  - `WEBSITE_DEFAULT_TITLE=Duc Tran's Blog`
  - `WEBSITE_DEFAULT_DESCRIPTION=Personal development blog`
  - `WEBSITE_PUBLIC_GRAPHQL_API_URL=https://cms-api.ducth.dev/graphql/immutable`
  - `WEBSITE_PUBLIC_GRAPHQL_CACHE_API_URL=https://cms-api.ducth.dev/graphql/cache`
  - `WEBSITE_PUBLIC_MEDIA_BASE_URL=https://cms-api.ducth.dev/media`
  (File is gitignored per repo convention — won't show in `git status`.)
- `deployments/docker-swarm/apps/docker-compose.yaml` — added
  `ports: - "3001:3001"` to the `ducth-dev-website` service for direct
  host debugging. Uncommitted on this branch — keep or drop per your call.

## Branch state
- Branch: `hotfix/ducth-dev-website-launch-2026-07-24` (off `main` at `468971b`)
- Commits on this branch:
  - `81b4da4` fix(traefik): correct YAML indentation and Go-template escaping
  - `0f348c4` docs: hotfix notes for ducth-dev-website launch
- Working tree (uncommitted): `docker-compose.yaml` debug port mapping only.
- api/web: untouched (no code changes since `093edd5`).

Duc to resolve Issue 2 locally (the SSR source patch), then rebuild +
redeploy as shown above.