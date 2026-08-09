## Context

### Source-derived current state (revalidated 2026-08-09)

**Dockerfile** (`apps/api/Dockerfile:1-16`):

```dockerfile
FROM rust:1.97 as build

WORKDIR /usr/local/my-cms

COPY . .

RUN cargo build --release --workspace


FROM rust:1.97-slim
WORKDIR /app
COPY --from=build /usr/local/my-cms/target/release/my-cms-api .
COPY --from=build /usr/local/my-cms/target/release/domain_posts .

EXPOSE 8989
CMD ["/app/my-cms-api"]
```

The build stage compiles the full workspace (`--workspace`), which is
required because the `gateway` crate depends on every `domain_*` crate.
The runtime stage copies two binaries (`my-cms-api` and `domain_posts`).
After Slice 2, the `domain_posts` binary no longer exists; the second
`COPY` line fails the build at runtime.

**Docker Swarm migrate service** (`deployments/docker-swarm/apps/docker-compose.yaml:56-70`):

```yaml
migrate:
  ...
  entrypoint: ["/app/domain_posts"]
  command: ["migrate", "up"]
```

This is the only consumer of the `domain_posts` binary in production.
After Slice 2, the `/app/domain_posts` binary is absent; the `migrate`
service fails to start.

**Doc references to retire** (revalidated 2026-08-09):

| File | Line(s) | Current text | Replacement |
|------|---------|--------------|-------------|
| `docs/api-architecture.md` | 1, 9, 76, 98, 107, 509, 523, 578 | `domain_posts migrate up` / `cargo run -p domain_posts -- migrate` / `apps/api/domain_posts/src/main.rs` | `my-cms-api migrate up` (operator) / `cargo run -p gateway -- migrate up` (local) |
| `docs/pluggable-domain-refactor.md` | 44, 120, 122 | `domain_posts migrate up` / `cargo run -p domain_posts -- migrate --list` / `cargo run -p domain_posts` | `my-cms-api migrate up` / `cargo run -p gateway -- migrate --list` / `cargo run -p gateway` |
| `docs/ai-platform.md` | 58 | `cargo run -p domain_posts -- migrate [--list]` | `cargo run -p gateway -- migrate [--list]` |
| `.opencode/agents/product-owner.md` | 72 | `/app/domain_posts migrate up` | `/app/my-cms-api migrate up` |
| `.opencode/agents/software-architect.md` | 75, 97 | `apps/api/domain_posts/src/main.rs` | `apps/api/gateway/src/main.rs` |

### Graph evidence

`code-review-graph_get_minimal_context(task="decompose parent wire-all-domains...")`
returns the same graph as Slices 1 and 2 (built at HEAD `5735e47`,
branch `feat/wire-all-domains-and-collapse-to-gateway-binary`,
`head_matches_build: true`). The graph covers Dockerfile + compose file
via the docker_swarm community.

### Constraints (from AGENTS.md, the parent change, and pre-existing findings)

- The workspace build (`--workspace`) MUST be preserved in the Dockerfile
  because `gateway` depends on every `domain_*` crate.
- No new env vars. No new HTTP routes. No new database migrations.
- The `purge-legacy-cms-and-application-core` change (41/47 tasks) edits
  the same `Dockerfile` and `docker-compose.yaml`. This slice MUST land
  AFTER the in-progress change's last six tasks (which are doc-only)
  resolve, otherwise both changes will collide. Coordinate via the
  software-engineer review step.
- Archive and historical references in `openspec/changes/archive/` and
  `docs/superpowers/plans/2026-08-08-remove-legacy-migration-crate.md`
  are intentionally LEFT UNCHANGED (decision archaeology, not operator
  guidance).

## Goals / Non-Goals

**Goals**

- `apps/api/Dockerfile` runtime stage contains exactly one binary
  (`my-cms-api`).
- `deployments/docker-swarm/apps/docker-compose.yaml` `migrate` service
  uses `entrypoint: ["/app/my-cms-api"]` and `command: ["migrate", "up"]`.
- Five doc files (`docs/api-architecture.md`,
  `docs/pluggable-domain-refactor.md`, `docs/ai-platform.md`,
  `.opencode/agents/product-owner.md`,
  `.opencode/agents/software-architect.md`) are updated to name the
  gateway binary.
- A repo-wide `rg` for the retiring surface returns only archive-folder
  matches.
- `docker build -f apps/api/Dockerfile -t my-cms-api:dev .` succeeds.
- `docker run --rm my-cms-api:dev migrate --list` prints the four
  migration ids.

**Non-Goals**

- No Rust code change. Slices 1 and 2 own the Rust surface.
- No CLI surface change (Slice 2 owns the CLI).
- No k8s Helm chart change (`deployments/k8s/` already targets the
  `my-cms-api` deployment and delegates migration to the Docker Swarm
  one-shot).
- No OpenTelemetry / Jaeger wiring change.
- No new env vars. No new HTTP routes. No new database migrations.

## Decisions

### Decision 1: Keep workspace build, drop runtime `domain_posts` copy

**Driver.** The Dockerfile's build stage uses
`cargo build --release --workspace` because the `gateway` crate depends
on every `domain_*` crate (now five: `domain_interface`, `domain_posts`,
`domain_auth`, `domain_media`, `domain_user`). Dropping the workspace
flag breaks `cargo` because the dependencies are not in the build
manifest.

**Decision.** Edit `apps/api/Dockerfile` as follows:

```dockerfile
FROM rust:1.97 as build

WORKDIR /usr/local/my-cms

COPY . .

# Build the whole workspace — gateway depends on every domain_* crate.
RUN cargo build --release --workspace --bin my-cms-api

FROM rust:1.97-slim
WORKDIR /app
COPY --from=build /usr/local/my-cms/target/release/my-cms-api .

EXPOSE 8989
CMD ["/app/my-cms-api"]
```

The `--workspace --bin my-cms-api` combination builds the gateway binary
and its transitive dependencies (which is the full workspace) without
building the now-deleted `domain_posts` standalone binary.

**Alternatives considered.**
- (a) **Drop the `--workspace` flag and rely on `gateway`'s Cargo
  dependencies to pull in every domain crate transitively.** *Considered,
  equivalent.* The `--workspace` flag is explicit; leaving it in keeps
  the build intent visible. The `--bin my-cms-api` filter prevents
  cargo from building other `[[bin]]` targets (e.g.
  `domain_auth/src/main.rs`) that may exist for local development.
- (b) **Keep the workspace build but copy every binary.** *Rejected*:
  ships binaries the operator no longer invokes; the
  `purge-legacy-cms-and-application-core` change removed the prior
  `migration` bin for the same reason.

**Consequences.** `docker build` produces an image with `ls /app`
containing only `my-cms-api`. The `--bin my-cms-api` filter is the
source-of-truth for which binary the image ships.

**Verification.** `docker run --rm my-cms-api:dev ls /app` returns
`my-cms-api` only. `docker run --rm my-cms-api:dev migrate --list`
prints four migration ids (requires a local Docker daemon with a
reachable `DATABASE_URL`).

### Decision 2: Retarget the `migrate` compose service

**Driver.** `deployments/docker-swarm/apps/docker-compose.yaml:67-68`
invokes `/app/domain_posts migrate up`. After Slice 2, that binary no
longer exists.

**Decision.** Edit lines 67-68:

```yaml
entrypoint: ["/app/my-cms-api"]
command: ["migrate", "up"]
```

The `migrate` service keeps the same `depends_on` chain (`init-wait` →
`migrate` → `my-cms-api`) and the same `restart: "no"` policy. The
`migrate` container is a one-shot that exits 0 after applying
migrations, exactly as before.

**Alternatives considered.**
- (a) **Move the `migrate` service into the gateway's compose
  definition as an init container.** *Rejected*: changes the operator
  UX (operators run `docker compose up migrate` today); the retargeted
  one-shot preserves the existing invocation.
- (b) **Make `migrate` a Kubernetes Job rather than a Docker Swarm
  one-shot.** *Rejected*: the k8s Helm chart (`deployments/k8s/`) already
  targets the `my-cms-api` deployment; the Docker Swarm path is the
  operator-facing surface that this slice preserves.

**Consequences.** `docker compose -f deployments/docker-swarm/apps/docker-compose.yaml up migrate`
applies the four migrations and exits 0. The local `migrate` container
shares the `my-cms-api` Dockerfile; the runtime image is identical to
the `my-cms-api` container's image.

**Verification.** `rg '"/app/domain_posts"' deployments/` returns no
matches. `rg '"/app/my-cms-api"' deployments/docker-swarm/apps/docker-compose.yaml`
returns the new entrypoint.

### Decision 3: Doc updates — five files

**Driver.** Five doc files reference the retiring `domain_posts migrate`
CLI surface. Out-of-date docs cause operator confusion.

**Decision.** Update each occurrence with the gateway-binary equivalent
(per the source-revalidated table in the Context section above).
Archive and historical references are left unchanged.

**Alternatives considered.**
- (a) **Delete the historical doc references entirely.** *Rejected*: the
  archive folder preserves design history; deleting changes erases
  decision context.
- (b) **Replace references with a redirect note ("see
  `wire-all-domains-and-collapse-to-gateway-binary` for the new CLI").**
  *Rejected*: adds indirection for the operator; a one-line replacement
  is direct.

**Consequences.** Every operator-facing reference to the retiring CLI
is replaced. A repo-wide `rg` for the retiring surface returns only
archive-folder matches:

```bash
rg 'domain_posts migrate|cargo run -p domain_posts -- migrate|/app/domain_posts|domain_posts/src/main.rs' \
   docs/ .opencode/ deployments/
# Expected: only matches under openspec/changes/archive/ and docs/superpowers/.
```

**Verification.** The `rg` command above returns only the exempt
matches. The CI lint `openspec-changes-archive-only` (if defined) would
confirm the same.

### Decision 4: Capability grouping

**Driver.** The coordinator's note: "Slice 3 may introduce a new
capability `gateway-deployment-surface` OR extend `domain-api-cutover` —
pick whichever is cleaner."

**Decision.** Extend `domain-api-cutover`. The parent change's spec
already covers the Docker image and compose service in the "Gateway is
the sole deployed API binary" requirement (parent spec lines 26-48).
A separate `gateway-deployment-surface` capability would fragment the
same concept (deployed binary surface) across two specs.

**Consequences.** This slice's delta spec extends the
`domain-api-cutover` capability with one new requirement
("Operator-facing documentation matches the deployed CLI surface")
plus modifications to the "Gateway is the sole deployed API binary"
requirement's Docker scenarios. The cap stays single-concern.

**Verification.** `openspec list --specs` shows one canonical spec
under `domain-api-cutover`; no new spec is created.

### Decision 5: Test strategy

**Driver.** This slice is primarily a deployment-surface change with
doc updates. The verification is build-level, not behavioural.

**Decision.** Three verification layers:

1. **Build-level**:
   - `docker build -f apps/api/Dockerfile -t my-cms-api:dev apps/api`
     succeeds.
   - `docker run --rm my-cms-api:dev ls /app` returns `my-cms-api` only.
2. **Behavioural smoke** (requires local Docker daemon + reachable
   Postgres):
   - `docker run --rm my-cms-api:dev migrate --list` prints four
     migration ids.
   - `docker run --rm my-cms-api:dev migrate --help` prints usage,
     exits 0.
   - `docker run --rm my-cms-api:dev migrate <unknown>` prints usage to
     stderr, exits 1.
3. **Repo-wide `rg` lint**:
   - The command in Decision 3 returns only archive-folder matches.

**Alternatives considered.**
- (a) **Add a GitHub Actions workflow that builds the Docker image on
  every PR.** *Considered but deferred*: the Docker build requires a
  Postgres volume mount; CI integration is a follow-up.
- (b) **Skip the build-level verification (rely on operator smoke).**
  *Rejected*: the Dockerfile edit is the highest-risk change in this
  slice; a build failure is cheap to catch and expensive to ship.

**Verification.** All three layers run in the verification gate.

## Risks / Trade-offs

**[Risk]** The `purge-legacy-cms-and-application-core` change (41/47
tasks) edits the same `apps/api/Dockerfile` and
`deployments/docker-swarm/apps/docker-compose.yaml`. If that change
lands first with a different operator-CLI surface than expected, this
slice's Dockerfile + compose update may need a rebase. →
**Mitigation:** the in-progress change's last six tasks are doc-only
(per its `tasks.md` sections 8.x and 9.5). Coordinate via the
software-engineer review step before merging.

**[Risk]** Operators that pinned `/app/domain_posts migrate up` in
`docker-compose.override.yaml` (outside the tracked `docker-compose.yaml`)
will hit a missing-binary error on the next `docker compose up`. →
**Mitigation:** documented in the parent change's `proposal.md` Impact
("BREAKING for any operator that pinned `/app/domain_posts migrate up`
in a `docker-compose.override.yaml`. The replacement is
`/app/my-cms-api migrate up`.").

**[Risk]** The Dockerfile's `rust:1.97` base image is large (~1.5 GB).
A future change could optimise by switching to a multi-stage build with
`rust:1.97-slim` for the builder. → **Mitigation:** out of scope for
this slice; the build-time regression would dwarf the operator-facing
benefit.

**[Risk]** The doc-update `rg` command may miss new instances of the
retiring surface introduced after this slice lands (e.g. in a new doc
file). → **Mitigation:** add a CI lint
(`openspec-changes-archive-only`) as a follow-up if the drift recurs.

## Migration Plan

### Deployment
1. Land Slice 1 + Slice 2 first (the Rust surface).
2. Land this slice on the same branch.
3. Build the new image locally:
   `docker build -f apps/api/Dockerfile -t my-cms-api:dev apps/api`.
4. Push the image and update the Swarm stack. The `migrate` compose
   service picks up the new image automatically because it shares the
   `Dockerfile` with `my-cms-api`.
5. Verify `docker compose ... up migrate` applies the four migrations
   against a fresh database volume.
6. Verify `docker compose ... up my-cms-api` boots and `/health` returns
   `200 OK`.

### Rollback
Single atomic rollback: redeploy the prior image and prior compose file.
Both are restored from the release tag. No data rollback. No schema
rollback.

### Order of operations (suggested commit chain)
1. Edit `apps/api/Dockerfile`. Verify `docker build`. **Independent.**
2. Edit `deployments/docker-swarm/apps/docker-compose.yaml`. Verify
   `docker compose config`. **Depends on 1.**
3. Update the five doc files. Verify `rg` lint. **Independent of 1, 2.**
4. Run the full verification gate (task 4).

## Open Questions

1. **Should the `domain_auth/src/main.rs` placeholder bin be removed
   in this slice as well?** *Current default:* leave for a separate
   change; out of scope. **No action.**

2. **Should the Dockerfile's builder base image be slimmed to
   `rust:1.97-slim`?** *Current default:* leave unchanged. Build-time
   regression dominates; not a deployment-surface concern. **No action.**

3. **Should a CI lint (`openspec-changes-archive-only`) be added to
   enforce that no new live doc references the retiring surface?**
   *Considered but deferred*: a follow-up if the drift recurs. **No
   action in this slice.**
