# Tasks — fix-supabase-internal-url-docker-networking

> All tasks are small, testable units. Mark `- [x]` only after the **Verify** step passes.
> This change is a deployment/wiring fix. No production code logic changes — only env-var resolution at the API composition root and the matching `.env` / Compose updates.

## 1. Env files (Docker Compose stack)

- [ ] 1.1 Add `SUPABASE_INTERNAL_URL=http://supabase-kong:8000` to `deployments/docker-swarm/apps/.env.example`
    - Place under the existing `# ---------- Supabase (apps-side) ----------` block, immediately after `SUPABASE_URL=http://localhost:8000`.
    - Add an inline comment explaining: "Docker-network URL the API container uses to reach Kong. Falls back to `SUPABASE_URL` if unset. Frontend and host-side scripts keep using `SUPABASE_URL`."
- [ ] 1.2 Add the same line to `deployments/docker-swarm/apps/.env`
    - Same placement and comment as 1.1.
    - **Verify:** `grep -n SUPABASE_INTERNAL_URL deployments/docker-swarm/apps/.env`

## 2. Docker Compose wiring

- [ ] 2.1 Pass `SUPABASE_INTERNAL_URL` to the `my-cms-api` service in `deployments/docker-swarm/apps/docker-compose.yaml`
    - Add inside the `my-cms-api` `environment:` block, adjacent to `SUPABASE_URL: ${SUPABASE_URL}` (line 114).
    - Use the docker-compose `${VAR:-${DEFAULT}}` interpolation: `SUPABASE_INTERNAL_URL: ${SUPABASE_INTERNAL_URL:-${SUPABASE_URL}}`. This mirrors the existing fallback discipline used elsewhere (e.g. `MAX_BODY_LENGTH`).
    - **Verify:** `docker compose -f deployments/docker-swarm/apps/docker-compose.yaml --env-file deployments/docker-swarm/apps/.env config | grep -A1 my-cms-api | head -30` shows `SUPABASE_INTERNAL_URL` in the resolved env.

## 3. Host-side env files

- [ ] 3.1 Add `SUPABASE_INTERNAL_URL=http://localhost:8000` to `apps/api/.env.example`
    - Place under the existing `# ---------- Supabase (apps-side) ----------` block, immediately after `SUPABASE_URL=http://localhost:8000` (line 41).
    - Inline comment: "Optional. Falls back to `SUPABASE_URL`. Same value as `SUPABASE_URL` here because `cargo run` outside Docker resolves Kong via the host-exposed port. Set this to `http://supabase-kong:8000` only if running the API inside the apps Docker Compose stack."
- [ ] 3.2 Add the same line to `apps/api/.env`
    - Same placement and comment as 3.1.
    - **Verify:** `grep -n SUPABASE_INTERNAL_URL apps/api/.env`

## 4. API composition root

- [ ] 4.1 Update `apps/api/src/bin/my-cms-api.rs::construct_app_state()` to read `SUPABASE_INTERNAL_URL` and pass it to all three Supabase clients
    - **Files:** `apps/api/src/bin/my-cms-api.rs`
    - After the existing `let supabase_url = env::var("SUPABASE_URL")...` (line 239), add `let supabase_internal_url = env::var("SUPABASE_INTERNAL_URL").unwrap_or_else(|_| supabase_url.clone());`.
    - Replace `supabase_url.clone()` in the `SupabaseStorage::new(...)` call (line 251) with `supabase_internal_url.clone()`.
    - Replace `supabase_url` in the `SupabaseAdminClient::new(...)` call (line 258) with `supabase_internal_url`.
    - In `construct_supabase_auth_layer(...)` (line 282), replace the `let supabase_url = ...` and the `supabase_url:` field of `SupabaseAuthConfig { ... }` to use `supabase_internal_url` instead. Since `construct_supabase_auth_layer` is a separate function, hoist the resolution: either pass `supabase_internal_url` in as a parameter, or duplicate the `.unwrap_or_else(|_| supabase_url.clone())` pattern.
    - Add an inline comment above the new `let supabase_internal_url = ...` line: "Any new outbound Supabase client MUST receive `supabase_internal_url`, not `supabase_url`."
    - **Verify:** `cd apps/api && cargo check -p cms`

## 5. Verification gate

- [ ] 5.1 Run `cargo check` and `cargo test` for the backend workspace
    - **Verify:** `cd apps/api && cargo check && cargo test` — both succeed. The 26 WireMock-based `SupabaseAdminClient` tests + the 2 sanitisation tests + the existing `SupabaseStorage` tests stay green (no logic changes; only the env-var resolution at boot changes).
- [ ] 5.2 Run `cargo fmt -- --check` and `cargo clippy --all-targets -- -D warnings`
    - **Verify:** `cd apps/api && cargo fmt -- --check && cargo clippy --all-targets -- -D warnings` — green for any files touched by this change. Pre-existing clippy errors in unrelated files (per the `add-user-management-admin-page` task 14 resume notes) are out of scope and not regressions of this change.
- [ ] 5.3 Run `pnpm --dir apps/web build`
    - **Verify:** build succeeds. The frontend is untouched by this change; this confirms no regression.
- [ ] 5.4 Manual smoke — restart the API container and verify the resolved env
    - `docker compose -f deployments/docker-swarm/apps/docker-compose.yaml --env-file deployments/docker-swarm/apps/.env up -d my-cms-api`
    - `docker exec my-cms-api printenv SUPABASE_INTERNAL_URL` → `http://supabase-kong:8000`
    - `docker exec my-cms-api printenv SUPABASE_URL` → `http://localhost:8000` (unchanged, kept as fallback source)
- [ ] 5.5 Manual smoke — exercise the user-management endpoints
    - Sign in to the admin panel as the seeded administrator.
    - Open `/admin/users` — the list now loads with HTTP 200 (was 500).
    - Create a writer, edit their role, ban, unban, delete. All endpoints now reach GoTrue via the Docker network.
- [ ] 5.6 Manual smoke — confirm media upload (the latent bug)
    - Open a post editor and upload an image. The upload succeeds (was failing silently when run inside Docker).
    - `curl -i -H "Authorization: Bearer <admin-jwt>" http://localhost:8989/media?prefix=` → 200 with object list.

## Hand-off

When all tasks are complete and the verification gate passes:

1. `openspec-verify-change fix-supabase-internal-url-docker-networking`
2. `openspec-sync-specs fix-supabase-internal-url-docker-networking`
3. `openspec-archive-change fix-supabase-internal-url-docker-networking`

After archive, resume the paused `add-user-management-admin-page` change (tasks 11, 12, 14, 16–28 are unaffected by this fix — task 28's manual smoke now exercises the working `/users` endpoints).
