use std::sync::Arc;

use super::create_request::CreateCategoryRequest;
use crate::{
    commands::tag::read::read_handler::{TagReadHandler, TagReadHandlerTrait},
    common::{
        app_error::{AppError, DbErrExt, TransactionDbErrExt},
        datetime_generator::generate_vietname_now,
    },
    entities::{categories, category_tags, tags},
    Categories, Tags,
};
use sea_orm::{
    DatabaseConnection, DbErr, EntityTrait, IntoActiveModel, Set, TransactionError,
    TransactionTrait,
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
        let tag_read_handler = TagReadHandler {
            db: Arc::clone(&self.db),
        };

        // Prepare Category
        let model: categories::Model = body.into_model();
        let model = categories::Model {
            created_by: actor_email.clone().unwrap_or("System".to_string()),
            created_at: generate_vietname_now(),
            ..model
        };
        let create_category = categories::ActiveModel {
            ..model.into_active_model()
        };
        // Prepare Tags
        let tags: Vec<String> = body.tag_names.unwrap_or_default();
        let classifed_tags = tag_read_handler
            .handle_get_and_classify_tags_by_names(tags)
            .await
            .map_err(|e| e.to_app_error())?;

        let existing_tag_ids = classifed_tags
            .existing_tags
            .iter()
            .map(|tag| tag.id)
            .collect::<Vec<Uuid>>();

        let result: Result<Uuid, TransactionError<DbErr>> = self
            .db
            .as_ref()
            .transaction::<_, Uuid, DbErr>(|tx| {
                Box::pin(async move {
                    // Insert Category
                    let inserted_category = Categories::insert(create_category).exec(tx).await?;

                    // TODO: Move logic new tags to tag commands
                    // Insert New Tags
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
                                    created_by: Set(actor_email
                                        .clone()
                                        .unwrap_or("System".to_string())),
                                    created_at: Set(generate_vietname_now()),
                                    ..Default::default()
                                }
                            })
                            .collect::<Vec<tags::ActiveModel>>();
                        Tags::insert_many(new_tags).exec(tx).await?;
                    }

                    // Combine New Tag Ids and Existing Tag Ids
                    let all_tags = existing_tag_ids
                        .into_iter()
                        .chain(new_tag_ids.into_iter())
                        .collect::<Vec<Uuid>>();

                    // Insert Category Tags
                    if !all_tags.is_empty() {
                        let category_tags = all_tags
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
                            .await?;
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
