# Add User Management Admin Page

## Why

The My-CMS admin has no UI to manage CMS operators. Today, the only paths to provision a CMS user are running the seeder script (`reset-supabase.sh`) or calling GoTrue's admin API by hand with `curl` + `SERVICE_ROLE_KEY`. There is no way to:

- See who currently has access to the CMS
- Onboard a new editor or administrator
- Rotate or set a password without touching secret files
- Change a user's role
- Remove a leaver

The only authoritative spec for users today is `supabase-auth`. Users live in GoTrue's `auth.users` Postgres table (managed by GoTrue), with roles declared in `app_metadata.roles`. The current `apps/api/src/api/administrator/` module hosts only `POST /administrator/database/migration` (a DBA endpoint) — there is no `user` submodule in `apps/api/src/api/`, and no `users` admin page in `apps/web/src/app/admin/`.

Operators need a self-service admin page to manage CMS users — list, invite, edit, revoke — without leaving the panel or running scripts. This is the first step toward a real "Administration" section in the sidebar, and it unlocks future admin pages (e.g. audit log, settings) to follow the same pattern.

## What Changes

- **New `user` API module** under `apps/api/src/api/user/` and `apps/api/application_core/src/commands/user/`, mirroring the `category` module's `create/` `read/` `modify/` `delete/` layout.
- **New `SupabaseAdminClient`** in `application_core/src/commands/user/` that wraps GoTrue's `POST/GET/PUT/DELETE /auth/v1/admin/users` endpoints using `SUPABASE_SERVICE_ROLE_KEY`, mirroring the `SupabaseStorage` pattern in the media module.
- **Five new REST endpoints**, all wired into `protected_administrator_router()` in `apps/api/src/bin/my-cms-api.rs` (which already requires the `my-headless-cms-administrator` role):
  - `GET /users` — list CMS users (optional filters: `role`, `email` search, pagination)
  - `GET /users/{user_id}` — fetch one user
  - `POST /users` — create a new user (email, password, initial role, `email_confirm: true`)
  - `PUT /users/{user_id}` — update email, role, and `banned_until`
  - `DELETE /users/{user_id}` — hard-delete a user from GoTrue
- **New admin page** at `/admin/users` with the standard list/create/edit flow mirroring `categories` (table with sort, filter, pagination, delete confirmation modal, FAB on the form).
- **Sidebar entry** "Users" under a new "Administration" collapsible section in `apps/web/src/app/admin/components/left-menu.tsx` (future-proof for additional admin pages).
- **No new database migration** and **no new SeaORM entity** — user records live in GoTrue. We do not duplicate user state into our schema.

## Capabilities

### New

- **`user-management`** — Full CRUD for CMS operators through the admin panel. List, create, update (email, role, banned state), delete. Restricted to the `my-headless-cms-administrator` role. Backed by GoTrue's admin API via a new `SupabaseAdminClient`. Mirrors the `category` module's Command Pattern (trait + struct per verb) and the media module's external-service client pattern.

### Modified

- **`supabase-auth`** — **No behaviour change.** The existing requirements ("Administrator account is seeded on Supabase reset", "SupabaseAuthLayer required_roles uses OR semantics", "Public sign-up is closed in local dev") remain authoritative. The new `user-management` capability consumes GoTrue as a black box and does not change JWT validation, the role check, the seeder, or the env-variable contract. The new `/users` routes live on the existing `protected_administrator_router`, which already gates on `my-headless-cms-administrator`.

## Impact

| Layer | Impact |
|-------|--------|
| **API routing** | New module `apps/api/src/api/user/` (mirrors `category/`). Five new handlers wired into `protected_administrator_router()` in `apps/api/src/bin/my-cms-api.rs`. **No** change to `protected_router` or `public_router`. |
| **Application core** | New module `apps/api/application_core/src/commands/user/` with `SupabaseAdminClient` (reqwest-based, `ServiceRoleKey`-authenticated) + `AppUserModel` DTO + `UserCreateRequest` / `UserModifyRequest` request types. Four command handlers (Create, Read List, Read One, Modify, Delete) following the trait+struct pattern. |
| **Auth middleware** | None — `SupabaseAuthLayer` already supports role gating; the new endpoints are added to the router that already restricts to `my-headless-cms-administrator`. |
| **AppState** | New optional field `supabase_admin_client: Arc<SupabaseAdminClient>` constructed in `construct_app_state()` from `SUPABASE_URL` + `SUPABASE_SERVICE_ROLE_KEY` (both already in env). |
| **Database / migrations** | None — GoTrue is the source of truth. No new table, no new SeaORM entity. |
| **GraphQL schema** | None — REST-only, consistent with the existing admin endpoints. |
| **Frontend pages** | New `apps/web/src/app/admin/users/page.tsx` (list), `users/user-form.tsx` (shared create/edit form), `users/create/page.tsx`, `users/edit/[id]/page.tsx`. New Zod schemas in `apps/web/src/schemas/user.schema.ts`. New domain types in `apps/web/src/domains/user.ts`. New request/response models in `apps/web/src/models/`. |
| **Frontend navigation** | `apps/web/src/app/admin/components/left-menu.tsx` gets a new "Administration" collapsible section containing a "Users" link. The existing "Resources" section is untouched. |
| **Environment variables** | None new — `SUPABASE_URL` and `SUPABASE_SERVICE_ROLE_KEY` are already required. |
| **Documentation** | This change does not touch `docs/superpowers/` (legacy). All artifacts live in `openspec/`. |

## Non-Goals

- **Self-service signup, password-reset UI, MFA, OAuth provider configuration** — public sign-up is disabled by `GOTRUE_DISABLE_SIGNUP=true` (see `supabase-auth`). User-facing recovery flows stay in GoTrue's default pages or are out of scope.
- **Audit log of admin actions on users** — not in v1. (Could be added later by writing to a new audit table; the command handlers would be the natural place to instrument.)
- **Bulk import / CSV upload** — operators will create users one at a time.
- **Per-user permission granularity beyond `app_metadata.roles`** — GoTrue's role model is the only permission model in v1. We do not invent a separate RBAC layer.
- **Soft-delete UI / "trash" view** — GoTrue's delete is hard. Banning (`banned_until`) is the soft-disable primitive and is exposed as a field on the edit form, not as a separate page.
- **Public-facing profile pages or user directories** — this is purely an internal CMS operator feature.
- **Email confirmation flow / SMTP** — the admin creates users with `email_confirm: true` (mirroring the existing seeder). Recovery emails are out of scope.
- **GraphQL surface for users** — the existing convention is REST for admin endpoints; we follow it.

## Open Questions

The following decisions affect the implementation. The software-architect will pin these down (or ask the user) before drafting `specs/`, `design.md`, and `tasks.md`:

1. **Self-delete guard** — Should an admin be able to delete their own account from the list? (Recommended: **no** — disable the delete button when `user.id === currentUser.id` in the UI, and return HTTP 400 from the API as a safety net.) Confirm or override.
2. **Editable fields** — Which fields should the edit form allow? Proposed: **email, role (single-select from the recognised roles), and `banned_until` toggle** (banned = set to a far-future timestamp; unbanned = clear it). Password rotation is intentionally excluded in v1 (out of scope). Confirm or override.
3. **Recognised role list** — Should the create/edit form's role dropdown list only the two roles already used in code (`my-headless-cms-administrator`, `my-headless-cms-writer`), or allow free-form input? (Recommended: **dropdown of the two recognised roles only** — free-form is the #1 source of "why doesn't my role work" tickets, and GoTrue's `app_metadata.roles` is opaque so a typo silently breaks auth.) Confirm.
4. **Delete semantics** — Hard delete (`DELETE /auth/v1/admin/users/{id}`) or ban-by-default? (Recommended: **hard delete** for the explicit "Delete" action; ban stays available as a field on the edit form so an admin can disable a user without losing the record.) Confirm.
5. **Create flow — admin sets password vs invite email** — GoTrue supports (a) admin sets the initial password, (b) admin sends a recovery/invite email and the user sets their own. Which should we use? (Recommended: **(a) — admin sets the password, the API returns it once in the create response, the form shows it in a success toast, and the operator shares it out-of-band** — matches the current seeder pattern and avoids depending on SMTP.) Confirm.