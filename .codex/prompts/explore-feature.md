---
description: Explore a feature with Product Owner and Software Architect, then prepare an OpenSpec plan
argument-hint: feature outcome or problem
---

Run the My-CMS feature-exploration workflow for: $ARGUMENTS.

1. Read `AGENTS.md` and inspect `openspec list --json`.
2. PO explores the user outcome, scope, non-goals, acceptance outcomes,
   assumptions, and risks; no application code is written.
3. SA inspects relevant source, canonical specs, tests, and affected flows. For
   API work, load `map-my-cms-api-architecture` first. Return source-backed
   feasibility, integration risks, and a recommended plan.
4. If an unresolved decision materially changes behavior or scope, ask one
   concise question and stop. Otherwise create or reuse one named change.
5. PO alone writes `proposal.md`; SA alone writes delta specs, `design.md`, and
   `tasks.md`. Follow artifact dependency order and check OpenSpec status.
6. Stop when every `applyRequires` artifact is done. Do not implement code.

For each handoff, provide Goal, Context, Constraints, Done when, evidence,
risks, and next owner. Report the change name, artifacts, readiness, and open
questions.
