use sea_orm::{
    DatabaseConnection, EntityTrait, IntoActiveModel, TransactionError, TransactionTrait,
};
use std::sync::Arc;
use tracing::instrument;
use uuid::Uuid;

use crate::{
    commands::tag::create::create_handler::{TagCreateHandler, TagCreateHandlerTrait},
    common::{app_error::AppError, datetime_generator::generate_vietnam_now},
    entities::{post_tags, post_translations, posts},
    Posts,
};

use super::create_request::CreatePostRequest;

pub trait PostCreateHandlerTrait {
    fn handle_create_post(
        &self,
        body: CreatePostRequest,
        actor_email: Option<String>,
    ) -> impl std::future::Future<Output = Result<Uuid, AppError>>;
}

#[derive(Debug)]
pub struct PostCreateHandler {
    pub db: Arc<DatabaseConnection>,
}

impl PostCreateHandlerTrait for PostCreateHandler {
    #[instrument]
    async fn handle_create_post(
        &self,
        body: CreatePostRequest,
        actor_email: Option<String>,
    ) -> Result<Uuid, AppError> {
        let tag_create_handler = TagCreateHandler {
            db: self.db.clone(),
        };
        // Prepare Category
        let model: posts::Model = body.into_model();
        let model = posts::Model {
            created_by: actor_email.clone().unwrap_or("System".to_string()),
            created_at: generate_vietnam_now(),
            ..model
        };
        let create_model = posts::ActiveModel {
            ..model.into_active_model()
        };

        // Prepare Tags
        let tags: Vec<String> = body.tag_names.unwrap_or_default();

        // Prepare Translations
        let translations = body.translations.unwrap_or_default();

        let result: Result<Uuid, TransactionError<AppError>> = self
            .db
            .as_ref()
            .transaction::<_, Uuid, AppError>(|tx| {
                Box::pin(async move {
                    // Insert New Tags
                    let create_tags_response_task =
                        tag_create_handler.handle_create_tags_in_transaction(tags, actor_email, tx);

                    // Insert Category
                    let inserted_post = Posts::insert(create_model)
                        .exec(tx)
                        .await
                        .map_err(|e| e.into())?;

                    // Combine New Tag Ids and Existing Tag Ids
                    let create_tags_response = create_tags_response_task.await?;
                    let all_tag_ids = create_tags_response
                        .existing_tag_ids
                        .into_iter()
                        .chain(create_tags_response.new_tag_ids)
                        .collect::<Vec<Uuid>>();

                    // Insert Category Tags
                    if !all_tag_ids.is_empty() {
                        let post_tags = all_tag_ids
                            .iter()
                            .map(|tag_id| {
                                post_tags::Model {
                                    post_id: inserted_post.last_insert_id,
                                    tag_id: tag_id.to_owned(),
                                }
                                .into_active_model()
                            })
                            .collect::<Vec<post_tags::ActiveModel>>();

                        post_tags::Entity::insert_many(post_tags)
                            .exec(tx)
                            .await
                            .map_err(|e| e.into())?;
                    }

                    // Insert Post Translations
                    if !translations.is_empty() {
                        let post_translations = translations
                            .into_iter()
                            .map(|translation| {
                                post_translations::Model {
                                    post_id: inserted_post.last_insert_id,
                                    ..translation.into_model()
                                }
                                .into_active_model()
                            })
                            .collect::<Vec<post_translations::ActiveModel>>();

                        post_translations::Entity::insert_many(post_translations)
                            .exec(tx)
                            .await
                            .map_err(|e| e.into())?;
                    }

                    Ok(inserted_post.last_insert_id)
                })
            })
            .await;

        match result {
            Ok(inserted_id) => Ok(inserted_id),
            Err(e) => Err(e.into()),
        }
    }
}
