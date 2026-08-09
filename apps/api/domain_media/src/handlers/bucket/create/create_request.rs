use serde::{Deserialize, Serialize};

#[derive(Debug, Clone, Deserialize, Serialize)]
#[serde(rename_all = "camelCase")]
pub struct CreateBucketRequest {
    pub name: String,
    pub public: Option<bool>,
    pub file_size_limit: Option<u64>,
    pub allowed_mime_types: Option<Vec<String>>,
}
