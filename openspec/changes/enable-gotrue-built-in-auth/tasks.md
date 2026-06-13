## 1. Frontend: replace OAuth trigger with email+password form

- [ ] 1.1 Create `frontend/src/app/admin/login/schema.ts` with a Zod schema: `email` is a valid email; `password` is at least 8 characters. Export `LoginSchema` and the inferred `LoginInput` type.
- [ ] 1.2 Create `frontend/src/app/admin/login/page.tsx` rendering a React Hook Form (`useForm<LoginInput>` with `zodResolver(LoginSchema)`), DaisyUI `<input type="email">` and `<input type="password">`, and a `<button type="submit" className="btn btn-primary">`. On submit, call `supabase.auth.signInWithPassword({ email, password })`. On success, read `?from=` from `useSearchParams` and `navigate(from ?? "/admin")`. On failure, `toast.error(err.message)`. Read `auth/supabase.ts`'s `getSupabaseClient()` for the call.
- [ ] 1.3 Register the new route in the admin router (wherever `/admin/posts`, `/admin/categories`, etc. are registered) as `path="/admin/login"` wrapping the page in the existing admin layout. Do NOT wrap the login page in `ProtectedRoute` (it is the unauthenticated destination).
- [ ] 1.4 In `frontend/src/auth/AuthContext.tsx`, replace the `login()` body (currently `signInWithOAuth({ provider: "keycloak" })`) with a navigation shim that calls `useNavigate()` and navigates to `/admin/login`. Drop the `signInWithOAuth` import if it is no longer used anywhere.
- [ ] 1.5 In `frontend/src/app/admin/components/top-bar.tsx`, change the Login button (line 59) from `onClick={login}` to `onClick={login}` only if `login` now navigates — verify and adjust to use `useNavigate` directly if cleaner.
- [ ] 1.6 In `frontend/src/auth/ProtectedRoute.tsx`, remove the `useEffect` that auto-triggers `login()` (the `loginTriggered` ref, the `sessionStorage` `supabase_login_attempt` flag, the hash-inspection for `state=` / `code=` / `session_state=`). Replace the render-while-unauthenticated branch with `<Navigate to="/admin/login" replace state={{ from: location.pathname }} />`. Keep the `loading` spinner branch.
- [ ] 1.7 In `frontend/src/config/api.config.ts`, remove the `_keycloak?: unknown` field (line 96) and any related import or type. Verify nothing else references it via `grep -r keycloak frontend/src`.

## 2. Frontend: tests for the new login flow

- [ ] 2.1 Add a unit test for `LoginSchema` (valid email + 8+ char password passes; invalid email fails; password < 8 chars fails).
- [ ] 2.2 Add a component test for the login page: typing valid credentials and submitting calls `signInWithPassword` with the typed values; submitting invalid credentials does not call the API and shows a validation error; an API error response shows a toast and does not navigate.

## 3. Supabase: close public sign-up

- [ ] 3.1 In `docker-compose.supabase.yaml`, add `GOTRUE_DISABLE_SIGNUP: ${GOTRUE_DISABLE_SIGNUP:-true}` to the `auth` service's `environment` block.
- [ ] 3.2 In `.env.supabase`, add the line `GOTRUE_DISABLE_SIGNUP=true` with a comment explaining it.
- [ ] 3.3 Restart the auth container and verify with `curl -X POST $SUPABASE_PUBLIC_URL/auth/v1/signup -H "apikey: $ANON_KEY" -H "Content-Type: application/json" -d '{"email":"x@y.com","password":"password123"}'` returns HTTP 403.

## 4. Infra: one-shot admin seeder

- [ ] 4.1 Create `volumes/secrets/.gitignore` containing a single line `admin-password.txt` so the generated password file is never committed.
- [ ] 4.2 Create `scripts/seed-admin.sh`: a bash script that (a) reads `SERVICE_ROLE_KEY` and `SUPABASE_PUBLIC_URL` from `.env.supabase`, (b) checks whether the admin user already exists via `GET /auth/v1/admin/users?email=...` with the service-role key, (c) if not present, generates a 24-char random alphanumeric password with `LC_ALL=C tr -dc 'A-Za-z0-9' < /dev/urandom | head -c 24`, calls `POST /auth/v1/admin/users` with `{ email: "${SEED_ADMIN_EMAIL:-admin@my-cms.local}", password, email_confirm: true, app_metadata: { roles: ["my-headless-cms-administrator"] } }`, writes `email` and `password` to `volumes/secrets/admin-password.txt`, and prints the credentials to stdout, (d) if the user already exists, exits 0 with a "skipping" message. The script must exit non-zero only on actual API failures (e.g. GoTrue not reachable, malformed response).
- [ ] 4.3 Make `scripts/seed-admin.sh` executable (`chmod +x`) and `set -euo pipefail` at the top.
- [ ] 4.4 In `reset-supabase.sh`, after the existing "wait for auth healthy" step, invoke `bash scripts/seed-admin.sh` and echo the seeder's stdout so the operator sees the generated password.

## 5. Verification

- [ ] 5.1 `pnpm --dir frontend build` succeeds with no TypeScript errors after the frontend changes.
- [ ] 5.2 `cargo check` succeeds (the backend is unchanged, but confirm nothing accidentally references a removed symbol).
- [ ] 5.3 End-to-end manual smoke: bring up Supabase + apps, navigate to `http://localhost:5173/admin`, see the redirect to `/admin/login`, submit the seeded credentials, land on the admin dashboard, click Logout in the top bar, see the redirect back to `/admin/login`. Confirm `Mailpit` at `:8025` has no leaked sign-up email (sign-up is closed). Confirm `curl -X POST $SUPABASE_PUBLIC_URL/auth/v1/signup ...` returns 403.
- [ ] 5.4 Run a second `reset-supabase.sh` and confirm the seeder is a no-op (the admin user is not recreated and the existing password file is not overwritten).
- [ ] 5.5 `grep -rni keycloak frontend/src services/src` returns no matches (other than this change's own `tasks.md` and `proposal.md`).
