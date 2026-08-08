## 1. Baseline contracts and graph evidence

- [x] 1.1 [All layers; prerequisite: active extraction changes complete] Refresh the code-review graph at implementation HEAD, inventory callers/callees/importers/flows/tests for every legacy adapter and both runtimes, and record the complete legacy method/path/mount matrix from `apps/api/src/bin/legacy_bootstrap.rs`; verify with graph `get_minimal_context`, targeted `query_graph`, `get_impact_radius`, `get_affected_flows`, and source search.
- [ ] 1.2 [Gateway/tests; prerequisite: 1.1] Add failing route-inventory tests that require `my-cms-api` composition to expose all public, protected, and administrator routes currently listed in `legacy_bootstrap`; verify with the focused gateway test target.
- [ ] 1.3 [Compatibility/tests; prerequisite: 1.1] Capture representative contract tests for status/envelope, 401/403, private-media obscuring, multipart/body limits, Storage errors, GoTrue errors, and migration authorization before moving adapters; verify tests pass against the baseline or document existing failures before edits.

## 2. Media API ownership

- [ ] 2.1 [domain_media API; prerequisite: 1.3] Move public media read/image adapters into `apps/api/domain_media/src/api/media/read/`, first adding/retaining tests for path extraction, resize parameters, cache keys, public/private bucket behavior, and mapped failures; delegate all policy and storage work to existing handlers.
- [ ] 2.2 [domain_media API; prerequisite: 1.3] Move authenticated media list/create/delete/metadata adapters into `domain_media/src/api/media/`, with test-first coverage for multipart validation, supported content type, single/batch deletion, response envelope, and body-size failure; remove production `unwrap` only through equivalent `AppError` mapping.
- [ ] 2.3 [domain_media API; prerequisite: 1.3] Move administrator bucket create/list/get/update/delete/empty adapters into `domain_media/src/api/bucket/`, with test-first coverage for name validation, reserved/private behavior, cache invalidation, not-found obscuring, and Storage error mapping.
- [ ] 2.4 [domain_media service; prerequisite: 2.1-2.3] Add domain-local router state and `DomainMediaService` implementing `DomainService`; initialize `MediaConfig`, media cache, and visibility cache once, return bare route registrations in the exact legacy mounts, validate required env without leaking secrets, and verify domain router/mount tests plus existing wiremock tests.

## 3. User API ownership

- [ ] 3.1 [domain_user API; prerequisite: 1.3] Move user create/read-list/read-one/modify/delete/reset-password adapters into `apps/api/domain_user/src/api/user/`; write failing adapter tests first for actor propagation, validation errors, self-protection, recognized roles, and response/error parity.
- [ ] 3.2 [domain_user service; prerequisite: 3.1] Add domain-local router state and `DomainUserService` implementing `DomainService`; construct one redacted `SupabaseAdminClient`, register every route as administrator-only, validate URL/service-role configuration before bind, and verify router/auth tests plus all GoTrue wiremock tests.

## 4. Gateway composition and administrator operation

- [ ] 4.1 [gateway; prerequisite: 2.4, 3.2] Register `DomainMediaService` and `DomainUserService` in `gateway::manifest`, preserving `DomainPostService` and `DomainAuthService`; verify manifest health/config behavior and the route-inventory test from 1.2.
- [ ] 4.2 [gateway API; prerequisite: 4.1] Add the administrator migration adapter at the gateway boundary, delegate to the existing orchestrator using the same service manifest/database connection, and test administrator authorization, success, and mapped failure without changing migration identities.
- [ ] 4.3 [gateway middleware; prerequisite: 4.1] Add failing then passing integration tests for public/protected/administrator auth, writer/admin role OR semantics, CORS, cookies, body limit, and telemetry layer application/order; verify no protected domain router is mounted bare.
- [ ] 4.4 [gateway contracts; prerequisite: 4.2-4.3] Run the full route/contract matrix against the composed gateway, including post/category/tag/GraphQL routes already owned by `domain_posts`, media/buckets, users, root/health, and migration; compare contractually significant results with the baseline from 1.3.

## 5. Legacy removal after parity

- [ ] 5.1 [cms API; prerequisite: 4.4] Delete duplicate post/category/tag adapters under `apps/api/src/api` only after source/graph search confirms canonical `domain_posts` routes and tests cover every consumer; verify no route or test imports the deleted modules.
- [ ] 5.2 [cms API; prerequisite: 4.4] Delete moved media and user adapter trees under `apps/api/src/api` and remove obsolete module declarations/re-exports; verify all live imports resolve to `domain_media` or `domain_user`.
- [ ] 5.3 [runtime; prerequisite: 5.1-5.2] Delete `apps/api/src/bin/legacy_bootstrap.rs`, legacy `AppState`, root/common/presentation shims, and unused root `cms` dependencies only after `cargo build --bin my-cms-api` and route parity pass; retain any compatibility crate/file with a proven importer and document the follow-up.
- [ ] 5.4 [architecture; prerequisite: 5.3] Search for forbidden live references (`cms::api`, `legacy_bootstrap`, legacy `AppState`, duplicate adapter symbols), confirm API adapters contain no business logic, and confirm no generated SeaORM entity or migration identity changed; verify with source search and diff review.

## 6. Rollout, rollback, and verification

- [ ] 6.1 [domain tests; prerequisite: 5.4] Run focused tests for `domain_media`, `domain_user`, `domain_posts`, `domain_auth`, `domain_interface`, and gateway, including PostgreSQL testcontainers and Storage/GoTrue wiremock suites; record pass counts and any intentionally ignored live-service tests.
- [ ] 6.2 [graph; prerequisite: 5.4] Rebuild/update the graph and inspect changed communities, callers/callees/imports, affected flows, impact radius, bridge/hub nodes, and `tests_for` every high-risk adapter; resolve or explicitly justify each material gap.
- [ ] 6.3 [staging; prerequisite: 6.1-6.2] Build the gateway image, deploy alongside the prior legacy image, smoke test the complete route matrix and compare auth/error telemetry; document traffic cutover and image-level rollback criteria without database rollback.
- [ ] 6.4 [OpenSpec/repository; prerequisite: 6.3] Run `openspec status --change "relocate-legacy-api-adapters-to-domains" --json`, `openspec validate "relocate-legacy-api-adapters-to-domains" --type change --strict --json`, `cargo check --workspace --all-targets`, `cargo test --workspace --all-targets --no-fail-fast`, `cargo fmt --all -- --check`, `cargo clippy --workspace --all-targets --all-features`, `pnpm --dir apps/web build`, and `git diff --check`; resolve critical failures and report warnings.
