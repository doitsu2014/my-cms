## Context

This change repairs the image-build input boundary for the API, admin web application, and Ducth website without changing application behavior or deploying an image.

### Current-state evidence

- **Observed:** `apps/api/Dockerfile` builds the complete Rust workspace from the app-scoped `apps/api` context and copies only `target/release/my-cms-api` into the runtime image. Its `.dockerignore` does not exclude `target/`; local inspection measured `apps/api/target` at 94 GB, matching the excessive context-transfer symptom.
- **Observed:** `apps/web/package.json` and `apps/ducth-dev-website/package.json` each declare `editor-prose` as `file:../../packages/editor-prose`. Their Dockerfiles use only `/app` as the primary build context, so that relative dependency resolves to `/packages/editor-prose`, which is absent during `pnpm install` and the builder stage.
- **Observed:** `apps/web/Dockerfile` copies only `package.json` and performs a non-frozen `pnpm install`; it does not copy the committed `pnpm-lock.yaml`. The Ducth Dockerfile already copies its lockfile and uses `--frozen-lockfile`.
- **Observed:** `deployments/docker-swarm/apps/docker-compose.yaml` keeps all primary contexts app-scoped but supplies no additional contexts to `my-cms-frontend` or `ducth-dev-website`. The same API image is built by both `migrate` and `my-cms-api` services.
- **Observed:** `.github/workflows/release-my-cms-admin-image.yml` builds `./apps/web`, has no `build-contexts` input, and triggers only on `apps/web/**`. No non-publishing workflow currently builds all three application images.
- **Observed:** `packages/editor-prose` has source, lockfile, and local dependency installation data but no `.dockerignore`; inspection measured it at 99 MB, so its named context must be explicitly bounded.
- **Observed:** the gateway binary exposes public `/health` and `/healthz` handlers in `apps/api/gateway/src/main.rs`, while `apps/api/gateway/src/migrate_cli.rs` supports `my-cms-api migrate <verb>`. The image change must preserve both entrypoints because Compose reuses the image for migration and runtime services.

### Architecture evidence limitation

The mandatory `code-review-graph` query was not callable in this session, and the supplied graph metadata says its snapshot was built on `hotfix/ducth_website` while the workspace is on `main`. It was therefore not used. This design substitutes targeted source reads, canonical-spec review, package/lockfile inspection, Compose and workflow inspection, current test/workflow searches, Buildx/Compose capability checks (`docker buildx v0.35.0`, Compose v5.3.1), and working-tree size inspection. No graph-derived callers, flows, or tests are claimed.

### Affected flow

```text
Docker/Compose/GitHub Actions entry point
  -> app-scoped primary context (apps/api | apps/web | apps/ducth-dev-website)
  -> optional named editor-prose context (packages/editor-prose)
  -> Dockerfile dependency stage
  -> pnpm local file dependency or Cargo workspace build
  -> final application image
  -> (outside this change) Compose runtime/migration service or published release image
```

No HTTP route, authorization boundary, SeaORM entity, migration, external service, secret contract, or frontend UX flow changes.

## Goals / Non-Goals

**Goals:**

- Make all three declared application images reproducibly buildable from supported local, Compose, and applicable release entry points.
- Keep primary Docker contexts app-scoped, exclude local build/dependency artifacts, and provide only `packages/editor-prose` as a named additional frontend context.
- Preserve the local `file:` dependency contract and make container dependency installation deterministic from committed lockfiles.
- Add non-publishing CI coverage for all three images and prevent admin release drift when the shared package changes.
- Preserve the API image's migration CLI and runtime `/healthz` behavior and the Ducth image's runtime-user and healthcheck behavior.

**Non-Goals:**

- Changing application source behavior, API contracts, database state, schema/migrations, authentication, runtime environment variables, network routing, base images, or deployment policy.
- Switching to a repository-root Docker context, publishing `editor-prose`, or replacing its `file:` dependency with a registry package.
- Starting containers, running a production rollout, or changing image tags as part of this change.
- Correcting unrelated pre-existing Compose/canonical-spec drift (including runtime ports); this change only adds build-input configuration and preserves the service's existing runtime fields.

## Decisions

### Decision: Retain app-scoped primary contexts and use one named `editor-prose` context

**Driver:** Frontend builds require the workspace-local package, while the API's 94 GB local `target/` directory demonstrates the cost and risk of a broad context.

**Decision:** Keep `apps/api`, `apps/web`, and `apps/ducth-dev-website` as the only primary build contexts. Add `editor-prose` as a BuildKit named context whose source is exactly `packages/editor-prose` for the two frontend builds. Add `target/` to `apps/api/.dockerignore` and add a package-local `.dockerignore` to exclude `node_modules`, build output, coverage, credentials, editor metadata, and logs.

**Alternatives considered:**

- Use the repository root as every Docker build context — rejected because it transfers unrelated app artifacts, weakens secret/input isolation, and reintroduces the API context-size failure.
- Publish `editor-prose` to a package registry — rejected because it changes the approved local dependency contract and adds registry/release lifecycle scope.
- Copy the package ad hoc from the host outside BuildKit contexts — rejected because it is non-portable and unavailable to GitHub Actions builders.

**Consequences:** Dockerfile `COPY --from=editor-prose` syntax and all supported build callers must use BuildKit. Missing named contexts fail early, which is intentional and covered by CI.

### Decision: Stage the shared package at `/packages/editor-prose` in both frontend build stages

**Driver:** The existing manifest paths resolve from `/app` to `/packages/editor-prose`; `pnpm` must access the package during install, and the builder must retain it when copied dependency metadata references the local directory.

**Decision:** In both frontend Dockerfiles, copy the named context into `/packages/editor-prose` before `pnpm install` in `deps` and make it available again in `builder` before `pnpm run build`. The admin `deps` stage will copy `pnpm-lock.yaml` and use `pnpm install --frozen-lockfile`; the Ducth stage retains its existing frozen install. The context root is copied, not a hand-maintained subset, so package source and manifest changes are coherent; the package `.dockerignore` supplies the boundary.

**Alternatives considered:**

- Change manifests to a different in-container dependency path — rejected because it changes the local development contract and lockfile resolution.
- Copy the package only in `deps` — rejected because pnpm's local-file resolution can retain a link/path that the builder still needs.
- Vendoring the package into each app — rejected because it duplicates the canonical package and violates the existing package specification.

**Consequences:** Dockerfile layers are cacheable by lockfile and package content, and package edits correctly invalidate frontend dependency/build layers. No package source is generated or modified.

### Decision: Align Compose, direct Docker, release, and CI entry points

**Driver:** A Dockerfile contract is insufficient if Compose or GitHub Actions omit the additional context; current local and release build paths already diverge.

**Decision:**

- Add `additional_contexts.editor-prose: ../../../packages/editor-prose` to both frontend Compose build blocks, preserving their primary `context` values.
- Use the direct repository-root invocation `docker build --build-context editor-prose=packages/editor-prose -f <app>/Dockerfile <app>` for each frontend image; the API remains `docker build -f apps/api/Dockerfile apps/api`.
- Set `build-contexts: editor-prose=./packages/editor-prose` in the admin release workflow and expand its path filter to `packages/editor-prose/**`.
- Add a dedicated non-publishing image validation workflow for pull requests and relevant pushes. It will build the three images with their declared bounded contexts and validate the Compose configuration/build path. It must not log in, push, or require runtime secrets.

**Alternatives considered:**

- Update only Compose — rejected because the admin release workflow would still fail and direct developer builds would remain undocumented/unverified.
- Reuse the publishing release job as the only test gate — rejected because it couples validation to registry credentials and can publish before a regression is isolated.
- Add just static YAML checks — rejected because they cannot prove that pnpm resolves the local package in the container.

**Consequences:** CI spends time building Rust and Node images, but failures are isolated by image name and catch actual Docker input defects. A CI build may access public base images and package registries; that is an external availability dependency, not an application secret dependency.

### Decision: Treat this as build/deployment configuration with no data migration or runtime rollout

**Driver:** The changed inputs are Docker ignore rules, Dockerfiles, Compose build metadata, and workflows. The delivered binaries, database schema, HTTP contracts, configuration keys, and service image names remain unchanged.

**Decision:** Do not add a SeaORM migration, generated-entity work, runtime API instrumentation, or application-layer code. Release validation will be a staged build and smoke assessment after implementation evidence is available; environment-changing commands require explicit release authorization.

**Alternatives considered:**

- Bundle a deployment rollout with the repair — rejected because the approved change explicitly excludes deployment and needs no runtime contract change.
- Skip runtime smoke planning — rejected because the API image is used for both migration and serving, and the website has a healthcheck; release readiness still needs evidence when deployment is separately authorized.

**Consequences:** rollback is configuration/source revert plus reusing prior image digests/tags. There is no data rollback and no compatibility window.

## Contracts and layer impact

| Outcome | Contract / affected layer | Implementation boundary | Verification |
| --- | --- | --- | --- |
| Bounded API input | `apps/api/.dockerignore`; API Docker build context | Docker build only; no gateway code change | Build with a populated local `target/` and inspect BuildKit context output; run the API image build |
| Shared frontend package | `editor-prose` BuildKit named context; `/packages/editor-prose` in build stages | Frontend Dockerfiles and package-local ignore file | Direct named-context builds; frozen install; frontend tests/builds |
| Local Compose parity | `additional_contexts` on both frontend services | `deployments/docker-swarm/apps/docker-compose.yaml` | `docker compose config`; all-app Compose build |
| Admin release parity | `build-contexts` and path filter | `.github/workflows/release-my-cms-admin-image.yml` | Workflow YAML review and a local/direct admin image build |
| Regression gate | Non-publishing GitHub Actions workflow | CI only; no registry login/push | CI execution builds API, admin, and website images and labels the failed image |
| Deployment contract | `website-deployment` delta specification | OpenSpec documentation | OpenSpec status/validation and direct website named-context build |

## Security, reliability, and operations

- Build contexts are a supply-chain and privacy boundary. The package `.dockerignore` and API `target/` exclusion prevent unneeded local dependencies, artifacts, and common credentials from reaching the builder. Existing Docker ignore exclusions remain in force.
- The CI validation uses no deployment secrets or runtime variables and is non-publishing. The admin release job retains its existing registry-secret boundary but receives an explicit local package context.
- No request-time observability changes are necessary. CI logs must name the image build step that failed; BuildKit context transfer output is retained as diagnostic evidence for the API size regression.
- There is no data transaction, concurrency, idempotency, cache, background work, authentication, or third-party API behavior introduced by this change.
- External risks are base-image and package-registry availability, BuildKit/Compose support, and platform-specific image build duration. CI must surface these as infrastructure failures rather than silently bypassing image validation.

## Verification strategy

| Requirement / risk | Test-first or verification activity | Target command / evidence |
| --- | --- | --- |
| API excludes local artifacts | Add/adjust a focused build-contract check before ignore/Dockerfile edits; create a harmless `apps/api/target` sentinel when validating the context | `docker build --progress=plain -t my-cms-api:build-check -f apps/api/Dockerfile apps/api` and BuildKit context log review |
| Admin and website local package inputs | Add/adjust a focused build-contract check; run direct builds with the named context | `docker build --build-context editor-prose=packages/editor-prose -t my-cms-admin:build-check -f apps/web/Dockerfile apps/web`; equivalent website command |
| Compose wiring | Validate rendered Compose configuration, then build all application services | `docker compose -f deployments/docker-swarm/apps/docker-compose.yaml config`; `docker compose -f deployments/docker-swarm/apps/docker-compose.yaml build` |
| Frontend behavior preservation | Run existing test/build suites after container changes | `pnpm --dir apps/web test`, `pnpm --dir apps/web build`, `pnpm --dir apps/ducth-dev-website test`, `pnpm --dir apps/ducth-dev-website build` |
| API behavior preservation | Run repository Rust verification gate | `cargo check`, `cargo test`, `cargo fmt -- --check`, `cargo clippy` from `apps/api` or the repository's documented workspace command |
| CI non-publishing gate | Review workflow permissions/steps and run it in CI | workflow logs show three build attempts and no login/push step |
| Authorized staging release only | Bring the existing stack up with its authorized environment, then check image/runtime probes | API `GET /healthz`; admin `GET /health`; website `GET /en` and Docker health; Traefik route checks |

## Migration Plan

1. Add the ignore boundaries and Dockerfile named-context staging while preserving image names, entrypoints, base images, ports, users, and healthchecks.
2. Add the two Compose `additional_contexts` declarations and the admin release workflow's `build-contexts` and package trigger.
3. Add the non-publishing three-image CI build gate, then run focused direct and Compose build checks.
4. Run the full Rust and frontend verification gates. Supply the completed implementation evidence and operational impact summary to the Release Engineer.
5. No environment is changed by this change. If a later staging deployment is authorized, build/validate staging images first; check API `/healthz`, admin `/health`, Ducth `/en` and its health status, then Traefik routes. Promote only with separate release authorization.

### Rollback

If an image build or staging smoke check fails, revert the Docker ignore, Dockerfile, Compose, and workflow changes together or redeploy the prior immutable image digest/tag after explicit authorization. The rollback does not require a database migration, data recovery, cache purge, or configuration-key rollback. A release workflow path-filter change can be reverted independently only after confirming it does not leave the Dockerfile requiring a context the workflow fails to provide.

## Risks / Trade-offs

- **[Named-context support differs across builders]** → Use the repository's observed Compose v5.3.1 and Buildx v0.35.0 in local verification, use Docker Buildx in CI, and fail clearly when a builder cannot provide the context.
- **[Package context grows or includes sensitive/local content]** → Add and test the package-local `.dockerignore`; retain an exact package directory source rather than broad repository context.
- **[Dockerfile staging path diverges from `file:` dependency resolution]** → Preserve `/app` and stage at `/packages/editor-prose`; prove both dependency installation and compilation with frozen direct builds.
- **[Compose, release, and CI drift]** → Require all entry points in the specification and validate the actual Compose build plus the release workflow's context/filter configuration.
- **[Long Rust image builds slow CI]** → Use cache where appropriate but retain a true image build; use per-image named steps/logs so a failure is actionable.
- **[External registries/base images are unavailable]** → Treat as an infrastructure failure with rerun/availability remediation, not as permission to skip image validation.
- **[Pre-existing Compose runtime/spec drift is accidentally expanded]** → Tasks are restricted to build inputs and tests; preserve unrelated ports, routing, and runtime fields and record any discovered drift separately.

## Open Questions

None block implementation. The implementation owner may choose the exact workflow filename and job/matrix layout provided it remains non-publishing, builds all three images, and honors the required bounded contexts. Any request to change runtime Compose behavior or perform staging/production commands requires separate scope and release authorization.
