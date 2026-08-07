//! `ResetPasswordRequest` / `ResetPasswordResponse` — request and response
//! payloads for the user password-reset command.

use serde::{Deserialize, Serialize};

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct ResetPasswordRequest {
    pub password: String,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct ResetPasswordResponse {
    pub temporary_password: String,
}
