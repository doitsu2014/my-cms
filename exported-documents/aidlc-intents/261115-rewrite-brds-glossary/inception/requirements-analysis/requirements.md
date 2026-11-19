# Requirements

## Sources and Evidence Boundaries

- Initial request: rewrite all project BRDs in the shared knowledge folder and provide a glossary. [desc]
- The approved intent in `aidlc/spaces/default/intents/261115-rewrite-brds-glossary/ideation/intent-capture/intent-statement.md` specifies rewriting the seven shared project-contract files, reconciling them against source code and maintained documentation, keeping them concise and testable, and adding a project/domain glossary. [intent]
- Current source code is authoritative for statements about implemented behavior. Shared project knowledge describes intended contracts; maintained documentation is supporting context. Conflicts that cannot be resolved from source must be recorded, not silently reconciled. [Q1]
- The current CodeKB is explicitly partial. Statements not verified by the available source scan must be labeled unverified/open and cannot be represented as confirmed behavior. [Q3] [codekb]
- The glossary is a standalone `glossary.md` under `aidlc/spaces/default/knowledge/aidlc-shared/`, limited to project/domain terms, abbreviations, and product-specific names. [Q2] [intent]

## Functional Requirements

### Contract Inventory and Reconciliation

**FR1 — Rewrite the complete contract set.** Rewrite all seven existing shared project-contract files: `admin-editor.md`, `auth-users-media.md`, `backend-domain-architecture.md`, `deployment-operations.md`, `posts-content-and-translation.md`, `public-site.md`, and `spec-migration-index.md`. Preserve these as the project's shared requirements location; do not create a parallel requirements repository. [intent] [spec-migration-index.md]

- **FR1.1 — Preserve contract coverage.** The rewritten files collectively retain the project behavior, constraints, ownership boundaries, integrations, and superseded decisions represented by the current contracts, while removing duplication and resolving statements that contradict verified current source. [intent] [spec-migration-index.md]
- **FR1.2 — Keep contracts concise and testable.** Express behavior as clear rules or conditions with observable outcomes where the source and available evidence support them; avoid vague goals and unsupported claims. [intent] [Q1] [requirements-guide.md]
- **FR1.3 — Reconcile each contract to its evidence.** Use source code as authority for implemented behavior; use shared project knowledge as the intended contract and maintained docs as supporting context. Preserve discrepancies that source cannot settle as explicit conflicts or open questions. [Q1] [spec-migration-index.md]
- **FR1.4 — Mark uncertain statements.** Any statement not verified by source or sufficient maintained documentation is labeled `Unverified` or recorded under an open question, and is not presented as confirmed current behavior. [Q3] [codekb]
- **FR1.5 — Preserve source traceability.** Each contract identifies the applicable source paths and maintained documentation needed to validate its behavior, updating stale or inaccurate references during rewriting. [intent] [spec-migration-index.md]
- **FR1.6 — Maintain the seven-file inventory index.** Update `spec-migration-index.md` to accurately describe the rewritten contract files, their coverage and maintained-document relationships, and the applicable source precedence and evidence limitations. [intent] [spec-migration-index.md]

### Glossary

**FR2 — Add a shared project glossary.** Create `aidlc/spaces/default/knowledge/aidlc-shared/glossary.md` as a standalone glossary for project/domain terms, abbreviations, and product-specific names used in the seven contract files. [intent] [Q2]

- **FR2.1 — Define terms in project context.** Each glossary entry gives the expansion, when applicable, and a concise project-specific definition grounded in the contracts or source; omit unsupported expansions and mark unresolved terms as open. [Q2] [Q3]
- **FR2.2 — Link the glossary from contracts.** Add a concise reference to `glossary.md` from the rewritten project-contract files where terminology needs shared interpretation. [Q2]
- **FR2.3 — Exclude methodology glossary terms.** Do not use the project glossary to define AI-DLC workflow or process terminology. [intent]

### Verification

**FR3 — Verify completeness and consistency.** Before considering the rewrite complete, check all seven files and the glossary against the confirmed inventory, their source references, cross-file terminology, and the evidence boundary. [intent] [Q1] [Q2] [Q3]

- **FR3.1 — Verify file coverage.** Confirm all seven identified contract files were rewritten and the standalone glossary exists at the specified path. [intent] [Q2]
- **FR3.2 — Verify cross-document consistency.** Check that shared terms, links, system ownership, documented behavior, and source precedence do not conflict across the seven contracts and glossary. [intent] [spec-migration-index.md]
- **FR3.3 — Verify uncertainty labels.** Confirm unverified behavior and unresolved source/document conflicts are explicitly labeled and not stated as established current behavior. [Q1] [Q3]
- **FR3.4 — Verify no accidental process duplication.** Confirm that rewriting leaves project requirements in the shared knowledge folder and does not recreate OpenSpec or another requirements repository. [spec-migration-index.md]

## Non-Functional Requirements

**NFR1 — Evidence fidelity.** Every statement asserting current implementation behavior must be supported by source that was inspected or by a maintained document whose claim was checked against source; unsupported or conflicting claims are explicitly qualified. [Q1] [Q3] [codekb]

**NFR2 — Readability and maintainability.** The seven contracts and glossary must use consistent terminology, clear headings, concise prose, and links rather than repeated definitions; requirements remain understandable to project contributors. [intent] [Q2]

**NFR3 — No false verification claims.** The documents must not imply test, build, lint, runtime, or exhaustive source validation occurred unless that validation is actually performed and recorded. [codekb]

## Constraints

- The seven contract files and glossary remain in `aidlc/spaces/default/knowledge/aidlc-shared/`. [intent] [Q2] [spec-migration-index.md]
- Source code governs descriptions of current implemented behavior; shared project knowledge describes intended contracts and maintained docs support them. Unresolved discrepancies must remain visible. [Q1]
- The available CodeKB scan is partial and static. Do not treat its skimmed paths as deeply verified; no tests, builds, or programs were run during that scan. [codekb]
- The glossary covers product/project terminology, not AI-DLC workflow vocabulary. [intent]

## Assumptions

- The existing seven contract documents in the shared knowledge directory are the complete set meant by “all BRDs”; this is the user-confirmed inventory. [intent]
- The repository's existing package boundaries and maintained docs remain useful evidence locations, subject to source verification during the rewrite. [codekb] [spec-migration-index.md]

## Out of Scope

- Adding or changing application code, APIs, infrastructure, or runtime behavior. [intent]
- Recreating OpenSpec, archived change folders, or another project-requirements system. [spec-migration-index.md]
- Adding AI-DLC workflow terms to the project glossary. [intent]
- Claiming exhaustive verification of source paths not deeply examined, or claiming tests/builds passed when none were run. [codekb]

## Open Questions

- Which additional source paths need deeper inspection to resolve any contract statements that fall outside the existing partial CodeKB coverage? [codekb]
- For any conflict between current source and intended project contract that cannot be resolved from source or maintained documentation, what decision should the project owner make? [Q1]
