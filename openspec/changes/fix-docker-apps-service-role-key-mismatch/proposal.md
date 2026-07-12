## Why

When the `my-cms-api` container runs in the apps Docker Compose stack, every outbound call to GoTrue's admin API (e.g. `GET /auth/v1/admin/users` from the new `SupabaseAdminClient`) returns **HTTP 401 "Invalid authentication credentials"**, even though the request reaches GoTrue successfully. The cause is a credential mismatch: the API container is sending the placeholder string `devkey` as the bearer token, while the GoTrue container is configured to expect a real HS256 JWT signed with the project's `JWT_SECRET`. The credential drift sits in `deployments/docker-swarm/apps/.env` line 17, where `SERVICE_ROLE_KEY=devkey` instead of the JWT, and is mirrored in `apps/api/.env.example` line 22.

The five other references to `SERVICE_ROLE_KEY` (`apps/api/.env`, `apps/api/.env.local`, `deployments/docker-swarm/supabase/.env`, `deployments/docker-swarm/supabase/.env.example`, `deployments/docker-swarm/apps/.env.example`) all carry the real JWT — only one of two real `.env` files is stale, and one of two templates is stale. The `local-dev-environment` spec already mandates that `SERVICE_ROLE_KEY` appears in both per-stack env files with a `KEEP IN SYNC` header, and the in-flight `fix-supabase-internal-url-docker-networking` change added a `KEEP IN SYNC` discipline, but neither caught this particular drift.

This is a Fast Fix — two one-line env-file edits, no production code, no tests beyond a manual smoke against `GET /auth/v1/admin/users`.

## What Changes

- `deployments/docker-swarm/apps/.env` — replace `SERVICE_ROLE_KEY=devkey` (line 17) with the same real JWT already used by `deployments/docker-swarm/supabase/.env` (line 19). This unblocks the API container's GoTrue admin calls.
- `apps/api/.env.example` — replace `SERVICE_ROLE_KEY=devkey` (line 22) with the same JWT. The template must not ship a placeholder; new developers copying the example would otherwise hit the same 401.
- No new env vars, no new modules, no application code touched. Both edits are kept under the existing `KEEP IN SYNC with …` header comments.
- The two `.env.example` files in the docker-swarm sub-tree are already correct and need no edit.

## Capabilities

### New Capabilities

None.

### Modified Capabilities

- `local-dev-environment` — extend the existing "Two per-stack env files with shared values synchronised" requirement with a concrete scenario that pins down the failure mode observed here: when one side's `SERVICE_ROLE_KEY` is the placeholder `devkey` and the other side's is a real JWT, the GoTrue admin API returns 401 with the body `{"message":"Invalid authentication credentials"}`. This makes the spec testable for future drift.

## Impact

| Layer | Impact |
|---|---|
| **Application code** | None. `SupabaseAdminClient` already sends the bearer token correctly; the only fix is the value of the env var it reads. |
| **Env files** | 2 line edits: `deployments/docker-swarm/apps/.env:17` and `apps/api/.env.example:22`. |
| **OpenSpec specs** | One delta in `openspec/changes/fix-docker-apps-service-role-key-mismatch/specs/local-dev-environment/spec.md`, extending the existing shared-value-drift scenario. |
| **OpenSpec other change** | Discovered while implementing `fix-supabase-internal-url-docker-networking`; that change is not modified. The two changes are independent and can be applied in either order. |
| **Tests** | No automated tests added. Verification is a manual smoke: `curl -i -H "Authorization: Bearer $ADMIN_JWT" http://localhost:8989/users` should return 200 with the seeded user list. |
| **Production** | This is a local dev env bug. Production deploys manage secrets out-of-band; if a production env was ever affected the same fix applies (set `SERVICE_ROLE_KEY` to the JWT GoTrue was started with). |
