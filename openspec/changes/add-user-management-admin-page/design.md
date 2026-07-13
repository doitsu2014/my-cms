# Add User Management Admin Page — Design

## Context

My-CMS today has no UI to manage CMS operators. The only paths to provision a CMS user are running `reset-supabase.sh` (the seeder) or hand-crafting a `curl` against GoTrue's admin API with `SUPABASE_SERVICE_ROLE_KEY`. Operators cannot see who has access, onboard new editors, change roles, or remove leavers from inside the panel.

The authoritative spec for users today is `supabase-auth`: users live in GoTrue's `auth.users` table and roles live in `app_metadata.roles`. The existing `apps/api/src/api/administrator/` module hosts only `POST /administrator/database/migration`, gated by `protected_administrator_router()` (which requires `my-headless-cms-administrator`). There is no `user` submodule on either the API or the frontend.

This change adds a new `user-management` capability that:

- Exposes 5 REST endpoints under `protected_administrator_router()` — list, fetch one, create, update, delete — backed by GoTrue's `/auth/v1/admin/users` endpoints.
- Adds a new `apps/api/application_core/src/commands/user/` module that mirrors `commands/category/` (Command Pattern: trait + struct + impl per verb) and `commands/media/` (`SupabaseStorage` style external-HTTP client, here called `SupabaseAdminClient`).
- Adds a new React admin page `/admin/users` with list/create/edit flow mirroring `categories`.
- Adds an "Administration" collapsible section in the sidebar (future-proof for additional admin pages).
- Does **not** introduce a new database table, a new migration, or a new SeaORM entity.

The change deliberately reuses existing patterns rather than inventing new abstractions: layered architecture (API → Application Core → External), Command Pattern, `AppError` + `ApiResponseError` for error mapping, and the existing `SupabaseAuthLayer` for role gating.

## Goals

- Provide a complete CRUD experience for CMS operators inside the admin panel.
- Restrict every admin-user endpoint to callers holding `my-headless-cms-administrator` in `app_metadata.roles`.
- Make the `SUPABASE_SERVICE_ROLE_KEY` path observable, testable, and safe (no leaks in responses or logs).
- Emit a structured audit event (`actor_user_id`, `target_user_id`, `action`) on every successful mutation.
- Mirror existing patterns (category CRUD, media SupabaseStorage) so the change fits the codebase with minimal new abstractions.

## Non-Goals

(Reaffirming and tightening the proposal's non-goals; nothing new is added beyond what exploration surfaced.)

- **Self-service signup, password reset UI, MFA, OAuth providers** — sign-up is disabled by `GOTRUE_DISABLE_SIGNUP=true` (see `supabase-auth`). Recovery flows remain GoTrue's default pages.
- **Audit log persistence (database table for audit events)** — v1 only emits tracing events. Persisting them is future work.
- **Bulk import / CSV upload** — operators create users one at a time.
- **Per-user permissions beyond `app_metadata.roles`** — GoTrue's role model is the only RBAC layer.
- **Soft-delete / trash view** — GoTrue's delete is hard. Banning is the soft-disable primitive and lives on the edit form.
- **Public-facing user directory or profile pages** — purely internal CMS operator management.
- **Email confirmation flow / SMTP** — admin creates users with `email_confirm: true` (mirrors the existing seeder).
- **GraphQL surface for users** — REST-only, consistent with other admin endpoints.
- **NEW non-goal surfaced during exploration: frontend role-aware route guard.** — `/admin/users` is reachable by any authenticated user because `ProtectedRoute` only checks `authenticated`. We will add a lightweight role-gate wrapper component (`AdminOnlyRoute`) that rejects non-admins with a `Forbidden` screen, but full admin-only routing for every future admin page is out of scope for this change.

## Decisions

### 1. Self-delete guard

**Decision:** An administrator cannot delete their own account via the API or the UI. The UI disables the delete button when `row.userId === currentUser.id`; the API additionally returns HTTP 400 with a `Logical` error code as a safety net.

**Reason:** The risk of an admin accidentally locking themselves out of the panel outweighs the rare case of a sole admin needing to delete themselves. Self-deletion should be done by another admin or by re-running the seeder. Layered defence (UI + API) matches the conservative pattern used elsewhere in the codebase (e.g. the optimistic-concurrency check on category modify is duplicated in the API layer).

### 2. Editable fields

**Decision:** The edit form allows `email`, `role` (single-select from the recognised list), and `banned` (boolean toggle). Password rotation is intentionally excluded in v1.

**Reason:** These three fields are the ones the operator needs to manage a leaver, demote/promote a teammate, and disable a compromised account. Password rotation needs an invite/reset flow that we do not have in v1 (no SMTP, no recovery page). The proposal already lists this as out of scope.

### 3. Recognised role list

**Decision:** The role dropdown lists exactly two options: `my-headless-cms-administrator` and `my-headless-cms-writer`. Free-form role input is rejected by both the frontend (the `<select>` element) and the API (`Validation` error if a non-recognised value arrives).

**Reason:** GoTrue's `app_metadata.roles` is opaque to the JWT validator — a typo silently breaks authentication and produces confusing "why doesn't my role work" tickets. A closed enum makes the contract explicit at both ends. Adding more recognised roles is a one-line change in `UserRole` (Rust) and `UserRoleEnum` (TS).

### 4. Delete semantics

**Decision:** The explicit "Delete" action is a hard delete via GoTrue's `DELETE /auth/v1/admin/users/{user_id}`. The ban toggle on the edit form is the soft-disable primitive and uses GoTrue's `ban_duration` mechanism.

**Reason:** Hard delete matches the operator's mental model for "remove this leaver". Ban covers the "keep the audit trail but block sign-in" case without polluting the list view. Both primitives are first-class in GoTrue.

### 5. Create flow

**Decision:** The admin sets the initial password in the create form. The command handler forwards it to GoTrue (`POST /auth/v1/admin/users` with `email_confirm: true`) and returns the same plaintext password in a one-time `temporaryPassword` field on the create response. The frontend surfaces this in a success toast (`toast.success("User created. Share this password securely: <password>", { duration: 30000 })`). Subsequent list/read calls never include the password.

**Reason:** Avoids depending on SMTP, matches the existing seeder pattern (`reset-supabase.sh` already does this), and is testable end-to-end without external services. The one-time surface and the "never returned on read" property are codified in the spec as testable requirements.

### 6. Error code variant (new)

**Decision:** Add `ErrorCode::Conflict = "409"` to `apps/api/src/presentation_models/api_response.rs`, mapped to HTTP 409. Use it for the duplicate-email-on-create and duplicate-email-on-update cases.

**Reason:** HTTP 409 is the correct semantic for a uniqueness violation; the existing `Logical` → HTTP 400 mapping is incorrect. Adding the variant is a 2-line, additive change that does not alter any existing endpoint's behaviour. The frontend does not pattern-match on `errorCode` (it uses `response.status`), so no frontend code breaks.

### 7. AuthContext exposes the caller's user id

**Decision:** Extend `AuthContext.userInfo` with `id: user.id` (the Supabase `sub` claim) so the user list page can compare row vs. current user for the self-delete guard.

**Reason:** The existing `userInfo` shape already exposes `email`, `name`, `username`, `picture`; adding `id` is consistent and zero-cost. The Supabase `User` type already carries `.id`.

## Architecture

### Module layout

```
apps/api/application_core/src/commands/user/
├── mod.rs                          # pub mod {create, read_list, read_one, modify, delete}; pub use supabase_admin_client::*
├── supabase_admin_client.rs        # Reqwest client + GoTrue DTOs (list/get/create/update/delete)
├── dto.rs                          # AppUserModel, CreateUserResponse, recognised roles helper
├── create/
│   ├── mod.rs
│   ├── create_request.rs           # CreateUserRequest { email, password, role }
│   └── create_handler.rs           # CreateUserHandlerTrait + struct + impl + #[instrument] + unit tests
├── read_list/
│   ├── mod.rs
│   ├── read_list_handler.rs        # ReadListUserHandlerTrait + struct + impl (filters + pagination)
├── read_one/
│   ├── mod.rs
│   ├── read_one_handler.rs         # ReadOneUserHandlerTrait + struct + impl
├── modify/
│   ├── mod.rs
│   ├── modify_request.rs           # ModifyUserRequest { id, email?, role?, banned? }
│   └── modify_handler.rs           # ModifyUserHandlerTrait + struct + impl
└── delete/
    ├── mod.rs
    └── delete_handler.rs           # DeleteUserHandlerTrait + struct + impl (self-delete guard)

apps/api/src/api/user/
├── mod.rs                          # pub mod {create, read_list, read_one, modify, delete}
├── create/create_handler.rs        # thin handler: extract → command → response
├── read_list/read_list_handler.rs  # thin handler
├── read_one/read_one_handler.rs    # thin handler
├── modify/modify_handler.rs        # thin handler
└── delete/delete_handler.rs        # thin handler
```

### `SupabaseAdminClient`

A new `SupabaseAdminClient` struct in `application_core/src/commands/user/supabase_admin_client.rs`, modelled exactly on `SupabaseStorage`:

- Holds `supabase_url: String`, `service_role_key: String`, `client: reqwest::Client`.
- `Debug` impl redacts `service_role_key` as `"<redacted>"` (mirrors `SupabaseStorage`).
- `new(supabase_url, service_role_key) -> Self` builds a `Client` with a 30-second timeout.
- Base URL: `{supabase_url}/auth/v1/admin`.
- Every request sets `Authorization: Bearer <service_role_key>` and `apikey: <service_role_key>`.
- Methods: `list_users(page, per_page)`, `get_user(id)`, `create_user(req)`, `update_user(id, req)`, `delete_user(id)`.
- Each method maps non-2xx responses to `AppError` variants (see Error mapping table below) and **never** includes `service_role_key` in log messages.

### `AppUserModel` (DTO)

```rust
#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct AppUserModel {
    pub id: Uuid,
    pub email: String,                 // always lowercase
    pub role: Option<String>,          // first element of app_metadata.roles if recognised, else None
    pub banned: bool,                  // banned_until set and > now
    pub created_at: DateTime<Utc>,
    pub updated_at: DateTime<Utc>,
    pub last_sign_in_at: Option<DateTime<Utc>>,
}

pub struct CreateUserResponse {
    pub user: AppUserModel,
    pub temporary_password: String,    // surfaced once, never returned by GETs
}
```

A `RECOGNISED_ROLES: &[&str] = &["my-headless-cms-administrator", "my-headless-cms-writer"]` constant lives next to `AppUserModel` and is the single source of truth on the backend.

### AppState

Add one field to `AppState` in `apps/api/src/lib.rs`:

```rust
pub struct AppState {
    pub conn: Arc<DatabaseConnection>,
    pub media_config: Arc<MediaConfig>,
    pub media_cache: Arc<Cache<MediaCacheKey, CachedMedia>>,
    pub graphql_immutable_schema: Arc<Schema>,
    pub graphql_mutable_schema: Arc<Schema>,
    pub supabase_admin_client: Arc<SupabaseAdminClient>,   // NEW
}
```

`construct_app_state()` in `apps/api/src/bin/my-cms-api.rs` builds the client from `SUPABASE_URL` + `SUPABASE_SERVICE_ROLE_KEY` and **fails fast at startup** if `SUPABASE_SERVICE_ROLE_KEY` is missing (`.expect("SUPABASE_SERVICE_ROLE_KEY must be set for /users endpoints")`). This makes the dependency explicit and prevents silent degradation.

### Router wiring

`protected_administrator_router()` in `apps/api/src/bin/my-cms-api.rs` gains a single new route block alongside the existing `/administrator/database/migration`:

```rust
.route(
    "/users",
    get(api::user::read_list::read_list_handler::api_list_users)
        .post(api::user::create::create_handler::api_create_user)
        .put(api::user::modify::modify_handler::api_modify_user)
        .delete(api::user::delete::delete_handler::api_delete_user),
)
.route(
    "/users/{user_id}",
    get(api::user::read_one::read_one_handler::api_get_user),
)
```

The existing `SupabaseAuthLayer` (configured with `required_roles = ["my-headless-cms-administrator"]`) is unchanged.

## Data flow

For every endpoint, the flow is identical and matches the layered architecture:

```
HTTP request
    │
    ▼
Axum router (protected_administrator_router)
    │
    ▼
SupabaseAuthLayer  ──── 401 (no token) / 403 (wrong role) → ApiResponseError
    │
    ▼
API handler (apps/api/src/api/user/.../handler.rs)
    │   • Extract Path / Query / Json / Extension<SupabaseToken>
    │   • Build command handler struct from AppState fields
    │   • Call trait method
    │   • Convert Result<T, AppError> → ApiResponseWith<T> | ApiResponseError
    ▼
Command handler (apps/api/application_core/src/commands/user/.../handler.rs)
    │   • #[instrument] + tracing::info! for audit
    │   • Validate inputs (recognised role, non-empty fields, password length)
    │   • Translate DTOs → SupabaseAdminClient payloads
    │   • Call SupabaseAdminClient method
    │   • Translate response → AppUserModel / CreateUserResponse
    ▼
SupabaseAdminClient (reqwest)
    │   • Bearer auth + apikey header
    ▼
GoTrue /auth/v1/admin/users (HTTP)
```

### Per-endpoint specifics

| Endpoint | Method on `SupabaseAdminClient` | Notable command-handler logic |
|---|---|---|
| `GET /users` | `list_users(page, per_page)` (forwarded as query params) | Parse `page`, `perPage`, `role`, `email` query params; reject `page < 1` or `per_page < 1 || per_page > 200` |
| `GET /users/{user_id}` | `get_user(id)` | 404 → `AppError::NotFound` |
| `POST /users` | `create_user(CreateUserRequest)` | Lowercase email; validate password length ≥ 8; validate role ∈ `RECOGNISED_ROLES`; pass `email_confirm: true`; return `CreateUserResponse { user, temporary_password }` |
| `PUT /users/{user_id}` | `update_user(id, ModifyUserRequest)` | Validate role ∈ `RECOGNISED_ROLES` if provided; translate `banned: true` → `ban_duration: "876000h"` (≈100y), `banned: false` → no `ban_duration`; preserve other fields by omitting them from the request |
| `DELETE /users/{user_id}` | `delete_user(id)` | Compare `{user_id}` to `token.user_id()`; if equal, return `AppError::Logical("Cannot delete your own account")` |

## Error mapping table

| GoTrue response | Command-handler `AppError` | `ApiResponseError::error_code` | HTTP status |
|---|---|---|---|
| 200/201 (success) | — | — | 200/201/204 |
| 400 malformed request | `AppError::Validation(field, msg)` | `ValidationError` | 400 |
| 401 unauthorized (shouldn't happen with `SERVICE_ROLE_KEY`, but defensive) | `AppError::Logical(msg)` | `Logical` | 400 |
| 404 not found | `AppError::NotFound` | `NotFound` | 404 |
| 409 duplicate / conflict (email already in use) | `AppError::Logical(msg)` overridden with `with_error_code(ErrorCode::Conflict)` in handler | `Conflict` (NEW) | 409 |
| 422 validation | `AppError::Validation(field, msg)` | `ValidationError` | 400 |
| 5xx upstream | `AppError::StorageError(msg)` (sanitised — no SERVICE_ROLE_KEY) | `ConnectionError` | 500 |
| Network failure (timeout, connection refused) | `AppError::StorageError(msg)` | `ConnectionError` | 500 |
| Self-delete attempt | `AppError::Logical("Cannot delete your own account")` | `Logical` | 400 |

**Sanitisation rule:** every error message is constructed as a static template + a non-secret field (e.g. `format!("GoTrue list users failed ({}): <body>", status)`). No format string interpolates `self.service_role_key`. A unit test asserts that the rendered message does not contain the configured key.

## Frontend design

### Routes

Three new routes, all wrapped in `<ProtectedRoute><AdminLayout>`. App.tsx gains:

```tsx
<Route path="/admin/users" element={<ProtectedRoute><AdminOnlyRoute><AdminUsersListPage /></AdminOnlyRoute></AdminLayout></ProtectedRoute>} />
<Route path="/admin/users/create" element={<ProtectedRoute><AdminOnlyRoute><AdminCreateUserPage /></AdminOnlyRoute></AdminLayout></ProtectedRoute>} />
<Route path="/admin/users/edit/:id" element={<ProtectedRoute><AdminOnlyRoute><AdminEditUserPage /></AdminOnlyRoute></AdminLayout></ProtectedRoute>} />
```

`AdminOnlyRoute` is a new tiny wrapper that reads `app_metadata.roles` from `useAuth().user`, redirects to `/admin` (or renders a "Forbidden" card) if the role is missing. We add it because `ProtectedRoute` only checks `authenticated`.

### Files

```
apps/web/src/domains/user.ts                              # AppUserModel, UserRoleEnum, CreateUserModel, ModifyUserModel, CreateUserResponse
apps/web/src/schemas/user.schema.ts                       # Zod userFormSchema
apps/web/src/models/CreateUserModel.ts                    # CreateUserModel interface
apps/web/src/models/ModifyUserModel.ts                    # ModifyUserModel interface
apps/web/src/app/admin/users/page.tsx                     # list page (table + filter + pagination + delete modal)
apps/web/src/app/admin/users/user-form.tsx                # shared create/edit form
apps/web/src/app/admin/users/create/page.tsx              # wraps UserForm with id=undefined
apps/web/src/app/admin/users/edit/[id]/page.tsx           # wraps UserForm with id=route param
apps/web/src/app/admin/components/admin-only-route.tsx   # NEW: role-gate wrapper
apps/web/src/app/admin/components/left-menu.tsx           # MODIFIED: add "Administration" <details> with "Users" link
apps/web/src/auth/AuthContext.tsx                         # MODIFIED: extend userInfo with id
```

### Page structure (mirror categories)

`/admin/users` list page (`users/page.tsx`):
- Header with "Users" title and `+ New User` link to `/admin/users/create`.
- Collapsible filter row (mirrors `categories/page.tsx`): text search on email, `<select>` for role.
- Table with columns: Email, Role (badge), Status (Banned badge if `banned: true`), Created, Actions (Edit / Delete).
- Edit button → `/admin/users/edit/{id}`.
- Delete button opens a DaisyUI `<dialog>` modal identical in style to the categories delete modal. The button is **disabled** (with a tooltip "You cannot delete your own account") when `user.id === currentUser.id`.
- Client-side pagination mirroring categories (10 per page, prev/next/numbered).
- On successful delete: `toast.success("User deleted successfully")` and remove the row.
- On error: `toast.error(errorData.errors[0] ?? "Failed to delete user")`.

`/admin/users/create` and `/admin/users/edit/[id]` share `user-form.tsx`:
- **Create mode** shows: Email, Password, Role (dropdown), Banned (hidden, defaults to `false`).
- **Edit mode** shows: Email, Role (dropdown), Banned (toggle). Password field is NOT shown — out of scope in v1.
- On submit (create): POST `/users` → on success, show a long-duration toast `toast.success(\`User created. Share this password securely: \${password}\`, { duration: 30000 })` and navigate to `/admin/users`. The toast text uses the password the admin typed (not anything from the response, since they are the same value) so the password is rendered exactly once in the UI as required by the spec.
- On submit (edit): PUT `/users/{id}` → on success, `toast.success("User updated")` and navigate to `/admin/users`.
- FAB pattern mirrors `category-form.tsx` (expandable cancel + save buttons).

### Sidebar update

`apps/web/src/app/admin/components/left-menu.tsx` gains a second `<details>` block under the existing "Resources" block, named "Administration", containing a single `<MenuItem displayName="Users" slug="/admin/users" />`. The existing "Resources" block is unchanged.

## Testing strategy

### Unit tests (Rust, fast, in-process)

- **`SupabaseAdminClient`**: WireMock-based tests, mirroring the `SupabaseStorage` test suite (already in `commands/media/supabase_storage.rs`).
  - `list_users_issues_bearer_and_apikey_headers`
  - `get_user_returns_404_on_not_found`
  - `create_user_posts_email_confirm_and_app_metadata`
  - `update_user_translates_banned_boolean_to_ban_duration`
  - `delete_user_issues_delete_to_admin_path`
  - `error_messages_never_include_service_role_key` (sanitisation assertion — synthetic secret used in the test)
- **Command handlers** (`create_handler.rs`, `modify_handler.rs`, `delete_handler.rs`, `read_*_handler.rs`): unit tests with a mocked `SupabaseAdminClient` (mockall or hand-rolled trait object). The trait abstraction enables this even though the handler holds an `Arc<SupabaseAdminClient>` directly — we extract a thin `SupabaseAdminClientTrait` (mirroring `CategoryCreateHandlerTrait`) so the handler can be tested against a mock. Concretely:
  - `CreateUserHandler` with mocked client: returns `CreateUserResponse` on success; returns `AppError::Validation` on short password; returns `AppError::Logical` with `Conflict` code on duplicate email.
  - `ModifyUserHandler` with mocked client: banned → `ban_duration: "876000h"` translation; unrecognised role → `Validation`; unknown id → `NotFound`.
  - `DeleteUserHandler`: self-delete guard returns `AppError::Logical`; success returns 204-via-`AppError::Ok`-equivalent.
  - `ReadListUserHandler`: pagination param validation; role + email filter pass-through.
  - `ReadOneUserHandler`: 404 path.
- **Sanitisation tests** (regression suite): every command handler test that asserts on an error message also asserts `!msg.contains(&service_role_key)`.

### Frontend tests

- **`user-form.tsx`**: at minimum, a Vitest test that asserts `password` is rendered on the create route and NOT rendered on the edit route. (Pattern mirrors `login/page.test.tsx`.)
- **List page**: smoke test that the self-row's delete button is disabled (uses a stubbed `useAuth`).
- **Build verification**: `pnpm --dir apps/web build` must pass before any task group is signed off.

### Out of scope (integration tests against GoTrue)

End-to-end tests against a live GoTrue container are not in scope for this change. The deployment-level smoke tests under `deployments/docker-swarm/` already exercise `POST /auth/v1/admin/users` via the seeder and the Kong gateway; they will validate the full request path when the new admin page is run against the local stack during manual smoke. If integration coverage becomes desirable, a follow-up change can wire `supabase-testcontainers-modules` (already a dev-dep) into a `#[test]` harness under `application_core/src/commands/user/`.

### Verification gate

Before any task group is marked complete:

```bash
cargo check
cargo test
cargo fmt -- --check
cargo clippy
pnpm --dir apps/web build
```

## Risks

| Risk | Likelihood | Impact | Mitigation |
|---|---|---|---|
| `SUPABASE_SERVICE_ROLE_KEY` accidentally leaks into a response body, log line, or error message | Low | Critical (full impersonation of any user) | The `Debug` impl redacts the key. Every error path uses a sanitised template. A regression test (`error_messages_never_include_service_role_key`) runs in CI. |
| Admin locks themselves out (self-demotes to writer and signs out, or sole admin deletes their own account) | Medium | High | Self-delete is blocked at API + UI. Self-demotion via edit is allowed (the operator can ask another admin to fix it). The seeder still exists as the recovery path. |
| GoTrue schema drift (field rename or removal in a future Supabase release) | Low | Medium | `SupabaseAdminClient` is the only module that touches GoTrue's wire format; if drift occurs, the blast radius is one file plus its tests. A version assertion on `GoTrue-Version` header could be added later. |
| Race condition: two admins edit the same user concurrently → last write wins, no optimistic locking | Medium | Low | GoTrue does not expose row versioning. v1 accepts last-write-wins. The audit event includes `actor_user_id` so the change is traceable. |
| Hard-deleting a user orphans content (e.g. posts whose `created_by` email matches) | Medium | Low | The proposal makes hard delete explicit. Orphaned emails remain on the post row as a historical string. A future "merge / reassign content on delete" feature is out of scope. |
| Adding `ErrorCode::Conflict` alters the wire contract for all other endpoints | Low | None | The variant is additive; no existing endpoint returns it. The frontend does not pattern-match on `errorCode`. |
| Frontend-only role guard (`AdminOnlyRoute`) is bypassed if a writer types `/admin/users` in the URL and the backend already 403s | Low | None | Belt-and-braces: the backend is the source of truth. The guard exists only to give a friendlier error UX. |