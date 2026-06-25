# Fix SUPABASE_URL Split-Horizon Docker Networking

## Why

The new user-management admin page (change `add-user-management-admin-page`) calls GoTrue's `/auth/v1/admin/users` endpoints from inside the `my-cms-api` Docker container via `SupabaseAdminClient`. Today every call fails with:

```
GoTrue list users request failed: error sending request for url (http://localhost:8000/auth/v1/admin/users?page=1&per_page=200)
```

The API container's `SUPABASE_URL=http://localhost:8000` resolves **to the API container itself** (`172.18.0.15`) — not to `supabase-kong` (`172.18.0.11`). Nothing listens on port 8000 inside the API container, so `reqwest` reports `error sending request for url`. From the **host**, `localhost:8000` works because `EXPOSE_KONG_PORT=8000` in `supabase/.env` binds Kong to the host loopback.

The same split-horizon bug already lives in `SupabaseStorage` (`commands/media/supabase_storage.rs:80,87,…`) — it builds `{supabase_url}/storage/v1/...` URLs that fail the same way from the API container. Media uploads have been silently broken for the same reason; the user-management feature is what surfaced the issue because it is the first code path that exercises an outgoing HTTP call from the API container at request time.

JWT auth still works because `SupabaseAuthLayer` validates HS256 with a shared `JWT_SECRET` first (`common/supabase_auth.rs:496`); the JWKS fallback at line 510 is never triggered for HS256 tokens. So the bug is invisible to auth and visible to storage + admin operations.

The fix is a deployment/wiring change: introduce a Docker-internal URL the backend uses to reach Kong, while keeping the host-facing URL for the browser/frontend. This is a configuration-level fix — no production code logic changes (other than one env-var resolution at the composition root).

## What Changes

- **Add a new env var `SUPABASE_INTERNAL_URL`** consumed only by the backend (`apps/api`). It is the URL the API container uses to reach Kong via the shared Docker network. It defaults to `SUPABASE_URL` when unset, preserving backward compatibility for host-side `cargo run` workflows.
- **Pass `SUPABASE_INTERNAL_URL` to the backend composition root** in `apps/api/src/bin/my-cms-api.rs` for the three clients that originate outbound HTTP requests to Supabase: `SupabaseStorage` (storage API), `SupabaseAdminClient` (GoTrue admin), and `SupabaseAuthLayer` (JWKS fallback). All three currently use `SUPABASE_URL`, which is the source of the bug.
- **Set `SUPABASE_INTERNAL_URL=http://supabase-kong:8000`** in the Docker-Compose apps `.env` so the API container reaches Kong by its Docker service hostname. The frontend and host-side scripts keep `SUPABASE_URL=http://localhost:8000`.
- **No changes** to frontend env vars (`PUBLIC_SUPABASE_URL` unchanged), to `SupabaseAdminClient`/`SupabaseStorage` code (they continue to read a single `supabase_url` field; only the *value* changes), or to `SupabaseAuthLayer` JWT logic.
- **No new capability is introduced.** The requirement belongs in the existing `local-dev-environment` capability (which already describes the Supabase/apps Compose split and shared network).

## Capabilities

### Modified

- **`local-dev-environment`**: Add a requirement that the API container resolves Supabase via an **internal** URL distinct from the **public** URL the browser uses. Specifically: the API container's outbound calls to Kong/GoTrue/Storage SHALL use the Docker service hostname `supabase-kong:8000` (or `SUPABASE_INTERNAL_URL` env value), not `localhost:8000`. The frontend's `PUBLIC_SUPABASE_URL` and host-side scripts SHALL continue to use the host-exposed URL.

### New

None. This is a deployment-config fix, not a new feature.

## Impact

| Layer | Impact |
|---|---|
| **API composition root** | `apps/api/src/bin/my-cms-api.rs` — `construct_app_state()` reads `SUPABASE_INTERNAL_URL` (defaulting to `SUPABASE_URL`); the value is passed to `SupabaseStorage::new()` and `SupabaseAdminClient::new()`. `construct_supabase_auth_layer()` likewise reads `SUPABASE_INTERNAL_URL`. **One-line change per client.** |
| **Env files** | `deployments/docker-swarm/apps/.env.example` + `.env` — add `SUPABASE_INTERNAL_URL=http://supabase-kong:8000`. Update the "KEEP IN SYNC" header comments to reference the new var. |
| **Docker Compose** | `deployments/docker-swarm/apps/docker-compose.yaml` — pass `SUPABASE_INTERNAL_URL` to the `my-cms-api` service. Pattern matches the existing `${VAR:-${DEFAULT}}` fallback used for `MAX_BODY_LENGTH` etc. |
| **Host-side dev `.env`** | `apps/api/.env.example` + `.env` — add a documented `SUPABASE_INTERNAL_URL=http://localhost:8000` line (or comment explaining it defaults to `SUPABASE_URL`). No behavior change for host-side `cargo run`. |
| **Frontend** | None. `apps/web/.env*` keep `PUBLIC_SUPABASE_URL` as the browser-direct URL. |
| **Supabase / Traefik** | None. `supabase/.env*` and `traefik/.env*` are untouched. |
| **Existing specs** | `local-dev-environment` gains one Requirement + 2-3 Scenarios. `supabase-storage` and `supabase-auth` reference `{SUPABASE_URL}` generically; the *resolved* value now depends on whether the caller is the API container (uses internal URL) or the host/frontend (uses public URL). No requirement text changes — both still resolve through `SupabaseStorage.supabase_url` / `SupabaseAuthConfig.supabase_url`, just sourced from a different env var at the composition root. |
| **Tests** | The 26 WireMock-based `SupabaseAdminClient` unit tests and the `SupabaseStorage` tests are unaffected (they inject their own base URL). The new wiring is verified by `cargo check` + `cargo test` + `pnpm build`. Manual smoke (run `curl /users` from the host → 200; verify media upload → 200) is the only integration check needed. |
| **Risk** | Very low. The new env var is additive and defaults to the old behavior. Misconfiguration (e.g. forgetting to set it for production) falls back to the old broken state — same failure mode as today, not worse. |

## Non-Goals

- **Re-architecting service discovery** (Consul, Kubernetes DNS, Traefik routing for backend-to-Kong). The split-horizon env var is the simplest fix that closes the bug; a service mesh is out of scope.
- **Changing the frontend's URL** (`PUBLIC_SUPABASE_URL` is correct for browser-direct GoTrue auth and stays as-is).
- **Removing `localhost:8000` from any config.** It remains valid for: the frontend, host-side scripts (`reset-supabase.sh`, seeder), and any future `cargo run` workflow without Docker.
- **Adding tests for the env-var resolution.** The composition root is a 3-line read with a fallback; it's exercised by the existing boot + manual smoke. Adding a unit test for `.unwrap_or_else(|_| …)` is not high-value.
- **Fixing the pre-existing 15 clippy errors** in unrelated files (tracked separately in `add-user-management-admin-page` task 14 resume notes).

## Open Questions

None — the design is straightforward and follows the existing `API_EXTERNAL_URL` precedent (the apps `.env` already carries both a public `SUPABASE_URL` and a `API_EXTERNAL_URL` that Kong uses to know its public URL; this change adds the symmetric internal-URL pair).

## Verification

```bash
# 1. Apply the env + compose changes; restart API
docker compose -f deployments/docker-swarm/apps/docker-compose.yaml --env-file deployments/docker-swarm/apps/.env up -d my-cms-api

# 2. Confirm the env landed inside the container
docker exec my-cms-api printenv SUPABASE_INTERNAL_URL   # → http://supabase-kong:8000

# 3. User management endpoints now resolve through the Docker network
curl -i -H "Authorization: Bearer <admin-jwt>" http://localhost:8989/users
# Expected: 200 with the seeded admin user (was 500 with "error sending request")

# 4. Storage endpoints also resolve (latent bug closed)
curl -i -H "Authorization: Bearer <admin-jwt>" http://localhost:8989/media?prefix=
# Expected: 200 with object list

# 5. Standard verify gate (no logic changes, but ensure no regression)
cd apps/api && cargo check && cargo test && cargo fmt -- --check
pnpm --dir apps/web build
```
