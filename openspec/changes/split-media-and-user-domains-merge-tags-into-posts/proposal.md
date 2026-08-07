## Why

`application_core/src/commands` currently exposes legacy domain boundaries that do not match the CMS domain model and, in the current branch, overlaps with the ongoing domain extraction work. A source-aligned internal reorganization will make post/tag, media, and user ownership explicit without changing runtime contracts.

## What Changes

- Move media command ownership into the dedicated `commands/media/` domain module, preserving storage clients, bucket commands, media commands, cache types, traits, errors, tracing, and tests.
- Move user command ownership into the dedicated `commands/user/` domain module, preserving Supabase admin integration, DTOs, handlers, traits, errors, tracing, and tests.
- Absorb legacy tag commands into the post domain at `commands/post/tags/` (or the equivalent existing post tag-helper module selected by the implementation after source reconciliation); remove the separate `commands/tag` module boundary.
- Update command module declarations, public re-exports, application wiring, API imports, bootstrap imports, and in-module paths only as required by the relocation.
- Preserve all HTTP routes, GraphQL schema and operations, request/response shapes, authentication and authorization behavior, persistence behavior, and external Supabase contracts.
- Do not alter entities, migrations, database data, or generated SeaORM files.

## Capabilities

### New Capabilities
- `media-domain`: Media and bucket command ownership is exposed under the media domain boundary.
- `user-domain`: User-management command ownership is exposed under the user domain boundary.

### Modified Capabilities
- `post-domain`: Post command ownership absorbs tag commands while preserving tag behavior and post/tag integration.

### Removed Capabilities
- `tags-domain`: The standalone command-module boundary is removed; tag HTTP behavior is not removed.

## Impact

Affected source includes `apps/api/application_core/src/commands/{mod.rs,tag/**,media/**,user/**}`, API adapters under `apps/api/src/api/{tag,media,user}/`, `apps/api/src/lib.rs`, `apps/api/src/bin/legacy_bootstrap.rs`, and any current post/domain crate re-exports or imports discovered by exhaustive search. No public endpoint, GraphQL, schema, entity, migration, or database contract changes are intended.
