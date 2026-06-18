## Context

The Supabase GoTrue instance in local dev (exposed via Kong at `http://localhost:8001/auth/v1`) does not have Keycloak enabled as an external OAuth provider. The frontend's `AuthContext.login()` therefore fails with `{"code":400,"error_code":"validation_failed","msg":"Unsupported provider: provider is not enabled"}` before any redirect happens. The product does not want to operate a Keycloak server.

The codebase has already migrated away from Keycloak on the backend: `SupabaseAuthLayer` (`services/src/common/supabase_auth.rs:52-162`) validates GoTrue-issued JWTs (HS256 with `JWT_SECRET`, RS256 fallback via JWKS) and exposes the user identity as `Extension<SupabaseToken>`. The frontend migrated the auth client from `keycloak-js` to `@supabase/supabase-js` in commit history. What is left is the *sign-in trigger* on the frontend: a stray `signInWithOAuth({ provider: "keycloak" })` call that was never finished.

This change finishes that migration by switching the sign-in trigger from OAuth-Keycloak to GoTrue's built-in email+password path, closing public sign-up (the product is a personal CMS — only the operator should be able to log in), and adding a one-shot admin seeder so a fresh reset always has a usable account.

## Goals / Non-Goals

**Goals:**

- Make the admin login flow work end-to-end with **zero external identity providers** (no Keycloak, no Google, no GitHub — only GoTrue's built-in email+password).
- Keep the backend `SupabaseAuthLayer` unchanged: the JWT shape from `signInWithPassword` is identical to what GoTrue would have issued from any other sign-in method, so all role checks and audit columns keep working.
- Make a fresh `reset-supabase.sh` always produce a working admin login (random password generated, persisted to a gitignored file, printed to stdout).
- Close the public sign-up endpoint so a fresh reset does not expose a public registration form.

**Non-Goals:**

- Multi-user onboarding (writers, editors, role assignment UI). The seeder creates one administrator; extending to many users is a follow-up.
- A password reset UI. In local dev, password reset emails are captured by Mailpit at `:8025` and a reset link works out of the box once the operator opens the email. A real reset page is a follow-up.
- Replacing the AuthContext abstraction. The context surface (`user`, `session`, `getAccessToken`, `signOut`, `login`, `logout`) stays; only `login`'s body changes from an OAuth redirect to a thin navigation shim.
- Backend changes. The Rust `SupabaseAuthLayer` continues to validate GoTrue tokens as-is. Adding a server-side `/login` endpoint is unnecessary — `@supabase/supabase-js` already calls `POST /auth/v1/token?grant_type=password` on the frontend.
- Modifying the in-progress `split-supabase-and-apps-compose` change. This change adds files to the new layout (compose, env, scripts) without touching that change's diff.

## Decisions

### D1. Pre-seeded admin with GoTrue's admin `createUser` API — not SQL, not a migrations file

GoTrue stores users in the `auth.users` table inside a schema whose internal layout changes between Supabase versions. Inserting rows directly via SQL is fragile (column additions, trigger expectations, the `encrypted_password` bcrypt format). The supported path is the **admin REST endpoint** `POST /auth/v1/admin/users` with the `SERVICE_ROLE_KEY` and `{ email, password, email_confirm: true, app_metadata: { roles: ["my-headless-cms-administrator"] } }`. GoTrue does the bcrypt hash, fires any triggers correctly, and returns the new `user.id` (which doubles as `auth.uid()` in RLS policies).

**Alternative considered:** Seed via SeaORM migration in `services/migration/`. Rejected because (a) it bypasses the bcrypt path, (b) it ties admin creation to a database migration (a one-shot concern, not a schema concern), and (c) it breaks against Supabase minor upgrades if the internal schema changes.

### D2. Random password generated on first reset, persisted to `volumes/secrets/admin-password.txt`

The seeder generates a 24-character alphanumeric password on first run and writes it to `volumes/secrets/admin-password.txt` (gitignored). On subsequent runs it checks whether the user already exists and skips both generation and `createUser`. This is the right balance for a local-dev stack:

- No secrets committed to git.
- No manual step the operator must remember.
- The first reset run prints the password to stdout; if the operator misses it, re-running with a forced reseed (`--force` or removing the user from GoTrue) regenerates one.

**Alternative considered:** Hardcode `admin@example.com` / `changeme` in `.env.supabase`. Rejected — leaks a known credential into the repo and produces a "this is the default" footgun across developer machines.

**Alternative considered:** Force the operator to set `SEED_ADMIN_EMAIL` / `SEED_ADMIN_PASSWORD` in `.env.supabase` before first boot. Rejected — adds a manual step that is easy to forget and easy to do wrong; the seeder is more useful when it is self-contained.

### D3. `GOTRUE_DISABLE_SIGNUP=true` to close public sign-up, but keep `createUser` open

GoTrue reads `GOTRUE_DISABLE_SIGNUP` at boot; when set, `POST /auth/v1/signup` returns 403, but the **admin** endpoints (`/auth/v1/admin/users` with `SERVICE_ROLE_KEY`) are still authorized. This is exactly the policy the product wants: no public registration, but the seeder can still create users.

**Alternative considered:** Filter sign-up by email domain in a GoTrue hook. Rejected — adds GoTrue runtime complexity for no benefit; the disable-signup switch already exists and is the documented path.

### D4. New `frontend/src/app/admin/login/page.tsx` as a real form, not a redirect page

The login page is a controlled React Hook Form with Zod validation (`email` is a valid email; `password` is at least 8 chars), renders a DaisyUI `<input>` pair and a `<button type="submit" className="btn btn-primary">`, and calls `supabase.auth.signInWithPassword` on submit. On success it navigates to the original `from` location (read from `?from=` query string, default `/admin`). On failure it toasts the GoTrue error message via Sonner.

This replaces both (a) the OAuth redirect that did not work and (b) the implicit "ProtectedRoute will trigger login for you" flow. The auto-redirect is removed because it is unnecessary for an email+password form: the user only types credentials when they explicitly click "Login" in the top bar.

**Alternative considered:** Keep the auto-redirect and have ProtectedRoute render the form inline. Rejected — confuses deep-linking (a user bookmarking `/admin/posts` would never see the form, just a spinner), and the existing form-on-a-dedicated-route pattern matches the rest of the admin (`/admin/posts/new`, `/admin/categories/edit/:id`).

### D5. `login()` in AuthContext becomes a navigation shim, not a network call

`AuthContext.login()` is consumed in two places: the top-bar Login button and (previously) ProtectedRoute. After this change, both want the same thing: navigate to `/admin/login`. The function is preserved as `() => navigate("/admin/login")` so existing call sites keep compiling. The form page itself calls `supabase.auth.signInWithPassword` directly — AuthContext does not wrap a successful sign-in (the `onAuthStateChange` listener already updates context state).

**Alternative considered:** Have `AuthContext.login(email, password)` perform `signInWithPassword` and return a result. Rejected — mixes the auth provider with form state and forces the form page to thread `setError` through context, which is more code and less testable.

### D6. Repair the Kong gateway as part of this change (discovered during implementation)

During the implementation of tasks 3.3 (verify signup is closed) and 4.1–4.4 (add the admin seeder), an independent failure mode was discovered that is **not** part of the originally scoped change but is a prerequisite for the change to be testable end-to-end.

**The bug:** `volumes/api/kong.yml` enables the `key-auth` plugin on every authenticated route (`auth-v1`, `rest`, `rest-graphql`, `realtime`, `meta`, `analytics`) but is missing the top-level `consumers:` section. Without registered consumers, Kong's `key-auth` plugin rejects every API call with `401 Invalid authentication credentials` before the request reaches the upstream service (GoTrue, PostgREST, etc.). The stack has been running in this broken state — every authenticated Supabase API call has been silently returning 401. The only reason any of the Supabase tooling appeared to work is that the `*-open*` routes (`/auth/v1/verify`, `/auth/v1/callback`, `/auth/v1/authorize`, `/auth/v1/sso/saml`) intentionally have no `key-auth` and have been the only path Kong actually forwards.

**This is the real root cause of the original problem report** ("`{"code":400,"error_code":"validation_failed","msg":"Unsupported provider: provider is not enabled"}`"). The frontend was sending `apikey: devkey` to `/auth/v1/authorize`; Kong's `auth-v1-open-authorize` route has no `key-auth`, so Kong forwarded the request to GoTrue, which then rejected it because Keycloak was never configured as a provider. After this change moves sign-in to `signInWithPassword` (which hits `/auth/v1/token?grant_type=password`, routed by the *authenticated* `auth-v1` service), the call would have failed at Kong with 401 instead of at GoTrue with 400 — same broken end state, different layer.

**Decision:** Fix the Kong config in this change, not in a separate one.

**Rationale:**
- The Kong fix and the email+password migration are operationally inseparable: the seeder (4.1–4.4), the signup-disable verification (3.3), and the end-to-end smoke test (5.3) all depend on a working authenticated path through Kong. Without the Kong fix, none of those tasks can pass.
- The bug has been latent for the lifetime of the local stack (the stack has been up 6+ hours; it is not a regression from this change), but it is also a **hard blocker** for the change to be verifiable. Shipping a change whose tasks 3.3, 4.x, and 5.3 cannot be verified is a worse outcome than expanding scope.
- The fix is mechanical and vendor-from-upstream: copy the upstream Supabase `kong.yml` `consumers` block + `request-transformer` plugin, vendor the `kong-entrypoint.sh` env-substitution script, and update the Kong service in `docker-compose.supabase.yaml` to use the entrypoint. The local stack is already using upstream's `kong:2.8.1` image and the same env-var names, so the upstream pattern drops in with minimal change.

**Alternative considered:** Punt the Kong fix to a separate change. Rejected because the email+password migration's verification gate (5.3) is the most important end-to-end test in the whole change, and it cannot pass without the Kong fix. Shipping two changes serially doubles the round-trip cost (two PRs, two CI runs, two deploys) for what is functionally one user-visible fix ("the admin login flow works end-to-end").

**Scope of the fix (see tasks 6.1–6.4):**
- Vendor upstream `volumes/api/kong-entrypoint.sh` and update `volumes/api/kong.yml` to add `consumers` + `request-transformer` (keeping the local slim set of routes — no edge functions, MCP, analytics-to-logflare, oauth-authorization-server, JWKS, or SSO-SAML).
- Update the Kong service in `docker-compose.supabase.yaml` to mount `temp.yml` + `kong-entrypoint.sh`, set `entrypoint: [/home/kong/kong-entrypoint.sh]`, and add `SUPABASE_ANON_KEY` / `SUPABASE_SERVICE_KEY` env vars.
- One-line spec correction: scenario for "Public sign-up returns 403" becomes "returns 422 with `error_code: signup_disabled`" (the actual GoTrue v2.179.0 response).

## Risks / Trade-offs

- **Forgotten admin password on first reset** → The seeder prints the generated password to stdout and also writes it to `volumes/secrets/admin-password.txt`. The reset script's summary line explicitly calls out the file path. If both are missed, re-running `reset-supabase.sh` after deleting the user (or the script supports a `--rotate` flag) regenerates a new password.
- **`@supabase/supabase-js` rate-limits `signInWithPassword`** (by default, 30 attempts per 5 min per IP) → In local dev, this is not a real risk (the only client is a developer clicking the form). If it ever became one, the rate-limit is on GoTrue's side and tunable via `GOTRUE_RATE_LIMIT_HEADER` / `GOTRUE_RATE_LIMIT_EMAIL_SENT` env vars.
- **Frontend `login()` becomes a navigation shim — name is now slightly misleading** → Acceptable. The function still means "initiate sign-in flow", and the comment in AuthContext notes the change. A follow-up could rename to `goToLogin()` if desired.
- **OAuth callback detection code in `ProtectedRoute` becomes dead code** → Removed entirely. If we ever add another OAuth provider, the detection logic can come back. YAGNI for now.
- **`_keycloak?: unknown` field in `api.config.ts` is removed** → Verified by grep that nothing else references it. If a stale reference slips through, `tsc` will flag it.
- **Race between `seed-admin.sh` and `auth` service becoming healthy** → `reset-supabase.sh` already waits for `auth` to respond on `/auth/v1/health` before declaring Supabase ready. The seeder is invoked after that wait, so GoTrue is up by the time `createUser` is called.
- **Kong fix scope expansion** → The Kong repair adds ~3 new files (`kong-entrypoint.sh`, modified `kong.yml`, modified `docker-compose.supabase.yaml`) and one new test step. It is bigger than a "just add a seeder" change, but it is the smallest end-to-end-correct outcome. The alternative — shipping the email+password code with broken verification — is worse.
