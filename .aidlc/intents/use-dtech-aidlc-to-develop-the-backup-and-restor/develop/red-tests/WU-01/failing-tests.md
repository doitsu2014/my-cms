# FT-01 Failing tests (code manifest)

- **ID:** FT-01
- **Stage:** Develop / red-tests (builder-verifier). Repeats once per work unit; one section per unit.
- **Status:** WU-01 red run recorded 2026-11-22. Later units not started.
- **Inputs:** [work-units](../../implementation-design/work-units.md) WU-01, [test-strategy](../../implementation-design/test-strategy.md) TS-01, TS-02, TS-08a, TS-11, [interfaces-and-contracts](../../implementation-design/interfaces-and-contracts.md) section 8, [functional-design](../../implementation-design/functional-design.md) F-01, F-15, F-39, [acceptance-criteria](../../../analyze/problem-framing/acceptance-criteria.md).
- **Evidence:** [red-evidence.md](red-evidence.md), raw log [evidence/red-wu01-cargo-test.log](../evidence/red-wu01-cargo-test.log).

Provenance: rows are **Observed** from the conductor's own run of `cargo test -p domain_backup --no-fail-fast` on 2026-11-22 (Windows 11, cargo 1.100.0). AC/T mappings marked `~` are the nearest criterion for a test that test-strategy does not list by name (Assumption, to confirm at build-and-verify).

## WU-01 Crate scaffold, domain types, config, clock, errors

### Files

| Path | Change | Notes |
|---|---|---|
| `apps/api/Cargo.toml` | modified | adds workspace member `domain_backup` only |
| `apps/api/Cargo.lock` | modified | gains the `domain_backup` package entry |
| `apps/api/domain_backup/Cargo.toml` | new | deps `chrono`, `serde`, `thiserror` (declared, currently unused: cargo warns), dev-dep `serde_json`; versions match sibling crates; comments record that `sea-orm-migration` and `tokio-util` are deferred to the units that use them and that the lock is std `File::try_lock` (RK-06) |
| `apps/api/domain_backup/src/lib.rs` | new | module root |
| `apps/api/domain_backup/src/domain/mod.rs` | new | module list |
| `apps/api/domain_backup/src/domain/config.rs` | new | shell: `DisabledReason`, `BackupBounds`, `BackupConfig`, `BackupConfigState`, `BackupConfig::load(lookup)`, `from_env()`, `is_loopback_bind`, `validate_storage_root`, `StorageRootError`; tests |
| `apps/api/domain_backup/src/domain/status.rs` | new | shell: `BackupJobStatus` (all documented values plus `Incomplete`), `ALL`, `JobKind`, `checked_status_for_write`, `project_status`; tests |
| `apps/api/domain_backup/src/domain/state_machine.rs` | new | shell: `is_allowed(from, to)`; tests |
| `apps/api/domain_backup/src/domain/error.rs` | new | declarations only (no Display text) |
| `apps/api/domain_backup/src/domain/response.rs` | new | declarations only |
| `apps/api/domain_backup/src/domain/clock.rs` | new | `Clock` port and `SystemClock` shell |

Stub style (test-strategy "WU-01 concrete red tests" rule): logic-free permissive stubs, no `todo!()`. `as_str` returns `""`; `Default` returns zeros and empty strings; `load` returns `Enabled` with zero/empty fields; `is_loopback_bind` returns `true`; `validate_storage_root` returns `Ok(())`; `is_allowed` returns `true`; `checked_status_for_write`/`project_status` return `Ok(status)`; `BackupJobStatus` has a plain serde derive with no rename. Observed by the conductor (non-test code read 2026-11-22).

### Tests

All in crate `domain_backup`, module prefix `domain::`. Command: from `apps/api`, `cargo test -p domain_backup --no-fail-fast`.

| # | Test | TS | AC | T | Expected red reason | Observed |
|---|---|---|---|---|---|---|
| 1 | `config::tests::defaults_match_owner_values` | TS-01 | AC-003 S3 | T-13 | stub default 0 vs owner value (e.g. expiry 86400 s) | FAILED (assertion, config.rs:191) |
| 2 | `config::tests::disabled_unless_backup_enabled_is_true` | TS-02 | AC-003 S3 | T-13 | stub returns `Enabled` for non-`true` flag | FAILED (config.rs:222) |
| 3 | `config::tests::not_configured_takes_precedence` | TS-02 | AC-003 S3~ | T-13~ | empty env returns `Enabled` | FAILED (config.rs:230) |
| 4 | `config::tests::non_loopback_bind_takes_precedence_over_invalid_config` | TS-02, TS-08a | AC-005 S1~ | T-04 | `Enabled` instead of `NON_LOOPBACK_BIND` | FAILED (config.rs:251) |
| 5 | `config::tests::disabled_reason_projection_covers_null` | TS-02 | AC-003 S3~ | T-13~ | `None` where `Some("NOT_CONFIGURED")` expected | FAILED (config.rs:261) |
| 6 | `config::tests::disabled_reason_closed_set_spelling` | TS-02 | AC-003 S3~ | T-13~ | `as_str` returns `""` | FAILED (config.rs:320) |
| 7 | `config::tests::host_default_is_loopback_and_non_loopback_host_disables` | TS-02, TS-08a | AC-005 S1 | T-04 | `HOST=0.0.0.0` returns `Enabled` | FAILED (config.rs:309) |
| 8 | `config::tests::enabled_without_storage_root_refuses` | TS-02 | AC-003 S3 | T-13, T-14 | missing CFG-04 returns `Enabled` | FAILED (config.rs:173) |
| 9 | `config::tests::storage_root_rejects_forbidden_and_relative_paths` | TS-02 | AC-003 S3 | T-14 | forbidden root returns `Ok` | FAILED (config.rs:440) |
| 10 | `config::tests::storage_root_must_be_private_on_unix` | TS-02 | AC-003 S3 | T-14 | `#[cfg(unix)]` | NOT RUN (not compiled on Windows) |
| 11 | `config::tests::invalid_numeric_env_refuses_capability` | TS-02 | AC-003 S3 | T-13 | `abc` returns `Enabled` | FAILED (config.rs:173) |
| 12 | `config::tests::non_canonical_and_overflowing_numbers_are_invalid` | TS-02 | AC-003 S3~ | T-13~ | `+86400` returns `Enabled` | FAILED (config.rs:173) |
| 13 | `config::tests::bounds_other_than_default_disable_with_invalid_config` | TS-02 | AC-003 S3, AC-002 S2 | T-13 | below/above default returns `Enabled` | FAILED (config.rs:173) |
| 14 | `config::tests::escalation_owner_cannot_be_overridden` | TS-02 | AC-003 S3~ | T-13~ | owner `""` not `project_owner` | FAILED (config.rs:527) |
| 15 | `config::tests::ingress_limits_required` | TS-02 | AC-003 S3 | T-13 | absent CFG-17/CFG-18 returns `Enabled` | FAILED (config.rs:173) |
| 16 | `config::tests::allowed_origins_required_and_well_formed` | TS-02 | AC-005 S1 | T-04 | missing/malformed CFG-16 returns `Enabled` | FAILED (config.rs:173) |
| 17 | `config::tests::invalid_config_always_yields_disabled_state` | TS-02 | AC-003 S3~ | T-13~ | bad value returns `Enabled` (no panic) | FAILED (config.rs:296) |
| 18 | `config::tests::is_loopback_bind_accepts_only_loopback_ip_literals` | TS-08a | AC-005 S1 | T-04 | `0.0.0.0` accepted as loopback | FAILED (config.rs:617) |
| 19 | `state_machine::tests::no_transition_reaches_succeeded_or_recovery_required` | TS-11 | AC-002 S3 | T-06 | transition into `Succeeded`/`RecoveryRequired` allowed | FAILED (state_machine.rs:20) |
| 20 | `state_machine::tests::terminal_states_have_no_outgoing_transitions` | TS-11 | AC-002 S3 | T-06 | exit from a terminal state allowed | FAILED (state_machine.rs:33) |
| 21 | `state_machine::tests::only_the_four_u1_edges_are_allowed` | TS-11 | AC-002 S3~ | T-06 | non-edge pair allowed | FAILED (state_machine.rs:53) |
| 22 | `status::tests::no_write_or_projection_yields_succeeded_or_recovery_required` | TS-11 | AC-002 S3 | T-06 | `Succeeded` write returns `Ok` | FAILED (status.rs:93) |
| 23 | `status::tests::wire_spelling_keeps_documented_values_and_adds_incomplete` | TS-11 | AC-002 S3~ | T-06~ | serializes `"Queued"`, expected `"queued"` | FAILED (status.rs:117) |
| 24 | `status::tests::all_lists_every_variant_once` | TS-11 (support) | none | none | structural guard on `ALL` | PASSED, not red (structural; does not count as red evidence) |

Totals (Observed): 23 tests ran; 22 failed on assertions; 1 passed (structural); 1 Unix-only test not compiled on this host.

### Not covered by a WU-01 red test (explicit)

- TS-02 "gateway still starts and answers 503 `BACKUP_DISABLED`" part: deferred to WU-08/WU-10 (needs the router).
- TS-01 "a 4th unlink attempt never occurs": deferred to WU-06 (cleaner).
- `from_env()` smoke test: not written, because it would need process-environment mutation; `load(lookup)` carries the behaviour (deviation from the test-strategy `ENV_LOCK`/`test_support.rs` idea, recorded in red-evidence).
- Windows owner-only ACL (T-14): not tested; owner accepted this as a residual on 2026-11-22.
- `error.rs`, `response.rs`, `clock.rs`: no WU-01 test targets them; their behaviour is tested in the units that use them.
