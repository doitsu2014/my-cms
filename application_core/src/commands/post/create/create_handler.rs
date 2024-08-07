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
    use migration::{Migrator, MigratorTrait};
    use sea_orm::Database;
    use std::sync::Arc;
    use testcontainers::runners::AsyncRunner;
    use testcontainers_modules::postgres::Postgres;

    use crate::{
        commands::{
            category::create::{
                create_handler::{CategoryCreateHandler, CategoryCreateHandlerTrait},
                create_request::CreateCategoryRequest,
            },
            post::{
                create::{
                    create_handler::{PostCreateHandler, PostCreateHandlerTrait},
                    create_request::CreatePostRequest,
                },
                read::read_handler::{PostReadHandler, PostReadHandlerTrait},
            },
        },
        entities::sea_orm_active_enums::CategoryType,
    };

    #[async_std::test]
    async fn handle_create_post_testcase_01() {
        let beginning_test_timestamp = chrono::Utc::now();
        let postgres = Postgres::default().start().await.unwrap();

        let connection_string: String = format!(
            "postgres://postgres:postgres@127.0.0.1:{}/postgres",
            postgres.get_host_port_ipv4(5432).await.unwrap()
        );
        let conn = Database::connect(&connection_string).await.unwrap();
        Migrator::refresh(&conn).await.unwrap();

        let arc_conn = Arc::new(conn.clone());

        let category_create_handler = CategoryCreateHandler {
            db: arc_conn.clone(),
        };
        let post_create_handler = PostCreateHandler {
            db: arc_conn.clone(),
        };
        let post_read_handler = PostReadHandler {
            db: arc_conn.clone(),
        };

        let create_category_request = CreateCategoryRequest {
            display_name: "Blog Category".to_string(),
            slug: "blog-category".to_string(),
            category_type: CategoryType::Blog,
            parent_id: None,
            tag_names: None,
        };

        let created_category_id = category_create_handler
            .handle_create_category_with_tags(create_category_request, None)
            .await
            .unwrap();

        let create_post_request = CreatePostRequest {
            title: "Post Title".to_string(),
            content: "Post Content".to_string(),
            preview_content: None,
            published: false,
            category_id: created_category_id,
            slug: "post-title".to_string(),
        };

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
