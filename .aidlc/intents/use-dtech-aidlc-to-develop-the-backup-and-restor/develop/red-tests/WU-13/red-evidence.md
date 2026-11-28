# WU-13 red evidence

Command (apps/web): `pnpm exec vitest run src/App.test.tsx src/app/admin/backup src/app/admin/components/left-menu.test.tsx` (log: `evidence/red.log`)
Result: Test Files 5 failed (5); Tests 20 failed | 1 passed (21). The 1 pass is the existing SEO menu test (guard).

Setup note: the worker worktree was created from `main` (`6f0ed65`) instead of the intent branch, so WU-12 (committed on the intent branch as `ffd2519`) was missing there. The first run failed on `Failed to resolve import "@/domains/backup"` (setup error, not counted). The worker copied the WU-12 files into the worktree uncommitted and re-ran; `evidence/red.log` is the second run. Page, selector and panel were stubs returning null so the tests import and run.

Verbatim assertion excerpts:
- TestingLibraryElementError: Unable to find an accessible element with the role "link" and name "Backup" (left-menu ts35)
- TestingLibraryElementError: Unable to find an element with the text: backup page content. (App route, admin)
- TestingLibraryElementError: Unable to find an element with the text: /do not have permission/i. (App route, writer)
- TestingLibraryElementError: Unable to find an accessible element with the role "radio" and name `/Local synthetic instance/` (TS-46)
- TestingLibraryElementError: Unable to find an element with the text: /no backup instance is available/i. (TS-38)
- TestingLibraryElementError: Unable to find an accessible element with the role "button" and name `/create backup/i` (TS-52 panel and page)

Why behavioural: each test renders real components or the real App router and fails an assertion on missing behaviour (no menu link, no route, nothing rendered by the stubs); none fails on compile, import or setup in the recorded run.
Limitation of the red: panel/selector/page failures are "element missing" from null stubs, the usual red shape for UI components.
Later correction (green phase): ts46_marks_the_selected_instance... clicked an already-checked radio, which never fires onChange; the test was fixed to click an unchecked radio. That is a defect in my test, not a weakening of intent.

Environment: macOS Darwin 25.6.0, node v24.14.1, vitest in apps/web.
