//! `AppError` — the single error type returned by every handler in the
//! post domain. Re-exported from `application_core::common::app_error` for
//! compatibility during the transition; will own its own definition once
//! the post handlers physically move into `domain_posts`.

pub use application_core::common::app_error::AppError;