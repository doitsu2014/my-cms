# WU-13 known limitations
- Browser verification blocked by E2E-ENV-01; component tests only.
- TS-35 and the client guard are UX only; server enforces authorization.
- Job status display, polling, list, preview, banner are WU-14.
- `pnpm lint` fails at config load (react-hooks recommended-latest), pre-existing; touched files linted with a temporary config. 23 pre-existing tsc errors in untouched files.
- Created-job confirmation shows only the job id.
- The worker worktree was created from `main` instead of the intent branch; its commit applied cleanly onto the intent branch because the touched web files are unchanged between those bases.
