# Next-intent notes

For whoever starts the next backup-related intent (U2 or a release intent). Knowledge record: `.aidlc/knowledge/use-dtech-aidlc-to-develop-the-backup-and-restor.md`.

## Start here

- The U1 code is on branch `doitsu2014/migrate-exported-documents-to`, not on `main`. Decide RK-16 (release/merge and migration gating) first: merging as-is would auto-release an image that creates `backup_*` tables in every environment.
- Read `docs/features/backup-restore.md` section 14 for what exists today and the open questions; read the U1 contracts in `develop/implementation-design/interfaces-and-contracts.md` before changing any API shape (the web client in `apps/web/src/domains/backup.ts` parses responses strictly).

## Practical notes for working in this repo

- Run Docker-backed Rust tests with `cargo test --workspace --no-fail-fast -- --ignored --test-threads=4`; full parallelism overloads Docker on a laptop.
- On macOS use `crate::test_temp_dir()` for backup storage roots in tests (the OS temp dir is behind a symlink that CFG-04 rejects).
- `pnpm lint` in `apps/web` is broken by its ESLint config; lint touched files with a temporary copy of `eslint.config.mjs` without the `reactHooks` lines until it is fixed. `tsc --noEmit` has 23 pre-existing errors in untouched files.
- `pnpm scan:bundle-canary` (apps/web) rebuilds the app with canary env values and scans `dist`; rerun it after adding any build-time variable.
- Claude Code worktrees created for parallel workers started from `main`, not from the intent branch; reset a worker's worktree to the intent branch tip before work.

## Before any real-data work (U2)

- Answer Q-04, Q-06, Q-07, Q-10, Q-11, Q-12, audit retention and the non-owner technical guards; set performance targets P-01..P-05.
- Choose E2E-ENV-01 so browser-level checks can run; add Docker and web tests to CI (RK-09).
- Resolve RK-15 (posts and seo on separate migration tables) and add the TS-34 "upgrade beside posts and seo" test.
