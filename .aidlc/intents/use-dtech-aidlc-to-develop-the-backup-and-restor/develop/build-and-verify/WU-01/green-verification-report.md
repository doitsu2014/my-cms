# GVR-01 Green verification report: WU-01

- **Stage:** Develop / build-and-verify, work unit WU-01.
- **Status:** WU-01 focused tests and workspace build/static checks passed in the 2026-11-25 refresh. The complete current workspace regression is **not green**; this report does not claim an overall green stage. See `regression-report.md`.
- **Provenance:** Results below were observed in this session on Windows 11, Cargo/rustc 1.100.0, Docker 29.7.2.
- **Scope:** Verify existing implementation only. No source or behavior changes were made.
- **Evidence:** Fresh raw logs are in `../evidence/refresh-wu01-*`.

## Environment

| Check | Observed result |
|---|---|
| `dtech-aidlc config prerequisite check` | Exit 0; declared Docker prerequisite passed. |
| `cargo --version`; `rustc --version` | Cargo 1.100.0; rustc 1.100.0. |
| `docker info --format '{{.ServerVersion}}'` | Exit 0; Docker 29.7.2. |
| Commands required by this WU from `PROJECT.md` | Rust fmt, lint, workspace check and test suites run below. |
| Node/pnpm and frontend checks | Not rechecked/run in this refresh; frontend is outside WU-01 scope and no web files changed. |

## Focused WU-01 and workspace checks

Commands ran from the repository root with `--manifest-path apps/api/Cargo.toml`. Each command's metadata log records command, cwd, timestamps and exit code; paired stdout/stderr logs are beside it.

| Check | Observed result | Raw evidence basename |
|---|---|---|
| `cargo test --manifest-path apps/api/Cargo.toml -p domain_backup domain::config::tests --no-fail-fast` | Exit 0; 29 passed, 0 failed. | `refresh-wu01-focused-config` |
| `cargo test --manifest-path apps/api/Cargo.toml -p domain_backup domain::state_machine::tests --no-fail-fast` | Exit 0; 3 passed, 0 failed. | `refresh-wu01-focused-state-machine` |
| `cargo test --manifest-path apps/api/Cargo.toml -p domain_backup domain::status::tests --no-fail-fast` | Exit 0; 3 passed, 0 failed. | `refresh-wu01-focused-status` |
| **Focused WU-01 total** | **35 passed, 0 failed.** | Above three logs |
| `cargo fmt --manifest-path apps/api/Cargo.toml -- --check` | Exit 0. | `refresh-wu01-fmt` |
| `cargo clippy --manifest-path apps/api/Cargo.toml --workspace --all-targets` | Exit 0; warnings remain. | `refresh-wu01-clippy` |
| `cargo check --manifest-path apps/api/Cargo.toml --workspace` | Exit 0. | `refresh-wu01-check-workspace` |
| `dtech-aidlc ids check --stage build-and-verify` | Exit 0; `ok: true`; no unregistered identifiers. Uncovered function IDs are assigned to other units. | `refresh-wu01-ids-check` |

The focused suites verify the WU-01 config, pure state-machine and status guards only. They do not verify persistence, gateway composition or end-to-end behavior. The prior WU-01 red evidence is the historical 2026-11-22 run at `../../red-tests/WU-01/`; it was reused and not rerun in this behavior-neutral refresh.

## Full workspace outcome (not green)

The full current workspace was run from the repository root with Docker available:

- Normal: `cargo test --manifest-path apps/api/Cargo.toml --workspace --no-fail-fast` — exit 101; **369 passed, 13 failed, 134 ignored**.
- Docker-ignored: `cargo test --manifest-path apps/api/Cargo.toml --workspace --no-fail-fast -- --ignored` — exit 101; **131 passed, 3 failed**.

The 13 normal failures and 3 ignored failures are detailed in `regression-report.md`. All are reported as failures, even though the recorded work-unit ownership places them outside WU-01. The historical baseline's passing test identities all passed in the current matching run; this does not satisfy the strict no-new-failures rule because the current failure set is not empty. Thus the WU-01 focused result is green while aggregate regression and the release-candidate gate are red.

## Quality and scope

- `FN-01` configuration behavior: focused config tests passed, including approved defaults, disabled reasons, required values, strict numeric parsing, storage-root checks and CFG validation (`TS-01`, `TS-02`).
- `FN-39` loopback IP guard: covered by the passing config suite (`TS-08a`). Gateway wiring is later work and was not verified.
- `FN-15` pure state guard and U1 status write/projection guard: focused state-machine and status suites passed (pure `TS-11` portion). Persistence and API projections are later work and not claimed here.
- No endpoint, gateway composition, database persistence or end-to-end behavior is claimed verified by WU-01.
- No codebase-map update: no entry point or gateway composition was changed in this WU-01 verification.

## Acceptance-criterion trace table

Each acceptance criterion has a row. Function references use registered `FN-*` IDs; criteria and stories are from Analyze. Tests not assigned to WU-01 are marked not run here. Results describe only evidence for this WU-01 refresh.

| Story | Criterion | Function | Test | Result |
|---|---|---|---|---|
| `US-01`, `US-05` | `AC-001` — Instance selection and authorization | `FN-01` (configuration only) | `enabled_without_storage_root_refuses`, `padded_required_text_is_invalid_config` | Config checks passed; instance listing and authorization not run (later units). |
| `US-02`, `US-06` | `AC-002` S1 — Job status is observable | — | None in WU-01 | Not run; later units. |
| `US-02`, `US-06` | `AC-002` S2 — Required capture fails | `FN-01` (bounds only) | `bounds_other_than_default_disable_with_invalid_config` | Bounds check passed; capture outcome not run (later units). |
| `US-02`, `US-06` | `AC-002` S3 — U1 database-only result | `FN-15` | `no_transition_reaches_succeeded_or_recovery_required`, `terminal_states_have_no_outgoing_transitions`, `only_the_four_u1_edges_are_allowed`, `no_write_or_projection_yields_succeeded_or_recovery_required`, `wire_spelling_keeps_documented_values_and_adds_incomplete` | Pure guards and enum checks passed; persistence/API not run. |
| `US-03` | `AC-003` S1 — Invalid or incompatible package | — | None in WU-01 | Not run; handling is later scope. |
| `US-03` | `AC-003` S2 — Unsafe archive | — | None in WU-01 | Not run; deferred with real packages. |
| `US-03`, `US-02`, `US-04`, `US-06` | `AC-003` S3 — U1 real-data staging is not enabled | `FN-01` | `disabled_unless_backup_enabled_is_true`, `not_configured_takes_precedence`, `non_loopback_bind_takes_precedence_over_invalid_config`, `invalid_numeric_env_refuses_capability`, `non_canonical_and_overflowing_numbers_are_invalid`, `bounds_other_than_default_disable_with_invalid_config`, `ingress_limits_required`, `allowed_origins_required_and_well_formed`, `storage_root_rejects_forbidden_and_relative_paths`, `is_loopback_bind_accepts_only_loopback_ip_literals`; storage-root hardening tests | Config/path/loopback checks passed; full lifecycle and persistence guarantee not claimed until later units. |
| `US-04` | `AC-004` S1 — Confirmation describes impact | — | None in WU-01 | Not run; later unit. |
| `US-04` | `AC-004` S2 — Restore is cancelled | — | None in WU-01 | Not run; later unit. |
| `US-04` | `AC-004` S3 — Restore failure | — | None in WU-01 | Not run; deferred because restore execution is not authorized in U1. |
| `US-05`, `US-06` | `AC-005` S1 — Unauthorized operation or resource access | `FN-01`, `FN-39` | `is_loopback_bind_accepts_only_loopback_ip_literals`, `host_default_is_loopback_and_non_loopback_host_disables`, `non_loopback_bind_takes_precedence_over_invalid_config`, `allowed_origins_required_and_well_formed`, `origin_port_bounds_are_enforced`, `origin_host_label_hyphen_edges_refused` | Config and bind-guard portion passed; server authorization not run. |
| `US-05`, `US-06` | `AC-005` S2 — Audit records a sensitive operation | — | None in WU-01 | Not run; later units. |
| `US-05`, `US-06` | `AC-005` S3 — Audit store unavailable before effect | — | None in WU-01 | Not run; later units. |
| `US-02`, `US-03`, `US-04` | `AC-006` S1 — Production restore readiness | — | None in WU-01 | Not run; production readiness is out of scope. |
| `US-02`, `US-03`, `US-04` | `AC-006` S2 — Unresolved U1 design blockers | `FN-01` (approved bounds encoded only) | `defaults_match_owner_values` | Owner-approved config values passed; gate review of unresolved blockers is later work and not claimed. |

## Limitations

See `known-limitations.md` for Unix-only permission validation, Windows ACL residual, consumer-supplied forbidden paths, open CFG values, untested projection/clock types, and the aggregate regression failures. This verification does not constitute release-candidate approval or authorize deployment, production use, real-data staging, upload, restore or downloads.
