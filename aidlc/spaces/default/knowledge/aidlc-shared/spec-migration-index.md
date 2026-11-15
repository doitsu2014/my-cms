# Spec migration index

Migrated on 2026-11-15 from the removed OpenSpec tree into this knowledge
directory. These files are the project contract AI-DLC agents load from
`aidlc/spaces/default/knowledge/aidlc-shared/`. They are not a second copy of
`docs/`. When narrative and contract disagree, prefer source, then this
knowledge, then `docs/`.

OpenSpec was removed on purpose. Do not recreate `openspec/`, and do not treat
archived change folders as a live process.

## How to read these files

Requirements below are the repository contract after reconciling canonical
specs with unsynced active changes and with source. They are not a claim that
every scenario has a passing test in this worktree. Validate against the
source paths named in each file.

## Maintained docs (do not duplicate)

- `docs/architecture/software-architecture.md` — gateway, domains, mounts, data.
- `docs/features/ai-translation.md` — translation jobs, reuse, paragraph checks.
- `docs/features/editor-prose-contract.md` — shared `.article-prose` rendering.
- `README.md` and `deployments/docker-swarm/README.md` — local stacks and ports.

## Knowledge map

| File | Replaces |
| --- | --- |
| `backend-domain-architecture.md` | `api-gateway-bootstrap`, `domain-api-cutover`, `domain-service-interface`, `domain-post-service`, `domain-posts` (ownership only), `legacy-cms-purge`, `posts-graphql-mount` (mount ownership) |
| `auth-users-media.md` | `domain-auth-service`, `supabase-auth`, `supabase-storage`, `media-bucket-management`, `image-transformation`, `user-management` |
| `posts-content-and-translation.md` | `domain-posts` behavior, `pgvector-vector-search`, `posts-graphql-mount` schema contract, translation-refresh delta |
| `admin-editor.md` | `editor-prose-package`, Quick Toolbar, image insertion, Mermaid authoring |
| `public-site.md` | website shell, frontend, reader, design system, content states, about, metadata, SEO head assets, sitemap |
| `deployment-operations.md` | `local-dev-environment`, `website-deployment`, healthz, OTLP, image build contexts, Docker Hub publishing |

`agent-workflow-governance` was procedure. Durable rules (one writer, evidence
before handoff, no production change without explicit authorization) live in
root `AGENTS.md`. The only agent definitions are `.claude/agents/aidlc-*`.

## Active changes folded in (2026-11-15)

These were unsynced. Their unique requirements are in the files above. Task
checkboxes were not treated as implementation proof.

- `add-admin-editor-quick-toolbar`
- `add-mermaid-diagram-support` (Mermaid, prose, reader, translation refresh)
- `preserve-post-detail-thumbnail-ratio`
- `add-seo-head-asset-management`
- `standardize-website-and-admin-metadata`
- `add-website-sitemap-and-crawler-directives`
- `add-website-healthz-probe`
- `add-website-otlp-tracing`
- `restore-gateway-otlp-tracing`
- `fix-app-image-build-contexts`
- `automate-docker-hub-image-publishing`

## Superseded — do not reintroduce

- Standalone posts binary and `cargo run -p domain_posts` as an HTTP or
  migration deployment. The only shipped API binary is gateway `my-cms-api`.
- `application_core` crate and any re-export shim. The directory is gone.
  `domain_posts::handlers::tag_helper` is the canonical tag implementation;
  the old `application_core` tag commands were duplicates and were deleted,
  not relocated.
- Legacy `apps/api/src` `cms` package, `legacy_bootstrap`, and a separate
  `migration` crate. Post migrations stay in `domain_posts`. Operators use
  `my-cms-api migrate`.
- Gateway manifest of exactly four domains. Source registers five:
  posts, auth, media, user, and `domain_seo`.
- Keycloak tokens. Auth is Supabase GoTrue JWTs.
- Server-side image resizing with the `image` crate, and redirecting browsers
  to an internal Supabase render URL. Resized requests proxy transformed bytes.
- Root Compose files named `docker-compose.supabase.yaml` /
  `docker-compose.my-cms.yaml` and root env files `.env.supabase` /
  `.env.my-cms`. Local stacks live under `deployments/docker-swarm/`.
- Using `GET /en` as the website container health check. The contract is
  `GET /healthz`.
- A pull-request workflow `.github/workflows/validate-app-image-builds.yml`.
  It is not in the tree. The later publishing contract forbids a replacement
  pull-request image-validation workflow. Bounded contexts remain required;
  publishers and a local workflow-contract check carry that verification.

## Known source leftovers (not a license to revive them)

- `apps/api/domain_auth/src/main.rs` is a stub that prints that a standalone
  auth binary is not implemented. It is not a deployment surface. Do not grow
  it into a second API.
- Comments that still say "transition" or "standalone binary" are historical.
  Update them toward `docs/architecture/software-architecture.md` when touched.

## Canonical inventory that was condensed

24 capabilities, about 193 requirements. Archived changes (38) were not copied.
The only archived decision retained on its own is the tag-helper ownership
note above.
