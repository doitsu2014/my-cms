# AI Commands

This module contains AI-powered business services for the CMS.

## Translate Service

The translate service uses OpenAI's GPT-4o-mini model to automatically translate post content to any language. It supports both synchronous and background processing, with **HTML-aware content chunking** for large texts.

### Features

- **HTML-Aware Chunking**: Intelligently handles HTML content by preserving structure and tags
  - Automatically detects HTML content
  - Splits at block-level element boundaries (e.g., `</p>`, `</div>`, `</section>`)
  - Never breaks HTML tags or attributes
  - Instructs translator to preserve HTML structure
- **Plain Text Chunking**: Falls back to sentence-based chunking for non-HTML content (max 2000 characters)
- **Parallel Processing**: Translates multiple chunks concurrently for faster processing
- **Background Processing**: Option to run translations in background without blocking the main thread
- **Automatic Slug Generation**: Creates URL-friendly slugs for translated content
- **Database Persistence**: Saves translations to the `post_translations` table

### Usage

#### Synchronous Translation (Waits for completion)

```rust
use application_core::commands::ai::translate::{
    translate_handler::{PostTranslateHandler, PostTranslateHandlerTrait},
    translate_request::TranslatePostRequest,
};
use std::sync::Arc;

// Initialize the handler with database connection
let handler = PostTranslateHandler {
    db: Arc::new(database_connection),
};

// Create a translation request
let request = TranslatePostRequest::new(
    post_id,
    "Vietnamese".to_string(), // or "VI", "Spanish", "French", etc.
);

// Translate the post
let result = handler
    .handle_translate_post(request, openai_api_key)
    .await?;

// The result contains:
// - post_translation_id: UUID of the created translation
// - translated_title: Translated post title
// - translated_preview_content: Translated preview content
// - translated_content: Translated post content (with HTML preserved)
```

#### Background Translation (Returns immediately)

For large content or when you don't want to block the caller:

```rust
// Translate in background
let translation_id = handler
    .handle_translate_post_background(request, openai_api_key)
    .await?;

// Returns immediately with the translation ID
// Translation happens in background
// Check logs for completion status
```

### Configuration

Set the `OPENAI_API_KEY` environment variable in your `.env` file:

```
OPENAI_API_KEY=sk-...
```

### HTML Content Handling

The service automatically detects HTML content and uses specialized chunking:

1. **Detection**: Checks for common HTML tags (`<p>`, `<div>`, `<h*>`, etc.)
2. **HTML Parsing**: Uses `html5ever` to parse and understand HTML structure
3. **Smart Chunking**: 
   - Attempts to split at block-level element boundaries
   - Falls back to tag-based splitting if needed
   - Preserves all HTML tags, attributes, and structure
4. **Translation Prompt**: Instructs OpenAI to:
   - Only translate text content within tags
   - Never translate HTML tag names or attributes
   - Preserve all HTML structure exactly
5. **Reassembly**: Combines translated chunks without separators to maintain HTML integrity

### Content Chunking Examples

#### HTML Content
```html
<!-- Input -->
<p>First paragraph.</p><div>Second section.</div>

<!-- Chunks -->
Chunk 1: <p>First paragraph.</p>
Chunk 2: <div>Second section.</div>

<!-- Each chunk is translated while preserving HTML -->
```

#### Plain Text Content
```text
First sentence. Second sentence. Third sentence.

Chunks at sentence boundaries if too long.
```

### Performance Considerations

- **Small content** (< 2000 chars): Single API call, fast response
- **Large HTML** (> 2000 chars): Multiple parallel API calls at element boundaries
- **Large plain text** (> 2000 chars): Multiple parallel API calls at sentence boundaries
- **Background mode**: Non-blocking, ideal for batch operations or user-triggered actions

### Testing

The integration test is marked with `#[ignore]` by default because it requires a valid OpenAI API key. To run the test:

1. Set your OpenAI API key in the environment
2. Run: `OPENAI_API_KEY=sk-... cargo test -- --ignored --test-threads=1`

### Error Handling

The handler returns `AppError::OpenAIError` for any OpenAI API issues, including:
- Invalid API key
- Rate limiting
- Network errors
- Empty responses
- Task join errors

### Logging

Background translations log their status:
- Success: `info` level with post_id and language
- Failure: `error` level with post_id, language, and error details
