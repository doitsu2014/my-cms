use serde::{Deserialize, Serialize};

use crate::entities::tags::Model;

#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct GetAndClassifyTagCommandResponse {
    pub new_tags: Vec<Model>,
    pub existing_tags: Vec<Model>,
}
