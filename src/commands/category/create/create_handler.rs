use super::create_request::CreateCategoryRequest;
use crate::{
    keycloak_extension::ExtractKeyCloakToken, ApiResponseError, ApiResponseWith, AppState,
    AxumResponse, ErrorCode,
};
use application_core::{
    common::datetime_generator::generate_vietname_now, entities::categories, Categories,
};
use axum::{extract::State, response::IntoResponse, Extension, Json};
use axum_keycloak_auth::decode::KeycloakToken;
use sea_orm::{prelude::Uuid, DatabaseConnection, DbErr, EntityTrait, IntoActiveModel};
use tower_cookies::Cookies;
use tracing::instrument;

#[instrument]
pub async fn handle_create_category(
    conn: &DatabaseConnection,
    body: CreateCategoryRequest,
    actor_email: Option<String>,
) -> Result<Uuid, DbErr> {
    let model: categories::Model = body.into_model();
    let model = categories::Model {
        created_by: actor_email.unwrap_or("System".to_string()),
        created_at: generate_vietname_now(),
        ..model
    };
    let active_model = categories::ActiveModel {
        ..model.into_active_model()
    };
    let result = Categories::insert(active_model).exec(conn).await?;
    Result::Ok(result.last_insert_id)
}

#[instrument]
pub async fn api_create_category(
    state: State<AppState>,
    cookies: Cookies,
    Extension(token): Extension<KeycloakToken<String>>,
    Json(body): Json<CreateCategoryRequest>,
) -> impl IntoResponse {
    let result = handle_create_category(&state.conn, body, Some(token.extract_email().email)).await;

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
    use application_core::entities::{categories::Model, sea_orm_active_enums::CategoryType};
    use migration::Migrator;
    use sea_orm::Database;
    use sea_orm_migration::prelude::*;
    use testcontainers::runners::AsyncRunner;
    use testcontainers_modules::postgres::Postgres;

    use crate::commands::category::{
        create::{create_handler::handle_create_category, create_request::CreateCategoryRequest},
        read::read_handler::handle_get_all_categories,
    };

    #[async_std::test]
    async fn handle_create_cartegory_testcase_01() {
        let beginning_test_timestamp = chrono::Utc::now();
        let postgres = Postgres::default().start().await.unwrap();
        let connection_string: String = format!(
            "postgres://postgres:postgres@127.0.0.1:{}/postgres",
            postgres.get_host_port_ipv4(5432).await.unwrap()
        );
        let conn = Database::connect(&connection_string).await.unwrap();
        Migrator::refresh(&conn).await.unwrap();

        let request = CreateCategoryRequest {
            display_name: "Category 1".to_string(),
            slug: "category-1".to_string(),
            category_type: CategoryType::Blog,
            parent_id: None,
        };
        let result = handle_create_category(&conn, request, None).await.unwrap();
        assert!(!result.is_nil());

        let category_in_db = handle_get_all_categories(&conn).await.unwrap();
        let first = category_in_db.first().unwrap();
        assert_eq!(result, first.id);
        assert!(first.created_by == "System");
        assert!(first.created_at >= beginning_test_timestamp);
        assert!(first.row_version == 1);
    }

    #[async_std::test]
    async fn handle_create_cartegory_testcase_parent() {
        // TODO: Make those steps to common_tests
        let postgres = Postgres::default().start().await.unwrap();
        let connection_string: String = format!(
            "postgres://postgres:postgres@127.0.0.1:{}/postgres",
            postgres.get_host_port_ipv4(5432).await.unwrap()
        );
        let conn = Database::connect(&connection_string).await.unwrap();
        Migrator::refresh(&conn).await.unwrap();

        let parent_request = CreateCategoryRequest {
            display_name: "Category 1".to_string(),
            slug: "category-1".to_string(),
            category_type: CategoryType::Blog,
            parent_id: None,
        };
        let parent = handle_create_category(&conn, parent_request, None)
            .await
            .unwrap();

        let child_request = CreateCategoryRequest {
            display_name: "Child of Category 1".to_string(),
            slug: "child-of-category-1".to_string(),
            category_type: CategoryType::Blog,
            parent_id: Some(parent),
        };
        let child = handle_create_category(&conn, child_request, None)
            .await
            .unwrap();
        let categories_in_db: Vec<Model> = handle_get_all_categories(&conn).await.unwrap();

        let first = categories_in_db
            .iter()
            .filter(|x| x.parent_id.is_none())
            .next()
            .unwrap();

        let child_instance = categories_in_db
            .iter()
            .filter(|x| x.id == child)
            .next()
            .unwrap();

        assert_eq!(child_instance.parent_id.unwrap(), first.id);
    }
}
