# Component Inventory

## gateway
- **Type:** Rust application composition root/API binary.
- **Responsibilities:** Compose domain services/routes, common configuration/middleware, listeners, and CLI pathways.
- **Dependencies:** `domain_interface` and domain crates; exact runtime wiring beyond inspected entry paths is not exhaustively characterized.
- **Evidence:** `apps/api/gateway/` manifest and selected entry/CLI source examined.

## domain_interface
- **Type:** Rust shared contract crate.
- **Responsibilities:** Shared service interfaces and domain/API types.
- **Dependencies:** Shared Rust ecosystem; consumers include gateway and domain crates.
- **Evidence:** Workspace boundaries and source tree inventoried.

## domain_posts
- **Type:** Rust domain crate.
- **Responsibilities:** Posts, categories, tags, translations/jobs, GraphQL contribution, persistence entities and owned migrations.
- **Dependencies:** `domain_interface`, PostgreSQL/SeaORM ecosystem, translation integrations as configured.
- **Evidence:** Source tree inventoried and representative structure examined; exact behavior may require deeper validation.

## domain_auth
- **Type:** Rust domain/integration crate.
- **Responsibilities:** Authentication configuration and JWT/role middleware integration.
- **Dependencies:** `domain_interface` and identity/token ecosystem.
- **Evidence:** Manifest and source tree inventoried; exact authorization behavior not fully validated.

## domain_media
- **Type:** Rust domain crate.
- **Responsibilities:** Media/bucket APIs and Supabase Storage integration.
- **Dependencies:** `domain_interface`, storage integration.
- **Evidence:** Source tree inventoried; detailed endpoint semantics not fully validated.

## domain_user
- **Type:** Rust domain crate.
- **Responsibilities:** Administrator user operations through Supabase/GoTrue integration.
- **Dependencies:** `domain_interface`, GoTrue integration.
- **Evidence:** Source tree inventoried; detailed route/validation behavior not fully validated.

## domain_seo
- **Type:** Rust domain crate.
- **Responsibilities:** SEO head-asset persistence, routes, and public delivery integration.
- **Dependencies:** `domain_interface` and persistence layer.
- **Evidence:** Source tree inventoried; detailed runtime behavior not fully validated.

## test_helpers
- **Type:** Rust test-support crate.
- **Responsibilities:** Reusable backend test utilities.
- **Dependencies:** Backend crates/test ecosystem.
- **Evidence:** Workspace membership and source tree inventoried.

## admin_side
- **Type:** React/TypeScript administrative web application (`apps/web`).
- **Responsibilities:** CMS administration and editing UI.
- **Dependencies:** React, React Router, Apollo Client, Supabase JS, shared `editor-prose` package.
- **Evidence:** Manifest and configuration examined; app source skimmed.

## apps/ducth-dev-website
- **Type:** React/TypeScript public reader with Express server.
- **Responsibilities:** Localized public rendering and server-side support.
- **Dependencies:** React, React Router, Apollo Client, Express, shared `editor-prose` package.
- **Evidence:** Manifest and package layout examined; app source skimmed.

## packages/editor-prose
- **Type:** Shared frontend library.
- **Responsibilities:** Shared saved-article prose rendering consumed by admin and public reader.
- **Dependencies:** Frontend tooling and rendering libraries.
- **Evidence:** Package manifest and source/test layout skimmed; behavior not exhaustively verified.

## deployments/docker-swarm
- **Type:** Deployment/configuration package.
- **Responsibilities:** Docker-based local/deployment configuration.
- **Dependencies:** Docker and configured services.
- **Evidence:** Directory-level inventory; no deployment executed.

## deployments/k8s
- **Type:** Deployment/configuration package.
- **Responsibilities:** Kubernetes manifests/configuration.
- **Dependencies:** Kubernetes tooling and configured services.
- **Evidence:** Directory-level inventory; no deployment executed.
