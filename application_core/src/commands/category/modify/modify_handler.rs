use std::sync::Arc;

use crate::{
    commands::{
        category::read::category_read_handler::{
            self, CategoryReadHandlerTrait, CategoryReadResponse,
        },
        tag::create::create_handler::{TagCreateHandler, TagCreateHandlerTrait},
    },
    common::app_error::{AppError, DbErrExt, TransactionDbErrExt},
    entities::{
        categories::{self, Column},
        category_tags,
    },
};
use sea_orm::{
    prelude::Uuid, sea_query::Expr, DatabaseConnection, EntityTrait, QueryFilter, Set,
    TransactionTrait,
};
use tracing::instrument;

use super::modify_request::ModifyCategoryRequest;

pub trait CategoryModifyHandlerTrait {
    fn handle_modify_category(
        &self,
        body: ModifyCategoryRequest,
        actor_email: Option<String>,
    ) -> impl std::future::Future<Output = Result<Uuid, AppError>>;
}

#[derive(Debug)]
pub struct CategoryModifyHandler {
    pub db: Arc<DatabaseConnection>,
}

impl CategoryModifyHandlerTrait for CategoryModifyHandler {
    #[instrument]
    async fn handle_modify_category(
        &self,
        body: ModifyCategoryRequest,
        actor_email: Option<String>,
    ) -> Result<Uuid, AppError> {
        let category_read_handler = category_read_handler::CategoryReadHandler {
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

                    // 2. Insert new tags
                    let tags: Vec<String> = body.tag_names.unwrap_or_default().clone();
                    let create_tags_response = tag_create_handler
                        .handle_create_tags_in_transaction(tags.clone(), actor_email, tx)
                        .await?;
                    // Combine New Tag Ids and Existing Tag Ids
                    let new_tag_ids = create_tags_response.new_tag_ids;

                    // 2.1. Get existing category
                    let db_category: CategoryReadResponse = category_read_handler
                        .handle_get_category(modified_id)
                        .await
                        .map_err(|err| err.to_app_error())?;

                    // 2.2 Figure out tags to delete
                    let tags_to_delete: Vec<Uuid> = db_category
                        .tags
                        .iter()
                        .filter(|t| !tags.contains(&t.name))
                        .map(|t| t.id)
                        .collect();

                    // 3. Update Category and Tags
                    // 3.1. Delete and Insert Tags
                    if !tags_to_delete.is_empty() {
                        category_tags::Entity::delete_many()
                            .filter(Expr::col(category_tags::Column::CategoryId).eq(modified_id))
                            .filter(Expr::col(category_tags::Column::TagId).is_in(tags_to_delete))
                            .exec(tx)
                            .await
                            .map_err(|err| err.to_app_error())?;
                    }
                    // 3.2. Insert Category Tags
                    if !new_tag_ids.is_empty() {
                        let category_tags_to_insert = new_tag_ids
                            .iter()
                            .map(|tag_id| category_tags::ActiveModel {
                                category_id: Set(body.id),
                                tag_id: Set(tag_id.to_owned()),
                            })
                            .collect::<Vec<category_tags::ActiveModel>>();

                        category_tags::Entity::insert_many(category_tags_to_insert)
                            .exec(tx)
                            .await
                            .map_err(|err| err.to_app_error())?;
                    }

                    // 3.3. Modify Category information
                    let modified_result = categories::Entity::update_many()
                        .set(model)
                        .filter(Expr::col(Column::Id).eq(modified_id))
                        .filter(Expr::col(Column::RowVersion).eq(current_row_version))
                        .exec(tx)
                        .await
                        .map_err(|err| err.to_app_error())?;

                    match modified_result.rows_affected == 0 {
                        true => {
                            return Err(AppError::Logical("Row version is not matched".to_string()))
                        }
                        false => (),
                    }

                    Ok(modified_id)
                })
            })
            .await
            .map_err(|e| e.to_app_error());

        result
    }
}

#[cfg(test)]
mod tests {
    use std::sync::Arc;
    use test_helpers::{setup_test_space, ContainerAsyncPostgresEx};

    use crate::{
        commands::category::{
            create::create_handler::{CategoryCreateHandler, CategoryCreateHandlerTrait},
            modify::{
                modify_handler::{CategoryModifyHandler, CategoryModifyHandlerTrait},
                modify_request::ModifyCategoryRequest,
            },
            read::category_read_handler::{CategoryReadHandler, CategoryReadHandlerTrait},
            test::fake_create_category_request,
        },
        entities::sea_orm_active_enums::CategoryType,
        StringExtension,
    };

    #[async_std::test]
    async fn handle_modify_category_testcase_successfully() {
        let beginning_test_timestamp = chrono::Utc::now();
        let test_space = setup_test_space().await;
        let database = test_space.postgres.get_database_connection().await;
        let number_of_tags = 2;
        let create_request = fake_create_category_request(number_of_tags);
        let origin_display_name = create_request.display_name.clone();

        let create_handler = CategoryCreateHandler {
            db: Arc::new(database.clone()),
        };
        let modify_handler = CategoryModifyHandler {
            db: Arc::new(database.clone()),
        };
        let read_handler = CategoryReadHandler {
            db: Arc::new(database),
        };
        let create_result = create_handler
            .handle_create_category_with_tags(create_request, Some("System".to_string()))
            .await
            .unwrap();

        assert!(!create_result.is_nil());
        let updated_name = format!("{} Updated", origin_display_name);
        let request = ModifyCategoryRequest {
            id: create_result,
            display_name: updated_name.clone(),
            category_type: CategoryType::Blog,
            parent_id: None,
            row_version: 1,
            tag_names: None,
        };

        let result = modify_handler
            .handle_modify_category(request, Some("System".to_string()))
            .await
            .unwrap();
        assert!(!result.is_nil());
        let category_in_db = read_handler.handle_get_all_categories().await.unwrap();
        let first = &category_in_db.first().unwrap();

        assert_eq!(result, first.id);
        assert!(first.created_by == "System");
        assert!(first.created_at >= beginning_test_timestamp);
        assert!(first.row_version == 2);
        assert!(first.display_name == updated_name);
        assert!(first.slug == updated_name.to_slug());
        assert_eq!(first.tags.len(), 0);
    }

    #[async_std::test]
    async fn handle_modify_category_testcase_failed_due_to_rowversion() {
        let test_space = setup_test_space().await;
        let conn = test_space.postgres.get_database_connection().await;
        let create_request = fake_create_category_request(5);
        let origin_display_name = create_request.display_name.clone();

        let create_handler = CategoryCreateHandler {
            db: Arc::new(conn.clone()),
        };
        let modify_handler = CategoryModifyHandler {
            db: Arc::new(conn.clone()),
        };

        let create_result = create_handler
            .handle_create_category_with_tags(create_request, Some("System".to_string()))
            .await
            .unwrap();
        assert!(!create_result.is_nil());

        let updated_name = format!("{} Updated", origin_display_name);
        let wrong_row_version = 0;
        let request = ModifyCategoryRequest {
            id: create_result,
            display_name: updated_name.clone(),
            category_type: CategoryType::Blog,
            parent_id: None,
            row_version: wrong_row_version,
            tag_names: None,
        };

        let result = modify_handler
            .handle_modify_category(request, Some("System".to_string()))
            .await;
        assert!(result.is_err());
    }
}
