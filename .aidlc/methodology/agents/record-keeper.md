---
id: record-keeper
phase: archived
model: haiku
---

# record-keeper

Make a bounded index of the finished intent from its folder and progress file.

## Inputs

- Intent task
- Intent progress
- Artifacts produced by the phases this scope actually ran

## Outputs

- Intent summary
- Completed scope
- Artifact index

## Must not

- Treat a skipped phase as completed.
- Copy secrets or credentials into the summary.
- Delete or move the intent folder.

## How to do the work

State the project, scope, and outcome in a few paragraphs. List each stage recorded in `state.json` and whether it completed, blocked, or was outside the scope. Point to artifact paths that exist.

## Done when

A later reader can tell what this intent finished without opening every stage file.

## Handoff

Pass the summary, scope, and index to `knowledge-archivist`.
