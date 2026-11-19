# Code Structure

## Repository Organization

| Area | Role | Analysis depth |
|---|---|---|
| `apps/api/gateway/` | Rust API composition root, server startup, CLI entry points | Gateway entry and backup/migration CLI examined |
| `apps/api/domain_interface/` | Shared domain contracts and types | Source tree inventoried; representative layouts examined |
| `apps/api/domain_posts/` | Content, translation, GraphQL, persistence, owned migrations | Tree inventoried; selected representative inspection |
| `apps/api/domain_auth/` | Authentication and authorization integration | Tree inventoried; selected representative inspection |
| `apps/api/domain_media/` | Media and storage integration | Tree inventoried; selected representative inspection |
| `apps/api/domain_user/` | User administration integration | Tree inventoried; selected representative inspection |
| `apps/api/domain_seo/` | SEO domain | Tree inventoried; selected representative inspection |
| `apps/api/test_helpers/` | Shared backend testing support | Tree inventoried; representative layout examined |
| `apps/web/` | Admin React/TypeScript app | Source skimmed/inventoried, not every component read |
| `apps/ducth-dev-website/` | Public React/TypeScript reader with Express server | Source skimmed/inventoried |
| `packages/editor-prose/` | Shared article prose rendering package | Package and source/test layout skimmed |
| `deployments/` | Docker Swarm and Kubernetes deployment material | Directory inventory only; no deployment executed |

## Build and Workspace Layout
The backend is a Cargo workspace rooted at `apps/api/Cargo.toml`. The frontend uses pnpm workspaces and per-package manifests/configuration. Domain crate boundaries and package membership were examined; no build command was run.

## Source Classification
- **Configuration/build**: Cargo and package manifests, TypeScript/build/lint configuration, workflow YAML, deployment manifests.
- **Controller/handler and composition**: gateway startup and domain route/handler modules.
- **Service/use case and domain**: domain crate services and business logic.
- **Model/entity and persistence**: domain-owned entities and migrations, especially in `domain_posts`.
- **Utility/integration**: shared interfaces, storage/auth adapters, test helpers.
- **UI/static**: React app components, styles, public assets, and shared prose package.
- **Test**: Rust tests and frontend Vitest/Testing Library test areas.

This classification is architectural and representative, not a complete per-file catalogue.

## Backup/Restore Change Surface
Intent-relevant paths include `apps/api/gateway/src/main.rs`, `apps/api/gateway/src/backup_cli.rs`, gateway Cargo dependencies/lockfile, and `docs/features/backup-restore.md`. The current documentation characterizes the CLI as a skeleton and the feature as a target specification. Do not infer complete API/UI/job wiring from these paths alone.

## Coverage Limitations
The run was mixed-depth. `apps/web/src/`, public-reader source, and the editor package were skimmed; deployment/config and remaining documentation were inventoried at directory level. Generated/dependency/cache folders and AI-DLC internals were excluded. Exact counts and full dependency adjacency for every source file are not claimed.
