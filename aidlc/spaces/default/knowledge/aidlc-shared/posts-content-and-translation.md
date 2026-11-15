# Posts, GraphQL, translation, and search

Behavior narrative: `docs/features/ai-translation.md`.
Mount ownership: `backend-domain-architecture.md`.

Validate: `apps/api/domain_posts/src/handlers/`,
`apps/api/domain_posts/src/domain/graphql.rs`,
`apps/api/domain_posts/src/domain/vector_store.rs`,
`apps/api/domain_posts/src/handlers/post/translate/`.

## Posts domain

`domain_posts` owns post and category CRUD, the tag helper, the AI model
catalogue, translation jobs, GraphQL schema contribution, entities, and
migrations. HTTP adapters in `domain_posts::api` call those handlers and
return the existing response envelope.

Lookup for a translation is three tiers, in order:

1. Exact row for `(post_id, language_code)`.
2. pgvector cosine match at similarity `>= 0.95`.
3. OpenAI chat completion.

A database hit skips later tiers. Similarity-reused text is not run through
the paragraph-coverage validator. A newly generated HTML translation is
validated before save.

## Paragraph coverage

WHEN source HTML has one or more `<p>` tags and the translation has fewer
than `max(1, source_count / 2)` paragraphs, THEN the handler returns
`AppError::Validation("translation", ...)` and MUST NOT write or overwrite
`post_translations`.

The HTML system prompt tells the model to keep every `<p>`, not to return
headings only, and not to truncate. Rejections log a warning with post id,
language, and both paragraph counts. The prompt constant is `pub(crate)` so
the colocated regression test can read it.

Embeddings use `text-embedding-3-small` (`vector(1536)`) over title plus up
to 8,000 characters of content. Search uses pgvector cosine distance and the
ivfflat index. `(post_id, language_code)` is unique; writes upsert. If the
pgvector extension is unavailable, migration skips the embeddings table and
translation continues without semantic reuse. A missing `OPENAI_API_KEY`
fails post-domain configuration validation.

GraphQL keeps the historical seven-entity schema plus `CategoryType`.
Mutation gating and unlimited depth/complexity stay until a later change
says otherwise.

## Admin translation refresh

The admin edit form MUST NOT poll `GET /posts/{post_id}/translate/jobs`.

- WHEN an author starts a background translation, THEN the server job is
  created and the form values, TipTap state, and open Article Preview stay
  as they are, including a visible Mermaid snapshot.
- WHEN that job finishes while the author has unsaved edits, THEN the form
  does not reload or overwrite them. The result appears after an explicit
  refresh or after reopening the form.
- Refresh warns through the existing unsaved-changes pattern, or reloads
  immediately when the form is clean. Refresh does not start polling.
