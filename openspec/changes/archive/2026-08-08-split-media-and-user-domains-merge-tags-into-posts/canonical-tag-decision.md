## Canonical tag decision (recorded at start of implementation)

**Decision:** `apps/api/domain_posts/src/handlers/tag_helper/` is the canonical
tag implementation. The legacy `apps/api/application_core/src/commands/tag/` is
duplicate code that must be deleted (not relocated) because the post domain
already owns tag operations.

**Evidence:**
1. `domain_posts::handlers::tag_helper::mod.rs:6-10` declares the module is
   the canonical post-domain owner and that future `domain_tags` extraction
   will consume it as the source of truth.
2. Recent commit `d133a9c refactor: migrate legacy post-related code to
   domain_posts crate` established the post-domain crate as the migration
   destination; tag operations are part of the post migration.
3. `apps/api/src/api/tag/delete/delete_handler.rs` (the only HTTP `tag`
   adapter in the legacy `apps/api` tree) calls
   `PostDeleteHandler::handle_delete_posts`, not any `commands::tag::*` symbol
   — proving there is no `commands::tag` consumer in the API layer.
4. `commands::tag` has no consumer in production code: a workspace-wide
   grep for `commands::tag` returns only:
   - internal tests inside `commands::tag/{read,create}/...`
   - one test in
     `domain_posts::handlers::tag_helper::read::read_handler.rs:98`
     that imports `commands::tag::delete::delete_handler`
   - doc comments in `domain_posts::handlers::test.rs:4` and
     `domain_posts::handlers::tag_helper::mod.rs:6`
   - `application_core/README.md:6` test command reference.
5. The `tag_helper` module is intentionally `pub(crate)` and is the shared
   internal copy used by `domain_posts::handlers::post::create` and
   `domain_posts::handlers::post::modify`.
6. Both `commands::tag` and `tag_helper` have `create` + `read` for
   `TagCreateHandler`/`TagCreateHandlerTrait` and `TagReadHandler`/
   `TagReadHandlerTrait` (logic is duplicated). The legacy module also has
   `TagDeleteHandler`, which the canonical module lacks. We will copy the
   `delete` logic into `tag_helper::delete` so the canonical module becomes
   fully self-contained, matching the `tag_helper::mod.rs:6` goal.

**Rationale:** The spec wording "tag commands under the post domain" translates
to the actual code-level post domain, which is `domain_posts::handlers::post`.
The architect explicitly approved this fallback in
`openspec/changes/.../design.md` § Decisions 2.
