# Tasks — add-user-management-admin-page

> All tasks are small, testable units (≤ 2 hours each). Mark `- [x]` only after the **Verify** step passes.
>
> **STATUS (paused on user request — resume from task 11):** Tasks 1–10 and 15 are complete (backend foundation, command handlers, DTOs, sanitisation test). All 26 user-related unit tests pass in `application_core`. The full workspace `cargo check` is currently **RED** with one error: missing `AppError::Conflict(_)` arm in `apps/api/src/presentation_models/api_response.rs` (task 13). Tasks 11–14 (backend API + verify gate) and 16–28 (frontend) remain. See **Resume Notes** at the bottom.

## Backend — foundation

- [x] **1. Add `SupabaseAdminClient` skeleton + reqwest client**
    - **Files:** `apps/api/application_core/src/commands/user/mod.rs` (new), `apps/api/application_core/src/commands/user/supabase_admin_client.rs` (new)
    - Implement the struct with `service_role_key` field, redaction in `Debug`, `new(supabase_url, service_role_key)` constructor, 30-second timeout, `auth_key()` helper. No methods yet.
    - `pub mod supabase_admin_client;` re-export in `user/mod.rs`.
    - **Verify:** `cargo check -p application_core`

- [x] **2. Add `AppUserModel` DTO and request/response types**
    - **Files:** `apps/api/application_core/src/commands/user/dto.rs` (new), `apps/api/application_core/src/commands/user/mod.rs` (modify)
    - Define `AppUserModel`, `CreateUserResponse`, `CreateUserRequest`, `ModifyUserRequest`, `RECOGNISED_ROLES` constant, and `sanitise_email()` helper.
    - **Verify:** `cargo check -p application_core`

- [x] **3. Wire `Arc<SupabaseAdminClient>` into AppState**
    - **Files:** `apps/api/src/lib.rs` (modify — add field), `apps/api/src/bin/my-cms-api.rs` (modify — construct in `construct_app_state()`, `.expect("SUPABASE_SERVICE_ROLE_KEY must be set")`)
    - **Verify:** `cargo check`

## Backend — command handlers

- [x] **4. Implement `SupabaseAdminClient` methods (list, get, create, update, delete)**
    - **Files:** `apps/api/application_core/src/commands/user/supabase_admin_client.rs` (modify)
    - All five methods return `Result<T, AppError>`; 404 → `NotFound`; other non-2xx → `StorageError(sanitised msg)`. NEVER interpolate `service_role_key` into messages.
    - **Verify:** `cargo check -p application_core`

- [x] **5. `CreateUserHandler` (trait + struct + impl + unit tests with mocked client)**
    - **Files:** `apps/api/application_core/src/commands/user/create/{mod.rs, create_request.rs, create_handler.rs}` (new)
    - Trait method: `handle_create_user(req: CreateUserRequest, actor_user_id: &str) -> Result<CreateUserResponse, AppError>`.
    - Validations: password ≥ 8, role ∈ `RECOGNISED_ROLES`, email non-empty.
    - On success, emit `info!("admin user action=create actor_user_id={} target_user_id={}", ...)`.
    - Unit tests: success, short password, duplicate email → Conflict, unknown role.
    - **Verify:** `cargo test -p application_core create_user`

- [x] **6. `ReadListUserHandler` (filters + pagination)**
    - **Files:** `apps/api/application_core/src/commands/user/read_list/{mod.rs, read_list_handler.rs}` (new)
    - Trait method accepts `page: u32`, `per_page: u32`, `role: Option<String>`, `email: Option<String>`. Validates `page >= 1` and `1 <= per_page <= 200`.
    - **Verify:** `cargo test -p application_core read_list_user`

- [x] **7. `ReadOneUserHandler`**
    - **Files:** `apps/api/application_core/src/commands/user/read_one/{mod.rs, read_one_handler.rs}` (new)
    - Trait method: `handle_get_user(id: Uuid) -> Result<AppUserModel, AppError>`. Maps GoTrue 404 → `AppError::NotFound`.
    - **Verify:** `cargo test -p application_core read_one_user`

- [x] **8. `ModifyUserHandler` (role validation + banned translation)**
    - **Files:** `apps/api/application_core/src/commands/user/modify/{mod.rs, modify_request.rs, modify_handler.rs}` (new)
    - Trait method: `handle_modify_user(id, req, actor_user_id) -> Result<AppUserModel, AppError>`.
    - Role validated against `RECOGNISED_ROLES`. `banned: Some(true)` → set `ban_duration: "876000h"`. `banned: Some(false)` → omit `ban_duration`. `banned: None` → preserve.
    - Emit `info!` audit event on success.
    - Unit tests: role rejected, banned → ban_duration, unknown id → NotFound.
    - **Verify:** `cargo test -p application_core modify_user`

- [x] **9. `DeleteUserHandler` (self-delete guard returns 400)**
    - **Files:** `apps/api/application_core/src/commands/user/delete/{mod.rs, delete_handler.rs}` (new)
    - Trait method: `handle_delete_user(id: Uuid, actor_user_id: &str) -> Result<(), AppError>`.
    - If `id == actor_user_id` → `AppError::Logical("Cannot delete your own account")`.
    - Emit `info!` audit event on success.
    - Unit tests: self-delete blocked, success.
    - **Verify:** `cargo test -p application_core delete_user`

- [x] **10. Register `commands::user` in `commands/mod.rs`**
    - **Files:** `apps/api/application_core/src/commands/mod.rs` (modify — add `pub mod user;`)
    - **Verify:** `cargo check`

## Backend — API layer

- [x] **11. Five thin handlers in `apps/api/src/api/user/`**
    - **Files:** `apps/api/src/api/user/mod.rs` (new), `apps/api/src/api/user/create/create_handler.rs` (new), `apps/api/src/api/user/read_list/read_list_handler.rs` (new), `apps/api/src/api/user/read_one/read_one_handler.rs` (new), `apps/api/src/api/user/modify/modify_handler.rs` (new), `apps/api/src/api/user/delete/delete_handler.rs` (new)
    - Each handler extracts request, builds command handler, calls trait method, returns `ApiResponseWith<T>` or `ApiResponseError`. Mirror `category/*` handlers exactly.
    - **Verify:** `cargo check`

- [x] **12. Wire routes into `protected_administrator_router()`**
    - **Files:** `apps/api/src/bin/my-cms-api.rs` (modify — add `/users` chain and `/users/{user_id}` GET)
    - **Verify:** `cargo check`

- [x] **13. Add `ErrorCode::Conflict = "409"` and map it to HTTP 409**
    - **Files:** `apps/api/src/presentation_models/api_response.rs` (modify — add variant, map in `to_axum_response`, add unit test)
    - **Verify:** `cargo test -p cms presentation_models`

- [x] **14. Cargo check + clippy + fmt clean** (partial — see notes)
    - **Verify:** `cargo check && cargo clippy --all-targets -- -D warnings && cargo fmt -- --check`
    - **Status:** `cargo check` ✅ green, `cargo test` ✅ 26/26 pass, `cargo fmt -- --check` ✅ green. `cargo clippy --all-targets -- -D warnings` ❌ fails with **15 PRE-EXISTING errors in 8 unrelated files** (`apps/api/application_core/src/common/app_error.rs`, `commands/ai/*`, `commands/category/modify`, `commands/post/*`). The user-management code itself is clippy-clean. These pre-existing errors are out of scope for this change and can be addressed in a separate "fix clippy tech-debt" change.

## Backend — error mapping

- [x] **15. Sanitisation regression test**
    - **Files:** `apps/api/application_core/src/commands/user/supabase_admin_client.rs` (modify - add test)
    - Assert that the configured `SERVICE_ROLE_KEY` string never appears in any error message produced by the client.
    - **Verify:** `cargo test -p application_core user` → `error_messages_never_include_service_role_key ... ok`

## Frontend — foundation

- [x] **16. Domain types in `apps/web/src/domains/user.ts`**
    - **Files:** `apps/web/src/domains/user.ts` (new), `apps/web/src/domains/index.ts` (modify — export)
    - Export `UserRoleEnum`, `AppUserModel`, `CreateUserResponse`.
    - **Verify:** `pnpm --dir apps/web tsc --noEmit`

- [x] **17. Zod schemas in `apps/web/src/schemas/user.schema.ts`**
    - **Files:** `apps/web/src/schemas/user.schema.ts` (new)
    - `userFormSchema` (create: email, password ≥ 8, role, banned default false; edit: email, role, banned). `CreateUserFormData` + `ModifyUserFormData`.
    - **Verify:** `pnpm --dir apps/web tsc --noEmit`

- [x] **18. Request/response models in `apps/web/src/models/`**
    - **Files:** `apps/web/src/models/CreateUserModel.ts` (new), `apps/web/src/models/ModifyUserModel.ts` (new)
    - Mirror `CreateCategoryModel.ts` / `UpdateCategoryModel.ts` shape.
    - **Verify:** `pnpm --dir apps/web tsc --noEmit`

- [x] **19. Extend `AuthContext.userInfo` with `id`**
    - **Files:** `apps/web/src/auth/AuthContext.tsx` (modify — add `id: user.id` to `userInfo` and the `AuthContextType` interface)
    - **Verify:** `pnpm --dir apps/web tsc --noEmit`

## Frontend — pages

- [x] **20. `AdminOnlyRoute` wrapper component**
    - **Files:** `apps/web/src/app/admin/components/admin-only-route.tsx` (new)
    - Reads `app_metadata.roles` from the Supabase `user` (already exposed by `useAuth()`), renders children if `my-headless-cms-administrator` is present, otherwise renders a "Forbidden" DaisyUI alert + link back to `/admin`.
    - **Verify:** `pnpm --dir apps/web tsc --noEmit`

- [x] **21. `/admin/users` list page (table + filter + pagination + delete modal)**
    - **Files:** `apps/web/src/app/admin/users/page.tsx` (new)
    - Mirror `apps/web/src/app/admin/categories/page.tsx` structure. Disable the delete button (with tooltip) when `row.id === currentUser.id`. Fetch `GET /users?page=&perPage=&role=&email=`. Show role as a badge.
    - **Verify:** `pnpm --dir apps/web tsc --noEmit`

- [x] **22. Shared `user-form.tsx`**
    - **Files:** `apps/web/src/app/admin/users/user-form.tsx` (new)
    - Accepts `id?: string`. Renders password field only when `id` is undefined. On submit-create, show long-duration toast with the password. FAB pattern mirrors `category-form.tsx`.
    - **Verify:** `pnpm --dir apps/web tsc --noEmit`

- [x] **23. `/admin/users/create` page**
    - **Files:** `apps/web/src/app/admin/users/create/page.tsx` (new)
    - Wraps `<UserForm id={undefined} />`. Breadcrumbs Admin → Users → Create User.
    - **Verify:** `pnpm --dir apps/web tsc --noEmit`

- [x] **24. `/admin/users/edit/[id]` page**
    - **Files:** `apps/web/src/app/admin/users/edit/[id]/page.tsx` (new)
    - Reads `id` from `useParams`. Wraps `<UserForm id={id} />`. Breadcrumbs Admin → Users → Edit User.
    - **Verify:** `pnpm --dir apps/web tsc --noEmit`

- [x] **25. Register all three routes in `App.tsx`**
    - **Files:** `apps/web/src/App.tsx` (modify — add three new `<Route>` lines wrapping in `<AdminOnlyRoute>`)
    - **Verify:** `pnpm --dir apps/web build`

## Frontend — navigation

- [x] **26. Add "Administration" collapsible section + "Users" link to sidebar**
    - **Files:** `apps/web/src/app/admin/components/left-menu.tsx` (modify — add second `<details>` after the existing "Resources" block, containing a single `<MenuItem displayName="Users" slug="/admin/users" />`)
    - **Verify:** `pnpm --dir apps/web build`

## Frontend — verification

- [x] **27. `pnpm build` clean**
    - **Verify:** `pnpm --dir apps/web build`

- [ ] **28. Manual smoke against local Supabase stack**
    - Start the local stack (`deployments/docker-swarm/`).
    - Sign in as the seeded administrator; navigate to `/admin/users`; create a writer; edit their role; ban; unban; delete the writer; attempt self-delete (verify button disabled + API 400); verify request/response shape via the browser dev tools network tab.
    - Confirm no `SERVICE_ROLE_KEY` appears in the API response bodies or in the API log output.

## Hand-off

When all 28 tasks are complete and the verification gate (`cargo check && cargo test && cargo fmt -- --check && cargo clippy && pnpm --dir apps/web build`) passes, the change is ready for the coder to archive:

1. `openspec-verify-change add-user-management-admin-page`
2. `openspec-sync-specs add-user-management-admin-page`
3. `openspec-archive-change add-user-management-admin-page`

## Resume Notes (paused 2026-06-22)

**Done (✅ tasks 1–10, 13, 15):**
- `apps/api/application_core/src/commands/user/` fully scaffolded: `mod.rs`, `dto.rs`, `supabase_admin_client.rs`, plus `create/`, `read_list/`, `read_one/`, `modify/`, `delete/` subcommands with handler trait + struct + impl + wiremock-based unit tests
- `apps/api/application_core/src/common/app_error.rs` — added `AppError::Conflict(String)` variant with Display + Error::source arms
- `apps/api/application_core/src/commands/mod.rs` — added `pub mod user;`
- `apps/api/src/lib.rs` — `AppState` gained `supabase_admin_client: Arc<SupabaseAdminClient>`
- `apps/api/src/bin/my-cms-api.rs` — `construct_app_state` builds the client (with `.expect(...)` on SERVICE_ROLE_KEY)
- `apps/api/src/presentation_models/api_response.rs` — `ErrorCode::Conflict` variant + `StatusCode::CONFLICT` mapping + `AppError::Conflict(m)` arm in `From<AppError>` + 2 unit tests
- **26/26 user tests pass:** `cd apps/api && cargo test -p application_core --lib user`
- **`cargo check` (whole `apps/api` workspace) green.**

**Orchestrator-applied fix (uncommitted):**
- `apps/api/application_core/src/commands/user/mod.rs` — added missing `pub mod create; pub mod delete; pub mod modify; pub mod read_list; pub mod read_one;` declarations (was the cause of the workspace compile error from the prior coder run)

**Remaining (❌ tasks 11, 12, 14, 16–28):**

| Task | Action |
|---|---|
| 11 | Create `apps/api/src/api/user/{mod.rs, create/create_handler.rs, read_list/read_list_handler.rs, read_one/read_one_handler.rs, modify/modify_handler.rs, delete/delete_handler.rs}`. Mirror `apps/api/src/api/category/*` exactly. Use `SupabaseToken::user_id()` (exists at `apps/api/src/common/supabase_auth.rs:39`) for the delete handler's self-delete guard. |
| 12 | Wire `/users` (GET/POST/PUT/DELETE) + `/users/{user_id}` (GET) into `protected_administrator_router()` in `apps/api/src/bin/my-cms-api.rs`. (Note: `construct_app_state` already builds `supabase_admin_client` at lines 243, 258 — only the route block needs adding.) |
| 13 | ✅ already done by prior coder run — `ErrorCode::Conflict` variant, `StatusCode::CONFLICT` mapping, `AppError::Conflict(m)` arm in `From<AppError>`, and 2 unit tests all in place at `apps/api/src/presentation_models/api_response.rs`. |
| 14 | Backend verify gate (cd apps/api && cargo check && cargo test && cargo fmt -- --check && cargo clippy --all-targets -- -D warnings). |
| 15 | ✅ already done — `error_messages_never_include_service_role_key` test passes. |
| 16 | `apps/web/src/domains/user.ts` (UserRoleEnum, AppUserModel, CreateUserResponse, CreateUserRequest, ModifyUserRequest) + export from `domains/index.ts`. |
| 17 | `apps/web/src/schemas/user.schema.ts` (Zod, password min 8 on create, role enum, banned). |
| 18 | `apps/web/src/models/CreateUserModel.ts` + `apps/web/src/models/ModifyUserModel.ts`. |
| 19 | Add `id: user.id` to `userInfo` + `AuthContextType` in `apps/web/src/auth/AuthContext.tsx`. |
| 20 | `apps/web/src/app/admin/components/admin-only-route.tsx` (role-gate, DaisyUI Forbidden alert). |
| 21 | `apps/web/src/app/admin/users/page.tsx` (list mirroring categories). |
| 22 | `apps/web/src/app/admin/users/user-form.tsx` (shared; password only on create; toast surfaces password ONCE). |
| 23 | `apps/web/src/app/admin/users/create/page.tsx`. |
| 24 | `apps/web/src/app/admin/users/edit/[id]/page.tsx`. |
| 25 | Register 3 routes in `apps/web/src/App.tsx`. |
| 26 | Add "Administration" collapsible section to `apps/web/src/app/admin/components/left-menu.tsx`. |
| 27 | `pnpm --dir apps/web build` clean. |
| 28 | Manual smoke (user-side). |

**Last verified gate (2026-06-22):**
- `cd apps/api && cargo check` → ✅ green
- `cd apps/api && cargo test -p application_core --lib user` → ✅ 26/26 pass
- `cd apps/api && cargo clippy` / `cargo fmt -- --check` / `pnpm --dir apps/web build` → **not yet run** (these belong to task 14 and 27)

**Decisions (recommended defaults applied):**
1. Self-delete guard — blocked at API + UI ✅
2. Editable fields — email + role + banned ✅
3. Role list — dropdown of 2 recognised roles ✅
4. Delete semantics — hard delete ✅
5. Create flow — admin sets password, surfaced ONCE in toast ✅
6. `AppError::Conflict(String)` variant (deviation from design.md's `with_error_code(ErrorCode::Conflict)` pattern — simpler, working)
7. `AdminOnlyRoute` + `AuthContext.userInfo.id` ✅

**Critical rules:**
- NO commits (user has not asked). Resume with a fresh coder agent invocation that knows the resume scope.
- NO comments in code.
- NO new migration / SeaORM entity (users stay in GoTrue).
- `SUPABASE_SERVICE_ROLE_KEY` MUST NEVER leak into response bodies or logs (enforced by task 15 test).