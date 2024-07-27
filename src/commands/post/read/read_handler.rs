use application_core::{entities::posts::Model, Posts};
use axum::{extract::State, response::IntoResponse};
use sea_orm::DatabaseConnection;
use sea_orm::DbErr;
use sea_orm::EntityTrait;
use tracing::instrument;

use crate::ApiResponseError;
use crate::AxumResponse;
use crate::ErrorCode;
use crate::{ApiResponseWith, AppState};

#[instrument]
pub async fn handle_get_all_posts(conn: &DatabaseConnection) -> Result<Vec<Model>, DbErr> {
    let db_result = Posts::find().all(conn).await?;
    Result::Ok(db_result)
}

#[instrument]
pub async fn api_get_all_posts(state: State<AppState>) -> impl IntoResponse {
    let result = handle_get_all_posts(&state.conn).await;
    match result {
        Ok(posts) => ApiResponseWith::new(posts).to_axum_response(),
        Err(e) => ApiResponseError::new()
            .with_error_code(ErrorCode::UnknownError)
            .add_error(e.to_string())
            .to_axum_response(),
    }
}

#[cfg(test)]
mod tests {
    use application_core::entities::sea_orm_active_enums::CategoryType;
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
            slug: "blog-category".to_string(),
            category_type: CategoryType::Blog,
            parent_id: None,
        };

        let created_category_id = handle_create_category(&conn, create_category_request, None)
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
                None,
            )
            .await;
        }

        let result = handle_get_all_posts(&conn).await.unwrap();
        assert_eq!(result.len(), 3);
    }
}
