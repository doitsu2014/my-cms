# Stakeholder Map

## Direct users
- **System administrator (feature operator):** selects an allowed instance, starts and monitors backup jobs, downloads only verified eligible packages, uploads packages, reviews preflight, and explicitly confirms any permitted restore. Needs clear environment labels, safe errors, impact disclosure, and unambiguous status.
- **Operations staff (incident responder):** monitors jobs and handles failed or recovery-required outcomes. Needs traceable references, safe diagnostics, and known recovery procedures.

## Affected users
- **CMS end users:** do not operate this feature; their content, identity, media, and service availability are affected by backup/restore quality and any restore interruption.

## Decision owners and delivery partners
- **System/product owner:** owns feature scope and policy decisions, including supported environments/versions, data inclusion, consistency, package limits/storage/retention, production authorization, recovery targets, and any second-person approval.
- **Business/product analyst:** clarifies and prioritizes unresolved requirements without assuming technical feasibility.
- **Development team:** resolves documented U1 design blockers and implements within approved boundaries.
- **QA/test team:** verifies behavior, security boundaries, failure handling, and restore drills using approved representative environments.
- **Security/compliance reviewers:** assess authorization, secret handling, encryption, audit, package safety, and retention controls where applicable.

## Dependencies and open ownership
The feature specification identifies outstanding decisions about persistence/migration ownership, durable cleanup/reconciliation, status-contract mapping, provider staging, and broader Supabase recovery support (`docs/features/backup-restore.md`, §§8, 12). Named individuals or formal approval roles are not identified in available project context; ownership is therefore unresolved rather than inferred.