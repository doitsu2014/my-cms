## Why

The current `docker-compose.yml` rolls Supabase and the my-cms apps into one monolithic file with a hand-rolled init-SQL pipeline (`init-config` envsubst service + `00-setup-roles.sql.template`). Two problems result:

1. **Drift from upstream Supabase.** Upstream's `supabase/postgres` image already bootstraps the Supabase roles with the correct grants on first boot; our custom script both duplicates that work and silently dropped one grant — `GRANT CREATE ON DATABASE postgres TO supabase_storage_admin` — which causes the storage-api tenant migration to fail with `SQLSTATE 42501` (`permission denied for database postgres`). The error surfaces as a crash loop on `supabase-storage`.
2. **Coupled lifecycles.** my-cms apps and the Supabase stack are brought up, reset, and torn down as one unit, even though they have independent release cadences and may be replaced by hosted equivalents in the future (e.g., a managed Supabase project in production). The Supabase half of the stack is also slowly diverging from the upstream self-hosted layout, which means copy-paste guidance from Supabase docs no longer applies.

This change splits the monolithic compose into two standalone files joined only by an external `supabase_network`, and adopts the upstream Supabase init-SQL pattern (vendor the upstream `roles.sql` / `jwt.sql` / etc., add a tiny my-cms-specific grants file).

## What Changes

- **New `docker-compose.supabase.yaml`** at the repo root, derived from `supabase/supabase` `master/docker/docker-compose.yml` (the official self-hosted layout). Declares the minimal Supabase set: `db`, `supavisor`, `auth` (GoTrue), `rest` (PostgREST), `realtime`, `storage`, `imgproxy`, `meta`, `studio`, `kong`, `mailpit`. Pins the same image tags the project already uses. Joins the external `supabase_network`.
- **New `docker-compose.my-cms.yaml`** at the repo root, declaring only the project-owned services: an `init-wait` helper (alpine, polls `db:5432`), `migrate` (SeaORM one-shot), `my-cms-api`, `my-cms-frontend`, and `jaeger`. Joins the same external `supabase_network`. Service discovery uses the supabase compose's container hostnames (`db`, `auth`, `storage`, `kong`, …).
- **New `volumes/db/`** (replaces `volumes/db/init/`) with the upstream-vendored init SQL files: `roles.sql`, `jwt.sql`, `webhooks.sql`, `_supabase.sql`, `realtime.sql`, `logs.sql`, `pooler.sql`. Plus one my-cms-specific file `99-my-cms-grants.sql` (defensive `GRANT CREATE ON DATABASE postgres TO supabase_storage_admin;` + `CREATE EXTENSION IF NOT EXISTS vector`).
- **New `.env.supabase`** (Supabase-specific variables) and **`.env.my-cms`** (apps-specific variables). Shared values (`POSTGRES_PASSWORD`, `JWT_SECRET`, `ANON_KEY`, `SERVICE_ROLE_KEY`, `SUPABASE_PUBLIC_URL`, `SITE_URL`, `API_EXTERNAL_URL`) appear in both files with a "KEEP IN SYNC" header comment.
- **New `reset-supabase.sh`** and **`reset-apps.sh`** replacing `resetsupabasedb.sh`. Each script targets exactly one compose file.
- **Modified `docker-compose.test.yml`** to use the same external `supabase_network` pattern and contain only test-scoped apps (no Supabase services declared in the test file).
- **Removed `docker-compose.yml`**, **`docker-compose.override.example.yml`**, **`resetsupabasedb.sh`**, and **`volumes/db/init/00-setup-roles.sql.template`**.
- **Modified `.gitignore`** to ignore the two new `.env` files.

## Capabilities

### Modified Capabilities

- `local-dev-environment`: the "one-command bring-up" and "single env file" requirements no longer apply. Replaced by "two-compose bring-up on a shared external network" and "two per-stack env files with explicit shared-value synchronization". The reset script requirement now expects two scripts. The "hot-reload override" requirement is removed (the override pattern does not apply to two-file setups).

## Impact

- **New files:** `docker-compose.supabase.yaml`, `docker-compose.my-cms.yaml`, `.env.supabase`, `.env.my-cms`, `reset-supabase.sh`, `reset-apps.sh`, `volumes/db/roles.sql`, `volumes/db/jwt.sql`, `volumes/db/webhooks.sql`, `volumes/db/_supabase.sql`, `volumes/db/realtime.sql`, `volumes/db/logs.sql`, `volumes/db/pooler.sql`, `volumes/db/99-my-cms-grants.sql`.
- **Modified files:** `docker-compose.test.yml` (refactored), `.gitignore` (add `.env.supabase`, `.env.my-cms`).
- **Deleted files:** `docker-compose.yml`, `docker-compose.override.example.yml`, `resetsupabasedb.sh`, `volumes/db/init/00-setup-roles.sql.template`, the entire `volumes/db/init/` directory.
- **Runtime:** adds a user-created external Docker network `supabase_network`. Existing named volumes (`supabase_db_data`, `supabase_storage_data`, `mailpit_data`) are reused by `docker-compose.supabase.yaml` so data is preserved through the migration.
- **Developer workflow change:**
  - First time only: `docker network create supabase_network`
  - Start Supabase: `docker compose -f docker-compose.supabase.yaml --env-file .env.supabase up -d`
  - Start Apps (after Supabase is healthy): `docker compose -f docker-compose.my-cms.yaml --env-file .env.my-cms up -d`
  - Reset Supabase only: `./reset-supabase.sh`
  - Reset Apps only: `./reset-apps.sh`
- **Out of scope (handled elsewhere or by follow-up changes):**
  - Production deployment (uses Helm; compose is local-dev only).
  - Application code changes in `apps/api/` or `apps/web/` — the existing `my-cms-api` connects to `db:5432` over Docker DNS, which still resolves.
  - The pre-existing `permission denied for database postgres` storage-admin bug is fixed as a side effect of switching to upstream `roles.sql` + the defensive grant in `99-my-cms-grants.sql`.
