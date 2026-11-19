<!-- INVARIANT: examples are single-line HTML comments so a fresh template parses to total=0 (MEMORY_EMPTY). Do NOT un-comment or split across lines. t100 guards this. -->
> This file is kept up to date automatically while the stage runs. Add observations at the review step, not by editing here directly.

## Interpretations
- 2026-11-17T05:00:00Z — Treated `docs/features/backup-restore.md` as target business intent, not proof of current implementation; the partial CodeKB and practices record set explicit bounds on current claims.

## Deviations

## Tradeoffs
- 2026-11-17T05:00:00Z — Resolved initial scope to self-hosted Supabase, My-CMS/Auth/Storage, best-effort consistency, and compatible cross-instance restore from human answers. Production restore is explicitly excluded from the first release due to localhost-only practice.

## Open questions
- 2026-11-17T05:00:00Z — Exact Supabase/Postgres versions, Auth password-hash recovery method, bucket/object scope, package size/retention, encryption after download, authorization roles, and RPO/RTO remain unresolved and need explicit constraints or assumptions in requirements.
- 2026-11-17T06:18:10Z — The review request was dispatched but no review artifact has appeared yet; waiting for reviewer output before recording a verdict.
