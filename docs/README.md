# My-CMS documentation

This documentation describes the system as it is implemented in this repository.

## Start here

- [Software architecture](architecture/software-architecture.md) — gateway,
  domain boundaries, routing, data, and deployment model.
- [AI translation](features/ai-translation.md) — author workflow, background
  jobs, quality safeguards, and pgvector reuse.
- [Editor prose contract](features/editor-prose-contract.md) — consistent
  TipTap rendering between the admin and Ducth.dev.
- [Agent-team workflow](development/agent-team.md) — OpenSpec and role-based
  delivery workflow.

## Source of truth

Documentation explains intent and navigation; source code and active OpenSpec
change artifacts are authoritative when they disagree.

- `apps/api/` owns the Rust API and gateway composition.
- `apps/web/` is the authenticated admin application.
- `apps/ducth-dev-website/` is the public reader application.
- `packages/editor-prose/` owns the shared rendered-article contract.
- `openspec/` owns requirements, decisions, and implementation tasks.
- `AGENTS.md` defines the required workflow for contributors and coding agents.

## Conventions

Keep feature documentation close to user-visible behavior and name source paths
for implementation details. Do not add new material under the legacy
`docs/superpowers/` path; new change artifacts belong in `openspec/`.
