use crate::entities::tags::{ActiveModel, Model};

pub struct GetAndClassifyTagCommandResponse {
    pub new_tags: Vec<ActiveModel>,
    pub existing_tags: Vec<Model>,
}
