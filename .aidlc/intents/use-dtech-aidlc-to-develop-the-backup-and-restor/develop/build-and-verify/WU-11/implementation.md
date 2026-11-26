# IM-11 Implementation: WU-11 CLI fail-closed tests and doc reconciliation

- **ID:** IM-11 (WU-11)
- **Stage:** Develop / build-and-verify (builder-verifier), unit WU-11.
- **Status:** implemented and verified 2026-11-26.
- **Red stage:** skipped by design. The unit changes no behavior, so its tests pass against the existing code (characterization tests). See section 3 of [green-verification-report.md](green-verification-report.md) for the evidence that the tests can fail.

## Change (one file)

`apps/api/gateway/src/backup_cli.rs`, additive only: 192 lines added, 0 removed. Production code is untouched; the non-test changes are comments.

| Part | Change |
|---|---|
| module doc | a "Vocabulary" paragraph: the `outcome` strings of `Manifest` (`success`, `failed`) belong to the manifest format only and are unrelated to the `domain_backup` job status; `create`, `list` and `restore` never contact a database or object store in this build |
| `Manifest.outcome` | a doc comment stating the same |
| `mod tests` | five TS-33 tests and their helpers (`ENV_LOCK`, `EnvScope`). The six existing tests are unchanged. |

## TS-33 tests

Environment variables are process-global, so each test holds a static `tokio::sync::Mutex` and an `EnvScope` that clears every variable the CLI can read and restores the previous values on drop.

| Test | Asserts |
|---|---|
| `ts33_restore_always_refuses_and_never_succeeds` | `restore` with no id, a valid id, traversal ids, an empty id and an 81-character id all return `FAILURE`, with and without credentials and settings |
| `ts33_create_fails_closed_in_every_configuration` | `create` returns `FAILURE` for all 16 combinations of (no credential, `AWS_ACCESS_KEY_ID`, `AWS_PROFILE`, `AWS_WEB_IDENTITY_TOKEN_FILE`) and (no settings, endpoint only, root only, both) |
| `ts33_list_fails_closed_with_and_without_credentials` | `list` returns `FAILURE` in three configurations |
| `ts33_unknown_and_empty_input_fail_closed_and_only_help_succeeds` | no args, empty, `backup`, an unknown word, wrong case, `--version` and `-x` return `FAILURE`; only `--help` and `-h` return `SUCCESS` |
| `ts33_a_fully_configured_cli_contacts_no_database_or_object_store` | with credentials, `BACKUP_S3_ENDPOINT`, `BACKUP_LOCAL_ROOT`, `DATABASE_URL` and `SUPABASE_DB_URL` all pointing at a local listener, `create`, `list`, `restore` and an unknown command return `FAILURE` and the listener receives no connection |

## Identifiers

No identifier was minted. `dtech-aidlc ids check` is clean.
