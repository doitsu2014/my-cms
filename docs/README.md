# My-CMS documentation

This documentation describes the system as it is implemented in this repository.

## Start here

- [Software architecture](architecture/software-architecture.md) — gateway,
  domain boundaries, routing, data, and deployment model.
- [AI translation](features/ai-translation.md) — author workflow, background
  jobs, quality safeguards, and pgvector reuse.
- [Backup and restore](features/backup-restore.md) — admin-managed portable
  backups, ZIP import, instance selection, and restore safety contract.
- [Editor prose contract](features/editor-prose-contract.md) — consistent
  TipTap rendering between the admin and Ducth.dev.
- [Agent workflow](development/agent-team.md) — Claude Code and AI-DLC
  delivery rules.

## Source of truth

Documentation explains intent and navigation. Source code wins when it
disagrees with a doc. Current requirements live in
`aidlc/spaces/default/knowledge/aidlc-shared/`.

- `apps/api/` owns the Rust API and gateway composition.
- `apps/web/` is the authenticated admin application.
- `apps/ducth-dev-website/` is the public reader application.
- `packages/editor-prose/` owns the shared rendered-article contract.
- `AGENTS.md` defines how contributors and coding agents work in this repo.

## Conventions

Keep feature documentation close to user-visible behavior and name source paths
for implementation details. Do not add material under `docs/superpowers/`.
New change records belong in an AI-DLC intent, not a separate spec tree.
