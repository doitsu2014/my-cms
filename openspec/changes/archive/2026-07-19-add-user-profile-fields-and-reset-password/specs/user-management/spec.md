# user-management Specification (Delta)

> This is an **additive delta** for the existing `user-management` capability
> (defined by the in-flight `add-user-management-admin-page` change). It uses
> `## ADDED Requirements` exclusively so the two open deltas compose regardless
> of archive order. It will be merged into `openspec/specs/user-management/spec.md`
> on archive.

## ADDED Requirements

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
