# WU-06 implementation record

- **Unit:** WU-06 — Allocation reconciliation and retention store contracts
- **Status:** Implemented WU-06 store/adapter responsibilities; verification of all assigned store tests is green. Full suite remains blocked only by failures outside this unit (named below).
- **Scope:** TS-42 store adapter, TS-44b allocation-CAS store semantics, TS-56 full allocation owner-joined inventory, and TS-65 purge rollback. No lifecycle orchestration changes are claimed.

## WU-06-owned source changes

| File | Change |
|---|---|
| `apps/api/domain_backup/src/ports/allocation_store.rs` | Define `AllocationRecoveryRow`-based complete recovery inventory and fixed-shape allocation reconcile CAS port contract, retaining behaviorless defaults for other existing trait implementers. |
| `apps/api/domain_backup/src/infra/db_allocation_store.rs` | Implement an unfiltered allocations-to-jobs LEFT JOIN that preserves null owners and decodes nullable owner status/epoch. Implement per-allocation CAS on id/state/state_version/epoch; lost CAS returns `Lost` without audit or mutation. Applied CAS updates and writes `allocation.reconciled`/`success`/`RECONCILED` in the same transaction, with configured instance, optional job id, owner correlation id or allocation-UUID fallback. Audit failure rolls back the row change. |
| `apps/api/domain_backup/src/ports/audit.rs` | Add the owner-specified `allocation.reconciled` audit event to the closed event set. |
| `apps/api/domain_backup/src/infra/tests/audit_sink.rs` | Keep the independent closed-event spelling contract aligned with the owner-approved event. |
| `apps/api/domain_backup/src/ports/retention_store.rs` | Provide the dedicated `RetentionStore::purge_retained` port and `PurgeReport`. |
| `apps/api/domain_backup/src/infra/db_retention_store.rs` | Implement one transaction for purge: select eligible terminal jobs strictly past retention and with no non-deleted allocation; audit `retention.purged` before deletion; delete idempotency rows, deleted allocations, then jobs; retain audit rows; roll back all changes when audit/DB operations fail. No purge audit is emitted when there are no eligible jobs. |
| `apps/api/domain_backup/src/infra/tests/retention.rs` | The audit-history fixture uses a non-purge historical event so the assertion distinguishes the new purge audit; the probe mutex guard ends before subsequent awaits. |

`apps/api/domain_backup/src/infra/tests/fs_support.rs` has no remaining diff. The worktree already contains WU-05 edits to `apps/api/domain_backup/src/lifecycle/test_support.rs` (registration-delay helper for the worker deadline test); that unrelated change was not introduced or further edited by WU-06. `apps/api/domain_backup/src/lifecycle/**` implementation and tests are excluded from WU-06 scope.

## Verification summary

The red evidence named four WU-06 implementation defects: empty recovery inventory, applied reconciliation CAS incorrectly lost, zero purge count, and TS-65's required audit call counter at zero. Green checks confirmed the corresponding store implementations now reach the expected assertions, including raw allocation/audit state remaining unchanged after a rejected allocation audit and raw jobs/allocations/idempotency/audit unchanged after the TS-65 retention audit rejection.

Full command, pass counts, baseline comparison, and residual non-WU-06 failures are recorded in `green-verification-report.md` and `regression-report.md`.
