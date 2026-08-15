## MODIFIED Requirements

### Requirement: Three-stage Node 20 Alpine Dockerfile

The repository SHALL include `apps/ducth-dev-website/Dockerfile` with three stages:
- `deps` — `FROM node:20-alpine`, runs `corepack enable pnpm && pnpm install --frozen-lockfile` to produce `node_modules`.
- `builder` — `FROM node:20-alpine`, copies `node_modules` from `deps`, copies the application source, runs `pnpm run build` to produce `dist/client/*` and `dist/server/index.mjs`.
- `runner` — `FROM node:20-alpine`, creates non-root user `expressjs` with uid 1001 and group `nodejs` with gid 1001, copies `dist`, `server.prod.mjs`, `package.json`, and production `node_modules` from `deps`, sets `ENV NODE_ENV=production`, `ENV PORT=3001`, `EXPOSE 3001`, `USER expressjs`, and `CMD ["node", "server.prod.mjs"]`.

The file SHALL use `# syntax=docker.io/docker/dockerfile:1` as its first line. The `deps` and `builder` stages SHALL receive the workspace-local `packages/editor-prose` package through the named BuildKit context `editor-prose`, make it available at the path required by the application's `file:../../packages/editor-prose` dependency, and SHALL NOT require the repository root as the primary context.

#### Scenario: Build produces a runnable image
- **WHEN** `docker build --build-context editor-prose=packages/editor-prose -f apps/ducth-dev-website/Dockerfile apps/ducth-dev-website` completes from the repository root
- **THEN** the resulting image has a `node:20-alpine`-based final layer with `USER expressjs`
- **AND** `docker run --rm <image> node -e "console.log(require('./package.json').name)"` prints `ducth-dev-website`

#### Scenario: Build fails fast on missing lockfile
- **WHEN** `pnpm-lock.yaml` is absent from the build context
- **THEN** the `deps` stage fails with a non-zero exit code
- **AND** the failure message names `pnpm-lock.yaml`

#### Scenario: Build fails fast on missing editor package context
- **WHEN** the Ducth website Dockerfile is invoked without the named `editor-prose` BuildKit context
- **THEN** the build fails with a non-zero exit code before a runnable image is produced
- **AND** the failure identifies the missing `editor-prose` build input

## ADDED Requirements

### Requirement: Website Compose build SHALL provide a bounded shared package context

The `ducth-dev-website` service in `deployments/docker-swarm/apps/docker-compose.yaml` SHALL retain `apps/ducth-dev-website` as its primary build context and SHALL provide `packages/editor-prose` as an `editor-prose` additional build context. The service SHALL NOT widen its primary context to the repository root to obtain the shared package. This build-input requirement SHALL preserve the existing service networking, environment, runtime, and routing contracts.

#### Scenario: Compose build supplies the shared package input
- **WHEN** `docker compose -f deployments/docker-swarm/apps/docker-compose.yaml build ducth-dev-website` is run from its supported repository invocation
- **THEN** the service build receives `apps/ducth-dev-website` as its primary context and `packages/editor-prose` as its `editor-prose` additional context
- **AND** the website image build succeeds without a repository-root primary context
