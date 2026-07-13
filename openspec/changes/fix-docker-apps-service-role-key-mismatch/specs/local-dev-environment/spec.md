## MODIFIED Requirements

### Requirement: Two per-stack env files with shared values synchronised

The repository SHALL provide two env templates at the project root: `.env.supabase.example` (variables consumed by `docker-compose.supabase.yaml`) and `.env.my-cms.example` (variables consumed by `docker-compose.my-cms.yaml`). Variables that are consumed by both stacks — at minimum `POSTGRES_PASSWORD`, `JWT_SECRET`, `ANON_KEY`, `SERVICE_ROLE_KEY`, `SUPABASE_PUBLIC_URL`, `SITE_URL`, `API_EXTERNAL_URL` — SHALL appear in both env files with a header comment reading `KEEP IN SYNC with .env.{other}` on the value. The `SERVICE_ROLE_KEY` value SHALL be a real HS256 JWT signed with the project's `JWT_SECRET` on both sides; the literal placeholder `devkey` SHALL NOT be used in any committed env file or template.

#### Scenario: New developer onboarding

- **WHEN** a new developer clones the repository and copies both env templates
- **THEN** they only need to set `POSTGRES_PASSWORD` and `JWT_SECRET` (and any other secrets) in `.env.supabase`, and mirror those values into `.env.my-cms`
- **AND** no other env file is required to bring either stack online

#### Scenario: Shared value drift on `POSTGRES_PASSWORD`

- **WHEN** a developer sets `POSTGRES_PASSWORD=alpha` in `.env.supabase` and `POSTGRES_PASSWORD=beta` in `.env.my-cms`
- **THEN** the apps compose's `migrate` or `my-cms-api` services fail to authenticate to the Supabase `db`
- **AND** the error message in the affected service log identifies the role and password mismatch

#### Scenario: Shared value drift on `SERVICE_ROLE_KEY` (GoTrue admin API rejects the API container)

- **WHEN** `.env.supabase` contains a real HS256 JWT for `SERVICE_ROLE_KEY` (the value GoTrue is started with)
- **AND** `.env.my-cms` contains the literal placeholder string `devkey` for `SERVICE_ROLE_KEY`
- **THEN** the `my-cms-api` container's outbound call to `GET /auth/v1/admin/users` (or any other GoTrue admin endpoint) reaches GoTrue successfully
- **AND** GoTrue returns HTTP 401 with a response body containing `{"message":"Invalid authentication credentials"}`
- **AND** the API's `SupabaseAdminClient` surfaces this as `AppError::Logical("GoTrue list users authorisation error (401 Unauthorized): …")` (or the equivalent `GoTrue <verb> users` message for non-list endpoints)
