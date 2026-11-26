# KL-05 Known limitations: WU-05

- **ID:** KL-05 (WU-05)
- **Stage:** Develop / build-and-verify.
- **Status:** WU-05 verification limitations and follow-ups; earlier items are recorded in [WU-04 known limitations](../WU-04/known-limitations.md).

| # | Limitation | Impact | Owner or next step |
|---|---|---|---|
| L-01 | Windows-only verification in this run | Unix 0700/0600 permission tests and file-symlink privilege cases cannot be fully verified on this Windows host. | Run ignored and non-ignored Rust suites on Ubuntu CI before the release-candidate gate (carried from WU-04). |
| L-02 | Test double simulates registration delay by advancing FakeClock and paused Tokio time | The deterministic regression proves the capture budget is recomputed after registration, but it does not measure actual PostgreSQL latency. | The DB worker integration suite exercises the real allocation store and passes; no latency target is claimed. |
| L-03 | `ClaimPolicy` remains a WU-05 port/double | WU-05 verifies fail-closed policy behavior, but production policy wiring is not present yet. | WU-08 implements the real policy port and TS-05b. |
| L-04 | Runtime, reconciler and sweeper integration is not yet composed | WU-05 leaves a job `running` after terminal audit/store retries are exhausted; it returns `Stalled` and retains the allocation for recovery. | WU-06 implements runtime start, reconciliation, cleanup and sweeper behavior. |
| L-05 | No real capture, staging, packaging, download or restore | The only adapter writes a synthetic fixture; no production backup data is handled. | Explicitly out of U1 scope and not authorized. |
| L-06 | Test-strategy audit actor wording differs from IFC | C-1 follows IFC section 7: worker-generated `job.running`, `job.incomplete` and `job.failed` are system events with NULL actor; the user actor is recorded on acceptance/job row. | Reconcile wording in a later design touch-up; tests pin the approved WU-05 interpretation. |
| L-07 | FD 7.1 diagram also draws queued -> failed `INSTANCE_NOT_ALLOWED` | WU-05 follows IFC 10.1, which rechecks policy after claim and fails running -> failed. | Reconcile the diagram during later design maintenance. |
| L-08 | WU-05 is uncommitted at this verification point | Implementation and build evidence remain local. | Commit/push only when the owner asks. |

## Rollback

WU-05 production behavior is confined to `apps/api/domain_backup` and has no gateway composition yet. Revert the WU-05 source and its `Cargo.toml` change from commit `53a9570` onward to roll back this unit. The database schema changes are from WU-02 and are not part of WU-05. No migrations or lockfile changes were made here.

## Verification refresh (Observed, 2026-11-24)

- WU-05 focused worker tests (27), fixture tests (5) and Docker-backed worker integration tests (15) passed; `cargo fmt -- --check`, workspace `cargo clippy --all-targets`, and `dtech-aidlc ids check` also passed. Raw logs: `../evidence/refresh-wu05-*.log`.
- The full-workspace normal and ignored test commands are currently **not green** (exit 101). They fail only in new WU-17/18/19 lifecycle tests (13 names) and new WU-06 reconciliation-store tests (7 names); none of the 290 normal or 39 ignored baseline-passing tests failed. See [regression-report.md](regression-report.md) for exact test names and set comparison. These failures belong to later work units, not the WU-05 focused suite.
- No further WU-05-specific code or test delta beyond the already documented implementation and deadline regression was found. No source/test files were changed during this refresh.
