# Deployment, local operations, and publishing

Local narrative and ports: `README.md` and
`deployments/docker-swarm/README.md`.

Validate: `deployments/docker-swarm/`, `apps/api/Dockerfile`,
`apps/web` and `apps/ducth-dev-website` Dockerfiles,
`.github/workflows/release-my-cms-image.yml`,
`.github/workflows/release-my-cms-admin-image.yml`,
`.github/workflows/release-my-cms-ducth-dev-website-image.yml`,
`.github/workflows/publish-github-release-images.yml`,
`apps/api/gateway/src/main.rs`.

## Local stacks

Three Compose stacks share the external `supabase_network`: Supabase, apps,
and Traefik, under `deployments/docker-swarm/`. Bring-up is `bootstrap.sh`,
then `supabase/reset.sh` and `apps/reset.sh`. Env files are per stack
(`supabase/.env`, `apps/.env`, `traefik/.env`). `POSTGRES_PASSWORD`,
`JWT_SECRET`, `ANON_KEY`, and `SERVICE_ROLE_KEY` match across Supabase and
apps. Database init is upstream Supabase SQL plus a small grants file.
`init-wait` gates app migration. The API uses the Docker-internal Supabase
URL; browsers use the public URL. Reset scripts are local-only and drop
local volumes.

Documented host ports include API `8989`, admin `3002`, reader `3001`,
Jaeger `16686`, Traefik `8080`. Supabase service ports are the ones in the
stack README, not a retired root compose file.

## Website image and health

The public site image is a three-stage Node 20 Alpine build, non-root
runtime, on `supabase_network`, routed by Traefik. Env is documented and
the service can roll back on its own.

`GET /healthz` is unauthenticated and returns 200 when Express is accepting
connections. It does not render React, read the SSR template, create
Apollo, call GraphQL, or fetch media. The image healthcheck is wget against
`http://localhost:3001/healthz` at interval 30s, timeout 10s, start period
5s, retries 3. `GET /en` stays a reader route and is not the probe. Missing
`WEBSITE_PUBLIC_GRAPHQL_API_URL` exits before the probe matters.

The published website host port binds to loopback so public traffic cannot
skip Traefik.

## Website traces

Export exists only when `ENABLED_OTLP_EXPORTER` is exactly `true`. Compose
maps `WEBSITE_ENABLED_OTLP_EXPORTER`, `WEBSITE_OTEL_SERVICE_NAME`,
`WEBSITE_OTEL_EXPORTER_OTLP_TRACES_ENDPOINT`, and
`WEBSITE_OTEL_TRACES_SAMPLER` onto the OTEL variables. Default service name
is `ducth-dev-website`. Local Jaeger is
`http://jaeger:4318/v1/traces`. Disabled, invalid, or failed export does
not change startup, SSR, or `/healthz`, and does not put telemetry config
in the browser bundle.

Non-health SSR requests create a server span, continue an incoming W3C
`traceparent`, and send `traceparent` on the Apollo call to the public
GraphQL URL without changing the GraphQL contract. `/healthz` creates no
span. Spans may record client address (first valid IP in Traefik-normalized
`x-forwarded-for`, else the TCP peer; ignore `x-real-ip`), raw User-Agent,
and bounded browser/OS/device attributes. They MUST NOT record bodies,
HTML, GraphQL documents or variables, cookies, authorization, telemetry
secrets, or query strings. Traefik trusts forwarded headers only from
Cloudflare CIDRs. No browser OpenTelemetry SDK.

Gateway OTLP is specified in `backend-domain-architecture.md`.

## Image build contexts

Primary contexts stay app-scoped: `apps/api`, `apps/web`,
`apps/ducth-dev-website`. The repo root is not a primary context. The API
context excludes `target/`. Both frontend builds receive
`packages/editor-prose` only as the named BuildKit context `editor-prose`,
without `node_modules` or secrets, and install with a frozen lockfile.
Compose and the admin release workflow pass that named context. Admin
release path filters include `packages/editor-prose/**`.

There is no `.github/workflows/validate-app-image-builds.yml` and no
pull-request image build. Do not add one. A local check of workflow files
asserts repositories, path filters, tag policy, and contexts without
logging into Docker Hub.

## Docker Hub

| Image | Repository | Main path filters |
| --- | --- | --- |
| API | `doitsu2014/my-cms` | `apps/api/**`, its workflow file |
| Admin | `doitsu2014/my-cms-admin` | `apps/web/**`, `packages/editor-prose/**`, its workflow file |
| Ducth | `doitsu2014/my-cms-ducth-dev-website` | `apps/ducth-dev-website/**`, `packages/editor-prose/**`, its workflow file |

Main publishers run on push to `main` for those inputs only, not on version
tags and not on pull requests. Success tags the image with the source SHA
and moves `latest` for that main build.

A published GitHub Release whose tag is exactly `vX.Y.Z` (numeric
components) builds all three images from that ref. Each gets the version
tag and the SHA tag. `latest` is not created or moved. If any repository
already has that version tag, the workflow fails before pushing. Concurrency
is per version with `cancel-in-progress: false`. A bad or partial release
is fixed by a new version, not by retagging.

After all three pushes succeed, release notes get one delimited manifest
(version, repository, tag, SHA, digest). A rerun replaces that block. A
partial failure records an incomplete block and fails the job. It never
claims a complete set. Permissions are contents read/write for checkout and
the note update only. Docker Hub credentials come from repository secrets
and are not printed. The workflow does not deploy.
