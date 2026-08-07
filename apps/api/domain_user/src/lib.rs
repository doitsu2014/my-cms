//! `domain_user` — self-contained user domain service.
//!
//! Owns user CRUD handlers, password reset, user DTOs (`AppUserModel`,
//! `CreateUserResponse`, `ModifyUserRequest`, `ResetPasswordRequest`,
//! `ResetPasswordResponse`), the `SupabaseAdminClient` adapter, and the
//! user-related validation helpers (`is_recognised_role`,
//! `sanitise_email`, `validate_full_name`, `validate_phone`,
//! `BAN_DURATION`, `RECOGNISED_ROLES`).
//!
//! See `openspec/changes/split-media-and-user-domains-merge-tags-into-posts/design.md`
//! for the architectural context.

#![deny(unsafe_code)]
#![warn(missing_debug_implementations)]

pub mod domain;
pub mod dto;
pub mod handlers;
pub mod observability;

pub use domain::error::AppError;
