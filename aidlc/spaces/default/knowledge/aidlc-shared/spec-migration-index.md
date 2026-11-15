# Project contract index

This directory is the shared project requirements location. It is not a duplicate of `docs/`, and must not be replaced by another requirements system. The seven contracts and standalone [glossary](glossary.md) are maintained here.

## Evidence and authority

For claims about current implementation, current source code is authoritative. Shared knowledge records intended contract and decisions; maintained documentation is supporting context. When source is only partially inspected, mark behavior **Unverified** and cite the evidence boundary. If source and intended contract conflict and source cannot settle policy, record both claims, references, and that project-owner decision is required. A documentation claim alone does not establish implementation.

The available CodeKB is a partial static scan dated 2026-11-15. Deep paths and shallow paths are listed in `aidlc/spaces/default/codekb/my-cms/reverse-engineering-timestamp.md`; no tests, builds, or programs were run for that scan. Its listed areas must not be treated as exhaustively verified.

## Contract map and coverage disposition

The current seven files are rewritten from their existing capability/requirement statements. This map provides a checkable destination for each document's contract coverage. The former canonical inventory is known only as **24 capabilities and about 193 requirements**; its individual source rows are not present in the available record. Accordingly, this rewrite does not invent a 193-row inventory or claim exact item-by-item reconciliation. Historical granularity and exact counts remain an explicit coverage gap pending recovery of the source inventory.

| Existing contract / capability group | Destination | Disposition |
|---|---|---|
| Gateway composition, domain ownership, mount/role model, migrations, extension boundaries, OTLP | `backend-domain-architecture.md` | Retained; detailed current behavior outside inspected evidence labeled Unverified. |
| Supabase authentication, admin users, media/buckets, image transformation | `auth-users-media.md` | Retained as intended contract; unsupported implementation details labeled Unverified. |
| Posts ownership, GraphQL, translation lookup/validation/embeddings, admin refresh | `posts-content-and-translation.md` | Retained; source/maintained-doc polling conflict called out as Unresolved. |
| Shared prose rendering, Quick Toolbar, images, Mermaid | `admin-editor.md` | Retained as intended contract; details not confirmed by partial scan marked Unverified. |
| Localized reader, states, metadata/SEO, public SEO assets, sitemap/robots | `public-site.md` | Retained as intended contract; source behavior marked Unverified where uninspected. |
| Local stacks, health, telemetry, image contexts, Docker Hub/release publishing | `deployment-operations.md` | Retained; current config/runtime claims marked Unverified. |
| Seven-file inventory, evidence precedence, active-change map, superseded/archived limits | `spec-migration-index.md` | Retained and updated. |
| Project/domain names and abbreviations used across contracts | `glossary.md` | Added; AI-DLC workflow terms excluded. |

### Historical capability inventory limits

- The existing index states the former canonical inventory contained 24 capabilities and about 193 requirements; it does not include those individual rows. There is therefore no honest way to assign exactly one destination or superseded/duplicate disposition to each historical item from the available files.
- Current seven contract files were individually read and mapped by document and topic above. Their complete original itemized inventories were not separately preserved as stable IDs, so the table is a document-level coverage map, not a fabricated line-by-line checklist.
- No unmapped *available contract file* remains. Exact item-level historical coverage is **Unverified / open**, not represented as complete.
- The historical index says 38 archived changes were not copied. They remain excluded; the tag-helper ownership note is retained because the current index explicitly preserves it. No archived requirement is newly revived.

## Maintained documentation

- `docs/architecture/software-architecture.md` — gateway and domain ownership; supporting narrative.
- `docs/features/ai-translation.md` — translation behavior; older polling language conflicts with the current intended no-polling admin requirement and is not treated as current implementation proof.
- `docs/features/editor-prose-contract.md` — shared article prose boundary.
- `README.md` and `deployments/docker-swarm/README.md` — local environment guidance.

These files support validation but do not replace contracts or source evidence.

## Prior capability/change map

| Existing index topic / change | Primary destination |
|---|---|
| `api-gateway-bootstrap`, `domain-api-cutover`, `domain-service-interface`, `domain-post-service`, ownership portion of `domain-posts`, `legacy-cms-purge`, GraphQL mount ownership | `backend-domain-architecture.md` |
| `domain-auth-service`, `supabase-auth`, `supabase-storage`, `media-bucket-management`, `image-transformation`, `user-management` | `auth-users-media.md` |
| `domain-posts` behavior, `pgvector-vector-search`, post GraphQL schema, translation-refresh delta | `posts-content-and-translation.md` |
| `editor-prose-package`, Quick Toolbar, image insertion, Mermaid authoring | `admin-editor.md` |
| Website shell/frontend/reader/design/content states/about/metadata/SEO assets/sitemap | `public-site.md` |
| Local development, website deployment, healthz, OTLP, image contexts, Docker Hub publishing | `deployment-operations.md` |
| Workflow procedure | Not copied as product requirement; durable process rules are outside this contract set. |

## Superseded decisions retained for prevention

- Do not restore a standalone posts binary, `application_core`, a separate migration crate, legacy `apps/api/src` CMS package, or `legacy_bootstrap`; ship `my-cms-api` through the gateway and keep post migrations in `domain_posts`.
- Do not reduce the gateway domain manifest to four; existing contract names five domains, including `domain_seo`.
- Do not restore Keycloak tokens; intended auth provider is Supabase GoTrue JWT.
- Do not restore server-side image decoding or redirect browsers to an internal Supabase render URL; prior contract specifies proxying transformed bytes, though current implementation remains to verify.
- Do not revive retired root Compose filenames/env files; local stacks reside under `deployments/docker-swarm/`.
- Website probe is intended as `/healthz`, not `/en`.
- Do not add `.github/workflows/validate-app-image-builds.yml`; project contract rejects a replacement PR image-validation workflow.
- `domain_auth/src/main.rs` is described as a stub, not a deployment surface; verify before relying on current source.

## Glossary and terminology

Every contract links to `glossary.md`. It defines project-specific terms only, not AI-DLC workflow vocabulary. Terms are grounded in this contract set or explicitly unresolved; do not infer unsupported expansions.
