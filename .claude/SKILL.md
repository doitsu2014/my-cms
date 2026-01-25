# SKILL.md - Architecture & Patterns Guide

This document describes the architecture patterns, coding guidelines, and best practices for the My-CMS project.

## Table of Contents

1. [Backend Architecture (Rust)](#backend-architecture-rust)
2. [Frontend Architecture (React)](#frontend-architecture-react)
3. [AI Translation Patterns](#ai-translation-patterns)
4. [Media Management Patterns](#media-management-patterns)
5. [Testing Patterns](#testing-patterns)
6. [Deployment Patterns](#deployment-patterns)

---

## Backend Architecture (Rust)

### Layered Architecture Overview

```
┌─────────────────────────────────────────────────────────────┐
│                    Presentation Layer                        │
│  src/api/           - HTTP handlers (thin controllers)       │
│  src/presentation_models/ - API response/error models        │
└─────────────────────────────────────────────────────────────┘
                              │
                              ▼
┌─────────────────────────────────────────────────────────────┐
│                    Application Layer                         │
│  application_core/src/commands/ - Use case handlers          │
│  application_core/src/graphql/  - GraphQL schema             │
└─────────────────────────────────────────────────────────────┘
                              │
                              ▼
┌─────────────────────────────────────────────────────────────┐
│                      Domain Layer                            │
│  application_core/src/entities/ - SeaORM domain models       │
│  application_core/src/common/   - Shared domain logic        │
└─────────────────────────────────────────────────────────────┘
                              │
                              ▼
┌─────────────────────────────────────────────────────────────┐
│                   Infrastructure Layer                       │
│  migration/         - Database migrations                    │
│  PostgreSQL         - Primary database                       │
│  S3                 - Media storage                          │
│  Qdrant             - Vector database (optional)             │
└─────────────────────────────────────────────────────────────┘
```

**Key Principle**: Dependencies flow inward. The `application_core` crate has no knowledge of Axum or HTTP concerns.

### Command Handler Pattern

Each domain entity follows CRUD organization:

```
application_core/src/commands/{entity}/
├── mod.rs
├── create/
│   ├── mod.rs
│   ├── create_handler.rs    # Handler trait + implementation
│   └── create_request.rs    # Request DTO
├── read/
│   ├── mod.rs
│   ├── read_handler.rs
│   └── read_response.rs     # Response DTO
├── modify/
│   ├── mod.rs
│   ├── modify_handler.rs
│   └── modify_request.rs
└── delete/
    ├── mod.rs
    └── delete_handler.rs
```

### Handler Trait Pattern

Every handler defines a trait for testability and loose coupling:

```rust
pub trait PostCreateHandlerTrait {
    fn handle_create_post(
        &self,
        body: CreatePostRequest,
        actor_email: Option<String>,
    ) -> impl std::future::Future<Output = Result<Uuid, AppError>>;
}

#[derive(Debug)]
pub struct PostCreateHandler {
    pub db: Arc<DatabaseConnection>,
}

impl PostCreateHandlerTrait for PostCreateHandler {
    #[instrument]
    async fn handle_create_post(
        &self,
        body: CreatePostRequest,
        actor_email: Option<String>,
    ) -> Result<Uuid, AppError> {
        // Business logic with transaction
    }
}
```

### Request/Response DTOs

Request models convert to domain entities:

```rust
#[derive(Deserialize, Serialize, Debug, Clone)]
#[serde(rename_all = "camelCase")]
pub struct CreatePostRequest {
    pub title: String,
    pub content: String,
    pub published: bool,
    pub category_id: Uuid,
    // ...
}

impl CreatePostRequest {
    pub fn into_model(&self) -> posts::Model {
        // Convert DTO to domain model with business logic (e.g., slug generation)
    }
}
```

### API Handler Pattern (Thin Controllers)

API handlers in `src/api/` are thin - they only:
1. Extract HTTP/auth concerns
2. Delegate to application core
3. Map results to HTTP responses

```rust
#[instrument]
pub async fn api_create_post(
    state: State<AppState>,
    Extension(token): Extension<KeycloakToken<String>>,
    Json(body): Json<CreatePostRequest>,
) -> impl IntoResponse {
    let handler = PostCreateHandler {
        db: state.conn.clone(),
    };

    match handler.handle_create_post(body, Some(token.extract_email().email)).await {
        Ok(id) => ApiResponseWith::new(id.to_string()).to_axum_response(),
        Err(e) => ApiResponseError::from(e).to_axum_response(),
    }
}
```

### Error Handling

Centralized domain error type in `application_core/src/common/app_error.rs`:

```rust
pub enum AppError {
    Db(DbErr),
    DbTx(TransactionError<DbErr>),
    S3Error(S3Error),
    Validation(String, String),        // (field, message)
    Logical(String),                   // Business logic errors
    ConcurrencyOptimistic(String),     // Optimistic locking conflicts
    NotFound,
    OpenAIError(String),               // AI service errors
    Unknown,
}
```

Presentation layer maps domain errors to HTTP responses via `From<AppError>` implementation.

### Transaction Management

Use SeaORM transactions for multi-entity operations:

```rust
let result = self.db.as_ref()
    .transaction::<_, Uuid, AppError>(|tx| {
        Box::pin(async move {
            // 1. Create related entities
            let tags = tag_handler.handle_create_tags_in_transaction(tags, tx).await?;

            // 2. Insert main entity
            let post = Posts::insert(model).exec(tx).await?;

            // 3. Insert relationships
            post_tags::Entity::insert_many(relations).exec(tx).await?;

            Ok(post.last_insert_id)
        })
    })
    .await;
```

### Optimistic Concurrency Control

Entities use `row_version` for conflict detection:

```rust
// Check and increment version
let current_version = body.row_version;
model.row_version = Set(current_version + 1);

let result = Entity::update_many()
    .set(model)
    .filter(Column::Id.eq(id))
    .filter(Column::RowVersion.eq(current_version))
    .exec(tx)
    .await?;

if result.rows_affected == 0 {
    return Err(AppError::ConcurrencyOptimistic("Version mismatch".to_string()));
}
```

### Domain Extensions

Shared utilities in `application_core/src/common/extensions.rs`:

```rust
pub trait StringExtension {
    fn to_slug(&self) -> String;
}

impl StringExtension for String {
    fn to_slug(&self) -> String {
        slugify!(self)
    }
}
```

Use in request DTOs: `self.title.to_slug()`

### GraphQL Integration

Two schema types in `application_core/src/graphql/`:
- **Immutable schema**: Public read-only queries
- **Mutable schema**: Protected mutations

Built with `async-graphql` and `seaography` for automatic entity-to-GraphQL mapping.

### Authentication & Authorization

Uses Keycloak via `axum-keycloak-auth` with role-based access control:

```rust
// Three router groups in my-cms-api.rs:
// 1. public_router - Health checks, immutable GraphQL, media delivery
// 2. protected_router - CRUD operations (requires my-headless-cms-writer)
// 3. protected_administrator_router - Migrations (requires my-headless-cms-administrator)
```

---

## Frontend Architecture (React)

### Technology Stack

- **Framework**: React 19 with TypeScript
- **Build Tool**: Rsbuild (Rspack-based bundler)
- **UI Framework**: DaisyUI 5.x + Tailwind CSS 4.x
- **Rich Text Editor**: TipTap with extensive extensions
- **Forms**: React Hook Form + Zod validation
- **GraphQL Client**: Apollo Client
- **Routing**: React Router v7
- **Authentication**: Keycloak JS (Authorization Code Flow + PKCE)

### Project Structure

```
frontend/
├── src/
│   ├── app/                  # Pages and layouts
│   │   └── admin/            # Admin pages (blogs, categories, media)
│   ├── auth/                 # Keycloak authentication
│   ├── components/           # Reusable UI components
│   ├── config/               # Configuration files
│   ├── domains/              # Domain models
│   ├── infrastructure/       # GraphQL client, utilities
│   ├── models/               # Data models
│   └── schemas/              # Zod validation schemas
├── public/                   # Static assets
├── rsbuild.config.ts         # Build configuration
└── tailwind.config.ts        # Tailwind configuration
```

### Component Patterns

**1. Functional Components with Hooks**

```typescript
import { useState, useEffect } from 'react';

export function BlogList() {
  const [blogs, setBlogs] = useState<Blog[]>([]);

  useEffect(() => {
    // Fetch blogs
  }, []);

  return <div>{/* Render blogs */}</div>;
}
```

**2. Composition Pattern**

Build complex UIs from simple components:

```typescript
export function BlogCard({ blog }: { blog: Blog }) {
  return (
    <Card>
      <CardHeader title={blog.title} />
      <CardBody content={blog.preview} />
      <CardActions>
        <EditButton blogId={blog.id} />
        <DeleteButton blogId={blog.id} />
      </CardActions>
    </Card>
  );
}
```

### Form Management Pattern

Use React Hook Form with Zod validation:

```typescript
import { useForm } from 'react-hook-form';
import { zodResolver } from '@hookform/resolvers/zod';
import { z } from 'zod';

const blogSchema = z.object({
  title: z.string().min(1, 'Title is required'),
  content: z.string().min(10, 'Content must be at least 10 characters'),
  published: z.boolean(),
});

type BlogFormData = z.infer<typeof blogSchema>;

export function BlogForm() {
  const {
    register,
    handleSubmit,
    formState: { errors },
  } = useForm<BlogFormData>({
    resolver: zodResolver(blogSchema),
  });

  const onSubmit = (data: BlogFormData) => {
    // Handle form submission
  };

  return (
    <form onSubmit={handleSubmit(onSubmit)}>
      <input {...register('title')} />
      {errors.title && <span>{errors.title.message}</span>}
      {/* Other fields */}
    </form>
  );
}
```

### TipTap Rich Text Editor Pattern

Configure TipTap with extensions:

```typescript
import { useEditor, EditorContent } from '@tiptap/react';
import StarterKit from '@tiptap/starter-kit';
import Image from '@tiptap/extension-image';
import Link from '@tiptap/extension-link';

export function RichTextEditor({ content, onChange }: EditorProps) {
  const editor = useEditor({
    extensions: [
      StarterKit,
      Image,
      Link,
      // Add more extensions as needed
    ],
    content,
    onUpdate: ({ editor }) => {
      onChange(editor.getHTML());
    },
  });

  return <EditorContent editor={editor} />;
}
```

### GraphQL Data Fetching Pattern

Use Apollo Client hooks:

```typescript
import { useQuery, gql } from '@apollo/client';

const GET_BLOGS = gql`
  query GetBlogs {
    blogs {
      id
      title
      content
      published
    }
  }
`;

export function BlogList() {
  const { loading, error, data } = useQuery(GET_BLOGS);

  if (loading) return <LoadingSpinner />;
  if (error) return <ErrorMessage error={error} />;

  return (
    <div>
      {data.blogs.map((blog) => (
        <BlogCard key={blog.id} blog={blog} />
      ))}
    </div>
  );
}
```

### Authentication Context Pattern

Share auth state across app:

```typescript
// auth/KeycloakProvider.tsx
import Keycloak from 'keycloak-js';
import { createContext, useContext, useEffect, useState } from 'react';

const KeycloakContext = createContext<KeycloakContextType | null>(null);

export function KeycloakProvider({ children }) {
  const [keycloak, setKeycloak] = useState<Keycloak | null>(null);
  const [authenticated, setAuthenticated] = useState(false);

  useEffect(() => {
    const kc = new Keycloak({
      url: process.env.PUBLIC_KEYCLOAK_URL,
      realm: process.env.PUBLIC_KEYCLOAK_REALM,
      clientId: process.env.PUBLIC_KEYCLOAK_CLIENT_ID,
    });

    kc.init({ onLoad: 'login-required', pkceMethod: 'S256' })
      .then((auth) => {
        setAuthenticated(auth);
        setKeycloak(kc);
      });
  }, []);

  return (
    <KeycloakContext.Provider value={{ keycloak, authenticated }}>
      {children}
    </KeycloakContext.Provider>
  );
}

export const useKeycloak = () => useContext(KeycloakContext);
```

---

## AI Translation Patterns

### 3-Tier Lookup Strategy

Cost optimization strategy to minimize OpenAI API usage:

```rust
// 1. Database cache lookup (fastest, free)
if let Some(existing) = lookup_existing_translation(db, post_id, language).await? {
    return Ok(existing);
}

// 2. Vector similarity search (fast, free if configured)
if let Some(similar) = find_similar_translation(vector_store, post, language).await? {
    if similar.score >= 0.95 {
        // Reuse similar translation
        return Ok(create_from_similar(similar));
    }
}

// 3. OpenAI translation (slow, costs money)
let translation = translate_from_openai(post, language, api_key).await?;
```

**Benefits**:
- **Tier 1**: Instant response for exact matches
- **Tier 2**: 95%+ similarity reuse saves API costs
- **Tier 3**: Only call OpenAI when necessary

### Content Chunking Pattern

Handle large content by splitting into processable chunks:

```rust
const MAX_CHUNK_SIZE: usize = 1500; // Characters per chunk
const MAX_TOKENS_PER_REQUEST: u16 = 8000;

// HTML-aware chunking (preserves structure)
if is_html_content(content) {
    let chunks = chunk_html_content(content, MAX_CHUNK_SIZE);
    // Translate chunks in parallel
    translate_chunks_parallel(chunks).await
} else {
    // Plain text chunking (preserves sentences)
    let chunks = chunk_text(content, MAX_CHUNK_SIZE);
    translate_chunks_parallel(chunks).await
}
```

### Parallel Processing Pattern

Use `tokio::task::JoinSet` for concurrent translations:

```rust
let mut join_set = JoinSet::new();

for (index, chunk) in chunks.into_iter().enumerate() {
    join_set.spawn(async move {
        translate_text_internal(&client, &chunk, language).await
            .map(|text| (index, text))
    });
}

// Collect and reassemble in order
let mut results = Vec::new();
while let Some(result) = join_set.join_next().await {
    results.push(result??);
}
results.sort_by_key(|(index, _)| *index);
let combined = results.into_iter().map(|(_, text)| text).collect::<String>();
```

### Background Task Pattern

Execute long-running translations asynchronously:

```rust
async fn handle_translate_post_background(
    &self,
    request: TranslatePostRequest,
    openai_api_key: String,
) -> Result<Uuid, AppError> {
    let translation_id = Uuid::new_v4();
    let db = self.db.clone();

    tokio::spawn(async move {
        match translate_post(db, request, api_key).await {
            Ok(_) => tracing::info!("Background translation completed"),
            Err(e) => tracing::error!("Background translation failed: {}", e),
        }
    });

    Ok(translation_id) // Return immediately
}
```

### Vector Store Integration Pattern

Store and search embeddings for similarity matching:

```rust
// Store translation in Qdrant after creation
vector_store.store_translation(
    post_id,
    language_code,
    translation_id,
    title,
    &content_for_embedding,
).await?;

// Search for similar content
let similar = vector_store.search_similar_translations(
    &search_text,
    limit: 5
).await?;

// Filter by similarity threshold
for (metadata, score) in similar {
    if score >= SIMILARITY_REUSE_THRESHOLD {
        // Reuse this translation
    }
}
```

---

## Media Management Patterns

### Upload with S3 Integration

```rust
// Upload to S3-compatible storage
pub async fn upload_to_s3(
    bucket: &Bucket,
    file_path: &str,
    content_type: &str,
    data: &[u8],
) -> Result<(), S3Error> {
    bucket.put_object_with_content_type(file_path, data, content_type).await?;
    Ok(())
}
```

### Image Resizing Pattern

Resize images on-the-fly with query parameters:

```rust
// GET /media/images/{path}?w=800&h=600
pub async fn serve_resized_image(
    Path(file_path): Path<String>,
    Query(params): Query<ImageParams>,
) -> impl IntoResponse {
    let original = fetch_from_s3(&file_path).await?;

    if let (Some(width), Some(height)) = (params.w, params.h) {
        let img = image::load_from_memory(&original)?;
        let resized = img.resize(width, height, FilterType::Lanczos3);
        let mut buffer = Vec::new();
        resized.write_to(&mut buffer, ImageFormat::Jpeg)?;
        return Ok(buffer);
    }

    Ok(original)
}
```

### Caching Pattern

Use Moka for in-memory caching:

```rust
use moka::future::Cache;

lazy_static! {
    static ref MEDIA_CACHE: Cache<String, Vec<u8>> = Cache::builder()
        .max_capacity(500)
        .time_to_live(Duration::from_secs(3600))
        .build();
}

pub async fn get_cached_media(key: &str) -> Option<Vec<u8>> {
    MEDIA_CACHE.get(key).await
}

pub async fn cache_media(key: String, data: Vec<u8>) {
    MEDIA_CACHE.insert(key, data).await;
}
```

---

## Testing Patterns

### Unit Tests with SeaORM Mock

```rust
#[cfg(test)]
mod tests {
    use sea_orm::{DatabaseBackend, MockDatabase, MockExecResult};

    #[test]
    fn test_create_post() {
        let db = MockDatabase::new(DatabaseBackend::Postgres)
            .append_query_results(vec![vec![posts::Model {
                id: Uuid::new_v4(),
                title: "Test".to_string(),
                // ...
            }]])
            .append_exec_results(vec![MockExecResult {
                last_insert_id: 1,
                rows_affected: 1,
            }])
            .into_connection();

        // Test with mocked database
    }
}
```

### Integration Tests with Testcontainers

```rust
use test_helpers::{setup_test_space, ContainerAsyncPostgresEx};

#[async_std::test]
async fn integration_test_create_post() {
    // Spins up PostgreSQL container
    let test_space = setup_test_space().await;
    let db = test_space.postgres.get_database_connection().await;
    let arc_conn = Arc::new(db);

    let handler = PostCreateHandler { db: arc_conn };
    let request = CreatePostRequest {
        title: "Test Post".to_string(),
        content: "Test Content".to_string(),
        published: true,
        category_id: test_category_id,
    };

    let result = handler.handle_create_post(request, None).await;
    assert!(result.is_ok());
}
```

### Mocking External Services

Use `wiremock` for HTTP service mocking:

```rust
use wiremock::{Mock, MockServer, ResponseTemplate, matchers::{method, path}};

#[async_std::test]
async fn test_openai_translation() {
    let mock_server = MockServer::start().await;

    Mock::given(method("POST"))
        .and(path("/chat/completions"))
        .respond_with(ResponseTemplate::new(200).set_body_json(json!({
            "choices": [{
                "message": { "content": "Translated text" }
            }]
        })))
        .mount(&mock_server)
        .await;

    // Test translation with mocked OpenAI endpoint
}
```

### Test Helpers Pattern

Create reusable test utilities in `test_helpers/`:

```rust
pub async fn setup_test_space() -> TestSpace {
    // Start containers, run migrations, return test environment
}

pub fn fake_create_post_request(category_id: Uuid, index: usize) -> CreatePostRequest {
    CreatePostRequest {
        title: format!("Test Post {}", index),
        content: format!("Test Content {}", index),
        published: true,
        category_id,
    }
}
```

---

## Deployment Patterns

### Docker Multi-Stage Build

```dockerfile
# Stage 1: Build
FROM rust:1.75 as builder
WORKDIR /app
COPY . .
RUN cargo build --release

# Stage 2: Runtime
FROM debian:bookworm-slim
COPY --from=builder /app/target/release/my-cms-api /usr/local/bin/
CMD ["my-cms-api"]
```

### Kubernetes Deployment with Helm

```yaml
# deployments/charts/my-cms-api/templates/deployment.yaml
apiVersion: apps/v1
kind: Deployment
metadata:
  name: {{ include "my-cms-api.fullname" . }}
spec:
  replicas: {{ .Values.replicaCount }}
  template:
    spec:
      containers:
      - name: api
        image: "{{ .Values.image.repository }}:{{ .Values.image.tag }}"
        env:
        - name: DATABASE_URL
          valueFrom:
            secretKeyRef:
              name: {{ .Values.database.secretName }}
              key: connection-string
        resources:
          limits:
            cpu: {{ .Values.resources.limits.cpu }}
            memory: {{ .Values.resources.limits.memory }}
```

### CI/CD Workflow Pattern

```yaml
# .github/workflows/ci-my-cms.yml
name: CI - My CMS API

on:
  push:
    branches: [main, develop]
  pull_request:

jobs:
  test:
    runs-on: ubuntu-latest
    steps:
      - uses: actions/checkout@v3
      - uses: actions-rs/toolchain@v1
        with:
          toolchain: stable
      - name: Run tests
        run: cargo test --all --all-features

  build:
    needs: test
    runs-on: ubuntu-latest
    steps:
      - name: Build Docker image
        run: docker build -t my-cms-api:${{ github.sha }} .
      - name: Push to registry
        run: docker push my-cms-api:${{ github.sha }}
```

### Environment Configuration Pattern

Use `.env` files with validation:

```rust
use dotenv::dotenv;
use std::env;

#[derive(Debug)]
pub struct Config {
    pub database_url: String,
    pub host: String,
    pub port: u16,
    pub s3_endpoint: String,
    pub openai_api_key: Option<String>,
}

impl Config {
    pub fn from_env() -> Result<Self, String> {
        dotenv().ok();

        Ok(Config {
            database_url: env::var("DATABASE_URL")
                .map_err(|_| "DATABASE_URL must be set")?,
            host: env::var("HOST").unwrap_or_else(|_| "127.0.0.1".to_string()),
            port: env::var("PORT")
                .unwrap_or_else(|_| "8989".to_string())
                .parse()
                .map_err(|_| "PORT must be a valid number")?,
            s3_endpoint: env::var("S3_ENDPOINT")
                .map_err(|_| "S3_ENDPOINT must be set")?,
            openai_api_key: env::var("OPENAI_API_KEY").ok(),
        })
    }
}
```

---

## Adding a New Entity

1. **Create migration** in `migration/src/`:
   ```rust
   // m20240101_000000_create_new_entity.rs
   pub struct Migration;

   impl MigrationTrait for Migration {
       async fn up(&self, manager: &SchemaManager) -> Result<(), DbErr> {
           manager.create_table(/* ... */).await
       }
   }
   ```

2. **Generate entity** after migration:
   ```bash
   sea-orm-cli generate entity \
     --database-url <url> \
     -o application_core/src/entities \
     --with-serde both \
     --model-extra-attributes 'serde(rename_all = "camelCase")' \
     --seaography
   ```

3. **Create command handlers** in `application_core/src/commands/{entity}/`:
   - Define handler traits
   - Implement request/response DTOs
   - Add `into_model()` conversion methods

4. **Create API handlers** in `src/api/{entity}/`:
   - Thin controllers delegating to command handlers
   - Register routes in `src/bin/my-cms-api.rs`

5. **Export modules** in respective `mod.rs` files

6. **Write tests**:
   - Unit tests with SeaORM mock
   - Integration tests with testcontainers

---

## Key Design Principles

1. **Dependency Inversion**: Application core defines traits; presentation layer provides implementations
2. **Single Responsibility**: Each handler has one job (create, read, modify, delete)
3. **Separation of Concerns**: HTTP concerns stay in `src/`, business logic in `application_core/`
4. **Explicit Dependencies**: Use `Arc<DatabaseConnection>` passed via struct fields
5. **Fail Fast**: Validate at boundaries, propagate errors with `?`
6. **Observability**: Use `#[instrument]` macro on handlers for distributed tracing
7. **Type Safety**: Leverage Rust's type system and TypeScript's strict mode
8. **Cost Optimization**: Use caching, similarity search, and background tasks to minimize API costs
9. **Testability**: Design for testing with dependency injection and mocking
10. **Performance**: Use async/await, parallel processing, and caching where appropriate

---

## Common Patterns Summary

| Pattern | Use Case | Location |
|---------|----------|----------|
| Command Handler | Business logic encapsulation | `application_core/src/commands/` |
| Thin Controller | HTTP request handling | `src/api/` |
| DTO Conversion | API ↔ Domain model mapping | Request/Response structs |
| Transaction | Multi-entity operations | Handler implementations |
| Optimistic Locking | Concurrent update prevention | Entity modify operations |
| 3-Tier Lookup | Cost-optimized AI translation | AI translation handler |
| Chunking | Large content processing | AI translation handler |
| Parallel Processing | Concurrent task execution | `tokio::task::JoinSet` |
| Caching | Performance optimization | Media handlers |
| Repository | Data access abstraction | SeaORM entities |
| Dependency Injection | Testability | Handler constructors |
| Mock Testing | Unit tests | SeaORM mock, wiremock |
| Integration Testing | End-to-end tests | Testcontainers |
| Context Provider | Shared state (React) | Auth, theme contexts |
| Form Validation | Input validation | React Hook Form + Zod |
| GraphQL Client | Data fetching | Apollo Client hooks |

---

## Technology References

### Backend
- [Rust Documentation](https://doc.rust-lang.org/)
- [SeaORM Documentation](https://www.sea-ql.org/SeaORM/)
- [Axum Documentation](https://docs.rs/axum/)
- [async-graphql](https://async-graphql.github.io/async-graphql/)
- [Tokio Runtime](https://tokio.rs/)

### Frontend
- [React Documentation](https://react.dev/)
- [Rsbuild Documentation](https://rsbuild.dev/)
- [DaisyUI Components](https://daisyui.com/)
- [TipTap Editor](https://tiptap.dev/)
- [React Hook Form](https://react-hook-form.com/)
- [Zod Validation](https://zod.dev/)
- [Apollo Client](https://www.apollographql.com/docs/react/)

### Infrastructure
- [Docker Documentation](https://docs.docker.com/)
- [Kubernetes Documentation](https://kubernetes.io/docs/)
- [Helm Charts](https://helm.sh/docs/)
- [Keycloak Documentation](https://www.keycloak.org/documentation)
