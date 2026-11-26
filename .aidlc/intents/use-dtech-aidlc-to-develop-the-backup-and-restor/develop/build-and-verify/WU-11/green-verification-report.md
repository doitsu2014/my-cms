# GV-11 Green verification report: WU-11

- **ID:** GV-11 (WU-11)
- **Stage:** Develop / build-and-verify (builder-verifier), unit WU-11.
- **Status:** green on the unit's own tests, 2026-11-26. Workspace regression: see [regression-report.md](regression-report.md).
- **Environment:** Windows 11, cargo 1.100.0. WU-11 tests need no Docker. Logs are in [../evidence/](../evidence/).

## 1. Results (Observed)

| Command (from `apps/api`) | Exit | Result | Log |
|---|---|---|---|
| `cargo test -p gateway backup_cli` | 0 | 11 passed: the 6 existing tests and the 5 new TS-33 tests | `green-wu11-focused.log` |
| `cargo fmt --all -- --check` | 0 | formatted | `green-wu11-fmt.log` |
| `cargo clippy --workspace --all-targets` | 0 | warnings only in code that existed before WU-11 (dead-code warnings for the manifest helpers, which the CLI does not call) | `green-wu11-clippy.log` |
| `dtech-aidlc ids check --stage build-and-verify` | 0 | `ok: true` | n/a |

The gateway test binary went from 17 to 22 passing tests (+5).

## 2. No behavior change (Observed)

`diff` of `backup_cli.rs` against the pre-WU-11 copy: 192 lines added, 0 removed. Before `#[cfg(test)]` the additions are comment lines only (a module-doc paragraph and one field doc comment). The six existing tests are unchanged.

## 3. The tests can fail (mutation checks, Observed)

Red was skipped because the code already fails closed, so these mutations stand in for it. Each edit was applied to the non-test part of `handle_args`, the `backup_cli` tests were run, and the file was restored byte for byte (checked with `cmp`). Raw result: `green-wu11-mutations.json`. In that file the `compiled` field is wrong (the script matched the `error: test failed` summary line); the `failed` lists show every mutant compiled and ran.

| Mutation | Tests that failed |
|---|---|
| `restore` returns `SUCCESS` | `ts33_restore_always_refuses_and_never_succeeds`, `ts33_a_fully_configured_cli_contacts_no_database_or_object_store` |
| fully configured `create` returns `SUCCESS` | `ts33_create_fails_closed_in_every_configuration`, `ts33_a_fully_configured_cli_contacts_no_database_or_object_store` |
| unknown input returns `SUCCESS` | `ts33_unknown_and_empty_input_fail_closed_and_only_help_succeeds`, `ts33_a_fully_configured_cli_contacts_no_database_or_object_store` |
| `list` with credentials returns `SUCCESS` | `ts33_list_fails_closed_with_and_without_credentials`, `ts33_a_fully_configured_cli_contacts_no_database_or_object_store` |
| `create` opens a TCP connection to `BACKUP_S3_ENDPOINT` | `ts33_a_fully_configured_cli_contacts_no_database_or_object_store` (the only test that can see it) |

## 4. Not verified here

- The tests call `handle_args` in-process. They check exit codes, not the stderr text, because the function prints with `eprintln!` and changing that would alter behavior.
- "No DB or S3 contact" is shown by a local listener receiving no connection while `handle_args` runs. It does not prove the absence of a connection to a different address, but the only addresses the CLI can read are the variables the test points at the listener.
- Windows only.
