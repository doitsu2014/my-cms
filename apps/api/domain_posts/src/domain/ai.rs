//! OpenAI client factory used by the post translation handler.
//!
//! The actual translation logic lives in `handlers::post::translate` (which
//! owns the prompt engineering, pgvector lookup, and translation chunking).
//! This module exposes the `OpenAIConfig`/`Client` factories and the
//! `OPENAI_API_KEY` env contract.

use async_openai::{config::OpenAIConfig, Client};

/// Build an OpenAI client from the env var. Returns `None` if the key is
/// missing — the caller decides whether that is a fatal configuration error.
pub fn openai_client_from_env() -> Option<Client<OpenAIConfig>> {
    std::env::var("OPENAI_API_KEY")
        .ok()
        .map(|key| Client::with_config(OpenAIConfig::new().with_api_key(key)))
}