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

### Requirement: Dedicated pull-request image validation SHALL be removed and publisher contracts SHALL remain locally verifiable

The system SHALL remove `.github/workflows/validate-app-image-builds.yml` and SHALL NOT introduce a replacement pull-request application-image validation workflow in this change. The three publishing workflows SHALL NOT declare a `pull_request` trigger. A repository-local workflow-contract verification SHALL validate the approved image repositories, main-only path filters, absence of semantic-version tag triggers, SHA/main-`latest` tag policy, and the API and frontend bounded build-context declarations without authenticating to Docker Hub or pushing an image.

#### Scenario: Dedicated validation workflow is absent
- **WHEN** the repository workflows are inspected after this change
- **THEN** `.github/workflows/validate-app-image-builds.yml` does not exist
- **AND** no API, admin, or Ducth publishing workflow is triggered by a pull request

#### Scenario: Publisher contract fails safely before a registry write
- **WHEN** a publisher workflow has an incorrect repository, trigger, tag policy, or frontend build context
- **THEN** the local workflow-contract verification exits non-zero
- **AND** it performs no Docker Hub login or image publication

### Requirement: Docker Hub publishing credentials SHALL be isolated from tracked configuration

The system SHALL obtain Docker Hub publishing authentication only from GitHub Actions repository secrets in a publisher job that is selected for `main` or an intentional manual rebuild. Publisher jobs SHALL use credentials authorized only for the three approved repositories. Workflow files, local contract checks, logs, and image metadata SHALL NOT contain credential values.

#### Scenario: Main publisher authenticates without exposing credentials
- **WHEN** a selected main-branch publisher starts
- **THEN** it authenticates to Docker Hub using configured repository secrets before pushing its approved image
- **AND** the credential values are not written to the repository or exposed in workflow output

#### Scenario: Local contract verification has no publishing credential path
- **WHEN** the repository-local workflow-contract verification runs
- **THEN** it has no Docker Hub login step and does not require Docker Hub publishing secrets
- **AND** it can succeed or fail solely on static workflow-contract results
