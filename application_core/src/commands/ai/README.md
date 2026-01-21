# AI Commands

This module contains AI-powered business services for the CMS.

## Translate Service

The translate service uses OpenAI's GPT-4o-mini model to automatically translate post content to any language.

### Usage

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

### Configuration

Set the `OPENAI_API_KEY` environment variable in your `.env` file:

```
OPENAI_API_KEY=sk-...
```

### Features

- Uses OpenAI GPT-4o-mini model for cost-effective translations
- Translates title, preview content, and main content
- Automatically generates slugs for translated content
- Saves translations to the `post_translations` table
- Maintains relationship with the original post

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
