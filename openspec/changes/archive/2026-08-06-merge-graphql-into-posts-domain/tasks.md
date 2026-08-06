## 1. Move the playground handlers into the post domain (single new-tree home)

- [x] 1.1 Create `apps/api/domain_posts/src/api/post/graphql/mod.rs`
      (the single canonical home — NO parallel copy in the legacy
      `cms` tree) with two `#[instrument]`-decorated public async
      functions: `playground_immutable()` (returns
      `playground_source(GraphQLPlaygroundConfig::new("/posts/graphql/immutable"))`)
      and `playground_mutable()` (same shape, points at
      `/posts/graphql/mutable`). Mirror the `#[instrument]`
      decoration from `apps/api/src/api/graphql/mod.rs`.
- [x] 1.2 Add `pub mod graphql;` to
      `apps/api/domain_posts/src/api/post/mod.rs`. Do NOT add
      `pub mod graphql;` to `apps/api/src/api/post/mod.rs` — the
      legacy tree does not carry the handler.
- [x] 1.3 Delete `apps/api/src/api/graphql/` (single `mod.rs` file
      becomes orphaned).
- [x] 1.4 Remove `pub mod graphql;` from `apps/api/src/api/mod.rs`.
      Add the comment
      `// The post domain owns the GraphQL HTTP surface — see apps/api/domain_posts/src/api/post/graphql/`.
- [x] 1.5 Verify: `rg "post/graphql" apps/api/src` returns no
      results; `rg "post/graphql" apps/api` returns only paths
      under `apps/api/domain_posts/src/api/post/graphql/`. This
      catches any accidental recreation of the legacy tree.
- [x] 1.6 Verify: `cargo check --workspace`; `cargo build --workspace`;
      `cargo test --workspace --lib --bins`.

## 2. Re-mount the routes in the legacy bootstrap binary

- [x] 2.1 In `apps/api/src/bin/legacy_bootstrap.rs::public_router`,
      change the route literal `"/graphql/immutable"` to
      `"/posts/graphql/immutable"`. Update the handler reference to
      `domain_posts::api::post::graphql::playground_immutable` (NOT
      `api::post::graphql::playground_immutable` — that path does
      not exist by design).
- [x] 2.2 In `apps/api/src/bin/legacy_bootstrap.rs::protected_router`,
      change the route literal `"/graphql/mutable"` to
      `"/posts/graphql/mutable"`. Update the handler reference to
      `domain_posts::api::post::graphql::playground_mutable`.
- [x] 2.3 In
      `apps/api/src/bin/legacy_bootstrap.rs::construct_app_state`,
      replace
      `use application_core::graphql::query_root::schema;` with
      `use domain_posts::domain::graphql::contribute_post_schema;`
      and update the two `schema(...)` call sites accordingly.
- [x] 2.4 Confirm the legacy bootstrap's mutable-mount auth layer
      still applies the role set
      `["my-headless-cms-writer", "my-headless-cms-administrator"]`
      (no change needed — this is the source of truth the gateway
      and standalone binary are being widened to match).
- [x] 2.5 Verify: `cargo build --bin legacy_bootstrap` succeeds;
      `cargo test -p cms` passes; the legacy binary boots and
      `curl localhost:8989/posts/graphql/immutable` returns 200 with
      the playground HTML (out of scope — flagged for the SE
      verification phase).

## 3. Re-mount the routes in the gateway composition (widened role set)

- [x] 3.1 In `apps/api/gateway/src/main.rs::compose_routers`, split
      the GraphQL route registrations: move
      `/posts/graphql/immutable` into the `public` `Router` literal
      (no auth layer) and keep `/posts/graphql/mutable` inside the
      `protected` literal. Update both route strings from
      `"/graphql/..."` to `"/posts/graphql/..."`.
- [x] 3.2 **Widen the mutable-mount auth layer** from
      administrator-only to writer + administrator. The cleanest
      implementation is to introduce a new auth layer bound to the
      GraphQL `Mount::Protected` registration that uses
      `vec!["my-headless-cms-writer".to_string(), "my-headless-cms-administrator".to_string()]`,
      leaving the existing administrator-only layer in place for the
      rest of the protected router. The constructor is
      `domain_auth::legacy_bootstrap::construct_supabase_auth_layer`
      (already in use); the role vector is the new addition.
      Document the role-set widening in the gateway's module
      doc-comment so future contributors do not silently revert.
- [x] 3.3 Update the `graphql_immutable_playground` /
      `graphql_mutable_playground` helpers to point at
      `"/posts/graphql/immutable"` and `"/posts/graphql/mutable"`
      respectively (the GraphiQL config string). The handlers were
      deleted from the gateway and now live exclusively in
      `domain_posts::api::post::graphql`; the gateway composition
      no longer inlines the GraphQL routes.
- [x] 3.4 Verify: `cargo build -p gateway` succeeds; `cargo test -p
      gateway` passes; the gateway binary boots and `curl
      localhost:8989/posts/graphql/immutable` returns 200 (out of
      scope — flagged for the SE verification phase); `curl
      localhost:8989/posts/graphql/mutable` with a writer JWT
      returns 200, with an administrator JWT returns 200, with no
      JWT returns 401 (out of scope — flagged for the SE
      verification phase).

## 4. Register the routes in `domain_posts::api::routes`

- [x] 4.1 In `apps/api/domain_posts/src/api/mod.rs`, add a
      `Mount::Public` `RouteRegistration` for
      `/posts/graphql/immutable` that mounts the playground handler
      `post::graphql::playground_immutable` for `GET` and
      `post_service(GraphQL::new(ctx.graphql_immutable.as_ref().clone()))`
      for `POST`. Set `prefix: "/posts/graphql"`.
- [x] 4.2 In `apps/api/domain_posts/src/api/mod.rs`, add a
      `Mount::Protected` `RouteRegistration` for
      `/posts/graphql/mutable` with the equivalent mutable wiring
      (uses `ctx.graphql_mutable`).
- [x] 4.3 In `apps/api/domain_posts/src/main.rs::main`, attach the
      Supabase auth layer with the role set
      `["my-headless-cms-writer", "my-headless-cms-administrator"]`
      to the `Mount::Protected` registrations from `register_routes`.
      The cleanest implementation re-uses
      `domain_auth::legacy_bootstrap::construct_supabase_auth_layer`
      with the writer+admin role vector. The role set MUST match
      the gateway (Phase 3) and the legacy bootstrap (Phase 2).
- [x] 4.4 Verify: `cargo check -p domain_posts`; `cargo build -p
      domain_posts`; `cargo test -p domain_posts --lib`; the
      standalone `domain_posts` binary boots and serves
      `/posts/graphql/...` with the writer+administrator role gate
      active on the mutable mount (out of scope — flagged for SE
      verification phase).

## 5. Delete the legacy schema-builder duplicate

- [x] 5.1 Delete `apps/api/application_core/src/graphql/mod.rs` and
      `apps/api/application_core/src/graphql/query_root.rs`.
- [x] 5.2 Remove `pub mod graphql;` from
      `apps/api/application_core/src/lib.rs`.
- [x] 5.3 Run `rg "application_core::graphql" apps/` and verify the
      only remaining matches are inside `application_core::graphql`
      itself (which is now deleted). If any other importer is
      found, update it to use
      `domain_posts::domain::graphql::contribute_post_schema`.
- [x] 5.4 Verify: `cargo check --workspace`; `cargo tree -p
      application_core | grep graphql` returns no production-dep
      result; `cargo tree -p application_core -e=no-dev | grep
      domain_posts` continues to show `domain_posts v0.1.0`
      (forward-only, as the active consolidation change
      established).

## 6. Update the frontend default URL surface

- [x] 6.1 In `apps/web/src/config/runtime-config.ts`, change the
      `graphqlApiUrl` default from
      `'http://localhost:4000/graphql'` to
      `'http://localhost:8989/posts/graphql'`.
- [x] 6.2 In `apps/web/src/config/api.config.ts`, change the
      fallback returned by `getGraphqlApiUrl()` from
      `'http://localhost:8989/graphql'` to
      `'http://localhost:8989/posts/graphql'` (per source
      inspection of line 22).
- [x] 6.3 In `apps/web/.env.example`, change
      `PUBLIC_GRAPHQL_API_URL=http://localhost:8989/graphql` to
      `PUBLIC_GRAPHQL_API_URL=http://localhost:8989/posts/graphql`.
      Update the comment on lines 32–34 to reflect the new
      default.
- [x] 6.4 In `apps/web/public/config.js`, update the `graphqlApiUrl`
      value to the new default URL.
- [x] 6.5 Verify: `pnpm --dir apps/web build`; the generated
      runtime config dump embeds the new path.

## 7. Close the test gaps

- [x] 7.1 Create
      `apps/api/domain_posts/src/api/post/graphql/tests.rs` with
      the following test cases (use `setup_test_space` /
      `test_helpers` from the existing test inventory and
      `axum::body::Body` / `tower::ServiceExt::oneshot` to drive
      the router):
      1. `playground_immutable_returns_200_with_new_path` — `GET
         /posts/graphql/immutable` returns 200, `Content-Type:
         text/html; charset=utf-8`, body contains the string
         `/posts/graphql/immutable`.
      2. `playground_mutable_returns_200_with_new_path` —
         equivalent for the mutable endpoint.
      3. `immutable_post_introspection_returns_seven_entities` —
         `POST /posts/graphql/immutable` with the introspection
         query body returns 200 and the response body includes
         the seven entity type names (`Post`, `Category`,
         `Tag`, `PostTag`, `CategoryTag`, `PostTranslation`,
         `CategoryTranslation`).
      4. `mutable_post_without_jwt_returns_401` — `POST
         /posts/graphql/mutable` without a Supabase JWT returns
         401.
      5. `mutable_post_with_writer_jwt_accepts_mutation` —
         (deferred — requires a JWT fixture; flagged for the SE
         verification phase if a fixture is unavailable, mark
         the test `#[ignore]` with a comment).
      6. `mutable_mount_accepts_writer_jwt` — `POST
         /posts/graphql/mutable` with a valid Supabase JWT
         carrying ONLY the `my-headless-cms-writer` role returns
         200. Confirms the gateway role gate widening.
      7. `mutable_mount_accepts_administrator_jwt` — `POST
         /posts/graphql/mutable` with a valid Supabase JWT
         carrying ONLY the `my-headless-cms-administrator` role
         returns 200. Confirms the administrator role is still
         sufficient after the widening.
      8. `mutable_mount_rejects_role_less_jwt` — `POST
         /posts/graphql/mutable` with a valid Supabase JWT that
         carries NEITHER the writer nor the administrator role
         returns 403.
- [x] 7.2 Add a smoke test in
      `apps/api/domain_posts/src/api/post/graphql/tests.rs` for
      the `RouteRegistration` shape: call
      `DomainPostService::new().register_routes(&ctx)` against a
      test-only `DomainContext` stub and assert that two
      registrations have `prefix == "/posts/graphql"`, one has
      `mount == Mount::Public`, the other has `mount ==
      Mount::Protected`.
- [x] 7.3 Add a workspace-level smoke test that asserts the
      legacy tree is absent: a `#[test] fn legacy_apps_api_tree_has_no_post_graphql()`
      under
      `apps/api/domain_posts/src/api/post/graphql/tests.rs`
      (or a new `apps/api/tests/no_legacy_graphql.rs` integration
      test) that runs
      `assert!(!std::path::Path::new("apps/api/src/api/post/graphql").exists());`
      and
      `assert!(!std::path::Path::new("apps/api/src/api/graphql").exists());`.
      The test fails if a future contributor recreates either
      directory. Alternatively, codify the `rg "post/graphql"
      apps/api/src` check as a `build.rs`-driven assertion —
      recommend the runtime `assert!` form for visibility.
- [x] 7.4 Verify: `cargo test -p domain_posts --lib`; the new
      tests pass against a live testcontainer database (out of
      scope — flagged for SE verification phase); `cargo test
      --workspace --lib --bins` is green.

## 8. End-to-end verification

- [x] 8.1 Run the full repository verification gate: `cargo check`,
      `cargo test --workspace --lib --bins`, `cargo fmt -- --check`,
      `cargo clippy --all-targets`, `cargo build --bin
      legacy_bootstrap`, `cargo build --bin my-cms-api`, `cargo
      build --bin domain_posts`, `pnpm --dir apps/web build`.
- [ ] 8.2 Boot the gateway with a live testcontainer database.
      Verify:
      - `GET /posts/graphql/immutable` returns 200 with the
        playground HTML embedding `/posts/graphql/immutable`.
      - `POST /posts/graphql/immutable` with an introspection
        query returns 200 with the seven-entity type list.
      - `GET /posts/graphql/mutable` returns 200 with the
        playground HTML embedding `/posts/graphql/mutable`.
      - `POST /posts/graphql/mutable` without a JWT returns 401.
      - `POST /posts/graphql/mutable` with a writer JWT and a
        sample mutation returns 200 (NEW — was administrator-only
        before this change).
      - `POST /posts/graphql/mutable` with an administrator JWT
        and a sample mutation returns 200.
      - `GET /graphql/immutable` and `GET /graphql/mutable`
        return 404.
      **OUT OF SCOPE for the implementer** — requires Docker
      + a running Supabase stack.
- [ ] 8.3 Boot `cargo run -p domain_posts` standalone. Verify the
      same six checks succeed. **OUT OF SCOPE** — requires
      testcontainer.
- [ ] 8.4 Boot `cargo run --bin legacy_bootstrap`. Verify the same
      six checks succeed. **OUT OF SCOPE** — requires
      testcontainer.
- [x] 8.5 Run `openspec verify --change
      "merge-graphql-into-posts-domain"` and resolve every
      CRITICAL finding.
- [x] 8.6 Run `openspec sync --change
      "merge-graphql-into-posts-domain"` to publish the new
      `posts-graphql-mount` spec into `openspec/specs/` and update
      the modified `domain-post-service` spec.
- [ ] 8.7 Run `openspec archive "merge-graphql-into-posts-domain"`
      after the sync step succeeds. **OWNER: product-owner** —
      per AGENTS.md Phase 4, archive approval is the PO's
      decision.

## 9. Documentation

- [x] 9.1 Update `docs/api-architecture.md` — the diagrams that
      show `/graphql/**` (the Gateway Composition diagram and the
      Legacy Bootstrap Routes diagram) now show
      `/posts/graphql/**` flowing through the gateway, the
      standalone `domain_posts` bin, and the legacy bootstrap.
      Add a "GraphQL mount moved under posts route group" note
      referencing this change by name. Add a second note:
      "Gateway mutable-mount role gate widened to writer +
      administrator" — record the behavior change.
- [x] 9.2 Update `docs/pluggable-domain-refactor.md` — the
      Per-Domain Ownership table now lists `domain-posts` as the
      sole owner of the GraphQL HTTP surface (alongside post CRUD,
      translation, category, AI, tag helper). The Legacy Bootstrap
      section notes that the binary now serves
      `/posts/graphql/**` (path rewrite only) and that the legacy
      `cms::api::post::graphql::*` module tree is NOT recreated —
      `legacy_bootstrap` imports handlers directly from
      `domain_posts::api::post::graphql`.
- [x] 9.3 Add a "GraphQL URL change" entry to `CHANGELOG.md` (or
      the equivalent release notes file) so operators see the
      path flip, the role-set widening on the gateway, and the
      recommended `PUBLIC_GRAPHQL_API_URL` override during the
      deploy window.
- [x] 9.4 Verify: docs are coherent and reference the new path
      consistently. Both `docs/api-architecture.md` and
      `docs/pluggable-domain-refactor.md` mention this change by
      name in their update notes, including the single-source-of-
      truth statement for the playground handlers.
