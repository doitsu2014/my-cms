# GVR-01 Green verification report: WU-01

- **ID:** GVR-01 (WU-01)
- **Stage:** Develop / build-and-verify, unit WU-01 of WU-01..WU-15. Not the release-candidate gate (that is presented once, after WU-15).
- **Status:** WU-01 green on this host, 2026-11-22.
- **Inputs:** [red-tests/WU-01](../../red-tests/WU-01/failing-tests.md), [implementation.md](implementation.md), [test-strategy](../../implementation-design/test-strategy.md), [acceptance-criteria](../../../analyze/problem-framing/acceptance-criteria.md), [user-stories](../../../analyze/problem-framing/user-stories.md), [quality-requirements](../../../analyze/context-and-risk/quality-requirements.md).
- **Logs:** [evidence/green-wu01-cargo-test.log](../evidence/green-wu01-cargo-test.log), [evidence/regression-wu01-cargo-fmt.log](../evidence/regression-wu01-cargo-fmt.log), [evidence/regression-wu01-cargo-clippy.log](../evidence/regression-wu01-cargo-clippy.log).

Provenance: results are Observed from the conductor's own runs on 2026-11-22. Host: Windows 11, cargo 1.100.0.

## 1. Red then green, same tests

- **Red** (2026-11-22, [red log](../../red-tests/evidence/red-wu01-cargo-test.log)): 23 ran, 22 failed on assertions, 1 structural pass.
- **Green:** from `apps/api`, `cargo test -p domain_backup --no-fail-fast` exits 0 with **35 passed, 0 failed, 0 ignored** (doc-tests 0). That is the same 23 tests plus 12 test-after hardening tests.
- **Red tests unchanged:** the test-module text from `#[cfg(test)]` was byte-identical to the red snapshot after the build. The conductor then applied `cargo fmt`, and whitespace/trailing-comma-stripped content still equals the red snapshot. No test was deleted, weakened, skipped or ignored.
- **Not run:** `config::tests::storage_root_must_be_private_on_unix` is `#[cfg(unix)]` and is not compiled on this Windows host. The two `#[cfg(windows)]` tests ran here and are compiled out on Unix.

## 2. Results per test level

| Level | Result |
|---|---|
| Unit (Rust, `domain_backup`) | 35 passed, 0 failed |
| Integration (DB, testcontainers) | not run: no WU-01 integration tests; Docker unavailable on this host |
| End to end | not run: not in WU-01 scope |
| Performance | not run: not in WU-01 scope |
| Security | unit-level only: loopback guard (T-04), config fail-closed (T-13), storage-root validation (T-11/T-14 partial), status guard (T-06). No composed-router or penetration check yet (WU-08/WU-10/WU-15). |
| Static | `cargo fmt -- --check` exit 0 (workspace); `cargo clippy --all-targets` exit 0, no warnings attributed to `domain_backup` |

## 3. Test-after hardening tests (not red evidence)

These cover hardening added during build review, after the red run. They are not counted as red-first. Instead, each was shown to fail when the check it targets was temporarily disabled, then the code was restored and byte-compared to the snapshot (builder-reported; the conductor verified the final green run and the snapshot discipline).

| Test | Behaviour | Mutation | Failed under mutation |
|---|---|---|---|
| `storage_root_rejects_link_on_ancestor` | junction/symlink on any ancestor | `ancestors()` to `ancestors().take(1)` | yes |
| `storage_root_rejects_parent_dir_component` | `..` refused | ParentDir arm emptied | yes |
| `storage_root_rejects_verbatim_prefix_on_windows` | `\\?\` refused (Windows) | prefix match narrowed | yes |
| `forbidden_comparison_is_case_insensitive_on_windows` | case-insensitive forbidden check | `.to_lowercase()` removed | yes |
| `storage_root_rejects_existing_file` | `NotDirectory` | check removed | yes |
| `storage_root_metadata_error_is_inaccessible` | non-NotFound error refused (interior NUL path) | `Err(_)` arm emptied | yes |
| `padded_required_text_is_invalid_config` | padded instance id refused | trim check dropped | yes |
| `from_env_non_utf8_value_fails_validation` | non-UTF-8 env refused (child process, own env untouched) | `to_string_lossy` substituted | yes |
| `load_refuses_link_or_file_storage_root` | `load` validates root | call in `parse` disabled | yes |
| `origin_port_bounds_are_enforced` | port 0 and 65536 refused | `n > 0` to `true`; `u16` to `u32` | yes |
| `origin_host_label_hyphen_edges_refused` | `-a` / `a-` labels refused | each check removed | yes |

## 4. Quality requirements

| QR | Status after WU-01 |
|---|---|
| O-07 Configuration validation on startup | Verified at unit level: any invalid or missing required value gives a disabled state with a closed-set reason and no panic (`invalid_config_always_yields_disabled_state`). The startup wiring is WU-10. |
| A-05 Staging expiry enforcement | Partial: the 24 h value is fixed and validated. Enforcement is WU-06. |
| P-05 Concurrent job limits | Partial: the limit of 1 is fixed and validated. Enforcement is WU-03/WU-05. |
| PR-05 Staging data protection | Partial: storage-root validation is implemented and tested. The Windows owner-only ACL is not verified (owner-accepted T-14 residual). The Unix 0700 check was not run here. |
| O-02 Safe error projection | Not verified: `project_error` exists but has no WU-01 test. Covered by the projection/canary tests in WU-08/WU-15. |
| All others | Not in WU-01 scope; not run. |

## 5. Codebase map

`domain_backup` is a new component, but it is not yet composed into the gateway or exposed as an entry point. The codebase map (`.aidlc/knowledge/_codebase-map.md`) is not updated in this unit. It will be updated in WU-10, when the crate becomes part of the running gateway and its routes and commands exist. This is recorded so the omission is visible.

## 6. Trace table

One row per acceptance criterion. "not run" means no test for this criterion exists yet in the completed units. The owning later unit is named.

| Story | Criterion | Function | Test | Result |
|---|---|---|---|---|
| US-01, US-05 | AC-001 Instance selection and authorization | F-01 (CFG-02/CFG-03 instance config only) | `enabled_without_storage_root_refuses`, `padded_required_text_is_invalid_config` (config part only); listing/authorization tests in WU-08/WU-13 | config part passed; listing and authorization not run |
| US-02, US-06 | AC-002 S1 Job status is observable | none yet (WU-05/WU-08/WU-14) | none yet | not run |
| US-02, US-06 | AC-002 S2 Required capture fails | F-01 (bounds) | `bounds_other_than_default_disable_with_invalid_config` (bounds part); terminal-outcome tests in WU-05 | bounds part passed; outcome not run |
| US-02, US-06 | AC-002 S3 U1 database-only result | F-15, status guard | `no_transition_reaches_succeeded_or_recovery_required`, `terminal_states_have_no_outgoing_transitions`, `only_the_four_u1_edges_are_allowed`, `no_write_or_projection_yields_succeeded_or_recovery_required`, `wire_spelling_keeps_documented_values_and_adds_incomplete` | passed (pure guards; persistence and API in WU-02/WU-08) |
| US-03 | AC-003 S1 Invalid or incompatible package | none (upload/preflight acceptance not authorized in U1; rejection previews in WU-09) | none yet | not run |
| US-03 | AC-003 S2 Unsafe archive | none (deferred with real packages) | none | not run (deferred) |
| US-03, US-02, US-04, US-06 | AC-003 S3 U1 real-data staging is not enabled | F-01, `validate_storage_root` | `disabled_unless_backup_enabled_is_true`, `not_configured_takes_precedence`, `invalid_numeric_env_refuses_capability`, `non_canonical_and_overflowing_numbers_are_invalid`, `ingress_limits_required`, `escalation_owner_cannot_be_overridden`, `invalid_config_always_yields_disabled_state`, `storage_root_rejects_forbidden_and_relative_paths`, the 12 hardening tests | passed (config fail-closed; staging disabled by construction until WU-04..WU-06) |
| US-04 | AC-004 S1 Confirmation describes impact | none yet (WU-09 identification fields) | none yet | not run |
| US-04 | AC-004 S2 Restore is cancelled | none yet (WU-09) | none yet | not run |
| US-04 | AC-004 S3 Restore failure | none (restore execution not authorized in U1) | none | not run (deferred) |
| US-05, US-06 | AC-005 S1 Unauthorized operation or resource access | F-39 (loopback guard), F-01 (CFG-16 origins) | `is_loopback_bind_accepts_only_loopback_ip_literals`, `host_default_is_loopback_and_non_loopback_host_disables`, `non_loopback_bind_takes_precedence_over_invalid_config`, `allowed_origins_required_and_well_formed`, `origin_port_bounds_are_enforced`, `origin_host_label_hyphen_edges_refused` | passed (guard and config part); authorization in WU-08/WU-10 not run |
| US-05, US-06 | AC-005 S2 Audit records a sensitive operation | none yet (WU-03/WU-05/WU-07) | none yet | not run |
| US-05, US-06 | AC-005 S3 Audit store unavailable before effect | none yet (WU-07/WU-06) | none yet | not run |
| US-02, US-03, US-04 | AC-006 S1 Production restore readiness | none (production not authorized) | none | not run (out of scope) |
| US-02, US-03, US-04 | AC-006 S2 Unresolved U1 design blockers | F-01 (owner-approved bounds encoded) | `defaults_match_owner_values` | passed (owner decisions encoded as fixed defaults) |
