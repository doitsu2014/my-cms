# RE-03 Red evidence: WU-03

- **ID:** RE-03 (WU-03)
- **Stage:** Develop / red-tests (builder-verifier), unit WU-03.
- **Status:** red run recorded on 2026-11-22.
- **Companion:** [failing-tests.md](failing-tests.md). Raw logs are in [../evidence/](../evidence/).
- **Baseline:** [../../baseline.md](../../baseline.md), the existing-suite baseline recorded once for dtech-aidlc 0.7.0 and reused here. No new baseline run was needed because no application code changed since it was taken.

Provenance: everything under Observed was run by the conductor on 2026-11-22: Windows 11 dev host, cargo 1.100.0, Docker 29.7.2, `postgres:15-alpine`.

## 1. Environment on turn one (Observed)

- `dtech-aidlc config prerequisite check`: `docker` passes (`docker info`, exit 0).
- The `orchestrate next` directive carried no warnings once `PROJECT.md` had the three sections 0.7.0 expects (owner decision below) and `onboard --rescan` acknowledged the change.

## 2. Owner decisions at the start of this stage

User-provided by the project owner on 2026-11-22, via AskUserQuestion:

- **PROJECT.md:** the existing facts were restructured under `## Commands`, `## Environment prerequisites` and `## Known baseline failures`. No new facts were added beyond the PostgreSQL 15 pin and the note that CI skips the ignored DB tests.
- **Branch:** stay on `doitsu2014/migrate-exported-documents-to` and do not create `feature/<intentId>`.

## 3. WU-03 red run (Observed)

- **No-Docker run** (`cargo test -p domain_backup --no-fail-fast`): exit 101; 37 passed, 4 failed, 75 ignored. All passes are pre-existing tests and all failures are new ones.
- **Docker run** (`-- --ignored`, about 58 s): exit 101; 34 passed (pre-existing), 41 failed (new).
- **How the 45 new tests fail:**
  - The failure messages are assertion texts (for example "exactly one create wins", "the job row must be written on the caller's transaction", "ConnectionAcquire(Timeout) must be treated as an outage").
  - There are no container, connection or timeout errors. The log's only "ConnectionAcquire" match is the classifier test's own assertion message.
- **Builds and identifiers:** `cargo check --workspace` exits 0, and `dtech-aidlc ids check` passes with 0 unregistered identifiers.
- **Snapshot:** taken after `cargo fmt -p domain_backup` (`--check` exit 0), so later formatting cannot touch the red tests.

### 3.1 How the red was produced

1. A dynamic workflow ran:
   - one builder (sonnet, from `stage.model`), briefed with the CLI directive verbatim and the `ids.json` rule;
   - two independent reviewers, one for spec fidelity and one for red integrity (they ran the Docker suite);
   - a fixer.
2. The reviewers returned 16 findings, 7 of them major. The fixer applied 12, including:
   - spy sinks proving the store hands its own uncommitted transaction to the audit sink;
   - deterministic lock-wait tests instead of probabilistic races;
   - the IFC 2.3 component shape;
   - mapping any unique violation to the concurrency conflict;
   - FD-consistent reasons per edge;
   - the closed `ReasonCode` set.
3. The fixer **rejected** one major finding, claiming that `AuditError::Rejected` and "6.6" do not exist. **The conductor overruled this:**
   - Functional-design section 6.6 defines the split. An audit statement rejected on a healthy connection is `AuditError::Rejected`, giving 503 `AUDIT_UNAVAILABLE` with store health unchanged. A connection-level failure is an outage, giving 503 `STORE_UNAVAILABLE`. An unclassifiable failure fails closed as an outage.
   - The fixer had searched only interfaces-and-contracts.
   - Before the red was recorded, the conductor:
     - added `AuditError::Rejected` and `StoreError::StoreUnavailable`;
     - made `FailingAuditSink` and `WriteThenFailSink` return `Rejected`;
     - changed the rename-the-table test to expect `Rejected`, since that is a schema rejection on a healthy connection;
     - added `OutageAuditSink` with `an_audit_outage_is_a_store_outage_for_transitions_and_claims`;
     - added the no-Docker classifier test `connection_level_and_unclassifiable_failures_are_an_outage`;
     - added a logic-free `classify_audit_error` stub that always returns `Rejected`.

### 3.2 Design choices made during red, for build-and-verify

| Topic | Decision | Basis |
|---|---|---|
| Store owns its audit writes | `DbJobStore::new(conn, Arc<dyn AuditSink>, Arc<dyn Clock>)`. `transition` and `claim_next` write the `job.*` audit row on their own transaction, and an audit failure refuses the change. | FN-34, IFC 6.3, WU-03 done-when; the contract's `transition` takes no tx |
| Path to `running` | `claim_next` is the only pinned path. `queued -> running` through `transition` is neither pinned nor refused. | FN-16; FD names the claim as the dispatch path |
| Unique violations | Any unique violation on `accept` (active-job index or idempotency triple) is `ConcurrencyConflict`. A primary-key clash is `Persistence`. | IFC 4 ("on ANY unique violation") |
| `queued -> failed` audit event | `job.failed` or `dispatch.failed`; either is accepted | FD 7.1 |
| Audit outcome strings | not pinned, only membership of the closed set | IFC section 7 does not define them |
| Audit failure split | `Rejected` gives `StoreError::AuditUnavailable`; `Unavailable` and unclassified failures give `StoreError::StoreUnavailable` | FD 6.6 (conductor) |
| `StoreError::Persistence` | unit variant with no database text | T-09, no leakage through `Debug` |
| Deferred trait members | `list` (WU-08), `fence_older_epochs` (WU-06) and `counters` (it would promote `serde_json`) | introduced by the units that use them |
| Concurrency 1 | tests never depend on more than one running job, so a per-instance and a global reading both pass | contract ambiguity, recorded |
| Stale-epoch claim rule | not pinned; the contract does not define it | open for WU-06 |

## 4. Gaps and limits

- No behaviour is implemented, so nothing is green for WU-03.
- `recording_is_plain_insert_and_never_touches_existing_rows` cannot tell a conflict-free `ON CONFLICT DO NOTHING` from a plain INSERT. The append-only trigger from WU-02 still blocks every UPDATE or DELETE.
- The first half of `a_claim_blocked_on_a_row_taken_by_another_transaction_does_not_claim_it_again` passes against the shell; its positive half makes it red.
- The real classification of driver errors into `Rejected` versus `Unavailable` is proven at DB level only for the healthy-connection rejection. Outage kinds are proven on constructed `DbErr` values, because a real connection loss cannot be produced deterministically here.
- Build-and-verify should again prove the target assertions by mutation once green, as it did for WU-02.
