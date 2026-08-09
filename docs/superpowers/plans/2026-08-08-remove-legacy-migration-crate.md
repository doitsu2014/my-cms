# Remove Legacy Migration Crate Implementation Plan

> **For agentic workers:** REQUIRED SUB-SKILL: Use superpowers:executing-plans to implement this plan task-by-task.

**Goal:** Remove `apps/api/migration/` and retarget operator migration execution to the canonical `domain_posts` CLI without changing migration identities.

**Architecture:** `domain_posts` remains the sole owner of SeaORM migrations and its `migrate` subcommand becomes the only operator CLI. `test_helpers` imports the canonical migrator directly; workspace, Docker, Compose, and documentation references are updated together.

**Tech Stack:** Rust, Cargo virtual workspace, SeaORM migration, Docker, Docker Compose, OpenSpec.

---

### Task 1: Retarget test helper migration imports

**Files:**
- Modify: `apps/api/test_helpers/Cargo.toml`
- Modify: `apps/api/test_helpers/src/lib.rs`

- [ ] Remove `migration = { path = "../migration" }` from `apps/api/test_helpers/Cargo.toml`.
- [ ] Add `domain_posts = { path = "../domain_posts" }` under `[dependencies]`.
- [ ] Replace `use migration::{Migrator, MigratorTrait};` with `use domain_posts::migrations::{Migrator, MigratorTrait};`.
- [ ] Run `cargo check -p test_helpers --all-targets` from `apps/api/`.

### Task 2: Remove migration workspace member and crate

**Files:**
- Modify: `apps/api/Cargo.toml`
- Delete: `apps/api/migration/Cargo.toml`
- Delete: `apps/api/migration/README.md`
- Delete: `apps/api/migration/src/lib.rs`
- Delete: `apps/api/migration/src/main.rs`

- [ ] Remove `"migration"` from the workspace members array.
- [ ] Delete the complete `apps/api/migration/` directory.
- [ ] Run `cargo check --workspace --all-targets` from `apps/api/`.
- [ ] Confirm metadata contains no `migration` package.

### Task 3: Retarget release image and migration service

**Files:**
- Modify: `apps/api/Dockerfile`
- Modify: `deployments/docker-swarm/apps/docker-compose.yaml`

- [ ] Replace the Dockerfile copy of `target/release/migration` with `target/release/domain_posts`.
- [ ] Change the Compose migration service entrypoint to `/app/domain_posts`.
- [ ] Change its command from `up` to `migrate up`.
- [ ] Verify no deployment file invokes `/app/migration`.

### Task 4: Refresh migration documentation

**Files:**
- Modify: `docs/api-architecture.md`
- Modify: `docs/pluggable-domain-refactor.md`
- Modify: `docs/adding-a-domain.md`
- Modify: `.opencode/agents/software-architect.md`
- Modify: `.opencode/agents/product-owner.md`
- Modify: `.agents/skills/map-my-cms-api-architecture/SKILL.md`
- Modify: `.agents/skills/map-my-cms-api-architecture/references/api-architecture.md`
- Modify: `.agents/skills/design-my-cms-api-change/references/change-checklists.md`

- [ ] Remove obsolete migration-crate descriptions.
- [ ] Document `domain_posts migrate up` as the operator command.
- [ ] Preserve references to canonical `domain_posts::migrations` and `migrations_cli`.

### Task 5: Verify binaries and migration identities

**Files:**
- None

- [ ] Run `cargo build --release --workspace` from `apps/api/`.
- [ ] Confirm `target/release/my-cms-api` and `target/release/domain_posts` exist.
- [ ] Confirm `target/release/migration` is not produced by a clean release build.
- [ ] Run `target/release/domain_posts migrate --list` and verify the four canonical migration IDs remain present and ordered.
- [ ] Run `cargo test --workspace` from `apps/api/`.
- [ ] Run `cargo fmt -- --check` and `cargo clippy --workspace --all-targets --all-features -- -D warnings` from `apps/api/`.
- [ ] Run `openspec validate purge-legacy-cms-and-application-core`.
- [ ] Run `openspec verify --change purge-legacy-cms-and-application-core` and resolve critical findings.
