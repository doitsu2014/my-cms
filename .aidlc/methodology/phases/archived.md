---
id: archived
name: Archived
order: 6
agents:
  - record-keeper
  - knowledge-archivist
stages:
  - intent-record
  - knowledge-writeback
gate: knowledge-writeback
---

# Archived

Close the intent and write what the next intent needs into the project knowledge house. Every scope ends here. Solo reaches it after Develop, Team after Launch, and Enterprise after Curate.

## Entry

- The last phase included by the active scope is approved.
- The intent folder contains the task, the progress file, and the artifacts produced so far.

## Exit

- The intent summary says what was done, what was skipped because of scope, and which artifacts exist.
- A knowledge record is written for later Analyze work.
- The project owner approves the write-back.

On rejection, revise the record. Do not mark the intent complete.

## Stages

1. **intent-record** — `record-keeper`. Produces an intent summary, the completed scope, and an artifact index.
2. **knowledge-writeback** — `knowledge-archivist`. Produces the knowledge record and notes for the next intent. This stage is the phase gate.

## Activities

- Read the active intent folder, not another project's records.
- Index only artifacts that exist. Do not invent missing phase outputs when a scope skipped Launch or Curate.
- Write the knowledge record to `.aidlc/knowledge/<intent-id>.md`.
- Leave the source intent folder in place. Archiving copies the lesson forward. It does not delete the task.

## Questions

- What did this scope finish, and what did it explicitly not do?
- Which decisions, limitations, and follow-ups should the next intent see?
- Did the project owner approve the write-back?

## Tracking

`knowledge-writeback` is the last stage of Solo, Team, and Enterprise. The next intent's Analyze phase reads `.aidlc/knowledge/`.
