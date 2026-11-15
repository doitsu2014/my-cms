# Auth, users, and media

Validate: `apps/api/domain_auth/`, `apps/api/domain_user/`,
`apps/api/domain_media/`, `apps/web` auth and admin user/media pages,
`deployments/docker-swarm/` GoTrue settings.

Auth middleware rules that apply to every route are in
`backend-domain-architecture.md`.

## Supabase auth

Protected and administrator routes accept Supabase GoTrue JWTs. Audience and
signature are checked. The admin app signs in with password, restores and
clears the session, and sends the bearer token on authenticated GraphQL calls.

- Service-role credentials stay on the server.
- Local development disables public sign-up.
- Supabase reset seeds an administrator account idempotently.
- Kong forwards authenticated Supabase API calls in the local stack.
- `SUPABASE_URL` is required for the public/auth URL. Historically
  `SUPABASE_INTERNAL_URL` is the in-Docker URL the API uses to reach GoTrue
  and Storage. The auth factory returns `Result` on missing env; it does not
  panic. Construction emits a tracing event and MUST NOT log secrets.
- The auth factory test helper lives with `test_lock`.

## Users

`domain_user` serves administrator routes:

`POST /users`, `GET /users`, `GET /users/:id`, `PUT /users/:id`,
`DELETE /users/:id`, `POST /users/:id/reset-password`.

- WHEN any `/users` route is called without the administrator role, THEN it is
  rejected before the handler.
- List supports filter and pagination and returns the normalized user model.
- Create and update validate email, recognised role, password, ban state, and
  optional profile fields.
- `full_name` is `user_metadata.full_name`. Phone is GoTrue's top-level phone
  and MUST be strict E.164. Saving a phone MUST NOT enable phone auth.
- Password reset returns the temporary password only on that response, never
  on later reads.
- Self-delete is rejected.
- Successful admin mutations emit an audit event with actor, target, and
  action. Upstream failures map to clean errors and MUST NOT leak the
  service-role secret or raw upstream secrets.

## Media and buckets

`domain_media` talks to Supabase Storage with the service-role key. Object
operations take an explicit bucket. If the request does not override the
bucket, the configured default (`MEDIA_BUCKET`) is used. Response URLs keep
the bucket query when the caller supplied one.

`MediaConfig::from_env()` reads `SUPABASE_URL`, `SUPABASE_SERVICE_ROLE_KEY`,
`MEDIA_BUCKET`, and `MEDIA_BASE_URL`. A missing value fails startup before
the listener binds.

Public delivery: a public bucket is readable; a private bucket is obscured as
404 for anonymous and non-admin callers. Visibility may be cached with the
specified TTL and capacity. Cache keys include bucket, path, and dimensions.
Storage failures map to API errors without leaking credentials.

Bucket admin (administrator mount, service-role required):

- List, get, create, partial update, empty (bucket remains), delete (purge
  allowed).
- Names match `^[a-z][a-z0-9_-]{2,62}$`.
- Create defaults to private.
- The configured default bucket is reserved: it cannot be created or deleted
  as an ordinary bucket.
- The admin media browser selects a bucket in the URL and sends that override
  on object calls.

## Image transformation

Resized reads proxy bytes from Supabase Image Transformation. The API does
not decode images itself and does not redirect the browser to an internal
render URL.

- WHEN `w` or `h` is non-empty, THEN the handler returns transformed bytes
  with content type and cache headers and no `Location`.
- WHEN no effective resize parameter is present, THEN the original bytes are
  returned.
- Upstream 404 is not found. Upstream 5xx is a storage error.
- The render URL includes requested dimensions. The download uses Storage
  auth headers.
- The resize cache key includes the bucket. A requested bucket such as
  `avatars` is forwarded. Older wording that forced every resize onto the
  default bucket is superseded by this rule.
