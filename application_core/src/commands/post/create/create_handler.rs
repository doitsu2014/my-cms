use sea_orm::{
    DatabaseConnection, EntityTrait, IntoActiveModel, TransactionError, TransactionTrait,
};
use std::sync::Arc;
use tracing::instrument;
use uuid::Uuid;

use crate::{
    commands::tag::create::create_handler::{TagCreateHandler, TagCreateHandlerTrait},
    common::{
        app_error::{AppError, AppErrorExt},
        datetime_generator::generate_vietname_now,
    },
    entities::{post_tags, posts},
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
            created_at: generate_vietname_now(),
            ..model
        };
        let create_model = posts::ActiveModel {
            ..model.into_active_model()
        };
        // Prepare Tags
        let tags: Vec<String> = body.tag_names.unwrap_or_default();

        let result: Result<Uuid, TransactionError<AppError>> = self
            .db
            .as_ref()
            .transaction::<_, Uuid, AppError>(|tx| {
                Box::pin(async move {
                    // Insert New Tags
                    let create_tags_response_task =
                        tag_create_handler.handle_create_tags_in_transaction(tags, actor_email, tx);

                    // Insert Category
                    let inserted_category = Posts::insert(create_model)
                        .exec(tx)
                        .await
                        .map_err(|e| e.to_app_error())?;

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
                                    post_id: inserted_category.last_insert_id,
                                    tag_id: tag_id.to_owned(),
                                }
                                .into_active_model()
                            })
                            .collect::<Vec<post_tags::ActiveModel>>();

                        post_tags::Entity::insert_many(post_tags)
                            .exec(tx)
                            .await
                            .map_err(|e| e.to_app_error())?;
                    }

                    Ok(inserted_category.last_insert_id)
                })
            })
            .await;

        match result {
            Ok(inserted_id) => Ok(inserted_id),
            Err(e) => Err(e.to_app_error()),
        }
    }
}

#[cfg(test)]
mod tests {
    use std::sync::Arc;
    use test_helpers::{setup_test_space, ContainerAsyncPostgresEx};

    use crate::commands::{
        category::{
            create::create_handler::{CategoryCreateHandler, CategoryCreateHandlerTrait},
            test::fake_create_category_request,
        },
        post::{
            create::create_handler::{PostCreateHandler, PostCreateHandlerTrait},
            read::read_handler::{PostReadHandler, PostReadHandlerTrait},
            test::fake_create_post_request,
        },
    };

    #[async_std::test]
    async fn handle_create_post_testcase_successfully() {
        let beginning_test_timestamp = chrono::Utc::now();
        let test_space = setup_test_space().await;
        let database = test_space.postgres.get_database_connection().await;

        let arc_conn = Arc::new(database);
        let category_create_handler = CategoryCreateHandler {
            db: arc_conn.clone(),
        };
        let post_create_handler = PostCreateHandler {
            db: arc_conn.clone(),
        };
        let post_read_handler = PostReadHandler {
            db: arc_conn.clone(),
        };
        let create_category_request = fake_create_category_request(5);
        let created_category_id = category_create_handler
            .handle_create_category_with_tags(create_category_request, None)
            .await
            .unwrap();
        let create_post_request = fake_create_post_request(created_category_id, 5);
        let result = post_create_handler
            .handle_create_post(create_post_request, None)
            .await
            .unwrap();

        let db_posts = post_read_handler.handle_get_all_posts().await.unwrap();
        let first = db_posts.first().unwrap();

        assert_eq!(result, first.id);
        assert!(first.created_by == "System");
        assert!(first.created_at >= beginning_test_timestamp);
        assert!(first.row_version == 1);
        assert!(first.tags.len() == 5);
    }
}
