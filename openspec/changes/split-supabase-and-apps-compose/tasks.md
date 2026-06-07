## 1. Vendor upstream Supabase init SQL

- [ ] 1.1 Fetch `volumes/db/roles.sql`, `volumes/db/jwt.sql`, `volumes/db/webhooks.sql`, `volumes/db/_supabase.sql`, `volumes/db/realtime.sql`, `volumes/db/logs.sql`, `volumes/db/pooler.sql` from `supabase/supabase` `master/docker/volumes/db/` on GitHub and save them into `volumes/db/` in this repo (no edits, no reformatting)
- [ ] 1.2 Create `volumes/db/99-my-cms-grants.sql` with the two defensive statements (`GRANT CREATE ON DATABASE postgres TO supabase_storage_admin;` and `CREATE EXTENSION IF NOT EXISTS vector WITH SCHEMA public;`) plus a header comment explaining the rationale

## 2. Create the Supabase compose file

- [ ] 2.1 Create `docker-compose.supabase.yaml` at the repo root, derived from `supabase/supabase` `master/docker/docker-compose.yml` with the minimal service set (`db`, `supavisor`, `auth`, `rest`, `realtime`, `storage`, `imgproxy`, `meta`, `studio`, `kong`, `mailpit`)
- [ ] 2.2 Pin image tags to the versions the project already uses: `supabase/postgres:15.14.1.132`, `supabase/gotrue:v2.179.0`, `postgrest/postgrest:v14.1`, `supabase/realtime:v2.67.1`, `supabase/storage-api:v1.60.2`, `darthsim/imgproxy:v4.0.3`, `supabase/postgres-meta:v0.96.6`, `supabase/studio:2026.05.25-sha-65c570e`, `kong:2.8.1`, `supabase/supavisor:2.7.4`, `axllent/mailpit:v1.23`
- [ ] 2.3 Mount the eight init SQL files (seven upstream + one custom) into the standard `/docker-entrypoint-initdb.d/init-scripts/` and `/docker-entrypoint-initdb.d/migrations/` paths per the upstream layout
- [ ] 2.4 Declare the external `supabase_network` (matches `supabase_network` in the apps compose)
- [ ] 2.5 Set `MP_SMTP_AUTH_ACCEPT_ANY: "true"` and `MP_SMTP_AUTH_ALLOW_INSECURE: "true"` on Mailpit to preserve the current dev convenience
- [ ] 2.6 Validate with `docker compose -f docker-compose.supabase.yaml config` (must exit 0)

## 3. Create the apps compose file

- [ ] 3.1 Create `docker-compose.my-cms.yaml` at the repo root with the apps services: `init-wait`, `migrate`, `my-cms-api`, `my-cms-frontend`, `jaeger`
- [ ] 3.2 Implement the `init-wait` service (alpine, polls `nc -z db 5432` in a 2-second loop, exits 0 on success)
- [ ] 3.3 Set `migrate` to `depends_on: init-wait: service_completed_successfully`, `restart: "no"`, `entrypoint: ["/app/migration"]`, `command: ["up"]`
- [ ] 3.4 Set `my-cms-api` to `depends_on: migrate: service_completed_successfully` and `jaeger: service_started`
- [ ] 3.5 Set `my-cms-frontend` to `depends_on: my-cms-api: service_started`
- [ ] 3.6 Declare the external `supabase_network` (same name as in the Supabase compose)
- [ ] 3.7 Remove the `init-config` service and any reference to `db_init_rendered` volume
- [ ] 3.8 Validate with `docker compose -f docker-compose.my-cms.yaml config` (must exit 0)

## 4. Create the per-stack env files

- [ ] 4.1 Create `.env.supabase.example` (template, gitignored copy will be `.env.supabase`) starting from the current `.env` values and adding the upstream Supabase variables: `POSTGRES_HOST=db`, `POSTGRES_PORT=5432`, `POSTGRES_DB=postgres`, `KONG_HTTP_PORT=8000`, `KONG_HTTPS_PORT=8443`, `STUDIO_DEFAULT_ORGANIZATION`, `STUDIO_DEFAULT_PROJECT`, `SECRET_KEY_BASE`, `VAULT_ENC_KEY`, `PG_META_CRYPTO_KEY`, `STORAGE_TENANT_ID=stub`, `GLOBAL_S3_BUCKET=stub`, `REGION=local`, `POOLER_TENANT_ID=your-tenant-id`, `POOLER_DEFAULT_POOL_SIZE=15`, `POOLER_MAX_CLIENT_CONN=100`, `POOLER_PROXY_PORT_TRANSACTION=6543`, `PGRST_DB_SCHEMAS=public,storage,graphql_public`, `PGRST_DB_MAX_ROWS=1000`, `JWT_EXPIRY=3600`, `IMGPROXY_AUTO_WEBP=true`, `DASHBOARD_USERNAME`, `DASHBOARD_PASSWORD`
- [ ] 4.2 Create `.env.my-cms.example` (template) carrying only the apps-compose vars: `DATABASE_URL`, `DATABASE_SCHEMA`, `HOST`, `PORT`, `RUST_LOG`, `ENABLED_OTLP_EXPORTER`, `OTEL_*`, `OPENAI_API_KEY`, `MEDIA_BASE_URL`, `MAX_BODY_LENGTH`, `SUPABASE_URL`, `SUPABASE_ANON_KEY`, `SUPABASE_SERVICE_ROLE_KEY`, `SUPABASE_JWT_SECRET`, `SUPABASE_STORAGE_BUCKET`, `AUTHORIZATION_AUDIENCE`, `PUBLIC_*`
- [ ] 4.3 Add `KEEP IN SYNC with .env.{other}` header comments to `POSTGRES_PASSWORD`, `JWT_SECRET`, `ANON_KEY`, `SERVICE_ROLE_KEY`, `SUPABASE_PUBLIC_URL`, `SITE_URL`, `API_EXTERNAL_URL` in both files
- [ ] 4.4 Add `.env.supabase` and `.env.my-cms` to `.gitignore`

## 5. Create the reset scripts

- [ ] 5.1 Create `reset-supabase.sh` that runs `docker compose -f docker-compose.supabase.yaml --env-file .env.supabase down -v --remove-orphans`, then `docker volume rm -f supabase_db_data supabase_storage_data mailpit_data 2>/dev/null || true`, then `docker compose -f docker-compose.supabase.yaml --env-file .env.supabase up -d`
- [ ] 5.2 Create `reset-apps.sh` that runs `docker compose -f docker-compose.my-cms.yaml --env-file .env.my-cms down -v --remove-orphans`, then brings the apps stack back up
- [ ] 5.3 Make both scripts executable (`chmod +x`)
- [ ] 5.4 Add a trailing `echo` block to each script that prints the access URLs developers expect (Studio `:8000`, API `:8989`, etc.)

## 6. Refactor the test compose

- [ ] 6.1 Remove Supabase service declarations (`db`, `auth`, `rest`, `storage`, `imgproxy`, `meta`, `studio`, `kong`, `mailpit`) from `docker-compose.test.yml`
- [ ] 6.2 Keep `migrate`, `my-cms-api`, `mailpit` (or drop mailpit if the test stack doesn't need it) and add the `init-wait` service
- [ ] 6.3 Declare the external `supabase_network` in the test compose
- [ ] 6.4 Validate with `docker compose -f docker-compose.test.yml config` (must exit 0)

## 7. Update `.gitignore`

- [ ] 7.1 Add `.env.supabase` and `.env.my-cms` to `.gitignore`
- [ ] 7.2 Confirm `.env` (the old single file) is still ignored (no change required)

## 8. Remove obsolete files

- [ ] 8.1 Delete `docker-compose.yml` (the old monolithic file)
- [ ] 8.2 Delete `docker-compose.override.example.yml`
- [ ] 8.3 Delete `resetsupabasedb.sh`
- [ ] 8.4 Delete `volumes/db/init/00-setup-roles.sql.template` and the entire `volumes/db/init/` directory

## 9. Verify

- [ ] 9.1 Run `docker compose -f docker-compose.supabase.yaml config --quiet` and `docker compose -f docker-compose.my-cms.yaml config --quiet` and `docker compose -f docker-compose.test.yml config --quiet` — all three must exit 0
- [ ] 9.2 Stop the running stack with `docker compose down -v` on the OLD `docker-compose.yml` (if still present) or stop existing containers manually
- [ ] 9.3 `docker network create supabase_network` (one-time, skip if it already exists)
- [ ] 9.4 `docker compose -f docker-compose.supabase.yaml --env-file .env.supabase up -d` and wait for `docker compose -f docker-compose.supabase.yaml ps` to show every service `running` or `healthy`
- [ ] 9.5 Confirm `docker compose -f docker-compose.supabase.yaml logs --tail=100 storage` does NOT contain the `SQLSTATE 42501 (aclcheck_error): permission denied for database postgres` message
- [ ] 9.6 Confirm `docker compose -f docker-compose.supabase.yaml logs --tail=100 auth` shows a successful boot (not the same 42501 error)
- [ ] 9.7 `docker compose -f docker-compose.my-cms.yaml --env-file .env.my-cms up -d` and wait for `docker compose -f docker-compose.my-cms.yaml ps` to show `init-wait: exited (0)`, `migrate: exited (0)`, `my-cms-api: running`, `my-cms-frontend: running`, `jaeger: running`
- [ ] 9.8 Open `http://localhost:8000` in a browser and log in to Supabase Studio with `admin` / `admin`
- [ ] 9.9 `curl http://localhost:8989/health` returns `{"status":"ok"}`
- [ ] 9.10 Open `http://localhost:3002` and confirm the React admin panel loads
- [ ] 9.11 Open `http://localhost:16686` and confirm Jaeger UI loads
- [ ] 9.12 Open `http://localhost:8025` and confirm Mailpit UI loads
- [ ] 9.13 Run `./reset-supabase.sh` and confirm the Supabase stack comes back to healthy
- [ ] 9.14 Run `./reset-apps.sh` and confirm the apps stack comes back to healthy

## 10. Commit

- [ ] 10.1 `git add openspec/changes/split-supabase-and-apps-compose/`
- [ ] 10.2 `git add docker-compose.supabase.yaml docker-compose.my-cms.yaml docker-compose.test.yml`
- [ ] 10.3 `git add volumes/db/ reset-supabase.sh reset-apps.sh .env.supabase.example .env.my-cms.example .gitignore`
- [ ] 10.4 `git rm docker-compose.yml docker-compose.override.example.yml resetsupabasedb.sh volumes/db/init/00-setup-roles.sql.template`
- [ ] 10.5 `git commit -m "feat(env): split docker-compose into standalone supabase + apps files on shared external network"`
