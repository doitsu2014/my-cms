## Why

Changes to an application can currently leave its Docker Hub image stale, and the Ducth website has no publishing path. The project needs a reliable, traceable way to publish each public application image only when its relevant source or shared build input changes.

## What Changes

- Establish automated Docker Hub publishing for the API, admin, and Ducth development website images from the main branch when their relevant build inputs change.
- Publish to the approved public repositories: `doitsu2014/my-cms`, `doitsu2014/my-cms-admin`, and `doitsu2014/my-cms-ducth-dev-website`.
- Select builds with application- and shared-package-aware path filters, so unrelated repository changes do not rebuild or republish an image.
- Delete the dedicated non-publishing application-image validation workflow; publisher configuration remains covered by safe local workflow-contract verification.
- Publish tags that identify the source revision and retain the established main-branch current-image convention, so consumers can select a traceable image rather than relying only on a mutable tag.

## Capabilities

### New Capabilities

- `docker-hub-image-publishing`: Build-selection, local workflow-contract verification, and public Docker Hub publishing behavior for the API, admin, and Ducth development website images.

### Modified Capabilities

- None.

## Impact

- Affected system: GitHub Actions workflows under `.github/workflows/`, including the existing API/admin release workflows, the new Ducth release workflow, and deletion of `validate-app-image-builds.yml`.
- External dependency: Docker Hub credentials configured as GitHub Actions secrets, with permission to publish the three approved public repositories.
- Build inputs: API files; admin and Ducth website files; and `packages/editor-prose` for both frontend images.
- No application behavior, API contract, database schema, image deployment, runtime configuration, or Docker Hub repository visibility is changed.

## Scope

In scope is building and pushing the three existing application images to their approved public Docker Hub repositories after relevant changes reach `main`, deleting the dedicated application-image validation workflow, retaining local workflow-contract verification, and producing source-traceable image tags. Manual workflow dispatch remains available for an intentional rebuild.

## Non-goals

- Deploying, promoting, or rolling back images in any environment.
- Changing Dockerfiles, application source, container runtime behavior, or Compose deployment configuration.
- Publishing images for unrelated packages or services.
- Rebuilding images solely because a semantic Git tag is created.
- Retaining or replacing `validate-app-image-builds.yml` with another pull-request image-validation workflow.
- Changing the visibility, ownership, or lifecycle policies of the approved Docker Hub repositories.

## Assumptions and Dependencies

- The approved Docker Hub repositories remain public and are owned by `doitsu2014`.
- GitHub Actions has access to valid Docker Hub publishing credentials through repository secrets; those credentials are never written to tracked workflow or script files.
- A change to `packages/editor-prose` is relevant to both frontend images; API changes are independent of that package.

## Risks

- Incorrect path filters can leave an image stale or trigger unnecessary publishing; publisher definitions must be covered by focused local workflow-contract verification.
- A mutable current-image tag can be overwritten; source-derived tags must provide a stable audit and rollback reference.
- Docker Hub or base-image availability can prevent a release build even when the application source is valid.

## Success Criteria

- A relevant API, admin, or Ducth website change merged to `main` publishes exactly that application's image to its approved Docker Hub repository.
- A shared editor-package change publishes refreshed admin and Ducth images without publishing the API image.
- No dedicated pull-request application-image validation workflow remains in `.github/workflows/`.
- Every published image can be associated with the source revision that produced it, and the current main image remains available through the established convention.
