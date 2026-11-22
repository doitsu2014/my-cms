# KL-01 Known limitations: WU-01

- **ID:** KL-01 (WU-01)
- **Stage:** Develop / build-and-verify, unit WU-01.
- **Status:** Open items carried to later units or to the release-candidate gate.

| # | Limitation | Impact | Owner or next step |
|---|---|---|---|
| L-01 | Docker Desktop does not start on this host, and there is no local PostgreSQL | No testcontainers test can run. The RK-15 shared-migration-table check is unverified. WU-02 onward cannot get DB-backed green evidence here. | Owner: restore Docker Desktop before WU-02 build. The conductor then runs RK-15 and the `--ignored` baseline. |
| L-02 | Unix-only `NotPrivate` branch and its test are not compiled on this Windows host | The 0700 check is unverified | Run `cargo test -p domain_backup` on Linux/CI before the release-candidate gate |
| L-03 | Windows owner-only ACL is not checked (T-14 partial) | Staging-directory privacy on Windows rests on the directory location only | Owner accepted this as a residual for synthetic-only data (2026-11-22). Re-check at the release-candidate gate. |
| L-04 | `BackupConfig::load` validates the storage root without the real forbidden set | A root under a forbidden application path is only refused when consumers call `validate_storage_root` with that set | WU-04 (fixture storage) must pass the forbidden set at first use. Documented on `BackupConfig`. |
| L-05 | CFG-17/CFG-18 accept any positive value up to `u64::MAX` | No sanity ceiling until the owner sets values | Owner decision needed before WU-10. The question should include an upper bound. |
| L-06 | 12 hardening tests are test-after, not red-first | They lack red evidence. A mutation proof stands in for it. | Accepted deviation, recorded in the green report section 3 |
| L-07 | Test modules were reformatted by `cargo fmt` after red | Whitespace only; verified equal to the red snapshot after stripping whitespace and trailing commas | Recorded. Future red stages should run `cargo fmt` before the red snapshot. |
| L-08 | `project_error` and `SystemClock` have no tests in WU-01 | Safe error projection (O-02) is unverified | WU-08 error-projection tests and WU-15 canary scan |
| L-09 | A failed or interrupted run of `storage_root_rejects_forbidden_and_relative_paths` can leave a temp directory with a scratch junction in `%TEMP%` (seen after earlier red runs; none left after the final green run). The junction points to a scratch folder, not the repository. | Disk litter on failed runs only | Test-hygiene fix to the Drop guard in a later red stage, as a reviewed test change |
| L-10 | Not composed into the gateway; no API, UI or persistence | WU-01 delivers no user-visible behaviour | WU-02..WU-15 |
| L-11 | Bracketed IPv6 origins (`http://[::1]:3000`) are refused by CFG-16 validation | A dev origin must use an IPv4 or hostname form | Accepted for U1 (comment in code). The dev-proxy path (owner decision E2E-ENV-01) uses a same-origin `localhost` origin. |

## Rollback

`domain_backup` is not referenced by any other crate. To roll back, remove the `domain_backup` member line from `apps/api/Cargo.toml` and the directory `apps/api/domain_backup/`, then let cargo prune the `Cargo.lock` entry. No database, configuration or runtime state is affected.
