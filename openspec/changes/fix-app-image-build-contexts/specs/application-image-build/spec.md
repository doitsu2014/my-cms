## ADDED Requirements

### Requirement: Application image build inputs SHALL remain bounded

The system SHALL build the API, admin web, and Ducth website images with each application's existing app-scoped primary Docker build context. The API build context SHALL exclude the local `target/` directory and other non-runtime local artifacts so Cargo build outputs are created inside the image build rather than transferred from the developer workstation. The system SHALL NOT use the repository root as any application's primary image-build context.

#### Scenario: API build excludes local Cargo outputs
- **WHEN** the API image is built from `apps/api` after a local Cargo build has created `apps/api/target/`
- **THEN** Docker receives no files from `apps/api/target/` in the primary build context
- **AND** the final image contains the release `my-cms-api` binary built by its Docker build stage

#### Scenario: Frontend primary contexts stay application-scoped
- **WHEN** a supported admin or Ducth website image-build entry point is inspected or run
- **THEN** its primary Docker context is respectively `apps/web` or `apps/ducth-dev-website`
- **AND** the repository root is not used as that build's primary context

### Requirement: Frontend images SHALL receive the shared editor package through a named bounded context

The admin web and Ducth website image-build contracts SHALL provide `packages/editor-prose` as a named BuildKit context called `editor-prose`, separate from each application's primary context. Each frontend Docker build SHALL make that package available at the path required by its existing `file:../../packages/editor-prose` dependency before dependency installation and during application compilation. The package context SHALL exclude dependency installations, build outputs, credentials, and other non-source local artifacts. Both frontend dependency installs in container builds SHALL use their committed lockfiles with frozen-lockfile semantics.

#### Scenario: Admin image resolves the local editor dependency deterministically
- **WHEN** the admin image is built with `editor-prose=packages/editor-prose` as its named context
- **THEN** dependency installation succeeds using `apps/web/pnpm-lock.yaml` with frozen-lockfile semantics
- **AND** the admin build completes without changing the lockfile or downloading a replacement for the local `editor-prose` package

#### Scenario: Ducth website image resolves the local editor dependency
- **WHEN** the Ducth website image is built with `editor-prose=packages/editor-prose` as its named context
- **THEN** the `file:../../packages/editor-prose` dependency resolves during dependency installation and compilation
- **AND** the resulting image retains its existing non-root runtime user and healthcheck contract

#### Scenario: Missing shared package context fails at dependency resolution
- **WHEN** either frontend image is built without the required `editor-prose` named context
- **THEN** the build exits non-zero before producing a successful application image
- **AND** the failure identifies the unavailable shared-package input rather than silently using an unrelated dependency source

### Requirement: Supported image-build entry points SHALL provide identical inputs

The apps Compose configuration SHALL provide the `editor-prose` named context to both frontend services while retaining their application-scoped primary contexts. The admin image release workflow SHALL provide the same named context to its Docker build action and SHALL run when files under either `apps/web/**` or `packages/editor-prose/**` change. The API image build entry points SHALL retain the `apps/api` primary context and benefit from its context exclusions.

#### Scenario: All-app Compose build has complete contexts
- **WHEN** `docker compose -f deployments/docker-swarm/apps/docker-compose.yaml build` is run from its supported repository invocation
- **THEN** the API, admin web, and Ducth website image builds receive all required build inputs
- **AND** both frontend services receive only their app primary context plus the named `editor-prose` context

#### Scenario: Admin release build includes shared package changes
- **WHEN** a change under `packages/editor-prose/**` is pushed to a configured release-workflow branch
- **THEN** the admin image release workflow is selected by its path filters
- **AND** its Docker build action supplies `editor-prose=./packages/editor-prose` as a build context

### Requirement: Continuous integration SHALL validate all application image builds without publishing images

The system SHALL run an automated, non-publishing image-build validation that builds the API, admin web, and Ducth website images using the same bounded contexts required by their supported entry points. The validation SHALL fail when a Dockerfile, ignore file, Compose configuration, shared package, or build-context wiring prevents any image from building. The validation SHALL run before a regression can be released and SHALL not require application secrets or a deployed stack.

#### Scenario: Image-build regression is detected before release
- **WHEN** a change breaks an application's Docker context, Dockerfile dependency input, or shared-package context wiring
- **THEN** the automated image-build validation fails for the affected application
- **AND** no image is published by that validation job

#### Scenario: Healthy repository passes image-build validation
- **WHEN** the API, admin web, and Ducth website Dockerfiles and declared build contexts are valid
- **THEN** the automated validation completes three successful image builds
- **AND** the job output identifies the failing image build if a later run fails
