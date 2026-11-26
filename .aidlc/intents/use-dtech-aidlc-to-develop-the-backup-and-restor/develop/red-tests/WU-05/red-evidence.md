# RE-05 Red evidence: WU-05

- **ID:** RE-05 (WU-05)
- **Stage:** Develop / red-tests (builder-verifier), unit WU-05.
- **Status:** red run recorded 2026-11-23.
- **Companion:** [failing-tests.md](failing-tests.md). Raw logs are in [../evidence/](../evidence/).
- **Baseline:** [../../baseline.md](../../baseline.md), reused.

Provenance:
- **Observed:** the conductor ran everything listed here on 2026-11-23, on Windows 11 with cargo 1.100.0, Docker and `postgres:15-alpine`.
- **Builder-reported:** nothing in this record relies only on the builder's report.

## 1. Environment and owner decisions

- **Prerequisite:** `docker` passed both in the `orchestrate next` directive and in `dtech-aidlc config prerequisite check`. The directive carried no warnings.
- **WU-05 "Blocked by" line:** the CFG-04 storage location and protection is resolved by the owner decisions in [../WU-01/red-evidence.md](../WU-01/red-evidence.md) section 1 (User-provided, project owner, 2026-11-22). The same decisions are recorded in [../WU-04/red-evidence.md](../WU-04/red-evidence.md) section 1.
- **Q-05 bounds used:**
  - job deadline 10 minutes (CFG-10);
  - staging expiry 24 h (CFG-05);
  - 256 MiB per allocation and 1 GiB total.
- **ADR-002:** no U1 job may reach `succeeded` or `recovery-required`. `no_path_ends_succeeded_or_recovery_required` asserts this over seven paths.
- **Environment:** no environment change was made. Unlike WU-04, there were no toolchain targets, image pulls or installs.

## 2. Red run (Observed)

| Command (from `apps/api`) | Exit | Result |
|---|---|---|
| `cargo fmt -p domain_backup -- --check` | 0 | formatted before the run |
| `cargo test -p domain_backup --no-fail-fast` ([log](../evidence/red-wu05-cargo-test.log)) | 101 | 84 passed, 29 failed (all new), 120 ignored; 0.30 s |
| `... -- --ignored` ([log](../evidence/red-wu05-cargo-test-ignored.log)) | 101 | 106 passed, 14 failed (all new), 113 filtered; 92 s |
| `cargo check --workspace` ([log](../evidence/red-wu05-cargo-check-workspace.log)) | 0 | existing warnings in other crates only |
| `cargo clippy -p domain_backup --all-targets` | 0 | no `domain_backup` warning |

- **Failure quality:** every one of the 43 failures panics at an assertion.
  - The left-hand values are the shell's answers: `Idle`, `Lost`, `Components([])` and `[Idle, Idle]`.
  - Seven tests report "worker finished before reaching the capture adapter: Ok(Idle)". That message is an assertion on the worker task's result, not a timeout. The default run takes 0.30 s.
  - The log shows no `unwrap`/`expect` panics, OS errors or connection errors.
- **Earlier tests:** all 82 non-Docker and 105 Docker tests from WU-01..WU-04 pass.
- **Unchanged files:** `git diff` shows that only module wiring and `Cargo.toml` changed among existing files. `apps/api/Cargo.lock` is unchanged.
- **Snapshot:** taken after formatting, for the byte-identity check at build-and-verify.

## 3. How the red was produced

- **Workflow:** one builder (sonnet), two independent Opus reviewers (spec fidelity; red integrity with the Docker suite) and a fixer (sonnet).
- **Briefing:** the CLI directive verbatim and the `ids.json` rule. It also gave an explicit ban on environment changes, allowed only crates already in the lock file, and repeated the retro rules: the fixer adds no production behaviour, and a rejected finding cites every design document searched.
- **Review outcome:** 20 findings, 6 critical or major.
  - **Applied:** 18.
  - **Partly applied:** 1. The `WorkerConfig` drift guard was added; the bounds-based constructor was rejected, with citations to IFC section 8 and TS-01.
  - **Handed to the conductor:** 1, the record corrections that went into this file.
- **Main corrections:**
  - **Wrong-reason reds:** DB audit assertions counted the `allocation.registered` row, so no implementation could pass them. They now filter job events.
  - **Timeouts as red reasons:** seven reds ran out a 30 s timeout instead of failing an assertion. `wait_entered` now selects on the worker task.
  - **Hang risk:** paused-time joins are now bounded, so a partial implementation cannot hang the suite.
  - **ALLOCATION_LIMIT:** it was tested only under an invalid configuration. A reachable total-budget case was added, both in memory and in the DB (four seeded 256 MiB rows).
  - **Races:** the sweeper double now refuses before the deadline. Both orders (sweeper first, worker timer first) and a barrier-held simultaneous claim are tested.
  - **Remaining deadline:** the timeout is the time left until `deadline_at`, not 600 s from the start of capture.
  - **Retries:** a refused terminal write stalls on every path, for both audit rejection and store outage, and keeps the allocation.
  - **Over-specified tests:** the late-worker tests now assert the property only: the row is unchanged, there is no new job audit row and no live allocation.

## 4. Decisions for build-and-verify

Conductor sign-off is given here, with the basis for each.

| Id | Decision | Basis | Conductor |
|---|---|---|---|
| C-1 | `job.running`, `job.incomplete` and `job.failed` carry `actor_sub` NULL | IFC section 7 ("NULL (system events null)"); the worker is a system actor that holds no actor token (IFC 7.4). The administrator is attributed on `job.accepted` (TS-44a) and on the job row, both joined by `job_id`. This keeps the WU-03 store mapping. | **decided**; pinned by `ts44b_job_events_use_the_system_actor`. The TS-44b row and the WU-05 text ("actor ... populated") are read as "the actor column follows IFC 7" (follow-up below). |
| C-2 | `job.incomplete` records outcome `failure` | The job produced no usable backup, and ADR-002 forbids a U1 job reading as a success. This keeps the WU-03 mapping. | **decided**; pinned by `ts44b_incomplete_event_outcome_is_failure` and the TS-44b column test |
| D-1 | A late worker returns `Lost`. If it registered an allocation, it releases it. | FN-19: the failing actor releases. A release is a compare-and-set, so the WU-06 sweeper releasing the same row is safe: one wins and the other gets `Lost`. | accepted; tests assert only "no live allocation" |
| D-2 | New `ClaimPolicy` port; an unavailable policy fails closed as `INSTANCE_NOT_ALLOWED` | Work-units discrepancy 1 (the F-04 re-check is a port stub here, made real in WU-08); IFC section 9 has no policy port. Fail-closed follows T-02. | accepted as recorded deviation Dev-1 |
| D-2b | "Retries up to 3 times" means 1 attempt plus 3 retries, 1 s apart, then `Stalled` with the job still `running` | FD section 3.2 (FN-16..FN-20); FD 6.6 leaves the job running, and the sweeper resolves it after the deadline | accepted |
| D-3 | `INSTANCE_NOT_ALLOWED` is applied only on `running -> failed`, after the claim | IFC 10.1. The FD 7.1 diagram also draws `queued -> failed`; IFC is the contract. | accepted; discrepancy recorded |
| D-4 | A failed job carries no components only where capture produced no outcome | IFC 2.3. Not asserted for `PERSISTENCE_ERROR` after a successful capture. | accepted |
| D-5 | The per-allocation `ALLOCATION_LIMIT` unit case lowers the limit below the fixture size | CFG-12 is fixed at 256 MiB (IFC section 8), so the reachable production case is the total budget, which is also tested | accepted |
| D-6 | `DISPATCH_FAILED` is triggered by worker spawn failure at runtime start and by the sweeper's queued-age bound | FD 7.1; both are WU-06. `fail_dispatch` is the entry point those callers use. | accepted |
| Dev-2 | `FakeClock` lives in `lifecycle/test_support.rs` (test-only) | `domain/clock.rs` has only the system clock, and WU-01 code must not change | accepted |

## 5. Gaps and limits

- **Structural tests:** three tests are not red: `late_worker_cannot_overwrite_a_terminal_job` (unit and DB) and the `WorkerConfig` drift guard. A no-op shell already satisfies a negative property. Build-and-verify must keep them green and prove them by mutation.
- **DB simultaneous dispatch:** `ts67_simultaneous_dispatch_has_exactly_one_winner` has no barrier in the DB variant. The store-level CAS evidence is WU-03's claim-race tests in `src/infra/tests/job_store.rs`; the in-memory `simultaneous_claims_have_exactly_one_winner` holds both claims at a barrier.
- **File writes in late-worker tests:** the in-memory late-worker tests do not model a file write. The DB variant does assert on files.
- **Owned elsewhere:**
  - the FD 6.7 degraded hand-off: WU-06 TS-29;
  - allocation audit events (TS-44b second half): WU-06;
  - the real claim policy (TS-05b): WU-08.
- **Identifier coverage:** the T-, ADR- and D- ids cited in code comments are spine ids that `ids.json` does not cover.
- **Follow-ups for a later design touch-up:**
  - align the TS-44b and WU-05 "actor populated" wording with C-1;
  - fix the FD 7.1 `queued -> failed: INSTANCE_NOT_ALLOWED` edge (D-3).
- **At the historical red snapshot:** nothing in WU-05 was implemented. The following build-and-verify stage implemented WU-05 and recorded the green results and mutation evidence in the linked reports; this historical statement is preserved as context.

## 6. Current focused replay (Observed, 2026-11-24)

Purpose: determine whether the historical red should be replayed now. This is a replay against the implemented WU-05 code, not a new red run. The baseline at [../../baseline.md](../../baseline.md) was reused and not rerun or overwritten.

- **Environment:** `dtech-aidlc config prerequisite check` passed the declared `docker` prerequisite (`docker info`, exit 0). `cargo --version` and `rustc --version` both report 1.100.0.
- **Focused worker unit tests:** from the repository root, `cargo test --manifest-path apps/api/Cargo.toml -p domain_backup lifecycle::worker::tests --no-fail-fast` — 27 passed, 0 failed, 0 ignored.
- **Focused fixture tests:** `cargo test --manifest-path apps/api/Cargo.toml -p domain_backup lifecycle::fixture_capture::tests --no-fail-fast` — 5 passed, 0 failed, 0 ignored.
- **Docker-backed worker tests:** `cargo test --manifest-path apps/api/Cargo.toml -p domain_backup infra::tests::worker:: --no-fail-fast -- --ignored` — 15 passed, 0 failed, 0 ignored.
- **Formatting:** `cargo fmt --manifest-path apps/api/Cargo.toml -p domain_backup -- --check` — exit 0.
- **Identifiers:** `dtech-aidlc ids check` — `ok`, 0 unregistered identifiers; the CLI also reports 39 uncovered function identifiers as an advisory.
- **Overinclusive exploratory check:** `cargo test --manifest-path apps/api/Cargo.toml -p domain_backup lifecycle:: --no-fail-fast` included non-WU-05 lifecycle tests and failed 13 such tests. The focused WU-05 worker and fixture test commands above pass; those unrelated lifecycle tests do not establish a WU-05 red.

**Conclusion:** The 2026-11-23 red and the later red-first deadline regression are already represented in the historical evidence and green report. No current WU-05-owned failure was observed. Do not alter tests or production code, do not fabricate a red run, and recommend reporting this red-tests rework as `skipped` because the redundant replay is already green.
