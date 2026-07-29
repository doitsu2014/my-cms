//! HTTP adapters for the post domain.
//!
//! These are thin Axum handlers that extract state, call the corresponding
//! application-layer command handler in `domain_posts::handlers::post::*`,
//! and return the existing `ApiResponseWith` / `ApiResponseError` envelope.
//!
//! During the transition the post handlers live under
//! `application_core::commands::post::*` and `application_core::commands::ai::translate::*`;
//! `domain_posts::handlers` re-exports them so the adapters below do not
//! have to depend on `application_core` directly.

pub mod post;
pub mod administrator;