use sea_orm::{sea_query::Expr, DatabaseConnection, DbErr, EntityTrait, QueryFilter, Set};
use std::sync::Arc;
use tracing::instrument;
use uuid::Uuid;

use crate::{
    common::datetime_generator::generate_vietname_now,
    entities::posts::{Column, Entity},
};

use super::modify_request::ModifyPostRequest;

pub trait PostModifyHandlerTrait {
    fn handle_modify_post(
        &self,
        body: ModifyPostRequest,
        actor_email: Option<String>,
    ) -> impl std::future::Future<Output = Result<Uuid, DbErr>>;
}

#[derive(Debug)]
pub struct PostModifyHandler {
    pub db: Arc<DatabaseConnection>,
}

impl PostModifyHandlerTrait for PostModifyHandler {
    #[instrument]
    async fn handle_modify_post(
        &self,
        body: ModifyPostRequest,
        actor_email: Option<String>,
    ) -> Result<Uuid, DbErr> {
        // check id does exist
        let post = Entity::find_by_id(body.id).one(self.db.as_ref()).await?;
        if post.is_none() {
            return Err(DbErr::RecordNotFound("Post not found".to_string()));
        }
        let current_row_version = body.row_version;
        let mut model = body.into_active_model();
        model.last_modified_by = Set(actor_email);
        model.last_modified_at = Set(Some(generate_vietname_now()));

        // Update  the category with current row version, if row version is not matched, return error
        let result = Entity::update_many()
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
                modify::{
                    modify_handler::{PostModifyHandler, PostModifyHandlerTrait},
                    modify_request::ModifyPostRequest,
                },
                read::read_handler::{PostReadHandler, PostReadHandlerTrait},
            },
        },
        entities::sea_orm_active_enums::CategoryType,
    };

    #[async_std::test]
    async fn handle_modify_post_testcase_successfully() {
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
        let post_modify_handler = PostModifyHandler {
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
            tags: None,
        };

        let created_category_id = category_create_handler
            .handle_create_category_with_tags(create_category_request, None)
            .await
            .unwrap();

        let create_post_request = CreatePostRequest {
            title: "Post Title".to_string(),
            preview_content: None,
            content: "Post Content".to_string(),
            published: false,
            category_id: created_category_id,
            slug: "post-title".to_string(),
        };

        let result = post_create_handler
            .handle_create_post(create_post_request, None)
            .await
            .unwrap();

        let request = ModifyPostRequest {
            id: result,
            title: "Post Title - Updated".to_string(),
            preview_content: None,
            content: "Post Content - Updated".to_string(),
            published: true,
            category_id: created_category_id,
            slug: "post-title-updated".to_string(),
            row_version: 1,
        };

        let result = post_modify_handler
            .handle_modify_post(request, Some("Last Modifier".to_string()))
            .await
            .unwrap();

        let posts_in_db = post_read_handler.handle_get_all_posts().await.unwrap();
        let first = posts_in_db.first().unwrap();

        assert_eq!(result, first.id);
        assert!(first.created_by == "System");
        assert!(first.created_at >= beginning_test_timestamp);
        assert!(first.row_version == 2);
        assert!(first.title == "Post Title - Updated");
        assert!(first.content == "Post Content - Updated");
        assert!(first.slug == "post-title-updated");
        assert!(first.last_modified_by == Some("Last Modifier".to_string()));
    }

    #[async_std::test]
    async fn handle_modify_post_testcase_failed() {
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
        let post_modify_handler = PostModifyHandler {
            db: arc_conn.clone(),
        };

        let create_category_request = CreateCategoryRequest {
            display_name: "Blog Category".to_string(),
            slug: "blog-category".to_string(),
            category_type: CategoryType::Blog,
            parent_id: None,
            tags: None,
        };

        let created_category_id = category_create_handler
            .handle_create_category_with_tags(create_category_request, None)
            .await
            .unwrap();

        let create_post_request = CreatePostRequest {
            title: "Post Title".to_string(),
            preview_content: None,
            content: "Post Content".to_string(),
            published: false,
            category_id: created_category_id,
            slug: "post-title".to_string(),
        };

        let result = post_create_handler
            .handle_create_post(create_post_request, None)
            .await
            .unwrap();

        let request = ModifyPostRequest {
            id: result,
            title: "Post Title - Updated".to_string(),
            preview_content: None,
            content: "Post Content - Updated".to_string(),
            published: true,
            category_id: created_category_id,
            slug: "post-title-updated".to_string(),
            row_version: 0,
        };

        let result = post_modify_handler
            .handle_modify_post(request, Some("System".to_string()))
            .await;
        assert!(result.is_err());
    }
}
