# Units of Work — Backup & Restore

## Decomposition rationale

The approved decomposition uses four coarse-grained, independently testable vertical slices. U1 is the smallest integrated localhost slice, spanning the protected API, durable job foundation, worker, archive integrity, and a deliberately incomplete database-only backup path. U1 must not claim a complete or restorable backup while required Auth and Storage content is absent. U2 completes backup coverage; U3 establishes safe, non-mutating restore preflight; U4 executes restore with explicit recovery outcomes. The DAG expresses build/test dependencies only. Delivery Planning selects economic sequencing separately.

The initial runtime is localhost-only and does not authorize production restore or define production deployment topology. The proposed local Tokio task and existing PostgreSQL metadata store follow the answered assumptions; restart, lease, and recovery semantics remain for later design. The exact supported database tables, GoTrue export/import capabilities, Storage scope, object-store configuration, archive limits, version matrix, and rollback mechanisms remain subject to downstream verification and design.

## Unit catalogue

| Unit ID | Directory | Kind | Responsibilities and deliverables | Deployment model | Complexity |
|---|---|---|---|---|---|
| U1 | `u1-backup-foundation` | `service` | Integrate configured-instance selection and server-side admin boundary with persisted job/package metadata, idempotency/target-lock foundation, local background execution, and safe archive/manifest/checksum primitives. Implement a narrow database-only backup vertical slice for localhost integration. Clearly mark this output as incomplete and unavailable as a complete/restoreable backup until required Auth and Storage capture is delivered. | Embedded in the existing gateway process for localhost; no independent production service claim. | L |
| U2 | `u2-complete-backup` | `service` | Add supported Auth/GoTrue and Supabase Storage capture to the backup flow; produce versioned package inventory, per-component integrity evidence and truthful best-effort consistency statement; enforce required-component failure semantics; persist completed package metadata and enable authorized download/long-term configured object-store integration. | Shared gateway worker/runtime and configured object store; exact deployment topology deferred. | L |
| U3 | `u3-restore-preflight` | `service` | Accept bounded private uploads; inspect archive without executing package content; reject unsafe paths, symlinks, conflicts, bombs and configured-limit violations; verify manifest, digests, required components and supported compatibility; show target and replace impact; require recovery-point feasibility and explicit confirmation before any target mutation. | Embedded in the existing gateway process and configured staging/object storage for localhost. | L |
| U4 | `u4-restore-execution` | `service` | Execute approved restore operations across supported database/Auth/Storage adapters; serialize conflicting target jobs; validate each component after restore; record rollback completion or `recovery-required`; expose safe job history, progress, audit references and cleanup outcomes. | Embedded localhost worker/runtime; no production deployment claim. | XL |

## Unit boundaries and constraints

- `BackupRestore` owns backup/restore job lifecycle, package metadata, orchestration and recovery state; it does not take ownership of CMS business records, GoTrue identities or Storage objects.
- `BackupAdminApi` is the authorization and configured-instance boundary. Browser input cannot supply arbitrary endpoints, credentials or service-role secrets.
- `BackupWorker` performs long-running work outside the request lifetime. Durable state is authoritative; in-process task execution must not imply crash-safe resume until leases/restart behavior is designed and tested.
- Adapter contracts remain narrow and capability-aware. Unsupported Auth export/restore or unverified database scope must fail closed rather than be reported as covered.
- Packages are data only; archive entries never cause command or script execution. Checksums and archive bounds are validated before restore.
- Restore is replace-in-scope, not merge. Upload, preflight, cancellation and lack of confirmation must leave the target unchanged. Success requires all required post-restore checks.
- Cross-system work is best-effort/saga-style, not globally atomic. On failure, report verified rollback or `recovery-required`; never report partial success as success.
- Retention, size, encryption-at-rest, supported version matrix, credentials/configuration, and operational limits must be explicit before real data use.

## Traceability overview

| Unit | Primary upstream stories | Scope summary |
|---|---|---|
| U1 | US1.1, US1.2 (initial integration slice only), US3.2 (audit foundation) | Configured source/admin boundary, durable job foundation, archive/integrity groundwork and clearly incomplete database-only end-to-end slice. |
| U2 | US1.2, US1.3, US3.1, US3.2 | Complete supported backup package, verified Auth/Storage capture, authorized download and job outcome visibility. |
| U3 | US2.1, US2.2 (preconditions and confirmation boundary), US3.2 | Safe upload and non-mutating preflight, compatibility and recovery-point gates. |
| U4 | US2.2, US3.1, US3.2 | Restore execution, post-validation, rollback/recovery state, concurrency and operator history. |

Every story row in `unit-of-work-story-map.md` is assigned to one or more implementing Units; a Unit named for a story is responsible for tracing all of that story's acceptance criteria in its Functional Design. The U1 row for US1.2 is explicitly a partial integrated slice, not completion of that story's full acceptance criteria; U2 is also assigned to US1.2 and owns completion of its required component coverage. NFR and cross-cutting constraints are mapped in the story map.
