use std::sync::Arc;

use super::create_request::CreateCategoryRequest;
use crate::{
    commands::tag::create::create_handler::{TagCreateHandler, TagCreateHandlerTrait},
    common::{app_error::AppError, datetime_generator::generate_vietnam_now},
    entities::{categories, category_tags},
    Categories,
};
use sea_orm::{
    DatabaseConnection, EntityTrait, IntoActiveModel, TransactionError, TransactionTrait,
};
use tracing::instrument;
use uuid::Uuid;

pub trait CategoryCreateHandlerTrait {
    fn handle_create_category_with_tags(
        &self,
        body: CreateCategoryRequest,
        actor_email: Option<String>,
    ) -> impl std::future::Future<Output = Result<Uuid, AppError>>;
}

#[derive(Debug)]
pub struct CategoryCreateHandler {
    pub db: Arc<DatabaseConnection>,
}

impl CategoryCreateHandlerTrait for CategoryCreateHandler {
    #[instrument]
    async fn handle_create_category_with_tags(
        &self,
        body: CreateCategoryRequest,
        actor_email: Option<String>,
    ) -> Result<Uuid, AppError> {
        let tag_create_handler = TagCreateHandler {
            db: self.db.clone(),
        };
        // Prepare Category
        let model: categories::Model = body.into_model();
        let model = categories::Model {
            created_by: actor_email.clone().unwrap_or("System".to_string()),
            created_at: generate_vietnam_now(),
            ..model
        };
        let create_category = categories::ActiveModel {
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
                    let create_tags_response = tag_create_handler
                        .handle_create_tags_in_transaction(tags, actor_email, tx)
                        .await?;

                    // Combine New Tag Ids and Existing Tag Ids
                    let all_tag_ids = create_tags_response
                        .existing_tag_ids
                        .into_iter()
                        .chain(create_tags_response.new_tag_ids)
                        .collect::<Vec<Uuid>>();

                    // Insert Category
                    let inserted_category = Categories::insert(create_category)
                        .exec(tx)
                        .await
                        .map_err(|e| e.into())?;

                    // Insert Category Tags
                    if !all_tag_ids.is_empty() {
                        let category_tags = all_tag_ids
                            .iter()
                            .map(|tag_id| {
                                category_tags::Model {
                                    category_id: inserted_category.last_insert_id,
                                    tag_id: tag_id.to_owned(),
                                }
                                .into_active_model()
                            })
                            .collect::<Vec<category_tags::ActiveModel>>();

                        category_tags::Entity::insert_many(category_tags)
                            .exec(tx)
                            .await
                            .map_err(|e| e.into())?;
                    }

                    Ok(inserted_category.last_insert_id)
                })
            })
            .await;

        match result {
            Ok(inserted_id) => Ok(inserted_id),
            Err(e) => Err(e.into()),
        }
    }
}

#[cfg(test)]
pub mod tests {
    use std::sync::Arc;
    use test_helpers::{setup_test_space, ContainerAsyncPostgresEx};

    use crate::commands::category::{
        create::create_handler::CategoryCreateHandlerTrait,
        read::category_read_handler::{CategoryReadHandler, CategoryReadHandlerTrait},
        test::{fake_create_category_request, fake_create_category_request_as_child},
    };

    #[async_std::test]
    async fn handle_create_cartegory_testcase_01() {
        let beginning_test_timestamp = chrono::Utc::now();
        let test_space = setup_test_space().await;
        let database = test_space.postgres.get_database_connection().await;
        let number_of_tags = 5;
        let request = fake_create_category_request(number_of_tags);

        let create_handler = super::CategoryCreateHandler {
            db: Arc::new(database.clone()),
        };
        let read_handler = CategoryReadHandler {
            db: Arc::new(database),
        };

        let result = create_handler
            .handle_create_category_with_tags(request, None)
            .await
            .unwrap();

        assert!(!result.is_nil());

        let category_in_db = read_handler.handle_get_all_categories().await.unwrap();
        let first = &category_in_db.first().unwrap();
        assert_eq!(result, first.id);
        assert!(first.created_by == "System");
        assert!(first.created_at >= beginning_test_timestamp);
        assert!(first.row_version == 1);
        assert!(first.tags.len() == number_of_tags);
    }

    #[async_std::test]
    async fn handle_create_cartegory_testcase_parent() {
        let test_space = setup_test_space().await;
        let conn = test_space.postgres.get_database_connection().await;
        let number_of_tags = 5;

        let create_handler = super::CategoryCreateHandler {
            db: Arc::new(conn.clone()),
        };
        let read_handler = CategoryReadHandler { db: Arc::new(conn) };
        let parent_request = fake_create_category_request(number_of_tags);
        let parent_id = create_handler
            .handle_create_category_with_tags(parent_request, None)
            .await
            .unwrap();
        let child_request = fake_create_category_request_as_child(parent_id, number_of_tags);
        let child = create_handler
            .handle_create_category_with_tags(child_request, None)
            .await
            .unwrap();
        let categories_in_db = read_handler.handle_get_all_categories().await.unwrap();
        let first = categories_in_db
            .iter()
            .find(|x| x.parent_id.is_none())
            .unwrap();
        let child_instance = categories_in_db.iter().find(|x| x.id == child).unwrap();
        assert_eq!(child_instance.parent_id.unwrap(), first.id);
    }
}
