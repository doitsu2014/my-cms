use std::sync::Arc;

use crate::{
    commands::{
        category::read::category_read_handler::{
            self, CategoryReadHandlerTrait, CategoryReadResponse,
        },
        tag::read::read_handler::{TagReadHandler, TagReadHandlerTrait},
    },
    common::datetime_generator::generate_vietname_now,
    entities::{
        categories::{self, Column},
        category_tags, tags,
    },
    Tags,
};
use sea_orm::{
    prelude::Uuid, sea_query::Expr, DatabaseConnection, DbErr, EntityTrait, IntoActiveModel,
    QueryFilter, Set, TransactionError, TransactionTrait,
};
use tracing::instrument;

use super::modify_request::ModifyCategoryRequest;

pub trait CategoryModifyHandlerTrait {
    fn handle_modify_category(
        &self,
        body: ModifyCategoryRequest,
        actor_email: Option<String>,
    ) -> impl std::future::Future<Output = Result<Uuid, TransactionError<DbErr>>>;
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
    ) -> Result<Uuid, TransactionError<DbErr>> {
        let category_read_handler = category_read_handler::CategoryReadHandler {
            db: self.db.clone(),
        };

        let tag_read_handler = TagReadHandler {
            db: self.db.clone(),
        };

        // Update  the category with current row version, if row version is not matched, return error
        let result: Result<Uuid, TransactionError<DbErr>> = self
            .db
            .as_ref()
            .transaction::<_, Uuid, DbErr>(|tx| {
                Box::pin(async move {
                    // Prepare Active Category
                    let modified_id = body.id;
                    let current_row_version = body.row_version;
                    let mut model = body.into_active_model();
                    model.last_modified_by = Set(actor_email.clone());

                    // get existing category
                    let category: CategoryReadResponse = category_read_handler
                        .handle_get_category(modified_id)
                        .await?;

                    // Prepare Tags
                    let tags: Vec<String> = body.tags.unwrap_or_default();
                    let classifed_tags = tag_read_handler
                        .handle_get_and_classify_tags_by_names(tags)
                        .await?;
                    let existing_tag_ids = classifed_tags
                        .existing_tags
                        .iter()
                        .map(|tag| tag.id)
                        .collect::<Vec<Uuid>>();

                    let modified_result = categories::Entity::update_many()
                        .set(model)
                        .filter(Expr::col(Column::Id).eq(modified_id))
                        .filter(Expr::col(Column::RowVersion).eq(current_row_version))
                        .exec(tx)
                        .await?;

                    match modified_result.rows_affected == 0 {
                        true => {
                            return Err(DbErr::Custom("Row version is not matched".to_string()))
                        }
                        false => (),
                    }

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
                                    category_id: body.id,
                                    tag_id: tag_id.to_owned(),
                                }
                                .into_active_model()
                            })
                            .collect::<Vec<category_tags::ActiveModel>>();

                        category_tags::Entity::insert_many(category_tags)
                            .exec(tx)
                            .await?;
                    }
                    Ok(modified_id)
                })
            })
            .await;

        match result {
            Ok(modified_id) => Ok(modified_id),
            Err(e) => Err(e.into()),
        }
    }
}

#[cfg(test)]
mod tests {
    use migration::{Migrator, MigratorTrait};
    use sea_orm::Database;
    use std::sync::Arc;
    use testcontainers::runners::AsyncRunner;
    use testcontainers_modules::postgres::Postgres;

    use crate::{
        commands::category::{
            create::{
                create_handler::{CategoryCreateHandler, CategoryCreateHandlerTrait},
                create_request::CreateCategoryRequest,
            },
            modify::{
                modify_handler::{CategoryModifyHandler, CategoryModifyHandlerTrait},
                modify_request::ModifyCategoryRequest,
            },
            read::category_read_handler::{CategoryReadHandler, CategoryReadHandlerTrait},
        },
        entities::sea_orm_active_enums::CategoryType,
    };

    #[async_std::test]
    async fn handle_modify_category_testcase_successfully() {
        let beginning_test_timestamp = chrono::Utc::now();
        let postgres = Postgres::default().start().await.unwrap();
        let connection_string: String = format!(
            "postgres://postgres:postgres@127.0.0.1:{}/postgres",
            postgres.get_host_port_ipv4(5432).await.unwrap()
        );
        let conn = Database::connect(&connection_string).await.unwrap();
        Migrator::refresh(&conn).await.unwrap();

        let create_request = CreateCategoryRequest {
            display_name: "Category 1".to_string(),
            slug: "category-1".to_string(),
            category_type: CategoryType::Blog,
            parent_id: None,
            tags: None,
        };

        let create_handler = CategoryCreateHandler {
            db: Arc::new(conn.clone()),
        };
        let modify_handler = CategoryModifyHandler {
            db: Arc::new(conn.clone()),
        };
        let read_handler = CategoryReadHandler { db: Arc::new(conn) };

        let create_result = create_handler
            .handle_create_category_with_tags(create_request, Some("System".to_string()))
            .await
            .unwrap();
        assert!(!create_result.is_nil());

        let request = ModifyCategoryRequest {
            id: create_result,
            display_name: "Category 1 - Updated".to_string(),
            slug: "category-1-updated".to_string(),
            category_type: CategoryType::Blog,
            parent_id: None,
            row_version: 1,
            tags: None,
        };

        let result = modify_handler
            .handle_modify_category(request, Some("System".to_string()))
            .await
            .unwrap();
        assert!(!result.is_nil());

        let category_in_db = read_handler.handle_get_all_categories().await.unwrap();
        let first = &category_in_db.first().unwrap().category;

        assert_eq!(result, first.id);
        assert!(first.created_by == "System");
        assert!(first.created_at >= beginning_test_timestamp);
        assert!(first.row_version == 2);
        assert!(first.display_name == "Category 1 - Updated");
        assert!(first.slug == "category-1-updated");
    }

    #[async_std::test]
    async fn handle_modify_category_testcase_failed() {
        let postgres = Postgres::default().start().await.unwrap();
        let connection_string: String = format!(
            "postgres://postgres:postgres@127.0.0.1:{}/postgres",
            postgres.get_host_port_ipv4(5432).await.unwrap()
        );
        let conn = Database::connect(&connection_string).await.unwrap();
        Migrator::refresh(&conn).await.unwrap();

        let create_request = CreateCategoryRequest {
            display_name: "Category 1".to_string(),
            slug: "category-1".to_string(),
            category_type: CategoryType::Blog,
            parent_id: None,
            tags: None,
        };

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

        let request = ModifyCategoryRequest {
            id: create_result,
            display_name: "Category 1 - Updated".to_string(),
            slug: "category-1-updated".to_string(),
            category_type: CategoryType::Blog,
            parent_id: None,
            row_version: 0,
            tags: None,
        };

        let result = modify_handler
            .handle_modify_category(request, Some("System".to_string()))
            .await;
        assert!(result.is_err());
    }
}
