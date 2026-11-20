# Acceptance criteria

Use this for the Analyze output `acceptance-criteria`. Write each criterion in Gherkin, the Given/When/Then form used by BDD and Cucumber. Give every criterion a stable identifier, as in ISO/IEC/IEEE 29148.

```markdown
# Acceptance criteria

## AC-001 <short name>
- Statement: The system shall <observable outcome>.
- Rationale: <why this matters>
- Source: <stakeholder or evidence>

### Scenario: <main success>
- Given <precondition>
- When <action>
- Then <observable result>

### Scenario: <important failure or edge>
- Given <precondition>
- When <action>
- Then <observable result>
```

Rules:

- One behavior per scenario. The Then clause must be observable.
- Do not name an implementation, a framework, or an internal function.
- Keep the identifier stable if the wording changes.
- A criterion without a Then clause is not acceptance criteria.
