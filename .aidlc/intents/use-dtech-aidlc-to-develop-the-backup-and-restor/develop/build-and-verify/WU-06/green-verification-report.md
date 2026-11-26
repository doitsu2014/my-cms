# WU-06 green verification report

- **Unit:** WU-06 — Allocation reconciliation and retention store contracts
- **Status:** WU-06-owned store tests passed in focused Docker-backed runs while Docker was available; subsequent replay attempts were blocked when Docker Desktop became unable to start. The full workspace also has WU-16-owned failures. No claim of phase-gate approval.
- **Environment:** Windows 11 Pro, Cargo/rustc 1.100.0; Docker prerequisite `docker info` passed; PostgreSQL 15 containers started successfully for focused ignored tests.
- **Baseline:** Reused unmodified from [Develop baseline](../../baseline.md), recorded 2026-11-22: normal Rust suite 290 passed / 0 failed / 39 ignored; Docker-ignored suite 39 passed / 0 failed. Baseline tests run identities remain unchanged.

## Assigned WU-06 test results

| Owned test/assertion | Result | Observed evidence |
|---|---|---|
| TS-56 `ts56_allocation_inventory_preserves_nullable_orphan_owners` | **PASS**, 1/1 | Full unfiltered set includes all 3 seeded allocation ids. Owner status/epoch is decoded for failed and incomplete jobs; the NULL-job orphan is retained with null owner fields. A non-null missing owner cannot be seeded because the schema FK prevents it. Log: `evidence/observed-green-wu06-ts56.log` (exact successful excerpt captured; later replay log records the Docker failure before assertions). |
| TS-44b applied `allocation_reconciliation_cas_updates_state_and_audits_atomically` | **PASS**, 1/1 | Applied expected-state/version/epoch CAS returns Applied; raw allocation is `release_pending`, version 3; one audit has event `allocation.reconciled`, outcome `success`, reason `RECONCILED`, configured instance, null actor/job for orphan, allocation UUID correlation fallback; probe confirms caller transaction and old state invisible outside transaction. Log: `evidence/observed-green-wu06-cas.log` (exact successful excerpt captured; later replay log records the Docker failure before assertions). |
| TS-44b lost `lost_allocation_reconciliation_cas_has_no_audit_or_state_change` | **PASS**, 1/1 | CAS returns Lost; raw allocation remains `written`, version 2, and no audit event is appended. Log: `evidence/observed-green-wu06-cas.log` (exact successful excerpt captured; later replay log records the Docker failure before assertions). |
| TS-44b rejected audit `reconciliation_audit_failure_rolls_back_allocation_and_audit` | **PASS**, 1/1 | Rejecting `WriteThenFailSink` is reached. The test verifies the raw allocation and audit rows equal their pre-call snapshots after rejection. Log: `evidence/observed-green-wu06-cas-audit-rollback.log` (exact successful excerpt captured; later replay log records Docker startup failure before assertions). |
| TS-42 adapter `retention_purge_orders_eligible_rows_and_retains_audit_and_ineligible_rows` | **PASS**, 1/1 | Purges 2 eligible terminal jobs, 2 idempotency rows, 1 deleted allocation; retains queued/running jobs and all jobs with unresolved allocations; purge audit is written first in same transaction, actual delete order is idempotency -> deleted allocations -> jobs, history remains, repeat is idempotent. Log: `evidence/observed-green-wu06-retention-order.log` (exact successful excerpt captured; later replay log records the Docker failure before assertions). |
| TS-65 store `retention_purge_audit_failure_rolls_back_every_table` | **PASS**, 1/1 | Explicit `retention.purged` sink invocation counter equals exactly one before the refusal is assessed; rejection is observed and raw job/allocation/idempotency/audit state equals pre-call snapshots, with no purge audit row. Log: `evidence/observed-green-wu06-retention-audit-rollback.log` (exact successful excerpt captured; later replay log records the Docker failure before assertions). |

## Additional checks

| Check | Result | Evidence |
|---|---|---|
| `cargo fmt --manifest-path apps/api/Cargo.toml -- --check` | Exit 0; captured after all source changes | `evidence/green-wu06-cargo-fmt.log` |
| `cargo clippy --manifest-path apps/api/Cargo.toml -p domain_backup --all-targets` | Exit 0 after the `needless_borrow` fix; the final captured run has only baseline JobStore/WU-16 lifecycle warnings. | `evidence/green-wu06-cargo-clippy.log` |
| `dtech-aidlc ids check` | Exit 0, `ok: true`, no unregistered identifiers | `evidence/green-wu06-ids-check.log` |

## Regression test results and ownership boundary

- `cargo test --manifest-path apps/api/Cargo.toml -p domain_backup --no-fail-fast`: **116 passed, 13 failed, 129 ignored**, exit 101. The 13 failures are lifecycle tests in the pre-existing/unowned `lifecycle/lifecycle_tests.rs`; each belongs to WU-16, not WU-06. Details and exact test identities are in `regression-report.md` and `evidence/regression-wu06-domain-backup.log`.
- `cargo test --manifest-path apps/api/Cargo.toml --workspace --no-fail-fast`: **253 passed across the other workspace crates; domain_backup 116 passed / 13 failed / 129 ignored** (369 passed total, 13 failed, 129 ignored), exit 101. All 13 current failures are WU-16 lifecycle tests. Full output: `evidence/regression-wu06-workspace-tests.log`.
- `cargo test --manifest-path apps/api/Cargo.toml --workspace --no-fail-fast -- --ignored`: **126 passed, 3 failed** in `domain_backup`, with the remaining ignored tests passing (other crates' ignored tests pass). The 3 failures are WU-16-owned JobStore epoch advance, job-fence CAS, and deadline listing tests; none is WU-06-owned. Full output: `evidence/regression-wu06-workspace-ignored.log`.

### Trace table

| Story | Criterion | Function | Test | Result |
|---|---|---|---|---|
| US-02 / US-03 / US-04 / US-06 | AC-003 S3 (store inventory contract) | FN-27 support / allocation recovery inventory | `infra::tests::reconciliation::ts56_allocation_inventory_preserves_nullable_orphan_owners` | passed |
| US-02 / US-06 | AC-005 S2 (allocation audit persistence) | FN-27 support / allocation reconcile CAS | `infra::tests::reconciliation::allocation_reconciliation_cas_updates_state_and_audits_atomically` | passed |
| US-02 / US-06 | AC-005 S2 (lost CAS has no side effects) | FN-27 support / allocation reconcile CAS | `infra::tests::reconciliation::lost_allocation_reconciliation_cas_has_no_audit_or_state_change` | passed |
| US-02 / US-06 | AC-005 S3 (audit refusal rolls back) | FN-27 support / allocation reconcile CAS | `infra::tests::reconciliation::reconciliation_audit_failure_rolls_back_allocation_and_audit` | passed |
| US-02 / US-03 / US-04 / US-06 | AC-002 S2 / AC-003 S3 (retention store) | FN-33 persistence adapter | `infra::tests::retention::retention_purge_orders_eligible_rows_and_retains_audit_and_ineligible_rows` | passed |
| US-02 / US-06 | AC-005 S3 (purge refusal rolls back) | FN-33 persistence adapter | `infra::tests::retention::retention_purge_audit_failure_rolls_back_every_table` | passed |

WU-16 behavior intentionally remains unimplemented here: runtime start/recovery, job epoch fencing/deadline listing, filesystem-to-ledger lifecycle reconciliation, cleanup/retry, truncated inventory fail-closed handling, missing-file lifecycle scans and sweep integration. No WU-16 acceptance is claimed.
