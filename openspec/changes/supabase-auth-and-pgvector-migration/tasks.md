## 1. Part A — pgvector migration

### 1.1 Database migration

- [x] 1.1.1 Create `services/migration/src/m20260531_000001_pgvector.rs` that enables the `vector` extension and creates the `embeddings` table with `vector(1536)` column, the `ivfflat (embedding vector_cosine_ops) WITH (lists = 100)` index, and the unique `(post_id, language_code)` index (Task A1)
- [x] 1.1.2 Register the new migration in `services/migration/src/lib.rs` and append it to `Migrator::migrations()` (Task A1)
- [x] 1.1.3 Run `cargo run -p migration -- fresh` against a clean Supabase DB and confirm "Migration successfully applied" (Task A1)

### 1.2 New pgvector-based VectorStore

- [x] 1.2.1 Create `services/application_core/src/commands/ai/vector_store_pg.rs` exposing `VectorStore::new`, `initialize`, `store_translation`, `search_similar_translations`, `find_translation` with the same signatures as the Qdrant version (Task A2)
- [x] 1.2.2 Implement `format_embedding_for_pg` (build the `[v1,v2,…]` string) and `create_content_preview` (truncate to 2000 chars at a sentence boundary) (Task A2)
- [x] 1.2.3 Implement `search_similar_translations` using `1.0 - (embedding <=> $1::vector)` and `ORDER BY embedding <=> $1::vector LIMIT N` (Task A2)
- [x] 1.2.4 Implement `store_translation` with `ON CONFLICT (post_id, language_code) DO UPDATE` (Task A2)
- [x] 1.2.5 Add `pub mod vector_store_pg;` to `services/application_core/src/commands/ai/mod.rs` (Task A2)

### 1.3 Wire the new VectorStore into the translate handler

- [x] 1.3.1 Update the import in `services/application_core/src/commands/ai/translate/translate_handler.rs` from `vector_store` to `vector_store_pg` (Task A3)
- [x] 1.3.2 Update `services/src/api/post/translate/translate_handler.rs` `initialize_vector_store` to take `Arc<DatabaseConnection>`, construct `VectorStore::new(db, key).await.ok()?`, and call `vs.initialize()` (Task A3)
- [x] 1.3.3 Pass `state.conn.clone()` (or equivalent) to `initialize_vector_store` at the call site (Task A3)
- [x] 1.3.4 Run `cargo check` and confirm no compile errors related to the vector store (Task A3)

### 1.4 Clean up Qdrant dependencies

- [x] 1.4.1 Remove the `qdrant-client = "1.11"` line from `services/application_core/Cargo.toml` (Task A4)
- [x] 1.4.2 Comment out or remove the `QDRANT_URL=...` line from `services/.env` (Task A4)
- [x] 1.4.3 Run `cargo build` and confirm the project compiles with no `qdrant-client` references (Task A4)

### 1.5 (Optional) Remove old Qdrant module

- [x] 1.5.1 Remove `pub mod vector_store;` from `services/application_core/src/commands/ai/mod.rs` (Task A5)
- [x] 1.5.2 Delete `services/application_core/src/commands/ai/vector_store.rs` (Task A5)

## 2. Part B — Supabase GoTrue migration

### 2.1 Backend JWT middleware

- [x] 2.1.1 Create `services/src/common/supabase_auth.rs` with `SupabaseAuthConfig`, `SupabaseClaims`, `SupabaseToken`, `SupabaseAuthLayer`, and `SupabaseAuthMiddleware` (Task B1)
- [x] 2.1.2 Implement `validate_supabase_token` that tries HS256 with `JWT_SECRET` first, then RS256 via the JWKS endpoint at `{SUPABASE_URL}/auth/v1/.well-known/jwks.json` (Task B1)
- [x] 2.1.3 Add `pub mod supabase_auth;` (and `pub mod supabase_token;` if split) to `services/src/common/mod.rs` (Task B1)

### 2.2 Replace KeycloakAuthLayer in the router

- [x] 2.2.1 Update `services/.env`: remove Keycloak variables, add `SUPABASE_URL=http://localhost:8000`, `SUPABASE_JWT_SECRET=...`, `AUTHORIZATION_AUDIENCE=authenticated` (Task B2)
- [x] 2.2.2 Update `services/src/bin/my-cms-api.rs`: remove `axum-keycloak-auth` imports, add `use crate::common::supabase_auth::*`, replace `construct_keycloak_auth_instance` with `construct_supabase_auth_layer` (Task B2)
- [x] 2.2.3 Replace `.layer(KeycloakAuthLayer::<String>::builder()...)` with `.layer(construct_supabase_auth_layer())` in both `protected_router()` and `protected_administrator_router()` (Task B2)
- [x] 2.2.4 Update each handler that extracts `KeycloakToken<String>` to extract `SupabaseToken` and read email via `token.email().unwrap_or_default().to_string()` (Task B2 — affected files: `services/src/api/post/{create,modify,delete,translate/translate,translate/job}_handler.rs`, `services/src/api/category/{create,modify,delete}_handler.rs`, `services/src/api/tag/delete/delete_handler.rs`, `services/src/api/media/{create,list,read/metadata,delete}_handler.rs`, `services/src/api/administrator/migration/migration_handler.rs`, `services/src/api/delete/delete_handler.rs`)
- [x] 2.2.5 Run `cargo check` and confirm the build succeeds with no `KeycloakToken` or `axum_keycloak_auth` references (Task B2)

### 2.3 Remove Keycloak Rust dependencies

- [x] 2.3.1 Remove `axum-keycloak-auth = "0.8.3"` and `oauth2 = "5.0.0"` from `services/Cargo.toml` (Task B3)
- [x] 2.3.2 Delete `services/src/common/keycloak_extension.rs` and remove `pub mod keycloak_extension;` from `services/src/common/mod.rs` (Task B3)
- [x] 2.3.3 Run `cargo build` and confirm a clean build with no warnings (Task B3)

### 2.4 Frontend Supabase auth

- [x] 2.4.1 Add `@supabase/supabase-js` to the frontend dependencies and remove `keycloak-js` (Task B4)
- [x] 2.4.2 Create `frontend/src/auth/supabase.ts` exporting `getSupabaseClient()` configured with `autoRefreshToken`, `persistSession`, `detectSessionInUrl` (Task B4)
- [x] 2.4.3 Rewrite `frontend/src/auth/AuthContext.tsx` to subscribe to `supabase.auth.onAuthStateChange` and expose `{ user, session, isLoading, getAccessToken, signOut }` (Task B4)
- [x] 2.4.4 Update `frontend/src/infrastructure/utilities.auth.ts` to call `getSupabaseClient().auth.getSession()` and return `Authorization: Bearer <access_token>` (Task B4)
- [x] 2.4.5 Update the `setContext` link in `frontend/src/infrastructure/graphQL/graphql-client.ts` to inject the Supabase access token (Task B4)
- [x] 2.4.6 Replace the `keycloak*` fields in `frontend/src/config/runtime-config.ts` with `supabaseUrl` and `supabaseAnonKey` (Task B4)
- [x] 2.4.7 Replace the `PUBLIC_KEYCLOAK_*` declarations in `frontend/src/env.d.ts` with `PUBLIC_SUPABASE_URL` and `PUBLIC_SUPABASE_ANON_KEY` (Task B4)
- [x] 2.4.8 Replace the Keycloak variables in `frontend/.env.example` with `PUBLIC_SUPABASE_URL` and `PUBLIC_SUPABASE_ANON_KEY` (Task B4)
- [x] 2.4.9 Delete `frontend/src/auth/keycloak.ts` (Task B4)
- [x] 2.4.10 Run `pnpm run build` and confirm the frontend builds successfully (Task B4)

## 3. Verify

- [x] 3.1 `cargo run -p migration -- fresh` applies the new pgvector migration cleanly on a fresh database
- [x] 3.2 The pgvector migration applies cleanly on top of existing migrations
- [x] 3.3 `VectorStore::store_translation` writes a row that `search_similar_translations` and `find_translation` can both read back
- [x] 3.4 The three-tier translation lookup (DB → pgvector → OpenAI) returns the cached translation when the similarity score is ≥ 0.95
- [x] 3.5 Requests without an `Authorization` header return 401
- [x] 3.6 Requests with an invalid signature return 401
- [x] 3.7 Requests with a valid Supabase JWT reach the handler and populate the `SupabaseToken` extension
- [x] 3.8 `token.email()` returns the caller's email for audit columns
- [x] 3.9 The React sign-in → use-app → token-refresh → sign-out flow works end-to-end
- [x] 3.10 GraphQL requests include `Authorization: Bearer <supabase_access_token>`
- [x] 3.11 `cargo test` passes for all existing tests
- [x] 3.12 `cargo build` succeeds with no warnings
