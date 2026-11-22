# Problem Statement

## Problem
My-CMS administrators do not yet have the intended admin-UI workflow to create a verified backup package and upload/preflight a package for restore into a configured instance. The feature specification describes the target experience as a ZIP package covering the approved database, Auth, and Storage scope, but warns that full Supabase export/recovery support must be verified before making such a claim (`docs/features/backup-restore.md`, §§1–2, 9).

## Observed current state
- The project is a brownfield Rust API gateway, React admin UI, and self-hosted Supabase system (`.aidlc/knowledge/_codebase-map.md`, §§1–4).
- A backup CLI skeleton exists, but the feature specification reports no complete ZIP workflow, admin UI, upload/import restore flow, or instance selection (`docs/features/backup-restore.md`, §9; codebase map §3).
- The prior U1 code-generation attempt made no application source changes. It stopped on unresolved persistence/migration and cleanup/reconciliation ownership, a job-status contract mismatch (`incomplete` versus the published enum), and provider staging questions (`docs/features/backup-restore.md`, §12).

## Intended outcome
Develop the feature in the currently documented and approved scope, preserving the distinction between the long-term product target and the constrained U1 localhost work. U1 is localhost-only and database-only; a database-only job must end `incomplete` or `failed`, never `succeeded`, and incomplete artifacts must not be downloadable. Real-data staging remains disabled until finite expiry and durable cleanup/reconciliation after crashes are designed and tested (`docs/features/backup-restore.md`, §§10–13).

This framing does not select an implementation or resolve outstanding decisions. It does not authorize production restore, full Auth/Storage backup, or broader Supabase-managed system data.

## Affected users
System administrators and operators need safe, observable backup/restore procedures; CMS end users benefit indirectly from recoverable data. Product/system owners must resolve policy and scope decisions. See `stakeholder-map.md`.

## Material open questions
See the feature specification §8 and the analysis artifacts. Notably unresolved are supported Supabase versions/hosting, exact schema scope and Auth recovery, consistency, cross-instance mapping, package limits/retention/storage, production authorization, and RPO/RTO/restore-drill targets. The U1 design blockers in §12 also remain open.