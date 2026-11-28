---
id: use-dtech-aidlc-to-develop-the-backup-and-restor
title: Backup and restore U1 slice (synthetic-only, localhost, admin-only)
origin: intent
scope: solo
closed: 2026-11-28T14:29:28Z
phases-run: analyze, ideate, develop, archived
supersedes:
tags: backup, restore, domain_backup, gateway, admin-web, security
---

# Backup and restore U1 slice (synthetic-only, localhost, admin-only)

## Outcome

Delivered the U1 slice of `docs/features/backup-restore.md` (Option A from the solution-direction gate) on branch `doitsu2014/migrate-exported-documents-to` (last commit `ad2d4ae`), not merged to `main`:

- New crate `apps/api/domain_backup`: config with fail-closed disabled reasons, status enum with `incomplete` (D-10), state machine where `succeeded` / `recovery-required` are unreachable for U1 jobs, own migration table `seaql_migrations_backup`, job and append-only audit stores, idempotent atomic acceptance (key stored as digest, one active job per instance), allocation ledger and fixture storage, worker with synthetic fixture capture, lifecycle reconciler / cleaner / sweeper / runtime with a start-up barrier and outage recovery, REST IF-01..IF-11 behind administrator, Host and Origin checks, download always 403, upload / preflight / restore always 403, labelled synthetic previews.
- Gateway (`apps/api/gateway`): `Mount::AdministratorIngress` (in `domain_interface`) with request-timeout (408) and header-size (431, before auth) layers on backup routes only; loopback guard on the effective bind host; runtime start after migrations; 2 s worker driver and 60 s scheduler driver; backup migration identities chained after seo.
- Admin web (`apps/web`): typed zod client `src/domains/backup.ts`; `/admin/backup` page with instance selector, create panel (fresh `crypto.randomUUID()` key, double-submit guard), job list/detail with bounded polling, status banner, synthetic preview panel; no download control anywhere.
- Verification: 18/18 work units with red/green evidence; final sweep Rust 440 + 219 Docker, web 183, web-bundle secret-canary scan with positive controls; threat table T-01..T-14 in `.aidlc/intents/use-dtech-aidlc-to-develop-the-backup-and-restor/develop/build-and-verify/WU-15/green-verification-report.md`. Release-candidate gate approved in AI mode with tagged follow-ups.
- Launch and Curate did not run (solo scope).

## Decisions

- Option A: admin UI first, dedicated `domain_backup` domain, public `incomplete` status (solution-direction gate, 2026-11-21).
- U1 boundary: localhost, synthetic-only fixtures, administrator-only, database slice, no restore, no download, never `succeeded`.
- CFG-17 `BACKUP_REQUEST_TIMEOUT_SECS` = 30 and CFG-18 `BACKUP_MAX_HEADER_BYTES` = 16384: owner-approved values (Duc Tran, 2026-11-28); both remain required when backup is enabled (no silent default).
- RK-15 (posts and seo share the default `seaql_migrations` table): gateway boot beside posts/seo descoped from U1 as a residual; no seo/posts changes (owner, 2026-11-28).
- RK-16: U1 stays off `main` until a separate release decision, because backup migrations register unconditionally and `main` auto-releases an image (owner, 2026-11-28).
- Production composition starts the F-16 worker with the runtime (FD 7.1), so created jobs reach `incomplete` without manual steps.

## Limitations

- No browser-level evidence (E2E-ENV-01: UI on `localhost:3002` calls the API cross-origin; no environment chosen). UI covered by component tests.
- CI (`ci-my-cms.yml`) runs `cargo test` without `--ignored` and no web tests: most backup tests (Docker-backed) never run in CI (RK-09).
- `migrate down` / `migrate status` cover only the posts migrator; no CLI rollback for `backup_*` tables (RK-08).
- TS-31 owner-only permission check partial on Windows (T-14 accepted residual).
- IF-11 `reconciliation.last_completed_at` is always null; metrics are in-memory only; a failed runtime start is not retried until restart.
- Test harness: the Docker-backed suite needs `--test-threads=4` on a laptop; at full parallelism (~220 testcontainers) one random test intermittently fails at container start (`PortNotExposed`, "postgres never became ready").
- Tooling pre-existing in this repo: `pnpm lint` fails at ESLint config load (`react-hooks` `recommended-latest`); 23 `tsc` errors in untouched web files.
- On macOS `std::env::temp_dir()` sits behind the `/var -> /private/var` symlink, which the CFG-04 storage-root check rejects; backup tests use `crate::test_temp_dir()`.

## Follow-ups

- Release and merge decision for U1 (RK-16), including whether backup migrations should register only when enabled.
- Choose E2E-ENV-01 and run the TS-41 UI part.
- Resolve RK-15 (separate migration tables for posts/seo) and test gateway boot with all migrators (TS-34 upgrade case).
- Add Docker-backed and web tests to CI (RK-09).
- Make `domain_backup/src/test_db.rs` robust at full parallelism (retry or concurrency limit).
- Fix the `apps/web` ESLint config so `pnpm lint` runs.
- Manual assistive-technology review of the backup page (TS-37).
- Open owner questions before any U2 (real data): Q-04 Supabase Auth/Storage recovery, Q-06 RPO/RTO, Q-07 production policy, Q-10 cross-store consistency, Q-11 encryption and keys, Q-12 toolchain readiness, audit retention, non-owner technical guards, performance targets P-01..P-05.

## Links

- Intent folder: `.aidlc/intents/use-dtech-aidlc-to-develop-the-backup-and-restor/`
- Intent record: `.aidlc/intents/use-dtech-aidlc-to-develop-the-backup-and-restor/archived/intent-record/intent-summary.md`, `completed-scope.md`, `artifact-index.md`
- Gate recommendation: `.aidlc/intents/use-dtech-aidlc-to-develop-the-backup-and-restor/develop/build-and-verify/gate-recommendation.md`
- Design: `.aidlc/intents/use-dtech-aidlc-to-develop-the-backup-and-restor/develop/implementation-design/` (interfaces-and-contracts, functional-design, test-strategy, work-units, ids.json)
- Product docs: `docs/features/backup-restore.md` (section 14), `docs/architecture/software-architecture.md`, `README.md` (backup configuration)
