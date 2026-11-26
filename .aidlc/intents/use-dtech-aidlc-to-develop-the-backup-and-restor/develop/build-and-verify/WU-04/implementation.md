# IMPL-04 Implementation record: WU-04

- **ID:** IMPL-04 (WU-04)
- **Stage:** Develop / build-and-verify, builder-verifier
- **Status:** Implementation unchanged; verification refreshed 2026-11-24. This is the WU-04 unit result, not the release-candidate gate.
- **Scope:** Allocation ledger, fixture storage, and namespace lock (FN-21..FN-24; TS-19, TS-30 budget coverage, TS-31, TS-32 lock coverage, and the bounded filesystem producer portion of TS-56).
- **Inputs:** `develop/implementation-design/work-units.json`, `implementation-plan.md`, `interfaces-and-contracts.md`, `functional-design.md`, `test-strategy.md`, `ids.json`; `develop/red-tests/WU-04/{failing-tests.md,red-evidence.md}`; recorded baseline at `develop/baseline.md`.

## Implementation

Production paths are under `apps/api/domain_backup/src/`:

- `infra/db_allocation_store.rs`: durable allocation ledger and budget enforcement.
- `infra/fs_fixture_storage.rs`: opaque-key fixture writes, deletion, classification, bounded inventory.
- `infra/fs_namespace_lock.rs`: exclusive lock file outside the storage namespace.
- `ports/allocation_store.rs`, `ports/fixture_storage.rs`, and `ports/namespace_lock.rs`: WU-04 port contracts and key/classification helpers.

No dependency or manifest change was made for WU-04. The adapters are not composed into the gateway; gateway composition is a later unit. Implementation details and original design decisions D1–D6 and Dev-1–Dev-5 are retained in the historical red/green evidence linked below.

## Verification refresh (Observed, Windows 11; Rust 1.100.0; Docker 29.7.2)

No source or test files were changed. The WU-04 red replay was intentionally skipped as instructed; the historical red record remains `develop/red-tests/WU-04/red-evidence.md`. Current focused reruns passed:

- `cargo test -p domain_backup infra::tests::fs_fixture_storage:: --no-fail-fast`: exit 0; 30 passed, 0 failed.
- `cargo test -p domain_backup infra::tests::fs_namespace_lock:: --no-fail-fast`: exit 0; 11 passed, 0 failed, including both real-process lock tests.
- `cargo test -p domain_backup infra::tests::allocation_store:: --no-fail-fast`: exit 0; 0 passed, 28 ignored because DB tests require Docker.
- `cargo test -p domain_backup infra::tests::allocation_store:: --no-fail-fast -- --ignored`: exit 0; 28 passed, 0 failed (Docker/PostgreSQL).

Full logs are under `develop/build-and-verify/evidence/refresh-wu04-*`.

Workspace `cargo test --no-fail-fast` and `cargo test --no-fail-fast -- --ignored` were also run; both return exit 101 due to later-unit WU-06/WU-16 tests. The same lifecycle and reconciliation failure set appears in the earlier `refresh-wu03-workspace-*` evidence, before this WU-04 refresh. The WU-04 filesystem/ledger/lock suites pass independently, and exact-name comparison confirms every passing baseline test still passes. These later-unit failures are disclosed in the regression report; they are not represented as WU-04 passes or introduced by a WU-04 source change.

Other checks: `cargo fmt -- --check` exit 0; `cargo clippy --all-targets` exit 0; `dtech-aidlc ids check` exit 0 with no unregistered identifiers. Clippy reports warnings in the workspace, including pre-existing warnings outside `domain_backup` and current warnings in other, later-unit `domain_backup` lifecycle/store contracts; no lint finding was established as a WU-04 defect.

## Trace to WU-04 scope

| Acceptance criterion | WU-04 function IDs | Test evidence | Result |
|---|---|---|---|
| AC-003 S3: synthetic-only fixture allocation/storage boundary | FN-21, FN-22, FN-23, FN-24 | Focused allocation store (28 Docker tests), fixture storage (30 tests), lock (11 tests); historical WU-04 red-to-green record | Focused suites pass |
| AC-002 S2: allocation budgets and cleanup ledger behavior | FN-21, FN-23 | Allocation-store budget, CAS, audit, retry and orphan tests | 28 Docker tests pass |
| AC-006 S2: bounded synthetic storage and safe ownership constraints | FN-21, FN-22 | Allocation budget tests; inventory max-entry/truncation and kind tests; lock exclusivity | Focused suites pass |

TS-56 scope split is respected: this refresh verifies the bounded filesystem inventory producer in WU-04. The owner-joined allocation inventory belongs to WU-06 and fail-closed reconciliation belongs to WU-16.

## Evidence links

- Red: `develop/red-tests/WU-04/failing-tests.md` and `red-evidence.md` (historical; red rerun skipped for this refresh).
- Refresh focused/full logs and workspace checks: `develop/build-and-verify/evidence/refresh-wu04-*`.
- Regression comparison and failure attribution: `regression-report.md`.
- Windows and residual constraints: `known-limitations.md`.
