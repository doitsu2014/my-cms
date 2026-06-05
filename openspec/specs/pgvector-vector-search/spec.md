# pgvector-vector-search Specification

## Purpose
TBD - created by archiving change supabase-auth-and-pgvector-migration. Update Purpose after archive.
## Requirements
### Requirement: Embeddings are stored in PostgreSQL pgvector

The system SHALL store translation embeddings in a PostgreSQL `embeddings` table on the Supabase PostgreSQL instance. Each row SHALL contain a `vector(1536)` embedding, the `post_id` foreign key, the `language_code`, an optional `translation_id`, a `title`, a `content_preview`, and `created_at` / `updated_at` timestamps.

#### Scenario: First translation stored

- **WHEN** the `PostTranslateHandler` calls `VectorStore::store_translation` for a `(post_id, language_code)` pair that has not been seen
- **THEN** a row is inserted into `embeddings` with a fresh UUID
- **AND** the row contains an OpenAI `text-embedding-3-small` embedding of `title + content[:8000]`

#### Scenario: Re-translation updates existing row

- **WHEN** the `PostTranslateHandler` calls `VectorStore::store_translation` for an existing `(post_id, language_code)`
- **THEN** the existing row is updated with the new `translation_id`, embedding, title, and content preview
- **AND** `updated_at` is set to the current timestamp

### Requirement: Cosine-similarity search uses pgvector

The system SHALL perform similarity search against `embeddings` using the cosine distance operator (`<=>`) on the `vector(1536)` column, ordered by ascending distance and limited to the requested count.

#### Scenario: Search returns top N by similarity

- **WHEN** `VectorStore::search_similar_translations(text, limit=5)` is called
- **THEN** the system returns up to 5 rows ordered by `embedding <=> $1::vector`
- **AND** each result exposes a `score` field equal to `1.0 - (embedding <=> $1::vector)`

#### Scenario: Search above reuse threshold is detected

- **WHEN** the system finds a result with `score ≥ 0.95`
- **THEN** the calling handler treats that translation as a cache hit and reuses it instead of calling OpenAI for a new translation

### Requirement: Exact lookup by post and language

The system SHALL provide `VectorStore::find_translation(post_id, language_code)` that returns the matching `TranslationMetadata` row, or `None` if no such row exists.

#### Scenario: Lookup hit

- **WHEN** a translation for `(post_id=X, language_code='en')` exists
- **THEN** `find_translation(X, 'en')` returns the stored `TranslationMetadata`

#### Scenario: Lookup miss

- **WHEN** no translation for `(post_id=X, language_code='zz')` exists
- **THEN** `find_translation(X, 'zz')` returns `None`
- **AND** no error is raised

### Requirement: pgvector extension is enabled

The migration SHALL enable the `vector` extension on the Supabase PostgreSQL instance and SHALL create the `embeddings` table only if it does not already exist.

#### Scenario: Fresh database

- **WHEN** `cargo run -p migration -- fresh` is run against a clean database
- **THEN** the `vector` extension is created
- **AND** the `embeddings` table is created
- **AND** the migration completes without error

#### Scenario: Existing database

- **WHEN** the migration runs against a database that has prior migrations
- **THEN** the new migration applies on top of them
- **AND** the existing tables are not dropped or modified

### Requirement: Similarity search uses the ivfflat index

The `embeddings` table SHALL have an `ivfflat` index on the `embedding` column with cosine-distance ops (`vector_cosine_ops`) and `lists = 100`. The system SHALL use the index for similarity queries.

#### Scenario: Query plan uses the index

- **WHEN** an `EXPLAIN` is run on a `SELECT ... ORDER BY embedding <=> $1::vector LIMIT 5`
- **THEN** the plan includes an `Index Scan` on `idx_embeddings_vector`

### Requirement: Unique index on (post_id, language_code)

The `embeddings` table SHALL have a unique index on `(post_id, language_code)` so that `find_translation` is a single index lookup and so that `store_translation` upserts via `ON CONFLICT`.

#### Scenario: ON CONFLICT upsert

- **WHEN** `store_translation` is called twice with the same `(post_id, language_code)`
- **THEN** the second call updates the existing row instead of raising a unique-constraint violation

### Requirement: Three-tier translation lookup still works

The system SHALL continue to support the DB → pgvector → OpenAI lookup pattern: first check the `post_translations` table for an exact row, then search pgvector for a similarity match above the reuse threshold, and only call OpenAI when neither is found.

#### Scenario: DB hit short-circuits

- **WHEN** the `post_translations` table already has a row for `(post_id, language_code)`
- **THEN** the handler returns that translation without consulting pgvector or OpenAI

#### Scenario: pgvector hit reuses translation

- **WHEN** the DB lookup misses but pgvector returns a row with `score ≥ 0.95`
- **THEN** the handler reuses the cached translation and skips the OpenAI call

#### Scenario: Cold path calls OpenAI

- **WHEN** neither the DB nor pgvector returns a usable match
- **THEN** the handler calls OpenAI to produce a fresh translation
- **AND** the new translation is stored via `store_translation`

