## 1. Environment template

- [x] 1.1 Create `.env.example` at the project root with all Supabase, my-cms API, frontend, Mailpit, and OTLP variables (Task 0.1)

## 2. Compose services

- [x] 2.1 Create `docker-compose.yml` with the full Supabase stack (db, supavisor, auth, rest, realtime, storage, imgproxy, meta, studio, kong, analytics, mailpit), plus `jaeger`, `my-cms-api`, and `my-cms-frontend` on `supabase_network` (Task 0.2)
- [x] 2.2 Create `volumes/api/kong.yml` with the declarative gateway routes for `/auth/v1`, `/rest/v1`, `/realtime/v1`, `/storage/v1`, `/analytics/v1`, `/pg/`, `/graphql/v1` (Task 0.3)
- [x] 2.3 Create `volumes/db/init/00-setup-roles.sql` to bootstrap the Supabase roles and enable the `vector`, `uuid-ossp`, and `pgcrypto` extensions (Task 0.4)
- [x] 2.4 Create `frontend/Dockerfile.dev` for the rsbuild dev server (Task 0.5)

## 3. Developer convenience

- [x] 3.1 Create `resetsupabasedb.sh` to wipe named volumes and start fresh (Task 0.6)
- [x] 3.2 Create `docker-compose.override.example.yml` for hot-reload overrides (Task 0.7)
- [x] 3.3 Update `.gitignore` to exclude `.env`, `docker-compose.override.yml`, and the `volumes/` mount (Task 0.8)

## 4. Verify

- [x] 4.1 Start the stack with `docker compose up -d` and confirm `docker compose ps` shows all services as `healthy` or `running` (Task 0.9)
- [x] 4.2 Health-check Supabase Studio (`:8000`), Kong (`:8001`), my-cms API (`:8989/health`), Jaeger (`:16686`), and Mailpit (`:8025`) (Task 0.9)
- [x] 4.3 Log in to Supabase Studio with `admin` / `admin` and create a Storage bucket (Task 0.9)
- [x] 4.4 Run `./resetsupabasedb.sh` to confirm a clean reset works end-to-end (Task 0.10)
