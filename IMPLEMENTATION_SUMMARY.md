# AI Translation Service - Implementation Summary

## Overview
This PR successfully adds an AI-powered translation service to the `application_core` module, enabling automatic translation of Post content to any language using OpenAI's GPT-4o-mini model. **Updated to support background processing and content chunking for large texts.**

## What Was Implemented

### 1. Core Components
- **Translation Handler** (`translate_handler.rs`): Main service implementing the translation logic
  - Fetches post from database
  - Translates title, preview_content, and content
  - **HTML-Aware Chunking**: Automatically detects and handles HTML content properly
  - **Plain Text Chunking**: Falls back to sentence-based chunking for non-HTML content
  - **NEW**: Parallel processing of chunks using tokio::spawn
  - **NEW**: Background processing mode with `handle_translate_post_background`
  - Generates slugs for translated content
  - Saves translations to database
  
- **Request/Response Models** (`translate_request.rs`, `translate_response.rs`): DTOs for the service

### 2. New Features

#### HTML-Aware Content Chunking (NEW)
- **Automatic HTML Detection**: Checks for common HTML tags to determine content type
- **HTML Parsing**: Uses `html5ever` library to parse and understand HTML structure
- **Smart Splitting**: 
  - Splits at block-level element boundaries (`</p>`, `</div>`, `</section>`, etc.)
  - Never breaks HTML tags or attributes
  - Preserves complete HTML structure
- **Specialized Translation Prompts**: Instructs OpenAI to:
  - Only translate text content within tags
  - Never translate HTML tag names or attributes
  - Preserve all HTML structure exactly
- **Fallback Strategy**: Multiple levels of chunking fallback for edge cases

#### Plain Text Chunking
- Automatically splits large content into manageable chunks (max 2000 characters)
- Smart splitting at sentence boundaries to maintain context
- Fallback to size-based chunking if no sentence terminators found

#### Parallel Processing
- Uses `tokio::task::JoinSet` to process chunks concurrently
- Each chunk is translated in parallel for faster processing
- Results are reassembled in correct order

#### Background Processing
- New `handle_translate_post_background` method for non-blocking translations
- Returns translation ID immediately
- Translation happens in background using `tokio::spawn`
- Logs success/failure for monitoring

### 3. Configuration
- Added `async-openai = "0.27"` dependency to Cargo.toml
- **NEW**: Added `html5ever = "0.27"` and `markup5ever_rcdom = "0.3"` for HTML parsing
- Added `OPENAI_API_KEY` to .env configuration file
- Made OpenAI model configurable via `DEFAULT_OPENAI_MODEL` constant
- **NEW**: Configurable chunk size via `MAX_CHUNK_SIZE` constant (default: 2000)

### 4. Error Handling
- Extended `AppError` enum with `OpenAIError(String)` variant
- Proper error propagation throughout the translation flow
- **NEW**: Task join error handling for parallel processing

### 5. Architecture
- Follows existing CQRS-like command handler pattern
- Uses dependency injection with `Arc<DatabaseConnection>`
- Implements trait-based design for testability
- Maintains consistency with existing codebase patterns

## Usage Examples

### Synchronous Translation
```rust
let handler = PostTranslateHandler { db: arc_conn };
let request = TranslatePostRequest::new(post_id, "Vietnamese".to_string());
let result = handler.handle_translate_post(request, openai_api_key).await?;
```

### Background Translation (NEW)
```rust
let handler = PostTranslateHandler { db: arc_conn };
let request = TranslatePostRequest::new(post_id, "Vietnamese".to_string());
let translation_id = handler
    .handle_translate_post_background(request, openai_api_key)
    .await?;
// Returns immediately, translation happens in background
```

## Performance

- **Small content** (< 2000 chars): Single API call, fast response
- **Large content** (> 2000 chars): Multiple parallel API calls
  - Example: 10,000 char content → 5 chunks → ~5 parallel API calls
  - Processing time: similar to single API call (due to parallelism)
- **Background mode**: Non-blocking, ideal for batch operations

## Testing
- All existing tests pass (14 passed)
- Integration test included (marked as `#[ignore]` - requires API key)
- No warnings or errors in build

## Security Considerations
✅ API key passed as parameter (not hardcoded)
✅ Input validation (post existence check)
✅ Proper error handling throughout
✅ Database operations use parameterized queries
✅ No SQL injection risks
✅ No exposed secrets
✅ **NEW**: Background tasks properly isolated with error logging

## Documentation
- Comprehensive README in `application_core/src/commands/ai/README.md`
- **Updated** with chunking and background processing documentation
- Inline comments explaining design decisions
- Usage examples and configuration instructions

## Design Decisions

1. **API Key as Parameter**: Rather than reading from environment directly, the handler accepts the API key as a parameter for better testability and flexibility

2. **GPT-4o-mini Model**: Chosen for cost-effectiveness while maintaining good translation quality

3. **Empty String for Null Preview**: When original preview_content is None, we use an empty string because the database schema requires a non-nullable String

4. **Configurable Model**: Model name defined as a constant for easy updates without code changes

5. **Sentence-Based Chunking (NEW)**: Splits at sentence boundaries (. ! ?) to maintain context and avoid cutting mid-sentence

6. **Parallel Processing (NEW)**: Uses tokio for concurrent chunk processing to minimize total translation time

7. **Background Mode (NEW)**: Separate method for background processing to support both blocking and non-blocking use cases

## Addressing User Feedback

✅ **"the context for translation too large"**
   - Implemented automatic content chunking with configurable size (2000 chars)
   - Handles arbitrarily large content without hitting token limits

✅ **"make the system does in background"**
   - Added `handle_translate_post_background` method
   - Uses `tokio::spawn` for non-blocking execution
   - Includes logging for monitoring background tasks

✅ **"tokenized the content to make multiple translation processes"**
   - Smart chunking at sentence boundaries for plain text
   - **HTML-aware chunking at block-level element boundaries**
   - Parallel processing using `JoinSet`
   - Maintains order and context across chunks

✅ **"Because the content is in html, so that if you just chunk the text in content, may be you translate wrong"** (NEW)
   - Implemented HTML detection and parsing using `html5ever`
   - Chunks at safe boundaries (block-level elements like `</p>`, `</div>`)
   - Never breaks HTML tags or attributes
   - Specialized translation prompts preserve HTML structure
   - Falls back gracefully for edge cases

## Future Enhancements (Optional)
- Add batch translation support for multiple posts
- Cache translations to reduce API calls
- Support for custom translation prompts
- Translation quality validation
- Retry logic for API failures
- Progress tracking for background translations
- Webhook notifications on completion
