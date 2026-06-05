## Context

The my-cms backend (`services/`) and React admin panel (`frontend/`) currently integrate with two services that are about to be replaced by Supabase. The local dev stack delivered by `unified-docker-compose-with-supabase` already runs the Supabase components, so this change is a pure backend/frontend migration that can be developed against a running Supabase instance.

**Part A — Vector search:** today, when a post is translated, the `PostTranslateHandler` calls `VectorStore::store_translation(...)` to upsert a 1536-dimensional `text-embedding-3-small` embedding plus metadata (post id, language code, translation id, title, content preview) into a Qdrant collection. The handler then calls `search_similar_translations(...)` to find existing translations to reuse, gated by a 0.95 cosine-similarity threshold. There is also a fast-path `find_translation(post_id, language_code)` for exact lookups. Qdrant is reached via gRPC at `QDRANT_URL` and its collection is initialised on API startup. The Qdrant dependency is the only thing in `application_core/Cargo.toml` that needs HTTP/gRPC clients for storage; removing it simplifies the binary.

**Part B — Auth:** every protected route in the Axum router goes through a `KeycloakAuthLayer` that validates the bearer token against `https://my-ids-admin.ducth.dev` (realm `my-blogs`) and exposes a `KeycloakToken<String>` extension. Handlers that need the caller's email (for audit columns) extract it via `token.extract_email().email`. The React admin imports `keycloak-js`, initialises it on mount, and exchanges tokens with Keycloak's authorization-code flow. The auth flow is duplicated: Keycloak for production and external users, plus the eventual need to integrate with Supabase.

## Goals / Non-Goals

**Goals:**

- Replace the Qdrant client with a PostgreSQL pgvector implementation that exposes the same `VectorStore` method signatures, so the `PostTranslateHandler` is not rewritten.
- Replace the Keycloak tower layer with a Supabase GoTrue JWT-validation layer (HS256 with `JWT_SECRET`, RS256 fallback via JWKS) that inserts a `SupabaseToken` extension in the same place `KeycloakToken` was.
- Move the React auth flow from `keycloak-js` to `@supabase/supabase-js`, keeping `AuthContext`'s public API (`user`, `session`, `isLoading`, `getAccessToken`, `signOut`) so consumers do not change.
- Remove `qdrant-client`, `axum-keycloak-auth`, `oauth2`, and `keycloak-js` from the dependency graph.

**Non-Goals:**

- Migrating the production deployment (this is a code change; the dev stack already runs Supabase).
- Implementing role-based access control inside the new middleware (e.g. distinguishing `my-headless-cms-writer` from `my-headless-cms-administrator`). For now, both protected router groups use the same `SupabaseAuthLayer`; role checks can be added per-handler or in a second layer variant later.
- Changing the three-tier translation lookup (DB → pgvector → OpenAI). The new `VectorStore` is a drop-in.
- Replacing SeaORM's entity generation or migration tooling.

## Decisions

### Decision 1: New `vector_store_pg` module keeps the existing `VectorStore` trait surface

`application_core/src/commands/ai/vector_store_pg.rs` exposes the same three methods as the Qdrant version (`store_translation`, `search_similar_translations`, `find_translation`) and the same `TranslationMetadata` / `SimilarityResult` types. The old `vector_store.rs` is kept (compiled) until the new module is fully exercised, then removed in Task A5.

**Rationale:** keeps the blast radius of the change confined to a single new module + one import line. The `PostTranslateHandler` is a one-line import change. Alternatives considered: rewriting the handler to call SQL directly — rejected, that would touch every code path that touches translation; keeping the Qdrant code path as a feature-flagged option — rejected, it would leave two code paths in the codebase indefinitely.

### Decision 2: `embeddings` table shape mirrors the Qdrant payload

Columns: `id uuid PK`, `post_id uuid FK→posts.id ON DELETE CASCADE`, `language_code string(50)`, `translation_id uuid FK→post_translations.id ON DELETE SET NULL`, `embedding vector(1536) NOT NULL`, `title string(512)`, `content_preview text`, `created_at`, `updated_at`. Two indexes: `ivfflat (embedding vector_cosine_ops) WITH (lists = 100)` for similarity search, and a unique index on `(post_id, language_code)` for the exact-lookup fast path.

**Rationale:** `vector(1536)` matches `text-embedding-3-small`. The `ivfflat` index is the right tool for cosine similarity on <= ~1M rows; `lists = 100` is the default recommended starting point. The unique `(post_id, language_code)` index lets `find_translation` be a single index lookup instead of a sequential scan. Alternatives considered: HNSW index — rejected for now, it requires `pgvector ≥ 0.5` and the existing 0.4 version; storing the embedding in `post_translations` — rejected, it conflates the translation row with its search index and breaks the three-tier cache.

### Decision 3: `format_embedding_for_pg` builds a `[v1,v2,…]` string and runs via `query_all`

Embeddings are produced by the OpenAI client, converted to a Postgres `vector` literal string (`[0.1,0.2,…]`), and passed as a parameter to raw SQL via SeaORM's `Statement::from_string`. The handler uses `query_all` and reads columns by name (`row.try_get::<f64>("", "similarity")` etc.).

**Rationale:** the handler already has a `DatabaseConnection`; reusing the same connection for the search keeps the data path inside the same transaction boundary that the rest of the handler uses. Alternatives considered: a dedicated `sea-orm` entity for the `embeddings` table — rejected, the entire point of `vector_store_pg` is to keep the SQL close to the storage format and avoid hand-rolled entity plumbing.

### Decision 4: Custom `SupabaseAuthLayer` tower middleware (not a third-party crate)

`SupabaseAuthLayer` validates the JWT in two passes: first HS256 with `JWT_SECRET` and `expected_audience`, then (on failure) RS256 by fetching the JWKS document at `{SUPABASE_URL}/auth/v1/.well-known/jwks.json`, picking the key by `kid`, and validating against that. On success, it inserts a `SupabaseToken { claims: SupabaseClaims }` extension. On failure (missing or invalid header), it returns HTTP 401 with `{"error": "..."}`.

**Rationale:** GoTrue supports both HS256 (developer-controlled symmetric secret) and RS256 (asymmetric, keys rotated by GoTrue). HS256 is simpler and works for local dev; RS256 is the right choice when keys need to rotate without redeploying. Alternatives considered: an off-the-shelf `axum-go-true-auth` crate — rejected, none of the candidates matched the project's claim layout (`app_metadata`, `user_metadata`); only validating HS256 — rejected, production keys will rotate and we need RS256 as a fallback.

### Decision 5: `SupabaseToken` and `SupabaseClaims` keep the public methods the handlers need

`SupabaseToken::user_id()`, `SupabaseToken::email()`, `SupabaseToken::role()` mirror the small subset of `KeycloakToken` methods the handlers actually use. `SupabaseClaims` carries `sub`, `email?`, `aud`, `role`, `exp`, `iat`, `app_metadata?`, `user_metadata?`. Handler call sites change from `token.extract_email().email` to `token.email().unwrap_or_default().to_string()`.

**Rationale:** minimises handler churn while letting us drop the `keycloak_extension` helper. Alternatives considered: a single trait `ExtractSupabaseToken` — kept optional; the migration updates all call sites to use the new struct's methods directly.

### Decision 6: Frontend `AuthContext` keeps the existing context shape

The new `AuthContext` exposes the same `user`, `session`, `isLoading`, `getAccessToken`, `signOut`. Internally, it calls `getSupabaseClient().auth.getSession()` on mount, subscribes via `supabase.auth.onAuthStateChange`, and stores the session in React state. The GraphQL auth link is rebuilt to call `getSupabaseClient().auth.getSession()` and inject the `access_token` as a Bearer header.

**Rationale:** consumers of `useAuth()` (every page, the GraphQL client, the auth utility helpers) continue to work without changes. Alternatives considered: switching to React Query for session state — rejected, the existing component lifecycle handles it well and a CQRS-style split is out of scope.

### Decision 7: OpenSpec keeps both sub-plans in one change with two capabilities

The two sub-plans (pgvector and Supabase auth) are tracked as **two capabilities under one change** (`pgvector-vector-search` and `supabase-auth`). They can be implemented by separate subagents in parallel, but they are not independent of this change: the proposal, design, and archive lifecycle are shared.

**Rationale:** the legacy plan was explicitly framed as "two independent sub-plans that can be executed in parallel. Use superpowers:subagent-driven-development." OpenSpec expresses that with one change and two capabilities, and Superpowers dispatches subagents per capability group. Alternatives considered: two separate changes — rejected, the archive date and "why" are the same; merging them into one capability — rejected, the API surface (vector store vs auth middleware) is distinct.

## Risks / Trade-offs

- **[No data migration path from Qdrant to pgvector]** → For local dev this is fine (no existing embeddings to preserve). For a future production migration, the team will need a one-shot script that streams from Qdrant to the new `embeddings` table; flagged as a follow-up.
- **[HS256 with shared `JWT_SECRET` is symmetric]** → anyone with the secret can mint tokens. Mitigation: keep the secret out of version control (`.env` only); switch to RS256 for production once the project is on Supabase Cloud.
- **[pgvector `ivfflat` index is approximate]** → recall is < 100% for the nearest-neighbour search. Mitigation: the three-tier cache falls back to OpenAI when no good match is found, so a missed neighbour is a cost, not a correctness issue.
- **[Role-based access control is not enforced in the middleware]** → all authenticated users can call any handler for now. Mitigation: add per-handler role checks (e.g. on `claims.app_metadata.roles`) or a second layer variant before the next release.
- **[Removing the old `vector_store.rs` and `keycloak_extension.rs` is a separate task]** → leaving them in the tree for one extra commit window keeps the rollback path short.
- **[JWKS endpoint requires network reachability to `SUPABASE_URL`]** → in CI or air-gapped environments the RS256 fallback will fail. Mitigation: HS256 is the primary path; the fallback only runs if HS256 validation fails.

## Migration Plan

- Backend: tasks A1 → A3 land in order (migration → new VectorStore → wire it into the handler). Tasks B1 → B2 land in order (middleware → router + handler migration). A4 (Cargo cleanup) and B3 (Keycloak dependency removal) are last. A5 (old `vector_store.rs` removal) is optional and gated on tests passing.
- Frontend: tasks B4 runs in parallel with B2-B3 once the backend middleware is in place.
- Local dev: bring up the stack with `docker compose up -d` (from the prior change), set `OPENAI_API_KEY` for the embedding calls, and run `cargo run -p migration -- fresh` to apply the new pgvector migration.
- Rollback: revert the change's PR; the `qdrant-client` and `axum-keycloak-auth` dependencies come back unchanged.

## Open Questions

- Where should role checks live — in a new `SupabaseRoleLayer` variant, or in each handler? (Deferred to a follow-up; this change uses the same layer for both router groups.)
- Do we need to surface `app_metadata.roles` in the GraphQL auth context for the React admin's "is-admin" UI? (Deferred; React reads `session.user.app_metadata` for now.)
