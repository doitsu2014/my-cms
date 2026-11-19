# Build Instructions

## Prerequisites

- Rust toolchain compatible with the repository's Cargo workspace.
- No S3, database, or Supabase credentials are required for the local build or tests.
- Do not run backup or restore commands against production from this stage.

## Build

Run from the workspace root:

`cargo check --manifest-path apps/api/Cargo.toml -p gateway`

## Verification

The focused backup CLI tests and gateway suite are run separately and recorded in `test-results.md`. A clean `cargo check` is required for the build result. The Code Generation implementation currently fails closed for backup creation/listing and disables restore because the transfer and complete platform recovery adapters are not implemented; passing compilation/tests must not be interpreted as a functional backup/restore validation.

## Troubleshooting

- If Cargo cannot resolve dependencies, inspect the exact registry/network error; do not substitute unrelated dependency versions or reduce quality checks.
- If a test needs production credentials or contacts an external service, do not run it; record it as not applicable or blocked with the reason.
