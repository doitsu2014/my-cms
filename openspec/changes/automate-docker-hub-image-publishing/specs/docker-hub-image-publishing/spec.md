## ADDED Requirements

### Requirement: Main-branch image publishing SHALL select only relevant application inputs

The system SHALL publish each application image only for a `push` to `main` that changes that image's declared build inputs or its own publishing-workflow definition. The API publisher SHALL be selected by changes under `apps/api/**` or `.github/workflows/release-my-cms-image.yml`. The admin publisher SHALL be selected by changes under `apps/web/**`, `packages/editor-prose/**`, or `.github/workflows/release-my-cms-admin-image.yml`. The Ducth publisher SHALL be selected by changes under `apps/ducth-dev-website/**`, `packages/editor-prose/**`, or its publishing-workflow definition. The publishing workflows SHALL NOT be triggered by semantic-version tag pushes.

#### Scenario: API-only source change publishes only the API image
- **WHEN** a commit that changes only a file under `apps/api/**` is pushed to `main`
- **THEN** the API publishing workflow builds and publishes the API image
- **AND** the admin and Ducth publishing workflows do not publish images for that commit

#### Scenario: Shared editor package change publishes both frontend images
- **WHEN** a commit that changes only a file under `packages/editor-prose/**` is pushed to `main`
- **THEN** the admin and Ducth publishing workflows each build and publish their image
- **AND** the API publishing workflow does not publish an image for that commit

#### Scenario: Unrelated repository change does not publish an application image
- **WHEN** a commit that changes none of the selected publishing inputs is pushed to `main`
- **THEN** no application image publishing workflow is selected by that commit
- **AND** no Docker Hub image tag is created or overwritten by the commit

### Requirement: Each selected publisher SHALL build and publish its approved public image with traceable tags

The system SHALL publish the API image to `doitsu2014/my-cms`, the admin image to `doitsu2014/my-cms-admin`, and the Ducth image to `doitsu2014/my-cms-ducth-dev-website`. Every published image SHALL receive a source-revision SHA tag. A successful `main` publishing run SHALL also update that image's `latest` tag as the explicit current-main convenience tag. The publishing workflow SHALL preserve each image's supported build context contract: `apps/api` for API; `apps/web` plus named `editor-prose=packages/editor-prose` for admin; and `apps/ducth-dev-website` plus the same named context for Ducth.

#### Scenario: Published API image identifies its source revision
- **WHEN** the API publishing workflow completes successfully for a selected `main` commit
- **THEN** Docker Hub contains a new `doitsu2014/my-cms` image tagged with that commit's SHA identifier
- **AND** the image is built from the API's app-scoped context and `latest` identifies the same successful main-branch build

#### Scenario: Published frontend images receive their shared package input
- **WHEN** either frontend publishing workflow completes successfully for a selected `main` commit
- **THEN** its image is published to its approved repository with a source-revision SHA tag and `latest`
- **AND** the build receives `packages/editor-prose` only through the named `editor-prose` context required by the existing Dockerfile contract

#### Scenario: Ducth image is published to its approved repository
- **WHEN** a selected Ducth application or shared editor-package change is pushed to `main`
- **THEN** the Ducth publishing workflow builds and pushes `doitsu2014/my-cms-ducth-dev-website`
- **AND** no deployment, Compose runtime configuration, or application source behavior is changed by the publishing run

### Requirement: Dedicated pull-request image validation SHALL be removed and publication contracts SHALL remain locally verifiable

The system SHALL remove `.github/workflows/validate-app-image-builds.yml` and SHALL NOT introduce a replacement pull-request application-image validation workflow in this change. The three main publishers and the release publisher SHALL NOT declare a `pull_request` trigger. A repository-local workflow-contract verification SHALL validate the approved image repositories, the main publishers' path filters and absence of semantic-version tag triggers, the SHA/main-`latest` policy, the release publisher's `release.published` trigger, exact-version validation, no-`latest` policy, manifest-update contract, and the API and frontend bounded build-context declarations without authenticating to Docker Hub, calling GitHub APIs, or pushing an image.

#### Scenario: Dedicated validation workflow is absent
- **WHEN** the repository workflows are inspected after this change
- **THEN** `.github/workflows/validate-app-image-builds.yml` does not exist
- **AND** no API, admin, or Ducth publishing workflow is triggered by a pull request

#### Scenario: Publication contract fails safely before an external write
- **WHEN** a main or release publisher workflow has an incorrect repository, trigger, tag policy, release-manifest policy, or frontend build context
- **THEN** the local workflow-contract verification exits non-zero
- **AND** it performs no Docker Hub login, GitHub Release update, or image publication

### Requirement: A published valid GitHub Release SHALL publish a complete versioned image set from its release ref

The system SHALL implement one release-publication workflow that triggers only for the GitHub `release` event type `published`. Before authenticating to Docker Hub or changing release notes, the workflow SHALL validate that the published release tag exactly matches `vX.Y.Z`, where each version component is one or more decimal digits. For a valid release, the workflow SHALL check out the release tag/ref with complete Git history and SHALL build all three approved component images from that same checked-out release revision: API from `apps/api`; admin from `apps/web` plus named `editor-prose=./packages/editor-prose`; and Ducth from `apps/ducth-dev-website` plus the same named context. Each image SHALL receive both the exact release-version tag and a source-revision SHA tag. The release workflow SHALL explicitly disable metadata-generated `latest` and SHALL NOT create, move, or otherwise alter `latest`.

#### Scenario: Valid published release produces three release-versioned images
- **WHEN** a maintainer publishes a GitHub Release tagged `v1.2.3`
- **THEN** the release-publication workflow checks out the `v1.2.3` release ref and builds API, admin, and Ducth images from that revision
- **AND** Docker Hub receives `doitsu2014/my-cms:v1.2.3`, `doitsu2014/my-cms-admin:v1.2.3`, and `doitsu2014/my-cms-ducth-dev-website:v1.2.3` plus source-revision SHA tags
- **AND** the run does not create or update any `latest` tag

#### Scenario: Invalid published release version has no external publication side effect
- **WHEN** a GitHub Release with a tag that does not exactly match `vX.Y.Z` is published
- **THEN** the validation job fails visibly before Docker Hub authentication, image push, or GitHub Release-note update
- **AND** no version, SHA, or `latest` image tag is created or changed by that run

### Requirement: Release-version tags SHALL be treated as immutable publication identities

For a valid release, the release-publication workflow SHALL inspect all three approved repositories for the exact release-version tag before it builds or pushes any image. If any repository already contains that release-version tag, the workflow SHALL fail without pushing any release image or changing its existing tag. The workflow SHALL use a release-version-specific concurrency group with `cancel-in-progress: false` so retries and duplicate delivery cannot race within this workflow; a failed or incomplete version SHALL be corrected by publishing a new release version, not by overwriting a version tag.

#### Scenario: Reused release version is rejected before publication
- **WHEN** a `v1.2.3` release is published and any approved Docker Hub repository already contains the `v1.2.3` tag
- **THEN** the release-publication workflow fails before it builds or pushes a release-version tag to any approved repository
- **AND** the existing tag remains unchanged and the release is not represented as a complete image publication

#### Scenario: Retry does not cancel an in-progress release publication
- **WHEN** a retry or duplicate event for the same release version starts while its release-publication workflow is still running
- **THEN** the workflow serializes the attempts in the release-version-specific concurrency group without cancelling the in-progress publication
- **AND** an attempt that observes existing release-version tags fails rather than repointing them

### Requirement: GitHub Release notes SHALL provide an idempotent release image manifest

After all three component builds and pushes succeed, the release-publication workflow SHALL upsert one clearly delimited image-manifest block in the associated GitHub Release notes. The block SHALL identify the release version, a complete status, and for every component its Docker Hub repository, exact release-version tag, checked-out source revision SHA, and Buildx-produced image digest. A rerun SHALL replace that workflow-owned block instead of duplicating it. If validation, immutability preflight, or any component build/push fails, the workflow SHALL finish failed; it SHALL NOT write or retain a manifest that claims a complete image set. When one or more component builds have started or succeeded before such a failure, the workflow SHALL upsert an explicitly incomplete block identifying successful artifacts with their available evidence and each failed or unavailable component, then fail the run.

#### Scenario: Complete release manifest is recorded after all image pushes
- **WHEN** all three builds and pushes for a valid release succeed
- **THEN** the associated GitHub Release notes contain exactly one workflow-owned complete manifest block
- **AND** that block lists each repository, its exact release-version tag, the common release-ref source SHA, and that Buildx step's digest

#### Scenario: Partial publication cannot be mistaken for a complete release
- **WHEN** one component build or push fails after another component has succeeded
- **THEN** the workflow records an explicitly incomplete release-manifest block that names the failed or unavailable component and preserves available successful-artifact evidence
- **AND** the workflow run fails and the release notes contain no complete-manifest claim

### Requirement: Release publication authorization SHALL be limited to required registry and release-note writes

The release-publication workflow SHALL declare `contents: read` and `contents: write` permissions only to read the release ref and update the associated GitHub Release notes. It SHALL use the existing Docker Hub repository secrets only for Docker Hub login, SHALL NOT add credentials, and SHALL NOT print secret values. The workflow SHALL use the GitHub-provided token only for the manifest upsert and SHALL NOT deploy, promote, mutate runtime configuration, or invoke any environment-changing deployment action.

#### Scenario: Release workflow has least required GitHub and registry authority
- **WHEN** the release-publication workflow is inspected or runs for a valid release
- **THEN** its GitHub token has only the declared contents read/write authority needed for checkout and release-note mutation
- **AND** Docker Hub credentials are sourced only from the existing repository secrets and are not emitted in logs, metadata, or the manifest

### Requirement: Docker Hub publishing credentials SHALL be isolated from tracked configuration

The system SHALL obtain Docker Hub publishing authentication only from GitHub Actions repository secrets in a publisher job selected for `main`, an intentional manual rebuild, or a validated published GitHub Release. Publisher jobs SHALL use credentials authorized only for the three approved repositories. Workflow files, local contract checks, logs, and image metadata SHALL NOT contain credential values.

#### Scenario: Main publisher authenticates without exposing credentials
- **WHEN** a selected main-branch publisher starts
- **THEN** it authenticates to Docker Hub using configured repository secrets before pushing its approved image
- **AND** the credential values are not written to the repository or exposed in workflow output

#### Scenario: Local contract verification has no publishing credential path
- **WHEN** the repository-local workflow-contract verification runs
- **THEN** it has no Docker Hub login step and does not require Docker Hub publishing secrets
- **AND** it can succeed or fail solely on static workflow-contract results
