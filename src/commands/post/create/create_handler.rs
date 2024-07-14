use axum::{extract::State, response::IntoResponse, Json};
use entity::post::{self};
use entity::prelude::*;
use sea_orm::{prelude::Uuid, DatabaseConnection, DbErr, EntityTrait, IntoActiveModel};
use tower_cookies::Cookies;
use tracing::instrument;

use crate::{ApiResponseError, ApiResponseWith, AppState, AxumResponse};

use super::create_request::CreatePostRequest;

#[instrument]
pub async fn handle_create_post(
    conn: &DatabaseConnection,
    body: CreatePostRequest,
) -> Result<Uuid, DbErr> {
    let model = body.into_model();
    let active_model = post::ActiveModel {
        ..model.into_active_model()
    };
    let result = Post::insert(active_model).exec(conn).await?;
    Result::Ok(result.last_insert_id)
}

#[instrument]
pub async fn handle_api_create_post(
    state: State<AppState>,
    cookies: Cookies,
    Json(body): Json<CreatePostRequest>,
) -> impl IntoResponse {
    let result = handle_create_post(&state.conn, body).await;
    match result {
        Ok(last_id) => {
            ApiResponseWith::new(last_id)
                .with_message("System has already created Post successfully".to_string())
                .to_axum_response();
        }
        Err(e) => {
            ApiResponseError::new()
                .with_error_code(crate::ErrorCode::UnknownError)
                .add_error(e.to_string())
                .to_axum_response();
        }
    };
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
            create::create_request::CreatePostRequest, read::read_handler::handle_get_all_posts,
        },
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

        let create_category_request = CreateCategoryRequest {
            display_name: "Blog Category".to_string(),
            category_type: CategoryTypeEnum::Blog,
            parent_id: None,
        };

        let created_category_id = handle_create_category(&conn, create_category_request)
            .await
            .unwrap();

        let create_post_request = CreatePostRequest {
            title: "Post Title".to_string(),
            content: "Post Content".to_string(),
            published: false,
            category_id: created_category_id,
            slug: "post-title".to_string(),
        };

        let result = super::handle_create_post(&conn, create_post_request)
            .await
            .unwrap();

        let db_posts = handle_get_all_posts(&conn).await.unwrap();
        let first = db_posts.first().unwrap();
        assert_eq!(result, first.id);
        assert!(first.created_by == "System");
        assert!(first.created_at >= beginning_test_timestamp);
    }
}
