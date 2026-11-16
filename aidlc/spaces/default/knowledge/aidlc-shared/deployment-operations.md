# Deployment and operations

[Glossary](glossary.md) · Local-operation references: [`README.md`](../../../../../README.md), [`deployments/docker-swarm/README.md`](../../../../../deployments/docker-swarm/README.md)

## Evidence boundary

The available static scan inventoried `deployments/docker-swarm/`, `deployments/k8s/`, selected Dockerfiles, GitHub workflows, and gateway entry points but did not execute deployments. Exact runtime and release behavior below is **Unverified** unless stated as a maintained-documentation instruction; verify current manifests/workflows before operational use.

## Local environment

Maintained setup documentation describes three Compose stacks under `deployments/docker-swarm/`: Supabase, applications, and Traefik, joined by external `supabase_network`. Bootstrap precedes reset scripts; per-stack environment files own configuration, with shared database/JWT/anon/service-role values kept consistent where required. Reset scripts can delete local volumes and must be treated as destructive. Documented host ports include API 8989, admin 3002, reader 3001, Jaeger 16686, and Traefik 8080; validate against current Compose before use. Retired root Compose filenames and root env files must not be revived.

## Health and telemetry

Project knowledge specifies website `GET /healthz` as the container health probe, not `GET /en`; described health behavior is limited to Express accepting connections. Website OTLP is intended to be opt-in and must not expose telemetry configuration in browser bundles or alter normal behavior when disabled/invalid. Gateway OTLP is described in [backend-domain-architecture](backend-domain-architecture.md). Health implementation, readiness semantics, trace coverage and privacy filters are **Unverified** by the partial scan.

## Build and publishing contracts

The intended primary image contexts are app-scoped (`apps/api`, `apps/web`, `apps/ducth-dev-website`); `packages/editor-prose` is supplied as a named BuildKit context to frontend builds. Existing knowledge records Docker Hub main-branch publishers and versioned GitHub Release publishing for API/admin/reader images, including tag and digest manifest behavior. These details, workflow filters, credentials, and failure behavior are **Unverified** until the current workflow files are inspected.

Do not add the retired `.github/workflows/validate-app-image-builds.yml` pull-request image workflow; the project contract specifies a local workflow-contract check instead. Do not infer deployments from image publishing: the release workflow is not a deployment operation.

## Open operational evidence

- **Unverified:** current Compose ports, healthcheck configuration, OTLP safeguards, image contexts, publisher triggers/tags, and release failure paths.
- Before deployment or release, inspect the source configuration and document rollback and smoke-test procedures in the operational change. No deployment, build, test, or runtime verification was performed for this documentation rewrite.
