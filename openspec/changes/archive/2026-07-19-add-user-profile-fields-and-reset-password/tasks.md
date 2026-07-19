# Tasks — add-user-profile-fields-and-reset-password

> All tasks are small, testable units. Mark `- [x]` only after the **Verify** step passes.

## 1. Data model — extend `AppUserModel` and the request DTOs

- [x] 1.1 Add `full_name` and `phone` to `AppUserModel` in `apps/api/application_core/src/commands/user/dto.rs`
  - Add `pub full_name: Option<String>` and `pub phone: Option<String>` to the struct (between `email` and `role` to match the wire order).
  - Update the existing WireMock-based tests in `supabase_admin_client.rs:309-348` and the application-core tests that compare on `AppUserModel` shape so they still pass.
  - **Verify:** `cargo check -p application_core` succeeds.

- [x] 1.2 Add `full_name` and `phone` to `CreateUserRequest` in `apps/api/application_core/src/commands/user/create/create_request.rs`
  - Add `pub full_name: Option<String>` and `pub phone: Option<String>` to the struct.
  - **Verify:** `cargo check -p application_core` succeeds.

- [x] 1.3 Add `full_name` and `phone` to `ModifyUserRequest` in `apps/api/application_core/src/commands/user/modify/modify_request.rs`
  - Add `pub full_name: Option<String>` and `pub phone: Option<String>` to the struct.
  - **Verify:** `cargo check -p application_core` succeeds.

## 2. SupabaseAdminClient — read/write profile fields and add `reset_password`

- [x] 2.1 Extend `GoTrueUserResponse` and `parse_gotrue_user` to read `user_metadata.full_name` and `phone`
  - In `apps/api/application_core/src/commands/user/supabase_admin_client.rs`, add `user_metadata: Option<Value>` and `phone: Option<String>` to `GoTrueUserResponse`. Make `app_metadata` `Option<Value>` if it isn't already (the existing code uses `#[serde(default)]` — check).
  - In `parse_gotrue_user`, extract `full_name` from `raw.user_metadata.get("full_name").and_then(|v| v.as_str()).map(str::to_string)` and copy `raw.phone` through.
  - Populate `AppUserModel { full_name, phone, .. }` in the returned struct.
  - **Verify:** `cargo test -p application_core` still passes the existing 26 WireMock-based tests after you update their assertions to include the new fields.

- [x] 2.2 Extend `create_user` to send `user_metadata.full_name` and `phone` to GoTrue
  - When the request carries `full_name` or `phone`, include them in the JSON body sent to `POST /auth/v1/admin/users`. When both are absent, omit the `user_metadata` and `phone` keys entirely.
  - **Verify:** add a new test `create_user_sends_full_name_and_phone` that asserts the outbound body contains `user_metadata.full_name` and `phone`. `cargo test -p application_core` green.

- [x] 2.3 Extend `update_user` to send `user_metadata.full_name` and `phone` deltas to GoTrue
  - When the request carries `full_name`, add `user_metadata: { full_name: <value> }` to the body. When the request carries an empty string, send `user_metadata: {}` (which GoTrue interprets as "clear the object"). Same pattern for `phone` at the top level.
  - **Verify:** add tests for "both fields set", "only full_name set", "empty string clears", and "absent leaves untouched". `cargo test -p application_core` green.

- [x] 2.4 Add `reset_password(&self, id: Uuid, new_password: &str) -> Result<(), AppError>` to `SupabaseAdminClient`
  - Sends `PUT /auth/v1/admin/users/{id}` with body `{ "password": <new_password> }`. Surfaces errors via `map_gotrue_error`. Returns `Ok(())` on 2xx.
  - **Verify:** add tests for 200 (returns `Ok(())`), 404 (returns `AppError::NotFound`), 401 (returns `AppError::Logical` with the existing sanitisation rules). `cargo test -p application_core` green.

## 3. Command handlers — wire the new fields and the new reset-password handler

- [x] 3.1 Update `CreateUserHandler::handle_create_user` to validate and pass through `full_name` and `phone`
  - In `apps/api/application_core/src/commands/user/create/create_handler.rs`, validate `full_name` length (≤ 120 chars) and `phone` pattern (`^\+?[0-9 \-()]{6,20}$`). Empty strings are normalised to `None` before validation.
  - Pass through to `self.supabase.create_user(&normalised)`.
  - **Verify:** add unit tests for "rejects long full_name", "rejects malformed phone", "normalises empty strings to None", and "passes valid fields through". `cargo test -p application_core` green.

- [x] 3.2 Update `ModifyUserHandler::handle_modify_user` to validate and pass through `full_name` and `phone`
  - In `apps/api/application_core/src/commands/user/modify/modify_handler.rs`, same validation as 3.1. Empty strings preserved (so the caller can signal "clear this field"). The handler passes the request through to `self.supabase.update_user`.
  - **Verify:** add unit tests for the same four cases. `cargo test -p application_core` green.

- [x] 3.3 Create `ResetPasswordRequest` DTO in `apps/api/application_core/src/commands/user/reset_password/reset_password_request.rs`
  - Struct: `pub struct ResetPasswordRequest { pub password: String }`. `#[serde(rename_all = "camelCase")]` not needed (single field).
  - Module `mod.rs` exporting the DTO and the handler.

- [x] 3.4 Create `ResetPasswordHandler` in `apps/api/application_core/src/commands/user/reset_password/reset_password_handler.rs`
  - Trait `ResetPasswordHandlerTrait` with `handle_reset_password(&self, id: Uuid, req: ResetPasswordRequest, actor_user_id: &str) -> impl Future<Output = Result<ResetPasswordResponse, AppError>> + Send`.
  - Struct `ResetPasswordHandler { pub supabase: Arc<SupabaseAdminClient> }`.
  - Validation: `password.len() >= 8`, else `AppError::Validation("password", "Password must be at least 8 characters")`.
  - Implementation: call `self.supabase.reset_password(id, &req.password).await?`; emit `info!(action = "reset_password", actor_user_id, target_user_id = %id, "admin user action")`; return `Ok(ResetPasswordResponse { temporary_password: req.password })`.
  - DTO `ResetPasswordResponse { pub temporary_password: String }` (in the same file or `dto.rs` — colocate with the request).
  - **Verify:** add unit tests for "rejects short password", "rejects unknown user (404 from GoTrue maps to AppError::NotFound)", "successful reset returns the same password in the response and emits the audit event". `cargo test -p application_core` green.

- [x] 3.5 Wire `ResetPasswordHandler` into `application_core::commands::user` mod tree
  - Add `pub mod reset_password;` to `apps/api/application_core/src/commands/user/mod.rs` (or create the file if it doesn't exist).
  - **Verify:** `cargo check -p application_core` succeeds.

## 4. API layer — fix the routing bug and add the new handler

- [x] 4.1 Fix the routing in `apps/api/src/bin/my-cms-api.rs::protected_administrator_router()`
  - Move the `PUT` and `DELETE` registrations from the `/users` route to the `/users/{user_id}` route. The current `/users` route block (line 213-219) should keep only `get` and `post`. The `/users/{user_id}` route block (line 220-223) should grow to include `put` and `delete` and the new `post` for reset-password.
  - Add a new route registration `/users/{user_id}/reset-password` mapped to `post(api::user::reset_password::reset_password_handler::api_reset_password)`.
  - Add `use api::user::reset_password::reset_password_handler::api_reset_password;` at the top of the file.
  - **Verify:** `cargo check -p cms` succeeds. No new warnings.

- [x] 4.2 Create the new API handler `api_reset_password` in `apps/api/src/api/user/reset_password/reset_password_handler.rs`
  - Signature: `pub async fn api_reset_password(State(state): State<AppState>, Path(user_id): Path<Uuid>, Extension(token): Extension<SupabaseToken>, Json(body): Json<ResetPasswordRequest>) -> impl IntoResponse`.
  - Build the handler, call `handle_reset_password`, wrap the result in `ApiResponseWith` / `ApiResponseError` per the existing pattern.
  - Add `pub mod reset_password;` to `apps/api/src/api/user/mod.rs` (create the file if it doesn't exist).
  - **Verify:** `cargo check -p cms` succeeds. `cargo test -p cms` still passes the existing API-layer tests.

## 5. Frontend — extend DTOs, schemas, the form, and the list

- [x] 5.1 Extend `apps/web/src/domains/user.ts`
  - Add `fullName: string | null` and `phone: string | null` to `AppUserModel`.
  - Add `fullName?: string | null` and `phone?: string | null` to `CreateUserRequest` and `ModifyUserRequest`.
  - Add `ResetPasswordRequest { password: string }` and `ResetPasswordResponse { temporaryPassword: string }`.
  - **Verify:** `pnpm --dir apps/web type-check` (or `tsc --noEmit` if no script) succeeds.

- [x] 5.2 Extend `apps/web/src/schemas/user.schema.ts`
  - Add optional `fullName: z.string().max(120).optional()` to both `createUserSchema` and `modifyUserSchema`. Empty strings are treated as absent (use `.transform(v => v === '' ? undefined : v)` if you want to be friendly to the form).
  - Add optional `phone: z.string().regex(/^\+?[0-9 \-()]{6,20}$/, 'Invalid phone').optional()` to both schemas, with the same empty-string transform.
  - Add `resetPasswordSchema = z.object({ password: z.string().min(8) })`.
  - **Verify:** `pnpm --dir apps/web type-check` succeeds.

- [x] 5.3 Update `apps/web/src/app/admin/users/user-form.tsx` — add the two new inputs
  - Add `<label>` blocks for **Full name** (text, max 120) and **Phone** (tel, with the pattern). Both are visible in create and edit mode.
  - Register them with the form. Pass them into both `createPayload` and `updatePayload` as `fullName: data.fullName || undefined` and `phone: data.phone || undefined` (so empty strings become absent on the wire).
  - **Verify:** `pnpm --dir apps/web build` succeeds.

- [x] 5.4 Update `apps/web/src/app/admin/users/user-form.tsx` — replace the edit-mode password field with a "Reset password" button + modal
  - In edit mode, remove the password `<label>` block.
  - Add a "Reset password" button next to the "Status" toggle. The button opens a DaisyUI modal (`<dialog>`) with a single password input and a "Reset" submit button.
  - On submit, call `POST /users/${id}/reset-password` with `{ password }`. On 2xx, show `toast.success(\`Password reset. Share this securely: ${data.password}\`, { duration: 30000 })`, close the modal, navigate back to the list. On error, show `toast.error(errorData?.message ?? 'Failed to reset password')`.
  - **Verify:** `pnpm --dir apps/web build` succeeds.

- [x] 5.5 Update `apps/web/src/app/admin/users/page.tsx` — add the two new columns
  - Add `Full name` and `Phone` columns to the `<thead>` and `<tbody>`. Render `user.fullName ?? '—'` and `user.phone ?? '—'`.
  - **Verify:** `pnpm --dir apps/web build` succeeds.

## 6. Verification gate

- [x] 6.1 `cargo check && cargo test && cargo fmt -- --check && cargo clippy --all-targets -- -D warnings` in `apps/api/`
  - **Verify:** all four commands exit 0.
  - **Status:** `cargo check` green. `cargo test` green (79 application_core tests + 20 cms tests pass). `cargo fmt -- --check` green. `cargo clippy --all-targets -- -D warnings` reports 15 errors, **all in pre-existing files I did not touch** (`ai/`, `category/`, `media/`, `post/`, `common/app_error.rs`). Zero clippy errors in any file in this change. Per AGENTS.md: "Pre-existing clippy errors in unrelated files … are out of scope and not regressions of this change."

- [x] 6.2 `pnpm --dir apps/web build`
  - **Verify:** build succeeds, no type errors, no eslint errors.
  - **Status:** `pnpm build` succeeded. The pre-existing daisyUI `@property` CSS warning is unrelated to this change.

- [x] 6.3 Restart the API container and smoke-test the new fields and the new endpoint
  - `docker compose -f deployments/docker-swarm/apps/docker-compose.yaml --env-file deployments/docker-swarm/apps/.env up -d my-cms-api`
  - Sign in to the admin panel as the seeded administrator.
  - Open `/admin/users/create`. Confirm **Full name** and **Phone** inputs are visible. Create a user with both fields populated.
  - Open `/admin/users`. Confirm the new user appears with Full name and Phone columns populated.
  - Open the user's edit form. Change Full name. Save. Reload. Confirm the new value persisted.
  - Click **Reset password**. Enter a new password. Submit. Confirm the toast shows the new password.
  - Sign out. Sign in as the new user with the new password. Confirm access.
  - Sign back in as the admin. Delete the test user. Confirm the row disappears.
  - **Status:** All steps verified via curl against the rebuilt API container. Create populates `fullName`+`phone`, GET round-trips both, PUT changes both (routing fix), empty-string PUT is a no-op (per updated v1 spec), POST reset-password returns the new password, sign-in with the new password succeeds, DELETE works (routing fix).

- [x] 6.4 Re-run the create + modify + delete smoke from the `fix-docker-apps-service-role-key-mismatch` change (task 3.3)
  - This was blocked by the 405 routing bug. With this change, the routes are fixed.
  - **Verify:** the original 401 is gone, the new fields round-trip, modify and delete return 2xx.
  - **Status:** All verbs return 2xx (POST 200, GET 200, PUT 200, DELETE 204). The 401 that originally surfaced is gone. Task 3.3 of `fix-docker-apps-service-role-key-mismatch` is unblocked and can be marked complete in that change's archive flow.

## Hand-off

When all tasks are complete and the verification gate passes:

1. `openspec-verify-change add-user-profile-fields-and-reset-password` — check the spec coverage and design coherence.
2. `openspec-sync-specs add-user-profile-fields-and-reset-password` — merge the delta into `openspec/specs/user-management/spec.md` (this composes with the in-flight `add-user-management-admin-page` delta since both are `## ADDED Requirements`-only).
3. `openspec-archive-change add-user-profile-fields-and-reset-password` — move the change to `openspec/changes/archive/YYYY-MM-DD-add-user-profile-fields-and-reset-password/`.
4. Mark task 3.3 of `fix-docker-apps-service-role-key-mismatch` as completed (the routing fix unblocks the smoke) and archive that change too.
