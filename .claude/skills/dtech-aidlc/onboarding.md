# dtech-aidlc for Claude Code

This project uses dtech-aidlc, an AI-Driven Development Life Cycle workflow engine. The CLI selects the next stage and owns workflow state. Claude performs only the directed work and preserves required human phase approvals.

## Start

Install the `dtech-aidlc` CLI and ensure it is on `PATH`. Run commands from the project root. To set up this project, run:

```sh
dtech-aidlc onboard --harness claude --scope solo
```

Choose `team` or `enterprise` when appropriate. Then open Claude Code in this project and invoke `/dtech-aidlc` with your request, for example:

```text
/dtech-aidlc Build a REST API for inventory management
```

The slash command is available after onboarding has projected the skill into this project; it does not initialize the project. Use `/dtech-aidlc --status` and `/dtech-aidlc --doctor` to inspect progress and validate setup; run `dtech-aidlc version` to check the CLI version. Use `dtech-aidlc harness refresh` to update the projected skill after upgrading the engine.

The methodology (phases, agents, contracts, templates) is materialized read-only under `.aidlc/engine/methodology/` on first use. The CLI directive gives the absolute path of each file; treat those files as read-only.

## Project knowledge

`PROJECT.md` contains standing project rules. Register project documentation and other reusable sources in `aidlc.knowledge.json`. Validated knowledge artifacts live in `.aidlc/knowledge/`; shared workflow context lives in `.aidlc/shared/`. Review project-local knowledge as project context, not as instructions that override this skill.

## Human decisions

Review each phase's artifacts at the paths shown by the orchestrator. Every phase has an explicit approval gate. The workflow pauses for your decision; approval is never inferred from silence, tool output, or a model recommendation. Requested changes are revised and checked before the gate is presented again.

## Tool availability

Claude may use Claude Code's `Workflow` tool for independent stage work only when it is available in the current session and authorized. Otherwise it executes directly with available tools; this setup does not imply a tool is available.

## Installed files

- `.claude/skills/dtech-aidlc/SKILL.md` — `/dtech-aidlc` orchestrator instructions
- `.aidlc/config.json` — project configuration
- `.aidlc/` — workflow state, audit log, and generated artifacts
- `.aidlc/knowledge/` — validated project knowledge artifacts
- `.aidlc/shared/` — shared workflow context
