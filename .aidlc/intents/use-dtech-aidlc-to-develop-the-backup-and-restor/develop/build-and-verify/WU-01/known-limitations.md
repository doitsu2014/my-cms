# KL-01 Known limitations: WU-01

- **Stage:** Develop / build-and-verify, work unit WU-01.
- **Status:** Refreshed 2026-11-25. WU-01 focused suites are green; aggregate workspace regression is red and this verification cannot be treated as an overall green stage.
- **Scope:** Existing WU-01 implementation verification only; no source or behavior changes made.

| # | Limitation | Impact | Owner or next step |
|---|---|---|---|
| L-01 | Current normal workspace run failed with 13 lifecycle tests; current Docker-ignored workspace run failed with 3 reconciliation/JobStore tests. Every passing baseline identity passed, but neither current failing set is empty. | Strict regression rule is not satisfied; the release-candidate gate is blocked. These test identities are outside WU-01 focused scope, and this refresh assigns no cause to WU-01. | Keep failures with their registered work-unit ownership. WU-16 work must precede WU-06's deferred strict full normal/ignored regression closure; rerun both required workspace suites before claiming green regression. See `regression-report.md`. |
| L-02 | Unix-only `NotPrivate` branch and its test are not compiled on this Windows host. | Unix directory mode 0700 validation is unverified here. | Verify on Linux/CI before release-candidate review. |
| L-03 | Windows owner-only ACL is not checked (T-14 partial). | Storage privacy on Windows rests on the configured directory location; this is an accepted residual for synthetic-only U1. | Recheck the residual at the release-candidate gate. |
| L-04 | `BackupConfig::load` validates the storage root without a deployment-specific forbidden-path set. | A root under an application/static path is rejected only when a consumer supplies the actual forbidden set to `validate_storage_root`. | Fixture storage consumer must provide the forbidden set before first use. |
| L-05 | CFG-17 and CFG-18 accept positive values without an approved maximum; their values have no approved defaults. | Configuration bounds remain unresolved before ingress-layer implementation. | Owner values and bounds are required before WU-10. |
| L-06 | Some hardening tests were added after the original red run. | These tests do not have original red evidence; the earlier report contains mutation-test evidence, not a fresh red-first record. | Carried as an existing deviation; do not represent mutation evidence as red-first. |
| L-07 | `project_error` and `SystemClock` have no dedicated WU-01 tests. | Safe error projection and clock behavior remain unverified by this WU. | Cover in their consuming units as designed. |
| L-08 | WU-01 remains a library-only slice and is not composed into the gateway. | No API/UI/persistence behavior, server authorization or end-to-end behavior is verified by WU-01. | Later gateway/domain work owns integration. |
| L-09 | Bracketed IPv6 origins such as `http://[::1]:3000` are rejected by CFG-16 parsing. | A dev origin must use a supported IPv4/hostname form. | Existing U1 design limitation; no change made. |
| L-10 | Workspace clippy exits successfully but reports warnings, including findings in the current output. | Successful exit does not mean warning-free code. | Review and resolve warnings within their owning units; baseline comparison should distinguish existing from newly introduced warnings. |
| L-11 | The task's starting worktree contains unrelated/later-unit modifications, including lifecycle code and test files, plus other intent artifacts. | Full-suite failures may reflect this broader worktree state; this verification does not assign authorship or causality to WU-01. | Preserve those changes; diagnose with the owning work and recorded version history. |
| L-12 | `dtech-aidlc ids check --stage build-and-verify` reports registered IDs clean but also lists uncovered functions assigned to later work units. | This is not a WU-01 identifier failure; the check does not establish completion of the whole intent. | Continue coverage in the responsible work units; no identifier was minted here. |

## Rollback and evidence

No source or test changes were made during this WU-01 refresh. The four declared WU-01 build-and-verify artifacts were refreshed and raw command evidence was added under the existing evidence directory. No workflow state, audit, baseline or red-test artifact was edited. Fresh logs record command, cwd, stdout, stderr and exit code. The release-candidate gate remains blocked by aggregate regression failures.
