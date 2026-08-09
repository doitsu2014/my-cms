//! HTTP adapters for the category domain — `GET/POST/PUT/DELETE /categories`
//! and `GET /categories/{category_id}`.
//!
//! Moved from `apps/api/src/api/category/*` per the
//! `consolidate-category-ai-translate-into-domain-posts` change. Each
//! adapter is a thin Axum handler that extracts the `DomainContext`,
//! delegates to the corresponding `crate::handlers::category::*` command
//! handler, and returns the existing `ApiResponseWith` / `ApiResponseError`
//! envelope.

pub mod create;
pub mod delete;
pub mod modify;
pub mod read;
