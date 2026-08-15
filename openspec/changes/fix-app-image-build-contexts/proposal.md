## Why

The API image build currently transfers local Rust build artifacts in its Docker context, making builds disproportionately slow and resource-intensive. The admin and Ducth website images cannot be built from their declared app contexts because both depend on the workspace-local `editor-prose` package, which is outside those contexts. This blocks reliable local Compose builds and the admin image release workflow.

## What Changes

- Establish a reproducible image-build contract for the API, admin, and Ducth website applications so each declared image can be built successfully from its supported entry points.
- Exclude local, non-runtime build artifacts from the API image context so they are not uploaded or considered during image construction.
- Make the shared `editor-prose` package available to the admin and Ducth website image builds through an explicitly bounded shared-package build context, while retaining each application's scoped primary context.
- Align the local Compose configuration and the admin image release workflow with the shared-package build requirement.
- Add automated image-build coverage for all three applications to detect broken Docker contexts before release.
- Update the Ducth website deployment contract to describe the required shared-package build context and its supported build invocation.

## Capabilities

### New Capabilities

- `application-image-build`: Reproducible, bounded image-build behavior and automated build validation for the API and admin applications, including their supported local and release entry points.

### Modified Capabilities

- `website-deployment`: The Ducth website image and Compose service build contract will require access to the shared `editor-prose` package through a bounded build context and will define a build invocation that supplies it.

## Impact

- Affected systems: API, admin web application, Ducth website, apps Compose stack, and the admin image release workflow.
- Affected build inputs: Docker context exclusions and the workspace-local `packages/editor-prose` package.
- No user-facing CMS workflow, API contract, database schema, migration, authentication behavior, runtime environment variable contract, or production deployment is changed.

## Scope

In scope is making the three existing application images buildable and verifying that their supported build entry points continue to produce runnable artifacts. The selected constraint is to keep primary build contexts app-scoped and provide only the shared editor package as a bounded additional context.

## Non-goals

- Converting the repository to a monorepo-wide Docker build context.
- Publishing `editor-prose` to an external package registry or changing its local `file:` dependency contract.
- Changing application features, network routing, health endpoints, image base platforms, or release/deployment policy.
- Deploying images to any environment as part of this change.

## Assumptions and Dependencies

- Docker BuildKit and the project’s Compose/release tooling support named additional build contexts.
- `packages/editor-prose` remains the canonical shared package for both frontend applications.
- CI can run non-publishing builds for the API, admin, and Ducth website images.

## Risks

- Local and CI build entry points can drift if the shared context is configured in only one place.
- Broadening a build context could reintroduce slow builds or accidentally include local artifacts or sensitive files; validation must assert the bounded-context contract.
- Container builds depend on base-image and package-registry availability during verification.

## Success Criteria

- The API image build no longer transfers local `target` artifacts in its build context.
- The admin and Ducth website images resolve the existing local `editor-prose` dependency and build successfully using their documented entry points.
- The all-app Compose build succeeds without widening any primary application context to the repository root.
- Automated validation covers all three application images before a release workflow can regress.
