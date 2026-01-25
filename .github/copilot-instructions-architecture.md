# My-CMS Architecture and Design Patterns

This document provides a comprehensive overview of the My-CMS architecture, design patterns, and best practices specific to this project.

## Table of Contents

1. [System Architecture](#system-architecture)
2. [Backend Architecture](#backend-architecture)
3. [Frontend Architecture](#frontend-architecture)
4. [AI Translation Service](#ai-translation-service)
5. [Media Management](#media-management)
6. [Database Design](#database-design)
7. [API Design](#api-design)
8. [Security Architecture](#security-architecture)
9. [Deployment Architecture](#deployment-architecture)

## System Architecture

### High-Level Overview

My-CMS is a modern headless CMS built with:
- **Backend**: Rust-based REST and GraphQL APIs
- **Frontend**: React-based admin panel
- **Database**: PostgreSQL with SeaORM
- **AI Services**: OpenAI for translation, Qdrant for vector search
- **Storage**: S3-compatible object storage
- **Observability**: OpenTelemetry + Jaeger tracing

### Architecture Layers

```
┌─────────────────────────────────────────────────────────────┐
│                        Clients                               │
│              (Admin Panel, External Apps)                    │
└──────────────────────┬──────────────────────────────────────┘
                       │
                       ▼
┌─────────────────────────────────────────────────────────────┐
│                      API Layer                               │
│   ┌──────────┬──────────┬──────────┬─────────────────┐     │
│   │   REST   │ GraphQL  │  Media   │   Public API    │     │
│   └──────────┴──────────┴──────────┴─────────────────┘     │
│           (services/src/api/)                               │
└──────────────────────┬──────────────────────────────────────┘
                       │
                       ▼
┌─────────────────────────────────────────────────────────────┐
│                  Application Core                            │
│   ┌──────────────────────────────────────────────────┐     │
│   │         Command Handlers                          │     │
│   │  ┌────────┬──────┬──────┬──────┬──────────┐     │     │
│   │  │Category│ Post │ Tag  │Media │    AI     │     │     │
│   │  └────────┴──────┴──────┴──────┴──────────┘     │     │
│   │                                                   │     │
│   │         Common Domain Logic                      │     │
│   └──────────────────────────────────────────────────┘     │
│        (services/application_core/src/)                     │
└──────────────────────┬──────────────────────────────────────┘
                       │
                       ▼
┌─────────────────────────────────────────────────────────────┐
│                   Data Layer                                 │
│   ┌──────────────────────────────────────────────────┐     │
│   │    SeaORM Entities (Auto-generated)              │     │
│   └──────────────────────────────────────────────────┘     │
│        (services/application_core/src/entities/)            │
└──────────────────────┬──────────────────────────────────────┘
                       │
                       ▼
┌─────────────────────────────────────────────────────────────┐
│                   PostgreSQL Database                        │
└─────────────────────────────────────────────────────────────┘

External Services:
┌──────────┐  ┌──────────┐  ┌──────────┐  ┌──────────┐
│  OpenAI  │  │  Qdrant  │  │    S3    │  │ Keycloak │
└──────────┘  └──────────┘  └──────────┘  └──────────┘
```

## Backend Architecture

### Project Structure

```
services/
├── src/                          # Main API service
│   ├── api/                      # API routes and handlers
│   │   ├── category/             # Category CRUD
│   │   ├── post/                 # Post CRUD + AI translation
│   │   ├── tag/                  # Tag management
│   │   ├── media/                # Media upload/management
│   │   ├── public/               # Public endpoints
│   │   ├── graphql/              # GraphQL endpoint
│   │   └── administrator/        # Admin operations
│   ├── common/                   # Shared utilities
│   │   ├── app_error.rs          # Custom error types
│   │   ├── app_state.rs          # Application state
│   │   └── middlewares/          # Axum middlewares
│   ├── presentation_models/      # Request/response DTOs
│   └── lib.rs                    # Library root
│
├── application_core/             # Business logic layer
│   └── src/
│       ├── commands/             # Command handlers
│       │   ├── category/         # Category operations
│       │   ├── post/             # Post operations
│       │   ├── tag/              # Tag operations
│       │   ├── media/            # Media operations
│       │   └── ai/               # AI services
│       │       ├── translate/    # Translation handler
│       │       └── vector_store.rs # Qdrant integration
│       ├── entities/             # SeaORM entities (generated)
│       ├── common/               # Domain utilities
│       └── lib.rs
│
├── migration/                    # Database migrations
│   └── src/
│       └── m*_*.rs               # Migration files
│
└── test_helpers/                 # Test utilities
    └── src/
        └── lib.rs
```

### Command Pattern

All business logic is organized using the Command Pattern:

```rust
// Define trait
pub trait CommandHandlerTrait {
    fn handle(&self, request: Request) 
        -> impl Future<Output = Result<Response, AppError>>;
}

// Implement handler
pub struct CommandHandler {
    db: Arc<DatabaseConnection>,
    // Other dependencies
}

impl CommandHandlerTrait for CommandHandler {
    async fn handle(&self, request: Request) 
        -> Result<Response, AppError> {
        // Business logic
    }
}
```

**Benefits:**
- Clear separation of concerns
- Easy to test with dependency injection
- Consistent structure across all operations
- Supports both sync and async operations

### Dependency Injection

Dependencies are injected through struct fields:

```rust
pub struct Handler {
    pub db: Arc<DatabaseConnection>,
    pub media_config: Arc<MediaConfig>,
    pub cache: Arc<Cache<K, V>>,
}

// In API handler
let handler = Handler {
    db: state.conn.clone(),
    media_config: state.media_config.clone(),
    cache: state.media_cache.clone(),
};
```

### Application State

Shared application state using Axum's `State` extractor:

```rust
#[derive(Clone)]
pub struct AppState {
    pub conn: Arc<DatabaseConnection>,
    pub media_config: Arc<MediaConfig>,
    pub media_cache: Arc<Cache<MediaCacheKey, CachedMedia>>,
    pub graphql_immutable_schema: Arc<Schema>,
    pub graphql_mutable_schema: Arc<Schema>,
}
```

## Frontend Architecture

### Project Structure

```
frontend/
├── src/
│   ├── app/                      # Application pages
│   │   ├── admin/                # Admin panel
│   │   │   ├── layout.tsx        # Admin layout
│   │   │   ├── page.tsx          # Dashboard
│   │   │   ├── categories/       # Category pages
│   │   │   ├── blogs/            # Blog post pages
│   │   │   ├── media/            # Media pages
│   │   │   └── components/       # Shared admin components
│   │   └── public/               # Public pages
│   │
│   ├── components/               # Reusable components
│   │   ├── toast-provider.tsx    # Toast notification setup
│   │   └── ...
│   │
│   ├── domains/                  # Domain type definitions
│   │   ├── category.ts           # Category types
│   │   ├── post.ts               # Post types
│   │   ├── tag.ts                # Tag types
│   │   └── index.ts
│   │
│   ├── models/                   # API request/response models
│   │   ├── CreateCategoryModel.ts
│   │   ├── UpdateCategoryModel.ts
│   │   └── MediaModels.ts
│   │
│   ├── schemas/                  # Zod validation schemas
│   │   └── category.schema.ts
│   │
│   ├── auth/                     # Authentication logic
│   │   └── AuthContext.tsx       # Auth context provider
│   │
│   ├── config/                   # Configuration
│   │   └── api.config.ts         # API utilities
│   │
│   ├── App.css                   # Global styles
│   └── index.tsx                 # App entry point
│
├── public/                       # Static assets
├── rsbuild.config.ts             # Build configuration
└── package.json
```

### Component Patterns

#### Container/Presentational Pattern

**Container Components** (pages):
```tsx
// Handles data fetching and business logic
export default function CategoriesPage() {
  const [categories, setCategories] = useState([]);
  const { token } = useAuth();
  
  useEffect(() => {
    fetchCategories();
  }, []);
  
  return <CategoryList categories={categories} />;
}
```

**Presentational Components**:
```tsx
// Focuses on rendering UI
export function CategoryList({ categories }: Props) {
  return (
    <div className="grid gap-4">
      {categories.map(cat => <CategoryCard key={cat.id} category={cat} />)}
    </div>
  );
}
```

#### Form Pattern

All forms use React Hook Form + Zod:
```tsx
// 1. Define schema
const schema = z.object({ /* fields */ });

// 2. Use in component
const { register, handleSubmit, formState: { errors } } = useForm({
  resolver: zodResolver(schema)
});

// 3. Handle submission
const onSubmit = async (data) => {
  // API call
};
```

## AI Translation Service

### 3-Tier Lookup Strategy

The AI translation service implements a cost-optimization strategy:

```
Request Translation
    │
    ▼
1. Check Database ──── Found? ──── Return existing
    │                              translation
    │ Not found
    ▼
2. Search Qdrant Vector DB
   (Similarity ≥ 95%)
    │
    ├── Found similar? ──── Reuse translation
    │                       (with attribution)
    │ Not similar enough
    ▼
3. Call OpenAI API ──── Translate ──── Store in DB
    │                                   + Qdrant
    ▼
Return new translation
```

**Implementation Details:**

```rust
// 1. Database lookup
let existing = lookup_existing_translation(db, post_id, language).await?;
if let Some(translation) = existing {
    return Ok(translation); // Cache hit
}

// 2. Vector similarity search
if let Some(similar) = find_similar_translation(vector_store, post, language).await? {
    if similar.similarity_score >= 0.95 {
        // Reuse similar translation
        return reuse_translation(similar);
    }
}

// 3. OpenAI translation
let (title, preview, content) = translate_from_openai(post, language, api_key).await?;
save_translation(db, post_id, language, &title, &preview, &content).await?;
store_in_vector_db(vector_store, post_id, language, translation_id, &title, &content).await;
```

### Parallel Chunk Processing

Large content is split into chunks and translated concurrently:

```rust
async fn translate_large_content(
    client: &Client,
    content: &str,
    language: &str,
) -> Result<String, AppError> {
    // Split into 1500-character chunks
    let chunks = split_content_into_chunks(content, 1500);
    
    // Translate chunks in parallel using JoinSet
    let mut join_set = JoinSet::new();
    for (i, chunk) in chunks.iter().enumerate() {
        let client = client.clone();
        let chunk = chunk.clone();
        let lang = language.to_string();
        
        join_set.spawn(async move {
            translate_chunk(&client, &chunk, &lang).await
        });
    }
    
    // Collect and join results
    let mut results = Vec::new();
    while let Some(result) = join_set.join_next().await {
        results.push(result??);
    }
    
    Ok(results.join("\n\n"))
}
```

**Key Parameters:**
- Max chunk size: 1500 characters
- Temperature: 0.3 (deterministic)
- Max tokens: 8000 per request
- Similarity threshold: 0.95

### Background Processing

Non-critical translations run in background:

```rust
pub async fn handle_translate_post_background(
    &self,
    request: TranslatePostRequest,
    openai_api_key: String,
) -> Result<Uuid, AppError> {
    let operation_id = Uuid::new_v4();
    let db = Arc::clone(&self.db);
    let vector_store = self.vector_store.clone();
    
    tokio::spawn(async move {
        match Self::perform_translation(db, vector_store, request, openai_api_key).await {
            Ok(_) => tracing::info!("Background translation {} completed", operation_id),
            Err(e) => tracing::error!("Background translation {} failed: {}", operation_id, e),
        }
    });
    
    Ok(operation_id)
}
```

## Media Management

### Upload Flow

```
Client Upload
    │
    ▼
Axum Multipart Handler
    │
    ▼
Validate File
  - Type check
  - Size limit
    │
    ▼
Process Image
  - Resize if needed
  - Generate metadata
    │
    ▼
Upload to S3
  - Generate unique key
  - Set ACL
    │
    ▼
Save to Database
  - Store metadata
  - Record URL
    │
    ▼
Return Response
```

### Caching Strategy

Media metadata is cached using Moka:

```rust
pub struct AppState {
    pub media_cache: Arc<Cache<MediaCacheKey, CachedMedia>>,
}

// Cache on read
let cached = cache.get_with(key, async {
    fetch_from_database(id).await
}).await;

// Invalidate on update/delete
cache.invalidate(&key).await;
```

**Cache Configuration:**
- Max capacity: 10,000 items
- TTL: 1 hour
- LRU eviction policy

## Database Design

### Schema-First Approach

1. **Create Migration**:
```rust
// migration/src/m*_create_table.rs
pub struct Migration;

impl MigrationTrait for Migration {
    async fn up(&self, manager: &SchemaManager) -> Result<(), DbErr> {
        manager
            .create_table(
                Table::create()
                    .table(Posts::Table)
                    .if_not_exists()
                    .col(/* columns */)
                    .to_owned(),
            )
            .await
    }
}
```

2. **Run Migration**:
```bash
sea-orm-cli migrate up
```

3. **Generate Entities**:
```bash
sea-orm-cli generate entity -o application_core/src/entities
```

### Key Tables

- **posts**: Blog posts (title, content, status)
- **post_translations**: Multi-language content
- **categories**: Content organization
- **category_translations**: Multi-language categories
- **tags**: Tagging system
- **media**: File metadata
- **media_folders**: Folder structure

### Relationships

```
categories 1──┐
              ├──* posts
tags       *──┘

posts 1──* post_translations

media 1──* media_folders
```

## API Design

### REST API Structure

```
/api/v1
  ├── /categories
  │   ├── GET    /           # List
  │   ├── POST   /           # Create
  │   ├── GET    /:id        # Read
  │   ├── PUT    /:id        # Update
  │   └── DELETE /:id        # Delete
  │
  ├── /posts
  │   ├── GET    /           # List
  │   ├── POST   /           # Create
  │   ├── GET    /:id        # Read
  │   ├── PUT    /:id        # Update
  │   ├── DELETE /:id        # Delete
  │   └── POST   /:id/translate  # AI translation
  │
  ├── /media
  │   ├── GET    /           # List
  │   ├── POST   /upload     # Upload
  │   ├── GET    /:id        # Get metadata
  │   └── DELETE /:id        # Delete
  │
  └── /graphql               # GraphQL endpoint
```

### Request/Response Format

**Standard Response:**
```json
{
  "data": { /* resource or array */ },
  "meta": {
    "total": 100,
    "page": 1,
    "perPage": 20
  }
}
```

**Error Response:**
```json
{
  "error": {
    "code": "NOT_FOUND",
    "message": "Resource not found",
    "details": { /* additional info */ }
  }
}
```

### GraphQL Schema

Generated using Seaography from SeaORM entities:

```graphql
type Post {
  id: UUID!
  title: String!
  slug: String!
  content: String!
  category: Category
  translations: [PostTranslation!]
  tags: [Tag!]
}

type Query {
  posts(limit: Int, offset: Int): [Post!]!
  post(id: UUID!): Post
}
```

## Security Architecture

### Authentication

**Keycloak Integration:**
- JWT tokens for API authentication
- Role-based access control (RBAC)
- Token refresh mechanism

**Flow:**
```
Client Login
    │
    ▼
Keycloak ──── JWT Token ───┐
                           │
                           ▼
Client ──── Token ───> Axum Middleware
                           │
                           ├── Validate Token
                           ├── Check Permissions
                           │
                           ▼
                       Protected Route
```

### Authorization

**Middleware Implementation:**
```rust
use axum_keycloak_auth::{
    decode::{KeycloakToken, KeycloakAuthStatus},
    layer::KeycloakAuthLayer,
};

// Add to router
let app = Router::new()
    .route("/protected", get(handler))
    .layer(KeycloakAuthLayer::new(/* config */));

// In handler
async fn handler(
    Extension(token): Extension<KeycloakToken<User>>,
) -> Result<Json<Response>, AppError> {
    // Access user info from token
    let user_id = token.sub;
    // ...
}
```

### Input Validation

1. **API Layer**: Validate request structure with Serde
2. **Business Layer**: Validate business rules
3. **Database Layer**: Enforce constraints

### Security Best Practices

1. **SQL Injection**: SeaORM uses parameterized queries
2. **XSS**: Sanitize HTML in rich text editor
3. **CSRF**: Not needed for JWT-based API
4. **Rate Limiting**: Implement at reverse proxy level
5. **CORS**: Configure allowed origins

## Deployment Architecture

### Container Strategy

```dockerfile
# Multi-stage build
FROM rust:1.75 as builder
WORKDIR /app
COPY . .
RUN cargo build --release

FROM debian:bookworm-slim
COPY --from=builder /app/target/release/cms /usr/local/bin/
CMD ["cms"]
```

### Kubernetes Deployment

```yaml
apiVersion: apps/v1
kind: Deployment
metadata:
  name: my-cms-api
spec:
  replicas: 3
  template:
    spec:
      containers:
      - name: api
        image: my-cms:latest
        env:
        - name: DATABASE_URL
          valueFrom:
            secretKeyRef:
              name: db-secret
              key: url
```

### Infrastructure Components

```
┌─────────────────────────────────────────────┐
│              Load Balancer                   │
└──────────────────┬──────────────────────────┘
                   │
       ┌───────────┴───────────┐
       │                       │
       ▼                       ▼
┌─────────────┐         ┌─────────────┐
│  API Pod 1  │         │  API Pod 2  │
└─────────────┘         └─────────────┘
       │                       │
       └───────────┬───────────┘
                   │
       ┌───────────┴───────────┐
       │                       │
       ▼                       ▼
┌─────────────┐         ┌─────────────┐
│  PostgreSQL │         │     S3      │
└─────────────┘         └─────────────┘
```

### Observability

**Tracing with OpenTelemetry:**
```
Application ──> OTLP Exporter ──> Jaeger ──> UI
```

**Metrics:**
- Request latency
- Error rates
- Database query performance
- Cache hit rates

## Performance Optimization

### Backend Optimizations

1. **Connection Pooling**: SeaORM manages DB connections
2. **Query Optimization**: Use indexes, limit result sets
3. **Caching**: Moka for in-memory caching
4. **Async Processing**: Tokio for concurrent operations
5. **Lazy Loading**: Load relations only when needed

### Frontend Optimizations

1. **Code Splitting**: Dynamic imports for routes
2. **Lazy Loading**: Images and components
3. **Memoization**: React.memo, useMemo, useCallback
4. **Bundle Optimization**: Tree shaking, minification
5. **CDN**: Serve static assets from CDN

## Monitoring and Alerting

### Key Metrics

- **API Latency**: p50, p95, p99
- **Error Rate**: 4xx, 5xx responses
- **Database Performance**: Query time, connection pool
- **Translation Costs**: OpenAI API usage
- **Storage Usage**: S3 bucket size

### Logging Strategy

- **Info**: Normal operations, state changes
- **Warn**: Recoverable errors, degraded performance
- **Error**: Failures requiring attention
- **Debug**: Development and troubleshooting

## Summary

The My-CMS architecture emphasizes:

1. **Separation of Concerns**: Clear layers with distinct responsibilities
2. **Type Safety**: Rust and TypeScript for compile-time guarantees
3. **Scalability**: Async processing, caching, horizontal scaling
4. **Cost Optimization**: Smart caching and similarity-based reuse
5. **Observability**: Comprehensive tracing and monitoring
6. **Security**: Defense in depth with multiple validation layers
7. **Maintainability**: Consistent patterns and clear structure
