# Code Generation Plan

## Summary

- Builds: seven revised shared project contracts and one project glossary
- Touches: `aidlc/spaces/default/knowledge/aidlc-shared/`
- Tests: one documentation consistency and whitespace verification pass

## Scope and Evidence

This is zero-Unit documentation work. The implementation changes only the seven contract files in `aidlc/spaces/default/knowledge/aidlc-shared/` and adds `glossary.md` in that same directory. It does not change application code, APIs, configuration, or runtime behavior.

Use current source code as authority for statements about implemented behavior. Treat shared project knowledge as intended contract and maintained docs as supporting context. Record unresolved source/intent discrepancies in the applicable contract as an explicit `Unresolved` item stating the competing claims, available source references, and that a project-owner decision is required; do not choose silently. Mark behavior not verified in the existing partial scan `Unverified`, cite the evidence limit, and leave it open rather than claiming it is current.

Before rewriting, build a source-to-destination coverage checklist from every capability/requirement in the existing seven contracts and the 24-capability/~193-requirement inventory recorded in `spec-migration-index.md`. Map each retained item to its destination section, or record it as superseded/duplicate with rationale. Completion requires every inventory item to have exactly one disposition; no unmapped items remain. Keep the historic 38 archived changes excluded as the index specifies unless a current contract explicitly incorporates one.

Extract glossary candidates by collecting project-specific domain names, acronyms, abbreviations, product terms, and named system components used in the seven rewritten files. Include every candidate whose meaning is not ordinary-language obvious or whose project-specific meaning matters; either define it with source/contract evidence or list it under `## Unresolved Terms`. Each rewritten contract must contain a link to `glossary.md`. A final pass checks every glossary link resolves and every glossary entry is used in at least one contract (or is explicitly identified as a foundational product term).

## Implementation Steps

1. [x] Build an inventory checklist from all seven existing contracts and the capability inventory in `spec-migration-index.md`; map available contract topics to destinations and document the unavailable historical item-level inventory as a limitation.
2. [x] Inspect the available source and maintained documentation; distinguish intended statements, partial-scan evidence, unverified behavior, and unresolved conflicts.
3. [x] Rewrite `backend-domain-architecture.md` and `auth-users-media.md` with concise ownership, API/authentication, persistence, and media contracts; label evidence gaps.
4. [x] Rewrite `posts-content-and-translation.md` and `admin-editor.md`, preserving intended contracts and identifying unverified behavior and the polling discrepancy.
5. [x] Rewrite `public-site.md` and `deployment-operations.md`, preserving intended reader, metadata, SEO, local-operation, deployment, and publishing contracts while marking unverified details.
6. [x] Rewrite `spec-migration-index.md` with the available topic-to-destination map, authority/evidence policy, and explicit historical inventory/archived-change limitations.
7. [x] Create `glossary.md` with alphabetized project terms, evidence references, unresolved terms, and links from all seven contracts.
8. [x] Run `git diff --check`; verify all seven glossary links and review the coverage map, glossary, and evidence-boundary labels. Historical 193-item coverage could not be asserted because individual rows were unavailable.
9. [x] Record changed files, evidence limitations, and verification results in `code-summary.md`; the traceability artifact was generated for this documentation-only work.

## Testing Contract

<!-- AIDLC-TESTING-CONTRACT: BEGIN -->
```json
{
  "version": 1,
  "methodology": "test-after",
  "source": "org",
  "ordering": "implement each applicable testable layer, then write and run that layer's tests.",
  "scope": "poc",
  "test_strategy": "minimal",
  "project_type": "brownfield",
  "applicable_notes": [
    {
      "layer": "org",
      "text": "We treat tests as a first-class deliverable in every Bolt. The specific\nmethodology (TDD, BDD, ATDD, or classic test-after) is affirmed at\npractices-discovery and recorded in `team.md` under this heading with explicit\n`Methodology` and `Ordering` fields; Code Generation resolves those fields\nindependently from coverage, tooling, and scope notes.\n\nWhen no posture has been affirmed, our default per scope is:\n- **Methodology**: test-after\n- **Ordering**: implement each applicable testable layer, then write and run\n  that layer's tests.\n- `mvp`, `enterprise`, `feature`, `infra`, `classic` add an 80% line-coverage\n  floor and CI execution before merge.\n- `bugfix`, `security-patch` add a targeted regression for the specific\n  bug/vulnerability and require the existing suite to remain green.\n- `express` uses the Minimal strategy: requirement-driven unit tests (one per\n  requirement, with a happy-path floor per component); existing tests remain\n  green.\n- `poc`, `refactor`, `workshop` add no extra new-test floor and require the\n  existing suite to remain green.\n\nThe active `Test Strategy` still applies in every scope and determines test\nvolume/types. Scope floors are additive; they never reduce or replace the\nselected strategy.\n\nBuild and Test verifies defined coverage floors and affirmed quality targets;\nthey may not be weakened to make a step pass.\n\nAffirm a stricter posture in `team.md` if the team commits to one."
    }
  ],
  "obligations": {
    "strategy": "minimal",
    "strategy_volume": [
      "One verifiable test per requirement at the narrowest effective level.",
      "At least one happy-path unit test per component.",
      "Unit tests are the default; a bugfix/security scope floor may require an integration or E2E regression when that is the narrowest level that reproduces the defect."
    ],
    "scope_floor": [
      "Keep the existing test suite green.",
      "This scope adds no extra new-test floor beyond the selected test strategy."
    ],
    "combination_rule": "Apply every selected-strategy obligation and every scope-floor obligation; neither replaces the other, and a targeted scope regression may add the narrowest necessary test type beyond the strategy default."
  },
  "plan_profile": {
    "methodology": "test-after",
    "runner_step": "Verify the existing test runner/configuration and record the exact unit-scoped command.",
    "runner_ready_before_first_test": true,
    "testable_layers": [
      "Data model / database behavior",
      "Repository / data access",
      "Business logic",
      "API / endpoint",
      "Frontend behavior"
    ],
    "steps": [
      "Project structure and production configuration skeleton.",
      "Verify the existing test runner/configuration and record the exact unit-scoped command.",
      "Data model / database behavior - implement.",
      "Data model / database behavior - write and run its tests after implementation.",
      "Repository / data access - implement.",
      "Repository / data access - write and run its tests after implementation.",
      "Business logic - implement.",
      "Business logic - write and run its tests after implementation.",
      "API / endpoint - implement.",
      "API / endpoint - write and run its tests after implementation.",
      "Frontend behavior - implement.",
      "Frontend behavior - write and run its tests after implementation.",
      "Environment/build configuration.",
      "Documentation and traceability."
    ]
  },
  "input_sha256": "sha256:ddd208517f68585f816e80821872bd21e26dc15e7bf545c6a78ec96b6afc20da",
  "contract_sha256": "sha256:8780c9b6738d34215859c7d0d02b149a16944cf2942a43d2c2dee67268c80df0"
}
```
<!-- AIDLC-TESTING-CONTRACT: END -->
