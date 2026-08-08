## Context

The `my-cms-api` production binary is produced exclusively by the `gateway` crate (`apps/api/gateway/Cargo.toml` declares `[[bin]] name = "my-cms-api" path = "src/main.rs"`). The root `apps/api/Cargo.toml` also declared a `cms` package (`[package] name = "cms"`, version `0.1.0`) and a duplicate `[[bin]] name = "my-cms-api" path = "gateway/src/main.rs"`, plus a root-level `[dependencies]` block that only fed the `cms` package. The `cms` library at `apps/api/src/lib.rs` declared an `AppState` and re-exported `apps/api/src/api::**` — but the only consumer of those re-exports was `apps/api/src/bin/legacy_bootstrap.rs`, which was itself only referenced by the root `[bin]` declaration. Neither `legacy_bootstrap` nor the `cms` lib was built into any image the deployment pipeline ships.

The `application_core` crate (`apps/api/application_core/**`) was the second piece of dead weight. Its final surface was: `src/commands/mod.rs` (an empty stub after the `split-media-and-user-domains-merge-tags-into-posts` archive change deleted the historical `commands::{post,ai,tag,media,user}` modules); `src/common/{mod.rs,app_error.rs,datetime_generator.rs,extensions.rs}` (defining `AppError`, a datetime generator, and extension traits, but only consumed by the legacy `apps/api/src/api/**` HTTP adapters — also being deleted); and `src/entities/mod.rs` (a pure re-export shim `pub use domain_posts::entities::*;`). The only remaining importers of `application_core::*` were the legacy HTTP adapters under `apps/api/src/api/**` and the stale `application_core` path-deps in `apps/api/Cargo.toml` + `apps/api/gateway/Cargo.toml`.

After Phase A retired `cms`, `application_core`, and `legacy_bootstrap`, the `migration` crate (`apps/api/migration/**`) remained useful only as a thin re-export shim (`apps/api/migration/src/lib.rs` → `pub use domain_posts::migrations::*;`) for `apps/api/test_helpers/src/lib.rs`'s `use migration::{Migrator, MigratorTrait};` import. Its standalone CLI binary (`apps/api/migration/src/main.rs`) was also the operator-facing migration entrypoint shipped in the container and invoked by the Docker Swarm `migrate` one-shot. Phase B (this revision) extends the cleanup by deleting `apps/api/migration/` wholesale and re-targeting `test_helpers`, the Docker image, and the operator migration workflow to the canonical `domain_posts` bin's `migrate` subcommand (`apps/api/domain_posts/src/main.rs` → `domain_posts::migrations_cli::handle_args`). The `domain_posts::migrations::Migrator` is the canonical migrator; only the consumer paths change.

The graph gate (`code-review-graph_get_minimal_context(task="purge-legacy-cms-and-application-core")`) reports 2,198 nodes / 23,446 edges / 507 files / medium risk (0.60) / 73 test gaps — consistent with the legacy tree being the largest single source of complexity in the workspace. The audit confirms no live cross-domain consumer of the legacy code: `domain_*` crates never import `cms::`, `application_core::`, or `migration::` directly; the only consumer of `migration::*` from runtime code is `apps/api/test_helpers/src/lib.rs` (which will be retargeted to `domain_posts::migrations`), and the only consumer of `application_core::*` was the legacy `apps/api/src/api/**` tree (already deleted in Phase A). `apps/web/` (frontend) has zero references to `application_core`, `legacy_bootstrap`, `migration`, or `cms::`.

## Goals / Non-Goals

**Goals:**
- Delete the legacy `cms` root package, the `legacy_bootstrap` binary, the `src/api/**` HTTP-adapter tree, and the `src/{common,presentation_models}/**` shims from `apps/api/src/**`. *(Phase A — already landed in the working tree at the time of this revision.)*
- Delete the `application_core` transitional crate from the workspace. *(Phase A — already landed.)*
- Remove the stale `application_core` and `migration` path-dependencies from `apps/api/gateway/Cargo.toml`. *(Phase A — already landed.)*
- Convert `apps/api/Cargo.toml` to a pure virtual workspace by removing the root `[package]`, the duplicated `[[bin]]`, and the root `[dependencies]` block. *(Phase A — already landed.)*
- **Extended in this revision (Phase B):** Delete `apps/api/migration/` wholesale — the standalone CLI binary, the re-export shim library, the `Cargo.toml`, and the `README.md`. Remove `"migration"` from `apps/api/Cargo.toml` `[workspace] members`.
- **Extended in this revision (Phase B):** Switch `apps/api/test_helpers/Cargo.toml` to depend on `domain_posts = { path = "../domain_posts" }` (in `[dependencies]`) instead of `migration = { path = "../migration" }`, and update `apps/api/test_helpers/src/lib.rs` to import `domain_posts::migrations::{Migrator, MigratorTrait}` instead of `migration::{Migrator, MigratorTrait}`. The canonical migrator is unchanged (`domain_posts::migrations::Migrator`); only the import path changes.
- **Extended in this revision (Phase B):** Update `apps/api/Dockerfile` to copy `target/release/domain_posts` (produced by `domain_posts`'s `[[bin]] name = "domain_posts"`) instead of `target/release/migration`.
- **Extended in this revision (Phase B):** Update `deployments/docker-swarm/apps/docker-compose.yaml` `migrate` service to set `entrypoint: ["/app/domain_posts"]` and `command: ["migrate", "up"]`, replacing the previous `entrypoint: ["/app/migration"]` + `command: ["up"]`.
- **Extended in this revision (Phase B):** Refresh `docs/api-architecture.md`, `docs/pluggable-domain-refactor.md`, `docs/adding-a-domain.md`, `docs/ai-platform.md`, `docs/README.md`, the `.opencode/agents/*.md` files, and the `.agents/skills/map-my-cms-api-architecture/` and `.agents/skills/design-my-cms-api-change/` references to reflect the post-Phase A + migration-crate-removed state. Historical references must be explicitly labelled.
- Keep `my-cms-api` (from `gateway`) and `domain_posts` (from `domain_posts`) as the only two binaries the container image ships. The shipped container image contains both binaries; the Docker Swarm `migrate` service invokes `/app/domain_posts migrate up`.
- Produce an implementation-ready `tasks.md` with one writer per artifact and dependency-ordered, independently verifiable vertical slices.

**Non-Goals:**
- Wiring `domain_media` or `domain_user` into `gateway::manifest()` (separate change).
- Adding a gateway-owned `/administrator/database/migration` adapter (separate change — out of scope; the route is no longer registered anywhere).
- Touching any `domain_*` crate beyond incidental Cargo-graph rebuild verification (in particular, do NOT modify the canonical `apps/api/domain_posts/src/migrations/**` identities — they are preserved exactly).
- Changing HTTP routes, GraphQL mounts, auth roles, error mapping, SeaORM migration identities, or external integration behavior.
- Refactoring `apps/api/test_helpers/` beyond its single import-path swap.
- Refactoring `apps/api/domain_posts::migrations_cli` (kept as-is — the `migrate` subcommand already handles `up` and `--list`).
- Updating the `.github/workflows/ci-my-cms.yml` (the workflow runs `cargo test --all --all-features --manifest-path apps/api/Cargo.toml` which continues to work).
- Updating `deployments/k8s/charts/my-cms-api/**` (no template references `cms`, `application_core`, `legacy_bootstrap`, or `migration`).

## Decisions

### Decision 1: Delete `apps/api/src/**` wholesale rather than file-by-file *(Phase A)*
**Driver:** The legacy tree is single-purpose (only consumed by `legacy_bootstrap`, which was also deleted). Per-file deletion would have been 65 separate commits with no incremental review value; atomic deletion produced one reviewable change.

**Alternatives considered:**
- (a) Per-file removal ordered by import graph — rejected: every file is only consumed by other legacy files or by `legacy_bootstrap`, so no file could be removed independently of the others. This would have produced 65 commits with no functional review benefit.
- (b) Keep `AppState` + `presentation_models` as a shared helper for future domains — rejected: `AppState` aggregates media/user/admin concerns and `domain_*` crates already construct their own state via `DomainContext`. The patterns in `presentation_models/api_response.rs` (`ApiResponseWith`, `AxumResponse`, `ErrorCode`) are not used by any `domain_*` crate and were deleted from `domain_posts` already.
- (c) Wholesale deletion — **selected**: produced a single commit that leaves the workspace in a coherent "post-Phase A" state, matches the audit's finding that the tree is unreachable, and avoids leaving confusing half-deleted intermediate states.

**Consequences:** The `cms` library target disappeared. The `my-cms-api` bin target inside `apps/api/Cargo.toml` became unresolvable (its `path = "gateway/src/main.rs"` would point at a foreign-crate source file), so it was removed in the same change.

### Decision 2: Convert `apps/api/Cargo.toml` to a pure virtual workspace *(Phase A)*
**Driver:** The root manifest previously had both `[workspace]` and `[package]` (a hybrid root). The `[package]` only existed to give the root manifest something to attach `[bin]` and `[dependencies]` to. Once the `cms` lib and `legacy_bootstrap` bin were deleted, the `[package]` and its attached `[bin]` and `[dependencies]` blocks had no purpose.

**Alternatives considered:**
- (a) Keep the root `[package] name = "cms"` as an empty no-source package — rejected: a no-source package is dead weight and `cargo metadata` reported it as a buildable package with no outputs, which was misleading.
- (b) Keep root `[dependencies]` for shared workspace deps — rejected: in a Cargo workspace, `[dependencies]` in the root manifest apply only to the root package, not to all members. The `domain_*` crates declare their own deps in their own Cargo.toml, so removing the root `[dependencies]` does not affect them.
- (c) Convert to pure virtual workspace — **selected**: removes a misleading package declaration, matches the post-Phase A reality, and matches the convention used by Cargo for purely multi-crate workspaces.

**Consequences:** `apps/api/Cargo.toml` became a 5–10 line `[workspace]` manifest. `cargo build --workspace` continues to work because all member crates declare their own deps.

### Decision 3: Delete `apps/api/migration/` wholesale and re-target operator CLI to `domain_posts migrate` *(Phase B — this revision)*
**Driver:** After Phase A retired `cms`, `application_core`, and `legacy_bootstrap`, the `migration` crate's only remaining purpose was to host a CLI binary (`apps/api/migration/src/main.rs` → `cli::run_cli(migration::Migrator)`) and a one-line re-export shim (`apps/api/migration/src/lib.rs` → `pub use domain_posts::migrations::*;`). `test_helpers` already imports the same `Migrator` from `domain_posts::migrations` indirectly via the shim, and `domain_posts` already owns the operator-facing migration CLI (`apps/api/domain_posts/src/main.rs` routes `migrate` to `domain_posts::migrations_cli::handle_args`, which calls `domain_posts::migrations::Migrator::up`). The `migration` crate is now pure indirection.

**Alternatives considered:**
- (a) Keep `migration` as a re-export shim only (no bin) — rejected: the re-export shim is also pure indirection once `test_helpers` is retargeted to `domain_posts::migrations`. There is no remaining consumer after Phase B retargets `test_helpers`.
- (b) Keep `migration` as-is (Phase A baseline) — rejected: keeping a transitional shim defeats the purpose of the Phase A cleanup and signals an architectural intent (`migration` as a peer workspace member) that does not match the source. The workspace metadata reports `migration` as a buildable package even though its only consumer is `test_helpers`.
- (c) Wholesale `migration` deletion + retarget `test_helpers`, Docker image, Compose `migrate` service, and documentation to `domain_posts` — **selected**: removes the last transitional crate, makes `domain_posts` the single source of truth for both the canonical migrator and the operator CLI, and matches the audit's finding that the `migration` crate is single-purpose and dead-weight after Phase A.

**Consequences:** The `migration` workspace member, library, and CLI binary are gone. The operator-facing workflow changes from `/app/migration up` to `/app/domain_posts migrate up` (or `cargo run -p domain_posts -- migrate` locally). `cargo metadata --manifest-path apps/api/Cargo.toml --format-version=1 | jq '.packages[] | .name'` reports exactly seven packages: `domain_auth`, `domain_interface`, `domain_media`, `domain_posts`, `domain_user`, `gateway`, `test_helpers`. The four canonical migration identities are preserved exactly (no `up` history change).

### Decision 4: Remove `application_core` and `migration` path-deps from `apps/api/gateway/Cargo.toml` in the same change *(Phase A)*
**Driver:** `apps/api/gateway/src/main.rs` never imports `application_core::*` or `migration::*`. The migration orchestrator (`run_orchestrator`) calls `domain_posts::migrations_cli::run(conn)`. The stale deps inflate build graph and signal an architectural intent that does not match the source.

**Alternatives considered:**
- (a) Defer the dep removal until a follow-up change — rejected: leaving stale deps in a freshly-cleaned manifest defeats the purpose of the cleanup and is confusing to future readers.
- (b) Remove both stale path-deps in the same change — **selected**: deterministic, single review, audit-aligned.

**Consequences:** No runtime behavior change. `cargo build -p gateway --all-targets` rebuilds with a smaller dep graph.

### Decision 5: One-at-a-time deletion with verification between steps, committed as 3 reviewable commits *(Phase A) + 3 follow-up commits (Phase B)*
**Driver:** Each deletion should leave the workspace in a buildable state for code review. Phase A produced three commits, each independently verifiable:

1. Delete `apps/api/src/**` + drop root `[package]`, `[[bin]]`, root `[dependencies]`, and `application_core` path-dep from `apps/api/Cargo.toml`. Verify `cargo check --workspace`.
2. Delete `apps/api/application_core/**` + remove from `members`. Verify `cargo check --workspace`.
3. Remove `application_core` and `migration` path-deps from `apps/api/gateway/Cargo.toml`. Verify `cargo check -p gateway --all-targets`.

Phase B (this revision) extends the sequence with three further commits:

4. Retarget `test_helpers` (`Cargo.toml` `migration` dep → `domain_posts` dep; `src/lib.rs` import path swap). Verify `cargo check -p test_helpers --all-targets`.
5. Delete `apps/api/migration/**` + remove `"migration"` from `apps/api/Cargo.toml` `members`. Verify `cargo check --workspace` and `cargo metadata`.
6. Retarget `apps/api/Dockerfile` and `deployments/docker-swarm/apps/docker-compose.yaml` `migrate` service to `/app/domain_posts migrate up`. No new build verification (manifests only); refresh docs and run the full repository verification gate at the end.

**Alternatives considered:**
- (a) One atomic commit — rejected: makes review harder, and risks hiding an unintended cross-crate impact.
- (b) Per-file commits — rejected: 65+ commits for `apps/api/src/**` alone, no incremental review benefit.
- (c) Three commits per Phase A + three commits per Phase B — **selected**: each is small, reviewable, and leaves a green build. Phase B's commit 5 (delete `apps/api/migration/**`) is the only commit that affects `cargo metadata` output; it lands before Phase B commit 6 (deployment surface retarget).

**Consequences:** Code review can stage-gate each commit. The repository history clearly records the Phase A + Phase B cleanup as a planned sequence.

### Decision 6: Update documentation in `docs/`, `.opencode/agents/`, and `.agents/skills/` after the source deletions land
**Driver:** The `docs/api-architecture.md` legacy-shims section described the runtime state that this change retires. After the source is gone, the doc must reflect reality. The `.opencode/agents/software-architect.md` and `.opencode/agents/product-owner.md` files reference `apps/api/application_core/src/commands/...` and `apps/api/migration/src/...` paths that no longer exist; these references produce incorrect guidance to future agents. The `.agents/skills/map-my-cms-api-architecture/references/api-architecture.md` and `.agents/skills/design-my-cms-api-change/references/change-checklists.md` files also reference the retired paths and need explicit historical labelling.

**Alternatives considered:**
- (a) Leave documentation as a historical record — rejected: the docs/api-architecture.md file is the canonical architecture map for future SA work. Confusing historical-but-no-longer-true sections are misleading.
- (b) Delete the legacy-shims section entirely — rejected: it still describes important context for the staged cutover that is now in progress.
- (c) Annotate the legacy-shims section with a clear note that the shims are retired and link to this change — **selected**: preserves historical context for future readers while making the runtime state explicit. All other affected docs receive parallel updates.

**Consequences:** Small doc touch-up; no source code change. Future SA reads of the architecture docs accurately reflect that `cms`, `application_core`, `legacy_bootstrap`, and `migration` are retired and that `domain_posts` owns the canonical migrator + operator CLI.

## Risks / Trade-offs

- **[Risk]** `apps/api/test_helpers/src/lib.rs` accidentally depends on something else from `migration::*` that is NOT currently imported. → **Mitigation:** The only `use migration::*` references were `use migration::{Migrator, MigratorTrait};` in `apps/api/test_helpers/src/lib.rs`, `pub use domain_posts::migrations::*;` in `apps/api/migration/src/lib.rs` (deleted), and `use sea_orm_migration::prelude::*;` + `cli::run_cli(migration::Migrator)` in `apps/api/migration/src/main.rs` (deleted). The retarget swap is one-to-one: `use domain_posts::migrations::{Migrator, MigratorTrait};` is the canonical path and is already exported from `apps/api/domain_posts/src/migrations/mod.rs`. Run `cargo check -p test_helpers --all-targets` after the swap to confirm.
- **[Risk]** Operator-facing Docker Swarm `migrate` service breaks after the retarget because the existing image still contains `/app/migration`. → **Mitigation:** `apps/api/Dockerfile` and `deployments/docker-swarm/apps/docker-compose.yaml` are updated in the same commit (Phase B commit 6). The new image contains only `/app/my-cms-api` and `/app/domain_posts`; the `migrate` service invokes `/app/domain_posts migrate up`. Existing operators that pinned `/app/migration up` via `docker-compose.override.yaml` must update to `/app/domain_posts migrate up` (or rely on the new Compose default). Documented in `proposal.md` Impact.
- **[Risk]** A documentation file (`.opencode/agents/`, `docs/`, `openspec/changes/archive/**`) still references `apps/api/migration/src/` or `apps/api/application_core/src/commands/...` paths and breaks a docs-only CI check. → **Mitigation:** The CI workflow (`.github/workflows/ci-my-cms.yml`) only runs `cargo test --all --all-features`; it does not grep markdown for path references. The docs touch-up is a soft-update, not a CI gate. Phase B commit 6 explicitly refreshes all in-scope docs to remove the stale references and label historical ones.
- **[Risk]** Removing the root `[dependencies]` block breaks some workspace member that relies on an implicit dep inheritance. → **Mitigation:** Cargo root-manifest `[dependencies]` only apply to the root package itself; they are not inherited by members. Each member crate declares its own deps. Confirmed by inspecting `apps/api/gateway/Cargo.toml`, `apps/api/domain_posts/Cargo.toml`, `apps/api/test_helpers/Cargo.toml`, and the `domain_*` Cargo.toml files: all are self-contained.
- **[Risk]** `cargo build --release --workspace` after the Phase B cleanup produces a different set of binaries than before. → **Mitigation:** The two shipped binaries (`my-cms-api` from `gateway`, `domain_posts` from `domain_posts`) are still produced. The `migration` binary is intentionally removed; it was the only transitional CLI. The `legacy_bootstrap` binary was already removed in Phase A; it was not shipped. The `cms` package had no output of its own.
- **[Risk]** The graph-knowledge cache (`code-review-graph`) was built before the cleanup and may continue to reference the deleted paths in `get_affected_flows` / `get_impact_radius` results. → **Mitigation:** This is a documentation-only risk. Future code-review work will rebuild the graph after the cleanup lands. The cleanup does not depend on the graph.
- **[Trade-off]** Phase A + Phase B leave a small gap: media/user routes are temporarily unreachable because the gateway still does not register `domain_media` or `domain_user`. → **Mitigation:** The follow-up change wires those domains. The `domain-api-cutover` requirement scenario "Legacy runtime is retired safely in Phase A" explicitly documents the gap and the follow-up plan.

## Migration Plan

**Deployment:** Operator-facing workflow change. The shipped container image now contains `my-cms-api` (from `gateway`) and `domain_posts` (from `domain_posts`). `apps/api/Dockerfile` line 13 is retargeted from `target/release/migration` to `target/release/domain_posts`. The Docker Swarm `migrate` service is retargeted from `entrypoint: ["/app/migration"]` + `command: ["up"]` to `entrypoint: ["/app/domain_posts"]` + `command: ["migrate", "up"]`. The Helm chart is unchanged. The migration identities and `up` semantics are unchanged.

**Rollback:** Each commit is independently revertable. To roll back: `git revert <commit-sha>` per commit in reverse order. No data, schema, or migration-identity change means no database rollback needed.

**Rollout:** Three Phase A commits + three Phase B commits land in a single PR (or sequenced PRs at the Product Owner's discretion). Each commit leaves `cargo check --workspace` green. Code review proceeds per commit. After merge to `main`, CI runs the existing `cargo test --all --all-features --manifest-path apps/api/Cargo.toml` workflow (already in `.github/workflows/ci-my-cms.yml`) and the existing local verification gate (`cargo check && cargo test && cargo fmt -- --check && cargo clippy`) per `AGENTS.md`.

## Open Questions

- None blocking Phase B. The legacy bootstrap's `/administrator/database/migration` route is intentionally retired; the route is no longer registered anywhere. If a future product decision re-introduces that route, the follow-up change will add a gateway-owned adapter that delegates to `domain_posts::migrations::Migrator` (the canonical migrator). This is documented in the `docs/api-architecture.md` cutover section and is out of scope for this change.