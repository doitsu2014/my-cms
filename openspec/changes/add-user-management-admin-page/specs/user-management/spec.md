# user-management Specification (Delta)

> This is a **delta spec** for the new `user-management` capability.
> It will be merged into `openspec/specs/user-management/spec.md` on archive.

## Purpose

Provide administrators of My-CMS with a self-service admin surface to manage the operators who can sign in to the CMS. Backed by GoTrue's admin API; users live in `auth.users` and roles live in `app_metadata.roles`. The CMS neither creates its own user table nor duplicates user state.

## Requirements

### Requirement: Admin can list CMS users

The system SHALL expose `GET /users` on `protected_administrator_router()`. The endpoint SHALL list CMS users via GoTrue's `GET /auth/v1/admin/users`. The default sort SHALL be `created_at` descending. The response SHALL be an array of `AppUserModel` objects.

`AppUserModel` SHALL contain at minimum: `id` (GoTrue UUID), `email` (lowercased), `role` (the first element of `app_metadata.roles` if any, otherwise `null`), `banned` (boolean — `true` when GoTrue's `banned_until` is set and in the future), `createdAt`, `updatedAt`, `lastSignInAt` (nullable).

#### Scenario: Default list

- **WHEN** an authenticated administrator calls `GET /users` with no query parameters
- **THEN** the response is HTTP 200
- **AND** the body contains an array of `AppUserModel` objects sorted by `createdAt` descending
- **AND** the array length does not exceed `perPage` (default 50)

#### Scenario: Filter by role

- **WHEN** the request includes `?role=my-headless-cms-writer`
- **THEN** every returned `AppUserModel` has `role === "my-headless-cms-writer"`

#### Scenario: Filter by email substring

- **WHEN** the request includes `?email=alice` (case-insensitive substring)
- **THEN** every returned `AppUserModel` has an `email` containing `alice` (case-insensitive)

#### Scenario: Pagination

- **WHEN** the request includes `?page=2&perPage=10`
- **THEN** the response contains at most 10 users representing page 2 of the filtered set

#### Scenario: Invalid page number

- **WHEN** the request includes `?page=0` or `?perPage=0` or `?perPage=500`
- **THEN** the response is HTTP 400 with a `Validation` error code

#### Scenario: Unauthenticated request

- **WHEN** the request has no `Authorization` header
- **THEN** the response is HTTP 401

#### Scenario: Wrong role

- **WHEN** the JWT's `app_metadata.roles` does not contain `my-headless-cms-administrator`
- **THEN** the response is HTTP 403

### Requirement: Admin can fetch a single user by id

The system SHALL expose `GET /users/{user_id}` on `protected_administrator_router()`. The endpoint SHALL fetch one CMS user from GoTrue's `GET /auth/v1/admin/users/{user_id}` and return an `AppUserModel`.

#### Scenario: Existing user

- **WHEN** the path `{user_id}` is a valid UUID that exists in GoTrue
- **THEN** the response is HTTP 200 with an `AppUserModel` in the body

#### Scenario: Unknown user id

- **WHEN** the path `{user_id}` is a valid UUID that does not exist in GoTrue
- **THEN** the response is HTTP 404

#### Scenario: Malformed user id

- **WHEN** the path `{user_id}` is not a valid UUID
- **THEN** the response is HTTP 400 with a `Validation` error code

#### Scenario: Authentication required

- **WHEN** the request lacks a valid administrator JWT
- **THEN** the response is HTTP 401 (missing/invalid token) or HTTP 403 (wrong role)

### Requirement: Admin can create a CMS user

The system SHALL expose `POST /users` on `protected_administrator_router()`. The endpoint SHALL create a user in GoTrue via `POST /auth/v1/admin/users` with `email_confirm: true` and `app_metadata: { roles: [<role>] }`. The plaintext password supplied by the admin SHALL be returned exactly once in the create response and SHALL NOT be returned by any subsequent list/read call.

The request body SHALL contain: `email` (string), `password` (string, ≥ 8 characters), `role` (one of the recognised roles). The response body SHALL contain the created `AppUserModel` plus a `temporaryPassword` field equal to the password supplied in the request.

The recognised roles SHALL be exactly: `my-headless-cms-administrator` and `my-headless-cms-writer`.

#### Scenario: Valid create

- **WHEN** the request body is `{ email, password, role }` with a valid email, a password of at least 8 characters, and a recognised role
- **THEN** the response is HTTP 201
- **AND** the body contains an `AppUserModel` for the new user
- **AND** the body contains a `temporaryPassword` field equal to the password supplied in the request

#### Scenario: Email is normalised

- **WHEN** the request body is `{ email: "Alice@Example.COM", password, role }`
- **THEN** GoTrue stores the email lowercased as `alice@example.com`
- **AND** the response `AppUserModel.email` is `alice@example.com`

#### Scenario: Duplicate email

- **WHEN** the request body contains an email that already exists in GoTrue
- **THEN** the response is HTTP 409 with a `Conflict` error code

#### Scenario: Password too short

- **WHEN** the request body's `password` is fewer than 8 characters
- **THEN** the response is HTTP 400 with a `Validation` error code

#### Scenario: Unrecognised role

- **WHEN** the request body's `role` is not in the recognised list
- **THEN** the response is HTTP 400 with a `Validation` error code

#### Scenario: Invalid email format

- **WHEN** the request body's `email` is not a syntactically valid email
- **THEN** the response is HTTP 400 with a `Validation` error code

#### Scenario: Plaintext password never returned on read

- **WHEN** an administrator reads the newly created user via `GET /users/{user_id}` or lists it via `GET /users`
- **THEN** the response body does NOT contain any password field

#### Scenario: Authentication required

- **WHEN** the request lacks a valid administrator JWT
- **THEN** the response is HTTP 401 (missing/invalid token) or HTTP 403 (wrong role)

### Requirement: Admin can update a CMS user

The system SHALL expose `PUT /users/{user_id}` on `protected_administrator_router()`. The endpoint SHALL update the user's `email`, `app_metadata.roles`, and ban state in GoTrue via `PUT /auth/v1/admin/users/{user_id}`. The endpoint SHALL expose `banned` as a boolean in the API; the implementation SHALL translate `banned: true` to a far-future `ban_duration` (effectively permanent) and `banned: false` to clearing the ban.

#### Scenario: Update email and role

- **WHEN** the request body is `{ email, role, banned: <unchanged> }` and the target user exists
- **THEN** the response is HTTP 200 with the updated `AppUserModel`
- **AND** GoTrue stores the new email and `app_metadata.roles` containing exactly `[role]`

#### Scenario: Ban a user

- **WHEN** the request body is `{ banned: true }` and the target user exists
- **THEN** the response is HTTP 200 with the updated `AppUserModel`
- **AND** `AppUserModel.banned === true`

#### Scenario: Unban a user

- **WHEN** the request body is `{ banned: false }` and the target user is currently banned
- **THEN** the response is HTTP 200 with the updated `AppUserModel`
- **AND** `AppUserModel.banned === false`

#### Scenario: Email collision with another user

- **WHEN** the request body updates `email` to a value owned by another existing user
- **THEN** the response is HTTP 409 with a `Conflict` error code

#### Scenario: Unrecognised role

- **WHEN** the request body's `role` is not in the recognised list
- **THEN** the response is HTTP 400 with a `Validation` error code

#### Scenario: Unknown user id

- **WHEN** the path `{user_id}` does not exist in GoTrue
- **THEN** the response is HTTP 404

#### Scenario: Authentication required

- **WHEN** the request lacks a valid administrator JWT
- **THEN** the response is HTTP 401 (missing/invalid token) or HTTP 403 (wrong role)

### Requirement: Admin can delete a CMS user

The system SHALL expose `DELETE /users/{user_id}` on `protected_administrator_router()`. The endpoint SHALL hard-delete the user in GoTrue via `DELETE /auth/v1/admin/users/{user_id}`. An administrator SHALL NOT be able to delete their own account via this endpoint.

#### Scenario: Successful delete

- **WHEN** the path `{user_id}` exists in GoTrue and is not the caller's own id
- **THEN** the response is HTTP 204

#### Scenario: Self-delete is blocked

- **WHEN** the path `{user_id}` equals the caller JWT's `sub` claim
- **THEN** the response is HTTP 400 with a `Logical` error code
- **AND** the user is not deleted in GoTrue

#### Scenario: Unknown user id

- **WHEN** the path `{user_id}` does not exist in GoTrue
- **THEN** the response is HTTP 404

#### Scenario: Malformed user id

- **WHEN** the path `{user_id}` is not a valid UUID
- **THEN** the response is HTTP 400 with a `Validation` error code

#### Scenario: Authentication required

- **WHEN** the request lacks a valid administrator JWT
- **THEN** the response is HTTP 401 (missing/invalid token) or HTTP 403 (wrong role)

### Requirement: All /users endpoints are gated by the administrator role

The five `/users` endpoints SHALL be wired onto `protected_administrator_router()` and SHALL require `my-headless-cms-administrator` in `app_metadata.roles`. The existing `SupabaseAuthLayer` SHALL enforce this check; no per-handler role check is required.

#### Scenario: Administrator JWT is accepted

- **WHEN** any request to `/users`, `/users/{user_id}` carries a valid JWT whose `app_metadata.roles` contains `my-headless-cms-administrator`
- **THEN** the middleware does not return 401 or 403
- **AND** the request reaches the handler

#### Scenario: Writer JWT is rejected

- **WHEN** any request to `/users*` carries a valid JWT whose `app_metadata.roles` contains only `my-headless-cms-writer`
- **THEN** the middleware returns HTTP 403 with body `{"error":"Insufficient permissions"}`

#### Scenario: No role is rejected

- **WHEN** any request to `/users*` carries a valid JWT with no `app_metadata.roles`
- **THEN** the middleware returns HTTP 403

#### Scenario: Missing token is rejected

- **WHEN** any request to `/users*` has no `Authorization` header
- **THEN** the middleware returns HTTP 401

### Requirement: External service failures map to clean errors without leaking secrets

When GoTrue returns a non-2xx response or the HTTP call to GoTrue fails, the command handler SHALL map the failure to an `AppError` variant. The response body returned to the client SHALL NOT contain the `SERVICE_ROLE_KEY`, GoTrue-internal stack traces, or any field whose value equals the configured `SUPABASE_SERVICE_ROLE_KEY`. The log message SHALL NOT contain the `SERVICE_ROLE_KEY`.

#### Scenario: GoTrue 5xx

- **WHEN** GoTrue returns a 5xx response
- **THEN** the API response is HTTP 500
- **AND** the response body does not contain the `SERVICE_ROLE_KEY`

#### Scenario: GoTrue 422 validation error

- **WHEN** GoTrue returns 422 with a structured validation error
- **THEN** the API response is HTTP 400 with a `Validation` error code
- **AND** the response body contains a sanitised message (no raw GoTrue stack frames)

#### Scenario: Network failure

- **WHEN** the HTTP request to GoTrue fails (timeout, connection refused, DNS error)
- **THEN** the API response is HTTP 500 with a `ConnectionError` error code
- **AND** the response body does not contain the `SERVICE_ROLE_KEY`

#### Scenario: Secret never logged

- **WHEN** a request fails for any reason
- **THEN** no log line emitted by the API includes the configured `SUPABASE_SERVICE_ROLE_KEY` as a substring

### Requirement: Successful admin mutations emit an audit log event

Every successful Create, Update, and Delete command SHALL emit a tracing `info!` event containing `actor_user_id` (the caller's `sub` claim), `target_user_id` (the affected GoTrue user id), and `action` (`"create"` | `"update"` | `"delete"`). Failed operations SHALL NOT emit this event.

#### Scenario: Create emits audit event

- **WHEN** a successful `POST /users` completes
- **THEN** an info-level tracing event is emitted
- **AND** the event contains `actor_user_id`, `target_user_id`, `action = "create"`

#### Scenario: Update emits audit event

- **WHEN** a successful `PUT /users/{user_id}` completes
- **THEN** an info-level tracing event is emitted
- **AND** the event contains `actor_user_id`, `target_user_id`, `action = "update"`

#### Scenario: Delete emits audit event

- **WHEN** a successful `DELETE /users/{user_id}` completes
- **THEN** an info-level tracing event is emitted
- **AND** the event contains `actor_user_id`, `target_user_id`, `action = "delete"`

#### Scenario: Failed operation does not emit audit event

- **WHEN** a Create, Update, or Delete command returns an `AppError`
- **THEN** no info-level audit event is emitted