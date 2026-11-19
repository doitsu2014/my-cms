# Unit Dependencies — Backup & Restore

## Dependency meaning

An edge `A depends on B` means A's code or tests need B's delivered code to build or pass. It does not prescribe an economic delivery order or critical path; Delivery Planning owns those decisions. There are no independent Units in this approved decomposition, so this DAG exposes no parallel Unit development opportunity.

## Direct dependency DAG

- `u1-backup-foundation`: no Unit dependencies. This first Unit is the integrated localhost slice: configured admin/API path, durable job foundation, worker, safe package primitives, and a deliberately incomplete database-only backup path. It can run before later Units because the slice proves the request-to-job-to-worker-to-package-integrity path using the project's existing database and does not claim Auth/Storage completeness or a usable complete restore package.
- `u2-complete-backup`: depends on `u1-backup-foundation`; extends its job and archive foundation with verified Auth and Storage capture and complete backup behavior.
- `u3-restore-preflight`: depends on `u2-complete-backup`; consumes the package format, integrity evidence, and component inventory established by complete backup, and adds safe upload/preflight without target mutation.
- `u4-restore-execution`: depends on `u3-restore-preflight`; consumes the approved preflight and recovery-point boundary before performing replace-in-scope restore and post-restore validation.

## Integration points

| Producer / prerequisite | Consumer | Contract boundary | Failure behavior |
|---|---|---|---|
| U1 job/persistence and configured-instance API | U2 backup capture | Stable job/package metadata, configured source IDs, safe status/error projection | Fail closed for missing configuration or invalid authorization; no client-supplied endpoints/secrets. |
| U1 archive primitives + U2 component inventory/capture | U3 preflight | Versioned manifest, per-component paths/counts/size/digests, best-effort consistency statement | Reject malformed/unsupported/incomplete packages before any target mutation. |
| U3 validated package and target compatibility result | U4 restore execution | Preflight result bound to package digest, target ID and configuration/version data; explicit confirmation and recovery-point evidence | Invalidate/reject if package, target, or relevant compatibility data changes; require a new preflight/confirmation. |
| U1-U4 persisted job transitions | Admin UI/history and audit | Safe DTOs with operation, status, progress if measurable, timestamps, actor/source/target references, digest and outcome | Never expose credentials, raw rows, object contents, or unsanitized upstream errors; failed audit policy must be explicit. |

All adapters retain source-data ownership. A cross-system restore is coordinated as a recoverable saga/state machine, not a global transaction; component failure must lead to verified rollback or `recovery-required`.

## Parallel development opportunities

None under the current DAG: every later Unit depends on its predecessor. Within a Unit, implementation tasks may be internally parallelized only where the Unit's own interfaces permit it; that does not create independent Unit nodes in this DAG.

## Machine-readable dependency block

```yaml
units:
  - name: u1-backup-foundation
    kind: service
    depends_on: []
  - name: u2-complete-backup
    kind: service
    depends_on: [u1-backup-foundation]
  - name: u3-restore-preflight
    kind: service
    depends_on: [u2-complete-backup]
  - name: u4-restore-execution
    kind: service
    depends_on: [u3-restore-preflight]
```
