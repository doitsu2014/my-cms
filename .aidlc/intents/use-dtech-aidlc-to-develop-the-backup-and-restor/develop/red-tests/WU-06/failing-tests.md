# WU-06 failing tests — store and adapter contracts

- **Status:** WU-06 store-level red coverage recorded against inert compile-time scaffolding; no real WU-06 store behavior was implemented.
- **Scope owned by this writer:** WU-06 store-level TS-42 retention-purge adapter contract; TS-44b allocation CAS audit persistence; TS-56 complete allocation LEFT JOIN inventory; and TS-65 retention-store rollback only. This is not full TS-42, TS-56, or TS-65 coverage.
- **Source tests:** `apps/api/domain_backup/src/infra/tests/reconciliation.rs`, `infra/tests/retention.rs`, `infra/tests/alloc_support.rs`; registration in `infra/tests/mod.rs`; inert compile-time defaults in allocation/retention ports and the DB adapter.
- **Baseline:** Reused unchanged from [intent baseline](../../baseline.md); not rerun or edited.
- **Lifecycle scope contamination:** Before this writer's changes, the working tree contained untracked `apps/api/domain_backup/src/lifecycle/cleaner.rs`, `lifecycle/reconciler.rs`, `lifecycle/runtime.rs`, `lifecycle/sweeper.rs`, `lifecycle/lifecycle_tests.rs` and modified `lifecycle/mod.rs`. They are pre-existing, unowned and unmodified by this WU-06 writer. No WU-16 lifecycle coverage is claimed. Their status and SHA-256 hashes were recorded before and after; hashes and status were unchanged.

## WU-06 store tests

| Test | Contract exercised | Current result |
|---|---|---|
| `ts56_allocation_inventory_preserves_nullable_orphan_owners` | TS-56 store portion: complete unfiltered allocation id set, LEFT JOIN owner status/epoch, NULL-owner orphan | **RED:** 1 run, 1 failed at exact id-set assertion: actual empty set vs 3 seeded allocation IDs. Owner-field checks follow and were not reached. A non-NULL missing owner cannot be seeded due to the schema FK; the representable NULL-owner orphan case is tested. |
| `allocation_reconciliation_cas_updates_state_and_audits_atomically` | TS-44b store portion: applied allocation CAS and exact `allocation.reconciled` fields plus same-transaction audit | **RED:** 1 run, 1 failed: got `Lost`, expected `Applied`. The test's explicit guard confirms the audit-field assertions were not reached and remain deferred to GREEN. |
| `lost_allocation_reconciliation_cas_has_no_audit_or_state_change` | TS-44b lost CAS no-mutation/no-audit behavior | **STRUCTURAL PASS ONLY:** 1 run, 1 passed against the inert default `Lost`; this is not behavior evidence. |
| `reconciliation_audit_failure_rolls_back_allocation_and_audit` | TS-44b rejection and raw allocation/audit rollback assertions | **RED, incomplete path:** 1 run, 1 failed at `failed.is_err()`. The current default returns `Lost` without invoking the rejection sink. Rollback is not proved. |
| `retention_purge_orders_eligible_rows_and_retains_audit_and_ineligible_rows` | TS-42 store/adapter portion: eligible purge counts, exclusions, audit-first probe, delete order and audit retention | **RED:** 1 run, 1 failed at expected purged job count 2, observed 0. Later assertions were not reached and are not claimed observed. |
| `retention_purge_audit_failure_rolls_back_every_table` | TS-65 store portion: audit rejection should refuse purge and preserve raw job/allocation/idempotency/audit state | **STRUCTURAL/INCOMPLETE RED:** 1 run, 1 failed because the explicit `retention.purged` invocation counter was 0, expected 1. The placeholder returned before invoking the sink, so the rejection path and all rollback assertions remain unreachable. No rollback was proved. |

For TS-65, the test's rejecting sink increments an `Arc<AtomicUsize>` exactly when it receives `AuditEventKind::RetentionPurged`, then returns `AuditError::Rejected`. The required count is exactly one before the test can claim the audit refusal path was exercised. In this red run it remained zero; therefore this is a structural/incomplete path, not evidence that rollback failed or passed.

## Commands and observed results

All focused commands ran from repository root against `apps/api/Cargo.toml`, with `-- --ignored --exact ... --nocapture` for individual PostgreSQL tests (or the two-test CAS filter), Docker/PostgreSQL 15 available. Complete stdout/stderr, command, exit code and timing are saved under `../evidence/`.

| Evidence log | Command filter | Result |
|---|---|---|
| `wu06-focused-ts56.log` | `ts56_allocation_inventory_preserves_nullable_orphan_owners` | Exit 101; 1 run, 1 failed at complete allocation id set (`[]` vs 3 IDs). |
| `wu06-focused-cas-applied.log` | `allocation_reconciliation_cas_updates_state_and_audits_atomically` exact | Exit 101; 1 run, 1 failed, got `Lost`, expected `Applied`; audit fields not reached. |
| `wu06-focused-cas-lost.log` | `lost_allocation_reconciliation_cas_has_no_audit_or_state_change` exact | Exit 0; 1 run, 1 passed structurally against inert default, not behavior evidence. |
| `wu06-focused-cas-audit-failure.log` | `reconciliation_audit_failure_rolls_back_allocation_and_audit` exact | Exit 101; 1 run, 1 failed because the placeholder did not invoke the failing sink; rollback not proved. |
| `wu06-focused-retention-order.log` | `retention_purge_orders_eligible_rows_and_retains_audit_and_ineligible_rows` exact | Exit 101; 1 run, 1 failed, expected purged count 2, got 0. |
| `wu06-focused-retention-audit-failure.log` | `retention_purge_audit_failure_rolls_back_every_table` exact | Exit 101; 1 run, 1 failed, required sink counter was 0 (expected 1); no rollback proved. |
| `wu06-focused-ts65-audit-refusal.log` | TS-65 retention audit refusal exact run | Exit 101; 1 run, 1 failed at sink counter 0 (expected 1); no rollback proved. This is the same test rerun after adding the counter, retained as separate evidence for the explicit TS-65 check. |
| `wu06-final-fmt-check.log` | `cargo fmt --manifest-path apps/api/Cargo.toml -- --check` | Exit 0; formatting check passed. |
| `wu06-final-ids-check.log` | `dtech-aidlc ids check` | Exit 0; `ok: true`, no unregistered identifiers. |

Previously recorded broad logs are not treated as WU-06-only evidence: `build-and-verify/evidence/wu06-store-red-focused.log` and the pre-existing `red-wu06-cargo-test-ignored.log` include unrelated WU-16 reconciliation/job tests and failures. No broad run is represented here as an exclusively WU-06 run.

## Scope boundaries and contamination

- WU-06 coverage here is restricted to: TS-42 **purge-adapter/store portion**, TS-44b **allocation CAS audit-store portion**, TS-56 **complete allocation owner-joined inventory-store portion**, and TS-65 **retention purge rollback-store portion**.
- No full TS-42 lifecycle injection/sweeper integration, TS-56 truncation/fail-closed lifecycle behavior, TS-65 deletion-claim or deletion-commit lifecycle behavior, or WU-16 behavior is claimed.
- Pre-existing untracked WU-16 files and the modified lifecycle module wiring listed above are unowned scope contamination. This writer did not change them. Before/after file hashes and path status were identical; see `../evidence/wu06-lifecycle-before-after.log`.
- No test baseline rerun; unchanged intent baseline remains authoritative. No workflow state/audit, WU-01..WU-05 or lifecycle paths were changed by this writer.
