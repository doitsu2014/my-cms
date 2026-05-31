# Design: Unified Docker Compose with Full Supabase Stack

**Date:** 2026-05-31
**Status:** Approved

## 1. Goal

Create a single `docker-compose.yml` that runs the full Supabase stack alongside all project services (my-cms API, frontend, Jaeger) for one-command local development startup.

## 2. Context

The project (`my-cms`) is a Rust-based headless CMS with:
- PostgreSQL via SeaORM (current) → replaced by Supabase PostgreSQL
- Keycloak for auth → Supabase GoTrue will eventually replace this
- S3 (Contabo) for media storage → Supabase Storage will eventually replace this
- Qdrant for vector search → pgvector on Supabase PostgreSQL replaces this
- Jaeger for distributed tracing → retained
- React frontend admin panel (rsbuild) → retained

## 3. Architecture

Single `docker-compose.yml` at project root with all services on a shared Docker network.

### 3.1 Supabase Stack

| Service | Container | Port | Purpose |
|---|---|---|---|
| `supabase-db` | PostgreSQL 15 | 5432 | Database with pgvector, postgis, etc. |
| `studio` | Supabase Studio | 8000 | Admin dashboard |
| `kong` | Kong API Gateway | 8001 | Routes requests to Supabase services |
| `auth` | GoTrue | internal | User authentication |
| `rest` | PostgREST | internal | Auto-generated REST API over PostgreSQL |
| `realtime` | Supabase Realtime | internal | WebSocket subscriptions |
| `storage` | Supabase Storage | internal | File storage with S3-compatible API |
| `analytics` | Logflare | internal | Query analytics |
| `imgproxy` | imgproxy | internal | Image resizing |
| `mailpit` | Mailpit | 8025 | Email testing for auth flows |
| `supavisor` | Supavisor | internal | Connection pooling |
| `edge-functions` | Deno | internal | Edge function runtime |

### 3.2 Project Services

| Service | Port | Purpose |
|---|---|---|
| `my-cms-api` | 8989 | Rust API server (build from `services/Dockerfile`) |
| `my-cms-frontend` | 3002 | React admin panel (rsbuild dev server) |
| `jaeger` | 16686 (UI), 4317 (OTLP) | Distributed tracing |

## 4. Environment Configuration

A single `.env` file at project root drives all services. Key variables:

```
# PostgreSQL / Supabase DB
POSTGRES_PASSWORD=your-super-secret-password
POSTGRES_DB=my-cms

# Supabase JWT (shared secret across GoTrue, PostgREST, Realtime, Storage)
JWT_SECRET=your-super-secret-jwt-token-with-at-least-32-characters
ANON_KEY=<generated from JWT_SECRET>
SERVICE_ROLE_KEY=<generated from JWT_SECRET>

# Supabase Dashboard credentials
DASHBOARD_USERNAME=admin
DASHBOARD_PASSWORD=admin

# my-cms API
DATABASE_URL=postgresql://postgres:${POSTGRES_PASSWORD}@supabase-db:5432/my-cms
DATABASE_SCHEMA=public
HOST=0.0.0.0
PORT=8989
ENABLED_OTLP_EXPORTER=true
OTEL_EXPORTER_OTLP_TRACES_ENDPOINT=http://jaeger:4317

# Keys (placeholders for local dev)
OPENAI_API_KEY=
AWS_ACCESS_KEY_ID=
AWS_SECRET_ACCESS_KEY=
S3_ENDPOINT=
S3_BUCKET_NAME=
MEDIA_BASE_URL=http://localhost:8989

# Keycloak (existing, external)
KEYCLOAK_ISSUER=https://my-ids-admin.ducth.dev
KEYCLOAK_REALM=my-blogs
AUTHORIZATION_AUDIENCE=my-cms-headless-api

# Frontend (build-time vars for rsbuild, passed via build args)
PUBLIC_KEYCLOAK_URL=https://my-ids-admin.ducth.dev
PUBLIC_KEYCLOAK_REALM=master
PUBLIC_KEYCLOAK_CLIENT_ID=my-blogs-admin-localhost
PUBLIC_GRAPHQL_API_URL=http://localhost:8989/graphql
PUBLIC_REST_API_URL=http://localhost:8989/api
PUBLIC_MEDIA_UPLOAD_API_URL=http://localhost:8989/api/media/upload
```

## 5. Database Schema

SeaORM migrations run against `supabase-db` on API startup. The `my-cms` database is created inside the `postgres` database. Migrations are applied via `sea-orm-cli migrate up` (run manually or as an init container).

## 6. Volumes

| Volume | Purpose |
|---|---|
| `supabase_db_data` | PostgreSQL persistent data |
| `supabase_storage_data` | Uploaded files via Supabase Storage |

## 7. Network

All services on a single `supabase_network` bridge network. Service discovery via Docker DNS (e.g., `supabase-db` resolves to the PostgreSQL container).

## 8. Startup Flow

1. `docker compose up -d` starts all services
2. Supabase PostgreSQL initializes with extensions (pgvector, postgis, etc.)
3. Supabase services (GoTrue, PostgREST, etc.) initialize and register with Kong
4. Jaeger starts accepting OTLP traces
5. `my-cms-api` starts, connects to `supabase-db`, runs migrations
6. `my-cms-frontend` starts dev server on port 3002
7. Access Supabase Studio at `http://localhost:8000`, API at `http://localhost:8989`, frontend at `http://localhost:3002`

## 9. Files to Create

1. **`docker-compose.yml`** — All services, networks, volumes, healthchecks, dependencies
2. **`.env.example`** — Template with all variables (no secrets)
3. **`resetsupabasedb.sh`** — Script to wipe and reset the Supabase database volumes (dev convenience)

## 10. Out of Scope

- Migrating the Rust API from Qdrant HTTP API to pgvector SQL queries (code change, separate task)
- Migrating Keycloak auth to Supabase GoTrue (code change, separate task)
- Migrating S3 media storage to Supabase Storage (code change, separate task)
- Production deployment (compose is for local dev only; production uses Helm charts)
