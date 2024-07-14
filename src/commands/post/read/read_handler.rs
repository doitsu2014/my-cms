use axum::{extract::State, response::IntoResponse};
use entity::post;
use entity::prelude::*;
use sea_orm::DatabaseConnection;
use sea_orm::DbErr;
use sea_orm::EntityTrait;
use tracing::instrument;

use crate::{ApiResponseWith, AppState, AxumResponse};

#[instrument]
pub async fn handle_get_all_posts(conn: &DatabaseConnection) -> Result<Vec<PostModel>, DbErr> {
    let db_result = Post::find().all(conn).await?;
    Result::Ok(db_result)
}

#[instrument]
pub async fn handle_api_get_all_posts(state: State<AppState>) -> impl IntoResponse {
    let all = post::Entity::find()
        .all(&state.conn)
        .await
        .expect("Failed to fetch all posts");
    ApiResponseWith::new(all).to_axum_response()
}

#[cfg(test)]
mod tests {
    use entity::category::CategoryTypeEnum;
    use migration::Migrator;
    use sea_orm::Database;
    use sea_orm_migration::prelude::*;
    use testcontainers::runners::AsyncRunner;
    use testcontainers_modules::postgres::Postgres;

    use crate::commands::{
        category::create::{
            create_handler::handle_create_category, create_request::CreateCategoryRequest,
        },
        post::{
            create::{create_handler::handle_create_post, create_request::CreatePostRequest},
            read::read_handler::handle_get_all_posts,
        },
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

        let create_category_request = CreateCategoryRequest {
            display_name: "Blog Category".to_string(),
            category_type: CategoryTypeEnum::Blog,
            parent_id: None,
        };

        let created_category_id = handle_create_category(&conn, create_category_request)
            .await
            .unwrap();

        // Create vec with 3 element integer
        for i in 0..3 {
            let _ = handle_create_post(
                &conn,
                CreatePostRequest {
                    title: format!("Post Title {}", i),
                    slug: format!("post-title-{}", i),
                    content: "Please help to fill some lorem content".to_string(),
                    published: false,
                    category_id: created_category_id,
                },
            )
            .await;
        }

        let result = handle_get_all_posts(&conn).await.unwrap();
        assert_eq!(result.len(), 3);
    }
}
