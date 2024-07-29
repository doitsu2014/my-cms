use std::sync::Arc;

use crate::entities::categories::{self, Column};
use sea_orm::{
    prelude::Uuid, sea_query::Expr, DatabaseConnection, DbErr, EntityTrait, QueryFilter, Set,
};
use tracing::instrument;

use super::modify_request::ModifyCategoryRequest;

pub trait CategoryModifyHandlerTrait {
    fn handle_modify_category(
        &self,
        body: ModifyCategoryRequest,
        actor_email: Option<String>,
    ) -> impl std::future::Future<Output = Result<Uuid, DbErr>>;
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
    ) -> Result<Uuid, DbErr> {
        // check id does exist
        let category = categories::Entity::find_by_id(body.id)
            .one(self.db.as_ref())
            .await?;
        if category.is_none() {
            return Err(DbErr::RecordNotFound("Category not found".to_string()));
        }

        let current_row_version = body.row_version;
        let mut model = body.into_active_model();
        model.last_modified_by = Set(actor_email);

        // Update  the category with current row version, if row version is not matched, return error
        let result = categories::Entity::update_many()
            .set(model)
            .filter(Expr::col(Column::Id).eq(body.id))
            .filter(Expr::col(Column::RowVersion).eq(current_row_version))
            .exec(self.db.as_ref())
            .await?;

        match result.rows_affected > 0 {
            true => Ok(body.id),
            false => Err(DbErr::Custom("Row version is not matched".to_string())),
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
        };

        let result = modify_handler
            .handle_modify_category(request, Some("System".to_string()))
            .await
            .unwrap();
        assert!(!result.is_nil());

        let category_in_db = read_handler.handle_get_all_categories().await.unwrap();
        let first = category_in_db.first().unwrap();

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
        };

        let result = modify_handler
            .handle_modify_category(request, Some("System".to_string()))
            .await;
        assert!(result.is_err());
    }
}
