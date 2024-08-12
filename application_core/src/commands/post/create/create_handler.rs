use sea_orm::{DatabaseConnection, DbErr, EntityTrait, IntoActiveModel, Set};
use std::sync::Arc;
use tracing::instrument;
use uuid::Uuid;

use crate::{
    common::datetime_generator::generate_vietname_now, entities::posts::ActiveModel, Posts,
};

use super::create_request::CreatePostRequest;

pub trait PostCreateHandlerTrait {
    fn handle_create_post(
        &self,
        body: CreatePostRequest,
        actor_email: Option<String>,
    ) -> impl std::future::Future<Output = Result<Uuid, DbErr>>;
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
    ) -> Result<Uuid, DbErr> {
        let model = body.into_model();
        let mut active_model = ActiveModel {
            ..model.into_active_model()
        };
        active_model.created_by = Set(actor_email.unwrap_or("System".to_string()));
        active_model.created_at = Set(generate_vietname_now());

        let result = Posts::insert(active_model).exec(self.db.as_ref()).await?;
        Result::Ok(result.last_insert_id)
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
    async fn handle_create_post_testcase_01() {
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
    }
}
