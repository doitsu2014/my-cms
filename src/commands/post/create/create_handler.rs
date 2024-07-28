use application_core::{common::datetime_generator::generate_vietname_now, entities::posts, Posts};
use axum::{extract::State, response::IntoResponse, Extension, Json};
use axum_keycloak_auth::decode::KeycloakToken;
use sea_orm::{prelude::Uuid, DatabaseConnection, DbErr, EntityTrait, IntoActiveModel, Set};
use tower_cookies::Cookies;
use tracing::instrument;

use crate::{
    keycloak_extension::ExtractKeyCloakToken, ApiResponseError, ApiResponseWith, AppState,
    AxumResponse, ErrorCode,
};

use super::create_request::CreatePostRequest;

#[instrument]
pub async fn handle_create_post(
    conn: &DatabaseConnection,
    body: CreatePostRequest,
    actor_email: Option<String>,
) -> Result<Uuid, DbErr> {
    let model = body.into_model();
    let mut active_model = posts::ActiveModel {
        ..model.into_active_model()
    };
    active_model.created_by = Set(actor_email.unwrap_or("System".to_string()));
    active_model.created_at = Set(generate_vietname_now());

    let result = Posts::insert(active_model).exec(conn).await?;
    Result::Ok(result.last_insert_id)
}

#[instrument]
pub async fn api_create_post(
    state: State<AppState>,
    cookies: Cookies,
    Extension(token): Extension<KeycloakToken<String>>,
    Json(body): Json<CreatePostRequest>,
) -> impl IntoResponse {
    let result = handle_create_post(&state.conn, body, Some(token.extract_email().email)).await;
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
        category::create::create_handler::handle_create_category_with_tags,
        commands::{
            category::create::create_request::CreateCategoryRequest,
            post::{
                create::create_request::CreatePostRequest, read::read_handler::handle_get_all_posts,
            },
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
            slug: "blog-category".to_string(),
            category_type: CategoryType::Blog,
            parent_id: None,
            tags: None,
        };

        let created_category_id =
            handle_create_category_with_tags(conn.clone(), create_category_request, None)
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

        let result = super::handle_create_post(&conn, create_post_request, None)
            .await
            .unwrap();

        let db_posts = handle_get_all_posts(&conn).await.unwrap();
        let first = db_posts.first().unwrap();
        assert_eq!(result, first.id);
        assert!(first.created_by == "System");
        assert!(first.created_at >= beginning_test_timestamp);
        assert!(first.row_version == 1);
    }
}
