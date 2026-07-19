## Why

The CMS user-management admin page, delivered by the in-flight `add-user-management-admin-page` change, lets administrators create, edit, and delete CMS operators — but the user record it manages is a skeleton. Today's `AppUserModel` carries only `id, email, role, banned, createdAt, updatedAt, lastSignInAt`. Real operators have a **name** and a **phone** for escalation; today an admin who wants to know "who is this user" has to read the email address and guess. There is also no way to recover an account whose password was lost: the create flow hands the admin a one-time password at provisioning, but if that note is gone, the admin's only option is to delete and re-create the user, which loses the user's existing data and role.

On top of that, the `add-user-management-admin-page` change has a latent bug: `PUT /users/{user_id}` and `DELETE /users/{user_id}` are registered on `/users` (no id) in `apps/api/src/bin/my-cms-api.rs:213-219`, while the handlers extract `Path<Uuid>` — so every edit and delete from the admin panel returns 405. The new profile fields and the new reset-password endpoint cannot be exercised end-to-end until that is fixed, so the routing fix ships in the same change.

## What Changes

**Backend (`apps/api/`)**

- Fix the routing bug in `apps/api/src/bin/my-cms-api.rs::protected_administrator_router()`: re-register `PUT` and `DELETE` on `/users/{user_id}` (where the handlers already expect `Path<Uuid>`). The current `PUT/DELETE` registrations on `/users` are removed; the existing `POST` and `GET` registrations stay.
- Extend `application_core/src/commands/user/dto.rs::AppUserModel` with two optional fields: `full_name: Option<String>` and `phone: Option<String>`. Both are sourced from GoTrue: `full_name` is read from `user_metadata.full_name`; `phone` is read from the top-level `phone` column on `auth.users`.
- Extend `application_core/src/commands/user/create/create_request.rs::CreateUserRequest` with optional `full_name` and `phone`. Extend `application_core/src/commands/user/modify/modify_request.rs::ModifyUserRequest` with optional `full_name` and `phone`.
- Update `application_core/src/commands/user/supabase_admin_client.rs`:
  - Extend `GoTrueUserResponse` and `parse_gotrue_user` to read `user_metadata.full_name` and `phone`.
  - Extend `create_user` to send `user_metadata: { full_name: <value> }` and `phone: <value>` when the request carries them. `email_confirm: true` and `app_metadata.roles` behaviour is unchanged.
  - Extend `update_user` to send `user_metadata` and `phone` deltas on `PUT /auth/v1/admin/users/{id}` when the request carries them.
  - Add a new method `reset_password(id, new_password) -> ()` that calls `PUT /auth/v1/admin/users/{id}` with `{ password: <new> }` and surfaces GoTrue errors through the existing `map_gotrue_error` path. The method is a write-only side effect; it returns `()`.
- Add a new `ResetPasswordHandler` in `application_core/src/commands/user/reset_password/` following the existing trait+struct pattern. Request DTO: `{ password: String }` (≥ 8 characters, validated by the handler, identical rule to create). The handler logs the audit event with `action = "reset_password"`.
- Add the API handler `api_reset_password` in `apps/api/src/api/user/reset_password/`, wired onto `protected_administrator_router()` at `POST /users/{user_id}/reset-password`. The response body is `{ temporaryPassword: <new> }` so the admin can copy it to the user out-of-band (matches the create flow).
- Add a new request DTO `ResetPasswordRequest` in `application_core/src/commands/user/reset_password/reset_password_request.rs`.

**Frontend (`apps/web/`)**

- Extend `src/domains/user.ts::AppUserModel` with optional `fullName: string | null` and `phone: string | null`.
- Extend `src/domains/user.ts::CreateUserRequest` and `ModifyUserRequest` with optional `fullName?: string | null` and `phone?: string | null`.
- Extend `src/schemas/user.schema.ts` with optional `fullName` (max 120 chars) and `phone` (strict E.164: `^\+[1-9]\d{6,14}$`, 7–15 total digits, e.g. `+14155550100`). Both fields are optional; absent is fine.
- Update `src/app/admin/users/user-form.tsx`:
  - Add **Full name** input (text, max 120 chars). Always visible (create and edit).
  - Add **Phone** input (tel, with strict E.164 validation). Always visible.
  - In edit mode, replace the create-only password field with a **"Reset password"** button that opens a modal containing a single password input. On submit, the modal calls `POST /users/{id}/reset-password` and shows the new password in a toast (same pattern as the create success toast: "Share this password securely: …", 30s duration).
- Update `src/app/admin/users/page.tsx`:
  - Add `fullName` and `phone` columns to the user table.
  - Add sort/filter for `fullName` (email-substring-style case-insensitive search if a fullName filter input is added; out of scope for v1 — just the column for now).
- No new navigation, no new sidebar entry.

**OpenSpec specs**

- Extend the `user-management` capability (currently a delta in the in-flight `add-user-management-admin-page` change) with three new requirements: `AppUserModel` profile fields, full_name/phone on create and modify, and admin-can-reset-password. The existing requirements for list/read/create/update/delete are unchanged — the new fields and the new endpoint are additive, so the delta uses `## ADDED Requirements` exclusively and does not MODIFY any existing requirement (which keeps the two open deltas merge-safe in either archive order).

## Capabilities

### New Capabilities

None.

### Modified Capabilities

- `user-management` — add three new requirements covering profile fields, full_name/phone on the create/modify endpoints, and admin-triggered password reset. The existing `user-management` delta in `add-user-management-admin-page` is unchanged; the two deltas compose when both are archived.

## Impact

| Layer | Impact |
|---|---|
| **API routing** | One-line fix: move `PUT` and `DELETE` registrations from `/users` to `/users/{user_id}` in `my-cms-api.rs:213-219`. Add one new route `POST /users/{user_id}/reset-password`. |
| **Application core** | Extend `AppUserModel`, `CreateUserRequest`, `ModifyUserRequest`. Extend `SupabaseAdminClient` with `user_metadata`/`phone` round-tripping and add `reset_password()`. Add a new `ResetPasswordHandler` (trait + struct). All existing command handler tests stay green; new tests added for the new fields and the new handler. |
| **Auth middleware** | None. The new endpoint is wired onto the existing `protected_administrator_router()`. |
| **GoTrue contract** | `user_metadata` and `phone` are already first-class on `auth.users` — no GoTrue migration, no schema change, no new env var. |
| **Database** | None. GoTrue remains the source of truth. |
| **GraphQL schema** | None. REST-only. |
| **Frontend pages** | `user-form.tsx` grows two new inputs and replaces the edit-mode password field with a "Reset password" button + modal. `page.tsx` grows two new columns. New Zod schema entries; new DTO entries. |
| **Frontend navigation** | None. |
| **Environment variables** | None. |
| **In-flight change** | Resolves the blocked task 3.3 of the `fix-docker-apps-service-role-key-mismatch` change — the credential chain was already verified, only the 405 from the broken routes prevented the create+modify+delete smoke. Once this change lands, the operator can finish that smoke. No edit to the in-flight change is required. |

## Open Questions

1. **`full_name` location** — chosen **A: `user_metadata.full_name`** (standard GoTrue convention, user-visible if a profile page is added later, no schema migration). If you'd rather keep it admin-only (option B, `app_metadata.full_name`) say so before implementation and I'll flip it.
2. **Phone validation** — chosen **strict E.164 (`^\+[1-9]\d{6,14}$`, 7–15 total digits, e.g. `+14155550100`)**. The original design proposed a loose pattern (`+?[0-9 \-()]{6,20}`) but a runtime smoke showed that GoTrue's `phone` column enforces strict E.164 at the database level — a loose API pattern would let through values that GoTrue rejects, producing an opaque 400 in the user-facing form. Switching the API to strict E.164 catches the mismatch at the form layer with a single, clear validation error. Updated inline during implementation; the design.md and spec.md reflect the new pattern.
3. **Password reset UX** — chosen **A: admin sets a new password** (matches the create flow, no SMTP dependency, returns the new password in the response for the admin to share). Alternative **B (recovery email)** is on the table if you'd rather not have the admin handle plaintext passwords — but `SITE_URL` is currently the admin frontend, so the recovery link would need a new user-facing route to land on.
4. **Profile fields on the user list** — chosen **two new columns (Full name, Phone)** with no filtering/sorting in v1. If you'd rather hide them behind an "expand" affordance to keep the table compact, say so.
