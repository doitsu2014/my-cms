# dtech-aidlc for Claude Code

This project uses dtech-aidlc, an AI-Driven Development Life Cycle workflow engine. The CLI selects the next stage and owns workflow state. Claude performs only the directed work and preserves the required human phase approvals.

## Start

Describe the work after invoking `/aidlc`, for example:

```text
/aidlc Build a REST API for inventory management
```

On first use, `/aidlc` asks for a workflow scope (`solo`, `team`, or `enterprise`) if one is not configured, initializes the project, then starts the requested intent. Use `/aidlc` to continue an active intent. Check progress with `dtech-aidlc status`; validate setup with `dtech-aidlc doctor`.

## Human decisions

Review each phase's artifacts at the paths shown by the orchestrator. Every phase has an explicit approval gate. The workflow pauses for your decision; approval is never inferred from silence, tool output, or a model recommendation. Requested changes are revised and checked before the gate is presented again.

## Tool availability

Claude may use Claude Code's `Workflow` tool for independent stage work only when it is available in the current session and authorized. Otherwise it executes directly with available tools; this setup does not imply a tool is available.

## Installed files

- `.claude/skills/aidlc/SKILL.md` — `/aidlc` orchestrator instructions
- `.claude/tools/` — dtech-aidlc CLI
- `.aidlc/methodology/` — project-local copy of the human-readable methodology; treat as read-only
- The CLI's `core/data/phase-graph.json` is its machine-readable workflow index; use `dtech-aidlc scope` and `dtech-aidlc doctor` to inspect it.
- `.aidlc/` — project configuration, state, audit log, and generated artifacts
