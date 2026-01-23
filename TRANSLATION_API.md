# Translation API Documentation

## Overview

The Translation API provides endpoints for translating Post content to different languages using OpenAI's GPT-4o-mini model. The service supports both synchronous and background translation modes.

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
  "targetLanguage": "VI"
}
```

**Fields:**
- `targetLanguage` (string): Target language code (e.g., "VI" for Vietnamese, "ES" for Spanish, "FR" for French)

**Response (200 OK):**
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

**Response Fields:**
- `translationId` (string): UUID of the created translation record
- `status` (string): Always "completed" for synchronous translation

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
curl -X POST "http://localhost:8989/posts/550e8400-e29b-41d4-a716-446655440000/translate" \
  -H "Authorization: Bearer YOUR_ACCESS_TOKEN" \
  -H "Content-Type: application/json" \
  -d '{"targetLanguage": "VI"}'
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
  "targetLanguage": "VI"
}
```

**Fields:**
- `targetLanguage` (string): Target language code (e.g., "VI" for Vietnamese, "ES" for Spanish, "FR" for French)

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
curl -X POST "http://localhost:8989/posts/550e8400-e29b-41d4-a716-446655440000/translate/background" \
  -H "Authorization: Bearer YOUR_ACCESS_TOKEN" \
  -H "Content-Type: application/json" \
  -d '{"targetLanguage": "VI"}'
```

---

## Features

### Translation Caching
- The service automatically checks for existing translations before making OpenAI API calls
- If a translation already exists for the same post and language, it returns immediately (0 API cost)
- Cache hits are logged for monitoring

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
2. **Temperature Control**: Uses 0.3 temperature for deterministic translations
3. **Token Limits**: Caps responses at 3000 tokens
4. **Smart Model**: Uses GPT-4o-mini (most cost-effective)
5. **Vector DB Similarity**: Optional Qdrant integration for reusing similar translations

## Rate Limiting

Rate limiting depends on your OpenAI API tier. The service includes:
- Automatic retry logic (handled by async-openai library)
- Error handling for rate limit responses
- Recommended: Implement application-level rate limiting for production

## Best Practices

1. **Use Background Mode for Large Posts**: Posts with >2000 characters benefit from background processing
2. **Monitor Logs**: Check logs for cache hits and Qdrant connection status
3. **Configure Qdrant**: Optional but recommended for cost optimization through semantic search
4. **Language Codes**: Use clear language codes (e.g., "Vietnamese" instead of "VI" for better results)
5. **Batch Operations**: Use background mode for translating multiple posts

## Examples

### Translate Multiple Posts

```bash
# Get posts to translate
posts=$(curl -H "Authorization: Bearer $TOKEN" \
  "http://localhost:8989/posts" | jq -r '.data[].id')

# Translate each in background
for post_id in $posts; do
  curl -X POST "http://localhost:8989/posts/$post_id/translate/background" \
    -H "Authorization: Bearer $TOKEN" \
    -H "Content-Type: application/json" \
    -d '{"targetLanguage": "Vietnamese"}'
  echo "Started translation for post $post_id"
done
```

### Check Translation Exists

Query the database to check if a translation exists:

```sql
SELECT id, language_code, title 
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

### Duplicate Translation Errors

**Issue:** Getting logical errors about duplicate translations

**Solution:** The service caches translations. To update:
1. Delete the existing translation from `post_translations` table
2. Run translation again
3. Or modify the business logic to allow updates

## Performance

**Synchronous Mode:**
- Small posts (<500 chars): ~2-3 seconds
- Medium posts (500-2000 chars): ~3-5 seconds
- Large posts (>2000 chars): ~5-15 seconds (depends on chunks)

**Background Mode:**
- Returns immediately (<100ms)
- Actual translation time same as synchronous
- Check database for completion

**With Qdrant:**
- Additional ~100-200ms for embedding generation
- Semantic search: <50ms per query

## Support

For issues or questions:
1. Check logs: Look for detailed error messages and stack traces
2. Verify configuration: Ensure `OPENAI_API_KEY` is set correctly
3. Test OpenAI connection: Try a simple API call to OpenAI directly
4. Review documentation: See `application_core/src/commands/ai/README.md` for implementation details
