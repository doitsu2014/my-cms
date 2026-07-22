---
name: openspec
description: Use the repository's OpenSpec lifecycle for requirements, proposals, design, implementation, verification, synchronization, and archival.
compatibility: Requires the OpenSpec CLI (`openspec`).
---

# OpenSpec workflow for Codex

OpenSpec owns the repository's versioned requirements and decision artifacts.
Use this skill whenever a task is a new feature, a non-trivial fix, a design
change, or an implementation of an existing OpenSpec change.

## Lifecycle

1. Explore requirements without implementing code. Read `openspec/specs/`, run
   `openspec list --json`, and inspect the relevant code paths.
2. Create or select a change under `openspec/changes/<name>/`.
3. Produce artifacts in dependency order:
   - `proposal.md`: Why, What Changes, Capabilities, and Impact.
   - `specs/<capability>/spec.md`: `### Requirement` and
     `#### Scenario` blocks using WHEN/THEN/AND language.
   - `design.md`: Context, Goals/Non-Goals, Decisions, architecture, and
     contracts.
   - `tasks.md`: numbered, independently verifiable `- [ ]` tasks with exact
     paths and verification steps.
4. Run `openspec status --change "<name>" --json` and stop artifact creation
   when every `applyRequires` artifact is `done`.
5. Implement tasks only after the change is ready. Read all four artifacts,
   use TDD for behavior changes, and mark each task `[x]` immediately after it
   passes verification.
6. Before completion, run `openspec verify --change "<name>"`, resolve
   critical findings, run the repository verification gate, then run
   `openspec sync --change "<name>"` and `openspec archive "<name>"` when the
   user has asked to finalize the change.

## Commands

```text
openspec list --json
openspec new change "<kebab-case-name>"
openspec status --change "<name>" --json
openspec instructions <artifact> --change "<name>" --json
openspec verify --change "<name>"
openspec sync --change "<name>"
openspec archive "<name>"
```

For a fast, complete artifact set, use `openspec-propose` semantics: create
the change and generate proposal, specs, design, and tasks in one pass. For a
reviewable flow, use `openspec-continue-change` semantics and create one
artifact at a time.

## Codex interaction rules

- If the change name is ambiguous, inspect `openspec list --json` and ask the
  user to select one; do not guess a change.
- Use `update_plan` to track multi-artifact or multi-task work.
- Ask a direct plain-text question when a product or architecture decision
  cannot be safely inferred.
- Do not create new OpenSpec artifacts under `docs/superpowers/`.
- The Software Architect (SA) owns specs, design, and tasks. The Software
  Engineer (SE) owns implementation and verification.
- SA and SE must use the `code-review-graph` MCP gates defined in `AGENTS.md`.
- Keep business logic out of API handlers and follow the repository's layered
  Rust and React conventions.

## Completion handoff

After implementation, verify completeness, correctness, and coherence against
the OpenSpec artifacts. Sync delta specs into `openspec/specs/` before archive.
Do not claim completion when required tasks, verification, or archive steps are
still outstanding.
