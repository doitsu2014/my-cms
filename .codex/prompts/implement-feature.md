---
description: Implement an approved OpenSpec feature with Designer and Software Engineer
argument-hint: active OpenSpec change name
---

Run the My-CMS feature-implementation workflow for change: $ARGUMENTS.

1. Read `AGENTS.md` and every artifact for the named active change. Run
   `openspec status --change "<name>" --json`. If proposal, specs, design, or
   tasks are incomplete, stop and return the change to its next OpenSpec owner.
2. PD inspects the approved plan and current UI, then returns guidance for
   responsive behavior, component anatomy, interaction/loading/error/
   destructive states, and accessibility. PD does not write production code or
   silently change scope.
3. SE maps each pending task to requirements, files, tests, and verification;
   then implements with RED → GREEN → REFACTOR. Mark a task `[x]` only after it
   passes verification.
4. Before edits and after each task group, apply the graph gates. If unavailable,
   record the limitation and use targeted search, `git diff`, and relevant tests.
   Return material plan conflicts to PO/SA.
5. Run targeted checks, then the repository verification gate as applicable:
   `cargo check`, `cargo test`, `cargo fmt -- --check`, `cargo clippy`, and
   `pnpm --dir apps/web build`.

Report completed tasks, changed files, test and verification outcomes,
graph/fallback findings, operational impact, risks, and next owner. Do not
archive or deploy unless explicitly requested and authorized.
