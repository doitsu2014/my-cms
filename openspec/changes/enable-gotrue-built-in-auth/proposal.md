## Why

The frontend currently calls `supabase.auth.signInWithOAuth({ provider: "keycloak" })` (see `frontend/src/auth/AuthContext.tsx:75-78`), but the local Supabase GoTrue instance does not have Keycloak configured as an external OAuth provider. The result is `{"code":400,"error_code":"validation_failed","msg":"Unsupported provider: provider is not enabled"}` returned by GoTrue on every login attempt, making the admin UI unreachable. The product does not need (and does not want to operate) a Keycloak server. The fix is to use GoTrue's built-in email+password sign-in path, pre-seeding a single administrator account out-of-band so public sign-up can stay closed.

## What Changes

- **Add a real login page** at `frontend/src/app/admin/login/page.tsx` that renders an email + password form and calls `supabase.auth.signInWithPassword`. Replaces the implicit "you'll be redirected somewhere" flow.
- **Replace the `signInWithOAuth("keycloak")` call in `AuthContext.tsx`** with a no-op `login()` whose semantics become "navigate to `/admin/login`". The form page performs the actual `signInWithPassword` call.
- **Simplify `ProtectedRoute`** to render a `<Navigate to="/admin/login" />` (or a link) when unauthenticated, removing the `sessionStorage` callback-detection dance that exists only to survive the OAuth round-trip.
- **Update the top-bar Login button** to navigate to `/admin/login` instead of triggering an OAuth redirect. Logout button is unchanged.
- **Close public sign-up in GoTrue** by setting `GOTRUE_DISABLE_SIGNUP=true` in the Supabase compose file.
- **Add a one-shot seeder** `scripts/seed-admin.sh` (or a `seed-admin` service in the apps compose) that calls GoTrue's admin `createUser` endpoint with `SERVICE_ROLE_KEY` to create the administrator with `email_confirm: true` and `app_metadata.roles = ["my-headless-cms-administrator"]`. Idempotent: skips if the user already exists. Password is generated randomly on first run and persisted to a gitignored file under `volumes/secrets/`.
- **Wire the seeder into `reset-supabase.sh`** so a clean reset always re-creates the admin user with a fresh password printed to stdout.
- **Drop the dead `_keycloak?: unknown` field** from `frontend/src/config/api.config.ts` (the only remaining `keycloak` mention in the frontend source).

The backend `SupabaseAuthLayer` (`services/src/common/supabase_auth.rs`) is **not modified**. The JWT shape GoTrue issues from `signInWithPassword` is identical to what it would have issued from an OAuth flow: same `sub`, `email`, `role`, `app_metadata` claims, same HS256 signing with the same `JWT_SECRET`, same `aud: "authenticated"`. The existing role-enforcement check (`required_roles` reading `app_metadata.roles`) keeps working unchanged.

## Capabilities

### Modified Capabilities

- `supabase-auth`: the sign-in *mechanism* changes from OAuth-Keycloak (which was never enabled) to GoTrue's built-in email+password flow. The sign-in *contract* (a GoTrue-issued JWT presented as `Authorization: Bearer …`, validated by `SupabaseAuthLayer`, with `app_metadata.roles` enforcing the administrator role) is unchanged. The public-facing requirement is the same; the *how* in the spec is updated to reflect the new method. A new requirement is added for the Kong gateway to forward authenticated Supabase API calls (the gateway was silently rejecting every authenticated call with 401; the fix is part of this change because the email+password flow's verification gate depends on it).

## Impact

- **New frontend files**: `frontend/src/app/admin/login/page.tsx` (login form), `frontend/src/app/admin/login/schema.ts` (Zod validation), `frontend/src/app/admin/login/schema.test.ts` (unit tests), `frontend/src/app/admin/login/page.test.tsx` (component tests), `frontend/vitest.config.ts` (test runner config), `frontend/src/test/setup.ts` (jest-dom matchers loader).
- **Modified frontend files**: `frontend/src/auth/AuthContext.tsx` (replace `login` body, drop `signInWithOAuth` import), `frontend/src/auth/ProtectedRoute.tsx` (drop callback detection + sessionStorage + auto-trigger), `frontend/src/app/admin/login/page.tsx` (add `noValidate` to the form so Zod is the sole validation source), `frontend/src/app/admin/components/top-bar.tsx` (no change — already compatible), `frontend/src/config/api.config.ts` (drop `_keycloak` field), `frontend/src/App.tsx` (register the new `/admin/login` route), `frontend/package.json` (add `test` and `test:watch` scripts + vitest / testing-library devDeps), `frontend/tsconfig.json` (add vitest globals + jest-dom types).
- **New infra files**: `scripts/seed-admin.sh` (one-shot admin seeder), `volumes/secrets/.gitignore` (ignore `admin-password.txt`), `volumes/api/kong-entrypoint.sh` (vendored from upstream Supabase, env-substitutes the Kong config and computes `$LUA_AUTH_EXPR` for the request-transformer plugin).
- **Modified infra files**: `docker-compose.supabase.yaml` (add `GOTRUE_DISABLE_SIGNUP` to `auth` env, change Kong service to mount `temp.yml` + `kong-entrypoint.sh` with an `entrypoint:` override and `SUPABASE_ANON_KEY` / `SUPABASE_SERVICE_KEY` env vars), `volumes/api/kong.yml` (add `consumers` block + `request-transformer` plugin with `$LUA_AUTH_EXPR` to each authenticated service), `.env.supabase` (add `GOTRUE_DISABLE_SIGNUP=true`), `reset-supabase.sh` (call `seed-admin.sh` after Supabase is healthy).
- **No backend Rust changes.** `SupabaseAuthLayer`, all `*_handler.rs` files extracting `SupabaseToken`, and the JWT secret/audience config stay as-is.
- **Env vars**: no new vars. Reuses existing `SERVICE_ROLE_KEY`, `SUPABASE_PUBLIC_URL`, `SITE_URL`, `ANON_KEY`, `JWT_SECRET`. The generated admin password is written to `volumes/secrets/admin-password.txt` (gitignored) and printed to stdout by `reset-supabase.sh`. Kong now reads `SUPABASE_ANON_KEY` and `SUPABASE_SERVICE_KEY` as env vars (which the compose file maps from the existing `ANON_KEY` and `SERVICE_ROLE_KEY`).
- **Out of scope (handled by other changes or follow-up)**:
  - Multi-user onboarding (writers, editors). The seeder creates a single admin; the same script can be extended later to create additional users with the `my-headless-cms-writer` role.
  - Password reset UI. In local dev, password reset emails land in Mailpit at `:8025`; a reset flow is a follow-up change.
  - The in-progress `split-supabase-and-apps-compose` change — this change depends on the new file layout taking effect (seeder lives in `scripts/`, env handling matches `.env.supabase`).
