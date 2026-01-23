# Vector Database Integration Guide for Translation Service

## Overview

This guide explains how to integrate a vector database like Qdrant with the translation service for semantic search, similarity matching, and cost optimization.

## Why Vector Database?

### Benefits

1. **Semantic Search**: Find translations by meaning, not just keywords
2. **Similar Content Discovery**: Identify related translations across languages
3. **Cost Optimization**: Reuse similar translations to reduce API calls
4. **Better UX**: Enable "similar posts" and content recommendations
5. **Embedding Cache**: Store embeddings for future similarity searches

### Use Cases

- Find translations of similar content before making new API calls
- Recommend related posts in different languages
- Build semantic search features
- Cluster similar content
- Detect duplicate/similar translations

## Architecture

```
┌──────────────────────────┐
│  Translation Handler     │
└─────────┬────────────────┘
          │
          ├──► PostgreSQL (Primary Storage)
          │    - Structured data
          │    - Relational queries
          │    - ACID transactions
          │
          └──► Qdrant (Vector Storage)
               - Embeddings
               - Semantic search
               - Similarity queries
```

## Integration Steps

### 1. Add Dependencies

```toml
[dependencies]
# In application_core/Cargo.toml
qdrant-client = "1.11"
serde_json = "1.0"
```

### 2. Configure Qdrant

**Docker Setup:**
```bash
docker run -p 6333:6333 -p 6334:6334 \
    -v $(pwd)/qdrant_storage:/qdrant/storage:z \
    qdrant/qdrant
```

**Environment Variables:**
```env
# In .env
QDRANT_URL=http://localhost:6334
OPENAI_API_KEY=sk-...
```

### 3. Create Vector Store Module

Create `application_core/src/commands/ai/vector_store.rs`:

```rust
use qdrant_client::Qdrant;
use qdrant_client::qdrant::{
    CreateCollectionBuilder, Distance, VectorParamsBuilder,
    PointStruct, UpsertPointsBuilder,
};
use async_openai::{
    config::OpenAIConfig,
    types::{CreateEmbeddingRequestArgs, EmbeddingInput},
    Client,
};

pub struct VectorStore {
    qdrant: Qdrant,
    openai_client: Client<OpenAIConfig>,
}

impl VectorStore {
    pub async fn new(qdrant_url: &str, api_key: String) -> Result<Self, Box<dyn std::error::Error>> {
        let qdrant = Qdrant::from_url(qdrant_url).build()?;
        let config = OpenAIConfig::new().with_api_key(api_key);
        let openai_client = Client::with_config(config);
        
        Ok(Self { qdrant, openai_client })
    }
    
    pub async fn initialize_collection(&self) -> Result<(), Box<dyn std::error::Error>> {
        self.qdrant.create_collection(
            CreateCollectionBuilder::new("translations")
                .vectors_config(VectorParamsBuilder::new(1536, Distance::Cosine))
        ).await?;
        Ok(())
    }
    
    pub async fn store_translation(
        &self,
        translation_id: &str,
        title: &str,
        content: &str,
        metadata: serde_json::Value,
    ) -> Result<(), Box<dyn std::error::Error>> {
        // Generate embedding
        let text = format!("{} {}", title, content);
        let embedding = self.generate_embedding(&text).await?;
        
        // Create point
        let point = PointStruct::new(
            translation_id.to_string(),
            embedding,
            metadata,
        );
        
        // Upsert to Qdrant
        self.qdrant.upsert_points("translations", vec![point], None).await?;
        Ok(())
    }
    
    async fn generate_embedding(&self, text: &str) -> Result<Vec<f32>, Box<dyn std::error::Error>> {
        let request = CreateEmbeddingRequestArgs::default()
            .model("text-embedding-3-small")
            .input(EmbeddingInput::String(text.to_string()))
            .build()?;
            
        let response = self.openai_client.embeddings().create(request).await?;
        Ok(response.data[0].embedding.clone())
    }
    
    pub async fn search_similar(
        &self,
        query_text: &str,
        limit: usize,
    ) -> Result<Vec<serde_json::Value>, Box<dyn std::error::Error>> {
        let embedding = self.generate_embedding(query_text).await?;
        
        let search_result = self.qdrant
            .search_points("translations", embedding, limit as u64, None, None, None)
            .await?;
            
        let results = search_result.result
            .into_iter()
            .map(|point| point.payload)
            .collect();
            
        Ok(results)
    }
}
```

### 4. Update Translation Handler

Modify `PostTranslateHandler` to integrate vector storage:

```rust
pub struct PostTranslateHandler {
    pub db: Arc<DatabaseConnection>,
    pub vector_store: Option<Arc<VectorStore>>,  // Optional for backward compatibility
}

impl PostTranslateHandlerTrait for PostTranslateHandler {
    async fn handle_translate_post(
        &self,
        request: TranslatePostRequest,
        openai_api_key: String,
    ) -> Result<TranslatePostResponse, AppError> {
        // ... existing translation logic ...
        
        // Store in vector database if available
        if let Some(vector_store) = &self.vector_store {
            let metadata = serde_json::json!({
                "post_id": post_id.to_string(),
                "language_code": language_code,
                "title": translated_title,
                "content_preview": translated_content.chars().take(500).collect::<String>(),
            });
            
            if let Err(e) = vector_store
                .store_translation(
                    &translation_id.to_string(),
                    &translated_title,
                    &translated_content,
                    metadata,
                )
                .await
            {
                // Log but don't fail translation
                tracing::warn!("Failed to store in vector DB: {}", e);
            }
        }
        
        // ... return translation result ...
    }
}
```

### 5. Initialize with Vector Store

```rust
// With vector storage
let vector_store = VectorStore::new(&qdrant_url, openai_api_key.clone()).await?;
vector_store.initialize_collection().await?;

let handler = PostTranslateHandler {
    db: Arc::new(database_connection),
    vector_store: Some(Arc::new(vector_store)),
};

// Without vector storage (backward compatible)
let handler = PostTranslateHandler {
    db: Arc::new(database_connection),
    vector_store: None,
};
```

## Usage Examples

### Semantic Search

```rust
// Find similar translations
let similar = vector_store
    .search_similar("How to use AI for translations", 5)
    .await?;

for result in similar {
    let post_id = result["post_id"].as_str().unwrap();
    let lang = result["language_code"].as_str().unwrap();
    let title = result["title"].as_str().unwrap();
    println!("Similar: {} ({}) - {}", post_id, lang, title);
}
```

### Cost Optimization

```rust
async fn translate_with_similarity_check(
    handler: &PostTranslateHandler,
    post: &Post,
    target_lang: &str,
) -> Result<Translation, AppError> {
    if let Some(vector_store) = &handler.vector_store {
        // Check for similar content
        let similar = vector_store
            .search_similar(&post.content, 1)
            .await?;
            
        if let Some(first) = similar.first() {
            let similarity_score = first["score"].as_f64().unwrap_or(0.0);
            
            // If very similar (>0.95), reuse translation with modifications
            if similarity_score > 0.95 {
                tracing::info!("Reusing similar translation (score: {})", similarity_score);
                // ... adapt existing translation ...
            }
        }
    }
    
    // Otherwise, create new translation
    handler.handle_translate_post(...).await
}
```

## Cost Analysis

### Without Vector DB

- Every translation requires OpenAI API call
- Cost: $0.15 per 1M input tokens + $0.60 per 1M output tokens (GPT-4o-mini)
- No way to find or reuse similar translations

### With Vector DB

- Initial setup: Embedding generation ($0.02 per 1M tokens)
- Storage: Free with self-hosted Qdrant
- Searches: Free (local vector similarity)
- Savings: 20-40% on translations through similarity matching

### Example Cost Savings

For 1000 blog posts:
- **Without vector DB**: 1000 full translations = ~$30-50
- **With vector DB**: 600 new + 400 adapted = ~$20-35
- **Savings**: ~$10-15 (30%) + better quality through consistency

## Best Practices

### 1. Batch Operations

```rust
// Batch store multiple translations
let points: Vec<PointStruct> = translations
    .iter()
    .map(|t| create_point(t))
    .collect();
    
vector_store.qdrant
    .upsert_points("translations", points, None)
    .await?;
```

### 2. Error Handling

```rust
// Don't fail translations if vector storage fails
match vector_store.store_translation(...).await {
    Ok(_) => tracing::info!("Stored in vector DB"),
    Err(e) => tracing::warn!("Vector storage failed: {}", e),
}
```

### 3. Metadata Strategy

```rust
// Store rich metadata for better filtering
let metadata = json!({
    "post_id": post_id,
    "language_code": lang,
    "category": category,
    "tags": tags,
    "created_at": timestamp,
    "word_count": content.split_whitespace().count(),
});
```

### 4. Search Optimization

```rust
// Add filters to searches
let search_result = qdrant.search_points(
    "translations",
    embedding,
    10,
    Some(filter_by_language("Vietnamese")),
    Some(true), // with_payload
    None,
).await?;
```

## Monitoring

### Key Metrics

- **Hit Rate**: % of translations found via similarity
- **API Savings**: Reduced OpenAI API calls
- **Search Latency**: Vector search performance
- **Storage Size**: Qdrant database growth

### Logging

```rust
tracing::info!(
    similarity_score = score,
    reused = true,
    "Found similar translation"
);
```

## Troubleshooting

### Connection Issues

```bash
# Check Qdrant is running
curl http://localhost:6333/health

# View collections
curl http://localhost:6333/collections
```

### Performance Issues

- Index vectors for faster search
- Use appropriate distance metric (Cosine for text)
- Limit payload size in search results
- Consider clustering for large datasets

## Future Enhancements

1. **Multi-language Embeddings**: Use multilingual models
2. **Hybrid Search**: Combine vector + keyword search
3. **Automatic Clustering**: Group similar content
4. **Quality Scoring**: Track translation quality metrics
5. **A/B Testing**: Compare vector-assisted vs. direct translation

## References

- [Qdrant Documentation](https://qdrant.tech/documentation/)
- [OpenAI Embeddings Guide](https://platform.openai.com/docs/guides/embeddings)
- [Semantic Search Best Practices](https://www.pinecone.io/learn/semantic-search/)

## Conclusion

Vector database integration enhances the translation service with:
- ✅ Semantic search capabilities
- ✅ Cost optimization through similarity matching
- ✅ Better content discovery
- ✅ Improved user experience
- ✅ Scalable architecture for future AI features

Implementation is optional but recommended for production systems with high translation volumes.
