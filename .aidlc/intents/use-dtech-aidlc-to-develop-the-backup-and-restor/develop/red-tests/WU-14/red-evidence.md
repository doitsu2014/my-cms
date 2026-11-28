# WU-14 red evidence

Command (in apps/web, worktree wf_4a2c2c5b-6a2-1, base 0021087):
`pnpm exec vitest run src/domains/backup.test.ts src/app/admin/backup` -> `evidence/red.log`

Result: Test Files 9 failed | 3 passed (12); Tests 72 failed | 54 passed (126).

Per file: backup.test.ts 37 tests / 3 failed; use-backup-status.test.ts 4 / 3; job-status-badge.test.tsx 11 / 10; use-backup-jobs.test.ts 14 / 13; backup-status-banner.test.tsx 12 / 11; job-list.test.tsx 7 / 7; job-detail.test.tsx 10 / 10; synthetic-preview-panel.test.tsx 10 / 10; page.status.test.tsx 5 / 5. The 3 passing files are the untouched WU-13 files (page.test.tsx, instance-selector, job-create-panel).

Why behavioural: before the red run the stubs compiled and imported cleanly (components returned null, hooks returned empty constant state, IF-09/IF-10 client functions returned undefined; zod schemas were complete). There were no import, syntax or setup failures. Failures are assertion failures on missing behaviour. Verbatim excerpts from red.log:

- `AssertionError: expected "vi.fn()" to be called 1 times, but got 0 times` (7 occurrences; hooks and panel call counts)
- `AssertionError: expected "vi.fn()" to be called 2 times, but got 0 times` (3; polling after 2 s, backoff)
- `TestingLibraryElementError: Unable to find role="table" and name /backup jobs/i` (3; job list renders nothing)
- `TestingLibraryElementError: Unable to find an element with the text: Unknown status.` (3; unknown fallback missing)
- `TestingLibraryElementError: Unable to find an element with the text: Incomplete.` (3)
- `TestingLibraryElementError: Unable to find an accessible element with the role "alert"` (7; banner and error copy missing)
- `TestingLibraryElementError: Unable to find an accessible element with the role "button" and name /run preflight/i` (8; preview panel renders nothing)

Testing-library "unable to find" errors are the assertion that the required rendered behaviour exists; they are not setup failures because the component mounted and the DOM was inspected.

Guards (passed in red, listed in failing-tests.md): scheme closure test and schema-rejection test (schemas already complete), constants test in use-backup-jobs.test.ts, "ignores a response after unmount" tests (stub never calls the API), "omits the incomplete description unless detailed" (null render), and all pre-existing WU-12/WU-13 tests.

One test-authoring defect found and fixed during green: `beforeEach(() => mock.mockReset())` returned the mock, which vitest ran as a teardown and produced a spurious unhandled rejection (use-backup-status.test.ts). Fixed by using a block body. Not a product defect.
