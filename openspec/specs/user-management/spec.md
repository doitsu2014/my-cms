# user-management Specification

## Purpose
TBD - created by archiving change add-user-profile-fields-and-reset-password. Update Purpose after archive.
## Requirements
### Requirement: AppUserModel carries profile fields

The `AppUserModel` returned by every user-management endpoint (`GET /users`, `GET /users/{user_id}`, `POST /users`, `PUT /users/{user_id}`) SHALL include two additional optional fields on top of the fields defined in the in-flight capability:

- `fullName: string | null` — read from GoTrue's `user_metadata.full_name`. The value SHALL be the JSON string when present, otherwise `null`. The key `full_name` is the single source of truth; the API SHALL NOT duplicate it into `app_metadata`.
- `phone: string | null` — read from the top-level `phone` column on GoTrue's `auth.users`. The value SHALL be the stored phone string when present, otherwise `null`.

The API SHALL NOT enable phone-based authentication as a side effect of writing the `phone` field. The `phone_confirm` flag SHALL remain `false` unless explicitly set by a separate, out-of-scope flow.

#### Scenario: Profile fields are present when set

- **WHEN** a GoTrue user has `user_metadata.full_name = "Alice Example"` and `phone = "+1 555-0100"`
- **THEN** the API response `AppUserModel.fullName === "Alice Example"`
- **AND** the API response `AppUserModel.phone === "+1 555-0100"`

#### Scenario: Profile fields are null when unset

- **WHEN** a GoTrue user has no `full_name` key in `user_metadata` and no `phone` value
- **THEN** the API response `AppUserModel.fullName === null`
- **AND** the API response `AppUserModel.phone === null`

#### Scenario: Profile fields are returned by the list endpoint

- **WHEN** an authenticated administrator calls `GET /users` and any user in the result has `full_name` or `phone` set
- **THEN** the response array contains the populated values for those users
- **AND** the response array contains `null` for users without those fields

#### Scenario: Profile fields are returned by the get-one endpoint

- **WHEN** an authenticated administrator calls `GET /users/{user_id}` for a user with `full_name` and `phone` set
- **THEN** the response body contains `fullName` and `phone` populated

### Requirement: Create user request accepts profile fields

The request body of `POST /users` SHALL accept two additional optional fields on top of the fields defined in the in-flight capability:

- `fullName: string` — when present, the API SHALL write it to GoTrue as `user_metadata.full_name`. The API SHALL reject values longer than 120 characters with HTTP 400 and a `Validation` error code. Empty string SHALL be normalised to "absent" (no `full_name` key written).
- `phone: string` — when present, the API SHALL write it to the top-level `phone` column on GoTrue. The value SHALL match strict E.164 (`^\+[1-9]\d{6,14}$`, 7–15 total digits, e.g. `+14155550100`). The API SHALL reject non-matching values with HTTP 400 and a `Validation` error code. Empty string SHALL be normalised to "absent" (no `phone` written).

When a field is absent from the request body, the API SHALL NOT write to the corresponding GoTrue field.

#### Scenario: Create with full name and phone

- **WHEN** the request body is `{ email, password, role, fullName: "Alice Example", phone: "+1 555-0100" }`
- **THEN** the response is HTTP 201
- **AND** GoTrue stores `user_metadata.full_name = "Alice Example"` and `phone = "+1 555-0100"`
- **AND** the response `AppUserModel.fullName === "Alice Example"`
- **AND** the response `AppUserModel.phone === "+1 555-0100"`

#### Scenario: Create without profile fields still works

- **WHEN** the request body is `{ email, password, role }` with no `fullName` or `phone`
- **THEN** the response is HTTP 201
- **AND** GoTrue stores no `full_name` key in `user_metadata` and no `phone` value
- **AND** the response `AppUserModel.fullName === null`
- **AND** the response `AppUserModel.phone === null`

#### Scenario: Reject overly long full name

- **WHEN** the request body's `fullName` is longer than 120 characters
- **THEN** the response is HTTP 400 with a `Validation` error code on the `fullName` field

#### Scenario: Reject malformed phone

- **WHEN** the request body's `phone` does not match strict E.164 (`^\+[1-9]\d{6,14}$`)
- **THEN** the response is HTTP 400 with a `Validation` error code on the `phone` field

#### Scenario: Existing create validations are unchanged

- **WHEN** the request body violates any of the in-flight capability's validations (invalid email, short password, unrecognised role)
- **THEN** the response is HTTP 400 with the corresponding `Validation` error code, and no profile-field change has been made to GoTrue

### Requirement: Modify user request accepts profile fields

The request body of `PUT /users/{user_id}` SHALL accept two additional optional fields on top of the fields defined in the in-flight capability:

- `fullName: string` — when present, the API SHALL write it to GoTrue as `user_metadata.full_name`. The same 120-character limit and empty-string normalisation as the create endpoint SHALL apply. When absent, the API SHALL NOT touch the user's `full_name`.
- `phone: string` — when present, the API SHALL write it to GoTrue's top-level `phone` column. The same strict E.164 pattern and empty-string normalisation as the create endpoint SHALL apply. When absent, the API SHALL NOT touch the user's `phone`.

The endpoint SHALL continue to use the in-flight capability's "patch not put" semantics: only fields that are explicitly present in the request body are written. The new fields follow the same rule.

#### Scenario: Update full name and phone

- **WHEN** the request body is `{ fullName: "Alice Newname", phone: "+44 20 7946 0958" }` and the target user exists
- **THEN** the response is HTTP 200 with the updated `AppUserModel`
- **AND** GoTrue stores `user_metadata.full_name = "Alice Newname"` and `phone = "+44 20 7946 0958"`

#### Scenario: Update leaves absent profile fields untouched

- **WHEN** the target user currently has `full_name = "Old Name"` and `phone = "+1 555-0100"`
- **AND** the request body is `{ role: "my-headless-cms-writer" }` (no `fullName`, no `phone`)
- **THEN** the response is HTTP 200
- **AND** GoTrue still stores `full_name = "Old Name"` and `phone = "+1 555-0100"`
- **AND** only the role has changed in GoTrue

#### Scenario: Update with empty-string profile fields leaves them untouched

- **WHEN** the target user currently has `full_name = "Old Name"` and `phone = "+1 555-0100"`
- **AND** the request body is `{ fullName: "", phone: "" }` (empty strings are treated as "no change")
- **THEN** the response is HTTP 200
- **AND** GoTrue still stores `full_name = "Old Name"` and `phone = "+1 555-0100"`
- **AND** the response `AppUserModel.fullName === "Old Name"` and `AppUserModel.phone === "+1 555-0100"`

#### Scenario: Reject invalid profile values on update

- **WHEN** the request body contains a `fullName` longer than 120 characters or a `phone` not matching strict E.164
- **THEN** the response is HTTP 400 with a `Validation` error code
- **AND** no field on the user has been changed in GoTrue

#### Scenario: Existing modify validations are unchanged

- **WHEN** the request body violates any of the in-flight capability's validations on email, role, or banned state
- **THEN** the response is HTTP 400 with the corresponding `Validation` error code, and no profile-field change has been made to GoTrue

### Requirement: Admin can reset a user's password

The system SHALL expose `POST /users/{user_id}/reset-password` on `protected_administrator_router()`. The endpoint SHALL set the target user's password in GoTrue by calling `PUT /auth/v1/admin/users/{user_id}` with the request body `{ password: <new> }`. The endpoint SHALL be wired onto the same router as the other user-management endpoints; no new auth wiring is required.

The request body SHALL contain `password: string` (≥ 8 characters). The same minimum-length rule as the create endpoint SHALL apply. The response body SHALL contain `{ temporaryPassword: string }` equal to the password supplied in the request. The plaintext password SHALL be returned exactly once in the reset response and SHALL NOT be returned by any subsequent list/read call.

A successful reset SHALL emit a tracing `info!` event containing `actor_user_id` (the caller's `sub` claim), `target_user_id` (the affected GoTrue user id), and `action = "reset_password"`. Failed operations SHALL NOT emit this event.

#### Scenario: Successful reset

- **WHEN** the path `{user_id}` exists in GoTrue and the request body is `{ password: "newsecret123" }`
- **THEN** the response is HTTP 200
- **AND** the body contains `{ temporaryPassword: "newsecret123" }`
- **AND** the target user can sign in to GoTrue with the new password
- **AND** an info-level tracing event is emitted with `action = "reset_password"`, `actor_user_id`, and `target_user_id`

#### Scenario: User signs in with the new password after reset

- **WHEN** an administrator resets a user's password via the endpoint
- **THEN** the user can immediately sign in to the CMS with the new password (no email confirmation, no recovery flow)

#### Scenario: Password too short

- **WHEN** the request body's `password` is fewer than 8 characters
- **THEN** the response is HTTP 400 with a `Validation` error code on the `password` field
- **AND** the user's password in GoTrue is unchanged

#### Scenario: Unknown user id

- **WHEN** the path `{user_id}` does not exist in GoTrue
- **THEN** the response is HTTP 404
- **AND** no password has been changed in GoTrue

#### Scenario: Malformed user id

- **WHEN** the path `{user_id}` is not a valid UUID
- **THEN** the response is HTTP 400 with a `Validation` error code

#### Scenario: Plaintext password never returned on read

- **WHEN** an administrator reads the affected user via `GET /users/{user_id}` or lists it via `GET /users` after a reset
- **THEN** the response body does NOT contain any password field

#### Scenario: Authentication required

- **WHEN** the request lacks a valid administrator JWT
- **THEN** the response is HTTP 401 (missing/invalid token) or HTTP 403 (wrong role)

#### Scenario: Failed reset does not emit audit event

- **WHEN** a reset call returns an `AppError` (validation, 404, GoTrue 4xx/5xx, network failure)
- **THEN** no info-level audit event with `action = "reset_password"` is emitted

### Requirement: Routes for modify, delete, and reset-password are wired with a path parameter

The router registration in `apps/api/src/bin/my-cms-api.rs::protected_administrator_router()` SHALL register `PUT /users/{user_id}` and `DELETE /users/{user_id}` (matching the `Path<Uuid>` signature of the corresponding handlers), and SHALL register `POST /users/{user_id}/reset-password` for the reset endpoint. The `GET` and `POST` registrations on `/users` (list, create) and the `GET` registration on `/users/{user_id}` (read one) SHALL remain unchanged. No handler SHALL be registered on `/users` for `PUT` or `DELETE` (those methods are reserved for the path-parameterised routes).

#### Scenario: Modify hits the right handler

- **WHEN** an authenticated administrator sends `PUT /users/{valid_uuid}` with a valid modify body
- **THEN** the request reaches `api_modify_user` and the response is HTTP 200 with the updated `AppUserModel`
- **AND** a `PUT /users` request (no id) returns HTTP 405 Method Not Allowed

#### Scenario: Delete hits the right handler

- **WHEN** an authenticated administrator sends `DELETE /users/{valid_uuid}` for an existing user
- **THEN** the request reaches `api_delete_user` and the response is HTTP 204
- **AND** a `DELETE /users` request (no id) returns HTTP 405 Method Not Allowed

#### Scenario: Reset-password hits the right handler

- **WHEN** an authenticated administrator sends `POST /users/{valid_uuid}/reset-password` with `{ password }`
- **THEN** the request reaches `api_reset_password` and the response is HTTP 200 with `{ temporaryPassword }`

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

