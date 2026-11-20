## Sources

- [desc] Initial description: "please rewrite all brds of this project in shared knowledge folder and glossary"
- [Q1] Rewrite all seven shared project-contract files.
- [Q2] Reconcile each contract against source code and maintained docs; keep concise, testable project contracts.
- [Q3] Add a glossary of project/domain terms, abbreviations, and product-specific names used across the seven contracts.
- [scope] Workflow-selected scope: `poc`.
- [intent] `aidlc/spaces/default/intents/261115-rewrite-brds-glossary/ideation/intent-capture/intent-statement.md`
- [codekb] `aidlc/spaces/default/codekb/my-cms/` (partial static analysis; see `reverse-engineering-timestamp.md`)
- [knowledge] `aidlc/spaces/default/knowledge/aidlc-shared/spec-migration-index.md`
- [knowledge] `aidlc/spaces/default/knowledge/aidlc-shared/admin-editor.md`
- [knowledge] `aidlc/spaces/default/knowledge/aidlc-shared/auth-users-media.md`
- [knowledge] `aidlc/spaces/default/knowledge/aidlc-shared/backend-domain-architecture.md`
- [knowledge] `aidlc/spaces/default/knowledge/aidlc-shared/deployment-operations.md`
- [knowledge] `aidlc/spaces/default/knowledge/aidlc-shared/posts-content-and-translation.md`
- [knowledge] `aidlc/spaces/default/knowledge/aidlc-shared/public-site.md`

## Q1. Resolution when sources disagree
The migration index currently says source code takes precedence over shared knowledge, which takes precedence over narrative documentation. You asked to reconcile the BRDs against source code and maintained docs. When these sources disagree, which should govern the rewritten project contract?

A. Use current source code as the behavior authority; use shared knowledge as the intended contract and maintained docs as supporting context, documenting unresolved conflicts
B. Treat maintained docs as authoritative where source behavior appears inconsistent
C. Not yet defined
X. Other (please specify)
[Answer]: A. Use current source code as the behavior authority; use shared knowledge as the intended contract and maintained docs as supporting context, documenting unresolved conflicts

## Q2. Glossary file and contents
The request calls for a glossary in the shared knowledge folder, and Q3 limited it to project/domain terms, abbreviations, and product-specific names. Should the glossary be a standalone file named `glossary.md` at `aidlc/spaces/default/knowledge/aidlc-shared/glossary.md`?

A. Yes; make it a standalone shared project glossary and link to it from the seven rewritten files where useful
B. Put the glossary only in the migration index
C. Not yet defined
X. Other (please specify)
[Answer]: A. Yes; make it a standalone shared project glossary and link to it from the seven rewritten files where useful

## Q3. Handling unsupported or stale statements
The reverse-engineering scan was partial and did not run tests or builds. How should a BRD statement be handled when its current behavior cannot be verified from the scanned source and maintained docs?

A. Keep it clearly labeled as unverified/open; do not present it as confirmed current behavior
B. Remove it from the rewritten contract unless a follow-up source review confirms it
C. Not yet defined
X. Other (please specify)
[Answer]: A. Keep it clearly labeled as unverified/open; do not present it as confirmed current behavior

## Consolidated Summary Confirmation
- Current source code governs descriptions of existing behavior; shared knowledge expresses the intended contract, and maintained docs provide supporting context. Unresolved conflicts are documented.
- Create a standalone `aidlc/spaces/default/knowledge/aidlc-shared/glossary.md` for project/domain terminology and link it from the rewritten contracts where useful.
- Keep statements that cannot be verified clearly labeled as unverified/open; do not present them as confirmed current behavior.

- Looks correct
- Request changes

[Answer]: Looks correct
