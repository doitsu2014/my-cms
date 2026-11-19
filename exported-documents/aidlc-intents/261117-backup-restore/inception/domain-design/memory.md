<!-- INVARIANT: examples are single-line HTML comments so a fresh template parses to total=0 (MEMORY_EMPTY). Do NOT un-comment or split across lines. t100 guards this. -->
> This file is kept up to date automatically while the stage runs. Add observations at the review step, not by editing here directly.

## Interpretations
- 2026-11-17T07:43:53Z — Added a dedicated BackupRestore orchestration component because jobs span database, GoTrue and Storage and require one lifecycle/integrity/recovery owner; source data remains owned by existing domains/platforms.
- 2026-11-19T05:52:00Z — User confirmed `domain_backup_restore` as a distinct Rust crate registered in the existing gateway process, owning durable job metadata and its migration; no separate service/process. Package bytes belong in a future configured private object store, not the metadata database; U1 real-data staging stays disabled until TLS/encryption/limits/cleanup gates pass.
<!-- example: 2026-05-29T10:14:32Z — chose REST over GraphQL; the consuming team only needs CRUD, revisit if subscriptions land -->

## Deviations
<!-- example: 2026-05-29T10:14:32Z — skipped the optional caching layer the stage prose suggested; the dataset is small enough that it adds risk -->

## Tradeoffs
<!-- example: 2026-05-29T10:14:32Z — picked TDD over BDD this run; the team is unit-first and the domain is well-understood -->

## Open questions
- 2026-11-17T07:47:20Z — Architecture reviewer dispatched but review file has not appeared after wait checks; waiting before recording a verdict.
<!-- example: 2026-05-29T10:14:32Z — confirm the retention window with compliance before the next stage hardens the schema -->
