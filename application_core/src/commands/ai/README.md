# AI Commands

This module contains AI-powered business services for the CMS.

## Translate Service

The translate service uses OpenAI's GPT-4o-mini model to automatically translate post content to any language. It supports both synchronous and background processing, with automatic content chunking for large texts.

### Features

- **Content Chunking**: Automatically splits large content into chunks (max 2000 characters) to handle OpenAI token limits
- **Parallel Processing**: Translates multiple chunks concurrently for faster processing
- **Background Processing**: Option to run translations in background without blocking the main thread
- **Smart Chunking**: Splits at sentence boundaries to maintain context and readability
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
// - translated_content: Translated post content
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

### Content Chunking

For large content (> 2000 characters), the service automatically:
1. Splits content at sentence boundaries (periods, exclamation marks, question marks)
2. Processes chunks in parallel using tokio tasks
3. Reassembles translated chunks in correct order
4. Maintains context with specialized prompts for chunk translation

### Performance Considerations

- **Small content** (< 2000 chars): Single API call, fast response
- **Large content** (> 2000 chars): Multiple parallel API calls, proportionally longer
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
