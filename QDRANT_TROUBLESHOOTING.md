# Qdrant Integration Troubleshooting Guide

This guide helps diagnose and resolve issues with the Qdrant vector database integration.

## Quick Checklist

- [ ] Qdrant server is running
- [ ] `QDRANT_URL` environment variable is set correctly
- [ ] `OPENAI_API_KEY` environment variable is set
- [ ] Network connectivity to Qdrant server
- [ ] Check application logs for Qdrant-related messages

## Common Issues

### Issue 1: No Collections or Items in Qdrant

**Symptoms:**
- Translations complete successfully
- No collection appears in Qdrant dashboard
- No points/items stored in collection

**Diagnosis Steps:**

1. **Check if QDRANT_URL is configured:**
   ```bash
   echo $QDRANT_URL
   # Should output something like: http://localhost:6334
   ```

2. **Check application logs for Qdrant messages:**
   Look for these log messages when making a translation request:
   ```
   ✓ QDRANT_URL configured: http://localhost:6334
   ✓ Successfully connected to Qdrant
   ✓ Created Qdrant collection: translations with 1536 dimensions
   # or
   ✓ Qdrant collection 'translations' already exists
   ✓ Qdrant vector store ready for use
   ✓ Successfully stored translation embedding in Qdrant...
   ✓ Verified: Point {uuid} exists in collection 'translations'
   ```

3. **Check for error messages:**
   Look for these error indicators:
   ```
   ✗ Failed to connect to Qdrant...
   ✗ Failed to initialize Qdrant collection...
   ✗ Failed to store translation embedding...
   ⚠ Warning: Point was not found after storage...
   ```

4. **Verify Qdrant is running:**
   ```bash
   curl http://localhost:6333/health
   # Should return: {"title":"healthz","version":"..."}
   ```

5. **Check Qdrant collections via API:**
   ```bash
   curl http://localhost:6333/collections
   # Should list collections including "translations"
   ```

6. **Check collection points count:**
   ```bash
   curl http://localhost:6333/collections/translations
   # Look for "points_count" field
   ```

**Common Causes:**

1. **QDRANT_URL not set:**
   - Log message: `QDRANT_URL not configured - vector storage disabled`
   - **Solution:** Set the environment variable:
     ```bash
     export QDRANT_URL=http://localhost:6334
     ```

2. **Wrong port configured:**
   - Qdrant has two ports: 6333 (HTTP) and 6334 (gRPC)
   - This application uses **gRPC port 6334**
   - **Solution:** Use `QDRANT_URL=http://localhost:6334`

3. **Qdrant not running:**
   - **Solution:** Start Qdrant:
     ```bash
     docker run -p 6333:6333 -p 6334:6334 qdrant/qdrant
     ```

4. **Network connectivity issues:**
   - If using Docker, ensure containers are on same network
   - If using remote Qdrant, check firewall rules
   - **Solution:** Test connectivity:
     ```bash
     nc -zv localhost 6334
     ```

5. **API using cached vector store initialization:**
   - The API initializes vector store per request
   - If initialization fails silently, subsequent requests won't retry
   - **Solution:** Restart the API server after fixing Qdrant configuration

### Issue 2: Collections Created but No Points

**Symptoms:**
- Collection "translations" exists in Qdrant
- `points_count` is 0
- Translations complete successfully

**Diagnosis:**

1. Check application logs for embedding storage:
   ```
   ✓ Successfully stored translation embedding...
   ✓ Verified: Point {uuid} exists...
   ```

2. If you see this warning:
   ```
   ⚠ Warning: Point {uuid} was not found after storage...
   ```
   This indicates the upsert succeeded but verification failed.

**Possible Causes:**

1. **OpenAI API key not set:**
   - Embedding generation requires OpenAI API key
   - **Solution:** Set `OPENAI_API_KEY`:
     ```bash
     export OPENAI_API_KEY=sk-...
     ```

2. **OpenAI API quota exceeded:**
   - Check logs for OpenAI error messages
   - **Solution:** Check your OpenAI account usage and limits

3. **Qdrant upsert failure:**
   - Check logs for upsert errors
   - **Solution:** Check Qdrant server logs:
     ```bash
     docker logs <qdrant-container-id>
     ```

4. **Timing issue (rare):**
   - Qdrant may not have committed the point yet
   - **Solution:** Query collection after a few seconds:
     ```bash
     curl http://localhost:6333/collections/translations
     ```

### Issue 3: Vector Store Initialization Fails

**Symptoms:**
- Log message: `✗ Failed to initialize Qdrant collection: ...`
- Translations still work but no vector storage

**Diagnosis:**

1. Check the specific error message in logs
2. Common errors:
   - "Connection refused" - Qdrant not running or wrong URL
   - "Unauthorized" - Qdrant requires authentication (not configured in code)
   - "Timeout" - Network issues or Qdrant overloaded

**Solutions:**

1. **Connection refused:**
   ```bash
   # Verify Qdrant is running
   docker ps | grep qdrant
   
   # Check port mapping
   docker port <qdrant-container-id>
   ```

2. **Timeout:**
   ```bash
   # Check Qdrant resource usage
   docker stats <qdrant-container-id>
   ```

3. **Authentication required:**
   - Current implementation doesn't support authentication
   - Use Qdrant without authentication or update code to add auth support

## Verification Steps

### 1. Manual Test of Full Flow

```bash
# 1. Set environment variables
export QDRANT_URL=http://localhost:6334
export OPENAI_API_KEY=sk-...

# 2. Start Qdrant
docker run -d -p 6333:6333 -p 6334:6334 --name qdrant qdrant/qdrant

# 3. Verify Qdrant is healthy
curl http://localhost:6333/health

# 4. Start your application
cargo run --bin my-cms-api

# 5. Make a translation request
curl -X POST "http://localhost:8989/posts/{post_id}/translate" \
  -H "Authorization: Bearer YOUR_TOKEN" \
  -H "Content-Type: application/json" \
  -d '{"targetLanguage": "VI"}'

# 6. Check logs for Qdrant messages (look for ✓ and ✗ symbols)

# 7. Verify collection was created
curl http://localhost:6333/collections/translations | jq

# 8. Check points count
curl http://localhost:6333/collections/translations | jq '.result.points_count'

# 9. List some points
curl http://localhost:6333/collections/translations/points/scroll \
  -H "Content-Type: application/json" \
  -d '{"limit": 5, "with_payload": true}'
```

### 2. Test Qdrant Connection from Code

```rust
// You can add this to your test suite
#[tokio::test]
#[ignore] // Run with: cargo test test_qdrant_connection -- --ignored
async fn test_qdrant_connection() {
    use application_core::commands::ai::vector_store::VectorStore;
    
    let qdrant_url = std::env::var("QDRANT_URL").expect("QDRANT_URL must be set");
    let api_key = std::env::var("OPENAI_API_KEY").expect("OPENAI_API_KEY must be set");
    
    let vector_store = VectorStore::new(&qdrant_url, api_key)
        .await
        .expect("Failed to connect to Qdrant");
    
    vector_store.initialize_collection()
        .await
        .expect("Failed to initialize collection");
    
    println!("✓ Successfully connected to Qdrant and initialized collection");
}
```

### 3. Check Qdrant Dashboard

If you have Qdrant dashboard accessible (usually at http://localhost:6333/dashboard):

1. Navigate to Collections
2. Look for "translations" collection
3. Check:
   - Vector dimensions (should be 1536)
   - Distance metric (should be Cosine)
   - Points count (should increase with each translation)
4. Click on the collection to view points
5. Inspect point payloads to verify metadata

## Expected Log Flow for Successful Operation

When everything works correctly, you should see logs like this:

```
[INFO] QDRANT_URL configured: http://localhost:6334
[INFO] Attempting to connect to Qdrant and initialize collection...
[INFO] ✓ Successfully connected to Qdrant
[INFO] Initializing Qdrant collection: translations
[INFO] ✓ Qdrant collection 'translations' already exists
[INFO] ✓ Qdrant vector store ready for use
[INFO] Vector store is configured - attempting to store translation embedding for post_id=abc123 language=VI
[INFO] Storing translation in Qdrant: post_id=abc123 language=VI translation_id=xyz789
[DEBUG] Generating embedding for 542 characters
[DEBUG] Generated embedding with 1536 dimensions
[DEBUG] Upserting point to collection 'translations'
[INFO] ✓ Successfully stored translation in Qdrant vector DB: post_id=abc123 language=VI translation_id=xyz789 (operation_id=12345)
[INFO] ✓ Verified: Point xyz789 exists in collection 'translations'
[INFO] ✓ Successfully stored translation embedding in Qdrant for post_id=abc123 language=VI translation_id=xyz789
```

## Getting Help

If you're still experiencing issues after following this guide:

1. **Collect diagnostic information:**
   - Application logs (with Qdrant-related messages)
   - Qdrant server logs: `docker logs <qdrant-container-id>`
   - Environment variables: `env | grep -E '(QDRANT|OPENAI)'`
   - Qdrant status: `curl http://localhost:6333/collections/translations`

2. **Check common patterns:**
   - Do you see "QDRANT_URL not configured" in logs? → Set the environment variable
   - Do you see "✗ Failed to connect"? → Verify Qdrant is running and accessible
   - Do you see "✓ Successfully stored" but no points in collection? → Check Qdrant logs

3. **Verify with minimal test:**
   - Use the test code provided in section 2 above
   - This isolates the Qdrant connection from the rest of the application

## Additional Resources

- [Qdrant Documentation](https://qdrant.tech/documentation/)
- [Qdrant Rust Client](https://github.com/qdrant/rust-client)
- [OpenAI Embeddings API](https://platform.openai.com/docs/guides/embeddings)
- Project documentation:
  - `VECTOR_DB_INTEGRATION_GUIDE.md` - Comprehensive integration guide
  - `TRANSLATION_API.md` - API documentation
  - `TRANSLATION_TESTS.md` - Testing guide
