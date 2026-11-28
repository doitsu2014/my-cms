# RE-08 Red evidence: WU-08

- **ID:** RE-08 (WU-08)
- **Stage:** Develop / red-tests, unit WU-08.
- **Recorded:** 2026-11-26, Windows 11, cargo 1.100.0.
- **Manifest:** [failing-tests.md](failing-tests.md).
- **Baseline:** Existing intent baseline is [../../baseline.md](../../baseline.md), recorded 2026-11-22; it was not rerun or overwritten.

## Environment and prerequisite checks observed

| Check | Result | Evidence |
|---|---|---|
| `dtech-aidlc --project D:/orca/my-cms/makara config prerequisite check` | `ok: true`; Docker check passed | CLI output observed before WU-08 tests |
| `cargo --version` / `rustc --version` | cargo 1.100.0 / rustc 1.100.0 | CLI output observed before WU-08 tests |
| `docker info --format '{{.ServerVersion}}'` | Docker 29.7.2 | CLI output observed before WU-08 tests |
| `cargo check --workspace` from `apps/api` | exit 0 | command output observed before WU-08 tests |
| `cargo fmt --all -- --check` before WU-08 test edit | exit 0 | command output observed before WU-08 tests |
| `cargo test --workspace --no-fail-fast` | exit 0; baseline suite passed before the WU-08 code change | command output observed before WU-08 tests |

The focused Docker-backed WU-08 tests in this red run are pure serializer/projection tests and do not require a database container. Docker availability is nevertheless verified for later WU-08 integration tests.

## Focused red run observed

Command from `apps/api`: `cargo test -p domain_backup handlers::job_response -- --nocapture`

- **Exit:** 101.
- **Result:** 0 passed, 3 failed, 0 ignored; 331 filtered out.
- **Compilation/setup:** successful. All failures are behavioral assertion failures, not compiler or environment errors.
- **Tests:**
  - `handlers::job_response::tests::ts12b_projection_refuses_success_and_recovery_required` — expected `Err(UnreachableStatus(Succeeded))`; shell projected success and `download_eligible: true`.
  - `handlers::job_response::tests::ts49_projection_omits_actor_and_unmeasured_progress_and_denies_download` — expected `progress: null`; shell returned measured-looking `0/0`.
  - `handlers::job_response::tests::ts49_projection_reports_progress_only_for_a_measured_database_component` — expected measured `1/1`; shell returned fabricated `0/0`.

Raw output: `evidence/red-wu08-job-response.log`.

## Formatting and IDs checks

| Command | Exit | Result | Log |
|---|---:|---|---|
| `cargo fmt --all -- --check` from `apps/api` | 0 | formatted after formatting the added test module | `evidence/red-wu08-fmt.log` (empty on success) |
| `dtech-aidlc --project D:/orca/my-cms/makara ids check --stage red-tests` | 0 | `ok: true`; no unregistered IDs | `evidence/red-wu08-ids.log` |

## Coverage limits / handoff

This is intentionally a narrow, executable red slice for the projection contract. It does not claim red evidence for all WU-08 requirements. The remaining registered WU-08 behaviors—authorization/policy, configured instance list, database-backed reads, body/query/content-type rejection mapping, Host/Origin, escalation status, metrics and canary behavior—remain to be covered with tests during WU-08 build-and-verify. Gateway-level composed-router tests belong to WU-10; full canary bundle scan belongs to WU-15. No external service or live credentials were used.
