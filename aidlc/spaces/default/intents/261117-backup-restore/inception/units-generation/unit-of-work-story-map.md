# Unit-to-Story Map — Backup & Restore

| Upstream item | Implementing Unit(s) | Coverage notes |
|---|---|---|
| US1.1 | U1 (`u1-backup-foundation`) | Configured instance selection, admin-only server enforcement, and rejection of arbitrary browser endpoints/credentials. |
| US1.2 | U1 (`u1-backup-foundation`); U2 (`u2-complete-backup`) | U1 supplies a database-only integration slice and must not claim full backup success; U2 adds required Auth and Storage capture, complete package, integrity, and truthful terminal status. All acceptance criteria are complete only with U2. |
| US1.3 | U2 (`u2-complete-backup`) | Verified package storage/download, policy/configuration refusal, sensitive-file warning, and authorization. |
| US2.1 | U3 (`u3-restore-preflight`) | Bounded private upload, safe archive validation, package/target compatibility and non-mutating preflight. |
| US2.2 | U3 (`u3-restore-preflight`); U4 (`u4-restore-execution`) | U3 owns explicit replace-impact confirmation and verified recovery-point preconditions; U4 owns execution, post-validation, rollback/recovery-required outcomes and target concurrency. |
| US3.1 | U2 (`u2-complete-backup`); U4 (`u4-restore-execution`) | Backup job/package status in U2; restore outcome/history, failure state and cleanup in U4. |
| US3.2 | U1 (`u1-backup-foundation`); U2 (`u2-complete-backup`); U3 (`u3-restore-preflight`); U4 (`u4-restore-execution`) | Audit foundation and non-sensitive actor/source/target/job/package/digest/outcome references across sensitive operations; U4 includes restore confirmation/recovery outcome. |
| NFR1 | U1 (`u1-backup-foundation`); U2 (`u2-complete-backup`); U3 (`u3-restore-preflight`); U4 (`u4-restore-execution`) | Server-side authorization, server-only configuration/secrets, secure transport, and redacted errors across all boundaries. |
| NFR2 | U2 (`u2-complete-backup`); U3 (`u3-restore-preflight`) | Package handling, configured private long-term storage, at-rest protection prerequisites, and handling warnings. |
| NFR3 | U1 (`u1-backup-foundation`); U2 (`u2-complete-backup`); U3 (`u3-restore-preflight`); U4 (`u4-restore-execution`) | Integrity evidence, count/size checks, and explicitly best-effort consistency from capture through restore validation. |
| NFR4 | U1 (`u1-backup-foundation`); U3 (`u3-restore-preflight`) | Bounded package creation primitives and bounded upload/archive inspection before target mutation. |
| NFR5 | U1 (`u1-backup-foundation`); U2 (`u2-complete-backup`); U4 (`u4-restore-execution`) | Job timing/size/component outcome and recovery status; concrete RPO/RTO remain open for design, not assumed here. |
| NFR6 | U1 (`u1-backup-foundation`); U2 (`u2-complete-backup`); U3 (`u3-restore-preflight`); U4 (`u4-restore-execution`) | Test-after per layer, coverage/CI objectives and localhost-only verification constraints apply throughout. |
| NFR7 | U1 (`u1-backup-foundation`); U2 (`u2-complete-backup`); U3 (`u3-restore-preflight`); U4 (`u4-restore-execution`) | Accessible UI states, keyboard operability and safe actionable errors apply across each UI slice. |

## Cross-cutting scope and constraints

- The scope is configured Supabase self-hosted instances on localhost; production restore is out of scope.
- Restore defaults to replace within the supported scope, not merge. No target mutation occurs before explicit confirmation and recovery-point verification.
- No client-provided endpoints, credentials, or scripts from an archive are trusted or executed.
- Auth export/password/session semantics, supported schema/table scope, cross-instance compatibility, object-store limits/retention/encryption, archive limits and rollback capabilities require explicit later verification/design.
- Mapping rows are not a claim that an unresolved platform capability exists. If a required capability cannot be verified, dependent behavior must fail closed and be represented honestly.
