# IMPL-01 Implementation manifest: WU-01

- **ID:** IMPL-01 (WU-01)
- **Stage:** Develop / build-and-verify (builder-verifier), unit WU-01 of WU-01..WU-15.
- **Status:** WU-01 implemented and verified on this host 2026-11-22. Uncommitted; on branch `doitsu2014/migrate-exported-documents-to` (owner rule 2026-11-22: branch-only until Launch, no merge to `main` during Develop).
- **Inputs:** [red-tests/WU-01/failing-tests.md](../../red-tests/WU-01/failing-tests.md), [red-evidence.md](../../red-tests/WU-01/red-evidence.md) (section 3.3 decisions), [work-units](../../implementation-design/work-units.md) WU-01, [interfaces-and-contracts](../../implementation-design/interfaces-and-contracts.md) section 8, [functional-design](../../implementation-design/functional-design.md) F-01, F-15, F-39.
- **Evidence:** [green-verification-report.md](green-verification-report.md), [regression-report.md](regression-report.md).

## Files

| Path | Change | Lines | Content |
|---|---|---|---|
| `apps/api/Cargo.toml` | modified | +1/-1 | workspace member `domain_backup` |
| `apps/api/Cargo.lock` | modified | +10 | `domain_backup` package entry; no new third-party crates |
| `apps/api/domain_backup/Cargo.toml` | new | 29 | deps `chrono`, `serde`, `thiserror`; dev-dep `serde_json`; versions match sibling crates; comments record deferred deps and the std `File::try_lock` decision |
| `apps/api/domain_backup/src/lib.rs` | new | 7 | crate root |
| `apps/api/domain_backup/src/domain/mod.rs` | new | 8 | module list |
| `apps/api/domain_backup/src/domain/config.rs` | new | 1237 (about 410 production, rest tests) | F-01, F-39 |
| `apps/api/domain_backup/src/domain/status.rs` | new | 150 | job status enum and U1 write/projection guard |
| `apps/api/domain_backup/src/domain/state_machine.rs` | new | 64 | F-15 pure transition guard |
| `apps/api/domain_backup/src/domain/error.rs` | new | 28 | `AppError` (thiserror) |
| `apps/api/domain_backup/src/domain/response.rs` | new | 35 | `project_error` safe error body |
| `apps/api/domain_backup/src/domain/clock.rs` | new | 18 | `Clock` port, `SystemClock` |

Nothing outside `apps/api/Cargo.toml`, `apps/api/Cargo.lock` and `apps/api/domain_backup/**` changed (Observed, `git status --short`). The gateway does not depend on or compose `domain_backup` yet (WU-10), so the running service is unaffected.

## Behaviour implemented

- **`BackupConfig::load(lookup)` / `from_env()` (F-01).** Decision order: `BACKUP_ENABLED` must be exactly `true`, else `NOT_CONFIGURED`. `HOST` (default `127.0.0.1`) must be a loopback IP literal, else `NON_LOOPBACK_BIND`. Then everything else, else `INVALID_CONFIG`:
  - CFG-05..CFG-14 and CFG-15 must equal the owner-approved values when present (24 h expiry, 1 h grace, 15 min sweep, 3 attempts, 60 s backoff, 10 min deadline, 1 concurrent job, 256 MiB, 1 GiB, 30 days, `project_owner`).
  - Numbers are canonical unsigned decimals only.
  - Instance id and label are non-empty with no surrounding whitespace.
  - The storage root (CFG-04) is required, absolute and passes `validate_storage_root` with no forbidden set.
  - Allowed origins (CFG-16) are `http(s)://host[:port]` with port 1..=65535, no path, wildcard or trailing slash. Bracketed IPv6 origins are out of scope for U1 (comment in code).
  - CFG-17/CFG-18 are required and non-zero.
  - `from_env` reads with `var_os` and maps non-UTF-8 values to an empty string, which fails validation.
  - Fields are private with read-only accessors, so no code outside `load` can build an `Enabled` config with other values.
- **`is_loopback_bind` (F-39).** True only for IP literals that std reports as loopback (127.0.0.0/8, `::1`). `[::1]`, `::ffff:127.0.0.1`, `0.0.0.0`, `::` and hostnames are false.
- **`validate_storage_root(root, forbidden)`.** Refuses, in order:
  - a relative path (`NotAbsolute`);
  - `..` or a verbatim/device prefix (`UnresolvedPath`);
  - a path under a forbidden path, compared case-insensitively on Windows (`UnderForbiddenPath`);
  - a junction or symlink on the root or any existing ancestor (`ReparsePoint`);
  - a metadata error other than not-found (`Inaccessible`);
  - an existing non-directory (`NotDirectory`);
  - on Unix, a mode other than 0700 (`NotPrivate`).

  A root that does not exist yet passes. Consumers must still call it with the real forbidden set before first use; `load` has no forbidden set (documented on `BackupConfig`).
- **`BackupJobStatus`.** Every documented value is kept and `incomplete` is added (ADR-002 as approved, D-10). Lowercase wire spelling, with `recovery-required` explicit.
- **`checked_status_for_write` / `project_status`.** One guard that refuses `succeeded` and `recovery-required` for the U1 job kind.
- **`is_allowed` (F-15).** Exactly queued to running, queued to failed, running to incomplete, running to failed.
- **`project_error`.** Maps each `AppError` to a fixed safe body (`BACKUP_DISABLED`, `JOB_NOT_FOUND`, `STORE_UNAVAILABLE`) and echoes the correlation id.
- **`SystemClock::now`.** Returns `Utc::now()`.

## How it was built (process record)

1. A builder (sonnet, per `stage.model`) wrote the minimal code to turn the 22 red tests green.
2. Two independent reviewers checked it:
   - Integrity: test modules byte-identical to the red snapshot; scope.
   - Correctness: 13 findings, including 4 major on `validate_storage_root`.
3. The fixer applied 8 findings to production code.
4. The conductor applied `cargo fmt` to the test modules as a formatting-only change. The conductor confirmed the whitespace-stripped content equals the red snapshot.
5. Because the review hardening was added after red, a builder added 12 test-after tests. Each was shown to fail when its check is disabled (mutation table in the green report). The production code was byte-compared to the snapshot after the mutations were restored.
6. The conductor ran the final green, regression, fmt and clippy runs independently.
