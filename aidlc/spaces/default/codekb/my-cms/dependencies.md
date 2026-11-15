# Dependencies

## Internal Package Relationships

- `gateway` → `domain_interface`, `domain_posts`, `domain_auth`, `domain_media`, `domain_user`, `domain_seo`.
- Domain crates implement/use the shared `domain_interface` service contract.
- `apps/web/` → local `packages/editor-prose/`.
- `apps/ducth-dev-website/` → local `packages/editor-prose/`.
- Deployment definitions build or package the API and frontend applications; exact image dependency graph was not fully traced.

## Principal External Dependencies and Services

| Dependency/service | Consumers or purpose | Evidence limits |
|---|---|---|
| Axum, Tokio, SeaORM | API HTTP/runtime/persistence stack. | Versions in `technology-stack.md`; full feature usage not enumerated. |
| async-graphql, Seaography | Posts GraphQL integration and schema contribution. | Exact generated schema not exhaustively read. |
| Supabase PostgreSQL/pgvector | Backend relational and vector persistence. | Runtime schema and all query dependencies not fully inventoried. |
| Supabase GoTrue/JWT | User administration and authentication/role middleware. | External contract and version details not fully scanned. |
| Supabase Storage | Media and bucket operations. | Full operation list not scanned. |
| OpenAI chat completions/embeddings | Translation and embedding paths in posts domain. | Exact external payload contracts not fully scanned. |
| React, Apollo Client, TipTap | Admin application and authoring/editor UI. | Additional frontend packages omitted from this bounded summary. |
| Express, React Router, i18next | Public-site SSR, routing, and localization. | Full runtime dependency graph not extracted. |
| Mermaid | Shared prose rendering enhancement. | Package declares version `11.17.2`. |
| Docker Compose/Swarm, Helm/Kubernetes | Deployment packaging/configuration. | Deployments were inventoried, not executed or validated. |

## Build and Test Tooling

Cargo builds the Rust workspace. pnpm manages separate frontend packages. Vitest/Testing Library and Rust's built-in test harness are present; workflow/config findings are summarized in `code-quality-assessment.md`. No dependency installation, build, or test was performed for this synthesis.