use serde::{Deserialize, Serialize};

#[derive(Debug, Clone, Deserialize, Serialize)]
#[serde(rename_all = "camelCase")]
pub struct UpdateBucketRequest {
    pub public: Option<bool>,
    pub file_size_limit: Option<Option<u64>>,
    pub allowed_mime_types: Option<Option<Vec<String>>>,
}
