# FT-08 Failing tests (code manifest): WU-08

- **ID:** FT-08 (WU-08)
- **Stage:** Develop / red-tests, unit WU-08.
- **Status:** Red run observed on 2026-11-26, Windows 11, cargo 1.100.0.
- **Unit:** Authorization policy, instance registry, REST read/create adapters, status endpoint, safe error projection.
- **Inputs:** WU-08 in `implementation-design/work-units.md` and `work-units.json`; `ids.json`; test strategy; IFC; functional design; Analyze acceptance criteria.
- **Evidence:** [red-evidence.md](red-evidence.md), raw test and formatting logs in `evidence/`.

## Scope of this red slice

WU-08 includes public REST and security contracts, so behavioral red-first is appropriate. This red run covers the isolated job-response projection boundary only. It is a deliberate thin vertical slice; it does not claim that the remaining WU-08 API tests have run or failed. The HTTP wiring, query/body extractors, registry and policy tests are to be added alongside implementation during build-and-verify as appropriate, with new behavioral-red tests first for security/public-contract cases.

## Changed files

Under `apps/api/domain_backup/src/`:

| Path | Change | Red behavior |
|---|---|---|
| `handlers/job_response.rs` | New compile-clean projection type and tests | Shell deliberately always marks downloadable, emits zero progress even for measured data, emits fabricated `0/0` when unmeasured, and permits all persisted statuses. |
| `handlers/mod.rs` | Wires `job_response` module | No production routing or behavior. |

No prior tests or production files outside WU-08 were changed. The shell exists only to compile and execute behavioral assertions; WU-08 build-and-verify replaces the shell with contract-compliant projection and expands the REST/API surface.

## Tests and observed failures

Command from `apps/api`: `cargo test -p domain_backup handlers::job_response -- --nocapture`

| Test | Registered TS | First behavioral assertion |
|---|---|---|
| `handlers::job_response::tests::ts12b_projection_refuses_success_and_recovery_required` | TS-12b | Expected `Err(UnreachableStatus(Succeeded))`; inert projection returned `Ok(JobResponse { status: Succeeded, download_eligible: true, ... })`. |
| `handlers::job_response::tests::ts49_projection_omits_actor_and_unmeasured_progress_and_denies_download` | TS-49, TS-12b | Expected unmeasured `progress: null`; shell returned `{"completed":0,"total":0}`. The test also checks actor/state-version/deadline omissions and download denial. |
| `handlers::job_response::tests::ts49_projection_reports_progress_only_for_a_measured_database_component` | TS-49 | Expected measured database progress `1/1`; shell returned fabricated `0/0`. |

Three tests ran; all three failed on assertions (exit 101). The crate compiled and setup succeeded. Exact raw results are in `evidence/red-wu08-job-response.log`.

## Planned WU-08 coverage not included in this red run

Identifiers below are registered; mentioning these is coverage planning, not a claim of test execution in this run: TS-04, TS-05b, TS-06, TS-17 API portion, TS-27b, TS-45, TS-57, TS-59, TS-60, TS-61, TS-62, TS-63. They will be tested in the WU-08 build-and-verify slice. TS-55 is owned by WU-10; TS-17 full bundle scan belongs to WU-15.

No TS identifier was minted. The CLI ID check returned `ok: true` for red-tests.