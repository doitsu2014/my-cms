use serde::{Deserialize, Serialize};

#[derive(Debug, Clone, Serialize, Deserialize, Default)]
#[serde(rename_all = "camelCase")]
pub struct ModifyUserRequest {
    pub email: Option<String>,
    pub role: Option<String>,
    pub banned: Option<bool>,
}
