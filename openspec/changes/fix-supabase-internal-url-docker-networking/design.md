# Fix SUPABASE_URL Split-Horizon Docker Networking — Design

## Context

The `local-dev-environment` capability (already authored via the `split-supabase-and-apps-compose` change) splits the local stack into two standalone Compose files joined by an external Docker network (`supabase_network`). The my-cms apps Compose (`deployments/docker-swarm/apps/docker-compose.yaml`) and the Supabase Compose (`deployments/docker-swarm/supabase/docker-compose.yaml`) both attach to that network so the `my-cms-api` container can resolve Supabase service hostnames like `db`, `auth`, `kong`, `storage`.

Today, the API container's `SUPABASE_URL=http://localhost:8000` reaches the **API container's own loopback** (`172.18.0.15`), not Kong (`172.18.0.11`). Nothing listens on port 8000 inside the API container, so every outbound call from `SupabaseStorage` and `SupabaseAdminClient` fails with `error sending request for url`. The frontend, the host, and any host-side scripts (`cargo run` outside Docker, `reset-supabase.sh`) use `localhost:8000` and are unaffected because Kong is host-exposed via `EXPOSE_KONG_PORT=8000`.

The bug surfaced with the `add-user-management-admin-page` change, which is the first feature to make a runtime HTTP call from `my-cms-api` to GoTrue's admin API. The latent failure in `SupabaseStorage` was not exercised by the existing media endpoints in a way that surfaced as a hard error (or it was never tested from the Docker path). JWT auth kept working because `SupabaseAuthLayer` validates HS256 with a shared `JWT_SECRET` (`apps/api/src/common/supabase_auth.rs:496`); the JWKS fetch is a fallback that HS256 tokens never trigger.

## Goals / Non-Goals

**Goals:**

- Make all outbound HTTP calls from the `my-cms-api` container (storage, GoTrue admin, JWKS fallback) reach Kong through the shared Docker network.
- Preserve the existing host-side workflow: `cargo run` outside Docker, host-side `reset-supabase.sh`, and the frontend's `PUBLIC_SUPABASE_URL` continue to use `localhost:8000` unchanged.
- One env var, one composition-root read per client. No new abstractions, no new modules.

**Non-Goals:**

- Re-architecting service discovery (Consul, K8s DNS, Traefik routing for backend-to-Kong).
- Changing the frontend's URL (browser-direct GoTrue auth).
- Removing `localhost:8000` from any config — it remains valid for host and frontend.
- Fixing the pre-existing 15 clippy errors in unrelated files (tracked separately).
- Updating the `local-dev-environment` spec's already-drifted port numbers (Studio on 8000, Kong on 8001 — out of scope for this change; the project actually runs Kong on 8000 and Studio on 3000).

## Decisions

### 1. One new env var, not multiple

**Decision:** Add a single `SUPABASE_INTERNAL_URL` env var, used by the API container for all outbound Supabase HTTP calls.

**Reason:** One var, one composition-root read per client. Symmetric with the existing `API_EXTERNAL_URL` precedent — the apps `.env` already carries both a public-facing `SUPABASE_URL` and a `API_EXTERNAL_URL` that Kong uses to learn its own public URL. The same split applies here: external URL = browser/host; internal URL = container-to-container.

**Alternatives considered:**
- *Per-client env vars (`STORAGE_INTERNAL_URL`, `GOTRUE_INTERNAL_URL`)* — rejected because all three clients (`SupabaseStorage`, `SupabaseAdminClient`, `SupabaseAuthLayer`) reach the same Kong endpoint; splitting them only adds noise.
- *Hardcode `http://supabase-kong:8000` in the API binary* — rejected because `cargo run` outside Docker would break (no `supabase-kong` DNS outside the network).
- *Use `host.docker.internal:8000`* — rejected because it requires Docker Desktop's special handling; doesn't generalise to Linux CI or non-Docker workflows.
- *Use Traefik for backend-to-Kong* — rejected because every backend request would then carry a `Host` header and a round-trip through the proxy; the Docker DNS path is one hop and zero config.

### 2. Default-to-`SUPABASE_URL` fallback

**Decision:** When `SUPABASE_INTERNAL_URL` is unset, the API uses `SUPABASE_URL` (the existing behavior). The fallback is implemented at the composition root as `.unwrap_or_else(|_| supabase_url.clone())`.

**Reason:** Backward compatible for anyone running `cargo run` outside Docker, host-side scripts, or any deployment that already has `SUPABASE_URL` pointing at a reachable host. The Docker Compose apps `.env` sets the new var explicitly, so the fallback is only exercised in non-Docker workflows.

**Alternative considered:** *Fail fast if `SUPABASE_INTERNAL_URL` is unset* — rejected because it would break all host-side development for anyone who has not yet updated their `.env`.

### 3. Resolve at the composition root, not inside each client

**Decision:** Read `SUPABASE_INTERNAL_URL` once in `apps/api/src/bin/my-cms-api.rs::construct_app_state()` and pass the resolved value to `SupabaseStorage::new()`, `SupabaseAdminClient::new()`, and `construct_supabase_auth_layer()`. No changes inside `SupabaseStorage`, `SupabaseAdminClient`, or `SupabaseAuthLayer` themselves.

**Reason:** The three clients already accept a `supabase_url` field; their internal logic is correct. The bug is that *the value passed in* is wrong when the caller runs inside Docker. Fixing it at the composition root preserves the layering (clients don't know about env vars or Docker) and keeps the WireMock-based unit tests green (they still inject their own URLs).

**Alternative considered:** *Make `SupabaseStorage` and `SupabaseAdminClient` accept both URLs and choose at call time* — rejected because it leaks environment knowledge into domain types and breaks the layering rule (clients are pure domain code; env resolution is an infrastructure concern).

### 4. Frontend stays on `PUBLIC_SUPABASE_URL`

**Decision:** Do not introduce a frontend-side `PUBLIC_SUPABASE_INTERNAL_URL`. The frontend continues to use `PUBLIC_SUPABASE_URL` for browser-direct GoTrue auth.

**Reason:** The frontend runs in the user's browser, not in a Docker container. `localhost:8000` from a browser is the developer's host loopback, which (with `EXPOSE_KONG_PORT=8000`) routes to Kong correctly. No split-horizon problem on the browser side.

### 5. `SUPABASE_INTERNAL_URL` in `apps/api/.env*` as documentation, not requirement

**Decision:** Add `SUPABASE_INTERNAL_URL=http://localhost:8000` (explicit, commented as "leave unset to use `SUPABASE_URL`") to both `apps/api/.env.example` and `apps/api/.env` so host-side developers see the var exists and understand its purpose. The value matches `SUPABASE_URL` so the behavior is identical to today.

**Reason:** Documentation-by-example. The defaulting behavior makes the var technically optional, but listing it in the template prevents confusion when someone sees `SUPABASE_INTERNAL_URL` referenced in the Compose file and wonders whether to set it on the host too.

## Architecture

```
┌────────────────────────────────────────┐
│ apps/api composition root              │
│ (my-cms-api.rs::construct_app_state)   │
│                                        │
│   SUPABASE_URL           ← env         │
│     (public / browser-facing)          │
│                                        │
│   SUPABASE_INTERNAL_URL  ← env         │
│     ↓ fallback to SUPABASE_URL         │
│                                        │
│   ┌──────────────────────────────┐     │
│   │ SupabaseStorage              │     │
│   │   .supabase_url = INTERNAL   │     │
│   │   calls {url}/storage/v1/... │     │
│   ├──────────────────────────────┤     │
│   │ SupabaseAdminClient          │     │
│   │   .supabase_url = INTERNAL   │     │
│   │   calls {url}/auth/v1/admin/ │     │
│   ├──────────────────────────────┤     │
│   │ SupabaseAuthLayer            │     │
│   │   .supabase_url = INTERNAL   │     │
│   │   JWKS fallback only         │     │
│   └──────────────────────────────┘     │
└────────────────────────────────────────┘
            │
            │ inside Docker:  → http://supabase-kong:8000
            │ outside Docker: → http://localhost:8000
            ▼
       ┌──────────────────┐
       │ Kong (:8000)     │
       │ routes /auth/v1  │
       │ routes /storage  │
       └──────────────────┘

Frontend / browser / host scripts → SUPABASE_URL → http://localhost:8000 → Kong
```

### Per-file changes

| File | Change |
|---|---|
| `deployments/docker-swarm/apps/.env.example` | Add `SUPABASE_INTERNAL_URL=http://supabase-kong:8000` |
| `deployments/docker-swarm/apps/.env` | Same |
| `deployments/docker-swarm/apps/docker-compose.yaml` | Pass `SUPABASE_INTERNAL_URL=${SUPABASE_INTERNAL_URL:-${SUPABASE_URL}}` to the `my-cms-api` service env block |
| `apps/api/src/bin/my-cms-api.rs` | In `construct_app_state()`: read `SUPABASE_INTERNAL_URL` with fallback to `supabase_url`; pass to `SupabaseStorage::new()`, `SupabaseAdminClient::new()`, and `construct_supabase_auth_layer()` |
| `apps/api/.env.example` | Add `SUPABASE_INTERNAL_URL=http://localhost:8000` with a comment explaining the optional fallback |
| `apps/api/.env` | Same |
| `apps/web/.env*`, `deployments/docker-swarm/{supabase,traefik}/.env*` | **No changes** |

### Composition-root diff (semantic)

```rust
// Before
let supabase_url = env::var("SUPABASE_URL").expect("SUPABASE_URL must be set");
// ... passed unchanged to Storage, AdminClient, AuthLayer

// After
let supabase_url = env::var("SUPABASE_URL").expect("SUPABASE_URL must be set");
let supabase_internal_url =
    env::var("SUPABASE_INTERNAL_URL").unwrap_or_else(|_| supabase_url.clone());
// supabase_internal_url passed to Storage, AdminClient, AuthLayer
// supabase_url retained only as the fallback source
```

## Risks / Trade-offs

- **`SUPABASE_INTERNAL_URL` accidentally left unset in production** → fallback to `SUPABASE_URL` reproduces today's bug. *Mitigation*: the apps `.env` sets it; production deploys that template-copy will inherit the value. Document in the spec that the fallback is for dev convenience, not production.
- **Drift between `SUPABASE_URL` and `SUPABASE_INTERNAL_URL`** in `.env` files → no automatic detection. *Mitigation*: header comment in `.env.example` calling out the `KEEP IN SYNC` relationship; the existing `KEEP IN SYNC` discipline already covers the other shared vars.
- **A future feature adds another outbound HTTP client** (e.g. an email-sending client) and forgets to use the internal URL → bug recurs in the new client. *Mitigation*: a code-comment near `construct_app_state()` warns that any new outbound Supabase client MUST receive `supabase_internal_url`, not `supabase_url`. Optional follow-up: a single `SupabaseEndpoints` struct that bundles the URLs and is passed to all clients — keeps the rule visible at the call site.
- **`SUPABASE_URL` continues to exist** as a dead-ish variable in the API env (only used as the fallback) → mild confusion. *Mitigation*: a header comment in the env file documents both vars and their roles.

## Migration Plan

This is a non-breaking, additive change. Migration is a single `git pull` + container restart:

1. Pull the changes.
2. `docker compose -f deployments/docker-swarm/apps/docker-compose.yaml --env-file deployments/docker-swarm/apps/.env up -d my-cms-api`
3. Verify: `docker exec my-cms-api printenv SUPABASE_INTERNAL_URL` → `http://supabase-kong:8000`
4. Smoke: `curl -i -H "Authorization: Bearer <admin-jwt>" http://localhost:8989/users` → 200 with the user list.

**Rollback:** revert the `.env` line and the `my-cms-api.rs` reads. The original behavior is restored. No data migration, no DB migration, no frontend deploy.

## Open Questions

None. The design follows an established precedent (`API_EXTERNAL_URL`) and the failure mode is fully understood from the diagnosis in the proposal.
