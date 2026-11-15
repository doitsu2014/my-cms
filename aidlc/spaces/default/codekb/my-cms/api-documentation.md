# API Documentation

## First-Party HTTP Surfaces

| Surface | Observed routes/contracts | Authentication / confidence |
|---|---|---|
| Gateway | `/`, `/health`, `/healthz` health routes. | Exact middleware for health routes not established. |
| Posts domain | REST routes for posts/content, categories, tags, translation, and translation jobs. | Route inventory is distributed across registration and handler modules; complete method/path/parameter list was not scanned. |
| Posts GraphQL | `/posts/graphql/immutable` and `/posts/graphql/mutable`; schema contribution includes seven historical entities. | Immutable is described as public; mutable as authenticated. Exact schema and authorization details were not exhaustively reviewed. |
| User domain | `POST /users`, `GET /users`, `GET /users/:id`, `PUT /users/:id`, `DELETE /users/:id`, `POST /users/:id/reset-password`. | Route list comes from project knowledge `auth-users-media.md`; service source supports domain responsibility. Confirm current router registration before relying on every route. |
| Media domain | Media and bucket operations. | Handlers/API modules inventoried, but full route/method list was not read. |
| SEO domain | Administrator head-asset CRUD and public delivery of enabled assets. | Route registration evidence in service and test; full request/response contract not enumerated. |
| Public website | `/:lang` routes for home, categories, category detail, post detail, and about. | Route composition in `apps/ducth-dev-website/src/AppContent.tsx`; these are website routes, not necessarily backend API endpoints. |

## External Integrations

- Supabase GoTrue/Admin API and JWT validation for identity and authentication.
- Supabase Storage for media objects.
- PostgreSQL/pgvector for persistence and vector-related capabilities.
- OpenAI chat completions and embeddings for translation/embedding workflows.

Exact external payload contracts, API versions, retry behavior, and all status/error mappings were not fully scanned. No OpenAPI specification or complete endpoint inventory is asserted by this scan.