# Authentication + Vector Search Migration Plan

> **For agentic workers:** These are two independent sub-plans that can be executed in parallel. Use superpowers:subagent-driven-development.
> Steps use checkbox (`- [ ]`) syntax for tracking.

**Goal:** Replace Qdrant with pgvector for vector search, and replace Keycloak with Supabase GoTrue for authentication.

**Architecture:** The VectorStore struct switches from `Qdrant` gRPC client to native PostgreSQL pgvector queries via SeaORM's raw SQL interface. Auth switches from `axum-keycloak-auth` tower layer to a custom Supabase GoTrue JWT validation middleware using `jsonwebtoken` (already a dependency). Frontend switches from `keycloak-js` to `@supabase/supabase-js`.

**Tech Stack:** Rust (axum, tower, sea-orm, jsonwebtoken, pgvector 0.4), TypeScript (React, @supabase/supabase-js)

**Files to Create:**
- `services/migration/src/m20260531_000001_pgvector.rs` — migration to enable pgvector + create embeddings table
- `services/application_core/src/commands/ai/vector_store_pg.rs` — new VectorStore using PostgreSQL pgvector
- `services/src/common/supabase_auth.rs` — custom tower middleware for Supabase GoTrue JWT validation
- `services/src/common/supabase_token.rs` — token extraction extension, equivalent to keycloak_extension

**Files to Modify:**
- `services/migration/src/lib.rs` — register new migration
- `services/application_core/src/commands/ai/translate/translate_handler.rs` — use new VectorStore
- `services/application_core/src/commands/ai/mod.rs` — update module declarations
- `services/application_core/src/commands/ai/vector_store.rs` — remove or gate behind feature flag
- `services/application_core/Cargo.toml` — remove qdrant-client, add pgvector dep if needed
- `services/src/api/post/translate/translate_handler.rs` — initialize new VectorStore
- `services/src/bin/my-cms-api.rs` — replace KeycloakAuthLayer with Supabase auth middleware, update .env vars
- `services/src/common/mod.rs` — declare new module
- `services/Cargo.toml` — replace axum-keycloak-auth, add jsonwebtoken usage
- `services/.env` — update auth vars, remove QDRANT_URL
- `frontend/src/auth/keycloak.ts` → replaced by `frontend/src/auth/supabase.ts`
- `frontend/src/auth/AuthContext.tsx` — switch to Supabase auth
- `frontend/src/infrastructure/utilities.auth.ts` — update token header builder
- `frontend/src/infrastructure/graphQL/graphql-client.ts` — update auth link
- `frontend/src/config/runtime-config.ts` — update config interface
- `frontend/src/env.d.ts` — update type declarations
- `frontend/.env.example` — update docs
- `services/application_core/src/commands/ai/translate/translate_response.rs` — update if needed

**Files to Delete (eventually):**
- `services/src/common/keycloak_extension.rs` — replaced by supabase_token.rs

---

## Part A: pgvector Migration (Qdrant → PostgreSQL)

### Task A1: Create pgvector Database Migration

**Files:**
- Create: `services/migration/src/m20260531_000001_pgvector.rs`
- Modify: `services/migration/src/lib.rs`

- [ ] **Step 1: Create migration file**

```rust
// services/migration/src/m20260531_000001_pgvector.rs

use sea_orm_migration::prelude::*;

#[derive(DeriveMigrationName)]
pub struct Migration;

#[async_trait::async_trait]
impl MigrationTrait for Migration {
    async fn up(&self, manager: &SchemaManager) -> Result<(), DbErr> {
        manager
            .get_connection()
            .execute_unprepared("CREATE EXTENSION IF NOT EXISTS vector")
            .await?;

        manager
            .create_table(
                Table::create()
                    .table(Embeddings::Table)
                    .if_not_exists()
                    .col(
                        ColumnDef::new(Embeddings::Id)
                            .uuid()
                            .not_null()
                            .primary_key(),
                    )
                    .col(ColumnDef::new(Embeddings::PostId).uuid().not_null())
                    .col(
                        ColumnDef::new(Embeddings::LanguageCode)
                            .string_len(50)
                            .not_null(),
                    )
                    .col(ColumnDef::new(Embeddings::TranslationId).uuid())
                    .col(
                        ColumnDef::new(Embeddings::Embedding)
                            .custom("vector(1536)")
                            .not_null(),
                    )
                    .col(ColumnDef::new(Embeddings::Title).string_len(512).not_null())
                    .col(ColumnDef::new(Embeddings::ContentPreview).text().not_null())
                    .col(
                        ColumnDef::new(Embeddings::CreatedAt)
                            .timestamp_with_time_zone()
                            .default(Expr::current_timestamp())
                            .not_null(),
                    )
                    .col(
                        ColumnDef::new(Embeddings::UpdatedAt)
                            .timestamp_with_time_zone()
                            .default(Expr::current_timestamp())
                            .not_null(),
                    )
                    .foreign_key(
                        ForeignKey::create()
                            .from(Embeddings::Table, Embeddings::PostId)
                            .to(Posts::Table, Posts::Id)
                            .on_delete(ForeignKeyAction::Cascade),
                    )
                    .foreign_key(
                        ForeignKey::create()
                            .from(Embeddings::Table, Embeddings::TranslationId)
                            .to(PostTranslations::Table, PostTranslations::Id)
                            .on_delete(ForeignKeyAction::SetNull),
                    )
                    .to_owned(),
            )
            .await?;

        // Index for vector similarity search (cosine distance)
        manager
            .get_connection()
            .execute_unprepared(
                "CREATE INDEX idx_embeddings_vector ON embeddings USING ivfflat (embedding vector_cosine_ops) WITH (lists = 100)",
            )
            .await?;

        // Index for exact lookup by post_id + language_code (replaces Qdrant scroll)
        manager
            .create_index(
                Index::create()
                    .name("idx_embeddings_post_lang")
                    .table(Embeddings::Table)
                    .col(Embeddings::PostId)
                    .col(Embeddings::LanguageCode)
                    .unique()
                    .to_owned(),
            )
            .await?;

        Ok(())
    }

    async fn down(&self, manager: &SchemaManager) -> Result<(), DbErr> {
        manager
            .drop_table(Table::drop().table(Embeddings::Table).to_owned())
            .await?;
        manager
            .get_connection()
            .execute_unprepared("DROP EXTENSION IF EXISTS vector")
            .await?;
        Ok(())
    }
}

#[derive(DeriveIden)]
enum Embeddings {
    Table,
    Id,
    PostId,
    LanguageCode,
    TranslationId,
    Embedding,
    Title,
    ContentPreview,
    CreatedAt,
    UpdatedAt,
}

#[derive(DeriveIden)]
enum Posts {
    Table,
    Id,
}

#[derive(DeriveIden)]
enum PostTranslations {
    Table,
    Id,
}
```

- [ ] **Step 2: Register migration**

```rust
// In services/migration/src/lib.rs, add to the existing vec![]:

mod m20240409_151952_release_100;
mod m20250330_151455_release_110;
mod m20260126_040610_release_300;
mod m20260531_000001_pgvector; // ADD THIS

// In the Migrator impl:
impl MigratorTrait for Migrator {
    fn migrations() -> Vec<Box<dyn MigrationTrait>> {
        vec![
            Box::new(m20240409_151952_release_100::Migration),
            Box::new(m20250330_151455_release_110::Migration),
            Box::new(m20260126_040610_release_300::Migration),
            Box::new(m20260531_000001_pgvector::Migration), // ADD THIS
        ]
    }
}
```

- [ ] **Step 3: Test migration applies cleanly**

```bash
cd services && cargo run -p migration -- fresh
```
Expected: "Migration successfully applied" or success status. Tables are created.

- [ ] **Step 4: Commit**

```bash
git add services/migration/src/m20260531_000001_pgvector.rs services/migration/src/lib.rs
git commit -m "feat: add pgvector embeddings table migration"
```

---

### Task A2: Create New VectorStore with pgvector

**Files:**
- Create: `services/application_core/src/commands/ai/vector_store_pg.rs`
- Modify: `services/application_core/src/commands/ai/mod.rs`

- [ ] **Step 1: Write the new VectorStore**

```rust
// services/application_core/src/commands/ai/vector_store_pg.rs

use async_openai::{types::CreateEmbeddingRequestArgs, Client};
use chrono::{DateTime, Utc};
use sea_orm::{ConnectionTrait, DbBackend, Statement};
use serde::{Deserialize, Serialize};
use std::sync::Arc;
use uuid::Uuid;

use crate::common::app_error::AppError;

pub(crate) const TRANSLATION_COLLECTION: &str = "translations";
pub(crate) const EMBEDDING_MODEL: &str = "text-embedding-3-small";
pub(crate) const EMBEDDING_DIMENSION: u32 = 1536;
pub(crate) const SIMILARITY_REUSE_THRESHOLD: f32 = 0.95;
pub(crate) const MAX_SEARCH_TEXT_LENGTH: usize = 8000;
pub(crate) const CONTENT_PREVIEW_LENGTH: usize = 2000;

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct TranslationMetadata {
    pub post_id: String,
    pub language_code: String,
    pub translation_id: String,
    pub title: String,
    pub content_preview: String,
}

#[derive(Debug, Clone)]
pub struct SimilarityResult {
    pub score: f32,
    pub metadata: TranslationMetadata,
}

#[derive(Debug)]
pub struct VectorStore {
    db: Arc<dyn ConnectionTrait + Send + Sync>,
    openai_client: Arc<Client>,
}

impl VectorStore {
    pub async fn new(
        db: Arc<dyn ConnectionTrait + Send + Sync>,
        openai_api_key: String,
    ) -> Result<Self, AppError> {
        let config = async_openai::config::OpenAIConfig::new().with_api_key(openai_api_key);
        let openai_client = Client::with_config(config);
        Ok(Self {
            db,
            openai_client: Arc::new(openai_client),
        })
    }

    pub async fn initialize(&self) -> Result<(), AppError> {
        // Verify pgvector extension exists
        self.db
            .execute(Statement::from_string(
                DbBackend::Postgres,
                "SELECT 1 FROM pg_extension WHERE extname = 'vector'".to_string(),
            ))
            .await
            .map_err(|e| AppError::OpenAIError(format!("pgvector extension not found: {}", e)))?;
        Ok(())
    }

    async fn generate_embedding(&self, text: &str) -> Result<Vec<f32>, AppError> {
        let request = CreateEmbeddingRequestArgs::default()
            .model(EMBEDDING_MODEL)
            .input(text)
            .build()
            .map_err(|e| AppError::OpenAIError(format!("Failed to build embedding request: {}", e)))?;

        let response = self
            .openai_client
            .embeddings()
            .create(request)
            .await
            .map_err(|e| AppError::OpenAIError(format!("Failed to generate embedding: {}", e)))?;

        response
            .data
            .first()
            .map(|d| d.embedding.clone())
            .ok_or_else(|| AppError::OpenAIError("No embedding returned".to_string()))
    }

    fn format_embedding_for_pg(embedding: &[f32]) -> String {
        let values: Vec<String> = embedding.iter().map(|v| v.to_string()).collect();
        format!("[{}]", values.join(","))
    }

    fn create_content_preview(content: &str) -> String {
        if content.len() <= CONTENT_PREVIEW_LENGTH {
            return content.to_string();
        }
        let truncated = &content[..CONTENT_PREVIEW_LENGTH];
        truncated
            .rfind(". ")
            .or_else(|| truncated.rfind("\n\n"))
            .map(|pos| truncated[..=pos].to_string())
            .unwrap_or_else(|| format!("{}...", truncated))
    }

    pub async fn store_translation(
        &self,
        post_id: &str,
        language_code: &str,
        translation_id: &str,
        title: &str,
        content: &str,
    ) -> Result<(), AppError> {
        let text_for_embedding =
            format!("{} {}", title, content.chars().take(MAX_SEARCH_TEXT_LENGTH).collect::<String>());
        let embedding = self.generate_embedding(&text_for_embedding).await?;
        let embedding_str = Self::format_embedding_for_pg(&embedding);
        let content_preview = Self::create_content_preview(content);
        let id = Uuid::new_v4();

        self.db
            .execute(Statement::from_string(
                DbBackend::Postgres,
                format!(
                    r#"INSERT INTO embeddings (id, post_id, language_code, translation_id, embedding, title, content_preview, created_at, updated_at)
                    VALUES ('{}'::uuid, '{}'::uuid, '{}', '{}'::uuid, '{}'::vector, '{}', '{}', NOW(), NOW())
                    ON CONFLICT (post_id, language_code) DO UPDATE SET
                        translation_id = EXCLUDED.translation_id,
                        embedding = EXCLUDED.embedding,
                        title = EXCLUDED.title,
                        content_preview = EXCLUDED.content_preview,
                        updated_at = NOW()"#,
                    id,
                    post_id,
                    language_code,
                    translation_id,
                    embedding_str,
                    title.replace('\'', "''"),
                    content_preview.replace('\'', "''"),
                ),
            ))
            .await
            .map_err(|e| AppError::OpenAIError(format!("Failed to store embedding: {}", e)))?;

        Ok(())
    }

    pub async fn search_similar_translations(
        &self,
        search_text: &str,
        limit: u64,
    ) -> Result<Vec<SimilarityResult>, AppError> {
        let embedding = self.generate_embedding(search_text).await?;
        let embedding_str = Self::format_embedding_for_pg(&embedding);

        let query = format!(
            r#"SELECT
                e.post_id,
                e.language_code,
                e.translation_id,
                e.title,
                e.content_preview,
                1.0 - (e.embedding <=> '{}'::vector) AS similarity
            FROM embeddings e
            ORDER BY e.embedding <=> '{}'::vector
            LIMIT {}"#,
            embedding_str, embedding_str, limit
        );

        let results = self
            .db
            .query_all(Statement::from_string(DbBackend::Postgres, query))
            .await
            .map_err(|e| {
                AppError::OpenAIError(format!("Failed to search embeddings: {}", e))
            })?;

        let mut similar = Vec::new();
        for row in results {
            let score: f32 = row
                .try_get::<f64>("", "similarity")
                .map(|v| v as f32)
                .unwrap_or(0.0);
            let post_id: String = row.try_get("", "post_id").unwrap_or_default();
            let language_code: String = row.try_get("", "language_code").unwrap_or_default();
            let translation_id: String = row.try_get("", "translation_id").unwrap_or_default();
            let title: String = row.try_get("", "title").unwrap_or_default();
            let content_preview: String = row.try_get("", "content_preview").unwrap_or_default();

            similar.push(SimilarityResult {
                score,
                metadata: TranslationMetadata {
                    post_id,
                    language_code,
                    translation_id,
                    title,
                    content_preview,
                },
            });
        }

        Ok(similar)
    }

    pub async fn find_translation(
        &self,
        post_id: &str,
        language_code: &str,
    ) -> Result<Option<TranslationMetadata>, AppError> {
        let query = format!(
            r#"SELECT post_id, language_code, translation_id, title, content_preview
            FROM embeddings
            WHERE post_id = '{}'::uuid AND language_code = '{}'"#,
            post_id, language_code
        );

        let results = self
            .db
            .query_all(Statement::from_string(DbBackend::Postgres, query))
            .await
            .map_err(|e| AppError::OpenAIError(format!("Failed to find embedding: {}", e)))?;

        Ok(results.first().map(|row| TranslationMetadata {
            post_id: row.try_get("", "post_id").unwrap_or_default(),
            language_code: row.try_get("", "language_code").unwrap_or_default(),
            translation_id: row.try_get("", "translation_id").unwrap_or_default(),
            title: row.try_get("", "title").unwrap_or_default(),
            content_preview: row.try_get("", "content_preview").unwrap_or_default(),
        }))
    }
}
```

- [ ] **Step 2: Update module declaration**

```rust
// In services/application_core/src/commands/ai/mod.rs

pub mod models;
pub mod translate;
pub mod vector_store;       // keep old for now
pub mod vector_store_pg;    // ADD THIS
```

- [ ] **Step 3: Commit**

```bash
git add services/application_core/src/commands/ai/vector_store_pg.rs services/application_core/src/commands/ai/mod.rs
git commit -m "feat: implement pgvector-based VectorStore"
```

---

### Task A3: Update TranslateHandler to Use New VectorStore

**Files:**
- Modify: `services/application_core/src/commands/ai/translate/translate_handler.rs`
- Modify: `services/src/api/post/translate/translate_handler.rs`

- [ ] **Step 1: Update the handler struct and trait**

In `services/application_core/src/commands/ai/translate/translate_handler.rs`, replace the VectorStore import and struct field:

```rust
// Change import from:
use crate::commands::ai::vector_store::VectorStore;
// To:
use crate::commands::ai::vector_store_pg::VectorStore;

// The struct stays the same pattern:
#[derive(Debug)]
pub struct PostTranslateHandler {
    pub db: Arc<DatabaseConnection>,
    pub vector_store: Option<Arc<VectorStore>>,
}
```

The `VectorStore` API (method signatures) is identical to the old Qdrant version:
- `store_translation(post_id, language_code, translation_id, title, content) -> Result<()>`
- `search_similar_translations(search_text, limit) -> Result<Vec<SimilarityResult>>`
- `find_translation(post_id, language_code) -> Result<Option<TranslationMetadata>>`

So no additional changes are needed in `translate_handler.rs` — the trait methods call the same VectorStore methods.

- [ ] **Step 2: Update API layer initialization**

In `services/src/api/post/translate/translate_handler.rs`, update the `initialize_vector_store` function:

```rust
// Change from:
async fn initialize_vector_store(openai_api_key: &str) -> Option<Arc<VectorStore>> {
    match env::var("QDRANT_URL") {
        Ok(qdrant_url) => {
            match VectorStore::new(&qdrant_url, openai_api_key.to_string()).await {
                Ok(vs) => {
                    if let Err(e) = vs.initialize_collection().await {
                        tracing::warn!("Failed to initialize Qdrant collection: {}", e);
                        None
                    } else {
                        Some(Arc::new(vs))
                    }
                }
                Err(e) => {
                    tracing::warn!("Failed to connect to Qdrant: {}", e);
                    None
                }
            }
        }
        Err(_) => {
            tracing::info!("QDRANT_URL not set, vector storage disabled");
            None
        }
    }
}

// To:
use crate::commands::ai::vector_store_pg::VectorStore;

async fn initialize_vector_store(
    db: Arc<DatabaseConnection>,
    openai_api_key: &str,
) -> Option<Arc<VectorStore>> {
    let vs = VectorStore::new(db, openai_api_key.to_string()).await.ok()?;
    if let Err(e) = vs.initialize().await {
        tracing::warn!("Failed to initialize pgvector: {}", e);
        None
    } else {
        Some(Arc::new(vs))
    }
}
```

And update the call site to pass `state.conn.clone()` instead of just the API key.

- [ ] **Step 3: Verify compilation**

```bash
cd services && cargo check
```
Expected: No compile errors related to vector store.

- [ ] **Step 4: Commit**

```bash
git add services/application_core/src/commands/ai/translate/translate_handler.rs services/src/api/post/translate/translate_handler.rs
git commit -m "refactor: switch translate handler to pgvector VectorStore"
```

---

### Task A4: Clean Up Qdrant Dependency and Config

**Files:**
- Modify: `services/application_core/Cargo.toml`
- Modify: `services/.env`

- [ ] **Step 1: Remove qdrant-client from Cargo.toml**

```toml
# In services/application_core/Cargo.toml, remove line:
# qdrant-client = "1.11"
```

- [ ] **Step 2: Update .env**

Remove or comment out `QDRANT_URL`:
```
# QDRANT_URL=http://localhost:6334  # REMOVED: now using pgvector on PostgreSQL
```

- [ ] **Step 3: Verify full build**

```bash
cd services && cargo build
```
Expected: Compilation succeeds. No reference to qdrant-client.

- [ ] **Step 4: Commit**

```bash
git add services/application_core/Cargo.toml services/.env
git commit -m "chore: remove qdrant-client dependency, vector search uses pgvector"
```

---

### Task A5: (Optional) Remove old vector_store.rs

**Files:**
- Delete or deprecate: `services/application_core/src/commands/ai/vector_store.rs`
- Modify: `services/application_core/src/commands/ai/mod.rs`

Only do this after all tests pass with the new implementation.

- [ ] **Step 1: Remove old module**

```rust
// In services/application_core/src/commands/ai/mod.rs, remove:
// pub mod vector_store;
```

- [ ] **Step 2: Delete the old file**

```bash
git rm services/application_core/src/commands/ai/vector_store.rs
```

- [ ] **Step 3: Commit**

```bash
git commit -m "chore: remove old Qdrant vector_store.rs"
```

---

## Part B: Keycloak → Supabase GoTrue Migration

### Task B1: Create Supabase JWT Validation Middleware

**Files:**
- Create: `services/src/common/supabase_auth.rs`
- Create: `services/src/common/supabase_token.rs`
- Modify: `services/src/common/mod.rs`

- [ ] **Step 1: Create JWT validation service**

```rust
// services/src/common/supabase_auth.rs

use axum::{
    extract::Request,
    http::StatusCode,
    response::IntoResponse,
};
use jsonwebtoken::{decode, decode_header, Algorithm, DecodingKey, Validation};
use serde::Deserialize;
use std::sync::Arc;
use tower::{Layer, Service};

const JWKS_URI_PATH: &str = "/auth/v1/.well-known/jwks.json";

pub struct SupabaseAuthConfig {
    pub supabase_url: String,
    pub jwt_secret: String, // The JWT_SECRET from Supabase settings (HMAC HS256)
    pub expected_audience: String,
}

#[derive(Debug, Deserialize)]
pub struct SupabaseClaims {
    pub sub: String,
    pub email: Option<String>,
    pub aud: String,
    pub role: String,
    pub exp: i64,
    pub iat: i64,
    pub app_metadata: Option<serde_json::Value>,
    pub user_metadata: Option<serde_json::Value>,
}

pub struct SupabaseToken {
    pub claims: SupabaseClaims,
}

impl SupabaseToken {
    pub fn user_id(&self) -> &str {
        &self.claims.sub
    }

    pub fn email(&self) -> Option<&str> {
        self.claims.email.as_deref()
    }

    pub fn role(&self) -> &str {
        &self.claims.role
    }
}

#[derive(Clone)]
pub struct SupabaseAuthLayer {
    config: Arc<SupabaseAuthConfig>,
}

impl SupabaseAuthLayer {
    pub fn new(config: SupabaseAuthConfig) -> Self {
        Self {
            config: Arc::new(config),
        }
    }
}

impl<S> Layer<S> for SupabaseAuthLayer {
    type Service = SupabaseAuthMiddleware<S>;

    fn layer(&self, inner: S) -> Self::Service {
        SupabaseAuthMiddleware {
            inner,
            config: self.config.clone(),
        }
    }
}

#[derive(Clone)]
pub struct SupabaseAuthMiddleware<S> {
    inner: S,
    config: Arc<SupabaseAuthConfig>,
}

impl<S, B> Service<Request<B>> for SupabaseAuthMiddleware<S>
where
    S: Service<Request<B>, Response = axum::response::Response> + Clone + Send + 'static,
    S::Future: Send,
    B: Send + 'static,
{
    type Response = S::Response;
    type Error = S::Error;
    type Future = std::pin::Pin<Box<dyn std::future::Future<Output = Result<Self::Response, Self::Error>> + Send>>;

    fn poll_ready(
        &mut self,
        cx: &mut std::task::Context<'_>,
    ) -> std::task::Poll<Result<(), Self::Error>> {
        self.inner.poll_ready(cx)
    }

    fn call(&mut self, mut req: Request<B>) -> Self::Future {
        let config = self.config.clone();
        let mut inner = self.inner.clone();

        Box::pin(async move {
            let auth_header = req
                .headers()
                .get("Authorization")
                .and_then(|v| v.to_str().ok())
                .and_then(|v| v.strip_prefix("Bearer "));

            let token_str = match auth_header {
                Some(t) => t.to_string(),
                None => {
                    return Ok((
                        StatusCode::UNAUTHORIZED,
                        r#"{"error": "Missing Authorization header"}"#,
                    )
                        .into_response());
                }
            };

            let claims = match validate_supabase_token(&token_str, &config).await {
                Ok(c) => c,
                Err(e) => {
                    return Ok((
                        StatusCode::UNAUTHORIZED,
                        format!(r#"{{"error": "{}"}}"#, e),
                    )
                        .into_response());
                }
            };

            req.extensions_mut().insert(SupabaseToken { claims });

            inner.call(req).await
        })
    }
}

async fn validate_supabase_token(
    token: &str,
    config: &SupabaseAuthConfig,
) -> Result<SupabaseClaims, String> {
    // Try HMAC (HS256) first using JWT_SECRET
    let decoding_key = DecodingKey::from_secret(config.jwt_secret.as_bytes());
    let mut validation = Validation::new(Algorithm::HS256);
    validation.set_audience(&[&config.expected_audience]);

    match decode::<SupabaseClaims>(token, &decoding_key, &validation) {
        Ok(token_data) => return Ok(token_data.claims),
        Err(_) => {
            // If HMAC fails, try RS256 via JWKS endpoint
            let header = decode_header(token).map_err(|e| format!("Invalid token header: {}", e))?;

            if header.alg == Algorithm::HS256 {
                return Err("Invalid JWT signature".to_string());
            }

            // Fetch JWKS to validate RS256 tokens
            let jwks_url = format!("{}{}", config.supabase_url, JWKS_URI_PATH);
            let jwks_response = reqwest::get(&jwks_url)
                .await
                .map_err(|e| format!("Failed to fetch JWKS: {}", e))?;

            let jwks: jsonwebtoken::jwk::JwkSet = jwks_response
                .json()
                .await
                .map_err(|e| format!("Failed to parse JWKS: {}", e))?;

            let kid = header.kid.ok_or("Token missing kid header")?;
            let jwk = jwks
                .find(&kid)
                .ok_or_else(|| format!("Key not found for kid: {}", kid))?;

            let decoding_key = DecodingKey::from_jwk(&jwk)
                .map_err(|e| format!("Failed to create decoding key from JWK: {}", e))?;

            let mut validation = Validation::new(header.alg);
            validation.set_audience(&[&config.expected_audience]);

            let token_data = decode::<SupabaseClaims>(token, &decoding_key, &validation)
                .map_err(|e| format!("JWT validation failed: {}", e))?;

            Ok(token_data.claims)
        }
    }
}
```

- [ ] **Step 2: Update module declarations**

```rust
// In services/src/common/mod.rs, add:
pub mod supabase_auth;
pub mod supabase_token;
```

- [ ] **Step 3: Commit**

```bash
git add services/src/common/supabase_auth.rs services/src/common/mod.rs
git commit -m "feat: add Supabase GoTrue JWT validation middleware"
```

---

### Task B2: Replace KeycloakAuthLayer in Router Setup

**Files:**
- Modify: `services/src/bin/my-cms-api.rs`
- Modify: `services/.env`

- [ ] **Step 1: Update environment variables in .env**

```env
# Replace Keycloak config:
# KEYCLOAK_ISSUER=https://my-ids-admin.ducth.dev
# KEYCLOAK_REALM=my-blogs
# AUTHORIZATION_AUDIENCE=my-cms-headless-api

# With Supabase config:
SUPABASE_URL=http://localhost:8000
SUPABASE_JWT_SECRET=your-super-secret-jwt-token-with-at-least-32-characters
AUTHORIZATION_AUDIENCE=authenticated
```

- [ ] **Step 2: Update router construction in my-cms-api.rs**

Replace the import and middleware construction. In `services/src/bin/my-cms-api.rs`:

```rust
// Remove imports:
// use axum_keycloak_auth::instance::KeycloakAuthInstance;
// use axum_keycloak_auth::layer::KeycloakAuthLayer;
// use axum_keycloak_auth::config::KeycloakConfig;
// use axum_keycloak_auth::mode::PassthroughMode;
// use url::Url;

// Add imports:
use crate::common::supabase_auth::{SupabaseAuthConfig, SupabaseAuthLayer};

// Remove function:
// fn construct_keycloak_auth_instance() -> KeycloakAuthInstance { ... }

// Add config construction:
fn construct_supabase_auth_layer() -> SupabaseAuthLayer {
    let supabase_url = env::var("SUPABASE_URL").expect("SUPABASE_URL must be set");
    let jwt_secret = env::var("SUPABASE_JWT_SECRET").expect("SUPABASE_JWT_SECRET must be set");
    let audience = env::var("AUTHORIZATION_AUDIENCE").unwrap_or("authenticated".to_string());

    SupabaseAuthLayer::new(SupabaseAuthConfig {
        supabase_url,
        jwt_secret,
        expected_audience: audience,
    })
}

// In protected_router() and protected_administrator_router():
// Replace:
//     .layer(
//         KeycloakAuthLayer::<String>::builder()
//             .instance(construct_keycloak_auth_instance())
//             .passthrough_mode(PassthroughMode::Block)
//             .persist_raw_claims(false)
//             .expected_audiences(vec![...])
//             .required_roles(vec![...])
//             .build()
//     )
// With:
//     .layer(construct_supabase_auth_layer())
```

Note: Role-based access control (`my-headless-cms-writer` vs `my-headless-cms-administrator`) moves from the middleware to a check on `token.claims.app_metadata` or a custom claim. You have two options:
1. Store roles in Supabase `app_metadata` and check in the handlers
2. Create a `role` enum in the middleware itself

For now, keep both routers using the same layer and add role checks in individual handlers (or add a second layer variant).

- [ ] **Step 3: Update handlers that extract KeycloakToken**

Replace all instances of:
```rust
Extension(token): Extension<KeycloakToken<String>>,
// and usage:
token.extract_email().email
```

With:
```rust
Extension(token): Extension<SupabaseToken>,
// and usage:
token.email().unwrap_or_default().to_string()
```

Affected files (update imports in each):
- `services/src/api/post/create/create_handler.rs`
- `services/src/api/post/modify/modify_handler.rs`
- `services/src/api/post/delete/delete_handler.rs`
- `services/src/api/post/translate/translate_handler.rs`
- `services/src/api/post/translate/job_handler.rs`
- `services/src/api/category/create/create_handler.rs`
- `services/src/api/category/modify/modify_handler.rs`
- `services/src/api/category/delete/delete_handler.rs`
- `services/src/api/tag/delete/delete_handler.rs`
- `services/src/api/media/create/create_handler.rs`
- `services/src/api/media/list/list_handler.rs`
- `services/src/api/media/read/metadata_handler.rs`
- `services/src/api/media/delete/delete_handler.rs`
- `services/src/api/administrator/migration/migration_handler.rs`
- `services/src/api/delete/delete_handler.rs`

Each handler change follows the same pattern:
```rust
// Old:
use crate::common::keycloak_extension::ExtractKeyCloakToken;
Extension(token): Extension<KeycloakToken<String>>,
let email = token.extract_email().email;

// New:
use crate::common::supabase_token::ExtractSupabaseToken;
Extension(token): Extension<SupabaseToken>,
let email = token.email().unwrap_or_default().to_string();
```

- [ ] **Step 4: Verify compilation**

```bash
cd services && cargo check
```
Expected: Compilation succeeds. No references to axum-keycloak-auth or KeycloakToken.

- [ ] **Step 5: Commit**

```bash
git add services/src/bin/my-cms-api.rs services/.env services/src/api/
git add services/src/common/
git commit -m "refactor: replace Keycloak auth with Supabase GoTrue JWT middleware"
```

---

### Task B3: Remove Keycloak Dependencies

**Files:**
- Modify: `services/Cargo.toml`
- Delete: `services/src/common/keycloak_extension.rs`
- Modify: `services/src/common/mod.rs`

- [ ] **Step 1: Remove from Cargo.toml**

```toml
# Remove lines:
# axum-keycloak-auth = "0.8.3"
# oauth2 = "5.0.0"
```

(`jsonwebtoken` stays — it is now used directly by the new middleware)

- [ ] **Step 2: Remove old extension file**

```bash
git rm services/src/common/keycloak_extension.rs
```

And remove from mod.rs:
```rust
// Remove: pub mod keycloak_extension;
```

- [ ] **Step 3: Verify full build**

```bash
cd services && cargo build
```
Expected: Successful build with no warnings.

- [ ] **Step 4: Commit**

```bash
git add services/Cargo.toml services/src/common/
git commit -m "chore: remove axum-keycloak-auth and keycloak_extension"
```

---

### Task B4: Update Frontend Auth (keycloak-js → @supabase/supabase-js)

**Files:**
- Create: `frontend/src/auth/supabase.ts`
- Modify: `frontend/src/auth/AuthContext.tsx`
- Modify: `frontend/src/infrastructure/utilities.auth.ts`
- Modify: `frontend/src/infrastructure/graphQL/graphql-client.ts`
- Modify: `frontend/src/config/runtime-config.ts`
- Modify: `frontend/src/env.d.ts`
- Modify: `frontend/.env.example`
- Delete: `frontend/src/auth/keycloak.ts`

- [ ] **Step 1: Install Supabase SDK**

```bash
cd frontend && npm install @supabase/supabase-js
```

- [ ] **Step 2: Create supabase client singleton**

```typescript
// frontend/src/auth/supabase.ts

import { createClient } from "@supabase/supabase-js";
import { getRuntimeConfig } from "../config/runtime-config";

let supabaseClient: ReturnType<typeof createClient> | null = null;

export function getSupabaseClient() {
  if (supabaseClient) return supabaseClient;

  const config = getRuntimeConfig();
  supabaseClient = createClient(config.supabaseUrl, config.supabaseAnonKey, {
    auth: {
      autoRefreshToken: true,
      persistSession: true,
      detectSessionInUrl: true,
    },
  });
  return supabaseClient;
}
```

- [ ] **Step 3: Rewrite AuthContext**

```typescript
// frontend/src/auth/AuthContext.tsx

import React, { createContext, useContext, useEffect, useState } from "react";
import type { Session, User } from "@supabase/supabase-js";
import { getSupabaseClient } from "./supabase";

interface AuthContextType {
  user: User | null;
  session: Session | null;
  isLoading: boolean;
  getAccessToken: () => Promise<string | null>;
  signOut: () => Promise<void>;
}

const AuthContext = createContext<AuthContextType | null>(null);

export function AuthProvider({ children }: { children: React.ReactNode }) {
  const [user, setUser] = useState<User | null>(null);
  const [session, setSession] = useState<Session | null>(null);
  const [isLoading, setIsLoading] = useState(true);

  useEffect(() => {
    const supabase = getSupabaseClient();

    supabase.auth.getSession().then(({ data: { session } }) => {
      setSession(session);
      setUser(session?.user ?? null);
      setIsLoading(false);
    });

    const { data: authListener } = supabase.auth.onAuthStateChange(
      (_event, session) => {
        setSession(session);
        setUser(session?.user ?? null);
      }
    );

    return () => authListener.subscription.unsubscribe();
  }, []);

  const getAccessToken = async () => {
    const { data } = await getSupabaseClient().auth.getSession();
    return data.session?.access_token ?? null;
  };

  const signOut = async () => {
    await getSupabaseClient().auth.signOut();
  };

  return (
    <AuthContext.Provider value={{ user, session, isLoading, getAccessToken, signOut }}>
      {children}
    </AuthContext.Provider>
  );
}

export function useAuth() {
  const ctx = useContext(AuthContext);
  if (!ctx) throw new Error("useAuth must be used within AuthProvider");
  return ctx;
}
```

- [ ] **Step 4: Update auth utility**

```typescript
// frontend/src/infrastructure/utilities.auth.ts

import { getSupabaseClient } from "../auth/supabase";

export async function getAuthorizationHeader(): Promise<{
  Authorization: string;
} | null> {
  const { data } = await getSupabaseClient().auth.getSession();
  const token = data.session?.access_token;
  if (!token) return null;
  return { Authorization: `Bearer ${token}` };
}
```

- [ ] **Step 5: Update GraphQL client**

In `frontend/src/infrastructure/graphQL/graphql-client.ts`, update the auth link:
```typescript
// Replace keycloak-based auth with:
import { getSupabaseClient } from "../../auth/supabase";

// In auth link setup:
const authLink = setContext(async (_, { headers }) => {
  const { data } = await getSupabaseClient().auth.getSession();
  const token = data.session?.access_token;
  return {
    headers: {
      ...headers,
      ...(token ? { Authorization: `Bearer ${token}` } : {}),
    },
  };
});
```

- [ ] **Step 6: Update config and types**

```typescript
// frontend/src/config/runtime-config.ts
// Replace keycloak fields:
export interface AppRuntimeConfig {
  supabaseUrl: string;
  supabaseAnonKey: string;
  graphqlApiUrl: string;
  graphqlCacheApiUrl: string;
  restApiUrl: string;
  mediaUploadApiUrl: string;
}
```

```typescript
// frontend/src/env.d.ts
// Replace PUBLIC_KEYCLOAK_* with:
declare const PUBLIC_SUPABASE_URL: string;
declare const PUBLIC_SUPABASE_ANON_KEY: string;
declare const PUBLIC_GRAPHQL_API_URL: string;
declare const PUBLIC_GRAPHQL_CACHE_API_URL: string;
declare const PUBLIC_REST_API_URL: string;
declare const PUBLIC_MEDIA_UPLOAD_API_URL: string;
```

- [ ] **Step 7: Update .env.example**

```bash
# frontend/.env.example
PUBLIC_SUPABASE_URL=http://localhost:8000
PUBLIC_SUPABASE_ANON_KEY=eyJhbGciOiJI...  # Your Supabase project anon key
PUBLIC_GRAPHQL_API_URL=http://localhost:8989/graphql
PUBLIC_REST_API_URL=http://localhost:8989/api
PUBLIC_MEDIA_UPLOAD_API_URL=http://localhost:8989/api/media/upload
```

- [ ] **Step 8: Remove old keycloak files**

```bash
git rm frontend/src/auth/keycloak.ts
cd frontend && npm uninstall keycloak-js
```

- [ ] **Step 9: Verify frontend builds**

```bash
cd frontend && npm run build
```
Expected: Successful build.

- [ ] **Step 10: Commit**

```bash
git add frontend/
git commit -m "feat: replace keycloak-js with @supabase/supabase-js for auth"
```

---

## Verification Checklist

- [ ] pgvector migration runs cleanly on a fresh db
- [ ] pgvector migration runs cleanly on an existing db (up from previous migration)
- [ ] VectorStore stores embeddings and can search them
- [ ] Three-tier translation lookup still works (DB → pgvector → OpenAI)
- [ ] Similarity re-use threshold (≥95%) works correctly
- [ ] Missing JWT returns 401
- [ ] Invalid JWT returns 401
- [ ] Valid JWT passes through middleware and populates token extension
- [ ] Email is correctly extracted from Supabase token for audit columns
- [ ] Frontend auth flow works: sign in → use app → token refresh → sign out
- [ ] GraphQL requests include Supabase Bearer token
- [ ] `cargo test` passes for all existing tests
- [ ] `cargo build` succeeds with no warnings
