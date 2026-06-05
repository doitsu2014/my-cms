# local-dev-environment Specification

## Purpose
TBD - created by archiving change unified-docker-compose-with-supabase. Update Purpose after archive.
## Requirements
### Requirement: One-command local stack bring-up

The repository SHALL provide a single `docker compose up -d` command at the project root that starts the full local development stack (Supabase services, the my-cms API, the React admin dev server, and Jaeger) on a shared Docker network.

#### Scenario: Clean start from scratch

- **WHEN** a developer runs `cp .env.example .env`, fills in `POSTGRES_PASSWORD` and `JWT_SECRET`, then runs `docker compose up -d`
- **THEN** all services start without error
- **AND** the `db` container reports healthy within its healthcheck interval

#### Scenario: Restart preserves data

- **WHEN** a developer runs `docker compose down` followed by `docker compose up -d`
- **THEN** the database, uploaded files, and Mailpit data persist across restarts

### Requirement: Single env file drives all services

The repository SHALL provide a single `.env.example` template at the project root that documents every environment variable consumed by any service in the stack (Supabase, my-cms API, frontend build args, Mailpit, OTLP, OpenAI).

#### Scenario: New developer onboarding

- **WHEN** a new developer clones the repository and copies `.env.example` to `.env`
- **THEN** they only need to set `POSTGRES_PASSWORD` and `JWT_SECRET` to bring the stack online
- **AND** no other service-specific env files are required to start any container

#### Scenario: Missing env file fails fast

- **WHEN** a developer runs `docker compose up -d` without a populated `.env`
- **THEN** services that require a missing variable fail to start
- **AND** the error message identifies which variable is missing

### Requirement: Supabase services are reachable on documented ports

The Supabase services SHALL be reachable on the following ports from the host: Studio on `:8000`, Kong gateway on `:8001`, PostgreSQL on `:5432`, and Mailpit UI on `:8025`.

#### Scenario: Studio accessible

- **WHEN** a developer opens `http://localhost:8000` in a browser
- **THEN** the Supabase Studio login page loads
- **AND** credentials `admin` / `admin` (from `DASHBOARD_USERNAME` / `DASHBOARD_PASSWORD`) authenticate successfully

#### Scenario: Kong gateway accessible

- **WHEN** a developer sends a request to `http://localhost:8001/auth/v1/health`
- **THEN** the Kong gateway responds
- **AND** the request is routed to the GoTrue service

### Requirement: Project services are reachable on documented ports

The my-cms project services SHALL be reachable on the following ports: API on `:8989`, frontend dev server on `:3002`, Jaeger UI on `:16686`, and Jaeger OTLP gRPC receiver on `:4317`.

#### Scenario: API health check

- **WHEN** a developer sends `GET http://localhost:8989/health`
- **THEN** the API responds with HTTP 200 and `{"status":"ok"}`

#### Scenario: Frontend dev server

- **WHEN** a developer opens `http://localhost:3002` in a browser
- **THEN** the React admin panel loads

#### Scenario: Jaeger UI

- **WHEN** a developer opens `http://localhost:16686` in a browser
- **THEN** the Jaeger UI loads

### Requirement: Database init script creates required roles and extensions

The `volumes/db/init/00-setup-roles.sql` script SHALL run on first database startup and SHALL create the roles `supabase_auth_admin`, `supabase_storage_admin`, `supabase_admin`, `anon`, `authenticated`, `authenticator`, `service_role` with the grants documented by upstream Supabase. The script SHALL also enable the `vector`, `uuid-ossp`, and `pgcrypto` extensions.

#### Scenario: pgvector enabled

- **WHEN** the database container finishes initialising
- **THEN** a query of `SELECT extname FROM pg_extension WHERE extname = 'vector'` returns one row

#### Scenario: Auth and storage services can connect

- **WHEN** GoTrue attempts to connect with role `supabase_auth_admin`
- **THEN** the connection succeeds
- **AND** the auth schema migrations can run

### Requirement: Reset script wipes persistent state and restarts cleanly

The `resetsupabasedb.sh` script SHALL stop all services, remove the `supabase_db_data`, `supabase_storage_data`, and `mailpit_data` named volumes, and start the stack fresh.

#### Scenario: Full reset

- **WHEN** a developer runs `./resetsupabasedb.sh`
- **THEN** all services are stopped
- **AND** the three named volumes are removed
- **AND** the stack is brought up from a clean state

#### Scenario: Reset preserves images

- **WHEN** `resetsupabasedb.sh` finishes
- **THEN** Docker images are not re-pulled unless the compose file references a different tag

### Requirement: Hot-reload override is opt-in via a separate file

The repository SHALL provide a `docker-compose.override.example.yml` template that demonstrates hot-reload for the API (bind-mount of source dirs) and the frontend (rsbuild HMR). The active `docker-compose.override.yml` SHALL be gitignored so per-developer overrides do not leak into version control.

#### Scenario: Override file absent

- **WHEN** no `docker-compose.override.yml` is present
- **THEN** the stack uses the canonical compose file and runs as committed

#### Scenario: Override file present

- **WHEN** a developer copies `docker-compose.override.example.yml` to `docker-compose.override.yml` and edits it
- **THEN** Docker Compose automatically merges the override on `up`
- **AND** the override file is not committed to git

