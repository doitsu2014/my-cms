## Developer Code Scan Results

### Scan Coverage
- **Analyzed deeply**:
  - `./` — full-repository inventory and architecture discovery were performed within the requested snapshot scope; this is a depth-mixed scan, not a claim that all 437+ source files were individually read.
  - `apps/api/Cargo.toml`, `apps/api/{gateway,domain_interface,domain_posts,domain_auth,domain_media,domain_user,domain_seo,test_helpers}/Cargo.toml` — workspace membership and crate boundaries.
  - `apps/api/gateway/src/main.rs`, `apps/api/gateway/src/backup_cli.rs`, `apps/api/gateway/src/migrate_cli.rs` — current uncommitted gateway wiring and backup/restore CLI surface, directly relevant to intent.
  - `apps/web/package.json`, `apps/ducth-dev-website/package.json`, `packages/editor-prose/package.json` and TypeScript configs/workspace descriptors — frontend build, frameworks and dependency versions.
  - `docs/features/backup-restore.md`, `README.md`, `docs/README.md`, `docs/architecture/`, `docs/features/` — project-maintained architecture, intent feature narrative and documentation structure.
  - `.github/workflows/`, `.github/action-rs/`, repository and app `.gitignore` files — CI/release/quality workflow and exclusions.
  - `apps/api/domain_posts/src/`, `apps/api/domain_auth/src/`, `apps/api/domain_media/src/`, `apps/api/domain_user/src/`, `apps/api/domain_seo/src/`, `apps/api/domain_interface/src/`, `apps/api/test_helpers/src/` — crate source trees inventoried; representative manifests and module layouts examined.
- **Skimmed only**:
  - `apps/web/src/` — UI source organized into app, auth, components, config, domains, infrastructure, metadata, models, schemas and tests; broad inventory, not every component read.
  - `apps/ducth-dev-website/src/` — app, components, i18n, infrastructure, lib, metadata, pages, test and types; broad inventory.
  - `packages/editor-prose/` — package and source/test layout.
  - `deployments/docker-swarm/`, `deployments/k8s/` — deployment/configuration inventory; no deployment executed.
  - `scripts/`, `.vscode/`, remaining documentation, root config and static assets — directory-level inventory.
- **Left out**:
  - `.git/`, `.claude/`, `aidlc/` — VCS metadata and AI-DLC installation/workspace excluded from project-code scan (the required intent-local handoff is the sole write).
  - `**/target/`, `**/node_modules/`, `**/dist/`, `**/build/`, `**/coverage/`, `.pnpm-store/`, `.idea/`, `.vscode` caches, `.od-skills/`, `.rtk/`, `.worktrees/`, `.code-review-graph/` — generated/dependency/tool material, ignored or non-product caches.
  - ignored secret/environment files and deployment volumes per `.gitignore`.

### Packages Found
- `gateway` — application composition root and shipped API binary — Rust — mounts domains, coordinates configuration, listeners, common middleware and CLI functions.
- `domain_interface` — shared domain contract — Rust — service interface and shared API/domain types.
- `domain_posts` — domain crate — Rust — posts, categories, tags, translations, translation jobs, GraphQL, persistence entities and owned migrations.
- `domain_auth` — domain/infrastructure crate — Rust — authentication configuration, JWT/role middleware and service integration.
- `domain_media` — domain crate — Rust — media and bucket APIs, Supabase Storage adapter and handlers.
- `domain_user` — domain crate — Rust — administrator user operations via Supabase/GoTrue integration.
- `domain_seo` — domain crate — Rust — SEO head asset validation, persistence and routes.
- `test_helpers` — test support crate — Rust — reusable backend test utilities.
- `apps/web` (`admin_side`) — web application — TypeScript/React — administrative CMS/editor.
- `apps/ducth-dev-website` — public reader — TypeScript/React with Express server — localized public site and SSR/server support.
- `packages/editor-prose` — shared frontend library — TypeScript/React — common saved-article prose rendering consumed by both frontends.
- `deployments/docker-swarm`, `deployments/k8s` — deployment/config packages — YAML, Docker and shell — local stacks and Kubernetes manifests.

### Build System
- **Type**: Cargo workspace for Rust API plus pnpm-managed TypeScript applications/packages; each frontend package advertises pnpm 10.33.0.
- **Config Files**: `apps/api/Cargo.toml`, individual crate `Cargo.toml`s, `apps/api/Cargo.lock`; frontend `package.json`, `pnpm-lock.yaml`, `pnpm-workspace.yaml`, `tsconfig.json`; Rsbuild, Vite/Vitest, ESLint, Prettier, Tailwind/PostCSS configs; Dockerfiles and deployment YAML; GitHub Actions under `.github/workflows/`.
- **Build Dependencies**: gateway depends on domain crates and shared interface; domains depend on `domain_interface`; post persistence and migration remain owned by `domain_posts`; `test_helpers` supports tests. Both web apps depend on local `editor-prose`. No project command was executed.

### APIs Discovered
- Axum HTTP APIs provided through the gateway and domain route composition: posts/category/tag/translation, auth-protected media and bucket operations, user administration, SEO assets, health and migration/backup CLI pathways. Exact route counts not enumerated due mixed-depth broad scan; GraphQL immutable/mutable surfaces are implemented in posts domain.
- Public website: React Router routes rendered via an Express 5 server; Apollo GraphQL client to backend.
- Admin: React Router/Apollo client and Supabase JS integration.

### Frameworks & Libraries
- Rust versions/dependency versions are declared in Cargo manifests/lockfile; key framework evidence is Axum and SeaORM, with Tokio/Serde ecosystem. Exact full dependency inventory intentionally delegated to architect synthesis.
- React `^19.1.1`, React Router `^7.9.0` admin / `^7.11.0` public, Apollo Client `^3.12.6` / `^3.14.0`, TypeScript `^5.9.2`, Rsbuild `^1.5.6`, Vitest `^4.1.8`.
- Public server: Express `^5.2.1`; Tailwind CSS `^4.0.12` admin / `^4.1.18` public; TipTap `^3.14.0` admin; Supabase JS `^2.106.2` admin; OpenTelemetry packages in public server.

### Test Coverage
- **Test Directories**: Rust inline/unit and integration tests across domain crates; frontend `apps/web/src/**/__tests__` and `test`; public `apps/ducth-dev-website/src/test` and `pages/__tests__`; package tests under `packages/editor-prose`.
- **Test Frameworks**: Cargo tests (not run); Vitest, Testing Library, jsdom; no test/build/program execution performed per instruction.
- **Coverage Config**: Rust grcov config `.github/action-rs/grcov.yml`; frontend coverage configuration presence not exhaustively confirmed. CI workflow exists at `.github/workflows/ci-my-cms.yml`.

### Code Quality Indicators
- **Linting**: ESLint and Prettier scripts in frontend manifests; Rust formatting/lint conventions use Cargo toolchain. Exact lint CI commands require workflow inspection.
- **CI/CD**: `.github/workflows/ci-my-cms.yml`, three image release workflows and `publish-github-release-images.yml`; archived workflow folder also exists. Deployment manifests under `deployments/`.
- **Documentation**: Root README, architecture/features/development docs, and maintained backup/restore feature spec are present. Documentation narratives include intended behavior; source code remains authority for current implementation.

### Technical Debt Signals
- The checkout contains substantial unrelated pending changes and untracked files, including gateway changes, `backup_cli.rs`, Cargo.lock/Cargo manifest edits and documentation. These are user work; do not overwrite. The intent-relevant `docs/features/backup-restore.md` and corresponding CLI file are untracked, and backup/restore contracts should be reviewed against source and tests by architect/next stage.
- Existing CodeKB was reported STALE. This handoff is a developer scan, not a replacement for nine CodeKB artifacts; architect synthesis is next link.
- Full individual source-file review of the 437 counted Rust/TS/SQL files was not feasible in this pass; Scan Coverage explicitly distinguishes inventory/deep investigation from areas only skimmed. Future claims of exact API/route count, comprehensive dependency graph, or code-quality rates require targeted source review.
- No test, build, app, or project executable was run.

## Handoff Summary
- **Intent-relevant finding**: The change sits in the existing Rust gateway and its backup CLI surface (`apps/api/gateway/src/main.rs`, `apps/api/gateway/src/backup_cli.rs`) with API workspace dependency changes (`apps/api/gateway/Cargo.toml`, `apps/api/Cargo.lock`); the human-authored requested contract is `docs/features/backup-restore.md`. Read the feature narrative and CLI implementation together, then trace database/storage integration, safety/authorization, failure semantics and tests before making behavioral claims.
- **Risks / follow-up**: Snapshot head commit: `22c668809535f0cbf5c297564e25524f721242e5` (2026-11-17). Worktree has pending/untracked user changes; preserve them. No programs/tests/builds run. This is mixed-depth repository coverage, not exhaustive per-file analysis; avoid asserting uninspected endpoint counts or implementation details. The nine shared CodeKB artifacts were not modified.
