# Interaction Specification — Backup & Restore

## Global navigation

- Entry point: Admin navigation item **Backup & Restore**.
- Page-level tabs: **Tạo backup** and **Khôi phục**; switching tabs preserves current entered state unless navigation/session expires, in which case unsaved upload/config selections must not be silently discarded.
- Labels and status text use the existing admin localization mechanism.

## Backup flow

1. **Select source**
   - Required field: configured self-hosted instance.
   - Display environment label and a non-secret identity hint; never display connection credentials.
   - Empty list: explain there is no configured source and disable submit.
2. **Start job**
   - Primary action: `Tạo backup`.
   - Disable while request is submitting; use server idempotency/deduplication to protect retries.
   - After accepted response, show job ID and job detail row.
3. **Running**
   - Poll or refresh by existing job status contract (transport TBD); show statuses by component only when known.
   - Keep failure distinct from partial/succeeded. No fabricated percent completion.
4. **Complete**
   - Enable download only after server verified package completion/integrity.
   - Show creation time, size and checksum/integrity result.
5. **Failure**
   - Explain component and next action; preserve safe diagnostic reference/job ID.
   - Never expose credentials, raw rows or object contents.

## Restore flow

1. **Select package and target**
   - Package ZIP and configured target are required.
   - Validate extension/client size for usability, but server remains authoritative.
   - Upload progress displays transferred bytes/total where available.
2. **Preflight**
   - Run archive safety checks, manifest/version, integrity checks and target compatibility.
   - Target remains read-only during preflight.
   - Present source label, target label, package ID, included scopes, replace semantics, compatibility, consistency caveat and warnings.
   - If any blocking check fails, show failed state with reason; disable continuation.
3. **Explicit confirmation**
   - Dedicated confirmation screen, not an implicit consequence of upload.
   - Prominently name the target and state data to be replaced.
   - Require exact target-label re-entry as an additional mis-target safeguard.
   - Require backend confirmation/recovery-point readiness before enabling final action.
   - `Quay lại` returns to preflight without mutation; cancel leaves target unchanged.
4. **Run restore**
   - Disable duplicate submit. Backend rejects/serializes concurrent jobs on same target.
   - Display overall job and component steps (recovery point, DB, Auth, Storage, validation) when supported.
   - Do not imply atomic cross-system transaction where none exists.
5. **Terminal state**
   - Success only after all required components and validation pass.
   - Failure identifies rollback-completed vs recovery-required; show safe next steps and audit/job ID.
   - Failed/partial target is never represented as success.

## Job history and detail

- History includes job ID, type, source/target, created time, terminal/current status, package metadata and action availability.
- Default sort newest first; pagination/cap is a design decision based on existing admin patterns.
- Expand job for component detail, consistency statement, checksum result and safe error summary.
- Resource access and downloads are server-authorized per request.

## Errors and confirmations

- Field errors stay next to the affected field and preserve user input.
- Network/service errors distinguish retryable from terminal where backend supplies classification.
- A retry of a create request must use an idempotency key or retrieve the existing job; no duplicate backup job from double-click/retry.
- Destructive confirmation must use explicit, localized wording and identify exact target; never rely on color alone.
- Session expiry during upload/confirmation preserves user selections where safe and requires re-authentication; re-check authorization before subsequent mutation.
