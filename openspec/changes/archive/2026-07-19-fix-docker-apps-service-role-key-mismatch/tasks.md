# Tasks — fix-docker-apps-service-role-key-mismatch

> All tasks are small, testable units. Mark `- [x]` only after the **Verify** step passes.
> This is a Fast Fix — two env-file edits, one container restart, one manual smoke.

## 1. Restore the `SERVICE_ROLE_KEY` in the live apps `.env`

- [x] 1.1 Replace the placeholder in `deployments/docker-swarm/apps/.env` line 17
  - Current value: `SERVICE_ROLE_KEY=devkey`
  - New value: copy the real HS256 JWT from `deployments/docker-swarm/supabase/.env` line 19 (search for `SERVICE_ROLE_KEY=` in that file and paste the JWT token after the `=`).
  - Keep the existing `# KEEP IN SYNC with .env.supabase` header comment above the line.
  - **Verify:** `grep -n '^SERVICE_ROLE_KEY=' deployments/docker-swarm/apps/.env` shows a value starting with `eyJ`, not the literal `devkey`.
  - **Verify:** `diff <(grep '^SERVICE_ROLE_KEY=' deployments/docker-swarm/apps/.env) <(grep '^SERVICE_ROLE_KEY=' deployments/docker-swarm/supabase/.env)` returns no output (values are identical).

## 2. Fix the host-side template

- [x] 2.1 Replace the placeholder in `apps/api/.env.example` line 22
  - Current value: `SERVICE_ROLE_KEY=devkey`
  - New value: same JWT as in 1.1. A developer who copies this template to `apps/api/.env` (or `.env.local`) will then have a valid bearer token for GoTrue.
  - Keep the existing `# KEEP IN SYNC with ../deployments/docker-swarm/supabase/.env` header comment above the line.
  - **Verify:** `grep -n '^SERVICE_ROLE_KEY=' apps/api/.env.example` shows a value starting with `eyJ`, not `devkey`.
  - **Verify:** `diff <(grep '^SERVICE_ROLE_KEY=' apps/api/.env.example) <(grep '^SERVICE_ROLE_KEY=' deployments/docker-swarm/supabase/.env)` returns no output.

## 3. Restart the API container and smoke-test

- [x] 3.1 Restart `my-cms-api` so the new env var is loaded
  - From the repo root: `docker compose -f deployments/docker-swarm/apps/docker-compose.yaml --env-file deployments/docker-swarm/apps/.env up -d my-cms-api`
  - **Verify:** `docker exec my-cms-api printenv SUPABASE_SERVICE_ROLE_KEY | head -c 6` prints `eyJhbG` (first six characters of the new JWT), not `devkey`.

- [x] 3.2 Confirm the user-list endpoint returns 200 (was 401)
  - Sign in to the admin panel as the seeded administrator and copy the bearer JWT, **or** mint one with `bash scripts/generate-jwt.sh` and use the `service_role` token.
  - Run: `curl -i -H "Authorization: Bearer <admin-jwt>" http://localhost:8989/users`
  - **Verify:** response is `HTTP/1.1 200 OK` with a JSON body containing the seeded administrator's email under `users`. (Was `HTTP/1.1 500 Internal Server Error` with `AppError::Logical("GoTrue list users authorisation error (401 Unauthorized): …")` before the fix.)

- [ ] 3.3 Confirm the other user-management verbs work (create, modify, delete)
  - In the admin panel, create a writer, edit the role, then delete the user.
  - **Verify:** each action returns 2xx and the user list updates. (Was 500 before the fix; the same `SERVICE_ROLE_KEY` is used by all five user-management handlers in `apps/api/application_core/src/commands/user/`.)
  - **Status:** PARTIALLY VERIFIED. `POST /users` (create) returns 200, the seeded admin can sign in, and `GET /users` returns the new entry. However, `PUT /users/{user_id}` and `DELETE /users/{user_id}` both return **405 Method Not Allowed** — a pre-existing routing bug in the `add-user-management-admin-page` change (handler signatures take `Path<Uuid>` but the routes are registered on `/users` with no id, see `apps/api/src/bin/my-cms-api.rs:213-219`). This is **out of scope** for `fix-docker-apps-service-role-key-mismatch`: a follow-up change should re-register the PUT/DELETE handlers on `/users/{user_id}`. The credential chain is independently verified — a service-role bearer reaches the API, hits the routing layer, and the 405 comes back from the Axum router, not from GoTrue. (Test user was created then deleted via the GoTrue admin API directly to leave the env clean.)
