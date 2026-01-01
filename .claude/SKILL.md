# SKILL.md - Clean Architecture Guide

This document describes the Clean Architecture patterns used in this Rust CMS project.

## Layered Architecture Overview

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
└─────────────────────────────────────────────────────────────┘
```

**Key Principle**: Dependencies flow inward. The `application_core` crate has no knowledge of Axum or HTTP concerns.

## Command Handler Pattern

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

## API Handler Pattern (Thin Controllers)

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

## Error Handling

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
    Unknown,
}
```

Presentation layer maps domain errors to HTTP responses via `From<AppError>` implementation.

## Transaction Management

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

## Optimistic Concurrency Control

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

## Adding a New Entity

1. **Create migration** in `migration/src/`:
   ```rust
   // m20240101_000000_create_new_entity.rs
   ```

2. **Generate entity** after migration:
   ```bash
   sea-orm-cli generate entity --database-url <url> -o application_core/src/entities ...
   ```

3. **Create command handlers** in `application_core/src/commands/{entity}/`:
   - Define handler traits
   - Implement request/response DTOs
   - Add `into_model()` conversion methods

4. **Create API handlers** in `src/api/{entity}/`:
   - Thin controllers delegating to command handlers
   - Register routes in `src/bin/my-cms-api.rs`

5. **Export modules** in respective `mod.rs` files

## Testing Strategy

### Unit Tests (with SeaORM mock)
```rust
#[cfg(test)]
mod tests {
    use sea_orm::{DatabaseBackend, MockDatabase};

    #[test]
    fn test_handler() {
        let db = MockDatabase::new(DatabaseBackend::Postgres)
            .append_query_results(vec![vec![expected_model]])
            .into_connection();
        // Test with mocked database
    }
}
```

### Integration Tests (with testcontainers)
```rust
#[async_std::test]
async fn integration_test() {
    let test_space = setup_test_space().await;  // Spins up PostgreSQL container
    let db = test_space.postgres.get_database_connection().await;

    let handler = PostCreateHandler { db: Arc::new(db) };
    let result = handler.handle_create_post(request, None).await;

    assert!(result.is_ok());
}
```

## Domain Extensions

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

## GraphQL Integration

Two schema types in `application_core/src/graphql/`:
- **Immutable schema**: Public read-only queries
- **Mutable schema**: Protected mutations

Built with `async-graphql` and `seaography` for automatic entity-to-GraphQL mapping.

## Key Design Principles

1. **Dependency Inversion**: Application core defines traits; presentation layer provides implementations
2. **Single Responsibility**: Each handler has one job (create, read, modify, delete)
3. **Separation of Concerns**: HTTP concerns stay in `src/`, business logic in `application_core/`
4. **Explicit Dependencies**: Use `Arc<DatabaseConnection>` passed via struct fields
5. **Fail Fast**: Validate at boundaries, propagate errors with `?`
6. **Observability**: Use `#[instrument]` macro on handlers for distributed tracing
