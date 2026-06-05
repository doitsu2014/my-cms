## Context

my-cms is a Rust headless CMS with a React (rsbuild) admin panel. The pre-change local stack is fragmented:

- **PostgreSQL** is provisioned manually (SeaORM migrations run against a developer-local database).
- **Auth** is delegated to an external Keycloak realm at `https://my-ids-admin.ducth.dev`.
- **Object storage** lives on Contabo S3 (`sin1.contabostorage.com`).
- **Vector search** runs on a separate Qdrant instance (HTTP/gRPC).
- **Tracing** uses a local Jaeger all-in-one.

This forces every developer to install and configure four external services before running the API, makes the "reset the database" loop painful, and leaves CI/integration tests without a reproducible local stack. The codebase is also migrating away from Keycloak/Qdrant/Contabo-S3 to Supabase equivalents (covered by follow-up changes); those migrations assume a Supabase-shaped local environment. This change provides that environment.

## Goals / Non-Goals

**Goals:**

- One command (`docker compose up -d`) brings the full local stack to a healthy state.
- A single `.env.example` (and copyable `.env`) drives every service — no scattered config.
- Service discovery via Docker DNS on a shared `supabase_network` bridge network.
- Reproducible resets via `resetsupabasedb.sh` (wipes named volumes, restarts).
- Healthchecks on database and storage services so dependents wait for readiness.
- All Supabase services exposed through Kong on `:8001` and Studio on `:8000`; the Rust API on `:8989`; the rsbuild dev server on `:3002`; Jaeger UI on `:16686`; Mailpit UI on `:8025`.
- pgvector (`vector`), `uuid-ossp`, and `pgcrypto` extensions are pre-installed by the init script so follow-up migrations can rely on them.

**Non-Goals:**

- Code changes in `services/` or `frontend/` to consume Supabase APIs (handled by follow-up changes for auth, vector search, and storage).
- Production deployment — compose is local development only. Production uses Helm charts.
- Migrating the Rust API from Qdrant to pgvector SQL (separate change).
- Migrating Keycloak auth to Supabase GoTrue (separate change).
- Migrating Contabo S3 media storage to Supabase Storage (separate change).

## Decisions

### Decision 1: Single root `docker-compose.yml` on a shared bridge network

All services live in one Compose file at the repo root, joined to a single `supabase_network` bridge. Service discovery uses Docker DNS (e.g., the API reaches Postgres at `db:5432`, the auth service at `auth:9999`, etc.).

**Rationale:** keeps the dev story to "one file, one command", avoids drift between multiple compose files, and matches the upstream self-hosted Supabase layout so snippets from Supabase docs apply directly. Alternatives considered: (a) split `docker-compose.supabase.yml` + `docker-compose.app.yml` with `include:` — rejected because the Supabase team does not ship that layout and most copy-paste guidance assumes a single file; (b) Helm/Compose-for-Prod — rejected, it adds Kubernetes-style complexity inappropriate for local dev.

### Decision 2: Pin to the official Supabase self-hosted image set

Use images straight from `supabase/*` (Postgres `supabase/postgres:15.6.1.148`, GoTrue `v2.179.6`, Storage `v1.33.23`, Realtime `v2.67.1`, Studio `2026.05.07-sha-2e4841f`, meta `v0.91.0`, Supavisor `2.7.6`) with Kong `2.8.1` and Jaeger `1.53`.

**Rationale:** every service has a "long-lived" version line that gets security patches; pinning the image tag (not `latest`) means reproducible bring-up. Alternatives considered: `latest` — rejected because reproducibility matters for integration tests and onboarding; building from source — rejected, far too slow for the day-to-day dev loop.

### Decision 3: Pre-create Supabase roles + extensions via `volumes/db/init/00-setup-roles.sql`

The init script creates `supabase_auth_admin`, `supabase_storage_admin`, `supabase_admin`, `anon`, `authenticated`, `authenticator`, `service_role`, then enables the `vector`, `uuid-ossp`, and `pgcrypto` extensions.

**Rationale:** Supabase services assume these roles exist with the documented grants; without them, GoTrue and Storage fail on first connection. Hard-coding `${POSTGRES_PASSWORD}`-style substitutions in mounted SQL is unreliable (Compose does not interpolate inside bind-mounted files), so the script documents that the password is written literally — acceptable for local dev. Alternatives considered: an init container that runs the SQL via psql — rejected, it adds another moving part; relying on GoTrue/Storage to bootstrap their own roles — rejected, some roles (`anon`, `authenticated`) are not auto-created by the upstream images.

### Decision 4: Single `.env` file is the configuration source of truth

A single `.env` at the repo root drives Postgres, JWT, SMTP, OpenAI, OTLP, frontend build args, and Supabase Storage.

**Rationale:** one place to look, one place to template. Splitting envs per service (multiple `env_file:` lines) would re-introduce the drift this change is trying to remove. Alternatives considered: per-service env files in subdirectories — rejected for the same drift reason.

### Decision 5: SeaORM migrations run on API startup

The API container connects to `db:5432` and SeaORM runs its migrations during boot; no init container is added.

**Rationale:** reuses the API's existing migration tooling, avoids duplicating the migration list. Alternatives considered: a separate `migrate` init container — rejected, it would require duplicating the Cargo workspace and source code; mounting migrations into the DB container — rejected, the DB image has no Rust toolchain.

### Decision 6: Named volumes for DB, storage, mailpit; bind mount for kong config and DB init SQL

- `supabase_db_data` → `/var/lib/postgresql/data` (DB persistent data)
- `supabase_storage_data` → `/var/lib/storage` (uploaded files) and read-only into imgproxy
- `mailpit_data` → `/data`
- `./volumes/api/kong.yml` → `/var/lib/kong/kong.yml:ro` (declarative gateway config)
- `./volumes/db/init/` → `/docker-entrypoint-initdb.d:ro` (role/extension bootstrap)

**Rationale:** large stateful data (DB, uploads, mailpit) lives in named volumes so it survives `docker compose down` and is portable; the small declarative configs stay in git via bind mounts so the team can review changes to Kong routing or DB init alongside the rest of the code. Alternatives considered: bind-mounting DB data on the host — rejected, it pollutes the working tree and makes `git status` noisy.

### Decision 7: Hot-reload override file is a template, not a default

`docker-compose.override.example.yml` shows how to enable rsbuild hot-reload and API source bind-mounts; the active override file is `.gitignored` so each developer can edit it freely.

**Rationale:** the default compose is deterministic and matches CI; hot-reload is a per-developer convenience that should not leak into the canonical file.

### Decision 8: Reset script wipes volumes explicitly (not `-v` flag)

`resetsupabasedb.sh` runs `docker compose down -v --remove-orphans` and then `docker volume rm -f supabase_db_data supabase_storage_data mailpit_data` for clarity, then brings services back up.

**Rationale:** named volumes are not always removed by `down -v` if other containers reference them; the explicit `rm` makes the intent obvious in script output. Alternatives considered: a Makefile target — rejected, plain bash keeps the onboarding step simple and works on every shell.

## Risks / Trade-offs

- **Compose is local-dev only.** A misconfigured production deploy using this file would be wrong (no TLS, single Postgres, Mailpit in place of a real SMTP). Mitigation: production uses Helm and the README/AGENTS.md points to that explicitly.
- **Init SQL is literal, not interpolated.** The DB init script must match `POSTGRES_PASSWORD` from `.env` or the auth/storage services will fail. Mitigation: the script is a single file; a developer setting up a new environment must keep the two in sync (documented in the script's header).
- **Pinned image versions will drift from upstream.** As Supabase releases new versions, the compose file will not auto-update. Mitigation: bump versions explicitly when the team wants new features/security fixes (tracked as a routine maintenance task).
- **Resource usage on developer machines.** Twelve-plus containers all running concurrently can stress lower-end laptops. Mitigation: `docker compose down` is one command; the reset script only touches the data volumes, not images.
- **`.env` is gitignored.** A new developer can forget to copy `.env.example` and get confusing failures. Mitigation: compose fails fast on the first service that requires `POSTGRES_PASSWORD`; the README points to the example file.

## Migration Plan

- Devs run `cp .env.example .env`, edit secrets, then `docker compose up -d`.
- Follow-up changes (auth, pgvector, storage) build on this environment without further env changes.

## Open Questions

- None at this time. The environment is in place; deeper behavior changes (pgvector SQL queries, GoTrue JWT validation in Rust, Supabase Storage upload/download) are tracked by the follow-up changes.
