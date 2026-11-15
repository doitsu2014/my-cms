# Component Inventory

## `gateway`
- **Type:** API composition-root binary (Rust)
- **Responsibility:** Builds the sole shipped `my-cms-api`; merges domain routes and applies shared concerns.
- **Dependencies:** `domain_interface`, `domain_posts`, `domain_auth`, `domain_media`, `domain_user`, `domain_seo`.

## `domain_interface`
- **Type:** Rust library/contract
- **Responsibility:** Defines `DomainService`, shared context, route/mount and migration descriptors, actor identity, and errors.
- **Dependencies:** Shared contract consumed by the gateway and domain crates.

## `domain_posts`
- **Type:** Rust domain library
- **Responsibility:** Posts, categories, tags, translations, GraphQL integration, persistence entities, and canonical post migrations.
- **Dependencies:** `domain_interface`, PostgreSQL/SeaORM, OpenAI integrations, and API framework components as defined in its manifest.

## `domain_auth`
- **Type:** Rust authentication/domain infrastructure library
- **Responsibility:** Supabase JWT validation and role middleware; no domain-owned routes or migrations evident in its service implementation.
- **Dependencies:** Supabase identity/JWT integration and `domain_interface` contract. Its `src/main.rs` stub is not a deployment surface.

## `domain_media`
- **Type:** Rust domain library
- **Responsibility:** Media and bucket APIs, Supabase Storage adapter, configuration, and caches.
- **Dependencies:** Supabase Storage and `domain_interface` contract.

## `domain_user`
- **Type:** Rust domain library
- **Responsibility:** Administrator user operations and Supabase GoTrue admin adapter.
- **Dependencies:** Supabase GoTrue/Admin API and `domain_interface` contract.

## `domain_seo`
- **Type:** Rust domain library
- **Responsibility:** Validated SEO head-asset storage, administrator CRUD, public delivery, and its migration.
- **Dependencies:** Persistence and `domain_interface` contract.

## `test_helpers`
- **Type:** Rust test-support library
- **Responsibility:** Test support present in the API workspace; detailed source not deeply analyzed.
- **Dependencies:** Not fully inventoried.

## `admin_side`
- **Type:** TypeScript/React frontend application (`apps/web/`)
- **Responsibility:** CMS administration and authoring, including TipTap editor integration.
- **Dependencies:** GraphQL/Apollo client, React, and local `editor-prose` package.

## `ducth-dev-website`
- **Type:** TypeScript/React frontend and SSR application (`apps/ducth-dev-website/`)
- **Responsibility:** Localized public reader, rendered through React with a Node/Express production server.
- **Dependencies:** API/content surfaces, React Router, i18next, and local `editor-prose` package.

## `editor-prose`
- **Type:** Shared TypeScript/React library (`packages/editor-prose/`)
- **Responsibility:** Common article-prose styles/integration and Mermaid rendering used by both frontends.
- **Dependencies:** Mermaid and React peer dependency (`>=18`).

## Deployment and Automation Components

- **`docker-swarm`** — Compose/Swarm configuration for application deployment.
- **`k8s`** — Kubernetes/Helm deployment chart and manifests.
- **GitHub Actions workflows** — CI and image/release publishing. Workflow execution was not validated.

Component names above are the exact headings used by the timestamp scope block. Inventory is bounded by the scan; some dependencies and implementation details are not exhaustive.