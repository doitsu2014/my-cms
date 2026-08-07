//! `dto` — user-domain data transfer objects and validation helpers.
//!
//! Moved wholesale from `application_core::commands::user::dto` per design
//! Decision 1 of `split-media-and-user-domains-merge-tags-into-posts`. The
//! `AppError` import has been retargeted to the canonical
//! `crate::domain::error::AppError` so this file no longer depends on the
//! legacy `application_core::common::app_error` shim.

use chrono::{DateTime, Utc};
use sea_orm::prelude::Uuid;
use serde::{Deserialize, Serialize};

use crate::domain::error::AppError;

pub const RECOGNISED_ROLES: &[&str] = &["my-headless-cms-administrator", "my-headless-cms-writer"];

pub const BAN_DURATION: &str = "876000h";

#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct AppUserModel {
    pub id: Uuid,
    pub email: String,
    pub full_name: Option<String>,
    pub phone: Option<String>,
    pub role: Option<String>,
    pub banned: bool,
    pub created_at: DateTime<Utc>,
    pub updated_at: DateTime<Utc>,
    pub last_sign_in_at: Option<DateTime<Utc>>,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct CreateUserResponse {
    pub user: AppUserModel,
    pub temporary_password: String,
}

pub fn sanitise_email(email: &str) -> String {
    email.trim().to_lowercase()
}

pub fn is_recognised_role(role: &str) -> bool {
    RECOGNISED_ROLES.contains(&role)
}

pub const FULL_NAME_MAX_LEN: usize = 120;
const PHONE_PATTERN: &str = r"^\+[1-9]\d{6,14}$";

pub fn validate_full_name(value: &str) -> Result<(), AppError> {
    if value.chars().count() > FULL_NAME_MAX_LEN {
        return Err(AppError::Validation(
            "fullName".to_string(),
            format!(
                "Full name must be {} characters or fewer",
                FULL_NAME_MAX_LEN
            ),
        ));
    }
    Ok(())
}

pub fn validate_phone(value: &str) -> Result<(), AppError> {
    let re = regex::Regex::new(PHONE_PATTERN).expect("PHONE_PATTERN must compile");
    if !re.is_match(value) {
        return Err(AppError::Validation(
            "phone".to_string(),
            "Phone must be in E.164 format (e.g. +14155550100)".to_string(),
        ));
    }
    Ok(())
}
