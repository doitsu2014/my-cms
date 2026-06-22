use chrono::{DateTime, Utc};
use sea_orm::prelude::Uuid;
use serde::{Deserialize, Serialize};

pub const RECOGNISED_ROLES: &[&str] = &[
    "my-headless-cms-administrator",
    "my-headless-cms-writer",
];

pub const BAN_DURATION: &str = "876000h";

#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct AppUserModel {
    pub id: Uuid,
    pub email: String,
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
    RECOGNISED_ROLES.iter().any(|r| *r == role)
}
