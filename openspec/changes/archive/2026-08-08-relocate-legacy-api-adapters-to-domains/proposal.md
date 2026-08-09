## Why

The production `my-cms-api` binary now composes domain services, but media and user HTTP adapters still live under the legacy `cms::api` tree and are reachable only through `legacy_bootstrap`; post/category/tag adapters remain duplicated there after their canonical copies moved to `domain_posts`. This split leaves two runtime compositions, duplicate API ownership, and a deployment compatibility risk, so the remaining adapters must move to their owning domains before the compatibility bootstrap can be retired.

## What Changes

- Add `DomainMediaService` and `DomainUserService` implementations that own their HTTP adapters and register the existing public, protected, and administrator routes through `domain_interface::DomainService`.
- Register media and user services in the gateway manifest and construct their Supabase/cache dependencies once at startup without changing route paths, methods, auth mounts, request/response contracts, storage behavior, or GoTrue behavior.
- Keep post/category/tag routes owned by `domain_posts`; delete their duplicate adapters from `apps/api/src/api` after parity tests prove the gateway routes are canonical.
- Move the administrator migration trigger into the gateway composition boundary and preserve its administrator-only contract while delegating migration execution to the existing orchestrator.
- Retire `legacy_bootstrap`, the legacy `AppState`, and obsolete `cms::api`, `common`, and presentation re-export modules only after all routes are mounted and tested in `my-cms-api`.
- Preserve migration identities and generated SeaORM entities; no database migration or manual generated-entity edit is included.
- Do not intentionally change any externally observable API behavior.

## Capabilities

### New Capabilities

- `domain-api-cutover`: Defines ownership, route parity, compatibility, and safe retirement requirements for moving the remaining legacy HTTP adapters into domain services and making the gateway the sole API runtime.

### Modified Capabilities

None. Existing user-management, storage, bucket, image, auth, GraphQL, and vector-search behavior remains normative and unchanged.

## Impact

Affected code is concentrated in `apps/api/src/{api,bin/legacy_bootstrap.rs,lib.rs,common,presentation_models}`, `apps/api/domain_{media,user}/`, `apps/api/domain_posts/src/api/`, `apps/api/gateway/src/main.rs`, `apps/api/domain_interface`, Cargo manifests, and route/contract tests. Supabase Storage, GoTrue, PostgreSQL, pgvector, GraphQL schemas, migration identities, HTTP paths, auth roles, response envelopes, and frontend callers remain compatible. This is the bounded follow-up to the active `refactor-api-into-pluggable-domain-libraries`, `migrate-legacy-to-domain-posts`, and `split-media-and-user-domains-merge-tags-into-posts` changes; it does not overwrite their completed artifacts.
