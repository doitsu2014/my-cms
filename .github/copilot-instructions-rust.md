# Rust Backend Coding Guidelines for My-CMS

This document provides detailed guidelines for writing Rust code in the My-CMS backend, with a focus on async programming, concurrency, and project-specific patterns.

## Table of Contents

1. [Architecture Patterns](#architecture-patterns)
2. [Async and Concurrency](#async-and-concurrency)
3. [Error Handling](#error-handling)
4. [Database Operations](#database-operations)
5. [API Development](#api-development)
6. [Testing](#testing)
7. [Code Examples](#code-examples)

## Architecture Patterns

### Layered Architecture

The backend follows a clean layered architecture:

```
API Layer (services/src/api/)
    ↓ calls
Application Core (services/application_core/src/)
    ↓ uses
Database Layer (SeaORM entities)
```

**Key Principles:**
- API layer handles HTTP routing and serialization
- Application core contains business logic (command handlers)
- Never put business logic in API handlers
- Use dependency injection through struct fields

### Command Handler Pattern

All business operations are implemented as command handlers:

```rust
pub trait SomeCommandHandlerTrait {
    fn handle_some_operation(
        &self,
        request: SomeRequest,
    ) -> impl std::future::Future<Output = Result<SomeResponse, AppError>>;
}

#[derive(Debug)]
pub struct SomeCommandHandler {
    pub db: Arc<DatabaseConnection>,
    // other dependencies
}

impl SomeCommandHandlerTrait for SomeCommandHandler {
    async fn handle_some_operation(
        &self,
        request: SomeRequest,
    ) -> Result<SomeResponse, AppError> {
        // Business logic here
    }
}
```

**Best Practices:**
- Define a trait for each command handler
- Implement the trait on a concrete struct
- Accept dependencies (database, config, etc.) via struct fields
- Return `Result<T, AppError>` from all fallible operations
- Use `#[instrument]` from tracing for observability

### Module Organization

```rust
// In mod.rs
pub mod some_operation;

pub use some_operation::{
    SomeRequest,
    SomeResponse,
    SomeHandler,
    SomeHandlerTrait,
};

// In some_operation/mod.rs
mod some_handler;
mod some_request;
mod some_response;

pub use some_handler::*;
pub use some_request::*;
pub use some_response::*;
```

## Async and Concurrency

### Tokio Runtime

The project uses Tokio as the async runtime with the `full` feature set.

```rust
use tokio::task::JoinSet;
use tokio::spawn;
```

### Async Function Guidelines

1. **Use `async fn` syntax** for async functions:
```rust
async fn fetch_data(id: Uuid) -> Result<Data, AppError> {
    // Implementation
}
```

2. **Use `.await` consistently**:
```rust
let result = some_async_operation().await?;
```

3. **Prefer trait async functions** with `impl Future`:
```rust
trait MyTrait {
    fn my_method(&self) -> impl std::future::Future<Output = Result<T, E>>;
}
```

### Parallel Processing with JoinSet

Use `JoinSet` for concurrent operations that need to be joined:

```rust
use tokio::task::JoinSet;

async fn process_items_concurrently(items: Vec<Item>) -> Result<Vec<Result>, AppError> {
    let mut join_set = JoinSet::new();
    
    // Spawn tasks
    for item in items {
        join_set.spawn(async move {
            process_single_item(item).await
        });
    }
    
    // Collect results
    let mut results = Vec::new();
    while let Some(result) = join_set.join_next().await {
        match result {
            Ok(inner_result) => results.push(inner_result?),
            Err(e) => return Err(AppError::TaskJoinError(format!("{}", e))),
        }
    }
    
    Ok(results)
}
```

**Real-world example from AI translation:**
```rust
// Translate chunks in parallel using JoinSet
let mut join_set = JoinSet::new();

for (i, chunk) in chunks.iter().enumerate() {
    let client = client.clone();
    let chunk = chunk.clone();
    let language = target_language_code.to_string();
    
    join_set.spawn(async move {
        Self::translate_text_internal(&client, &chunk, &language, &format!("chunk-{}", i)).await
    });
}

// Wait for all translations to complete
let mut translated_chunks = Vec::with_capacity(chunks.len());
while let Some(result) = join_set.join_next().await {
    match result {
        Ok(translation) => translated_chunks.push(translation?),
        Err(e) => return Err(AppError::OpenAIError(format!("Task join error: {}", e))),
    }
}
```

### Background Task Spawning

For fire-and-forget operations, use `tokio::spawn`:

```rust
pub async fn handle_operation_background(
    &self,
    request: Request,
) -> Result<Uuid, AppError> {
    let db = Arc::clone(&self.db);
    let operation_id = Uuid::new_v4();
    
    tokio::spawn(async move {
        match Self::perform_operation(db, request).await {
            Ok(_) => tracing::info!("Background operation {} completed", operation_id),
            Err(e) => tracing::error!("Background operation {} failed: {}", operation_id, e),
        }
    });
    
    Ok(operation_id)
}
```

### Concurrency Best Practices

1. **Avoid blocking operations** in async contexts:
   - Don't use `std::thread::sleep`, use `tokio::time::sleep`
   - Don't use blocking I/O operations

2. **Use Arc for shared ownership** across tasks:
```rust
let db = Arc::clone(&self.db);
tokio::spawn(async move {
    // Use db here
});
```

3. **Limit concurrency** when needed:
```rust
use tokio::sync::Semaphore;

let semaphore = Arc::new(Semaphore::new(10)); // Max 10 concurrent operations
```

4. **Handle cancellation** gracefully:
```rust
use tokio::select;

select! {
    result = some_operation() => {
        // Operation completed
    }
    _ = tokio::signal::ctrl_c() => {
        // Graceful shutdown
    }
}
```

## Error Handling

### AppError Type

Use the project's custom `AppError` type for all errors:

```rust
use application_core::common::app_error::AppError;

fn operation() -> Result<T, AppError> {
    // ...
}
```

### Error Propagation

Use the `?` operator to propagate errors:

```rust
async fn complex_operation() -> Result<Data, AppError> {
    let step1 = first_operation().await?;
    let step2 = second_operation(step1).await?;
    let result = third_operation(step2).await?;
    Ok(result)
}
```

### Error Conversion

Implement `From` trait for error conversion:

```rust
impl From<sea_orm::DbErr> for AppError {
    fn from(err: sea_orm::DbErr) -> Self {
        AppError::DatabaseError(err.to_string())
    }
}
```

### Error Context

Add context to errors when needed:

```rust
operation()
    .await
    .map_err(|e| AppError::OperationFailed(format!("Failed to process item {}: {}", id, e)))?
```

## Database Operations

### SeaORM Best Practices

The project uses SeaORM with a schema-first approach.

#### Entity Generation

Entities are auto-generated from the database schema:

```bash
sea-orm-cli generate entity \
  --database-url postgres://... \
  -o application_core/src/entities \
  --with-serde both \
  --model-extra-attributes 'serde(rename_all = "camelCase")' \
  --seaography
```

**Important:** Never manually edit generated entities. Make schema changes through migrations.

#### Query Patterns

1. **Find by ID:**
```rust
use application_core::entities::{posts, prelude::*};

let post = Posts::find_by_id(post_id)
    .one(&*db)
    .await?
    .ok_or(AppError::NotFound(format!("Post {} not found", post_id)))?;
```

2. **Filter queries:**
```rust
use sea_orm::{ColumnTrait, QueryFilter};

let translations = post_translations::Entity::find()
    .filter(post_translations::Column::PostId.eq(post_id))
    .filter(post_translations::Column::LanguageCode.eq(language_code))
    .all(&*db)
    .await?;
```

3. **Insert:**
```rust
let new_model = posts::ActiveModel {
    id: Set(Uuid::new_v4()),
    title: Set(title.to_string()),
    content: Set(content.to_string()),
    ..Default::default()
};

let result = Posts::insert(new_model)
    .exec(&*db)
    .await?;
```

4. **Update:**
```rust
let mut active_model: posts::ActiveModel = existing_post.into();
active_model.title = Set(new_title.to_string());
active_model.updated_at = Set(Some(chrono::Utc::now().naive_utc()));

let updated = active_model.update(&*db).await?;
```

5. **Delete:**
```rust
let result = Posts::delete_by_id(post_id)
    .exec(&*db)
    .await?;

if result.rows_affected == 0 {
    return Err(AppError::NotFound(format!("Post {} not found", post_id)));
}
```

#### Transaction Handling

Use transactions for multi-step operations:

```rust
use sea_orm::TransactionTrait;

let txn = db.begin().await?;

// Perform multiple operations
operation1(&txn).await?;
operation2(&txn).await?;

txn.commit().await?;
```

### Database Connection Management

Use `Arc<DatabaseConnection>` for sharing the connection:

```rust
#[derive(Debug)]
pub struct Handler {
    pub db: Arc<DatabaseConnection>,
}
```

## API Development

### Axum Handler Pattern

```rust
use axum::{
    extract::{Path, State},
    Json,
};
use uuid::Uuid;

pub async fn create_handler(
    State(state): State<AppState>,
    Json(request): Json<CreateRequest>,
) -> Result<Json<CreateResponse>, AppError> {
    let handler = SomeCommandHandler {
        db: state.conn.clone(),
    };
    
    let response = handler.handle_create(request).await?;
    Ok(Json(response))
}

pub async fn get_handler(
    State(state): State<AppState>,
    Path(id): Path<Uuid>,
) -> Result<Json<GetResponse>, AppError> {
    let handler = SomeCommandHandler {
        db: state.conn.clone(),
    };
    
    let response = handler.handle_get(id).await?;
    Ok(Json(response))
}
```

### Router Configuration

```rust
use axum::{
    routing::{get, post, put, delete},
    Router,
};

pub fn create_router() -> Router<AppState> {
    Router::new()
        .route("/items", post(create_handler))
        .route("/items/:id", get(get_handler))
        .route("/items/:id", put(update_handler))
        .route("/items/:id", delete(delete_handler))
}
```

### Request/Response Models

Use serde for serialization:

```rust
use serde::{Deserialize, Serialize};
use uuid::Uuid;

#[derive(Debug, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct CreateRequest {
    pub title: String,
    pub content: String,
}

#[derive(Debug, Serialize)]
#[serde(rename_all = "camelCase")]
pub struct CreateResponse {
    pub id: Uuid,
    pub created_at: String,
}
```

## Testing

### Unit Tests with SeaORM Mock

```rust
#[cfg(test)]
mod tests {
    use super::*;
    use sea_orm::{DatabaseBackend, MockDatabase};

    #[tokio::test]
    async fn test_find_by_id() {
        let db = MockDatabase::new(DatabaseBackend::Postgres)
            .append_query_results(vec![
                vec![posts::Model {
                    id: Uuid::new_v4(),
                    title: "Test".to_string(),
                    // ... other fields
                }]
            ])
            .into_connection();

        let handler = PostHandler { db: Arc::new(db) };
        let result = handler.find_by_id(test_id).await;
        
        assert!(result.is_ok());
    }
}
```

### Integration Tests with Testcontainers

```rust
#[cfg(test)]
mod integration_tests {
    use testcontainers::runners::AsyncRunner;
    use testcontainers_modules::postgres::Postgres;

    #[tokio::test]
    async fn test_full_workflow() {
        let postgres = Postgres::default().start().await.unwrap();
        let connection_string = format!(
            "postgres://postgres:postgres@127.0.0.1:{}/postgres",
            postgres.get_host_port_ipv4(5432).await.unwrap()
        );
        
        let db = Database::connect(&connection_string).await.unwrap();
        
        // Run migrations
        Migrator::up(&db, None).await.unwrap();
        
        // Test operations
        // ...
    }
}
```

## Code Examples

### Complete Command Handler Example

```rust
use async_trait::async_trait;
use sea_orm::{DatabaseConnection, EntityTrait, QueryFilter, ColumnTrait};
use std::sync::Arc;
use tracing::instrument;
use uuid::Uuid;

use crate::{
    common::app_error::AppError,
    entities::{posts, post_translations},
};

// Request model
#[derive(Debug)]
pub struct CreatePostRequest {
    pub title: String,
    pub content: String,
    pub category_id: Uuid,
}

// Response model
#[derive(Debug)]
pub struct CreatePostResponse {
    pub id: Uuid,
    pub title: String,
    pub slug: String,
}

// Trait definition
pub trait CreatePostHandlerTrait {
    fn handle_create_post(
        &self,
        request: CreatePostRequest,
    ) -> impl std::future::Future<Output = Result<CreatePostResponse, AppError>>;
}

// Handler implementation
#[derive(Debug)]
pub struct CreatePostHandler {
    pub db: Arc<DatabaseConnection>,
}

impl CreatePostHandlerTrait for CreatePostHandler {
    #[instrument(skip(self))]
    async fn handle_create_post(
        &self,
        request: CreatePostRequest,
    ) -> Result<CreatePostResponse, AppError> {
        // Generate UUID and slug
        let post_id = Uuid::new_v4();
        let slug = slugify::slugify!(&request.title, max_length = 100);
        
        // Create post model
        let new_post = posts::ActiveModel {
            id: sea_orm::Set(post_id),
            title: sea_orm::Set(request.title.clone()),
            slug: sea_orm::Set(slug.clone()),
            content: sea_orm::Set(request.content),
            category_id: sea_orm::Set(request.category_id),
            ..Default::default()
        };
        
        // Insert into database
        posts::Entity::insert(new_post)
            .exec(&*self.db)
            .await?;
        
        // Return response
        Ok(CreatePostResponse {
            id: post_id,
            title: request.title,
            slug,
        })
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use sea_orm::{DatabaseBackend, MockDatabase};

    #[tokio::test]
    async fn test_create_post() {
        let db = MockDatabase::new(DatabaseBackend::Postgres)
            .append_exec_results(vec![
                sea_orm::MockExecResult {
                    last_insert_id: 1,
                    rows_affected: 1,
                }
            ])
            .into_connection();

        let handler = CreatePostHandler {
            db: Arc::new(db),
        };

        let request = CreatePostRequest {
            title: "Test Post".to_string(),
            content: "Test content".to_string(),
            category_id: Uuid::new_v4(),
        };

        let result = handler.handle_create_post(request).await;
        assert!(result.is_ok());
    }
}
```

### Async Concurrency Example (AI Translation)

```rust
// Example from the AI translation service
async fn translate_large_content_internal(
    client: &Client<OpenAIConfig>,
    content: &str,
    target_language_code: &str,
) -> Result<String, AppError> {
    let is_html = Self::is_html_content(content);
    
    // Split into chunks
    let chunks = Self::split_content_into_chunks(content, MAX_CHUNK_SIZE);
    
    if chunks.len() == 1 {
        // Single chunk - translate directly
        return Self::translate_text_internal(client, &chunks[0], target_language_code, "content").await;
    }
    
    tracing::info!("Translating {} chunks in parallel", chunks.len());
    
    // Translate chunks in parallel using JoinSet
    let mut join_set = JoinSet::new();
    
    for (i, chunk) in chunks.iter().enumerate() {
        let client = client.clone();
        let chunk = chunk.clone();
        let language = target_language_code.to_string();
        
        join_set.spawn(async move {
            Self::translate_text_internal(&client, &chunk, &language, &format!("chunk-{}", i)).await
        });
    }
    
    // Collect results in order
    let mut translated_chunks = Vec::with_capacity(chunks.len());
    while let Some(result) = join_set.join_next().await {
        match result {
            Ok(translation) => translated_chunks.push(translation?),
            Err(e) => return Err(AppError::OpenAIError(format!("Task join error: {}", e))),
        }
    }
    
    // Join translated chunks
    let separator = if is_html { "" } else { "\n\n" };
    Ok(translated_chunks.join(separator))
}
```

## Performance Considerations

1. **Use connection pooling** - SeaORM handles this automatically
2. **Implement caching** - Use `moka` for in-memory caching
3. **Optimize database queries** - Use indexes, avoid N+1 queries
4. **Parallel processing** - Use `JoinSet` for independent operations
5. **Lazy evaluation** - Use iterators when possible

## Observability

### Tracing with OpenTelemetry

```rust
use tracing::{info, warn, error, instrument};

#[instrument(skip(self))]
async fn some_operation(&self, id: Uuid) -> Result<Data, AppError> {
    info!("Starting operation for id={}", id);
    
    let result = perform_work(id).await?;
    
    info!("Operation completed successfully for id={}", id);
    Ok(result)
}
```

**Best Practices:**
- Use `#[instrument]` on important functions
- Skip large fields with `skip(field_name)`
- Log important state changes
- Use appropriate log levels (info, warn, error)

## Summary

Key takeaways for Rust development in My-CMS:

1. **Follow layered architecture** - Keep concerns separated
2. **Use command handlers** - Encapsulate business logic
3. **Embrace async/await** - Use Tokio for concurrency
4. **Use JoinSet for parallel work** - Efficient concurrent processing
5. **Handle errors explicitly** - Use Result and AppError
6. **Use SeaORM correctly** - Never edit generated entities
7. **Write tests** - Use mocks for unit tests, testcontainers for integration
8. **Add tracing** - Use OpenTelemetry for observability
