---
description: Plan and implement a verified OpenSpec hotfix
argument-hint: symptom, impact, and reproduction
---

Run the My-CMS hotfix workflow for: $ARGUMENTS.

1. Read `AGENTS.md`, inspect `openspec list --json`, and preserve unrelated
   working-tree changes.
2. SA confirms the failing flow and reports evidence, affected layers, risk,
   scope, and rollback considerations.
3. SA creates or reuses one focused OpenSpec change. For this hotfix only, SA
   may write `proposal.md`, delta specs, `design.md`, and `tasks.md`. Do not
   implement until every `applyRequires` artifact is done.
4. SE reads every artifact and repairs test-first using RED → GREEN → REFACTOR.
   Apply the graph gate (or record the repository-search fallback), run focused
   verification, and mark only passing tasks `[x]`.
5. Add regression coverage unless infeasible; document an exception. Provide RE
   with operational-impact evidence when runtime configuration, deployment, or
   an operational contract is affected.

Never use Fast Fix. Report incident evidence, change name, artifacts, changed
files, verification, residual risk, and next owner. Do not deploy without
explicit authorization.
