## Why

Component-scoped `main` publishing keeps the current Docker Hub images fresh, but it does not give a manually announced product release a stable, documented set of container artifacts. A published GitHub Release must therefore create release-versioned images and record the exact immutable artifacts without changing the established current-main publishing policy.

## What Changes

- Retain automated Docker Hub publishing for the API, admin, and Ducth development website images from `main` only when their relevant application, shared-package, or publisher-workflow inputs change.
- Retain the approved public repositories: `doitsu2014/my-cms`, `doitsu2014/my-cms-admin`, and `doitsu2014/my-cms-ducth-dev-website`.
- Add a separate release-publication path: publishing a manually created GitHub Release with a `vX.Y.Z` version triggers a build of the release revision and publishes a corresponding immutable version tag for each approved component image.
- Preserve source-revision SHA tags for every publication. A release-version tag identifies the release image alongside its source SHA; it is never reused or overwritten for a different image.
- Document the repository, release-version tag, source revision, and resulting image digest for every successful component release build in the associated GitHub Release notes, giving operators a release-level image manifest.
- Keep `latest` as the current-main convenience tag only. A GitHub Release build must not create, move, or otherwise alter `latest`.
- Retain application- and shared-package-aware path filters for `main` publication, delete the dedicated non-publishing application-image validation workflow, and retain safe local workflow-contract verification.

## Capabilities

### New Capabilities

- `docker-hub-image-publishing`: Component-scoped main publishing, release-versioned Docker Hub image publication with GitHub Release image-manifest evidence, and local workflow-contract verification for the API, admin, and Ducth development website images.

### Modified Capabilities

- None.

## Impact

- Affected systems: GitHub Actions publisher workflows under `.github/workflows/`, GitHub Releases, Docker Hub, and the local workflow-contract verification that protects their publishing policy.
- External dependencies: Docker Hub credentials configured as GitHub Actions secrets, plus GitHub Actions permission to read the published release and record its image-manifest evidence in the release notes.
- Build inputs: API files; admin and Ducth website files; and `packages/editor-prose` for both frontend images. A release build uses the source revision identified by its GitHub Release.
- No application behavior, API contract, database schema, image deployment, runtime configuration, Dockerfile build contract, or Docker Hub repository visibility is changed.

## Scope

In scope is:

- Building and pushing the three existing application images to their approved public Docker Hub repositories after relevant changes reach `main`, with source-revision SHA tags and the existing current-main `latest` convention.
- When a maintainer manually publishes a GitHub Release whose version is in the `vX.Y.Z` form, building the release revision for all three approved application images and publishing one exact release-version tag per image.
- Treating each release-version tag as immutable, so an existing version cannot be repointed to a different image; a failed or incomplete release must be corrected with a new release version rather than overwriting a published version tag.
- Recording per-image repository, release-version tag, source revision, and digest in that GitHub Release's notes after successful publication. If the release cannot produce a complete image set, the release evidence must clearly identify the failed or unavailable component rather than imply a complete release.
- Deleting the dedicated application-image validation workflow, retaining local workflow-contract verification, and preserving intentional manual workflow dispatch for an exceptional rebuild.

## Non-goals

- Automatically creating GitHub Releases for every merge to `main`.
- Rebuilding images solely because a semantic Git tag is pushed, without a qualifying published GitHub Release.
- Moving `latest`, promoting images, deploying, or rolling back any environment as a result of a GitHub Release.
- Changing Dockerfiles, application source, container runtime behavior, Compose deployment configuration, image build contexts, or the approved image repositories.
- Publishing images for unrelated packages or services, changing Docker Hub repository visibility or ownership, or retaining/replacing `validate-app-image-builds.yml` with another pull-request image-validation workflow.

## Assumptions and Dependencies

- Release maintainers publish only a valid `vX.Y.Z` GitHub Release once its referenced source revision is ready to represent the API, admin, and Ducth images.
- The approved Docker Hub repositories remain public and are owned by `doitsu2014`.
- GitHub Actions has access to valid Docker Hub publishing credentials through repository secrets and sufficient repository authority to associate release image evidence with the published GitHub Release; credentials are never written to tracked workflow or script files.
- A change to `packages/editor-prose` is relevant to both frontend images; API changes are independent of that package.

## Risks

- Incorrect path filters can leave a current-main image stale or trigger unnecessary `main` publishing; publisher definitions must remain covered by focused local workflow-contract verification.
- A malformed, reused, or incorrectly targeted release version can make the release manifest misleading; version-format validation, immutable version policy, and source-revision evidence reduce that risk.
- Docker Hub, GitHub Release, or base-image availability can prevent one or more release images from being published; release notes must distinguish incomplete publication from a complete releasable image set.
- `latest` is mutable; it remains an operational convenience, while SHA tags, release-version tags, and digests provide the stable release and rollback references.

## Success Criteria

- A relevant API, admin, or Ducth website change merged to `main` publishes exactly that application's image to its approved Docker Hub repository; a shared editor-package change republishes both frontend images without publishing the API image.
- Every `main` publication has a source-revision SHA tag, and the established `latest` tag continues to represent only the most recent successful selected `main` build.
- Publishing a manually created `vX.Y.Z` GitHub Release produces versioned images for the API, admin, and Ducth repositories from the release revision, with each version tag permanently associated with its source revision and image digest.
- The associated GitHub Release notes provide a complete per-image manifest of repository, version tag, source revision, and digest, or explicitly identify an incomplete component publication.
- A release publication neither creates nor moves `latest`, and no deployment, promotion, or runtime-environment change occurs.
- No dedicated pull-request application-image validation workflow remains in `.github/workflows/`.
