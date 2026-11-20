# Team Allocation — Backup & Restore

## Ownership model

This allocation follows the Units and responsibilities in [`unit-of-work.md`](../units-generation/unit-of-work.md) and their sequential dependency structure in [`unit-of-work-dependency.md`](../units-generation/unit-of-work-dependency.md). The `classic` scope does not run Team Formation. All four Bolts are owned by `aidlc-developer-agent` (AI). There is no named human team, external delivery team, or agreed support rotation in the available inputs; this plan does not invent one. Human approvals remain at the workflow's design, verification-command and Unit checkpoint gates.

A **Bolt** is one Construction pass for a Unit ending in a verified outcome. A **mob** is the group working together on that Bolt; here the AI developer agent is the assigned executor, while the human remains the decision-maker at explicit gates. Review and specialist input occur through the workflow's declared architecture/security/quality reviews, not through an assumed external team.

| Bolt | Unit | Owner | Supporting expertise | Coordination / gate |
|---|---|---|---|---|
| B1 — Local job-to-package skeleton | U1 `u1-backup-foundation` | `aidlc-developer-agent` | Architecture for state and API boundaries; quality for integration and fail-closed tests; security for configured-instance authorization and archive safety. | Unit design approvals, human-authorized verification command, integrated localhost skeleton checkpoint. |
| B2 — Complete supported backup | U2 `u2-complete-backup` | `aidlc-developer-agent` | Architecture and platform expertise for database/Auth/Storage capabilities; security for package handling; quality for counts, checksums and coverage. | Capability evidence must precede any complete-backup claim; Unit checkpoint follows tests. |
| B3 — Safe restore preflight | U3 `u3-restore-preflight` | `aidlc-developer-agent` | Security for archive and upload boundaries; architecture for compatibility/preflight contract; UX/accessibility expertise for non-mutating status and replace-impact display. | Require evidence that rejected/preflight-only package paths do not mutate the target. |
| B4 — Controlled restore and recovery | U4 `u4-restore-execution` | `aidlc-developer-agent` | Architecture for saga/recovery; security for destructive confirmation and secrets; quality for failure-injection, rollback and post-restore validation. | Require a disposable configured target and verified recovery-point procedure before any mutation tests. |

## Collaboration and escalation

- Unit dependencies are serial; no parallel multi-Unit mob is assumed.
- Escalate unresolved platform capability to the project/platform owner once identified; no owner or response time has yet been confirmed.
- If GoTrue/Auth export or restoration semantics cannot be verified, stop the complete-backup/restore claim and surface the scope decision; do not substitute an unsupported implementation.
- If a recovery point cannot be created and verified, do not start target mutation.
- If real-data policy (object-store access/encryption, retention, size bounds) is missing, keep the work limited to controlled localhost fixtures and fail closed for real data.
