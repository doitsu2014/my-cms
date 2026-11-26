# GVR-04 Green verification report: WU-04

- **ID:** GVR-04 (WU-04)
- **Stage:** Develop / build-and-verify, work unit WU-04; not the release-candidate gate.
- **Status:** WU-04 focused verification passed on Windows 11, refreshed 2026-11-24. Full workspace regression is blocked by already-observed later-unit failures (see `regression-report.md`).
- **Implementation:** `implementation.md`.
- **Design and ID sources:** `develop/implementation-design/{work-units.json,implementation-plan.md,interfaces-and-contracts.md,functional-design.md,test-strategy.md,ids.json}`.
- **Historical red evidence:** `develop/red-tests/WU-04/{failing-tests.md,red-evidence.md}`. Red replay skipped for this refresh; no behavior or tests changed.
- **Environment:** `dtech-aidlc config prerequisite check` exit 0; Docker prerequisite passed (`docker info` exit 0). `cargo --version` / `rustc --version` report 1.100.0; Docker server 29.7.2.

## Focused checks

Commands run from `apps/api`; complete outputs in `develop/build-and-verify/evidence/`:

| Test level / check | Command | Exit | Observed |
|---|---|---:|---|
| Filesystem fixture storage | `cargo test -p domain_backup infra::tests::fs_fixture_storage:: --no-fail-fast` | 0 | 30 passed, 0 failed, 0 ignored |
| Namespace lock | `cargo test -p domain_backup infra::tests::fs_namespace_lock:: --no-fail-fast` | 0 | 11 passed, 0 failed, 0 ignored; includes parent/child process exclusivity checks |
| Allocation store, normal mode | `cargo test -p domain_backup infra::tests::allocation_store:: --no-fail-fast` | 0 | 0 passed, 28 ignored (Docker-only tests are correctly ignored in normal mode) |
| Allocation store, Docker mode | `cargo test -p domain_backup infra::tests::allocation_store:: --no-fail-fast -- --ignored` | 0 | 28 passed, 0 failed, 0 ignored |
| Format | `cargo fmt -- --check` | 0 | Formatting check passed |
| Lint | `cargo clippy --all-targets` | 0 | Command passed with workspace warnings; see limitations and regression record |
| Identifier registry | `dtech-aidlc ids check` | 0 | `ok: true`, WU-04 identifiers registered; no unregistered identifiers |

Logs: `refresh-wu04-focused-fs-fixture.log`, `refresh-wu04-focused-fs-lock.log`, `refresh-wu04-focused-allocation-normal.log`, `refresh-wu04-focused-allocation-ignored.log`, `refresh-wu04-fmt.log`, `refresh-wu04-clippy.log`, and `refresh-wu04-ids-check.log`.

## Workspace checks and scope boundary

- `cargo test --no-fail-fast`: exit 101; 13 failing `lifecycle::lifecycle_tests::*` tests (WU-16 scope), with 116 passing and 128 ignored in `domain_backup`'s library target. Other workspace targets in the log passed. These same 13 lifecycle failures appear in the pre-WU-04-refresh `refresh-wu03-workspace-normal.log`; WU-04 source/tests were not changed.
- `cargo test --no-fail-fast -- --ignored`: exit 101; 7 failing `infra::tests::reconciliation::*` tests (WU-06 store scope), with 121 passing in the `domain_backup` library target. These same 7 reconciliation failures appear in `refresh-wu03-workspace-ignored.log`; WU-04 source/tests were not changed.
- The failures are real workspace failures, not treated as passes. Their existing and later-unit ownership is detailed in the regression report. The full regression gate therefore remains blocked despite the WU-04 focused pass.
- The bounded filesystem inventory portion of TS-56 is owned here and passed in the focused storage suite. Complete nullable owner-joined allocation inventory is WU-06; lifecycle behavior for truncated inventory is WU-16 and is not claimed as passed here.

## Quality evidence

| Requirement | WU-04 evidence/result |
|---|---|
| TS-19 path/junction/reparse safety | Focused filesystem suite passed. Windows file-symlink tests exercise their visible skip path if symlink privilege is unavailable; no Unix symlink or permission claim is made from Windows. |
| TS-30 allocation budgets | 28 allocation-store integration tests passed with Docker, including per-allocation and total-budget enforcement, concurrency, cleanup retry behavior, and audit coupling. Deadline behavior is outside WU-04. |
| TS-31 storage privacy | Windows focused tests passed where applicable. Unix 0700/0600 checks are not compiled on Windows; Windows owner-only ACL verification is the recorded accepted residual for synthetic-only U1. |
| TS-32 lock | 11 focused tests passed, including two-process lock exclusivity. Gateway startup and admission ordering are outside WU-04. |
| TS-56 bounded inventory | Filesystem bound/truncation producer tests passed. Later allocation join/reconciler tests are outside WU-04 and currently fail as WU-06/WU-16 work. |

No gateway composition, deployment, web verification, end-to-end or performance checks were in WU-04 scope. No codebase-map entry was changed: the new crate is still not composed in the gateway; integration is a later unit.

## Trace table

| Story identifier | Criterion identifier | Function identifier | Test name(s) | Result |
|---|---|---|---|---|
| US-02, US-03, US-04, US-06 | AC-003 S3 | FN-21, FN-22 | Focused allocation-store and fixture-storage suites; see logs above | passed |
| US-02, US-06 | AC-002 S2 | FN-21, FN-23 | Focused `allocation_store` Docker suite (28 passed) | passed |
| US-02, US-03, US-04 | AC-006 S2 | FN-21, FN-22, FN-24 | Budget, bounded-inventory, and namespace-lock focused suites | passed |

## Known residuals

Windows cannot exercise the `cfg(unix)` TS-31 mode checks. File-symlink tests can skip when the host lacks privilege. The owner-accepted Windows ACL residual and the remaining race/locking limitations are listed in `known-limitations.md`. The full-suite later-unit failures mean the intent-wide regression result is blocked; WU-04 focused status alone does not approve the release candidate.
