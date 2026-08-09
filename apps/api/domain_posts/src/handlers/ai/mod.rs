//! AI subsystem for the post domain.
//!
//! Owns the OpenAI model registry and the OpenAI client factory used by the
//! post translation pipeline. Folded into `domain_posts` per the
//! `consolidate-category-ai-translate-into-domain-posts` change (Decision 2).

pub mod models;
pub mod openai_client_from_env;

pub use models::{ModelsHandler, ModelsHandlerTrait, ModelsListResponse, OpenAIModelInfo};
