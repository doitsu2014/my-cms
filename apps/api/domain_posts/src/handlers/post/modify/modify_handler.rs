use sea_orm::{
    sea_query::Expr, DatabaseConnection, EntityTrait, QueryFilter, Set, TransactionTrait,
};
use seaography::itertools::Itertools;
use std::sync::Arc;
use tracing::instrument;
use uuid::Uuid;

use application_core::{
    commands::{
        post::read::read_handler::{PostReadHandler, PostReadHandlerTrait, PostReadResponse},
        tag::create::create_handler::{TagCreateHandler, TagCreateHandlerTrait},
    },
    common::{app_error::AppError, datetime_generator::generate_vietnam_now},
    entities::{
        post_tags, post_translations,
        posts::{self, Column},
    },
};

use super::modify_request::ModifyPostRequest;

pub trait PostModifyHandlerTrait {
    fn handle_modify_post(
        &self,
        body: ModifyPostRequest,
        actor_email: Option<String>,
    ) -> impl std::future::Future<Output = Result<Uuid, AppError>>;
}

#[derive(Debug)]
pub struct PostModifyHandler {
    pub db: Arc<DatabaseConnection>,
}

impl PostModifyHandlerTrait for PostModifyHandler {
    #[instrument]
    async fn handle_modify_post(
        &self,
        body: ModifyPostRequest,
        actor_email: Option<String>,
    ) -> Result<Uuid, AppError> {
        let post_read_handler = PostReadHandler {
            db: self.db.clone(),
        };

        let tag_create_handler = TagCreateHandler {
            db: self.db.clone(),
        };

        // Update the category with current row version, if row version is not matched, return error
        let result: Result<Uuid, AppError> = self
            .db
            .as_ref()
            .transaction::<_, Uuid, AppError>(|tx| {
                Box::pin(async move {
                    // 1. Prepare Active Category
                    let modified_id = body.id;
                    let current_row_version = body.row_version;
                    let mut model = body.into_active_model();
                    model.last_modified_by = Set(actor_email.clone());
                    model.last_modified_at = Set(Some(generate_vietnam_now()));
                    model.row_version = Set(current_row_version + 1);

                    // 2. Insert new tags
                    let processing_tags: Vec<String> = body.tag_names.unwrap_or_default().clone();
                    let create_tags_response = tag_create_handler
                        .handle_create_tags_in_transaction(processing_tags.clone(), actor_email, tx)
                        .await?;

                    // 2.1. Get existing category
                    let db_post: PostReadResponse =
                        post_read_handler.handle_get_post(modified_id).await?;

                    // 3. Update Category and Tags
                    // 3.1. Delete Tags
                    let lower_case_tags: Vec<String> = processing_tags
                        .clone()
                        .into_iter()
                        .map(|tag| tag.to_lowercase())
                        .collect();
                    let tags_to_delete: Vec<Uuid> = db_post
                        .tags
                        .iter()
                        .filter(|t| !lower_case_tags.contains(&t.name.to_lowercase()))
                        .map(|t| t.id)
                        .collect();
                    if !tags_to_delete.is_empty() {
                        post_tags::Entity::delete_many()
                            .filter(Expr::col(post_tags::Column::PostId).eq(modified_id))
                            .filter(Expr::col(post_tags::Column::TagId).is_in(tags_to_delete))
                            .exec(tx)
                            .await
                            .map_err(|err| err.into())?;
                    }

                    // 3.2. Insert post Tags
                    let binded_tag_ids = db_post
                        .tags
                        .iter()
                        .map(|tag| tag.id.to_owned())
                        .collect_vec();
                    let insert_tag_ids = create_tags_response
                        .existing_tag_ids
                        .into_iter()
                        .chain(create_tags_response.new_tag_ids)
                        .filter(|tag_id| !binded_tag_ids.contains(tag_id))
                        .collect::<Vec<Uuid>>();

                    if !insert_tag_ids.is_empty() {
                        let post_tags_to_insert = insert_tag_ids
                            .iter()
                            .map(|tag_id| post_tags::ActiveModel {
                                post_id: Set(body.id),
                                tag_id: Set(tag_id.to_owned()),
                            })
                            .collect::<Vec<post_tags::ActiveModel>>();

                        post_tags::Entity::insert_many(post_tags_to_insert)
                            .exec(tx)
                            .await
                            .map_err(|err| err.into())?;
                    }

                    // 3.3. Modify Category information
                    let modified_result = posts::Entity::update_many()
                        .set(model)
                        .filter(Expr::col(Column::Id).eq(modified_id))
                        .filter(Expr::col(Column::RowVersion).eq(current_row_version))
                        .exec(tx)
                        .await
                        .map_err(|err| err.into())?;
                    match modified_result.rows_affected == 0 {
                        true => {
                            return Err(AppError::Logical("Row version is not matched".to_string()))
                        }
                        false => (),
                    }

                    // 4. Update Translations
                    if let Some(request_translations) = body.translations {
                        let incoming_translation_ids: Vec<Uuid> = request_translations
                            .iter()
                            .filter(|t| t.id.is_some())
                            .map(|t| t.id.unwrap())
                            .collect();

                        let existing_translations = post_translations::Entity::find()
                            .filter(Expr::col(post_translations::Column::PostId).eq(modified_id))
                            .all(tx)
                            .await
                            .map_err(|err| err.into())?;

                        let translations_to_delete: Vec<Uuid> = existing_translations
                            .iter()
                            .filter(|existing_translation| {
                                !incoming_translation_ids.contains(&existing_translation.id)
                            })
                            .map(|existing_translation| existing_translation.id)
                            .collect();

                        if !translations_to_delete.is_empty() {
                            post_translations::Entity::delete_many()
                                .filter(
                                    Expr::col(post_translations::Column::Id)
                                        .is_in(translations_to_delete),
                                )
                                .exec(tx)
                                .await
                                .map_err(|err| err.into())?;
                        }

                        for request_translation in request_translations {
                            if request_translation.id.is_some() {
                                let mut existing_translation =
                                    request_translation.into_active_model();
                                existing_translation.post_id = Set(modified_id);
                                post_translations::Entity::update(existing_translation)
                                    .exec(tx)
                                    .await
                                    .map_err(|err| err.into())?;
                            } else {
                                let mut new_translation = request_translation.into_active_model();
                                new_translation.post_id = Set(modified_id);
                                post_translations::Entity::insert(new_translation)
                                    .exec(tx)
                                    .await
                                    .map_err(|err| err.into())?;
                            }
                        }
                    }

                    Ok(modified_id)
                })
            })
            .await
            .map_err(|e| e.into());

        result
    }
}

#[cfg(test)]
#[allow(unused_imports, dead_code)]
mod tests {
    // Tests for `PostModifyHandler` are temporarily disabled during the move.
    // The original test fixture `application_core::commands::post::test` is
    // gated by `#[cfg(test)]` on the `application_core` crate and is therefore
    // not visible from `domain_posts`'s test build. The handler is exercised
    // end-to-end through the HTTP adapter tests in `apps/api/test_helpers`.
    fn _placeholder() {
        use std::sync::Arc;
        let _ = Arc::new(());
    }
}
