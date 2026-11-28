# Release-candidate gate recommendation

- **Gate:** `release-candidate` (phase Develop, final stage `build-and-verify` sweep). Gate mode `ai`; decided by the AI Orchestrator on 2026-11-28.
- **Candidate:** branch `doitsu2014/migrate-exported-documents-to` at `16fdfe0` (U1 backup slice: localhost, synthetic-only, administrator-only, database slice, no restore).
- **Inputs read:** `regression-report.md`, `green-verification-report.md`, `implementation.md`, `known-limitations.md` (this folder); every `WU-nn/` report; `WU-15/green-verification-report.md` (threat evidence record); `develop/baseline.md`; Develop exit criteria in the phase file.

## Exit criteria and evidence

| Develop exit criterion | Observed evidence | Met? |
|---|---|---|
| Implementation matches the approved scope and contracts | 18/18 units satisfied, each with red/green evidence and a done-when trace; IF-01..IF-11 contract tests (WU-08, WU-09, WU-10); web client fields checked against Rust serializers (WU-12, WU-14); `ids check` ok | yes |
| Verification evidence, known limitations, deployment/rollback notes recorded | Unit reports; `known-limitations.md`; `implementation.md` deployment and rollback notes (RK-08, RK-16); threat table T-01..T-14 (WU-15) | yes |
| Existing suite run after the change and compared with baseline | Sweep: Rust 440/0 + 219/0 Docker, web 183/0; untouched crates identical to the 2026-11-22 baseline; no previously passing test fails (`regression-report.md`) | yes |
| Owner approves the release candidate before launch activities | This AI-mode gate approval; launch is not entered or authorized by it (see boundary) | by this decision |

## Options considered

1. **Approve the release candidate with recorded residuals (selected).** All suites green; every threat T-01..T-13 has passing recorded tests, T-14 is partial with an owner-accepted residual; the residuals that remain (E2E-ENV-01, RK-15 descoped boot test, CI coverage, manual accessibility review, open owner questions) are already decided by the owner as out of U1 scope or are tracked follow-ups. Trade-off: the candidate has no browser-level and no CI evidence; these are stated, not hidden.
2. **Block until browser E2E (E2E-ENV-01) and CI coverage of Docker tests (RK-09) exist.** Strongest evidence, but both need decisions or work outside U1 (an environment choice not yet made, CI workflow changes in files outside the approved scope). Blocking would stall the candidate on items the owner has not put in U1.
3. **Block until the Docker test-harness flakiness is fixed.** The flake is a test-infrastructure issue (container start-up under ~220 parallel testcontainers), reproducibly absent at `--test-threads=4`, and never a product assertion failure. Fixing it is worthwhile but not a correctness gap in the candidate.

## Evidence trace: observed vs assumed

- **Observed:** sweep results and per-crate counts (`evidence/sweep-*.log`); bundle canary scan PASS with both positive controls; `succeeded` unreachable across state machine, DB constraint, serializer, worker and UI; ingress layers 431-before-401 and 408 (WU-10 TS-55); production worker drives jobs to `incomplete` (WU-10 `f37_started_service_runs_a_created_job_to_incomplete`, WU-15 TS-41 API); download 403 for every status (WU-09, WU-15).
- **Assumed / not verified:** behaviour in a browser (E2E-ENV-01 blocked); gateway boot alongside posts and seo migrators on one database (RK-15, owner-descoped); Windows owner-only ACL (TS-31 partial); performance under realistic data (TS-39/TS-40 measured on a 4 KiB fixture, debug build, no targets); CI behaviour (not observed).

## Owner decisions in force

- 2026-11-28 (Duc Tran, this session): CFG-17 = 30, CFG-18 = 16384; RK-15 descoped as residual; RK-16 keep U1 off `main`.
- Confirmed 2026-11-28 by Duc Tran: WU-17 scheduling decision (2026-11-26) and WU-08 revisit decision (2026-11-27), previously recorded by the AI Orchestrator.

## Decision

Approve the release candidate (option 1).

## Residual risks and follow-ups

- Migrations register unconditionally; merging to `main` would auto-release an image that creates `backup_*` tables in production. Mitigated by RK-16 (no merge until a separate release decision).
- No browser-level or CI evidence; Docker-backed tests do not run in CI.
- Docker-backed suite needs reduced parallelism on this host.
- Open product questions remain open and are not answered by this candidate.

## Boundary

This approval authorizes: closing Develop for this intent with the U1 slice as the release candidate on its feature branch, and recording the evidence above. It does **not** authorize: merging to `main`, building or publishing a release image, deployment to any environment, Launch activities, enabling backup on any shared or production instance, real-data staging, upload, restore or download, or answering any open owner question (Q-04, Q-06, Q-07, Q-10, Q-11, Q-12, audit retention, non-owner guards, P-01..P-05).

## Open items

- [follow-up] Release and merge decision for U1 (RK-16): separate owner decision before any merge to `main` or image release.
- [follow-up] E2E-ENV-01: choose a browser test environment, then run the TS-41 UI part.
- [follow-up] RK-15 residual: gateway boot beside posts and seo migrators (TS-34 upgrade case, TS-41 runtime part), owner-descoped from U1.
- [follow-up] RK-09: CI does not run Docker-backed (`--ignored`) or web tests; no green CI claimed.
- [follow-up] TS-31 on Windows (T-14 partial) and a Unix CI run of the permission checks.
- [follow-up] TS-37 manual assistive-technology review.
- [follow-up] Docker test-harness reliability at full parallelism (`domain_backup/src/test_db.rs` retry or concurrency limit).
- [follow-up] Owner questions Q-04, Q-06, Q-07, Q-10, Q-11, Q-12, audit retention, non-owner technical guards and performance targets P-01..P-05 stay open; U1 does not depend on them.
- [follow-up] Pre-existing tooling: `pnpm lint` ESLint config failure, 23 `tsc` errors in untouched web files, IF-11 `last_completed_at` always null, metrics exporter not wired.
