---
id: curate
name: Curate
order: 5
agents:
  - outcome-curator
  - knowledge-steward
stages:
  - outcome-review
  - knowledge-maintenance
gate: closure
---

# Curate

Capture outcomes, maintain durable project knowledge, and identify follow-up improvements.

## Entry

- Launch exit criteria are met and operational ownership is accepted.

## Exit

- Decision and outcome records are curated with owners and retention.
- Lessons learned and follow-up items are reviewed with the project owner.
- The project owner approves closure, or explicitly keeps the work open.

On rejection, keep the project open and revise the closeout record or follow-up plan.

## Stages

1. **outcome-review** — `outcome-curator`. Produces an outcome summary, lessons learned, and a follow-up register.
2. **knowledge-maintenance** — `knowledge-steward`. Produces a curated project record, retention and access notes, and a closure recommendation. This stage is the phase gate.

## Activities

- Compare the approved success measures with the observed outcome.
- Write lessons, follow-up items, and owners.
- Update the durable project record, including retention and access notes.
- Feed a new rule or backlog item back to Analyze when the outcome changes how later work should start.

## Questions

- What happened compared with what was intended?
- Which follow-ups have owners, and which risks remain open?
- Is the project owner closing the intent or keeping it open?

## Tracking

Closure is recorded on the intent `state.json`. A rejected closure leaves the intent active.
