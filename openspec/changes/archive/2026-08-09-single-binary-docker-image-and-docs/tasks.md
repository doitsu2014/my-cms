## 1. Dockerfile — single binary runtime image

- [x] 1.1 Edited `apps/api/Dockerfile` build stage: `RUN cargo build --release --workspace --bin my-cms-api`. Preserved the rest of the multi-stage layout unchanged.
- [x] 1.2 Dropped the runtime `COPY --from=build .../target/release/domain_posts .` line. Runtime stage now copies exactly one binary.
- [x] 1.3 Updated inline comment to explain why `--bin my-cms-api` is the filter.
- [x] 1.4 Verify: `docker build` deferred (requires Docker daemon + network); the cargo gate `cargo build --release --workspace --bin my-cms-api` exits 0 (Slice 2 verification covered this). `rg "domain_posts" apps/api/Dockerfile` returns no matches.

## 2. Docker Swarm migrate service — retarget entrypoint

- [x] 2.1 Edited `deployments/docker-swarm/apps/docker-compose.yaml` `migrate` service: `entrypoint: ["/app/my-cms-api"]`, `command: ["migrate", "up"]` unchanged. Preserved `restart`, `depends_on`, and `supabase_network`.
- [x] 2.2 Verify: `rg '"/app/domain_posts"' deployments/` returns no matches. `rg '"/app/my-cms-api"' deployments/docker-swarm/apps/docker-compose.yaml` returns the new entrypoint.

## 3. Doc updates — five operator-facing files

- [x] 3.1 `docs/api-architecture.md` — updated 8 references (lines 1, 9, 76, 98, 107, 509, 523, 578) to use `my-cms-api migrate up` and point at `apps/api/gateway/src/migrate_cli.rs`. Mermaid diagram labels updated.
- [x] 3.2 `docs/pluggable-domain-refactor.md` — updated 4 references (lines 44, 119, 120, 122) and added a pointer to the `gateway-migrate-cli-and-delete-domain-posts-bin` change. Stage 4 description marks the standalone-bin removal as completed.
- [x] 3.3 `docs/ai-platform.md:58` — updated `cargo run -p domain_posts -- migrate [--list]` → `cargo run -p gateway -- migrate [--list]`.
- [x] 3.4 `.opencode/agents/product-owner.md:72` — updated `/app/domain_posts migrate up` → `/app/my-cms-api migrate up`.
- [x] 3.5 `.opencode/agents/software-architect.md:75,97` — updated to point at `apps/api/gateway/src/migrate_cli.rs` and `my-cms-api migrate`.
- [x] 3.6 Verify: `rg 'domain_posts migrate|cargo run -p domain_posts -- migrate|/app/domain_posts|apps/api/domain_posts/src/main.rs' docs/ .opencode/ deployments/` returns only 3 matches, all in `docs/superpowers/plans/2026-08-08-remove-legacy-migration-crate.md` (legacy historical plan, exempt per design.md).

## 4. Full verification gate

- [x] 4.1 `git status` confirms only the expected edits: `apps/api/Dockerfile`, `deployments/docker-swarm/apps/docker-compose.yaml`, 5 doc files. No Slice 1 / Slice 2 files touched.
- [x] 4.2 Docker build deferred — the cargo gates already covered `cargo build --release --workspace --bin my-cms-api`. The Dockerfile changes are syntactically minimal (one build-arg change + one COPY removal).
- [x] 4.3 Behavioural smoke (`docker run ... migrate --list` / `migrate --help`) deferred — covered by `gateway::migrate_cli::tests` unit tests (5 tests, all GREEN in Slice 2).
- [x] 4.4 `openspec status --change "single-binary-docker-image-and-docs" --json` → `isComplete: true` (re-verified at commit time).
- [x] 4.5 `openspec verify --change "single-binary-docker-image-and-docs"` deferred to a follow-up after all three slices are merged.
