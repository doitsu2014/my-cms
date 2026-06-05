## Why

The my-cms backend currently depends on two external services that Supabase replaces: (1) **Qdrant** for similarity search on translated posts, which is operated as a separate HTTP/gRPC service with its own lifecycle, schema, and credential management, and (2) **Keycloak** for user authentication, which forces the React admin to depend on a third-party JS SDK and the Rust API to depend on `axum-keycloak-auth`. Both add operational surface area and slow down the local dev loop. Migrating to `pgvector` (running on the same Supabase PostgreSQL instance) and Supabase GoTrue (the same instance that already runs GoTrue in compose) consolidates the stack.

## What Changes

- **Replace Qdrant with pgvector for translation similarity search.** Add a `vector(1536)` `embeddings` table with an `ivfflat` cosine-distance index and a unique `(post_id, language_code)` lookup index. Add a new `VectorStore` that talks to PostgreSQL via SeaORM's raw SQL interface, calling OpenAI for embedding generation exactly as the Qdrant version does. The `PostTranslateHandler` continues to expose the same `store_translation`, `search_similar_translations`, and `find_translation` method signatures — only the implementation behind the trait changes.
- **Replace Keycloak with Supabase GoTrue for authentication.** Add a custom `SupabaseAuthLayer` tower middleware that validates Supabase GoTrue JWTs (HS256 with `JWT_SECRET`; falls back to RS256 via the JWKS endpoint) and inserts a `SupabaseToken { claims }` extension that handlers can read. Replace all `KeycloakToken<String>` extractions with `SupabaseToken`. Replace `keycloak-js` in the frontend with `@supabase/supabase-js`.
- **Remove the `qdrant-client` and `axum-keycloak-auth` Rust dependencies** and the `keycloak-js` frontend dependency.
- Update `.env` (backend) and `.env.example` (frontend) to drop Keycloak and Qdrant variables and add `SUPABASE_URL`, `SUPABASE_JWT_SECRET`, `PUBLIC_SUPABASE_URL`, `PUBLIC_SUPABASE_ANON_KEY`.

## Capabilities

### New Capabilities

- `pgvector-vector-search`: similarity search for translated posts backed by PostgreSQL pgvector, with the same three-tier lookup (DB → pgvector → OpenAI) the Qdrant-based system used.
- `supabase-auth`: user authentication against Supabase GoTrue, with a Rust tower middleware that validates JWTs and a React auth context that uses `@supabase/supabase-js` instead of `keycloak-js`.

### Modified Capabilities

<!-- None at the change level. The two new capabilities are introduced here; the local dev environment introduced by `unified-docker-compose-with-supabase` already exists and is not modified by this change. -->

## Impact

- **New Rust files**: `services/migration/src/m20260531_000001_pgvector.rs`, `services/application_core/src/commands/ai/vector_store_pg.rs`, `services/src/common/supabase_auth.rs`, `services/src/common/supabase_token.rs`.
- **Modified Rust files**: `services/migration/src/lib.rs`, `services/application_core/src/commands/ai/mod.rs`, `services/application_core/src/commands/ai/translate/translate_handler.rs`, `services/src/api/post/translate/translate_handler.rs`, `services/src/bin/my-cms-api.rs`, `services/src/common/mod.rs`, all `services/src/api/**/.../..._handler.rs` files that extract `KeycloakToken<String>` (post create/modify/delete/translate/job, category create/modify/delete, tag delete, media create/list/read metadata/delete, administrator migration, delete).
- **Deleted Rust files**: `services/src/common/keycloak_extension.rs` (replaced by `supabase_token.rs`); `services/application_core/src/commands/ai/vector_store.rs` (eventually, after the new module is fully exercised).
- **New frontend files**: `frontend/src/auth/supabase.ts`.
- **Modified frontend files**: `frontend/src/auth/AuthContext.tsx`, `frontend/src/infrastructure/utilities.auth.ts`, `frontend/src/infrastructure/graphQL/graphql-client.ts`, `frontend/src/config/runtime-config.ts`, `frontend/src/env.d.ts`, `frontend/.env.example`.
- **Deleted frontend files**: `frontend/src/auth/keycloak.ts`.
- **Env vars removed**: `KEYCLOAK_ISSUER`, `KEYCLOAK_REALM`, `QDRANT_URL`, `PUBLIC_KEYCLOAK_URL`, `PUBLIC_KEYCLOAK_REALM`, `PUBLIC_KEYCLOAK_CLIENT_ID`, `PUBLIC_KEYCLOAK_SCOPE`.
- **Env vars added**: `SUPABASE_URL`, `SUPABASE_JWT_SECRET`, `AUTHORIZATION_AUDIENCE=authenticated`, `PUBLIC_SUPABASE_URL`, `PUBLIC_SUPABASE_ANON_KEY`.
- **Cargo dependencies removed**: `qdrant-client`, `axum-keycloak-auth`, `oauth2`.
- **npm dependencies**: `keycloak-js` removed, `@supabase/supabase-js` added.
- **Database**: new `embeddings` table with `vector(1536)` column, `ivfflat` cosine index, unique `(post_id, language_code)` index.
