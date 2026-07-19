## ADDED Requirements

### Requirement: Backend uses Docker-internal Supabase URL distinct from the public one

The my-cms apps Compose stack SHALL expose a `SUPABASE_INTERNAL_URL` env var to the `my-cms-api` service. The value SHALL be the URL the API container uses for all outbound HTTP calls to Supabase (Kong) — typically `http://supabase-kong:8000` when running under Docker Compose on the shared `supabase_network`. When `SUPABASE_INTERNAL_URL` is unset, the API SHALL fall back to `SUPABASE_URL` (the existing host-facing value), preserving backward compatibility for non-Docker workflows such as `cargo run` and host-side scripts.

The `my-cms-api` container SHALL use `SUPABASE_INTERNAL_URL` (with fallback) as the `supabase_url` for `SupabaseStorage`, `SupabaseAdminClient`, and `SupabaseAuthLayer` (JWKS fallback). The frontend, host-side scripts, and any browser-direct callers SHALL continue to use `SUPABASE_URL` / `PUBLIC_SUPABASE_URL`.

#### Scenario: API container reaches GoTrue admin endpoints via Docker DNS

- **WHEN** the `my-cms-api` service starts with `SUPABASE_URL=http://localhost:8000` (unreachable from inside the container) and `SUPABASE_INTERNAL_URL=http://supabase-kong:8000`
- **AND** an authenticated administrator calls `GET /users`
- **THEN** the API's `SupabaseAdminClient::list_users()` issues a request to `http://supabase-kong:8000/auth/v1/admin/users?page=1&per_page=200`
- **AND** the request resolves through the shared `supabase_network` to the Kong container
- **AND** the API returns HTTP 200 with the user list

#### Scenario: Storage endpoints resolve through Docker DNS

- **WHEN** the `my-cms-api` service starts with `SUPABASE_INTERNAL_URL=http://supabase-kong:8000`
- **AND** an authenticated user calls `GET /media`
- **THEN** the API's `SupabaseStorage::list()` issues requests to `http://supabase-kong:8000/storage/v1/...`
- **AND** the request resolves through the shared `supabase_network` to Kong
- **AND** the API returns the media list

#### Scenario: Host-side cargo run falls back to SUPABASE_URL

- **WHEN** a developer runs `cargo run` from `apps/api` on the host with `SUPABASE_URL=http://localhost:8000` and `SUPABASE_INTERNAL_URL` unset
- **THEN** the API resolves `SUPABASE_INTERNAL_URL` to the value of `SUPABASE_URL`
- **AND** `SupabaseStorage`, `SupabaseAdminClient`, and `SupabaseAuthLayer` use `http://localhost:8000` for outbound calls
- **AND** the API behaves identically to the pre-change behavior (Kong is reached via the host-exposed port)

#### Scenario: Frontend still uses PUBLIC_SUPABASE_URL

- **WHEN** the React admin calls `supabase.auth.signInWithPassword(...)`
- **THEN** the `@supabase/supabase-js` client uses `PUBLIC_SUPABASE_URL` (browser-direct)
- **AND** no `SUPABASE_INTERNAL_URL` is required on the frontend

#### Scenario: Misconfigured internal URL surfaces a clear connection error

- **WHEN** the `my-cms-api` service starts with `SUPABASE_INTERNAL_URL=http://nonexistent-host:9999`
- **AND** an authenticated user calls `GET /users`
- **THEN** the API returns HTTP 500 with an `ApiResponseError` whose message indicates a connection failure
- **AND** the error message does not include any secret value (mirroring the sanitisation property of `SupabaseAdminClient`)
