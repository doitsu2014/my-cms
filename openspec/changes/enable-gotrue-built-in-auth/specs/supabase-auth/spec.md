## Purpose

TBD - created by archiving change enable-gotrue-built-in-auth. Update Purpose after archive.

## MODIFIED Requirements

### Requirement: React admin uses Supabase auth

The React admin SHALL use `@supabase/supabase-js` for authentication. A `getSupabaseClient()` singleton SHALL be created in `frontend/src/auth/supabase.ts`, configured with `PUBLIC_SUPABASE_URL` and `PUBLIC_SUPABASE_ANON_KEY`, with `autoRefreshToken`, `persistSession`, and `detectSessionInUrl` enabled. Sign-in SHALL be performed via GoTrue's built-in `signInWithPassword` flow (email + password); the admin SHALL NOT use `signInWithOAuth` for the Keycloak provider or any other external OAuth provider.

#### Scenario: Session restored on page load

- **WHEN** a user visits the admin panel with a valid Supabase session in storage
- **THEN** the `AuthProvider` exposes the session on first render
- **AND** `useAuth().session` is non-null

#### Scenario: Sign out clears the session

- **WHEN** the user calls `useAuth().signOut()`
- **THEN** Supabase clears the local session
- **AND** `useAuth().session` becomes `null`

#### Scenario: Login form uses signInWithPassword

- **WHEN** a user submits the login form at `/admin/login` with a valid email and password
- **THEN** the frontend calls `supabase.auth.signInWithPassword({ email, password })`
- **AND** on success the `AuthProvider` exposes the resulting session
- **AND** the user is navigated to the `?from=` target (default `/admin`)

#### Scenario: Invalid credentials surface a user-visible error

- **WHEN** the user submits credentials that GoTrue rejects
- **THEN** the login form displays a toast with the GoTrue error message
- **AND** `useAuth().session` remains `null`

#### Scenario: Login form rejects malformed input client-side

- **WHEN** the user submits the login form with an invalid email format or a password shorter than 8 characters
- **THEN** the form does not call `signInWithPassword`
- **AND** the relevant field shows a Zod validation error inline

### Requirement: Public sign-up is closed in local dev

GoTrue SHALL be configured with `GOTRUE_DISABLE_SIGNUP=true` in the local Supabase compose file. The public `POST /auth/v1/signup` endpoint SHALL reject requests with HTTP 422 and a response body containing `error_code: "signup_disabled"`. The admin `POST /auth/v1/admin/users` endpoint SHALL remain authorized for the `SERVICE_ROLE_KEY`.

#### Scenario: Public sign-up returns 422 with signup_disabled

- **WHEN** an unauthenticated client calls `POST /auth/v1/signup` with a new email and password
- **THEN** GoTrue returns HTTP 422
- **AND** the response body contains `error_code: "signup_disabled"`

#### Scenario: Admin user creation still works

- **WHEN** the seeder calls `POST /auth/v1/admin/users` with the `SERVICE_ROLE_KEY` header
- **THEN** GoTrue creates the user
- **AND** returns the new user object with a UUID `id`

### Requirement: Administrator account is seeded on Supabase reset

A one-shot seeder SHALL create a single administrator user in GoTrue on a fresh `reset-supabase.sh` run. The seeder SHALL call `POST /auth/v1/admin/users` with `{ email, password, email_confirm: true, app_metadata: { roles: ["my-headless-cms-administrator"] } }` using the `SERVICE_ROLE_KEY`. The seeder SHALL be idempotent: if the user already exists, it SHALL skip creation and SHALL NOT rotate the existing password. On first run the seeder SHALL generate a random 24-character alphanumeric password, write it to `volumes/secrets/admin-password.txt` (gitignored), and print the email + password to stdout.

#### Scenario: First reset creates the admin user

- **WHEN** `reset-supabase.sh` runs against an empty GoTrue database
- **THEN** a user exists in `auth.users` with `app_metadata.roles` containing `my-headless-cms-administrator`
- **AND** the generated password is written to `volumes/secrets/admin-password.txt`
- **AND** the email and password are printed to the script's stdout

#### Scenario: Subsequent reset is a no-op

- **WHEN** `reset-supabase.sh` runs again and the administrator user already exists
- **THEN** no new user is created
- **AND** the existing password is not rotated
- **AND** the existing `volumes/secrets/admin-password.txt` is left untouched

#### Scenario: Seeded admin can sign in

- **WHEN** the operator navigates to `/admin/login` and submits the seeded email and password
- **THEN** GoTrue accepts the credentials and returns a JWT
- **AND** the `SupabaseAuthLayer` accepts the JWT (the `my-headless-cms-administrator` role satisfies the `required_roles` check on admin endpoints)
- **AND** the operator lands on `/admin`

## REMOVED Requirements

### Requirement: Frontend triggers OAuth sign-in to an external provider
**Reason**: The Keycloak OAuth provider was never enabled in the local GoTrue instance, breaking the admin login flow. The product does not need (or want to operate) a Keycloak server. GoTrue's built-in email+password path is sufficient for a single-operator CMS.
**Migration**: Replace `signInWithOAuth({ provider: "keycloak" })` with `signInWithPassword({ email, password })` on the new `/admin/login` form. Seed the administrator user out-of-band via the admin API so the operator can sign in on a fresh reset.

### Requirement: ProtectedRoute auto-redirects to a sign-in flow with callback handling
**Reason**: The auto-redirect, `sessionStorage` callback flag, and `loginTriggered` ref existed solely to survive the OAuth round-trip (where the user briefly lands back on the SPA with `?code=...&state=...` in the URL hash). With email+password there is no round-trip — the user submits credentials and the SPA stays put.
**Migration**: `ProtectedRoute` now renders a `<Navigate to="/admin/login?from=..." />` when unauthenticated. The `sessionStorage` flag, `loginTriggered` ref, and OAuth-callback detection (hash inspection for `state=` / `code=` / `session_state=`) are removed.

## ADDED Requirements

### Requirement: Kong gateway forwards authenticated Supabase API calls

The Kong gateway in front of the local Supabase stack SHALL forward authenticated API calls (carrying `apikey` and `Authorization` headers) to the upstream GoTrue, PostgREST, and other Supabase services, instead of rejecting them with HTTP 401. Kong's `key-auth` plugin SHALL be configured with two registered consumers — `anon` (ACL group `anon`) and `service_role` (ACL group `admin`) — whose credentials match the `ANON_KEY` and `SERVICE_ROLE_KEY` env vars. A `request-transformer` plugin on each authenticated route SHALL rewrite the `apikey: $SUPABASE_ANON_KEY|SERVICE_KEY` request into a GoTrue-mintable `Authorization: Bearer <jwt>` header signed with the project's `JWT_SECRET`. The Kong service in `docker-compose.supabase.yaml` SHALL use a vendored `kong-entrypoint.sh` (env-substituting `$VAR` references and computing the `$LUA_AUTH_EXPR` template) mounted as the container's `entrypoint`, with the declarative config mounted as `temp.yml` and written to `/home/kong/kong.yml` after substitution.

#### Scenario: Admin user list reaches GoTrue through Kong

- **WHEN** the seeder or an operator calls `GET /auth/v1/admin/users` with the `apikey: <SERVICE_ROLE_KEY>` header
- **THEN** Kong forwards the request to GoTrue (does not return 401)
- **AND** GoTrue returns the user list (200 with a JSON array)

#### Scenario: Anonymous PostgREST query reaches the database through Kong

- **WHEN** a client calls `GET /rest/v1/<table>?select=*&limit=1` with the `apikey: <ANON_KEY>` header
- **THEN** Kong forwards the request to PostgREST (does not return 401)
- **AND** PostgREST returns the query result (200)

#### Scenario: Public sign-up still rejected

- **WHEN** an unauthenticated client calls `POST /auth/v1/signup`
- **THEN** Kong forwards the request to GoTrue
- **AND** GoTrue returns 422 with `error_code: signup_disabled` (see "Public sign-up is closed in local dev" requirement)

#### Scenario: Supabase JS client sign-in round-trip

- **WHEN** the React admin calls `supabase.auth.signInWithPassword({ email, password })`
- **THEN** the Supabase JS client POSTs to `/auth/v1/token?grant_type=password` through Kong
- **AND** Kong forwards the request to GoTrue
- **AND** GoTrue validates the credentials, mints a JWT, and returns it
- **AND** the JS client receives the session and updates `AuthContext`
