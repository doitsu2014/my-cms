# WU-06 red evidence — store and adapter contracts (historical pre-build snapshot)

- **Status:** This artifact records the historical WU-06 red-tests run against inert compile-time defaults, before the separately recorded WU-06 implementation. It is not the current implementation status: the store adapters have since been implemented and the assigned focused tests have recorded green PostgreSQL runs in `../../build-and-verify/WU-06/green-verification-report.md`. WU-06 is still incomplete because the strict full-workspace regression remains red; the approved sequence runs WU-17/18/19 first, then reruns WU-06's full normal and ignored workspace suites before any complete report.
- **Ownership boundary:** This evidence covers only WU-06's store portion of TS-42, TS-44b, TS-56 and TS-65. It does not claim full TS-42, TS-56 or TS-65 coverage and does not claim WU-17/18/19 lifecycle verification.
- **Environment:** Docker prerequisite passed via `dtech-aidlc config prerequisite check`; Docker is available and PostgreSQL 15 testcontainers started in the focused ignored tests.
- **Baseline:** Reused unchanged from [intent baseline](../../baseline.md); no baseline rerun or rewrite.
- **IDs and formatting:** `dtech-aidlc ids check` passed (`ok: true`, no unregistered IDs). `cargo fmt --manifest-path apps/api/Cargo.toml -- --check` passed after formatting the touched Rust tests.

## Focused red results

Complete outputs are preserved in the named logs under `../evidence/`. Each focused test command used the exact filter and `-- --ignored --exact <fully-qualified test> --nocapture` (except the applied-CAS set command, which intentionally filters the two `allocation_reconciliation_cas` cases).

| Test | Run result | What was actually observed |
|---|---|---|
| TS-56 `ts56_allocation_inventory_preserves_nullable_orphan_owners` | `wu06-focused-ts56.log`, exit 101, 1 run/1 failed | Runtime failure at complete unfiltered allocation-ID assertion: adapter returned empty set vs 3 expected IDs. Owner status/epoch checks were not reached. Schema FK disallows seeding a non-NULL missing-job reference; the store test uses the representable NULL-owner orphan. |
| TS-44b applied `allocation_reconciliation_cas_updates_state_and_audits_atomically` | `wu06-focused-cas-applied.log`, exit 101, 1 run/1 failed | Runtime failure: CAS returned `Lost`, expected `Applied`. A guard explicitly says the exact `allocation.reconciled` event/actor/instance/job/allocation/operation/digest/correlation/reason assertions below were not reached; they are deferred to GREEN. |
| TS-44b lost `lost_allocation_reconciliation_cas_has_no_audit_or_state_change` | `wu06-focused-cas-lost.log`, exit 0, 1 run/1 passed | Structural pass against inert default `Lost` only; not evidence of implemented store behavior. |
| TS-44b rejection `reconciliation_audit_failure_rolls_back_allocation_and_audit` | `wu06-focused-cas-audit-failure.log`, exit 101, 1 run/1 failed | Placeholder returned `Lost` without invoking the rejecting sink; `failed.is_err()` failed. Raw pre/post rollback assertions were not reached. No rollback proved. |
| TS-42 `retention_purge_orders_eligible_rows_and_retains_audit_and_ineligible_rows` | `wu06-focused-retention-order.log`, exit 101, 1 run/1 failed | Runtime failure: expected 2 eligible jobs purged, observed 0. The test's downstream audit-first probe, exclusion, deletion-order, retained-audit and repeat-idempotency assertions were not reached. |
| TS-65 `retention_purge_audit_failure_rolls_back_every_table` | `wu06-focused-ts65-audit-refusal.log`, exit 101, 1 run/1 failed | The rejecting sink has an `Arc<AtomicUsize>` counter that increments exactly on `retention.purged`, then returns `AuditError::Rejected`. Counter was 0, expected 1. Placeholder did not invoke the sink. The explicit invocation assertion stopped the test before `result.is_err()` and raw job/allocation/idempotency/audit rollback assertions. This is a structural/incomplete path, not rollback red evidence; no rollback was proved. |

`wu06-focused-retention-audit-failure.log` is the first pre-counter attempt for TS-65; exit 101 at the test's then-current `result.is_err()` assertion because the placeholder did not call the sink. It is superseded by the counter-guarded rerun above and is retained as complete run history, not as proof of audit refusal or rollback.

## Full command records

| Evidence log | Exact invocation / check | Outcome |
|---|---|---|
| `wu06-focused-ts56.log` | `cargo test --manifest-path apps/api/Cargo.toml -p domain_backup ts56_allocation_inventory_preserves_nullable_orphan_owners -- --ignored --exact infra::tests::reconciliation::ts56_allocation_inventory_preserves_nullable_orphan_owners --nocapture` | Exit 101; intended runtime id-set assertion failed. |
| `wu06-focused-cas.log` | `cargo test --manifest-path apps/api/Cargo.toml -p domain_backup allocation_reconciliation_cas -- --ignored --nocapture` | Exit 101; 2 run, applied case failed `Lost` vs `Applied`; lost case passed structurally. |
| `wu06-focused-retention-order.log` | `cargo test --manifest-path apps/api/Cargo.toml -p domain_backup retention_purge_orders_eligible_rows_and_retains_audit_and_ineligible_rows -- --ignored --exact infra::tests::retention::retention_purge_orders_eligible_rows_and_retains_audit_and_ineligible_rows --nocapture` | Exit 101; expected 2 purged jobs, got 0. |
| `wu06-focused-retention-audit-failure.log` | `cargo test --manifest-path apps/api/Cargo.toml -p domain_backup retention_purge_audit_failure_rolls_back_every_table -- --ignored --exact infra::tests::retention::retention_purge_audit_failure_rolls_back_every_table --nocapture` | Exit 101; initial attempt, placeholder returned success without sink invocation. |
| `wu06-focused-cas-applied.log` | `cargo test --manifest-path apps/api/Cargo.toml -p domain_backup allocation_reconciliation_cas_updates_state_and_audits_atomically -- --ignored --exact infra::tests::reconciliation::allocation_reconciliation_cas_updates_state_and_audits_atomically --nocapture` | Exit 101; exact applied CAS failed as above. |
| `wu06-focused-cas-lost.log` | `cargo test --manifest-path apps/api/Cargo.toml -p domain_backup lost_allocation_reconciliation_cas_has_no_audit_or_state_change -- --ignored --exact infra::tests::reconciliation::lost_allocation_reconciliation_cas_has_no_audit_or_state_change --nocapture` | Exit 0; structural-only lost CAS pass. |
| `wu06-focused-cas-audit-failure.log` | `cargo test --manifest-path apps/api/Cargo.toml -p domain_backup reconciliation_audit_failure_rolls_back_allocation_and_audit -- --ignored --exact infra::tests::reconciliation::reconciliation_audit_failure_rolls_back_allocation_and_audit --nocapture` | Exit 101; sink not reached, rollback not proved. |
| `wu06-focused-ts65-audit-refusal.log` | `cargo test --manifest-path apps/api/Cargo.toml -p domain_backup retention_purge_audit_failure_rolls_back_every_table -- --ignored --exact infra::tests::retention::retention_purge_audit_failure_rolls_back_every_table --nocapture` | Exit 101; counter 0, expected 1; incomplete path, no rollback proved. |
| `wu06-final-fmt-check.log` | `cargo fmt --manifest-path apps/api/Cargo.toml -- --check` | Exit 0. |
| `wu06-final-ids-check.log` | `dtech-aidlc ids check` | Exit 0; `ok: true`; no unregistered IDs. |

Each full log includes the actual invocation, work directory, stdout/stderr, exit code and timestamps. Earlier broad WU-06-labelled logs also include unrelated lifecycle/job tests and must not be read as WU-06-only evidence.

## WU-17/18/19 lifecycle paths: pre-existing contamination, not owned

At entry, status showed untracked `apps/api/domain_backup/src/lifecycle/{cleaner.rs,lifecycle_tests.rs,reconciler.rs,runtime.rs,sweeper.rs}` and modified `lifecycle/mod.rs`. These were treated as unowned pre-existing worktree content: this writer did not edit or delete them. Hashes and path status were captured before and after and remained identical in `../evidence/wu06-lifecycle-before-after.log`. The WU-06 changed-file scope does not include any lifecycle paths.

## Historical red-stage handoff (superseded by the recorded green stage)

At the time of this red run, the next intended activity was implementing the store-owned portions: TS-56 complete LEFT JOIN inventory; TS-44b applied/lost audited allocation CAS; TS-42 purge adapter ordering, exclusions, audit retention and idempotency; and TS-65 purge rollback with the `retention.purged` invocation counter. The subsequent WU-06 build-and-verify record and focused green logs show those assigned store tests passing, including TS-65's exactly-one audit invocation and rollback assertions.

Current sequence is distinct from that historical handoff: WU-17/18/19's lifecycle work and tests are separately owned and must be handled under WU-17/18/19's own directive before WU-06's final strict regression report. WU-06 is not complete until both full normal and ignored workspace runs pass the unchanged baseline comparison; do not treat this red snapshot as current implementation status or as lifecycle evidence.
