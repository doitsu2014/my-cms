# Intent summary

- **Intent:** `use-dtech-aidlc-to-develop-the-backup-and-restor`. Task: "Use dtech-aidlc to develop the backup and restore feature based on @docs/features/backup-restore.md."
- **Project:** my-cms (Rust API gateway with domain crates, React admin web app).
- **Scope / depth / mode:** `solo`, `full`, AI orchestration mode. Started 2026-11-21; Develop closed 2026-11-28.
- **Branch:** `doitsu2014/migrate-exported-documents-to`, last commit `ad2d4ae`. Not merged to `main` (owner decision RK-16).

## Outcome

The intent delivered the **U1 slice** of backup and restore chosen at the solution-direction gate (Option A): a localhost-only, administrator-only, synthetic-only database-slice workflow in a new `domain_backup` crate, composed by the gateway, with an admin web page. An administrator can see the configured local instance, create a synthetic backup job with a fresh idempotency key, follow it to its truthful terminal state (`incomplete` or `failed`, never `succeeded`), see the backup status and escalation banner, and run labelled synthetic preflight and restore-confirmation previews. Real upload, preflight, restore and download are refused by the server.

Underneath that, the domain persists jobs before answering, stores idempotency keys only as digests, allows one active job per instance, runs jobs in a worker outside the request, tracks staged bytes in an allocation ledger with reconciliation, cleanup, sweeping and a start-up barrier, writes append-only audit records, and fails closed on store or audit outages. The gateway mounts the routes behind a request timeout (30 s) and header-size limit (16384 bytes) ahead of administrator authentication, forces backup off on non-loopback binds, starts the runtime after migrations and drives the scheduler and worker.

All 18 work units passed red/green verification. The final regression sweep passed every suite: Rust 440 normal and 219 Docker-backed tests, web 183 tests, plus a clean web-bundle secret-canary scan with positive controls. The release-candidate gate was approved in AI mode with tagged follow-ups (`develop/build-and-verify/gate-recommendation.md`).

## What was not done

- Launch and Curate were outside the `solo` scope and did not run.
- Real data capture, complete ZIP backups, upload, restore and download were explicitly not authorized for U1.
- Browser end-to-end verification stayed blocked (E2E-ENV-01). Gateway boot beside the posts and seo migrators was descoped by the owner (RK-15). CI does not run the Docker-backed tests (RK-09).

See `completed-scope.md` for the stage-by-stage record and `artifact-index.md` for every artifact path.
