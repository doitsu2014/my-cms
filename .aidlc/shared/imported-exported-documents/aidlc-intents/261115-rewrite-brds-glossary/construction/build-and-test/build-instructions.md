# Build Instructions

## Scope

This is a documentation-only deliverable. No application code, package manifest, schema, container image, or runtime configuration was changed. There is nothing to compile, bundle, or deploy for this work.

The build and test inputs are the Code Generation plan at `aidlc/spaces/default/intents/261115-rewrite-brds-glossary/construction/code-generation/code-generation-plan.md`, the commands in its `unit-test-instructions.md`, and the change scope recorded in `code-summary.md`. Those three documents establish that no application build applies here.

## Prerequisites

- Git, with the repository checked out.
- No dependency installation was required for this work.
- No environment variables or local services were needed.

## Build Commands

No application build is applicable to this change. The repository's Rust, frontend, and container builds are unaffected and were not run.

## Verification Commands

Run from the repository root:

- `git diff --check` — checks tracked changes for whitespace errors.

## Expected Result

`git diff --check` exits successfully. Git may print LF-to-CRLF working-copy warnings on Windows; those are not whitespace errors.

## Troubleshooting

- If `git diff --check` reports whitespace errors, fix the named lines in the affected Markdown file and re-run it.
- If a Git line-ending warning appears, no action is required for this documentation change unless the repository's `.gitattributes` policy says otherwise.
