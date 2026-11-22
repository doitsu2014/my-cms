# IMPL-03 Implementation manifest: WU-03

- **ID:** IMPL-03 (WU-03)
- **Stage:** Develop / build-and-verify (builder-verifier), unit WU-03 of WU-01..WU-15.
- **Status:** WU-03 implemented and verified on 2026-11-22 against PostgreSQL 15 (Docker). Uncommitted, on branch `doitsu2014/migrate-exported-documents-to` (owner rule: branch-only until Launch).
- **Inputs:**
  - [red-tests/WU-03](../../red-tests/WU-03/failing-tests.md), including the section 3.2 design choices.
  - [ids.json](../../implementation-design/ids.json).
  - [interfaces-and-contracts](../../implementation-design/interfaces-and-contracts.md) sections 4, 7, 9 and 10.1.
  - [functional-design](../../implementation-design/functional-design.md): FN-15, FN-16 and FN-34 (legacy F-15, F-16, F-34), plus sections 6.6 and 7.1.
- **Evidence:** [green-verification-report.md](green-verification-report.md), [regression-report.md](regression-report.md), [known-limitations.md](known-limitations.md).

## Files (production code; tests are listed in the red manifest)

| Path (under `apps/api/domain_backup/`) | Lines | Content |
|---|---|---|
| `src/ports/job_store.rs` | 214 | `JobStore` trait (`accept`, `get`, `transition`, `claim_next`). Types `NewJob`, `IdemRecord`, `JobRow`, `ComponentState`, `TransitionResult`, `ReasonCode` (9 IFC 10.1 spellings), `StoreError` |
| `src/ports/audit.rs` | 158 | `AuditSink` trait. `AuditEvent`, `AuditEventKind` (16 closed spellings), `AuditOutcome`, `AuditError::{Rejected, Unavailable}`, and `Digest` (64 ASCII hex characters only) |
| `src/infra/pg_error.rs` | 67 (new) | Classifies a `DbErr` into `Outage`, `Database { code, constraint }` or `Other`. Never carries SQL or driver text. |
| `src/infra/db_job_store.rs` | 414 | `DbJobStore` (FN-15, FN-16) |
| `src/infra/db_audit_sink.rs` | 75 | `DbAuditSink` and `classify_audit_error` (FN-34) |
| `src/infra/mod.rs`, `src/ports/mod.rs`, `src/lib.rs` | small | Module wiring |
| `Cargo.toml` | modified | Adds `uuid` (red stage). `serde_json` moves from dev-dependencies to dependencies (build stage). |
| `apps/api/Cargo.lock` | modified | `uuid` edge for `domain_backup` |

No WU-01 or WU-02 code or test changed (checked with `diff -r` and `cmp` against the red snapshot). Gateway, other crates and web are untouched. The crate is still not composed into the gateway; that happens in WU-10.

## Behaviour

- **Error classification (`pg_error::classify`, FD 6.6):**
  - **Outage:** `ConnectionAcquire`, `Conn`, and sqlx `Io`/`Tls`/`PoolTimedOut`/`PoolClosed`/`WorkerCrashed`/`Protocol`. Also every SQLSTATE that is not a statement rejection (for example 57014, 57P*, 08*, 53*, 25006, 40*, XX*); this is an allow-list, so anything unknown fails closed.
  - **Database:** a statement rejection on a healthy connection, meaning SQLSTATE class 22, 23, 42, `P0*`, or `25P02`.
  - **Other:** anything else.
- **`DbAuditSink::record`:** one parameterised plain `INSERT INTO backup_audit` with a fresh UUID. It runs on the given transaction, or on its own connection when none is given. It never issues UPDATE, DELETE or `ON CONFLICT`. `classify_audit_error` maps `Database` to `Rejected`, and `Outage` or `Other` to `Unavailable`.
- **`accept`:**
  - Runs on the caller's transaction.
  - Checks both digests with `Digest::new` before any SQL and returns `Persistence` on failure.
  - Inserts the job as `queued` (`NewJob` has no status field), then the `backup_idempotency` row.
  - SQLSTATE 23505 maps to `ConcurrencyConflict`, except a `*_pkey` clash, which maps to `Persistence`.
  - Outage maps to `StoreUnavailable`; anything else maps to `Persistence`.
  - No audit row is written here; WU-07's handler writes `job.accepted`.
- **`get`:** a parameterised SELECT. Components are parsed with `serde_json`.
- **`transition`:**
  1. Edges outside `is_allowed` return `IllegalTransition` before any SQL.
  2. Otherwise a compare-and-set runs on the store's own transaction: `UPDATE ... WHERE id = $ AND status = $ AND state_version = $`, which bumps `state_version` and sets the time stamps, reason and components.
  3. If no row matches, the transaction rolls back and returns `Lost` when the job exists, or `NotFound` when it doesn't.
  4. If a row matches, the `job.*` audit row (`job.running`, `job.incomplete`, `job.failed`, or `dispatch.failed` for queued to failed with `DISPATCH_FAILED`) is written on the same transaction, which then commits.
  5. An audit `Rejected` rolls back and returns `AuditUnavailable`. An audit `Unavailable` rolls back and returns `StoreUnavailable`.
- **`claim_next`:**
  - One atomic statement on its own transaction: `UPDATE backup_jobs SET status = 'running', state_version + 1, epoch, started_at, deadline_at, updated_at WHERE status = 'queued' AND id = (SELECT id ... WHERE status = 'queued' ORDER BY created_at, id FOR UPDATE SKIP LOCKED LIMIT 1) RETURNING ...`.
  - No row means `Ok(None)`.
  - Otherwise `job.running` is written on the same transaction, which commits, with the same audit-failure mapping as `transition`.
- **Safety:**
  - All SQL is parameterised.
  - No `unwrap` or `expect` on runtime data.
  - `StoreError::Persistence` and `AuditError` carry no database text (T-09).

## How it was built (process record)

1. A builder (sonnet, `stage.model`) was briefed with the CLI directive verbatim and the `ids.json` rule. It got all tests green on the first pass.
2. Two reviewers ran:
   - Integrity: byte-compared the tests and checked scope.
   - Correctness, concurrency and safety: 6 findings, 1 major.
3. The fixer:
   - Narrowed the "statement rejection" classes to an allow-list (major).
   - Added the `accept` digest check.
   - Deferred four items, recorded in known-limitations.
4. A mutation agent ran 30 mutations across 17 guards. Its claim-atomicity result was wrong (green report, section 3).
5. The conductor:
   - Re-ran the claim mutations itself.
   - Added two test-after tests, each proven by mutation.
   - Ran the final green, regression, fmt, clippy and `ids check` itself.
