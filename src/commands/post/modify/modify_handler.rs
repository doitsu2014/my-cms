use super::modify_request::ModifyPostRequest;
use crate::{ApiResponseError, ApiResponseWith, AppState, AxumResponse, ErrorCode};
use application_core::entities::posts::{self, Column};
use axum::{extract::State, response::IntoResponse, Json};
use migration::Expr;
use sea_orm::{prelude::Uuid, DatabaseConnection, DbErr, EntityTrait, QueryFilter};
use tower_cookies::Cookies;
use tracing::instrument;

#[instrument]
pub async fn handle_modify_post(
    conn: &DatabaseConnection,
    body: ModifyPostRequest,
) -> Result<Uuid, DbErr> {
    // check id does exist
    let post = posts::Entity::find_by_id(body.id).one(conn).await?;
    if post.is_none() {
        return Err(DbErr::RecordNotFound("Post not found".to_string()));
    }

    let current_row_version = body.row_version;
    let model = body.into_active_model();

    // Update  the category with current row version, if row version is not matched, return error
    let result = posts::Entity::update_many()
        .set(model)
        .filter(Expr::col(Column::Id).eq(body.id))
        .filter(Expr::col(Column::RowVersion).eq(current_row_version))
        .exec(conn)
        .await?;

    match result.rows_affected > 0 {
        true => Ok(body.id),
        false => Err(DbErr::Custom("Row version is not matched".to_string())),
    }
}

#[instrument]
pub async fn handle_api_modify_post(
    state: State<AppState>,
    cookies: Cookies,
    Json(body): Json<ModifyPostRequest>,
) -> impl IntoResponse {
    let result = handle_modify_post(&state.conn, body).await;

    match result {
        Ok(inserted_id) => ApiResponseWith::new(inserted_id.to_string()).to_axum_response(),
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

    use crate::{
        category::create::{
            create_handler::handle_create_category, create_request::CreateCategoryRequest,
        },
        post::{
            create::{create_handler::handle_create_post, create_request::CreatePostRequest},
            modify::{modify_handler::handle_modify_post, modify_request::ModifyPostRequest},
            read::read_handler::handle_get_all_posts,
        },
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

        let create_category_request = CreateCategoryRequest {
            display_name: "Blog Category".to_string(),
            slug: "blog-category".to_string(),
            category_type: CategoryType::Blog,
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

        let result = handle_create_post(&conn, create_post_request)
            .await
            .unwrap();

        let request = ModifyPostRequest {
            id: result,
            title: "Post Title - Updated".to_string(),
            content: "Post Content - Updated".to_string(),
            published: true,
            category_id: created_category_id,
            slug: "post-title-updated".to_string(),
            row_version: 1,
        };

        let result = handle_modify_post(&conn, request).await.unwrap();

        let posts_in_db = handle_get_all_posts(&conn).await.unwrap();
        let first = posts_in_db.first().unwrap();

        assert_eq!(result, first.id);
        assert!(first.created_by == "System");
        assert!(first.created_at >= beginning_test_timestamp);
        assert!(first.row_version == 2);
        assert!(first.title == "Post Title - Updated");
        assert!(first.content == "Post Content - Updated");
        assert!(first.slug == "post-title-updated");
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

        let create_category_request = CreateCategoryRequest {
            display_name: "Blog Category".to_string(),
            slug: "blog-category".to_string(),
            category_type: CategoryType::Blog,
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

        let result = handle_create_post(&conn, create_post_request)
            .await
            .unwrap();

        let request = ModifyPostRequest {
            id: result,
            title: "Post Title - Updated".to_string(),
            content: "Post Content - Updated".to_string(),
            published: true,
            category_id: created_category_id,
            slug: "post-title-updated".to_string(),
            row_version: 0,
        };

        let result = handle_modify_post(&conn, request).await;
        assert!(result.is_err());
    }
}
