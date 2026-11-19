# Project glossary

Project-specific terminology used in the seven contracts. Definitions describe this project's context, not universal behavior. Evidence references point to contracts and maintained context; implementation details remain subject to each contract's evidence boundary.

## Terms

- **AI translation** — Generation or reuse of localized post title, preview, and article content through the posts domain. See [posts and translation](posts-content-and-translation.md); implementation details are partly unverified.
- **Apollo** — GraphQL client technology named in project docs for frontend/API requests. See [deployment and operations](deployment-operations.md) and `README.md`.
- **Article prose** — Shared read-only rendering contract for saved article HTML, owned by `packages/editor-prose` and rooted at `.article-prose`. See [admin editor](admin-editor.md) and `docs/features/editor-prose-contract.md`.
- **Ducth / Ducth.dev** — Project-specific name used for the public reader website (`apps/ducth-dev-website`). See [public site](public-site.md) and `README.md`.
- **GoTrue** — Supabase identity service used by the intended authentication and administrator-user integrations. See [authentication, users, and media](auth-users-media.md).
- **GraphQL immutable/mutable** — Two posts-domain GraphQL surfaces; immutable is intended public, mutable protected for writers or administrators. See [backend architecture](backend-domain-architecture.md); exact schema/enforcement is unverified.
- **Mermaid** — Diagram syntax supported as explicitly marked article code blocks, rendered by the shared prose contract. See [admin editor](admin-editor.md).
- **My-CMS / my-cms-api** — My-CMS is the publishing platform; `my-cms-api` is the documented sole shipped API binary, composed by the gateway. See [backend architecture](backend-domain-architecture.md) and `README.md`.
- **pgvector** — PostgreSQL vector extension used by the intended semantic translation-reuse path. See [posts and translation](posts-content-and-translation.md); extension availability and fallback require current-source validation.
- **Quick Toolbar** — Contextual editor toolbar whose intended insertion actions are image, video, and code block. See [admin editor](admin-editor.md); current implementation is unverified.
- **SeaORM** — Rust ORM/persistence layer named in the project architecture for domain-owned PostgreSQL access. See [backend architecture](backend-domain-architecture.md) and `docs/architecture/software-architecture.md`.
- **Supabase Storage** — Supabase object-storage service used by the intended media domain. See [authentication, users, and media](auth-users-media.md).
- **TipTap** — Rich-text editor used for admin authoring; its saved HTML is consumed by the shared article renderer. See [admin editor](admin-editor.md) and `docs/features/editor-prose-contract.md`.
- **Traefik** — Reverse proxy used by the documented local Docker environment. See [deployment and operations](deployment-operations.md) and `deployments/docker-swarm/README.md`.

## Unresolved terms

- **Ink & Tide** — Name of the public site's design-token/visual system in the existing contract. Its canonical definition or maintained design source was not present in the inspected evidence; project owner/design source should confirm before expanding the definition.
- **SEO head asset** — Project term for administrator-managed markup delivered into public document head. Exact permitted markup and validation rules are not established by the partial scan; see [public site](public-site.md).
- **Writer** — Authorization role referenced as `my-headless-cms-writer`; exact claim representation and runtime matching remain unverified. See [backend architecture](backend-domain-architecture.md).
