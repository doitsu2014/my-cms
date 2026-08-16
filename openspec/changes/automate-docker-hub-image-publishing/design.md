## Context

This change retains reliable, component-scoped `main` Docker Hub publishing and adds an immutable, release-versioned publication path for a manually published GitHub Release. It does not change an application, a container runtime contract, Compose configuration, or deployment state.

### Current-state evidence

- **Observed:** `.github/workflows/release-my-cms-image.yml` publishes `doitsu2014/my-cms` from the bounded `apps/api` context only on its selected `main` paths or an intentional dispatch. It produces a SHA tag and emits `latest` only when `github.ref` is `refs/heads/main`.
- **Observed:** `.github/workflows/release-my-cms-admin-image.yml` and `.github/workflows/release-my-cms-ducth-dev-website-image.yml` respectively publish the approved admin and Ducth repositories from an app-scoped primary context plus the required `editor-prose=./packages/editor-prose` named BuildKit context. Both follow the same SHA/main-`latest` convention.
- **Observed:** `.github/workflows/validate-app-image-builds.yml` is absent. `scripts/validate-app-image-build-contract.sh` statically validates the three main publishers, their repositories, contexts, tag policy, permissions, concurrency, and the deleted validation workflow without logging in or pushing.
- **Observed:** `apps/api/Dockerfile` builds the workspace gateway binary from its app-scoped context. The release workflow must preserve that artifact boundary.
- **Observed:** `apps/web/Dockerfile` and `apps/ducth-dev-website/Dockerfile` each require `packages/editor-prose` through a named BuildKit context at `/packages/editor-prose`. `scripts/validate-app-image-build-contract.sh` asserts this contract and explicitly prevents use of the repository root as the primary image context.
- **Observed:** Docker Swarm Compose builds all three images locally. Kubernetes chart defaults reference the API and admin Docker Hub repositories; no tracked deployment consumer references a Ducth Docker Hub image. Publishing Ducth is nevertheless explicitly approved in this change and does not add a deployment.

### Graph-gate limitation and fallback evidence

`code-review-graph` is unavailable in this architecture session, so no graph finding is asserted for this amendment. Targeted inspection of the three publisher workflows, local contract script, Dockerfiles, package manifests, Compose consumers, active proposal, and existing delta artifacts is the evidence source. GitHub Actions YAML and Dockerfiles are configuration boundaries rather than source-code call edges.

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

published GitHub Release vX.Y.Z
  -> release-tag validation (no external write if invalid)
  -> immutable version-tag preflight across all three repositories
  -> full-depth checkout of the release tag/ref
  -> Buildx API, admin, and Ducth builds with their existing bounded contexts
  -> Docker Hub release-version + SHA tags (never latest)
  -> idempotent complete or explicitly incomplete GitHub Release manifest
```

No request, API, database, SeaORM entity, migration, runtime environment, routing, or frontend interaction flow changes.

## Goals / Non-Goals

**Goals:**

- Publish the approved API, admin, and Ducth images only after relevant changes reach `main`.
- Build all three approved images from a manually published valid GitHub Release ref and make that version independently traceable.
- Retain bounded, reproducible Docker build inputs and use the same frontend named context in every applicable publisher.
- Remove the dedicated pull-request image-validation workflow while retaining safe repository-local publisher-contract checks.
- Make every published image traceable to its source revision while retaining `latest` as the explicit current-main convenience tag.
- Ensure a publisher definition can be locally checked before a registry write.
- Record Buildx digests in an idempotent GitHub Release-note manifest without overwriting maintainer-authored notes.

**Non-Goals:**

- Deploying, promoting, rolling back, or changing Helm/Docker Swarm image consumers.
- Changing Dockerfiles, Docker build contexts, Compose, image contents, application behavior, API routes, secrets at runtime, schemas, or migrations.
- Triggering a release from a semantic Git tag push, creating GitHub Releases automatically, moving `latest`, changing Docker Hub visibility/ownership, or adding images for Supabase, Traefik, or other services.
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

**Decision:** Each successful main publisher generates a Docker metadata SHA tag for its commit. For `main` only, it also generates `latest`. The design treats the SHA tag (or resulting image digest) as the immutable reference for audit, incident response, and any future deployment. The release publisher described below uses exact raw `vX.Y.Z` plus SHA tags and explicitly disables `latest`; it does not use a Docker metadata semver tag rule.

**Alternatives considered:**

- SHA-only tags: rejected because it removes the existing operator convenience of a current-main image.
- `latest` only: rejected because it is overwritten and cannot identify the exact source revision.
- Semver tags on Git tag pushes: rejected because a tag may not contain relevant component changes and path filters cannot constrain tag events.

**Consequences:** `latest` can remain mutable without losing traceability. This change does not update deployments to consume immutable tags; that is a separate release/deployment decision.

### Decision: Use one release-only workflow to publish every component from the published release ref

**Driver:** A product release must identify one complete, source-consistent image set. Adding a release trigger to the three existing main publishers would inherit their independent path selection and mutable-`latest` behavior, and would not provide one release manifest.

**Decision:** Add exactly one workflow, `.github/workflows/publish-github-release-images.yml`, triggered only by `release: types: [published]`. It has no `push`, `pull_request`, or `workflow_dispatch` trigger. An initial job validates the tag with `^v[0-9]+\.[0-9]+\.[0-9]+$`; an invalid tag fails before Docker login, Buildx, or release-note mutation. The release workflow builds API, admin, and Ducth regardless of changed paths, because publishing the release represents the entire tagged source revision. Each component job checks out `${{ github.event.release.tag_name }}` with `fetch-depth: 0`, captures `git rev-parse HEAD`, and verifies it is building the release tag/ref rather than the default branch.

The workflow preserves the existing contexts: API `apps/api`; admin `apps/web` plus `editor-prose=./packages/editor-prose`; Ducth `apps/ducth-dev-website` plus the same named context. Metadata uses an exact raw release tag and the established SHA-tag convention, with `flavor: latest=false` explicitly set and no semver rule. This prevents metadata defaults from creating or moving `latest`.

**Alternatives considered:**

- Add `release` triggers to the three main publisher files: rejected because it risks `latest`, cannot atomically report a complete set, and mixes incompatible source-selection rules.
- Trigger on a semantic-version tag push: rejected because it is not an explicit published release and GitHub does not apply path filters to tag pushes.
- Build default-branch head: rejected because it can differ from the source represented by the published release.

**Consequences:** Main current-image behavior remains unchanged. A manually published valid GitHub Release is the only automatic release-publication trigger, and all three image records point to the same tagged revision.

### Decision: Enforce a workflow-owned immutable-version and manifest protocol

**Driver:** A release tag must not silently be repointed, and a released GitHub page must not imply a full artifact set when a registry build only partially succeeded.

**Decision:** After validation and Docker login, the workflow inspects all three `repository:vX.Y.Z` manifests before it starts any Buildx push. Any existing version tag fails the run before release publication. Root workflow concurrency is keyed by release version (for example `docker-hub-release-${{ github.event.release.tag_name }}`) with `cancel-in-progress: false`, so retries serialize rather than cancel the in-progress attempt. This enforces immutability for workflow-owned writes; Docker Hub cannot make a check-then-push sequence atomic against an out-of-band registry writer, so the scoped credentials must not be used to mutate release tags manually.

The workflow has a final `always()` manifest job after the component jobs. Successful component jobs expose their checked-out SHA and `docker/build-push-action` `digest` output. The finalizer reads the existing GitHub Release body, replaces an existing marker-delimited block or appends one if absent, and writes it back with the GitHub-provided token. The block has a `COMPLETE` status only when all three builds/pushes succeed; it lists component, repository, exact tag, SHA, and digest. If one or more components started or succeeded but the set is incomplete, it instead writes `INCOMPLETE`, names failed/unavailable components, preserves available evidence, and exits non-zero. Invalid-tag and preflight failures do not update notes because publication did not begin.

**Alternatives considered:**

- Permit Buildx to overwrite a version tag on retry: rejected because release identity would be mutable.
- Append a note from each build job: rejected because retries duplicate notes and partial runs can appear complete.
- Write a manifest only after success: rejected because operators lose the evidence needed to diagnose a partial registry publication.

**Consequences:** A GitHub Release can remain published while its image publication workflow fails, but its owned manifest cannot claim completion. Correction requires a newly published release version; it never repoints the old version.

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

- Docker Hub is an external integration. Registry login occurs only after `main` path selection, an intentional manual dispatch, or a validated published GitHub Release. The local workflow-contract check has no login step, no publishing secret references, and no registry side effects.
- GitHub Actions logs must not echo secret values. The source SHA tag, generated image digest, workflow URL, and selected repository are safe operational telemetry and should be visible in job summaries/logs.
- This change has no data, request concurrency, cache, authentication, API compatibility, or runtime-config migration. Workflow concurrency protects per-image `latest` updates on `main` and serializes a release-version publication without cancellation.
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
| Release trigger and bounded source | Review `release: types: [published]`, exact `vX.Y.Z` validation, tag checkout, and all three Buildx contexts in the release publisher | Static contract-check output and workflow review show no tag-push trigger, default-branch checkout, or repository-root build context |
| Release tag safety and audit evidence | Exercise a valid release in an authorized repository and inspect the resulting GitHub Release and Docker Hub repositories | One `vX.Y.Z` tag plus SHA tag per image; release-note manifest names the common SHA and each Buildx digest; `latest` is unchanged |
| Incomplete release handling | Review the finalizer's `always()` path and, in an authorized non-production release test, force or observe one component failure | The release-note block is `INCOMPLETE`, names unavailable components, and the workflow fails without a complete claim |

The full application verification gate remains applicable after the CI configuration edits: `cargo check`, `cargo test`, `cargo fmt -- --check`, `cargo clippy` from `apps/api`, plus `pnpm --dir apps/web build`. Frontend test/build commands and the Ducth build should also be run when Docker or package build inputs are affected.

## Migration Plan

1. Retain the existing three main-only publishers and add the release-only publisher without changing Dockerfiles, app code, Compose, or existing image contexts.
2. Extend the local image-build contract script to distinguish the three main publishers from the release publisher and to reject a release path that could update `latest` or run on a semantic tag push.
3. Run workflow YAML review and the local image-build contract script. Run direct local Buildx builds only if Docker build inputs have changed.
4. Configure/confirm the scoped Docker Hub token and GitHub Actions release-note authority, then use an authorized manually published test `vX.Y.Z` release to record source SHA tags, version tags, digests, and release manifest behavior.
5. No environment rollout is authorized or required. Any later deployment uses a separately approved immutable SHA tag or digest and has its own release plan.

### Rollback

Revert the release publisher and local contract-check changes to stop future release-triggered publication while retaining the existing `main` publishers. The deleted validation workflow remains absent. If `latest` must be restored, an authorized operator retags the previously recorded SHA/digest in the affected Docker Hub repository; a GitHub Release build never moves `latest`. A partial release must be superseded with a newly published version; its existing version tags are not repointed. No database, application, or deployment rollback is required.

## Risks / Trade-offs

- **[GitHub tag events bypass path filtering]** → Keep semantic-version tag pushes out of the main publishers; use only an explicit published GitHub Release to request a whole-release image set.
- **[Incorrect filter leaves a public image stale]** → Include every declared Docker input and publisher workflow definition in the local contract check; verify path cases with controlled main runs.
- **[Shared package is omitted from a frontend build]** → Preserve the named BuildKit context and run the existing contract script plus real image builds.
- **[Docker Hub/base image/package registry outage]** → Treat the job as an external infrastructure failure, record any incomplete release-manifest evidence, and publish a new release version after remediation rather than bypassing immutable version controls.
- **[Mutable `latest` is overwritten out of order]** → Use per-image concurrency and regard SHA/digest as the rollback/audit reference.
- **[Token has excessive scope or leaks into tracked configuration]** → Use a scoped revocable token only in publisher jobs; the local contract check has no login or secret reference.
- **[Ducth image is published but not deployed]** → This is intentional approved artifact publication; a deployment consumer is a separate change with release authorization.
- **[A release is published but its image set is incomplete]** → The finalizer writes only an `INCOMPLETE` block and fails the workflow; operators must not promote the release and must publish a new version after remediation.

## Open Questions

None block implementation. The approved repositories, main-only publishing policy, Ducth publication, SHA tagging, retention of `latest`, GitHub Release event contract, and removal of the pull-request image-validation workflow are explicit. The implementation owner must verify repository-secret configuration and release-note write authority before enabling publishing; missing or unauthorized credentials are a release-configuration failure, not a reason to weaken the local contract check.
