## Why

The container image at `apps/api/Dockerfile:7-12` currently ships two
binaries — `my-cms-api` and `domain_posts` — and the Docker Swarm
`migrate` one-shot at
`deployments/docker-swarm/apps/docker-compose.yaml:67-68` invokes the
domain-specific binary via
`entrypoint: ["/app/domain_posts"] / command: ["migrate", "up"]`. After
Slice 2 (`gateway-migrate-cli-and-delete-domain-posts-bin`) the
`domain_posts` binary no longer exists: the migrate verb now lives on the
gateway binary (`my-cms-api migrate <verb>`), and the standalone bin's
`[[bin]]` has been removed from `apps/api/domain_posts/Cargo.toml`. The
container image and the compose service must be retargeted to match.

Five doc files also reference the retiring surface (`docs/api-architecture.md`,
`docs/pluggable-domain-refactor.md`, `docs/ai-platform.md`,
`.opencode/agents/product-owner.md`,
`.opencode/agents/software-architect.md`). Out-of-date docs cause
operator confusion and contradict the new CLI contract.

This slice is the third and final slice of the parent change
`openspec/changes/wire-all-domains-and-collapse-to-gateway-binary/`. It
collapses the deployment surface to the single `my-cms-api` binary and
updates every operator-facing doc reference. It deliberately does NOT
modify any Rust code (owned by Slice 1 + Slice 2) or any CLI surface
(owned by Slice 2).

## What Changes

- **MODIFIED** `apps/api/Dockerfile:7-12` — change
  `RUN cargo build --release --workspace` to keep the workspace build
  (the workspace build is required because `gateway` depends on every
  `domain_*` crate), and remove the `COPY --from=build .../domain_posts`
  line at line 13. The runtime image contains only `my-cms-api`.
- **MODIFIED** `deployments/docker-swarm/apps/docker-compose.yaml:67-68`
  — retarget the `migrate` service:
  - `entrypoint: ["/app/my-cms-api"]`
  - `command: ["migrate", "up"]`
- **MODIFIED** `docs/api-architecture.md` — replace every
  `domain_posts migrate up` reference with `my-cms-api migrate up`
  (operator path) and every `cargo run -p domain_posts -- migrate`
  reference with `cargo run -p gateway -- migrate up` (local path). Keep
  archive and historical references unchanged.
- **MODIFIED** `docs/pluggable-domain-refactor.md` — replace
  `cargo run -p domain_posts -- migrate --list` with
  `cargo run -p gateway -- migrate --list` and `cargo run -p domain_posts`
  (standalone) with `cargo run -p gateway` (composed). Update Stage 4
  description to mark the standalone-binary removal as completed.
- **MODIFIED** `docs/ai-platform.md:58` — replace
  `cargo run -p domain_posts -- migrate [--list]` with
  `cargo run -p gateway -- migrate [--list]`.
- **MODIFIED** `.opencode/agents/product-owner.md:72` — replace
  `/app/domain_posts migrate up` with `/app/my-cms-api migrate up`.
- **MODIFIED** `.opencode/agents/software-architect.md:75-97` — replace
  `apps/api/domain_posts/src/main.rs` with `apps/api/gateway/src/main.rs`
  in the migration-CLI row.

## Capabilities

### Modified Capabilities

- `domain-api-cutover`: The "Gateway is the sole deployed API binary"
  requirement's "Docker migrate service uses the gateway binary" scenario
  is updated to point at `my-cms-api` (the parent change already captured
  the contract in `specs/domain-api-cutover/spec.md:69-73`; this slice
  realises the contract in the deployment surface). The "No standalone
  domain binary is shipped" scenario (parent spec lines 30-34) is
  extended with the new `[[bin]]`-removed state of `domain_posts` and
  the retargeted compose service.

## Impact

- Affected code (all MODIFIED):
  - `apps/api/Dockerfile` — drop the `domain_posts` runtime copy.
  - `deployments/docker-swarm/apps/docker-compose.yaml` — retarget
    `migrate` entrypoint and command.
- Affected docs (all MODIFIED):
  - `docs/api-architecture.md`
  - `docs/pluggable-domain-refactor.md`
  - `docs/ai-platform.md`
  - `.opencode/agents/product-owner.md`
  - `.opencode/agents/software-architect.md`
- Affected deployment: the shipped container image contains one binary
  (`my-cms-api`) instead of two. Local builds run as
  `cargo run -p gateway -- migrate up`. The Docker Swarm `migrate`
  one-shot is functionally equivalent but smaller.
- **BREAKING** for any operator that pinned `/app/domain_posts migrate up`
  in a `docker-compose.override.yaml`. The replacement is
  `/app/my-cms-api migrate up`. This is a one-line edit per override;
  documented in the parent change's `proposal.md` Impact.
- No Rust code change. No CLI surface change. No new env vars. No new
  HTTP routes. No new database migrations.
- Archive and historical references in `openspec/changes/archive/` and
  `docs/superpowers/plans/2026-08-08-remove-legacy-migration-crate.md`
  are left unchanged (decision history, not operator guidance).

## Traceability to parent change

This slice is `tasks.md` §§6-7 of the parent change
`wire-all-domains-and-collapse-to-gateway-binary`. The parent change's
`proposal.md` (lines 47-58, 68-73), `design.md` Decisions 6-7, and the
"Gateway is the sole deployed API binary" requirement scenarios (parent
spec lines 28-48, 69-73) all apply verbatim to this slice.
