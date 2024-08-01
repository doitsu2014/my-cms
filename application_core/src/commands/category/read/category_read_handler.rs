use std::sync::Arc;

use sea_orm::{ColumnTrait, DatabaseConnection, DbErr, EntityTrait, QueryFilter};
use serde::{Deserialize, Serialize};
use uuid::Uuid;

use crate::{
    entities::{categories, tags},
    Categories, Tags,
};

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct CategoryReadResponse {
    pub category: categories::Model,
    pub tags: Vec<tags::Model>,
}

pub trait CategoryReadHandlerTrait {
    fn handle_get_all_categories(
        &self,
    ) -> impl std::future::Future<Output = Result<Vec<CategoryReadResponse>, DbErr>> + Send;

    fn handle_get_category(
        &self,
        id: Uuid,
    ) -> impl std::future::Future<Output = Result<CategoryReadResponse, DbErr>> + Send;
}

pub struct CategoryReadHandler {
    pub db: Arc<DatabaseConnection>,
}

impl CategoryReadHandlerTrait for CategoryReadHandler {
    async fn handle_get_all_categories(&self) -> Result<Vec<CategoryReadResponse>, DbErr> {
        let db_result = Categories::find()
            .find_with_related(Tags)
            .all(self.db.as_ref())
            .await?;

        let response = db_result
            .iter()
            .map(|c_and_tags| CategoryReadResponse {
                category: c_and_tags.0.to_owned(),
                tags: c_and_tags.1.to_owned(),
            })
            .collect::<Vec<CategoryReadResponse>>();

        // let category and tags
        Result::Ok(response)
    }

    async fn handle_get_category(&self, id: Uuid) -> Result<CategoryReadResponse, DbErr> {
        let db_result = Categories::find_by_id(id)
            .find_with_related(Tags)
            .all(self.db.as_ref())
            .await?;

        if db_result.is_empty() {
            return Result::Err(DbErr::RecordNotFound("Category not found".to_string()));
        }

        let response = db_result
            .iter()
            .map(|c_and_tags| CategoryReadResponse {
                category: c_and_tags.0.to_owned(),
                tags: c_and_tags.1.to_owned(),
            })
            .collect::<Vec<CategoryReadResponse>>()
            .first()
            .unwrap()
            .to_owned();

        // let category and tags
        Result::Ok(response)
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
            read::category_read_handler::{CategoryReadHandler, CategoryReadHandlerTrait},
        },
        entities::sea_orm_active_enums::CategoryType,
    };

    #[async_std::test]
    async fn handle_get_all_cartegory_testcase_01() {
        let postgres = Postgres::default().start().await.unwrap();

        let connection_string: String = format!(
            "postgres://postgres:postgres@127.0.0.1:{}/postgres",
            postgres.get_host_port_ipv4(5432).await.unwrap()
        );
        let conn = Database::connect(&connection_string).await.unwrap();
        Migrator::refresh(&conn).await.unwrap();

        let create_handler = CategoryCreateHandler {
            db: Arc::new(conn.clone()),
        };
        let read_handler = CategoryReadHandler { db: Arc::new(conn) };

        // Create vec with 3 element integer
        for i in 0..3 {
            let _ = create_handler
                .handle_create_category_with_tags(
                    CreateCategoryRequest {
                        display_name: format!("Category {}", i),
                        slug: format!("category-{}", i).to_string(),
                        category_type: CategoryType::Blog,
                        parent_id: None,
                        tags: None,
                    },
                    None,
                )
                .await;
        }

        let result = read_handler.handle_get_all_categories().await;

        match result {
            Ok(categories) => assert_eq!(categories.len(), 3),
            _ => panic!("Test failed"),
        }
    }
}
