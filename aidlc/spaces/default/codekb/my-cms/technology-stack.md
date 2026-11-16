# Technology Stack

Versions below are those recorded in the scanned package manifests; where the scan supplied no exact version, it is identified as such.

## Backend

- Rust 2021 edition; Cargo workspace under `apps/api/`.
- Axum `0.8.9`, Tokio `1.52.3`, SeaORM and SeaORM migration `1.1.20`.
- async-graphql `7.2.1`; Seaography `1.1.4`.
- PostgreSQL with pgvector, accessed through the backend persistence stack.

## Frontends and Shared Package

- React `19.1.1`; TypeScript `5.9.2`; Rsbuild `1.5.6`.
- Tailwind CSS 4 and DaisyUI 5; Apollo Client 3.x; TipTap `3.14.0` in the admin.
- Public site: Express `5.2.1`, React Router `7.11.0`, i18next `25.7.4`, OpenTelemetry Node SDK `0.212.0`.
- Shared prose package: Mermaid `11.17.2`; React peer dependency `>=18`.

## Platform and Delivery

- Supabase: PostgreSQL, GoTrue identity, JWT validation, and Storage.
- OpenAI: chat completions and embeddings.
- Docker Compose/Swarm, Traefik, self-hosted Supabase, Jaeger, Helm/Kubernetes, and GitHub Actions are identified in the scanned README/deployment/workflow inventory.

This is not a complete lockfile-derived bill of materials. Remaining packages and resolved transitive versions were not exhaustively inspected.