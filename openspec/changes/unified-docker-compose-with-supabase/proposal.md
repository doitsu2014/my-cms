## Why

Local development of my-cms currently requires developers to start the API, the React admin panel, and tracing manually, while the underlying PostgreSQL, auth, storage, and vector search services live on heterogeneous external systems (Qdrant, Keycloak, Contabo S3, plain Postgres). A single `docker compose up` should bring the entire local stack — Supabase, the Rust API, the rsbuild dev server, and Jaeger — online with a shared `.env` so onboarding, integration testing, and reset are one command.

## What Changes

- Add a single root `docker-compose.yml` that runs the full self-hosted Supabase stack (PostgreSQL 15 with pgvector + postgis, GoTrue, PostgREST, Realtime, Storage, Storage imgproxy, Studio, Kong, Supavisor, Logflare, Mailpit) alongside `my-cms-api`, `my-cms-frontend`, and Jaeger all-in-one.
- Add `.env.example` as the single source of truth for all service configuration (Postgres, JWT, SMTP, OpenAI, OTLP, Keycloak, frontend, Supabase Storage).
- Add `volumes/api/kong.yml` declarative gateway routing for `/auth/v1`, `/rest/v1`, `/realtime/v1`, `/storage/v1`, `/analytics/v1`, `/pg/`, `/graphql/v1`.
- Add `volumes/db/init/00-setup-roles.sql` to create the Supabase roles (`supabase_auth_admin`, `supabase_storage_admin`, `anon`, `authenticated`, `authenticator`, `service_role`, `supabase_admin`) and enable the `vector`, `uuid-ossp`, and `pgcrypto` extensions on first DB startup.
- Add `frontend/Dockerfile.dev` for the rsbuild dev server.
- Add `resetsupabasedb.sh` convenience script to wipe named volumes and start fresh.
- Add `docker-compose.override.example.yml` for hot-reload overrides.
- Update `.gitignore` to exclude `.env`, `docker-compose.override.yml`, and runtime volume mounts.

## Capabilities

### New Capabilities

- `local-dev-environment`: one-command local stack — Supabase + my-cms API + frontend + Jaeger — defined in a single `docker-compose.yml` with a unified `.env`, healthchecks, and a reset script.

### Modified Capabilities

<!-- None — this change introduces the dev-environment capability. Existing capabilities will be added by later changes (Supabase auth, pgvector, Supabase Storage). -->

## Impact

- New files: `docker-compose.yml`, `.env.example`, `resetsupabasedb.sh`, `docker-compose.override.example.yml`, `frontend/Dockerfile.dev`, `volumes/api/kong.yml`, `volumes/db/init/00-setup-roles.sql`.
- Modified files: `.gitignore` (ignore `.env`, `docker-compose.override.yml`, `volumes/`).
- Runtime: introduces `supabase_network` Docker network and three named volumes (`supabase_db_data`, `supabase_storage_data`, `mailpit_data`).
- Developer workflow: `cp .env.example .env` → edit secrets → `docker compose up -d`. Access points: Studio `:8000`, Kong `:8001`, API `:8989`, frontend `:3002`, Jaeger `:16686`, Mailpit `:8025`.
- Out of scope (handled by follow-up changes): Rust code changes for pgvector, Supabase GoTrue JWT, or Supabase Storage — this change is environment-only.
