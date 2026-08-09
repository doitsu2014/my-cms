//! OpenAI client factory used by the post translation pipeline.
//!
//! Moved from `domain_posts::domain::ai` per Decision 2 of the
//! `consolidate-category-ai-translate-into-domain-posts` change. The
//! factory is co-located with the model registry so the AI subsystem is
//! entirely owned by `domain_posts::handlers::ai::*`.

use async_openai::{config::OpenAIConfig, Client};

/// Build an OpenAI client from the env var. Returns `None` if the key is
/// missing — the caller decides whether that is a fatal configuration error.
pub fn openai_client_from_env() -> Option<Client<OpenAIConfig>> {
    std::env::var("OPENAI_API_KEY")
        .ok()
        .map(|key| Client::with_config(OpenAIConfig::new().with_api_key(key)))
}
