# Translation API Documentation

## Overview

The Translation API provides endpoints for translating Post content to different languages using OpenAI's GPT-4o-mini model. The service supports:
- **Synchronous and background translation modes**
- **Force re-translation** with Qdrant vector database similarity checking
- **Smart translation reuse** - automatically reuses highly similar existing translations to save costs

## Features

### Smart Translation Reuse (Cost Savings!)

When Qdrant is configured, the system automatically detects and reuses highly similar existing translations:

- **Automatic Detection**: Searches for translations with similarity score ≥ 0.95 (95% similar)
- **Cost Savings**: Eliminates OpenAI API calls for near-duplicate content (5-15% typical savings)
- **Faster Response**: Instant response for reused translations (~200-500ms vs 2-5 seconds)
- **Transparency**: Response includes `reusedFromSimilar` metadata when reuse occurs
- **Smart Logic**: Only reuses translations in same language, excludes self-matching

See `SMART_TRANSLATION_REUSE.md` for complete documentation.

## Authentication

All translation endpoints require authentication with the `my-headless-cms-writer` role via Keycloak.

## Configuration

Required environment variables:

```env
OPENAI_API_KEY=sk-...              # Required: OpenAI API key
QDRANT_URL=http://localhost:6334   # Optional: Qdrant vector database for semantic search
```

## Endpoints

### 1. Translate Post (Synchronous)

Translates a post and waits for completion before returning the result.

**Endpoint:** `POST /posts/{post_id}/translate`

**Request Headers:**
```
Authorization: Bearer {access_token}
Content-Type: application/json
```

**Path Parameters:**
- `post_id` (UUID): The ID of the post to translate

**Request Body:**
```json
{
  "targetLanguage": "VI",
  "forceRetranslate": false
}
```

**Fields:**
- `targetLanguage` (string, required): Target language code (e.g., "VI" for Vietnamese, "ES" for Spanish, "FR" for French)
- `forceRetranslate` (boolean, optional, default: false): When true, forces re-translation even if a translation already exists. The system will:
  1. Check Qdrant vector database for similar translations (if configured) and log results
  2. Delete the existing translation
  3. Create a new translation with the latest AI model

**Response (200 OK):**

*New Translation (OpenAI API called):*
```json
{
  "data": {
    "translationId": "550e8400-e29b-41d4-a716-446655440000",
    "status": "completed"
  },
  "isSuccess": true,
  "errors": []
}
```

*Reused Translation (Cost Savings!):*
```json
{
  "data": {
    "translationId": "550e8400-e29b-41d4-a716-446655440000",
    "status": "completed",
    "reusedFromSimilar": {
      "sourceTranslationId": "750e8400-e29b-41d4-a716-446655440002",
      "similarityScore": 0.972,
      "sourcePostId": "850e8400-e29b-41d4-a716-446655440003"
    }
  },
  "isSuccess": true,
  "errors": []
}
```

**Response Fields:**
- `translationId` (string): UUID of the created translation record
- `status` (string): Always "completed" for synchronous translation
- `reusedFromSimilar` (object, optional): Present when translation was reused from a similar existing translation (cost savings!)
  - `sourceTranslationId` (string): UUID of the reused translation
  - `similarityScore` (number): Semantic similarity score (0.95-1.0)
  - `sourcePostId` (string): UUID of the post that provided the reused translation

**Error Responses:**

*401 Unauthorized:*
```json
{
  "data": null,
  "isSuccess": false,
  "errors": ["Unauthorized"]
}
```

*404 Not Found:*
```json
{
  "data": null,
  "isSuccess": false,
  "errors": ["Post not found"],
  "errorCode": "NOT_FOUND"
}
```

*500 Internal Server Error:*
```json
{
  "data": null,
  "isSuccess": false,
  "errors": ["OPENAI_API_KEY environment variable not set"],
  "errorCode": "CONNECTION_ERROR"
}
```

**Example cURL:**
```bash
# Standard translation (uses cache if exists)
curl -X POST "http://localhost:8989/posts/550e8400-e29b-41d4-a716-446655440000/translate" \
  -H "Authorization: Bearer YOUR_ACCESS_TOKEN" \
  -H "Content-Type: application/json" \
  -d '{"targetLanguage": "VI"}'

# Force re-translation (checks Qdrant for similar translations)
curl -X POST "http://localhost:8989/posts/550e8400-e29b-41d4-a716-446655440000/translate" \
  -H "Authorization: Bearer YOUR_ACCESS_TOKEN" \
  -H "Content-Type: application/json" \
  -d '{"targetLanguage": "VI", "forceRetranslate": true}'
```

---

### 2. Translate Post (Background)

Starts translation in the background and returns immediately. Ideal for large posts or batch operations.

**Endpoint:** `POST /posts/{post_id}/translate/background`

**Request Headers:**
```
Authorization: Bearer {access_token}
Content-Type: application/json
```

**Path Parameters:**
- `post_id` (UUID): The ID of the post to translate

**Request Body:**
```json
{
  "targetLanguage": "VI",
  "forceRetranslate": false
}
```

**Fields:**
- `targetLanguage` (string, required): Target language code (e.g., "VI" for Vietnamese, "ES" for Spanish, "FR" for French)
- `forceRetranslate` (boolean, optional, default: false): When true, forces re-translation even if a translation already exists. The system will:
  1. Check Qdrant vector database for similar translations (if configured) and log results
  2. Delete the existing translation
  3. Create a new translation with the latest AI model

**Response (200 OK):**
```json
{
  "data": {
    "translationId": "550e8400-e29b-41d4-a716-446655440000",
    "status": "processing"
  },
  "isSuccess": true,
  "errors": []
}
```

**Response Fields:**
- `translationId` (string): UUID of the translation record (pre-generated)
- `status` (string): Always "processing" for background translation

**Error Responses:**

Same error responses as synchronous endpoint.

**Example cURL:**
```bash
# Standard background translation
curl -X POST "http://localhost:8989/posts/550e8400-e29b-41d4-a716-446655440000/translate/background" \
  -H "Authorization: Bearer YOUR_ACCESS_TOKEN" \
  -H "Content-Type: application/json" \
  -d '{"targetLanguage": "VI"}'

# Force re-translation in background
curl -X POST "http://localhost:8989/posts/550e8400-e29b-41d4-a716-446655440000/translate/background" \
  -H "Authorization: Bearer YOUR_ACCESS_TOKEN" \
  -H "Content-Type: application/json" \
  -d '{"targetLanguage": "VI", "forceRetranslate": true}'
```

---

## Features

### Translation Caching
- The service automatically checks for existing translations before making OpenAI API calls
- If a translation already exists for the same post and language, it returns immediately (0 API cost)
- Cache hits are logged for monitoring
- Use `forceRetranslate: true` to bypass cache and create a new translation

### Force Re-translation
- Set `forceRetranslate: true` in the request body to re-translate existing content
- When force re-translation is enabled:
  1. System checks Qdrant vector database for similar translations (if configured)
  2. Logs up to 3 most similar translations with similarity scores
  3. Deletes the existing translation from the database
  4. Creates a brand new translation with the latest AI model
- Useful for:
  - Updating translations with improved AI models
  - Correcting translation issues
  - A/B testing different translations
  - Reprocessing after content changes

### Qdrant Similarity Check
- When `forceRetranslate: true` and Qdrant is configured:
  - Searches for semantically similar translations in the vector database
  - Logs similarity scores (0.0 to 1.0, higher is more similar)
  - Shows up to 3 most similar translations
  - Helps identify duplicate or related content
  - Provides insight for cost optimization
- Example log output:
  ```
  Found 5 similar translations in vector DB for post_id=...
    Similar: score=0.892 post_id=abc123 lang=VI title=Similar Article
    Similar: score=0.856 post_id=def456 lang=VI title=Related Content
    Similar: score=0.821 post_id=ghi789 lang=VI title=Another Post
  ```

### HTML-Aware Processing
- Automatically detects HTML content
- Preserves HTML structure and tags during translation
- Only translates text content within tags, never tag names or attributes
- Supports chunking for large HTML content (>2000 characters)

### Content Chunking
- **Plain Text**: Splits at sentence boundaries (periods, exclamation marks, question marks)
- **HTML Content**: Splits at block-level element boundaries (`</p>`, `</div>`, `</section>`, etc.)
- Parallel processing of chunks for faster translation
- Maximum chunk size: 2000 characters

### Vector Database Integration (Optional)
- If `QDRANT_URL` is configured, embeddings are automatically stored after successful translation
- Enables semantic search for similar translations
- Non-blocking: Qdrant failures don't affect translation success
- Uses OpenAI text-embedding-3-small (1536 dimensions)

### Background Processing
- Uses `tokio::spawn` for non-blocking execution
- Returns translation ID immediately
- Ideal for batch translation or large content
- Comprehensive logging for monitoring

## Translation Data Storage

Translations are stored in the `post_translations` table with the following fields:

- `id`: UUID of the translation record
- `post_id`: Reference to the original post
- `language_code`: Target language (e.g., "Vietnamese", "Spanish")
- `title`: Translated title
- `slug`: Auto-generated slug from translated title
- `preview_content`: Translated preview content
- `content`: Translated main content
- `created_at`: Timestamp of translation creation
- `updated_at`: Timestamp of last update

## Error Handling

The API returns errors in the following format:

```json
{
  "data": null,
  "isSuccess": false,
  "errors": ["Error message here"],
  "errorCode": "ERROR_CODE"
}
```

**Error Codes:**
- `CONNECTION_ERROR`: OpenAI API key not set or API connection issues
- `NOT_FOUND`: Post not found
- `VALIDATION_ERROR`: Invalid request body or parameters
- `LOGICAL`: Business logic error (e.g., duplicate translation)
- `UNKNOWN_ERROR`: Unexpected error

## Cost Optimization

The translation service includes several cost-saving features:

1. **Database Caching**: Checks for existing translations (0 cost for cache hits)
2. **Force Re-translation with Similarity Check**: Use Qdrant to identify similar content before re-translating
3. **Temperature Control**: Uses 0.3 temperature for deterministic translations
4. **Token Limits**: Caps responses at 3000 tokens
5. **Smart Model**: Uses GPT-4o-mini (most cost-effective)
6. **Vector DB Similarity**: Optional Qdrant integration for reusing similar translations

## Rate Limiting

Rate limiting depends on your OpenAI API tier. The service includes:
- Automatic retry logic (handled by async-openai library)
- Error handling for rate limit responses
- Recommended: Implement application-level rate limiting for production

## Best Practices

1. **Use Background Mode for Large Posts**: Posts with >2000 characters benefit from background processing
2. **Monitor Logs**: Check logs for cache hits and Qdrant connection status
3. **Configure Qdrant**: Optional but recommended for cost optimization through semantic search
4. **Use Force Re-translate Sparingly**: Only use when truly needed to avoid unnecessary API costs
5. **Check Similarity Scores**: Review Qdrant similarity logs to identify duplicate content
6. **Language Codes**: Use clear language codes (e.g., "Vietnamese" instead of "VI" for better results)
7. **Batch Operations**: Use background mode for translating multiple posts

## Examples

### Standard Translation (First Time)
```bash
curl -X POST "http://localhost:8989/posts/550e8400-e29b-41d4-a716-446655440000/translate" \
  -H "Authorization: Bearer $TOKEN" \
  -H "Content-Type: application/json" \
  -d '{"targetLanguage": "Vietnamese"}'
```

### Re-use Cached Translation
```bash
# Calling again with same post_id and language will return cached result
curl -X POST "http://localhost:8989/posts/550e8400-e29b-41d4-a716-446655440000/translate" \
  -H "Authorization: Bearer $TOKEN" \
  -H "Content-Type: application/json" \
  -d '{"targetLanguage": "Vietnamese"}'
```

### Force Re-translation with Similarity Check
```bash
curl -X POST "http://localhost:8989/posts/550e8400-e29b-41d4-a716-446655440000/translate" \
  -H "Authorization: Bearer $TOKEN" \
  -H "Content-Type: application/json" \
  -d '{
    "targetLanguage": "Vietnamese",
    "forceRetranslate": true
  }'

# Check logs to see similar translations found in Qdrant
```

### Translate Multiple Posts with Re-translation

```bash
# Get posts to translate
posts=$(curl -H "Authorization: Bearer $TOKEN" \
  "http://localhost:8989/posts" | jq -r '.data[].id')

# Translate each in background with force re-translate
for post_id in $posts; do
  curl -X POST "http://localhost:8989/posts/$post_id/translate/background" \
    -H "Authorization: Bearer $TOKEN" \
    -H "Content-Type: application/json" \
    -d '{
      "targetLanguage": "Vietnamese",
      "forceRetranslate": true
    }'
  echo "Started re-translation for post $post_id"
done
```

### Check Translation Exists

Query the database to check if a translation exists:

```sql
SELECT id, language_code, title, created_at, updated_at
FROM post_translations 
WHERE post_id = '550e8400-e29b-41d4-a716-446655440000' 
  AND language_code = 'Vietnamese';
```

## Troubleshooting

### "OPENAI_API_KEY environment variable not set"

**Solution:** Set the `OPENAI_API_KEY` environment variable in your `.env` file:

```env
OPENAI_API_KEY=sk-proj-...
```

### Translation Fails with Large Content

**Issue:** Content exceeds OpenAI token limits

**Solution:** The service automatically chunks large content. If issues persist:
1. Use background mode for better error handling
2. Check logs for specific error messages
3. Consider reducing content size or splitting into multiple posts

### Qdrant Connection Warnings

**Issue:** Seeing "Failed to connect to Qdrant" warnings

**Solution:** This is normal if Qdrant is not required. To enable:
1. Start Qdrant: `docker run -p 6333:6333 -p 6334:6334 qdrant/qdrant`
2. Set environment variable: `QDRANT_URL=http://localhost:6334`
3. Restart the API server

Note: Qdrant is optional. The service works without it.

### Force Re-translation Not Working

**Issue:** Setting `forceRetranslate: true` but getting cached result

**Solution:** Check the logs:
1. You should see "Force retranslation requested for post_id=..."
2. If not, verify JSON format: `{"targetLanguage": "VI", "forceRetranslate": true}`
3. Ensure no typos in field name (camelCase: `forceRetranslate`)

### No Similarity Results When Force Re-translating

**Issue:** No similar translations shown in logs when using `forceRetranslate: true`

**Solution:** 
1. Verify Qdrant is running and `QDRANT_URL` is set
2. Check if any translations are stored in Qdrant (first translation won't have similar ones)
3. Look for "Failed to search similar translations" warnings in logs

## Performance

**Synchronous Mode:**
- Small posts (<500 chars): ~2-3 seconds
- Medium posts (500-2000 chars): ~3-5 seconds
- Large posts (>2000 chars): ~5-15 seconds (depends on chunks)
- With force re-translate: +100-200ms for Qdrant similarity search

**Background Mode:**
- Returns immediately (<100ms)
- Actual translation time same as synchronous
- Check database for completion

**With Qdrant:**
- Additional ~100-200ms for embedding generation
- Semantic search: <50ms per query
- Force re-translate similarity check: ~100-150ms

## Support

For issues or questions:
1. Check logs: Look for detailed error messages and stack traces
2. Verify configuration: Ensure `OPENAI_API_KEY` is set correctly
3. Test OpenAI connection: Try a simple API call to OpenAI directly
4. Review documentation: See `application_core/src/commands/ai/README.md` for implementation details
