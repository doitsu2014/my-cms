# WU-15 implementation

- **Status:** Complete against the WU-15 "Done when" (attempt 1, 2026-11-28). No application code changed.
- **Author:** worktree worker (model `sonnet`, commit `3213b77`), integrated and verified by the conductor (one test removed, see green-verification-report.md "Integration change").

## Files

| File | Change |
|---|---|
| `apps/api/domain_backup/src/infra/tests/verification_sweep.rs` (new) | TS-39 latency measurement, TS-40 load bounds, TS-41 API end-to-end (create, worker, `incomplete`, download 403), TS-58 feature-doc check. Docker tests are `#[ignore]`. |
| `apps/api/domain_backup/src/infra/tests/mod.rs`, `http_api.rs` | Module registration; shared helpers (`config`, `test_domain_context`, `admin`, `request`, `body`) made `pub(super)`. |
| `apps/web/scripts/bundle-canary-scan.mjs` (new), `bundle-canary-scan.test.mjs` (new), `apps/web/package.json` (`scan:bundle-canary` script) | TS-17 full: builds the web app with 6 canary env values, scans `dist`, and runs two positive controls (canary injected into a copy of the real dist; a build whose source reads a `PUBLIC_*` variable). |
| `docs/features/backup-restore.md` | New section 14: U1 status, what exists, how to enable (pointing to README), open items Q-04/06/07/10/11/12, audit retention, non-owner guards, E2E-ENV-01, not-authorized list; history notes on older sections. |
| `docs/architecture/software-architecture.md` | `domain_backup` row, `AdministratorIngress` mount, ingress layers, runtime and worker start, six-domain manifest. |
