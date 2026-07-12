## Context

The my-cms local dev stack is split into two Docker Compose files — `deployments/docker-swarm/supabase/` (the Supabase side, owns GoTrue) and `deployments/docker-swarm/apps/` (the project side, owns `my-cms-api`). They share seven secret values across two per-stack `.env` files under a `KEEP IN SYNC with …` header. The `local-dev-environment` spec pins this discipline.

In the running stack, the API container calls GoTrue's admin API (`GET /auth/v1/admin/users` and siblings) through a new `SupabaseAdminClient` introduced by the in-flight `add-user-management-admin-page` change. The client sends `Authorization: Bearer <SERVICE_ROLE_KEY>` and `apikey: <SERVICE_ROLE_KEY>`, where `SERVICE_ROLE_KEY` is read from `SUPABASE_SERVICE_ROLE_KEY` at the composition root (`apps/api/src/bin/my-cms-api.rs:243-244`). GoTrue receives that bearer token, treats it as a JWT signed with `JWT_SECRET`, and returns 401 with `{"message":"Invalid authentication credentials"}` if verification fails.

The drift is in `deployments/docker-swarm/apps/.env` line 17:

```
SERVICE_ROLE_KEY=devkey        # <-- literal placeholder, not a JWT
```

while the other five references in the repo all carry a real HS256 JWT signed with the project's `JWT_SECRET`. The mirror template `deployments/docker-swarm/apps/.env.example` has the correct JWT, so a fresh `cp .env.example .env` produces a working stack — but the existing `apps/.env` was created at some point with the placeholder and never updated. The `apps/api/.env.example` template also ships the placeholder, so a developer copying it instead of the docker-swarm template would hit the same bug.

The neighbouring `fix-supabase-internal-url-docker-networking` change (in progress) addressed a different failure mode (the API container reaching its own loopback because `SUPABASE_URL=http://localhost:8000` resolves to `127.0.0.1` inside the container, causing `error sending request for url`). It is not affected by this change and does not need to be modified.

## Goals / Non-Goals

**Goals:**

- Restore the API container's ability to call GoTrue's admin API from the apps Docker Compose stack by aligning `SERVICE_ROLE_KEY` in `deployments/docker-swarm/apps/.env` with the value GoTrue was started with.
- Bring `apps/api/.env.example` in line with the docker-swarm template so a developer copying the wrong template does not reintroduce the bug.
- Make the failure mode testable: a `KEEP IN SYNC` violation on `SERVICE_ROLE_KEY` should be a named scenario in the `local-dev-environment` spec, not a silent regression.
- Keep the fix to a one-line config change per file. No code, no new scripts, no new modules.

**Non-Goals:**

- Introducing a runtime check that fails fast when `SERVICE_ROLE_KEY` is the literal `devkey`. A unit test in `supabase_admin_client` exists for the WireMock-based 401 path; a runtime guard is out of scope for a Fast Fix.
- Refactoring the `KEEP IN SYNC` discipline into a single source of truth (e.g. one canonical `.env` consumed by both sides, or a pre-flight sync script that the reset scripts run). The `KEEP IN SYNC` model has been used in the repo for over a year; replacing it is a larger refactor that belongs in a follow-up change if the team wants it.
- Rotating the JWT or the `JWT_SECRET`. The bug is value-mismatch, not compromise.
- Fixing the `apps/api/.env.example` placeholder for other shared secrets (e.g. `POSTGRES_PASSWORD=devpassword`, `JWT_SECRET=…`). Those values are deliberately the same on both sides today and have not drifted; they are not the bug.
- Touching the in-flight `fix-supabase-internal-url-docker-networking` change. The two changes are independent and can be applied in any order.

## Decisions

### 1. Edit the existing `.env` file in place, do not regenerate the JWT

**Decision:** Copy the JWT from `deployments/docker-swarm/supabase/.env` line 19 into `deployments/docker-swarm/apps/.env` line 17, and into `apps/api/.env.example` line 22. Do not run `scripts/generate-jwt.sh` for this fix.

**Reason:** The Supabase container is already running with a `JWT_SECRET` and a `SERVICE_ROLE_KEY` derived from that secret. Regenerating the JWT with `scripts/generate-jwt.sh` would mint a *new* JWT signed with the same `JWT_SECRET`, but that would also need to be copied into the supabase `.env` and the supabase stack would need to be restarted for GoTrue to pick it up. That is a much larger blast radius for a Fast Fix. Copying the existing JWT is a config edit, not a secret rotation, and does not require restarting GoTrue.

**Alternatives considered:**

- *Run `scripts/generate-jwt.sh` and write the output into both env files* — rejected because the generated JWT has a fresh `iat` / `exp` and would require restarting the Supabase stack to take effect. Out of scope for a Fast Fix.
- *Hardcode a new JWT in the change and require the developer to rotate* — rejected because the current JWT already works; the problem is only that it isn't in the apps `.env`. Rotating when not needed risks invalidating the admin password seeder and any active sessions.
- *Move `SERVICE_ROLE_KEY` into a `secrets/` file mounted into both containers* — rejected as a larger refactor (see Non-Goals).

### 2. Update the template, not just the live file

**Decision:** Also fix `apps/api/.env.example` line 22 so the bug cannot be reintroduced by a new developer copying the template.

**Reason:** `apps/api/.env.example` is the template for `cargo run` on the host. The host-side `.env` and `.env.local` already carry the correct JWT, so the host workflow works. But the template ships the placeholder, so a developer who sets up the project for the first time on a fresh clone (where they only edit `apps/api/.env.example` and copy it to `.env`) will hit the same 401 once they later switch to the Docker stack. Updating the template closes that loophole.

### 3. Pin the failure mode in the `local-dev-environment` spec

**Decision:** Add a new scenario under the existing "Two per-stack env files with shared values synchronised" requirement that exercises the GoTrue admin 401 path when `SERVICE_ROLE_KEY` is the literal `devkey` on one side and a real JWT on the other.

**Reason:** The current scenario in the spec ("Shared value drift", line 40-44 of `openspec/specs/local-dev-environment/spec.md`) only covers `POSTGRES_PASSWORD` drift and a `db` connection failure. The new scenario pins the *exact* failure mode that bit us: a literal `devkey` placeholder on one side produces a 401 from GoTrue's admin API, with the body `{"message":"Invalid authentication credentials"}`. This makes the `KEEP IN SYNC` discipline testable for the seven shared secrets, not just one.

**Alternative considered:** Add a separate "SERVICE_ROLE_KEY must be a real JWT" requirement under a new capability like `supabase-auth`. Rejected because the bug is about env-file sync discipline, which is owned by `local-dev-environment`; the JWT-vs-placeholder question is one specific instance of the broader drift problem.

### 4. No automated test, only a manual smoke

**Decision:** Verification is a single `curl` against `GET /users` after the API container is restarted. No `cargo test` is added; no new unit test is added in `supabase_admin_client`.

**Reason:** The existing WireMock-based test in `supabase_admin_client.rs:543-582` already covers the 401 path: when GoTrue returns 401 with a `{"message":"Invalid authentication credentials"}` body, the client surfaces `AppError::Logical("GoTrue list users authorisation error (401 Unauthorized): …")`. The bug is not in the client logic; it is in the env var the client reads. An env-var test would require a process-spawning integration test against the real Supabase stack, which is heavier than the bug warrants.

**Alternative considered:** Add a unit test that asserts `apps/.env`'s `SERVICE_ROLE_KEY` is not the literal string `devkey`. Rejected as fragile (a string-match test is easy to bypass with `devkey1` or `Service-Role-Key`) and not in the spirit of a Fast Fix.

## Risks / Trade-offs

- **The JWT in `supabase/.env` has a 1-year `exp`** → it will eventually expire and need regeneration via `scripts/generate-jwt.sh`. *Mitigation:* the script's header documents this; a future change can wire a pre-flight check into `reset-supabase.sh` if expiry tracking becomes a recurring issue. Out of scope here.
- **`apps/api/.env.example` and the docker-swarm `apps/.env.example` carry the same JWT** → if a developer rotates the JWT in one but not the other, the template and the live file drift. *Mitigation:* both files already have the same `KEEP IN SYNC` header; the new scenario in the spec makes drift detectable by an explicit test.
- **A future contributor adds a new shared secret and forgets the `KEEP IN SYNC` header** → the new secret could drift silently. *Mitigation:* out of scope for this change; covered indirectly by the "Two per-stack env files" requirement which already lists the seven known shared vars and the `KEEP IN SYNC` discipline.
- **Editing `apps/api/.env.example` does not affect anyone running the host-side workflow today** (their `.env` and `.env.local` already have the JWT) → there is no risk of regressing an existing setup. *Mitigation:* the edit is purely additive to the template.
- **Editing `deployments/docker-swarm/apps/.env` requires the API container to be restarted** to pick up the new value. *Mitigation:* `docker compose -f deployments/docker-swarm/apps/docker-compose.yaml up -d my-cms-api` is a one-liner, called out in the manual smoke.

## Migration Plan

This is a non-breaking, additive change. The migration is a single `git pull` + container restart:

1. Pull the change.
2. `docker compose -f deployments/docker-swarm/apps/docker-compose.yaml --env-file deployments/docker-swarm/apps/.env up -d my-cms-api` (restarts the API container so the new `SERVICE_ROLE_KEY` is loaded).
3. Smoke: `curl -i -H "Authorization: Bearer <admin-jwt>" http://localhost:8989/users` → 200 with the user list.
4. The Supabase container is *not* restarted; it keeps its existing `SERVICE_ROLE_KEY` because the new value matches.

**Rollback:** revert the two `.env` line edits. The original 401 returns. No data migration, no DB migration, no frontend deploy.

## Open Questions

None. The fix is mechanical, the failure mode is fully diagnosed, and the spec delta is small. A future change could introduce a pre-flight sync check (or a single-source `.env`) but that is out of scope here.
