use std::{future::Future, sync::Arc};

use sea_orm::{DatabaseConnection, DatabaseTransaction, EntityTrait, Set};
use tracing::instrument;
use uuid::Uuid;

use crate::{
    commands::tag::read::read_handler::{TagReadHandler, TagReadHandlerTrait},
    common::{
        app_error::{AppError, AppErrorExt},
        datetime_generator::generate_vietname_now,
    },
    entities::tags,
    Tags,
};

pub struct CreateTagsResponse {
    pub new_tag_ids: Vec<Uuid>,
    pub existing_tag_ids: Vec<Uuid>,
}

pub trait TagCreateHandlerTrait {
    fn handle_create_tags_in_transaction(
        &self,
        tags: Vec<String>,
        actor_email: Option<String>,
        transaction: &DatabaseTransaction,
    ) -> impl Future<Output = Result<CreateTagsResponse, AppError>> + Send;
}

#[derive(Debug)]
pub struct TagCreateHandler {
    pub db: Arc<DatabaseConnection>,
}

impl TagCreateHandlerTrait for TagCreateHandler {
    #[instrument]
    async fn handle_create_tags_in_transaction(
        &self,
        tags: Vec<String>,
        actor_email: Option<String>,
        transaction: &DatabaseTransaction,
    ) -> Result<CreateTagsResponse, AppError> {
        let tag_read_handler = TagReadHandler {
            db: self.db.clone(),
        };

        let classifed_tags = tag_read_handler
            .handle_get_and_classify_tags_by_names(tags)
            .await
            .map_err(|e| e.to_app_error())?;

        let mut new_tag_ids: Vec<Uuid> = vec![];
        if !classifed_tags.new_tags.is_empty() {
            let new_tags = classifed_tags
                .new_tags
                .iter()
                .map(|tag| {
                    let id = Uuid::new_v4();
                    new_tag_ids.push(id);
                    tags::ActiveModel {
                        id: Set(id),
                        name: tag.name.clone(),
                        slug: tag.slug.clone(),
                        created_by: Set(actor_email.clone().unwrap_or("System".to_string())),
                        created_at: Set(generate_vietname_now()),
                        ..Default::default()
                    }
                })
                .collect::<Vec<tags::ActiveModel>>();

            Tags::insert_many(new_tags)
                .exec(transaction)
                .await
                .map_err(|e| e.to_app_error())?;
        }

        let existing_tag_ids = classifed_tags
            .existing_tags
            .iter()
            .map(|tag| tag.id)
            .collect::<Vec<Uuid>>();

        Ok(CreateTagsResponse {
            new_tag_ids,
            existing_tag_ids,
        })
    }
}
