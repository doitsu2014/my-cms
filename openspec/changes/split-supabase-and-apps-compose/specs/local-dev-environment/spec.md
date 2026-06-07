## Purpose

TBD — capability modified by the `split-supabase-and-apps-compose` change. The capability describes how the my-cms local development environment is composed, configured, and reset, after splitting Supabase and the my-cms apps into two standalone Docker Compose files joined by an external network.

## MODIFIED Requirements

### Requirement: Two-compose bring-up on a shared external network

The repository SHALL provide the local development stack as two standalone Docker Compose files at the project root: `docker-compose.supabase.yaml` (the Supabase stack — `db`, `supavisor`, `auth`, `rest`, `realtime`, `storage`, `imgproxy`, `meta`, `studio`, `kong`, `mailpit`) and `docker-compose.my-cms.yaml` (the project apps — `init-wait`, `migrate`, `my-cms-api`, `my-cms-frontend`, `jaeger`). Both files SHALL declare a Docker network `supabase_network` as `external: true` with `name: supabase_network`, and SHALL join the my-cms apps to the Supabase stack by DNS resolution of the Supabase compose's container hostnames (`db`, `auth`, `storage`, `kong`, `realtime`, `meta`, `studio`, `imgproxy`, `mailpit`, `supavisor`).

#### Scenario: First-time setup

- **WHEN** a developer clones the repository, copies `.env.supabase.example` to `.env.supabase` and `.env.my-cms.example` to `.env.my-cms`, and edits the shared secrets in both files
- **AND** the developer runs `docker network create supabase_network` (one-time only)
- **THEN** `docker compose -f docker-compose.supabase.yaml --env-file .env.supabase up -d` starts the Supabase stack on the shared network
- **AND** `docker compose -f docker-compose.my-cms.yaml --env-file .env.my-cms up -d` starts the my-cms apps on the same shared network
- **AND** every Supabase service and every apps service reports `running` or `healthy` within its healthcheck interval

#### Scenario: Restart preserves data

- **WHEN** a developer runs `docker compose -f docker-compose.supabase.yaml down` followed by `docker compose -f docker-compose.supabase.yaml up -d`
- **THEN** the database, uploaded files, and Mailpit data persist across restarts
- **AND** the same applies to the apps compose independently

#### Scenario: External network absent

- **WHEN** a developer runs `docker compose -f docker-compose.supabase.yaml up -d` without first creating `supabase_network`
- **THEN** Compose fails with a clear error identifying the missing external network
- **AND** the error message includes the network name so the developer can create it with the documented command

### Requirement: Two per-stack env files with shared values synchronised

The repository SHALL provide two env templates at the project root: `.env.supabase.example` (variables consumed by `docker-compose.supabase.yaml`) and `.env.my-cms.example` (variables consumed by `docker-compose.my-cms.yaml`). Variables that are consumed by both stacks — at minimum `POSTGRES_PASSWORD`, `JWT_SECRET`, `ANON_KEY`, `SERVICE_ROLE_KEY`, `SUPABASE_PUBLIC_URL`, `SITE_URL`, `API_EXTERNAL_URL` — SHALL appear in both env files with a header comment reading `KEEP IN SYNC with .env.{other}` on the value.

#### Scenario: New developer onboarding

- **WHEN** a new developer clones the repository and copies both env templates
- **THEN** they only need to set `POSTGRES_PASSWORD` and `JWT_SECRET` (and any other secrets) in `.env.supabase`, and mirror those values into `.env.my-cms`
- **AND** no other env file is required to bring either stack online

#### Scenario: Shared value drift

- **WHEN** a developer sets `POSTGRES_PASSWORD=alpha` in `.env.supabase` and `POSTGRES_PASSWORD=beta` in `.env.my-cms`
- **THEN** the apps compose's `migrate` or `my-cms-api` services fail to authenticate to the Supabase `db`
- **AND** the error message in the affected service log identifies the role and password mismatch

### Requirement: Database init SQL uses upstream Supabase files plus a small my-cms grants file

The `volumes/db/` directory SHALL contain the upstream-vendored Supabase init SQL files: `roles.sql`, `jwt.sql`, `webhooks.sql`, `_supabase.sql`, `realtime.sql`, `logs.sql`, `pooler.sql`. The Supabase compose file SHALL mount these into the standard `supabase/postgres` init locations (`/docker-entrypoint-initdb.d/init-scripts/99-*.sql` and `/docker-entrypoint-initdb.d/migrations/9?-*.sql`). The directory SHALL also contain `99-my-cms-grants.sql`, which is mounted at `/docker-entrypoint-initdb.d/migrations/99-my-cms-grants.sql` and SHALL contain at minimum:

- `GRANT CREATE ON DATABASE postgres TO supabase_storage_admin;` (defensive, idempotent)
- `CREATE EXTENSION IF NOT EXISTS vector WITH SCHEMA public;` (defensive, idempotent)

The custom `init-config` envsubst service and `volumes/db/init/00-setup-roles.sql.template` SHALL NOT be present.

#### Scenario: pgvector enabled

- **WHEN** the database container finishes initialising
- **THEN** a query of `SELECT extname FROM pg_extension WHERE extname = 'vector'` returns one row

#### Scenario: Storage-api tenant migration succeeds

- **WHEN** the `storage` container starts and runs its tenant migration
- **THEN** the migration's `CREATE SCHEMA storage` step succeeds
- **AND** the storage-api container reports `running` or `healthy` (no `SQLSTATE 42501` crash loop)

#### Scenario: Auth service connects

- **WHEN** the `auth` (GoTrue) container starts and runs its migrations
- **THEN** the auth service connects with role `supabase_auth_admin`
- **AND** the auth service reports `running` or `healthy`

### Requirement: Two reset scripts, one per compose file

The repository SHALL provide two reset scripts at the project root:

- `reset-supabase.sh` — targets `docker-compose.supabase.yaml` only, stops the Supabase stack, removes its named volumes (`supabase_db_data`, `supabase_storage_data`, `mailpit_data`), and starts the stack fresh.
- `reset-apps.sh` — targets `docker-compose.my-cms.yaml` only, stops the my-cms apps, removes any apps-owned volumes, and starts the apps stack fresh.

Neither script SHALL touch the other stack's containers or volumes. Neither script SHALL remove the `supabase_network`.

#### Scenario: Reset Supabase only

- **WHEN** a developer runs `./reset-supabase.sh`
- **THEN** the Supabase containers are stopped and recreated
- **AND** the Supabase named volumes are wiped
- **AND** the my-cms apps stack is unaffected

#### Scenario: Reset apps only

- **WHEN** a developer runs `./reset-apps.sh`
- **THEN** the my-cms apps containers are stopped and recreated
- **AND** the Supabase stack is unaffected

#### Scenario: Reset preserves images

- **WHEN** `reset-supabase.sh` or `reset-apps.sh` finishes
- **THEN** Docker images are not re-pulled unless the compose file references a different tag

### Requirement: `init-wait` service gates the apps migration

The `docker-compose.my-cms.yaml` file SHALL declare an `init-wait` service (alpine, `restart: "no"`) that polls `db:5432` via `nc -z` until the TCP connection succeeds, then exits 0. The `migrate` service SHALL declare `depends_on: init-wait: service_completed_successfully`. The `my-cms-api` service SHALL declare `depends_on: migrate: service_completed_successfully`. Neither service SHALL declare `depends_on: db` (because `db` lives in a different Compose file).

#### Scenario: Apps compose started before Supabase is healthy

- **WHEN** a developer runs `docker compose -f docker-compose.my-cms.yaml up -d` while the Supabase `db` container is still starting
- **THEN** `init-wait` polls until `db:5432` accepts a connection
- **AND** `migrate` runs after `init-wait` exits 0
- **AND** `my-cms-api` starts after `migrate` exits 0
- **AND** the apps compose's `docker compose ps` eventually shows `init-wait: exited (0)`, `migrate: exited (0)`, `my-cms-api: running`

## REMOVED Requirements

### Requirement: One-command local stack bring-up

*Removed because the local development environment is no longer a single Compose file. Replaced by the "Two-compose bring-up on a shared external network" requirement above, which documents the two-command workflow.*

### Requirement: Single env file drives all services

*Removed because the stack is no longer driven by a single env file. Replaced by the "Two per-stack env files with shared values synchronised" requirement above.*

### Requirement: Hot-reload override is opt-in via a separate file

*Removed because the override pattern does not apply to two-file Compose setups. Per-developer hot-reload customisations are documented inline as commented examples in `docker-compose.my-cms.yaml`.*
