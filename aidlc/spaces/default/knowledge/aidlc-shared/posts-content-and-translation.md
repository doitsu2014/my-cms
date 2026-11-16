# Posts, content, and translation

[Glossary](glossary.md) · Mount boundary: [backend domain architecture](backend-domain-architecture.md) · Maintained narrative: [`docs/features/ai-translation.md`](../../../../../docs/features/ai-translation.md)

## Evidence boundary and ownership

`domain_posts` is the intended owner of post/category/tag behavior, translations, translation jobs, GraphQL contribution, persistence entities, and post migrations. Relevant paths are `apps/api/domain_posts/src/handlers/`, `src/domain/graphql.rs`, `src/domain/vector_store.rs`, and `src/handlers/post/translate/`. The scan is partial; exact route contracts, persistence semantics, and behavior below not verified by direct source review are **Unverified**. The maintained translation guide is supporting context, not proof.

## Translation contract

Intended normal lookup order: exact `(post_id, language_code)` translation; then pgvector cosine similarity at `>= 0.95`; then OpenAI generation. Reused text skips later tiers. Newly generated HTML is checked for paragraph coverage before save. Existing knowledge says a failed validation must not write or overwrite `post_translations`; exact threshold implementation and transaction behavior are **Unverified**.

The intended paragraph rule rejects a generated translation when source HTML has paragraphs and translated paragraph count is below `max(1, source_count / 2)`. Intended embedding model is `text-embedding-3-small`, using title plus up to 8,000 content characters, with vector dimension 1536 and cosine search. Missing pgvector is intended to disable semantic reuse without disabling translation; missing `OPENAI_API_KEY` is intended to fail domain configuration. Confirm all details against current migration, configuration, handlers and tests before relying on them.

Existing knowledge describes admin background translation as non-polling: starting a job preserves current form/TipTap/preview state, and completion must not overwrite unsaved edits; result appears after explicit refresh or reopening. Refresh follows the existing unsaved-change confirmation and does not start polling. This is **Unverified** against current admin source. Do not reintroduce polling absent owner-approved evidence.

## GraphQL

Posts owns the schema contribution and mounts immutable GraphQL publicly and mutable GraphQL behind the protected writer-or-administrator policy. The seven historical entities plus `CategoryType`, mutation gating, and depth/complexity policy are stated in the existing contract, but the exact current schema and limits are **Unverified**. See the shared [mount and role contract](backend-domain-architecture.md).

## Unresolved and follow-up

- **Unverified:** translation lookup/write atomicity, model/provider payloads, job lifecycle and UI refresh integration, schema contents and limits.
- The maintained guide describes prior polling and reload-on-completion, while the current shared contract explicitly prohibits admin polling and overwrite of unsaved edits. The CodeKB is not deep enough to settle implementation status. **Unresolved:** inspect current admin and handler source; record both claims and request project-owner decision if source does not settle intended behavior.
