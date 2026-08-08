## Context

The source is midway through a domain extraction. `apps/api/Cargo.toml:1-2` lists `domain_media`, `domain_posts`, and `domain_user`, while `apps/api/Cargo.toml:10-13` maps the production `my-cms-api` binary to `gateway/src/main.rs`. The gateway manifest currently registers only posts and auth (`apps/api/gateway/src/main.rs:39-48`), so its route surface omits media and users. The fallback `apps/api/src/bin/legacy_bootstrap.rs:86-235` still constructs three `Router<AppState>` groups and is the only runtime registering media, user, bucket, and administrator migration routes. Its `construct_app_state` creates database, Storage, caches, GraphQL, and GoTrue dependencies (`apps/api/src/bin/legacy_bootstrap.rs:237-280`).

Canonical business handlers have already moved: `domain_media/src/lib.rs:1-9` declares media/storage/cache ownership and `domain_user/src/lib.rs:1-8` declares user/GoTrue ownership. Their legacy HTTP adapters still reside under `apps/api/src/api/media/**` and `apps/api/src/api/user/**`; duplicate post/category/tag adapters remain under the same legacy tree even though `domain_posts` owns canonical adapters and route registration. `application_core/src/commands/mod.rs:1-3` is now an empty compatibility stub. This makes the remaining `cms` source primarily a bootstrap/API compatibility shell.

The active changes `refactor-api-into-pluggable-domain-libraries`, `migrate-legacy-to-domain-posts`, and `split-media-and-user-domains-merge-tags-into-posts` already define and implement the preceding extraction stages. This change is a non-overlapping final cutover: adapters, route registration, state ownership, then removal of the compatibility runtime.

Graph evidence was refreshed at HEAD `d6a4d67f` before design. The graph reports 2,096 nodes/22,797 edges, medium change risk (0.60), and 19 test gaps. Dominant affected Rust communities are `read-handler`, `entities-post`, and `read-api`; architecture warnings show coupling between handler and API communities. Before refresh the graph incorrectly resolved deleted `application_core` media/user paths, demonstrating that stale graph results must not be used. After refresh, source remains authoritative for exact ownership. The cutover touches the `main` flow and high-criticality user/media handlers, so route inventory and deterministic contract tests are release gates.

## Goals / Non-Goals

**Goals:**

- Make `my-cms-api` the sole complete API runtime.
- Move media and user adapters beside their canonical handlers and implement `DomainService` registration for both domains.
- Preserve every public contract and authorization mount while removing duplicate legacy adapters and state.
- Keep API adapters thin and domain handlers responsible for business behavior.
- Provide staged rollout, rollback, graph review, and test evidence.

**Non-Goals:**

- No route redesign, API versioning, frontend change, schema migration, entity regeneration, cache-policy change, GoTrue/Storage behavior change, GraphQL expansion, or error-envelope redesign.
- No new shared abstraction beyond the existing `domain_interface` contract.
- No manual edits to generated SeaORM entities.
- No cleanup unrelated to the legacy API/runtime ownership boundary.

## Decisions

### 1. Extend the existing `DomainService` composition

**Decision.** Add `service.rs` and `api/**` to `domain_media` and `domain_user`. Each service owns construction/validation of domain-specific dependencies and returns bare `RouteRegistration`s for the correct `Mount`; the gateway remains responsible for auth and cross-cutting middleware.

**Rationale.** This matches `DomainPostService` (`apps/api/domain_posts/src/service.rs:35-75`) and preserves dependency direction: gateway → interface/domain crates; API adapter → trait-backed handler. It avoids retaining `AppState` or making `RouteRegistration` generic.

**Alternatives.** (A) Keep two binaries behind Traefik: rejected because it perpetuates duplicate runtime state and incomplete gateway ownership. (B) Add a generic legacy shim to bridge `Router<AppState>` into `Router<DomainContext>`: rejected because the prior change already found this incompatible with the stable dyn-safe interface and it preserves legacy coupling. (C) Move only route registration while importing `cms::api`: rejected because domains would depend backward on the compatibility crate.

### 2. Move adapters, do not copy behavior

Media adapters move to `domain_media/src/api/**`; user adapters move to `domain_user/src/api/**`. Their extractor state changes from monolithic `AppState` to domain-owned state/dependencies, but handler calls and response contracts remain equivalent. Existing post/category/tag domain adapters are retained; their `apps/api/src/api/**` duplicates are deleted rather than moved again. The root/health endpoints remain gateway-owned. The administrator migration route becomes a gateway adapter invoking the existing orchestrator.

Thin-adapter review is mandatory because `apps/api/src/api/media/create/create_handler.rs:24` contains multipart extraction and content checks; domain validation and storage decisions must remain delegated. Production `unwrap`/`expect` paths encountered during the move are converted to `AppError` only where needed to preserve failure behavior safely; this is not permission to redesign responses.

### 3. Domain dependency ownership

`DomainMediaService` owns `MediaConfig`, media cache, and bucket-visibility cache initialized once. `DomainUserService` owns the `SupabaseAdminClient`, initialized once with server-only service-role credentials. Both receive the shared database through `DomainContext`. Secrets are never included in `Debug`, errors, or tracing fields. Required environment variables are validated during startup before binding.

The existing `DomainContext` is not expanded with media/user concrete types. Domain-specific dependencies remain fields of the concrete service and are injected into routers via domain-local Axum state. This avoids polluting the stable interface with concrete domains.

### 4. Route and authorization contract

Route inventory is derived from `legacy_bootstrap.rs:86-235`. Public media reads remain public; media writes remain writer-or-administrator; bucket and user operations remain administrator-only. Post GraphQL paths remain `/posts/graphql/{immutable,mutable}`. The administrator migration trigger remains administrator-only and delegates to the same gateway service manifest used at startup. Route methods, wildcard semantics, body limit, CORS, cookies, and auth audiences/roles require explicit parity tests.

### 5. Sequenced deletion and compatibility window

Implementation proceeds additively: add domain adapters/services, register them, prove parity, then remove duplicates and legacy runtime. No legacy file is deleted before route inventory tests pass. During rollout, retain the previously built legacy image as rollback artifact; source deletion occurs only after the gateway image passes staging smoke/contract tests. Because data and migration identities do not change, rollback is traffic/image rollback only.

### 6. Testing and observability

Add domain-local router tests for route/mount inventory and representative extraction/error behavior. Preserve handler tests, PostgreSQL testcontainers where database behavior is involved, and wiremock tests for Storage/GoTrue. Add gateway tests asserting the complete manifest and route matrix, 401/403 behavior, private media obscuring, and migration authorization. Trace service startup and route registration without secrets; retain request telemetry at the gateway boundary.

## Migration Plan

1. Baseline the legacy and gateway route matrices and refresh graph evidence.
2. Add domain-local states, adapters, services, and tests without removing legacy routes.
3. Register media/user services in the gateway; add the administrator migration adapter.
4. Run focused domain, gateway, auth, Storage, GoTrue, and route parity tests.
5. Deploy the gateway image to staging alongside the existing legacy image; compare representative success/failure responses and telemetry.
6. Shift traffic to the gateway. Roll back by restoring traffic to the legacy image if parity, error rate, latency, or auth behavior regresses.
7. After acceptance, delete `legacy_bootstrap`, duplicate `apps/api/src/api/**`, legacy `AppState`, common/presentation shims, and unused Cargo dependencies. Retain compatibility crates only where current importer searches prove they remain needed (for example test helpers/entity re-exports); remove them in a separately evidenced task if empty.
8. Rebuild the graph and run full verification. No database down migration or data rollback is required.

## Risks / Trade-offs

- **Route omission or wrong mount** → Check an explicit method/path/mount matrix against `legacy_bootstrap.rs:86-235` before deletion.
- **State-type migration changes behavior** → Keep domain-local dependencies identical and add adapter/handler contract tests before cutover.
- **Auth or middleware ordering drift** → Gateway-level 401/403/body-limit/CORS/cookie tests and staged response comparison.
- **Private media exposure** → Preserve bucket access policy and add anonymous/private/admin tests as a blocking gate.
- **Secret leakage** → Keep redacted client debug behavior and assert sensitive values are absent from errors/traces.
- **Graph test-gap noise** → Use refreshed graph only, validate graph findings against source, and require direct router tests for high-risk adapters.
- **Large deletion blast radius** → Add first, cut traffic second, delete last; retain the previous deployable image for rollback.
- **Scope collision with active changes** → Treat those changes as prerequisites and do not rewrite their artifacts; archive/sync coordination remains with the PO.

## Open Questions

- Must the administrator migration HTTP endpoint remain enabled long-term, or should it be deprecated after this behavior-preserving cutover? This change preserves it by default.
- Should `application_core` and `migration` compatibility crates be removed in this change if importer search reaches zero, or archived in a subsequent minimal cleanup? Default: remove only provably unused source/modules here and defer crate deletion if test helpers still import them.
