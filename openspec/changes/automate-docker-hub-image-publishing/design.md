## Context

This change adds reliable, component-scoped Docker Hub publishing without changing an application, a container runtime contract, Compose configuration, or deployment state.

### Current-state evidence

- **Observed:** `.github/workflows/release-my-cms-image.yml` already publishes `doitsu2014/my-cms` from the bounded `apps/api` context on `main` changes under `apps/api/**`. It also declares `v*.*.*` tag triggers, so a version tag is not component-change scoped. It uses Docker Hub repository secrets and produces branch, semver, SHA, and `latest` metadata tags.
- **Observed:** `.github/workflows/release-my-cms-admin-image.yml` already publishes `doitsu2014/my-cms-admin` from `apps/web` with the required `editor-prose=./packages/editor-prose` named BuildKit context. Its path filter already includes `apps/web/**` and `packages/editor-prose/**`; it also declares tag triggers.
- **Observed:** `.github/workflows/validate-app-image-builds.yml` is a dedicated non-publishing matrix that builds API, admin, and Ducth images. The user has explicitly removed it from the approved design; it and every tracked reference that makes it a required contract must be deleted or revised.
- **Observed:** `apps/api/Dockerfile` builds the whole Cargo workspace and ships the gateway `my-cms-api` binary. Graph traversal from `gateway/src/main.rs::main` reaches the migration CLI and the composed domain/service initialization; image publishing must preserve this artifact and its `apps/api` context. The graph has no workflow-YAML nodes (`semantic_search_nodes` found none), so workflow dependencies are established by targeted source reads.
- **Observed:** `apps/web/Dockerfile` and `apps/ducth-dev-website/Dockerfile` each require `packages/editor-prose` through a named BuildKit context at `/packages/editor-prose`. `scripts/validate-app-image-build-contract.sh` asserts this contract and explicitly prevents use of the repository root as the primary image context.
- **Observed:** Docker Swarm Compose builds all three images locally. Kubernetes chart defaults reference the API and admin Docker Hub repositories; no tracked deployment consumer references a Ducth Docker Hub image. Publishing Ducth is nevertheless explicitly approved in this change and does not add a deployment.

### Graph evidence

`get_minimal_context(task="automate-docker-hub-image-publishing")` reported a current `main` graph snapshot (1,893 nodes, 18,350 edges; head matches build) with medium risk. Traversing `apps/api/gateway/src/main.rs::main` identified its migration CLI, configuration, domain manifest, schema, router composition, and direct tests as API-image consumers. No graph result represents GitHub Actions or Dockerfile YAML; source inspection of workflows, Dockerfiles, package manifests, Compose, and the image-build contract script is the evidence source for those configuration flows.

### Affected flow

```text
push to main / intentional manual dispatch
  -> GitHub path selection
  -> selected publisher
  -> scoped Docker context
       API: apps/api
       Admin: apps/web + editor-prose named context
       Ducth: apps/ducth-dev-website + editor-prose named context
  -> Buildx image build
  -> Docker Hub SHA tag (+ latest only for main publishers)
```

No request, API, database, SeaORM entity, migration, runtime environment, routing, or frontend interaction flow changes.

## Goals / Non-Goals

**Goals:**

- Publish the approved API, admin, and Ducth images only after relevant changes reach `main`.
- Retain bounded, reproducible Docker build inputs and use the same frontend named context in every applicable publisher.
- Remove the dedicated pull-request image-validation workflow while retaining safe repository-local publisher-contract checks.
- Make every published image traceable to its source revision while retaining `latest` as the explicit current-main convenience tag.
- Ensure a publisher definition can be locally checked before a registry write.

**Non-Goals:**

- Deploying, promoting, rolling back, or changing Helm/Docker Swarm image consumers.
- Changing Dockerfiles, Docker build contexts, Compose, image contents, application behavior, API routes, secrets at runtime, schemas, or migrations.
- Publishing semantic-version tags, changing Docker Hub visibility/ownership, or adding images for Supabase, Traefik, or other services.
- Retaining or replacing the deleted workflow with a pull-request image-validation workflow.
- Introducing a repository-root Docker context or a new external package registry.

## Decisions

### Decision: Use three isolated main-only publishing workflows with native path filters

**Driver:** Each application has independent source inputs, while `packages/editor-prose` is a shared frontend dependency. Separate workflows provide transparent selection without a third-party changed-files action or a conditional matrix that must propagate outputs between jobs.

**Decision:**

- Update the existing API publisher to trigger on `push` to `main` with `apps/api/**` and its own workflow file.
- Update the existing admin publisher to trigger on `push` to `main` with `apps/web/**`, `packages/editor-prose/**`, and its own workflow file.
- Add a Ducth publisher with `apps/ducth-dev-website/**`, `packages/editor-prose/**`, and its own workflow file; it pushes only `doitsu2014/my-cms-ducth-dev-website`.
- Retain `workflow_dispatch` for an intentional rebuild. A dispatch from a non-main ref can publish its SHA tag but MUST NOT update `latest`; the existing `main`-ref tag condition enforces this.
- Remove semantic-version `push.tags` triggers from the publishers. GitHub Actions does not apply path filters to tag pushes, so retaining them would violate component-scoped publication.

**Alternatives considered:**

- One matrix publisher plus a changed-files action: rejected because it adds an action/output contract and makes per-image ownership, secrets, and failure logs less direct for only three images.
- One publisher on every repository change: rejected because it rebuilds unaffected images and violates the proposal's selection rule.
- Keep semantic-version tag triggers with paths: rejected because GitHub Actions ignores path filtering for tag pushes.

**Consequences:** A shared editor-package edit starts two publishers; an API edit starts one. A workflow-definition edit intentionally runs that workflow so its publishing configuration is exercised on `main`. `workflow_dispatch` remains an authorized, auditable registry side effect.

### Decision: Publish immutable SHA tags and retain `latest` only as the current-main convenience tag

**Driver:** Image consumers need a revision-resolvable artifact; the project already exposes a current-image convention through `latest`.

**Decision:** Each successful publisher generates a Docker metadata SHA tag for its commit. For `main` only, it also generates `latest`. The design explicitly recommends retaining `latest`, but treats the SHA tag (or resulting image digest) as the immutable reference for audit, incident response, and any future deployment. No semver tag is generated by this change.

**Alternatives considered:**

- SHA-only tags: rejected because it removes the existing operator convenience of a current-main image.
- `latest` only: rejected because it is overwritten and cannot identify the exact source revision.
- Semver tags on Git tag pushes: rejected because a tag may not contain relevant component changes and path filters cannot constrain tag events.

**Consequences:** `latest` can remain mutable without losing traceability. This change does not update deployments to consume immutable tags; that is a separate release/deployment decision.

### Decision: Preserve each image's established bounded build contract

**Driver:** The completed `fix-app-image-build-contexts` change established app-scoped primary contexts and named `editor-prose` contexts to avoid broad contexts and make frontend local dependencies reproducible.

**Decision:** The API publisher uses `apps/api` as the primary context. Admin and Ducth publishers use their app directories as primary context and `editor-prose=./packages/editor-prose` as the only additional context. The new Ducth publisher matches the Buildx, metadata, login, and cache pattern of the admin publisher, while retaining the Ducth Dockerfile's existing runner contract. A repository-local contract check confirms these declarations without running a CI image-build workflow.

**Alternatives considered:**

- Use the repository root as all build contexts: rejected because it violates the existing input-boundary specification and can transfer unrelated artifacts or credentials.
- Make frontend publishers install `editor-prose` from a registry: rejected because it changes the approved `file:` dependency contract.
- Build Ducth without publishing it: rejected because the approved proposal explicitly adds its Docker Hub destination.

**Consequences:** A shared package change properly invalidates and republishes both frontend images. The Ducth repository becomes a build artifact only; no deployment manifest is changed.

### Decision: Delete CI image validation and retain local publisher-contract verification

**Driver:** The user explicitly requested removal of `.github/workflows/validate-app-image-builds.yml`; the publishing change must not leave a pull-request image-validation workflow behind.

**Decision:** Delete `validate-app-image-builds.yml` and remove or revise every script assertion and task that requires it. Retain and expand the repository-local `scripts/validate-app-image-build-contract.sh` check so it can inspect the three publisher files for repository names, `main` path filters, no semantic-version tag triggers, SHA/main-`latest` metadata, and required BuildKit contexts. The check is static and shall not authenticate, push, or require Docker Hub secrets. Developers may run direct local Docker builds separately when Docker build inputs change, but they are not a replacement CI workflow in this change.

**Alternatives considered:**

- Retain the existing non-publishing matrix: rejected by the user's explicit deletion request.
- Replace it with another pull-request image-validation workflow: rejected because it conflicts with the approved scope.
- Remove all workflow checks: rejected because path-filter, tag, repository, and named-context drift would be unnecessarily easy to ship.

**Consequences:** Pull requests no longer get an automatic application-image build from this change. The focused local contract check remains the pre-push/review safeguard, and successful `main` publication remains the authoritative Buildx execution.

### Decision: Keep registry authorization minimal and avoid publisher races on `latest`

**Driver:** Docker Hub credentials are an external write authority, and concurrent main workflow runs can race to update mutable `latest`.

**Decision:** Publisher jobs declare `permissions: contents: read` and authenticate with repository secrets (`DOCKER_USERNAME` and a scoped Docker Hub access token stored in `DOCKER_PASSWORD`). The local contract check uses no credentials. Each publisher uses a workflow-specific concurrency group with `cancel-in-progress: true` so an older in-flight main build does not finish after a newer main build and replace `latest`. SHA tags remain traceable even if a superseded run is cancelled before publication.

**Alternatives considered:**

- Use a long-lived account password: rejected because a scoped token provides a smaller revocable publishing authority.
- Share one global concurrency group: rejected because independent API and frontend images can publish safely in parallel.
- No concurrency control: rejected because an older successful build can race with a newer commit for `latest`.

**Consequences:** Docker Hub credentials remain limited to publishing jobs. Repository administrators must configure the token with write access to the three approved public repositories before enabling the Ducth publisher.

## Security, operations, and compatibility

- Docker Hub is an external integration. Registry login occurs only after `main` path selection or an intentional manual dispatch. The local workflow-contract check has no login step, no publishing secret references, and no registry side effects.
- GitHub Actions logs must not echo secret values. The source SHA tag, generated image digest, workflow URL, and selected repository are safe operational telemetry and should be visible in job summaries/logs.
- This change has no data, request concurrency, cache, authentication, API compatibility, or runtime-config migration. Workflow concurrency is limited to controlling writes to each repository's mutable `latest` tag.
- Existing API runtime behavior remains the gateway binary, including `my-cms-api migrate <verb>` and router composition. Existing frontend runtime artifacts, health endpoints, and non-root Ducth container user are unchanged.

## Verification strategy

| Requirement / risk | Verification | Evidence |
| --- | --- | --- |
| Correct publisher selection | Run the local contract script; review publisher `push.branches`/`paths` and absence of tag triggers; exercise API-only, frontend-only, shared-package, workflow-only, and unrelated path cases with controlled main runs | Contract-check output and GitHub Actions publisher selection |
| Dedicated validation removal | Inspect `.github/workflows/` and run the local contract script | `validate-app-image-builds.yml` is absent; script does not require it |
| Correct image build inputs | Run the local contract script and direct local Buildx builds when Docker build inputs are affected | `./scripts/validate-app-image-build-contract.sh`; direct build logs |
| Frontend named-context parity | Inspect admin and Ducth publisher `build-contexts`; run direct local frontend builds when needed | Buildx configuration/logs show `editor-prose=./packages/editor-prose` |
| Traceable publications | Trigger selected main changes for each component and inspect metadata action outputs/Docker Hub | SHA tag and digest exist in the approved repository; `latest` points to the same selected main result |
| Race protection | Start consecutive main-branch runs for one image or review concurrency configuration | Superseded publisher is cancelled; latest completed selected revision owns `latest` |

The full application verification gate remains applicable after the CI configuration edits: `cargo check`, `cargo test`, `cargo fmt -- --check`, `cargo clippy` from `apps/api`, plus `pnpm --dir apps/web build`. Frontend test/build commands and the Ducth build should also be run when Docker or package build inputs are affected.

## Migration Plan

1. Add the Ducth publishing workflow and update API/admin trigger, tag, permission, concurrency, and metadata configuration without changing build contexts or image contents.
2. Delete `.github/workflows/validate-app-image-builds.yml` and revise the local image-build contract script so it no longer expects that workflow while it checks every publisher.
3. Run workflow lint/YAML review and the local image-build contract script; run direct local Buildx builds if Docker build inputs are changed.
4. Configure/confirm a scoped Docker Hub access token and run a selected `main` publication for each image. Record source SHA tag and digest.
5. No environment rollout is authorized or required. Any later deployment uses a separately approved immutable SHA tag or digest and has its own release plan.

### Rollback

Revert the relevant publishing-workflow and local contract-check commits to stop future automated publication or restore a prior contract. The deleted validation workflow remains absent. If `latest` must be restored, an authorized operator retags the previously recorded SHA/digest in the affected Docker Hub repository; no database, application, or deployment rollback is required. Existing immutable SHA-tagged images remain available unless an operator separately deletes them.

## Risks / Trade-offs

- **[GitHub tag events bypass path filtering]** → Remove semantic-version tag triggers; release selection is only `main` path-filtered pushes.
- **[Incorrect filter leaves a public image stale]** → Include every declared Docker input and publisher workflow definition in the local contract check; verify path cases with controlled main runs.
- **[Shared package is omitted from a frontend build]** → Preserve the named BuildKit context and run the existing contract script plus real image builds.
- **[Docker Hub/base image/package registry outage]** → Treat the job as an external infrastructure failure, retain source SHA evidence, and rerun after remediation rather than bypassing publication controls.
- **[Mutable `latest` is overwritten out of order]** → Use per-image concurrency and regard SHA/digest as the rollback/audit reference.
- **[Token has excessive scope or leaks into tracked configuration]** → Use a scoped revocable token only in publisher jobs; the local contract check has no login or secret reference.
- **[Ducth image is published but not deployed]** → This is intentional approved artifact publication; a deployment consumer is a separate change with release authorization.

## Open Questions

None block implementation. The approved repositories, main-only publishing policy, Ducth publication, SHA tagging, retention of `latest`, and removal of the pull-request image-validation workflow are explicit. The implementation owner must verify repository-secret configuration before enabling publishing; missing or unauthorized credentials are a release-configuration failure, not a reason to weaken the local contract check.
