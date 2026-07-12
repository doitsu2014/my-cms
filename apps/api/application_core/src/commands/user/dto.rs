use chrono::{DateTime, Utc};
use sea_orm::prelude::Uuid;
use serde::{Deserialize, Serialize};

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

pub fn validate_full_name(value: &str) -> Result<(), crate::common::app_error::AppError> {
    if value.chars().count() > FULL_NAME_MAX_LEN {
        return Err(crate::common::app_error::AppError::Validation(
            "fullName".to_string(),
            format!(
                "Full name must be {} characters or fewer",
                FULL_NAME_MAX_LEN
            ),
        ));
    }
    Ok(())
}

pub fn validate_phone(value: &str) -> Result<(), crate::common::app_error::AppError> {
    let re = regex::Regex::new(PHONE_PATTERN).expect("PHONE_PATTERN must compile");
    if !re.is_match(value) {
        return Err(crate::common::app_error::AppError::Validation(
            "phone".to_string(),
            "Phone must be in E.164 format (e.g. +14155550100)".to_string(),
        ));
    }
    Ok(())
}
