# Unit Test Instructions — Backup and Restore

## Scope

This workflow has no Unit DAG; use the Code Generation stage-level record. Test the new backup/restore Rust module and CLI through isolated unit tests with mocked PostgreSQL process execution and mocked S3/object-storage operations. Do not require production credentials or contact live services in unit tests.

## Framework and setup

- Framework: Rust unit tests (`#[test]` / async test runtime as used by the owning crate).
- First inspect the existing Cargo workspace and test conventions to identify the exact owning package and test filter. Do not assume the gateway package name or invoke project-wide tests unnecessarily.
- Existing test runner verification: inspect `apps/api/Cargo.toml` and the target crate manifest. Record the exact package-scoped command after confirming package/filter syntax.

## Required coverage

- At least one successful manifest/archive verification test.
- At least one successful backup orchestration test with fake database and object adapters.
- At least one successful restore orchestration test with validation completed before destructive actions.
- Error cases: database export failure; object transfer failure; checksum/manifest corruption; incomplete or unsupported backup format; restore failure after partial work.
- Input/security cases: unsafe backup ID/path, invalid S3 object key, shell metacharacters in database settings, and missing credentials must fail safely without leaking secret values.

## Commands

Before running tests, confirm the relevant crate and exact test filter from the workspace manifests and code. Use a package-scoped command, for example:

`cargo test --manifest-path apps/api/Cargo.toml -p <verified-package-name> <verified-test-filter>`

Replace angle-bracket placeholders only after confirming real names. Also run the repository's existing relevant test command for the affected Rust crate. Do not run destructive restore commands or tests against production systems.

## Coverage targets and data

- Minimal scope: requirement-driven tests, one verifiable test per requirement at the narrowest effective level, with a happy-path floor per component; existing relevant suite remains green.
- Use temporary directories for local archive/manifest tests and in-memory fake adapters for object store and database process boundaries.
- Use generated test keys and sentinel secret values; assert secrets are absent from logs/errors.
- Test both complete and partial-failure results; incomplete operations must not be represented as successful.
- No numeric project-wide coverage floor was established in Requirements Analysis; preserve any existing configured threshold and do not lower it.
