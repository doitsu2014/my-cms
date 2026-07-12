## Context

The CMS user-management admin page, delivered by the in-flight `add-user-management-admin-page` change, exposes a list/create/read/modify/delete surface backed by GoTrue's admin API. That change is 27/28 tasks complete; the remaining task is a manual smoke. A latent bug in the route registration was found while smoke-testing the prior `fix-docker-apps-service-role-key-mismatch` change: `PUT /users/{user_id}` and `DELETE /users/{user_id}` are wired on `/users` (no id), so Axum returns 405. The bug is independent of credentials and is blocking the modify and delete verbs from working at all.

In parallel, the user-management data model is too thin for the operator's day-to-day use. `AppUserModel` today has only `id, email, role, banned, createdAt, updatedAt, lastSignInAt` — no display name, no phone, no path to reset a forgotten password without delete+recreate. The admin who provisioned a user has the temporary password; once that note is gone, the user is unrecoverable.

GoTrue already has first-class slots for everything we need:

- `user_metadata` — a JSON blob on `auth.users` keyed by name. The conventional place for `full_name`.
- `phone` — a real column on `auth.users`. Admin API accepts it via `PUT /admin/users/{id}`. Setting it does **not** enable phone-based auth.
- `password` — admin API accepts a `password` field on `PUT /admin/users/{id}` for an admin-initiated reset.

So this change is a data-model extension + an endpoint addition + a routing fix. No GoTrue migration, no new env var, no new entity.

## Goals / Non-Goals

**Goals:**

- Add `full_name` and `phone` to the user record end-to-end (DB → DTO → request → response → form → table).
- Add an admin-only `POST /users/{user_id}/reset-password` endpoint that sets a new password and returns it once for the admin to share with the user out-of-band.
- Fix the routing bug in `my-cms-api.rs::protected_administrator_router()` so the existing `PUT` and `DELETE` handlers on `/users/{user_id}` actually respond.
- Keep the new delta merge-safe with the in-flight `add-user-management-admin-page` delta on the same `user-management` capability.
- Keep all existing command-handler tests green; add new tests for the new fields and the new endpoint.

**Non-Goals:**

- Storing `full_name` or `phone` in our own Postgres table. GoTrue is the source of truth for user records; duplicating into a separate table creates a drift problem.
- Enabling phone-based authentication. The `phone` field is informational; no `phone_change`, no confirmation flow, no OTP.
- User self-service profile changes (a user-facing profile page). Out of scope for an admin-only system.
- Bulk import / CSV upload.
- Soft-delete or "trash" view. Hard delete remains the only delete.
- Audit log persistence. The existing `info!` tracing event with `action = "reset_password"` matches the pattern; no audit table.
- A recovery-email flow. Picked option A (admin sets new password) to keep the change self-contained and avoid routing a GoTrue recovery link to a user-facing page that doesn't exist.

## Decisions

### 1. Fix the routing bug in this change, not as a separate Fast Fix

**Decision:** Re-register `PUT` and `DELETE` on `/users/{user_id}` in `my-cms-api.rs:213-219` as part of this change.

**Reason:** The new endpoints and the new fields cannot be exercised end-to-end through the admin panel while the modify/delete routes return 405. Folding the fix in here means the verification gate for this change is meaningful — the smoke test runs against a working modify endpoint. A separate Fast Fix would be correct but would leave this change unverifiable in the meantime.

**Alternatives considered:**

- *Two separate changes* (routing fix first, then feature) — rejected for the reason above; it defers the smoke that proves the feature works.
- *Move the fix into the in-flight `add-user-management-admin-page` change* — rejected: that change is 27/28 tasks done and arguably should have caught the bug. Re-opening it to add a fix would invalidate its task list and require re-running all 28 tasks.

### 2. `full_name` lives in `user_metadata.full_name`

**Decision:** Store the display name in GoTrue's `user_metadata.full_name` (option A from the explore discussion).

**Reason:** This is the standard GoTrue convention; tools, dashboards, and SDKs that read `user_metadata` already understand it. If a user-facing profile page is ever added, the user can see and edit their own name with no schema change. `app_metadata` is reserved for admin-controlled data (roles live there), and conflating profile fields with permission-bearing data is an anti-pattern.

**Alternatives considered:**

- *Option B (`app_metadata.full_name`)* — admin-only writes, but invisible to the user. Defeats the point of having a display name.
- *Option C (own Postgres table)* — most flexible, but introduces a second source of truth that can drift from GoTrue. Out of proportion to "display a name in a list".

### 3. `phone` lives in GoTrue's top-level `phone` column, no confirmation

**Decision:** Send `phone` as a top-level field on the GoTrue admin create/update bodies. Do **not** set `phone_confirm`. Do **not** enable phone auth. The API validates the value as strict E.164 (`^\+[1-9]\d{6,14}$`, 7–15 total digits) before forwarding to GoTrue.

**Reason (revised during implementation):** `phone` is the only first-class slot for phone numbers in GoTrue. The initial design proposed a loose pattern (`+?[0-9 \-()]{6,20}`) — but a runtime smoke revealed that GoTrue's `phone` column enforces strict E.164 at the database level, returning `400 "Invalid phone number format (E.164 required)"` for any value that does not match. A loose pattern at the API layer would let through values that GoTrue would reject, surfacing as a 400 in the user-facing form with no client-side hint. Switching the API to strict E.164 catches the mismatch at the form layer and produces a single, clear error. The `phone_confirm` flag remains `false`; the field is still informational from the CMS perspective (we do not send SMS), but the value is now guaranteed to be a globally-dialable format.

**Alternative considered:** Keep the loose pattern and rely on GoTrue's error to inform the user. Rejected because (a) the error message is verbose and not localised, (b) the loose pattern would silently truncate international formats that GoTrue accepts (e.g. `+44 (0) 20 7946 0958` → `+44 20 7946 0958`), and (c) the friendly pattern that GoTrue actually accepts is small enough to write down explicitly.

### 3a. `fullName` and `phone` cannot be cleared via the modify endpoint in v1

**Decision:** The modify endpoint SHALL treat an empty string for `fullName` or `phone` as "no change" (same as absent). Clearing these fields via the API is out of scope for v1.

**Reason (revised during implementation):** The original design said empty string means "clear the field" — `user_metadata: {}` would clear `user_metadata` in GoTrue, and an empty `phone` would clear the column. The runtime smoke showed that GoTrue's `phone` column rejects `""` outright with a 400; sending `user_metadata: {}` to clear is supported but the asymmetry between the two fields makes the UX confusing. The simpler v1 semantics are: empty/absent means "leave the existing value alone". The frontend form will not offer a "clear" button; if an admin needs to clear a field, they can set it to a different valid value. A future change can add explicit "clear" support with the right GoTrue-side semantics (probably a `phone_change: null` token).

### 4. Password reset: admin sets a new password, response carries it once

**Decision:** `POST /users/{user_id}/reset-password` accepts `{ password: string }`, calls GoTrue `PUT /admin/users/{id}` with `{ password: <new> }`, and returns `{ temporaryPassword: <new> }` to the admin. The frontend shows the password in a 30-second toast (same pattern as the create success toast).

**Reason:** This matches the create flow's "admin sets the password" pattern. The admin already has to deliver the password securely at provisioning; using the same pattern for reset means operators only need to remember one mental model. No SMTP dependency in dev or prod (Mailpit is the dev SMTP sink, but the recovery link target would have to be a new user-facing route — out of scope).

**Alternatives considered:**

- *Option B (admin triggers a recovery email)* — would require a new user-facing page at `SITE_URL` to handle the link, plus a `generate_link` call. The GoTrue `generate_link` endpoint works with our current setup, but the landing page is a new feature.
- *Option C (user self-service profile page)* — requires a user-facing auth flow that does not exist today.

### 5. New delta is `## ADDED Requirements` only — no `## MODIFIED Requirements`

**Decision:** The new `user-management` delta in this change adds three new requirements and modifies none of the existing ones. The existing in-flight delta in `add-user-management-admin-page` defines the current shape of the capability; this change extends it additively.

**Reason:** Two deltas to the same capability can land in either archive order. ADDED-only means our delta is idempotent against the in-flight one regardless of which archive runs first. MODIFY-ing an existing requirement (e.g. "Admin can create a CMS user" to add `full_name`/`phone` to the request body) would risk merge conflicts if the other change edits the same requirement first.

**Specific additions** (all new requirements, no edits to existing ones):

- `AppUserModel carries profile fields` — the model SHALL include `full_name: Option<String>` (from `user_metadata.full_name`) and `phone: Option<String>` (from the top-level `phone` column) on every read path (list, get, create response, update response, reset response). Both fields SHALL be `null` when unset.
- `Create user request accepts profile fields` — `POST /users` SHALL accept optional `fullName` and `phone`. When present, the API SHALL write them to `user_metadata.full_name` and the top-level `phone` field on GoTrue.
- `Modify user request accepts profile fields` — `PUT /users/{user_id}` SHALL accept optional `fullName` and `phone`. When present, the API SHALL update the corresponding GoTrue fields. When absent, the API SHALL NOT touch those fields (preserves "patch not put" semantics, matches the existing email/role/banned pattern).
- `Admin can reset a user's password` — `POST /users/{user_id}/reset-password` SHALL accept `{ password: string }` (≥ 8 characters) and SHALL set the user's password in GoTrue via `PUT /admin/users/{id}` with `{ password: <new> }`. The response body SHALL contain `{ temporaryPassword: <new> }`. The endpoint SHALL be wired onto `protected_administrator_router()`. A successful reset SHALL emit a tracing `info!` event with `action = "reset_password"`, `actor_user_id`, and `target_user_id`.

### 6. The new endpoint and the routing fix share a route registration

**Decision:** In the same edit to `my-cms-api.rs::protected_administrator_router()`, add the new route `POST /users/{user_id}/reset-password` alongside the corrected `PUT`/`DELETE` registrations. One router change, one verification.

**Reason:** Keeps the change small and atomic. The new route is also `protected_administrator_router()`-gated by construction; no extra auth wiring.

### 7. The `SupabaseAdminClient.reset_password` is a write-only side effect

**Decision:** The new method on `SupabaseAdminClient` is `pub async fn reset_password(&self, id: Uuid, new_password: &str) -> Result<(), AppError>`. It does not return the updated user (GoTrue's response is not authoritative for the new password — only the admin's input is). The command handler that wraps it logs the audit event and returns `{ temporaryPassword: new_password.to_string() }` to the API layer.

**Reason:** The "return the new password" contract belongs at the command layer (which owns the temporaryPassword shape), not the transport layer (which only knows the GoTrue response). The transport layer's job is to push the value to GoTrue; the command layer's job is to remember what was sent.

### 8. Frontend: edit-mode password field becomes a "Reset password" button

**Decision:** In `user-form.tsx` edit mode, the existing `password` input is removed. In its place, a "Reset password" button opens a DaisyUI modal with a single password input. Submitting the modal calls `POST /users/{id}/reset-password` and shows the new password in a 30-second toast (mirrors the create flow's password-return toast).

**Reason:** The edit form already has email/role/banned; bundling "set a new password" into the same form would conflate two different write paths (modify vs reset) and require the user to type a password they don't need to set. The button + modal mirrors the delete confirmation modal pattern already in `page.tsx`, so the UX is consistent.

**Alternatives considered:**

- *Inline "set new password" field on the edit form* — visually noisy when no reset is needed; mixes two intents.
- *Per-row "Reset" action in the list table* — hides the form context; admin can't see who they're resetting.

## Architecture

```
┌─────────────────────────────────────────────────────────────────┐
│  my-cms-api (Axum)                                              │
│  ─────────────────                                              │
│                                                                 │
│  protected_administrator_router()                               │
│  ├── /users                                                    │
│  │   ├── GET    → api_list_users     (unchanged)               │
│  │   └── POST   → api_create_user    (now: +fullName, +phone)  │
│  ├── /users/{user_id}                                          │
│  │   ├── GET    → api_get_user       (unchanged)               │
│  │   ├── PUT    → api_modify_user    (now: +fullName, +phone)  │
│  │   │                         (route: was on /users, now fixed) │
│  │   ├── DELETE → api_delete_user    (route: was on /users, fixed) │
│  │   └── POST   → api_reset_password (NEW)                    │
│  └── /administrator/database/migration (unchanged)             │
└─────────────────────────────────────────────────────────────────┘
                    │
                    ▼  SupabaseAdminClient
        ┌───────────────────────────────────┐
        │  GoTrue admin API                 │
        │  /auth/v1/admin/users             │
        │  /auth/v1/admin/users/{id}        │
        │                                   │
        │  user_metadata: { full_name }     │
        │  phone: "+1 555-…"                │
        │  password: "new-secret"           │
        │  email, app_metadata.roles …      │
        └───────────────────────────────────┘

┌─────────────────────────────────────────────────────────────────┐
│  React admin (apps/web)                                         │
│  ────────────────────────                                       │
│                                                                 │
│  /admin/users                                                   │
│  ├── table: Email | Full name | Phone | Role | Status | Actions│
│  └── create / edit form:                                        │
│      ├── Email                                                  │
│      ├── Full name        ← NEW                                │
│      ├── Phone             ← NEW                                │
│      ├── Role                                                     │
│      ├── Status (edit only)                                          │
│      └── (edit only) "Reset password" button → modal     ← NEW  │
└─────────────────────────────────────────────────────────────────┘
```

### Per-file changes

| File | Change |
|---|---|
| `apps/api/src/bin/my-cms-api.rs` | Move `PUT`/`DELETE` from `/users` to `/users/{user_id}`; add `POST /users/{user_id}/reset-password`; import the new handler module |
| `apps/api/application_core/src/commands/user/dto.rs` | Add `full_name: Option<String>`, `phone: Option<String>` to `AppUserModel` |
| `apps/api/application_core/src/commands/user/create/create_request.rs` | Add `full_name: Option<String>`, `phone: Option<String>` to `CreateUserRequest` |
| `apps/api/application_core/src/commands/user/create/create_handler.rs` | Validate optional `full_name` (≤ 120 chars), `phone` (loose pattern); pass through to `SupabaseAdminClient::create_user` |
| `apps/api/application_core/src/commands/user/modify/modify_request.rs` | Add `full_name: Option<String>`, `phone: Option<String>` to `ModifyUserRequest` |
| `apps/api/application_core/src/commands/user/modify/modify_handler.rs` | Validate optional fields; pass through to `SupabaseAdminClient::update_user` |
| `apps/api/application_core/src/commands/user/supabase_admin_client.rs` | Extend `GoTrueUserResponse` and `parse_gotrue_user`; extend `create_user` and `update_user` to send `user_metadata` / `phone`; add `reset_password(id, new_password)` |
| `apps/api/application_core/src/commands/user/reset_password/` (NEW) | `mod.rs`, `reset_password_request.rs` (DTO + validation), `reset_password_handler.rs` (trait + struct, audit log) |
| `apps/api/src/api/user/reset_password/` (NEW) | `reset_password_handler.rs` (Axum `Path` + `Json` extraction, calls command handler) |
| `apps/api/src/api/user/mod.rs` (if exists) | `pub mod reset_password;` |
| `apps/api/src/api/user/create/create_handler.rs` | No code change required; field pass-through is handled in the command handler |
| `apps/api/src/api/user/modify/modify_handler.rs` | No code change required; field pass-through is handled in the command handler |
| `apps/web/src/domains/user.ts` | Add `fullName`, `phone` to `AppUserModel`, `CreateUserRequest`, `ModifyUserRequest`; add `ResetPasswordRequest`, `ResetPasswordResponse` |
| `apps/web/src/schemas/user.schema.ts` | Add optional `fullName` (≤ 120 chars) and `phone` (loose pattern) to `createUserSchema` and `modifyUserSchema`; add `resetPasswordSchema` (≥ 8 chars) |
| `apps/web/src/app/admin/users/user-form.tsx` | Add `Full name` + `Phone` inputs; in edit mode, replace password field with "Reset password" button + modal; modal calls `POST /users/{id}/reset-password` and toasts the result |
| `apps/web/src/app/admin/users/page.tsx` | Add `Full name` and `Phone` columns to the user table |
| `openspec/changes/add-user-profile-fields-and-reset-password/specs/user-management/spec.md` | New delta: `## ADDED Requirements` block with three requirements (profile fields on model, profile fields on create/modify, admin-can-reset-password) |

## Risks / Trade-offs

- **Routing fix changes a route that may have been intentionally scoped to `/users` in the in-flight change** → would be a surprise to that change's reviewer. *Mitigation:* the existing handler signatures take `Path<Uuid>`, which proves the original intent was `/users/{user_id}`. The current registration is a typo. The new registration is the obvious correction.
- **`user_metadata.full_name` is a JSON key the team has to remember** → typo in the key silently breaks the round-trip. *Mitigation:* the value is round-tripped through a typed Rust struct and a typed TS interface; the typo would surface as `fullName === null` in the API response and fail the smoke test.
- **The loose phone regex accepts strings that are not dialable** → the field is informational, but a future change that consumes it (e.g. sending SMS) would have to re-validate. *Mitigation:* out of scope; the spec is clear that phone is informational in this change.
- **Admin sees the new password in a 30-second toast** → if the admin's screen is visible, the password leaks. *Mitigation:* matches the create flow's existing pattern; if a higher-security alternative is wanted later, that is its own change (e.g. require re-auth, copy-to-clipboard only, etc.).
- **Two deltas on `user-management` open simultaneously** → archive order matters if either MODIFIED an existing requirement. *Mitigation:* this change is ADDED-only, so order is irrelevant. Both deltas compose by concatenation of new requirements.
- **`reset_password` reuses the same audit `action` enum as create/update/delete** → adds a fourth value. *Mitigation:* the existing `info!` event already includes a free-form `action` string; no schema change needed.

## Migration Plan

Non-breaking, additive. The existing API surface continues to work (all new fields are optional, all new request bodies can omit them). The new endpoint is purely additive. The routing fix corrects a route that no current caller was successfully reaching (it was 405-ing), so the change has no observable behavior shift for callers that were already getting errors.

Steps:
1. Pull the change.
2. `cd apps/api && cargo check && cargo test` — all green.
3. `cd apps/api && cargo fmt -- --check && cargo clippy --all-targets -- -D warnings` — green.
4. `pnpm --dir apps/web build` — green.
5. Restart the API container: `docker compose -f deployments/docker-swarm/apps/docker-compose.yaml --env-file deployments/docker-swarm/apps/.env up -d my-cms-api`.
6. Smoke: sign in as the seeded admin, create a user with `fullName="Alice Example"`, `phone="+1 555-0100"`, verify the user appears in the list with those fields. Edit the user, change `fullName`, save, reload, verify the change persisted. Click "Reset password", set a new password, copy it from the toast, sign out and sign back in as that user with the new password. Delete the test user.

**Rollback:** revert the change. No data migration, no DB migration, no frontend deploy. The new `fullName`/`phone` columns disappear from the table; the data in GoTrue is still there but ignored.

## Open Questions

1. **Phone validation strictness** — default is loose. If you want strict E.164, say so before implementation.
2. **`full_name` location** — default is `user_metadata.full_name`. If you want `app_metadata.full_name` (admin-only), say so before implementation.
3. **Profile columns on the list** — default is two new columns. If you'd rather hide them behind a per-row "expand" affordance to keep the table compact, say so.
4. **Sort/filter on `full_name`** — out of scope for v1. If you want it, it can be added.
