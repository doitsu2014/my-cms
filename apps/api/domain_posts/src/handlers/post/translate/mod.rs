//! Translation pipeline handlers — `POST /posts/{post_id}/translate`,
//! `POST /posts/{post_id}/translate/background`, and the translation-job
//! lifecycle.
//!
//! Moved from `application_core::commands::ai::translate::*` per design
//! Decision 2 / Migration Plan step 4. The handlers continue to reference
//! the legacy `crate::entities::*` during the transition; the
//! entity set moves to `domain_posts::entities` in Task 4.6.

pub mod translate_handler;
pub mod translate_request;
pub mod translate_response;
pub mod translation_validator;

pub use translate_handler::{PostTranslateHandler, PostTranslateHandlerTrait};
pub use translate_request::TranslatePostRequest;
pub use translate_response::TranslatePostResponse;
pub use translation_validator::{count_paragraph_tags, validate_paragraph_coverage};
