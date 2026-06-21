## Context

The my-cms project's local development environment is defined by a single `docker-compose.yml` that brings up the full self-hosted Supabase stack alongside the Rust API, the rsbuild dev server, and Jaeger. This file was introduced by the previous `unified-docker-compose-with-supabase` change and is currently working — except for one well-known failure mode: the `supabase-storage` container crash-loops on startup with `SQLSTATE 42501 (aclcheck_error): permission denied for database postgres`.

The root cause is that the project's hand-rolled init script `volumes/db/init/00-setup-roles.sql.template` is missing one grant (`GRANT CREATE ON DATABASE postgres TO supabase_storage_admin;`) that the storage-api v1.60.x tenant migration needs in order to `CREATE SCHEMA storage`. The `supabase/postgres` base image's own init would grant this — but the project's custom script runs in the same `/docker-entrypoint-initdb.d` directory and was assumed to be additive; in practice, it overrides the image's default permission flow because the volume is initialised before any patch can be applied to the script.

Beyond the bug, two structural issues are pushing the team toward a split:

1. **Drift from upstream Supabase.** Upstream's `docker/docker-compose.yml` is the canonical reference for self-hosted Supabase. Their recommended init flow uses raw SQL files (no template substitution) mounted into `/docker-entrypoint-initdb.d/init-scripts/` and `/docker-entrypoint-initdb.d/migrations/`. The project's custom `init-config` envsubst service and `.template` files diverge from this layout, which means Supabase documentation, GitHub issues, and snippets do not apply directly.

2. **Coupled lifecycles.** The Supabase stack and the my-cms apps have independent upgrade cadences, different audiences (Supabase is a platform dependency, my-cms is the product), and may eventually be partially replaced by managed services. Locking them into one Compose file means every change touches both halves.

This change refactors the local environment into two standalone Compose files joined only by an external Docker network, and adopts the upstream Supabase init pattern (vendor the upstream SQL files, drop the envsubst pipeline).

## Goals / Non-Goals

**Goals:**

- Two standalone Compose files (`docker-compose.supabase.yaml`, `docker-compose.my-cms.yaml`) that can be started, stopped, and reset independently.
- The Supabase file mirrors upstream `supabase/supabase` `master/docker/docker-compose.yml` as closely as practical: same services, same env-var names, same init-SQL layout, same volume conventions.
- The my-cms apps file contains only project-owned services (`init-wait`, `migrate`, `my-cms-api`, `my-cms-frontend`, `jaeger`) and depends on the Supabase stack solely by DNS on a shared external network.
- The pre-existing `permission denied for database postgres` storage-admin bug is fixed as a side effect of switching to upstream `roles.sql` plus a defensive grant in `99-my-cms-grants.sql`.
- Two reset scripts (`reset-supabase.sh`, `reset-apps.sh`) target one compose file each.
- Two `.env` files (`.env.supabase`, `.env.my-cms`) drive their respective compose files. Shared values appear in both with a `KEEP IN SYNC` header.
- `docker-compose.test.yml` is refactored to use the same external-network pattern and contains only the test-scoped apps.

**Non-Goals:**

- Adopting every upstream service (e.g., `functions` edge runtime, `analytics`/Logflare) — out of scope unless an explicit need arises.
- Production deployment (uses Helm; compose is local-dev only).
- Changing `my-cms-api` to point at a different host, port, or protocol — the existing `DATABASE_URL=postgresql://supabase_admin:${POSTGRES_PASSWORD}@db:5432/postgres` still resolves under the new layout because `db` is the Supabase compose's container hostname, visible on the shared network.
- Replacing the local Supabase stack with a managed Supabase project — that is a larger change involving auth/URL migration and is tracked separately.

## Decisions

### Decision 1: Two standalone Compose files joined by an external `supabase_network`

Both files declare:

```yaml
networks:
  supabase_network:
    external: true
    name: supabase_network
```

The user creates the network once (`docker network create supabase_network`) and both compose files attach to it. The my-cms apps file references the Supabase compose's container hostnames (`db`, `auth`, `storage`, `kong`, `realtime`, `meta`, `studio`, `imgproxy`, `mailpit`, `supavisor`) for service discovery.

**Rationale:** maximum decoupling — either side can be brought up, replaced, or torn down without affecting the other; matches the upstream Supabase reference layout for their `docker-compose.yml` (the upstream file declares its own network; layering our own on top is the standard pattern). **Alternatives considered:** (a) `include:` directive in the apps file — rejected because it requires Compose v2.20+ and creates implicit coupling that defeats the split's purpose; (b) multi-file `-f` flags — rejected because the user must remember the file order on every command and there's no clear "this is the supabase one" handle.

### Decision 2: Adopt upstream Supabase init SQL (vendor the files, drop envsubst)

The Supabase compose file mounts the upstream-blessed SQL files into the standard locations:

| Mount source | Container target |
|---|---|
| `./volumes/db/roles.sql` | `/docker-entrypoint-initdb.d/init-scripts/99-roles.sql` |
| `./volumes/db/jwt.sql` | `/docker-entrypoint-initdb.d/init-scripts/99-jwt.sql` |
| `./volumes/db/webhooks.sql` | `/docker-entrypoint-initdb.d/init-scripts/98-webhooks.sql` |
| `./volumes/db/_supabase.sql` | `/docker-entrypoint-initdb.d/migrations/97-_supabase.sql` |
| `./volumes/db/realtime.sql` | `/docker-entrypoint-initdb.d/migrations/99-realtime.sql` |
| `./volumes/db/logs.sql` | `/docker-entrypoint-initdb.d/migrations/99-logs.sql` |
| `./volumes/db/pooler.sql` | `/docker-entrypoint-initdb.d/migrations/99-pooler.sql` |
| `./volumes/db/99-my-cms-grants.sql` | `/docker-entrypoint-initdb.d/migrations/99-my-cms-grants.sql` |

The `init-config` envsubst service and `00-setup-roles.sql.template` are removed. Password interpolation in `roles.sql` is handled by upstream's psql-variable pattern (`\set pgpass` ``echo "$POSTGRES_PASSWORD"`` ALTER USER ... PASSWORD :'pgpass').

**Rationale:** the upstream `supabase/postgres` image is built to work with this exact layout; matching it means Supabase's GitHub issues, docs, and version notes apply without translation. **Alternatives considered:** (a) keep the custom `init-config` pipeline and add only the missing grant — rejected because it leaves us coupled to a hand-rolled init that diverges from upstream; (b) write a new init from scratch — rejected, exactly the problem we're moving away from.

### Decision 3: `99-my-cms-grants.sql` is a small, defensive file

The my-cms-specific init file contains only:

```sql
-- Defensive: ensure supabase_storage_admin can CREATE the storage schema
-- during its tenant migration. The supabase/postgres image's init should
-- grant this, but the storage-api v1.60.x migration emits SQLSTATE 42501
-- (aclcheck_error) without it. Idempotent.
GRANT CREATE ON DATABASE postgres TO supabase_storage_admin;

-- Defensive: ensure pgvector is available for the my-cms AI translation
-- feature. The supabase/postgres image ships with it, but we add this to
-- guard against image downgrades.
CREATE EXTENSION IF NOT EXISTS vector WITH SCHEMA public;
```

**Rationale:** idempotent, minimal, and easy to reason about. Any future my-cms-specific grants (a custom role, a custom extension) get appended here. **Alternatives considered:** putting grants directly into `roles.sql` — rejected, we want to track upstream `roles.sql` verbatim and re-vendor cleanly when upstream changes.

### Decision 4: Apps compose uses an `init-wait` service in place of cross-file `depends_on`

The my-cms apps file has no `depends_on: db` reference (Compose cannot gate on a service declared in a different file). Instead, an `init-wait` service polls `db:5432` until the port accepts a TCP connection:

```yaml
init-wait:
  image: alpine:3.20
  restart: "no"
  networks:
    - supabase_network
  entrypoint: ["sh", "-c"]
  command:
    - |
      apk add --no-cache netcat-openbsd >/dev/null &&
      until nc -z db 5432; do
        echo 'init-wait: waiting for db:5432…';
        sleep 2;
      done &&
      echo 'init-wait: db is up'
```

`migrate` declares `depends_on: init-wait: service_completed_successfully` and `my-cms-api` declares `depends_on: migrate: service_completed_successfully` (same-file health gate).

**Rationale:** preserves the existing startup ordering (migrate after db, api after migrate) without violating Compose's per-file dependency model. **Alternatives considered:** (a) have `migrate` retry internally with `restart: on-failure` and remove the gate entirely — rejected because retry-only makes `docker compose ps` output ambiguous and conflates "migrate succeeded" with "migrate failed and is retrying"; (b) hard-block the apps compose until the user manually starts Supabase — rejected, we want `docker compose -f docker-compose.my-cms.yaml up -d` to "just work" once Supabase is up.

### Decision 5: Two `.env` files with shared values duplicated and labelled

`.env.supabase` carries the variables consumed by the Supabase compose (Supabase service env, Kong, Supavisor, Mailpit, Studio, Realtime, Storage). `.env.my-cms` carries the variables consumed by the apps compose (DATABASE_URL, RUST_LOG, OTLP, OpenAI, frontend build args, etc.).

Shared values (`POSTGRES_PASSWORD`, `JWT_SECRET`, `ANON_KEY`, `SERVICE_ROLE_KEY`, `SUPABASE_PUBLIC_URL`, `SITE_URL`, `API_EXTERNAL_URL`) appear in both files. Each shared value carries a header comment in both files:

```bash
# KEEP IN SYNC with .env.my-cms (same value required on both sides)
POSTGRES_PASSWORD=devpassword
```

**Rationale:** simpler than introducing a third shared file plus env_file composition; easier to grep "what env does this compose need"; values rarely change in practice. **Alternatives considered:** (a) a third `.env.shared` referenced by both files via Compose's `env_file:` list — rejected because Compose's `env_file` merge order is implicit and the resulting matrix (which var wins on a conflict) is non-obvious; (b) one combined `.env` with a giant comment header — rejected because it bloats both files' env context with irrelevant vars.

### Decision 6: Two reset scripts, one per compose file

`reset-supabase.sh` and `reset-apps.sh` each target exactly one compose file and its associated volumes. There is no "reset everything" script — the user runs them in order if they need a full reset.

**Rationale:** matches the standalone-file architecture; you should be able to reset the half you're working on without touching the other. **Alternatives considered:** (a) a single `reset.sh` that calls both — rejected because the common case is "I only changed my-cms code, I don't want to wipe Supabase data"; (b) keep the existing `resetsupabasedb.sh` updated to call both — rejected, same reasoning as (a) plus the name no longer fits.

### Decision 7: `docker-compose.test.yml` is refactored to use the same external-network pattern

The test file drops all Supabase service declarations and keeps only the test-scoped apps (`migrate`, `my-cms-api`, `mailpit`). It declares the same `supabase_network` as external and assumes the developer has already started the Supabase stack.

**Rationale:** tests should not bring up a full Supabase stack — that's expensive and unnecessary if a healthy one is already running. **Alternatives considered:** keep the test file self-contained with a miniature Supabase — rejected, we want the test path to be "I have Supabase up, I want to run my-cms-api against it".

### Decision 8: Drop the `docker-compose.override.example.yml` override pattern

The override pattern only works for single-file Compose; with two files, the user would need an override per file, plus a way to declare the override on the CLI. The two-file layout is already small enough that the few hot-reload bindings (rsbuild source bind-mount, API source bind-mount) can be inlined as commented-out examples in the apps file.

**Rationale:** fewer moving parts; no risk of contributors committing `docker-compose.override.yml`; the per-developer customization surface shrinks. **Alternatives considered:** ship two override templates (`docker-compose.supabase.override.example.yml`, `docker-compose.my-cms.override.example.yml`) — rejected, the hot-reload need is apps-only.

## Risks / Trade-offs

- **Two commands to bring up the stack.** Mitigation: README and the `reset-supabase.sh` script both print the second command after the first completes. A one-liner (`docker compose -f supabase.yaml up -d && docker compose -f apps.yaml up -d`) works for the common case.
- **Shared env values can drift between `.env.supabase` and `.env.my-cms`.** Mitigation: a header comment on each shared value in both files. A drift causes auth or DB connection failures that are immediately obvious in the logs.
- **External `supabase_network` is created out-of-band.** Mitigation: the README documents the one-time `docker network create supabase_network` step. The reset scripts do not touch the network.
- **Cross-file health gating is weaker than single-file `depends_on`.** The `init-wait` service replaces it but is a polling loop, not a Docker health gate. Mitigation: poll interval is 2s, the loop is bounded by the developer's patience.
- **Adopting upstream Supabase init SQL means re-vendoring when upstream changes.** Mitigation: the file layout mirrors upstream exactly, so a `git diff` against `upstream/master docker/volumes/db/` is trivial. A bump is a single PR.
- **Image versions still drift from upstream.** The Supabase compose pins the same tags the project already uses, not the latest upstream tags. Mitigation: bumping is a routine maintenance task tracked outside this change.
- **The defensive grants in `99-my-cms-grants.sql` may be redundant with the image's init.** Mitigation: `GRANT CREATE ON DATABASE` is a no-op if the role already has it; `CREATE EXTENSION IF NOT EXISTS` is idempotent. The redundancy is intentional — defensive against image downgrades.
- **The test compose assumes Supabase is already up.** Mitigation: a CI integration job must start Supabase before invoking the test compose; this is a CI-script change, not a code change in this repo.

## Migration Plan

The migration from the current single-file layout to the two-file layout is performed by executing `tasks.md` (Section 5) in order. Concretely:

1. **Pre-flight:** confirm the user has the named volumes `supabase_db_data`, `supabase_storage_data`, `mailpit_data` already initialised (data must survive the migration). Capture `.env` values into a scratch buffer.
2. **Vendor** the upstream Supabase init SQL files into `volumes/db/`.
3. **Add** `99-my-cms-grants.sql`.
4. **Create** `docker-compose.supabase.yaml` based on upstream with my-cms image tags.
5. **Create** `docker-compose.my-cms.yaml` with the apps + `init-wait`.
6. **Create** `.env.supabase` (copy `.env` and add the upstream Supabase vars that were missing) and `.env.my-cms` (carve out the my-cms-only vars; carry shared values into both).
7. **Refactor** `docker-compose.test.yml`.
8. **Create** `reset-supabase.sh` and `reset-apps.sh`.
9. **Verify** with `docker compose -f docker-compose.supabase.yaml config` (exits 0) and `docker compose -f docker-compose.my-cms.yaml config` (exits 0). Then bring both up and confirm `docker compose -f docker-compose.supabase.yaml ps` shows every service `running`/`healthy`, and `docker compose -f docker-compose.my-cms.yaml ps` shows the same.
10. **Delete** `docker-compose.yml`, `docker-compose.override.example.yml`, `resetsupabasedb.sh`, `volumes/db/init/00-setup-roles.sql.template`, and `volumes/db/init/`.
11. **Commit** the change in a single commit (or two: layout + cleanup) referencing the OpenSpec change id.

## Open Questions

- None. The two design questions raised during exploration (which upstream services to include, where to place the `migrate` service) are resolved by Decision 1 and Decision 4 above.
