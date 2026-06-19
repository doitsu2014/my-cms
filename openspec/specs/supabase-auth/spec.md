# supabase-auth Specification

## Purpose
TBD - created by archiving change supabase-auth-and-pgvector-migration. Update Purpose after archive.
## Requirements
### Requirement: Supabase GoTrue JWTs are accepted on protected routes

The system SHALL accept Supabase GoTrue JWTs on every protected Axum route. Tokens SHALL be validated with HS256 using the project's `SUPABASE_JWT_SECRET` and `AUTHORIZATION_AUDIENCE` (default `authenticated`). When HS256 validation fails, the middleware SHALL fall back to RS256 using the JWKS document at `{SUPABASE_URL}/auth/v1/.well-known/jwks.json`, picking the key by `kid`.

#### Scenario: Valid HS256 token

- **WHEN** a request carries `Authorization: Bearer <valid HS256 token>` issued by GoTrue
- **THEN** the request reaches the protected handler
- **AND** a `SupabaseToken` extension is available via `Extension<SupabaseToken>`

#### Scenario: Missing Authorization header

- **WHEN** a request to a protected route has no `Authorization` header
- **THEN** the middleware returns HTTP 401
- **AND** the response body is `{"error": "Missing Authorization header"}`

#### Scenario: Invalid signature

- **WHEN** a request carries a Bearer token that fails HS256 validation
- **AND** the token's `alg` is `HS256` (so the RS256 fallback cannot help)
- **THEN** the middleware returns HTTP 401
- **AND** the response body identifies the validation failure

#### Scenario: Audience mismatch

- **WHEN** a token's `aud` claim does not match `AUTHORIZATION_AUDIENCE`
- **THEN** the middleware returns HTTP 401

### Requirement: SupabaseToken extension exposes user identity

The system SHALL insert a `SupabaseToken { claims: SupabaseClaims }` extension on every authenticated request. The token SHALL expose `user_id()` (returns `claims.sub`), `email()` (returns `claims.email`), and `role()` (returns `claims.role`).

#### Scenario: Handler reads email for audit column

- **WHEN** a handler extracts `Extension(token): Extension<SupabaseToken>`
- **THEN** `token.email().unwrap_or_default().to_string()` returns the caller's email
- **AND** the value is written to the audit column of the affected row

#### Scenario: Handler reads user id

- **WHEN** a handler calls `token.user_id()`
- **THEN** it receives the Supabase user UUID (`claims.sub`)

### Requirement: All handlers stop using KeycloakToken

The system SHALL NOT use `KeycloakToken<String>`, `axum-keycloak-auth`, `KeycloakAuthLayer`, or `keycloak_extension` anywhere in `apps/api/src/api/**`. Every protected handler SHALL use `SupabaseToken` instead.

#### Scenario: Search for Keycloak references

- **WHEN** a developer greps `apps/api/src` for `KeycloakToken` and `axum_keycloak_auth`
- **THEN** no matches are found (other than the git history)

#### Scenario: Build succeeds without Keycloak crates

- **WHEN** `cargo build` is run with `axum-keycloak-auth` removed from `apps/api/Cargo.toml`
- **THEN** the build succeeds
- **AND** the API responds to health checks on `/health`

### Requirement: React admin uses Supabase auth

The React admin SHALL use `@supabase/supabase-js` for authentication. A `getSupabaseClient()` singleton SHALL be created in `apps/web/src/auth/supabase.ts`, configured with `PUBLIC_SUPABASE_URL` and `PUBLIC_SUPABASE_ANON_KEY`, with `autoRefreshToken`, `persistSession`, and `detectSessionInUrl` enabled.

#### Scenario: Session restored on page load

- **WHEN** a user visits the admin panel with a valid Supabase session in storage
- **THEN** the `AuthProvider` exposes the session on first render
- **AND** `useAuth().session` is non-null

#### Scenario: Sign out clears the session

- **WHEN** the user calls `useAuth().signOut()`
- **THEN** Supabase clears the local session
- **AND** `useAuth().session` becomes `null`

### Requirement: GraphQL client sends Supabase bearer tokens

The Apollo `setContext` link SHALL read the current Supabase session on every request and attach `Authorization: Bearer <access_token>` when a session exists.

#### Scenario: Authenticated request

- **WHEN** the user is signed in
- **THEN** every GraphQL request includes `Authorization: Bearer <access_token>` in its headers

#### Scenario: Unauthenticated request

- **WHEN** the user is not signed in
- **THEN** no `Authorization` header is attached
- **AND** the API responds with 401 for protected operations

### Requirement: Environment variables match Supabase

The backend `.env` and the frontend `.env.example` SHALL provide the Supabase configuration variables (`SUPABASE_URL`, `SUPABASE_JWT_SECRET`, `PUBLIC_SUPABASE_URL`, `PUBLIC_SUPABASE_ANON_KEY`) and SHALL NOT include Keycloak variables.

#### Scenario: Backend env

- **WHEN** the API container starts
- **THEN** `SUPABASE_URL` is set
- **AND** `SUPABASE_JWT_SECRET` is set
- **AND** `AUTHORIZATION_AUDIENCE` is set to `authenticated`
- **AND** `KEYCLOAK_ISSUER`, `KEYCLOAK_REALM`, `QDRANT_URL` are absent or unset

#### Scenario: Frontend env

- **WHEN** the rsbuild dev server starts
- **THEN** `PUBLIC_SUPABASE_URL` and `PUBLIC_SUPABASE_ANON_KEY` are present
- **AND** `PUBLIC_KEYCLOAK_URL`, `PUBLIC_KEYCLOAK_REALM`, `PUBLIC_KEYCLOAK_CLIENT_ID`, `PUBLIC_KEYCLOAK_SCOPE` are absent

### Requirement: SupabaseAuthLayer required_roles uses OR semantics

The `SupabaseAuthLayer` SHALL evaluate the `required_roles` vector as a **disjunction (OR)**: a request is authorized when the JWT's `app_metadata.roles` JSON array contains **at least one** string that case-sensitively equals an entry in `required_roles`. The role match SHALL be a case-sensitive string equality check against the elements of the `app_metadata.roles` JSON array. An empty `required_roles` vec SHALL mean "no role requirement" — the role check is skipped entirely and the request proceeds as long as the JWT is valid.

#### Scenario: User holds a single required role

- **WHEN** a request carries a valid JWT whose `app_metadata.roles` is `["my-headless-cms-writer"]`
- **AND** the `SupabaseAuthLayer` is configured with `required_roles = ["my-headless-cms-writer", "my-headless-cms-administrator"]`
- **THEN** the middleware allows the request (HTTP 200)
- **AND** a `SupabaseToken` extension is available via `Extension<SupabaseToken>`

#### Scenario: User holds a different role

- **WHEN** a request carries a valid JWT whose `app_metadata.roles` is `["my-headless-cms-editor"]`
- **AND** the `SupabaseAuthLayer` is configured with `required_roles = ["my-headless-cms-writer", "my-headless-cms-administrator"]`
- **THEN** the middleware returns HTTP 403
- **AND** the response body is `{"error":"Insufficient permissions"}`

#### Scenario: User has no app_metadata.roles

- **WHEN** a request carries a valid JWT whose `app_metadata` is `{}` (no `roles` key) or whose `app_metadata` field is absent
- **AND** the `SupabaseAuthLayer` is configured with `required_roles = ["my-headless-cms-writer", "my-headless-cms-administrator"]`
- **THEN** the middleware returns HTTP 403
- **AND** the response body is `{"error":"Insufficient permissions"}`

#### Scenario: User holds multiple roles and at least one matches

- **WHEN** a request carries a valid JWT whose `app_metadata.roles` is `["my-headless-cms-editor", "my-headless-cms-writer"]`
- **AND** the `SupabaseAuthLayer` is configured with `required_roles = ["my-headless-cms-writer", "my-headless-cms-administrator"]`
- **THEN** the middleware allows the request (HTTP 200)
- **AND** a `SupabaseToken` extension is available via `Extension<SupabaseToken>`

#### Scenario: Empty required_roles disables role enforcement

- **WHEN** a request carries a valid JWT whose `app_metadata` is `{}` (no roles at all)
- **AND** the `SupabaseAuthLayer` is configured with `required_roles = []`
- **THEN** the middleware allows the request (HTTP 200)
- **AND** a `SupabaseToken` extension is available via `Extension<SupabaseToken>`

