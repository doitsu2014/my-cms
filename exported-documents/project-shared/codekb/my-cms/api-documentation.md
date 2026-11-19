# API Documentation

## API Surfaces

| Surface | Location | Observed scope | Notes |
|---|---|---|---|
| Backend HTTP/GraphQL | Axum gateway and domain route composition | Posts/category/tag/translation, media/bucket, user, SEO, health and migration/backup pathways identified broadly | Exact route inventory, request/response contracts and route counts were not enumerated in the mixed-depth scan. |
| Posts GraphQL | `domain_posts` | Immutable and mutable surfaces identified | Exact schema and authorization enforcement require source-level verification. |
| Public reader | `apps/ducth-dev-website` | React Router routes and Express 5 server; Apollo client | Application source was skimmed, not exhaustively inspected. |
| Admin | `apps/web` | React Router/Apollo and Supabase JS integrations | Application source was skimmed, not exhaustively inspected. |
| Backup/restore | Gateway CLI skeleton and feature specification | No complete ZIP/UI/upload/import flow per `docs/features/backup-restore.md` | Specification describes desired workflows, not a current HTTP API contract. |

## Endpoint Inventory and Limits
The scan did not enumerate every HTTP method/path or exact request/response schema. Do not treat this document as an OpenAPI replacement or assert endpoint counts. The feature specification defines administrator workflows and acceptance criteria but does not by itself establish registered API routes.

## Authentication and Middleware
Gateway/domain architecture uses shared composition and auth middleware. Exact middleware ordering, route coverage, role mapping, status/error behavior, rate limits, and CORS details were not comprehensively validated in this scan. Any backup-related operation must be confirmed to enforce server-side authorization and object/job ownership before implementation is treated as complete.

## Backup/Restore Contract (Specified, Not Implemented)
The feature narrative describes source selection and backup job creation, download only after complete package and checksum verification, private upload, preflight without target mutation, explicit confirmation, recovery-point creation, restore/post-restore checks, and audit. It does not establish deployed endpoint paths or wire formats. The same document states there is no complete ZIP flow, admin UI, upload/import restore, or instance selection.
