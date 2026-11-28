# IMPL-03 Implementation manifest: WU-03

- **ID:** IMPL-03 (WU-03)
- **Stage:** Develop / build-and-verify (builder-verifier), unit WU-03 of WU-01..WU-15.
- **Status:** WU-03 implementation is present. Historical green evidence is preserved below; the 2026-11-24 verification refresh is **blocked** because workspace runs expose failures in later WU-06/WU-17/18/19 tests. WU-03-targeted tests pass. No application source or tests were changed during this refresh.
- **Inputs:**
  - [red-tests/WU-03](../../red-tests/WU-03/failing-tests.md), including the section 3.2 design choices.
  - [ids.json](../../implementation-design/ids.json).
  - [interfaces-and-contracts](../../implementation-design/interfaces-and-contracts.md) sections 4, 7, 9 and 10.1.
  - [functional-design](../../implementation-design/functional-design.md): FN-15, FN-16 and FN-34 (legacy F-15, F-16, F-34), plus sections 6.6 and 7.1.
- **Evidence:** [green-verification-report.md](green-verification-report.md), [regression-report.md](regression-report.md), [known-limitations.md](known-limitations.md).

## Files (production code; tests are listed in the red manifest)

| Path (under `apps/api/domain_backup/`) | Lines at original implementation | Content |
|---|---:|---|
| `src/ports/job_store.rs` | 214 | `JobStore` trait (`accept`, `get`, `transition`, `claim_next`). Types `NewJob`, `IdemRecord`, `JobRow`, `ComponentState`, `TransitionResult`, `ReasonCode` (9 IFC 10.1 spellings), `StoreError` |
| `src/ports/audit.rs` | 158 | `AuditSink` trait. `AuditEvent`, `AuditEventKind` (16 closed spellings), `AuditOutcome`, `AuditError::{Rejected, Unavailable}`, and `Digest` (64 ASCII hex characters only) |
| `src/infra/pg_error.rs` | 67 (new) | Classifies a `DbErr` into `Outage`, `Database { code, constraint }` or `Other`. Never carries SQL or driver text. |
| `src/infra/db_job_store.rs` | 414 | `DbJobStore` (FN-15, FN-16) |
| `src/infra/db_audit_sink.rs` | 75 | `DbAuditSink` and `classify_audit_error` (FN-34) |
| `src/infra/mod.rs`, `src/ports/mod.rs`, `src/lib.rs` | small | Module wiring |
| `Cargo.toml` | modified | Adds `uuid` (red stage). `serde_json` moves from dev-dependencies to dependencies (build stage). |
| `apps/api/Cargo.lock` | modified | `uuid` edge for `domain_backup` |

No WU-01 or WU-02 code or test changed during the original WU-03 implementation (as recorded in the historical report). Gateway, other crates and web were untouched then. The crate remains not composed into the gateway; that happens in WU-10.

## Implemented behavior (rechecked against current source)

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
- **`transition`:** checks the U1 edge before SQL; otherwise performs CAS on `(id, status, state_version)`, updating version and timestamps. A lost CAS reports `Lost` (or `NotFound` for an absent job). An applied transition writes the matching `job.*` event inside the same transaction; audit failure rolls it back and maps `Rejected` to `AuditUnavailable`, `Unavailable` to `StoreUnavailable`.
- **`claim_next`:** atomically claims the oldest queued job with `FOR UPDATE SKIP LOCKED`, writes epoch/start/deadline and increments state version. It audits `job.running` in the same transaction; an audit failure rolls the claim back.
- **Safety:** all SQL is parameterised; no runtime `unwrap`/`expect`; persistence and audit errors carry no database text (T-09).

## Historical implementation process (2026-11-22; not part of the refresh)

1. A builder (sonnet, `stage.model`) was briefed with the CLI directive verbatim and the `ids.json` rule. It got all tests green on the first pass.
2. Two reviewers ran integrity checks and correctness, concurrency and safety review (6 findings, 1 major).
3. The fixer narrowed statement-rejection classes to an allow-list, added the `accept` digest check and recorded four deferred items.
4. A mutation agent ran 30 mutations across 17 guards; its claim-atomicity result was wrong (green report, section 3).
5. The conductor re-ran claim mutations, added two test-after tests (each proven by mutation), and performed the historical final test, regression, fmt, clippy and `ids check` runs.

## Refresh scope and current disposition (2026-11-24)

Re-ran focused WU-03 tests, the entire Rust workspace normal and ignored suites, workspace formatting, workspace clippy and the requested identifier check. Docker was available. Focused WU-03 results are green: `infra::tests::job_store` normal 1 passed/35 ignored, Docker ignored 35 passed, and `infra::tests::audit_sink` normal 3 passed/8 ignored plus Docker ignored 8 passed. The full workspace is not green: the normal run failed 13 in-memory lifecycle tests belonging to later WU-17/18/19 work; the ignored run failed seven DB reconciliation tests belonging to later WU-06 work. Their failure names, exits, complete suite counts and evidence paths are in the refreshed verification/regression reports. These WU-06/WU-17/18/19 failures are outside WU-03 and must not be attributed to the WU-03 implementation, but the selected build-and-verify refresh cannot be reported complete while the requested full suite remains red.

No source or test change was made because the failures are outside WU-03, and the instruction requires stopping rather than implementing defects found in another unit. Current refresh should be reported blocked, not green.
