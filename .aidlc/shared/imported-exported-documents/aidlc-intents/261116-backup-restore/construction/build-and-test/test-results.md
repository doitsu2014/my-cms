# Test Results

## Build status

- **Status:** Passed — `cargo check --manifest-path apps/api/Cargo.toml -p gateway` completed with 0 errors and 21 warnings (workspace resolver warning, pre-existing missing-Debug warnings, new unused backup manifest/helpers, and a future-incompatibility warning).

## Tests

- `cargo test --manifest-path apps/api/Cargo.toml -p gateway backup_cli::tests` — **6 passed**, 11 filtered out.
- `cargo test --manifest-path apps/api/Cargo.toml -p gateway` — **17 passed**.
- `cargo fmt --manifest-path apps/api/gateway/Cargo.toml -- --check` — passed, no output.
- These are unit/helper tests only. No real S3, PostgreSQL, Supabase Storage, or destructive restore operation was invoked.

## Target Verification Matrix

| Target ID | Source | Expected | Actual | Evidence | Owning Stage | Verdict |
|---|---|---|---|---|---|---|
| FR1.1 | requirements.md — all Supabase PostgreSQL schemas | Backup contains all instance databases/schemas including platform-managed metadata | Not implemented; no complete PostgreSQL export | Code summary and `backup_cli.rs` fail-closed behavior | code-generation | Not Met |
| FR1.2 | requirements.md — image/blob storage | Backup includes configured stored objects | Not implemented; no object inventory/capture or S3 transfer | Code summary | code-generation | Not Met |
| FR1.3 | requirements.md — outcome record | Durable success/failure record and location | No durable outcome record; no successful backup path | Code summary | code-generation | Not Met |
| FR1.4 | requirements.md — archive integrity | Completed archive verified before success | No archive is produced or verified | Code summary | code-generation | Not Met |
| FR2.1 | requirements.md — PostgreSQL restore | Restore selected full database/schema backup | Restore intentionally disabled | Code summary and CLI refusal | code-generation | Not Met |
| FR2.2 | requirements.md — object restore | Restore matching image/blob objects | Restore intentionally disabled | Code summary | code-generation | Not Met |
| FR2.3 | requirements.md — restore outcome | Durable restore attempt outcome | No durable restore outcome; refusal is reported to stderr only | Code summary | code-generation | Not Met |
| NFR1 | requirements.md — operator-only restore | Restore unavailable to application callers; operator command boundary only | Implemented as CLI-only command; restore fails closed | `apps/api/gateway/src/main.rs`, `apps/api/gateway/src/backup_cli.rs` | code-generation | Met |
| NFR2 | requirements.md — integrity | Successful backup requires integrity check | No archive or success path exists | Code summary | code-generation | Not Met |
| NFR3 | requirements.md — consistency disclosure | A completed backup reports best-effort consistency | Manifest model states best-effort, but no completed backup record exists | `backup_cli.rs`, code summary | code-generation | Not Met |

## Coverage and external checks

- The Code Generation stage added six focused unit tests. The focused and full gateway test suites both passed in this stage.
- No live-service integration checks, S3 transfers, PostgreSQL exports, storage object transfers, or restore operations were run.

## Loop-Back Log

None.
