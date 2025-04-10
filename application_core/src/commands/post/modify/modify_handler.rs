use sea_orm::{
    sea_query::Expr, DatabaseConnection, EntityTrait, QueryFilter, Set, TransactionTrait,
};
use seaography::itertools::Itertools;
use std::sync::Arc;
use tracing::instrument;
use uuid::Uuid;

use crate::{
    commands::{
        post::read::read_handler::{PostReadHandler, PostReadHandlerTrait, PostReadResponse},
        tag::create::create_handler::{TagCreateHandler, TagCreateHandlerTrait},
    },
    common::{app_error::AppError, datetime_generator::generate_vietnam_now},
    entities::{
        post_tags,
        posts::{self, Column},
        post_translations,
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
mod tests {
    use std::sync::Arc;
    use test_helpers::{setup_test_space, ContainerAsyncPostgresEx};

    use crate::{
        commands::{
            category::{
                create::create_handler::{CategoryCreateHandler, CategoryCreateHandlerTrait},
                test::fake_create_category_request,
            },
            post::{
                create::create_handler::{PostCreateHandler, PostCreateHandlerTrait},
                modify::{
                    modify_handler::{PostModifyHandler, PostModifyHandlerTrait},
                    modify_request::ModifyPostRequest,
                },
                read::read_handler::{PostReadHandler, PostReadHandlerTrait},
                test::fake_create_post_request,
            },
        },
        StringExtension,
    };

    #[async_std::test]
    async fn handle_modify_post_testcase_successfully() {
        let beginning_test_timestamp = chrono::Utc::now();
        let test_space = setup_test_space().await;
        let database = test_space.postgres.get_database_connection().await;

        let arc_conn = Arc::new(database.clone());

        let category_create_handler = CategoryCreateHandler {
            db: arc_conn.clone(),
        };
        let post_create_handler = PostCreateHandler {
            db: arc_conn.clone(),
        };
        let post_modify_handler = PostModifyHandler {
            db: arc_conn.clone(),
        };
        let post_read_handler = PostReadHandler {
            db: arc_conn.clone(),
        };

        let create_category_request = fake_create_category_request(3);
        let created_category_id = category_create_handler
            .handle_create_category_with_tags(create_category_request, None)
            .await
            .unwrap();
        let create_post_request = fake_create_post_request(created_category_id, 5);
        let result = post_create_handler
            .handle_create_post(create_post_request.clone(), None)
            .await
            .unwrap();

        let updated_title = format!("{} Updated", create_post_request.title);
        let updated_content = format!("{} Updated", create_post_request.content);
        let request = ModifyPostRequest {
            id: result,
            title: updated_title.to_owned(),
            content: updated_content.to_owned(),
            preview_content: None,
            published: true,
            category_id: created_category_id,
            row_version: 1,
            tag_names: None,
            thumbnail_paths: vec![],
            translations: None,
        };
        let result = post_modify_handler
            .handle_modify_post(request.clone(), Some("Last Modifier".to_string()))
            .await
            .unwrap();
        let posts_in_db = post_read_handler.handle_get_all_posts().await.unwrap();
        let first = posts_in_db.first().unwrap();

        assert_eq!(result, first.id);
        assert!(first.created_at >= beginning_test_timestamp);
        assert!(first.row_version == 2);
        assert!(first.title == request.title);
        assert!(first.slug == request.title.to_slug());
        assert!(first.content == request.content);
        assert!(first.created_by == "System");
        assert!(first.last_modified_by == Some("Last Modifier".to_string()));
    }

    #[async_std::test]
    async fn handle_modify_post_testcase_failed() {
        let test_space = setup_test_space().await;
        let conn = test_space.postgres.get_database_connection().await;

        let arc_conn = Arc::new(conn.clone());
        let category_create_handler = CategoryCreateHandler {
            db: arc_conn.clone(),
        };
        let post_create_handler = PostCreateHandler {
            db: arc_conn.clone(),
        };
        let post_modify_handler = PostModifyHandler {
            db: arc_conn.clone(),
        };

        let create_category_request = fake_create_category_request(3);
        let created_category_id = category_create_handler
            .handle_create_category_with_tags(create_category_request, None)
            .await
            .unwrap();
        let create_post_request = fake_create_post_request(created_category_id, 5);
        let result = post_create_handler
            .handle_create_post(create_post_request.clone(), None)
            .await
            .unwrap();
        let updated_title = format!("{} Updated", create_post_request.title);
        let request = ModifyPostRequest {
            id: result,
            title: format!("{} Updated", updated_title),
            content: format!("{} Updated", create_post_request.content),
            preview_content: None,
            published: true,
            category_id: created_category_id,
            row_version: 0,
            tag_names: None,
            thumbnail_paths: vec![],
            translations: None,
        };

        let result = post_modify_handler
            .handle_modify_post(request, Some("System".to_string()))
            .await;
        assert!(result.is_err());
    }
}
