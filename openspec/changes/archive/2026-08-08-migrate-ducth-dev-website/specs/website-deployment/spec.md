# website-deployment Specification (delta)

## Purpose
Production containerization and stack wiring for the `ducth-dev-website` reader. Adds a three-stage Node 20 Alpine Dockerfile, a Docker Compose service on the shared `supabase_network`, and a Traefik Host-based router. Reconciles Compose labels with the static Traefik dynamic configuration so the static dynamic config remains the single source of routing truth.

## ADDED Requirements

### Requirement: Three-stage Node 20 Alpine Dockerfile

The repository SHALL include `apps/ducth-dev-website/Dockerfile` with three stages:
- `deps` — `FROM node:20-alpine`, runs `corepack enable pnpm && pnpm install --frozen-lockfile` to produce `node_modules`.
- `builder` — `FROM node:20-alpine`, copies `node_modules` from `deps`, copies the application source, runs `pnpm run build` to produce `dist/client/*` and `dist/server/index.mjs`.
- `runner` — `FROM node:20-alpine`, creates non-root user `expressjs` with uid 1001 and group `nodejs` with gid 1001, copies `dist`, `server.prod.mjs`, `package.json`, and production `node_modules` from `deps`, sets `ENV NODE_ENV=production`, `ENV PORT=3001`, `EXPOSE 3001`, `USER expressjs`, and `CMD ["node", "server.prod.mjs"]`.

The file SHALL use `# syntax=docker.io/docker/dockerfile:1` as its first line.

#### Scenario: Build produces a runnable image
- **WHEN** `docker build -f apps/ducth-dev-website/Dockerfile apps/ducth-dev-website` completes
- **THEN** the resulting image has a `node:20-alpine`-based final layer with `USER expressjs`
- **AND** `docker run --rm <image> node -e "console.log(require('./package.json').name)"` prints `ducth-dev-website`

#### Scenario: Build fails fast on missing lockfile
- **WHEN** `pnpm-lock.yaml` is absent from the build context
- **THEN** the `deps` stage fails with a non-zero exit code
- **AND** the failure message names `pnpm-lock.yaml`

### Requirement: Healthcheck endpoint

The runner stage SHALL declare `HEALTHCHECK --interval=30s --timeout=10s --start-period=5s --retries=3 CMD wget --no-verbose --tries=1 --spider http://localhost:3001/en || exit 1`. The `/en` path SHALL return HTTP 200 from a healthy Express process because the SSR handler short-circuits the locale-prefixed fallback to `/en`.

#### Scenario: Healthy container reports healthy
- **WHEN** the container is started with valid `WEBSITE_*` env vars
- **THEN** `docker inspect --format '{{.State.Health.Status}}' <container>` returns `healthy` within the healthcheck interval

#### Scenario: Misconfigured container reports unhealthy
- **WHEN** the container is started without `WEBSITE_PUBLIC_GRAPHQL_API_URL`
- **THEN** the container exits before the healthcheck runs
- **AND** `docker ps` does not list the container in `Up` state

### Requirement: Compose service contract

`deployments/docker-swarm/apps/docker-compose.yaml` SHALL define a `ducth-dev-website` service that:
- builds from `apps/ducth-dev-website` using its `Dockerfile`,
- sets `container_name: ducth-dev-website`,
- sets `restart: unless-stopped`,
- does NOT map port 3001 to the host (the service is reachable only via Traefik labels),
- declares `depends_on: my-cms-api { condition: service_started }`,
- joins the external `supabase_network`,
- consumes the env contract documented in `deployments/docker-swarm/apps/.env.example` (`WEBSITE_SITE_NAME`, `WEBSITE_SITE_URL`, `WEBSITE_AVATAR_URL`, `WEBSITE_DEFAULT_TITLE`, `WEBSITE_DEFAULT_DESCRIPTION`, `WEBSITE_PUBLIC_GRAPHQL_API_URL`, `WEBSITE_PUBLIC_GRAPHQL_CACHE_API_URL`, `WEBSITE_PUBLIC_MEDIA_BASE_URL`, plus `NODE_ENV=production` and `PORT=3001`).

#### Scenario: Service joins the shared network
- **WHEN** `docker compose -f deployments/docker-swarm/apps/docker-compose.yaml --env-file deployments/docker-swarm/apps/.env up -d` runs after the Supabase stack and `supabase_network` exist
- **THEN** `docker network inspect supabase_network --format '{{range .Containers}}{{.Name}} {{end}}'` includes `ducth-dev-website`
- **AND** the container can resolve `my-cms-api` to its container IP via DNS on that network

#### Scenario: Service waits for API readiness
- **WHEN** `my-cms-api` has not yet started
- **THEN** `ducth-dev-website` is not started
- **AND** the Compose log shows `ducth-dev-website` waiting on `my-cms-api`

#### Scenario: Service does not bind a host port
- **WHEN** `docker compose ps` is run with the apps compose up
- **THEN** the `ducth-dev-website` row does not include a host port mapping
- **AND** `curl http://localhost:3001/en` from the host machine does not connect

### Requirement: Traefik router and service in the dynamic config

`deployments/docker-swarm/traefik/dynamic/my-cms.yml` SHALL define:
- `routers.website` with `rule: "Host(\`ducth.dev\`)"`, `entryPoints: [web]`, `service: website-service`, `middlewares: [website-compress]`.
- `services.website-service.loadBalancer.servers: [{ url: "http://ducth-dev-website:3001" }]`.
- `middlewares.website-compress: { compress: {} }`.
- An extension of `api-cors.accessControlAllowOriginList` that adds `https://ducth.dev`.

The static dynamic config SHALL be the single source of truth for the Host rule, the entrypoint, and the load-balancer URL. The Compose service SHALL add only the `traefik.http.routers.website.middlewares=website-compress` label.

#### Scenario: Host-based routing reaches the website
- **WHEN** `curl -i -H "Host: ducth.dev" http://localhost/`
- **THEN** Traefik returns the response generated by `ducth-dev-website:3001`
- **AND** the response includes a `Content-Encoding: gzip` header

#### Scenario: Other Host rules are unaffected
- **WHEN** `curl -i -H "Host: cms-admin.ducth.dev" http://localhost/` is sent
- **THEN** Traefik still routes to `my-cms-frontend:80` (the existing admin router)

#### Scenario: CORS preflight allows the website origin
- **WHEN** an OPTIONS preflight is sent to `http://localhost/graphql/immutable` with `Origin: https://ducth.dev` and `Access-Control-Request-Method: POST`
- **THEN** the response includes `Access-Control-Allow-Origin: https://ducth.dev`
- **AND** the response includes `Access-Control-Allow-Methods: …, POST, …`

### Requirement: Env contract documentation

`deployments/docker-swarm/apps/.env.example` SHALL document every `WEBSITE_*` variable consumed by the `ducth-dev-website` service, with a short comment describing the purpose and an example value. `deployments/docker-swarm/traefik/.env.example` SHALL document `CMS_WEBSITE_HOST=ducth.dev`. `apps/ducth-dev-website/.env.example` SHALL document the same `WEBSITE_*` variables for local development outside Docker.

#### Scenario: Local developer can run the reader without Docker
- **WHEN** a developer copies `apps/ducth-dev-website/.env.example` to `apps/ducth-dev-website/.env`, fills the values, and runs `pnpm --dir apps/ducth-dev-website dev`
- **THEN** the dev server binds port 3001 and serves `http://localhost:3001/en`

#### Scenario: Compose env example lists every consumed variable
- **WHEN** a developer reads `deployments/docker-swarm/apps/.env.example`
- **THEN** every `WEBSITE_*` variable consumed by `docker-compose.yaml` is listed with a comment and an example value

### Requirement: Independent rollback

Removing the `ducth-dev-website` service block from the apps compose, deleting the `website` router and `website-service` from the Traefik dynamic file, deleting the `website-compress` middleware, removing `CMS_WEBSITE_HOST` from `deployments/docker-swarm/traefik/.env.example`, and removing all `WEBSITE_*` lines from `deployments/docker-swarm/apps/.env.example` SHALL be sufficient to roll the change back. After those edits, the rest of the stack (`supabase`, `init-wait`, `migrate`, `my-cms-api`, `my-cms-frontend`, `jaeger`) SHALL bring up unchanged. No other service SHALL depend on the website service.

#### Scenario: Reader rollback leaves the rest of the stack intact
- **WHEN** a developer performs the edits described in the requirement
- **AND** runs `docker compose -f deployments/docker-swarm/apps/docker-compose.yaml --env-file deployments/docker-swarm/apps/.env up -d`
- **THEN** `my-cms-api`, `my-cms-frontend`, `migrate`, `init-wait`, and `jaeger` start without error
- **AND** `curl -i http://localhost:8989/health` returns HTTP 200
- **AND** `curl -i http://localhost:3002/` returns HTTP 200

### Requirement: Local smoke gate

`curl -i http://localhost:8989/graphql/immutable -H 'content-type: application/json' -d '{"query":"{ categories(filters:{categoryType:{eq:Blog}}){ nodes { displayName slug } } }"}'` SHALL return HTTP 200 and a JSON body whose `data.categories.nodes` is a (possibly empty) array, when the Supabase stack has been seeded with at least one `Blog` category.

#### Scenario: Public GraphQL endpoint reachable from the reader
- **WHEN** the reader container starts and `my-cms-api` has finished its startup
- **THEN** `docker exec ducth-dev-website wget -q -O - http://my-cms-api:8989/health` returns a body containing the string `CMS is running successfully`

#### Scenario: Public GraphQL endpoint reachable from the host
- **WHEN** the apps compose is up and `my-cms-api` is healthy
- **THEN** the smoke gate `curl` above returns HTTP 200 with `data.categories.nodes` as a JSON array
