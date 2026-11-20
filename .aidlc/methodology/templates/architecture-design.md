# Architecture design

Use this for the Ideate output `architecture-design`. Follow the arc42 structure. Use C4 names for the context and building-block views: System Context, Container, and Component. Record each significant decision as an ADR with the headings Context, Decision, and Consequences.

```markdown
# Architecture design

## 1. Introduction and goals
- Purpose:
- Quality goals:
- Stakeholders:

## 2. Constraints
- Technical:
- Organizational:
- Convention:

## 3. Context and scope
- System context:
- External systems:
- Out of scope:

## 4. Solution strategy
- Chosen approach:
- Rejected alternatives:

## 5. Building block view
- Containers:
- Components:
- Responsibilities:

## 6. Runtime view
- Main scenario:
- Failure scenario:

## 7. Deployment view
- Environments:
- Runtime nodes:

## 8. Cross-cutting concepts
- Security:
- Data:
- Observability:

## 9. Architecture decisions
### ADR-001 <title>
- Status: proposed | accepted | superseded
- Context:
- Decision:
- Consequences:

## 10. Quality requirements
- Scenario:
- Measure:

## 11. Risks and technical debt

## 12. Glossary
```

Rules:

- Describe the approved direction only. Do not reopen an unapproved option as if it were chosen.
- A box in a diagram needs a responsibility and an interface. A picture alone is not the design.
- Mark any section that does not apply as "not applicable" and say why.
