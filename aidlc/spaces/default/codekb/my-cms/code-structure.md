# Code Structure

## Repository Areas

| Area | Role and observed organization |
|---|---|
| `apps/api/` | Cargo workspace. `gateway` is the application composition root; `domain_interface` defines shared contracts; `domain_posts`, `domain_auth`, `domain_media`, `domain_user`, and `domain_seo` provide domain capabilities; `test_helpers` supports tests. |
| `apps/web/` | TypeScript/React administrator interface, including authoring and TipTap editor integration. |
| `apps/ducth-dev-website/` | TypeScript/React public reader with Node/Express SSR and localized route composition. |
| `packages/editor-prose/` | Shared TypeScript/React article-prose integration, styles, Mermaid lifecycle, and tests. |
| `deployments/docker-swarm/` | Docker Compose/Swarm configuration and README. |
| `deployments/k8s/` | Kubernetes manifests/charts, including Helm chart inventory. |
| `.github/workflows/` | CI and image/release publishing workflows. |

## Classification and Patterns

- API: domain crates combine service/domain operations, handlers/routes, adapters, and persistence concerns; selected source shows gateway composition and shared service contracts. Exact file-by-file classification was not performed.
- Frontends: React application routing, GraphQL client/auth setup, and UI/editor code; the public site also has SSR/server entry points.
- Shared library: article-prose integration and Mermaid enhancement, with colocated tests.
- Deployment and automation: declarative Compose/Helm configuration and GitHub Actions workflows.

## Build Relationships

`gateway` depends on `domain_interface` and the domain crates. Frontends and public website use the local `packages/editor-prose` package. Deployment configurations build or package the services. See `dependencies.md` for the bounded dependency summary.

## Coverage Note

Deep reading was limited to the exact paths in the timestamp scope block. Other modules and tests within the listed application areas were skimmed or inventoried, not exhaustively classified; no tests or builds were run.