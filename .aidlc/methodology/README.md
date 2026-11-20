# AIDLC methodology

This directory defines a human-governed, phase-based workflow for taking a project from an initial request through delivery and learning. It is designed for a fresh project: begin with what the user actually asked for, make uncertainty visible, and do not assume prior decisions or artifacts exist.

Human-readable definitions live in [`phases/`](phases/) (one file per phase) and [`agents/`](agents/) (one file per agent). [`../data/phase-graph.json`](../data/phase-graph.json) is the machine index (schema 1.1.0): phase order, stage routing, gate IDs, outputs, scope membership, model aliases, and file paths. Phase purpose and entry/exit criteria, as well as agent responsibilities, stay in the Markdown files. This document explains how to use that graph and the contracts that govern its work.

## Principles

1. **Human ownership of intent and decisions.** Agents prepare analysis, options, plans, and evidence. They do not approve project gates, silently reinterpret the request, or claim a human decision was made.
2. **Evidence before confidence.** Separate observed facts, user-provided statements, assumptions, estimates, and unanswered questions. Trace important claims to their source where possible.
3. **Bounded, reviewable work.** Each stage has named inputs and outputs. Do only approved work, keep changes within the selected scope, and make limitations explicit.
4. **Approval before consequential transitions.** The project owner reviews and explicitly approves every phase gate. No approval means the workflow is paused at that gate.
5. **Traceability and reversibility.** Preserve decisions and verification evidence. Plan rollback before launch; do not perform destructive or externally visible actions without explicit authorization.
6. **Proportionate rigor.** Apply only phases included by the selected scope. A smaller scope is not a claim that omitted lifecycle concerns do not exist; note any important follow-up or residual risk.

## Phase and stage contracts

Phases run in this order: **Analyze → Ideate → Develop → Launch → Curate → Archived**. Solo, Team, and Enterprise all end at Archived. A phase can start only when its entry criteria are satisfied. Its agents produce the artifacts described in the graph's stages, then the project owner reviews the exit criteria and records an explicit decision. Rejection returns the work for revision; it does not permit the workflow to progress by default.

### Analyze

Establish the problem and its context, not a presumed solution. The brief analyst frames the request, stakeholders, and success measures. The context/risk analyst records relevant evidence, constraints, dependencies, assumptions, risks, and open questions. Label unknowns instead of filling gaps with invention.

**Approval gate — analysis baseline.** The project owner confirms that the framing represents their intent and that material uncertainties are visible. A change to the problem framing or success measures after approval requires renewed approval before downstream work relies on it.

### Ideate

Develop a bounded set of feasible, meaningfully distinct options. Describe trade-offs, dependencies, and risks in terms the project owner can assess. Make a recommendation, but do not present it as the selected direction until the owner chooses it. Record both the decision and what is out of scope or deferred.

**Approval gate — solution direction.** The project owner selects or rejects the proposed direction and approves the scope boundary. If rejected, return to option exploration. A material change to the chosen direction or scope returns to this gate.

### Develop

Plan and implement only the approved direction. Define interfaces/contracts and acceptance criteria before relying on implementation claims. Verification reports identify checks actually performed, their results, and any coverage gaps; they do not substitute confidence or code inspection for an unrun test. Keep the implementation aligned to approved scope. Surface deviations for approval rather than making unilateral product decisions.

**Approval gate — release candidate.** The project owner reviews the implementation, verification evidence, limitations, and deployment/rollback notes. Approval authorizes preparation to launch, not necessarily deployment or any external action. Rejected candidates return for correction and re-verification.

### Launch

Prepare a controlled release with a readiness checklist, communications draft, monitoring/support ownership, and a rollback path. State what action is proposed and its impact. Obtain explicit approval for deployment, publication, notifications, or other externally visible, destructive, or irreversible operations; a general phase approval is not blanket authorization for each such action.

**Approval gate — launch and handoff.** Confirm launch evidence, operational acceptance, and any rollback or follow-up required. If readiness is not established, pause or follow the approved rollback plan. Do not state that launch succeeded without observed evidence.

### Curate

Compare intended outcomes with available observations, distinguish correlation from demonstrated cause, retain useful decisions and lessons under the applicable access/retention rules, and assign follow-ups only with clear ownership. Curating does not authorize deleting, archiving, or broadening access to records.

**Approval gate — closure.** The project owner approves the outcome record and closure, or keeps the work open with explicit next steps. Closure is a recorded human decision, not an inference from inactivity.

## Scopes

The graph defines exact phase lists:

- **solo:** Analyze, Ideate, Develop.
- **team:** Analyze, Ideate, Develop, Launch.
- **enterprise:** Analyze, Ideate, Develop, Launch, Curate.

Select the scope with the project owner before execution. These labels determine workflow coverage only; they do not imply a particular team size, risk level, compliance status, or authority to execute actions. If omitted phases leave a material operational, retention, or outcome concern, flag it for explicit owner decision rather than implying it was addressed.

## Agent roster and model recommendations

The graph contains exactly ten agents, two assigned to each phase. Agent responsibility, inputs, outputs, and guardrails are defined in the linked agent Markdown files; the graph stores each agent's phase, model alias, and file path. Model aliases are recommendations for the named work, not claims that a model is available in every environment or that it may bypass policy, permissions, or human review. Use the listed model only where supported and authorized; escalate difficult, consequential, or ambiguous reasoning to a stronger supported model. A model recommendation never changes an approval gate or grants authority to act.

## Approval and recordkeeping protocol

For each gate, record at minimum: gate identifier, decision (approved/rejected/paused), approver identity or role, timestamp, reviewed artifact/version, rationale or requested changes, and any conditions. Approval must be affirmative and specific to the artifact and action under review. Silence, a generated plan, a checkmark, or an agent's own assessment is not approval. If context or scope materially changes after approval, obtain renewed approval for affected decisions.

Use the stage outputs as contracts, not a requirement to create empty paperwork. For small work, a concise record can satisfy multiple outputs if it preserves their substance and provenance. Never claim an artifact, test, approval, deployment, or handoff exists until it is actually produced or observed.

## Fresh-project kickoff

1. Capture the user's request without assuming prior project state.
2. Confirm scope and identify the project owner / approval role.
3. Begin Analyze; ask only the questions needed to resolve material unknowns, recording the rest.
4. Produce the Analyze outputs and pause for the analysis-baseline approval.
5. Continue phase by phase, maintaining links between decisions, contracts, implementation, and evidence.

A request that is clear enough to start does not need an exhaustive interview. Make reasonable, low-impact working assumptions explicit and reversible; ask before committing to consequential assumptions or actions.
