use std::sync::Arc;

use crate::{
    common::app_error::{AppError},
    entities::posts,
    Posts,
};
use sea_orm::{ColumnTrait, DatabaseConnection, EntityTrait, QueryFilter};
use tracing::{info, instrument};
use uuid::Uuid;

pub trait PostDeleteHandlerTrait {
    fn handle_delete_posts(
        &self,
        ids: Vec<Uuid>,
        actor_email: Option<String>,
    ) -> impl std::future::Future<Output = Result<u64, AppError>>;
}

#[derive(Debug)]
pub struct PostDeleteHandler {
    pub db: Arc<DatabaseConnection>,
}

impl PostDeleteHandlerTrait for PostDeleteHandler {
    #[instrument]
    async fn handle_delete_posts(
        &self,
        ids: Vec<Uuid>,
        actor_email: Option<String>,
    ) -> Result<u64, AppError> {
        let result = Posts::delete_many()
            .filter(posts::Column::Id.is_in(ids))
            .exec(self.db.as_ref())
            .await
            .map_err(|e| e.into())?;

        info!(
            "{} posts deleted by {}",
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

    use crate::commands::{
        category::{
            create::create_handler::{CategoryCreateHandler, CategoryCreateHandlerTrait},
            test::fake_create_category_request,
        },
        post::{
            create::create_handler::{PostCreateHandler, PostCreateHandlerTrait},
            delete::delete_handler::{PostDeleteHandler, PostDeleteHandlerTrait},
            read::read_handler::{PostReadHandler, PostReadHandlerTrait},
            test::fake_create_post_request,
        },
    };

    #[async_std::test]
    async fn handle_delete_posts_with_children() {
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
        let post_delete_handler = PostDeleteHandler {
            db: arc_conn.clone(),
        };
        let create_category_request = fake_create_category_request(5);
        let created_category_id = category_create_handler
            .handle_create_category_with_tags(create_category_request, None)
            .await
            .unwrap();
        let create_post_request = fake_create_post_request(created_category_id, 5);
        let create_result = post_create_handler
            .handle_create_post(create_post_request, None)
            .await
            .unwrap();

        // Delete
        let result = post_delete_handler
            .handle_delete_posts(vec![create_result], None)
            .await;

        assert!(result.is_ok());

        let db_posts = post_read_handler.handle_get_all_posts().await.unwrap();
        assert_eq!(db_posts.len(), 0);
    }
}
