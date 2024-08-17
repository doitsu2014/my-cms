use std::sync::Arc;

use crate::{
    common::app_error::{AppError, AppErrorExt},
    entities::categories,
    Categories,
};
use sea_orm::{ColumnTrait, DatabaseConnection, EntityTrait, QueryFilter};
use tracing::{info, instrument};
use uuid::Uuid;

pub trait CategoryDeleteHandlerTrait {
    fn handle_delete_categories(
        &self,
        ids: Vec<Uuid>,
        actor_email: Option<String>,
    ) -> impl std::future::Future<Output = Result<u64, AppError>>;
}

#[derive(Debug)]
pub struct CategoryDeleteHandler {
    pub db: Arc<DatabaseConnection>,
}

impl CategoryDeleteHandlerTrait for CategoryDeleteHandler {
    #[instrument]
    async fn handle_delete_categories(
        &self,
        ids: Vec<Uuid>,
        actor_email: Option<String>,
    ) -> Result<u64, AppError> {
        let result = Categories::delete_many()
            .filter(categories::Column::Id.is_in(ids))
            .exec(self.db.as_ref())
            .await
            .map_err(|e| e.to_app_error())?;

        info!(
            "{} categories deleted by {}",
            result.rows_affected,
            actor_email.unwrap_or_default()
        );

        return Ok(result.rows_affected);
    }
}

#[cfg(test)]
pub mod tests {
    use std::sync::Arc;
    use test_helpers::{setup_test_space, ContainerAsyncPostgresEx};

    use crate::commands::category::{
        create::create_handler::{CategoryCreateHandler, CategoryCreateHandlerTrait},
        read::category_read_handler::{CategoryReadHandler, CategoryReadHandlerTrait},
        test::{fake_create_category_request, fake_create_category_request_as_child},
    };

    use super::CategoryDeleteHandlerTrait;

    #[async_std::test]
    async fn handle_delete_categories_with_children() {
        let test_space = setup_test_space().await;
        let conn = test_space.postgres.get_database_connection().await;
        let number_of_tags = 5;

        let arc_conn = Arc::new(conn.clone());

        let delete_handler = super::CategoryDeleteHandler {
            db: arc_conn.clone(),
        };
        let create_handler = CategoryCreateHandler {
            db: arc_conn.clone(),
        };
        let read_handler = CategoryReadHandler { db: Arc::new(conn) };
        let parent_request = fake_create_category_request(number_of_tags);
        let parent_id = create_handler
            .handle_create_category_with_tags(parent_request, None)
            .await
            .unwrap();
        let child_request = fake_create_category_request_as_child(parent_id, number_of_tags);
        create_handler
            .handle_create_category_with_tags(child_request, None)
            .await
            .unwrap();

        let result = delete_handler
            .handle_delete_categories(vec![parent_id], None)
            .await;

        assert!(result.is_ok());
        let all_categories = read_handler.handle_get_all_categories().await.unwrap();
        assert_eq!(all_categories.len(), 0);
    }
}
