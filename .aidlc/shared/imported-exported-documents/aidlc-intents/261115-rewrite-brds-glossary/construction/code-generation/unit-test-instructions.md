# Verification Instructions

## Scope

This documentation-only change updates seven Markdown contracts and adds one Markdown glossary. There is no application source code change, package change, schema migration, or runtime behavior change.

## Test and Verification Method

Test-after applies: make the documentation changes, then run the narrowest relevant checks. Minimal strategy obligations are one verifiable check per requirement and a happy-path check; for this deliverable, the document coverage/consistency checklist is the requirement-level check and a Markdown whitespace/diff check is the happy path. No additional POC test floor applies.

## Commands

Run from the repository root after the documentation edits:

- `git diff --check` — verify no whitespace errors in tracked changes.
- Review the seven-file coverage checklist recorded in the Code Generation plan and confirm each inventory entry maps to a destination section or a documented superseded/duplicate disposition.
- Verify each of the seven contract files contains a working link to `glossary.md`; verify glossary entries are alphabetized and each is either referenced in at least one contract or explicitly marked as foundational.
- Review the final diff for unresolved statements that should be marked `Unverified` or `Unresolved`, unsupported claims, broken internal paths, duplicate definitions, and accidental edits outside the nine documentation files.

These checks are documented as the exact verification procedure for the deliverable; only commands actually run will be reported as executed in `code-summary.md`.

## Expected Results

- `git diff --check` exits successfully.
- Every item in the source-to-destination coverage checklist has a recorded disposition.
- All seven contract files link to the glossary and all glossary entries satisfy the stated inclusion rule.
- No generated or application-code changes appear in the final diff.

## Tests Not Applicable

No unit, integration, API, database, or UI tests are planned because no application code changes are in scope. If the rewrite exposes a source behavior that needs executable verification, mark it as unverified/open rather than claiming a test passed.
