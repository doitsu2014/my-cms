# Functional design

Use this for the Develop output `functional-design`. Follow the design views in ISO/IEC/IEEE 1016: context, composition, logical behavior, dependency, interface, interaction, state, and data. Each function must trace to an acceptance-criterion identifier.

```markdown
# Functional design

## 1. Purpose and trace
- Approved direction:
- Acceptance criteria covered: AC-001, AC-002

## 2. Context
- Actors:
- In scope:
- Out of scope:

## 3. Functional decomposition
### FN-001 <function name>
- Traces to: AC-001
- Actor:
- Preconditions:
- Main flow:
- Alternate and error flows:
- Postconditions:

## 4. Data
- Entity:
- Fields:
- Validation:
- Retention:

## 5. Interfaces
- Name:
- Input:
- Output:
- Errors:
- Idempotency:

## 6. Interaction
- Caller:
- Callee:
- Sequence:

## 7. State
- States:
- Transitions:
- Forbidden transitions:

## 8. Dependencies
- Dependency:
- Failure behavior:
```

Rules:

- Every function traces to at least one `AC-` identifier. A function with no criterion is out of scope until Analyze adds one.
- Specify behavior and interfaces. Do not turn this document into source code.
- Name the error the actor can observe, not only the internal exception type.
