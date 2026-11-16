# Agent workflow

`AGENTS.md` is the project contract. This page is only a map.

Claude Code with AWS AI-DLC is the only harness. The only agent definitions
are `.claude/agents/aidlc-*`. Do not add product-owner, designer, architect,
engineer, or release-engineer duplicates, and do not restore Codex or
OpenCode configs.

Structured work starts with `/aidlc`. Rules are
`aidlc/spaces/default/memory/`. Current product and engineering contracts are
`aidlc/spaces/default/knowledge/aidlc-shared/`. Explanatory docs stay in
`docs/`.

## Delivery rules that stay

- Exploration can run in parallel and stays read-only until an intent artifact
  is assigned to one writer.
- One writer per shared source file or intent artifact.
- Behavioral changes use RED–GREEN–REFACTOR. A task is done only after its
  check passes.
- Handoffs state goal, context, constraints, done-when, evidence, decisions,
  risks, and the next owner.
- Changes that affect `deployments/`, runtime configuration, or an operational
  contract include rollout, a rollback trigger, and post-deploy checks.
  Environment-changing commands need explicit user or release authorization.
- If the code-review graph is down, record that and use search, `git diff`,
  and tests.

The verification gate is in `AGENTS.md`.

## Fast fix

A fast fix is one non-generated file, at most 40 non-generated lines, no
behavior or operator change, and no auth, API, schema, dependency,
deployment, secret, or generated edit, plus a focused check. Anything else
goes through `/aidlc`.
