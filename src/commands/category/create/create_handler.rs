use super::create_request::CreateCategoryRequest;
use crate::{ApiResponseError, ApiResponseWith, AppState, AxumResponse};
use axum::{extract::State, response::IntoResponse, Json};
use entity::category;
use entity::prelude::Category;
use sea_orm::{prelude::Uuid, DatabaseConnection, DbErr, EntityTrait, IntoActiveModel};
use tower_cookies::Cookies;
use tracing::instrument;

#[instrument]
pub async fn handle_create_category(
    conn: &DatabaseConnection,
    body: CreateCategoryRequest,
) -> Result<Uuid, DbErr> {
    let model = body.into_model();
    let active_model = category::ActiveModel {
        ..model.into_active_model()
    };
    let result = Category::insert(active_model).exec(conn).await?;

    Result::Ok(result.last_insert_id)
}

#[instrument]
pub async fn handle_api_create_category(
    state: State<AppState>,
    cookies: Cookies,
    Json(body): Json<CreateCategoryRequest>,
) -> impl IntoResponse {
    let result = handle_create_category(&state.conn, body).await;

    match result {
        Ok(inserted_id) => {
            ApiResponseWith::new(inserted_id)
                .with_message("System has already create Category successfully".to_string())
                .to_axum_response();
        }
        Err(e) => {
            ApiResponseError::new()
                .with_error_code(crate::ErrorCode::UnknownError)
                .add_error(e.to_string())
                .to_axum_response();
        }
    }
}

#[cfg(test)]
mod tests {
    use entity::category::CategoryTypeEnum;
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
            category_type: CategoryTypeEnum::Blog,
        };
        let result = handle_create_category(&conn, request).await.unwrap();
        assert!(!result.is_nil());

        let category_in_db = handle_get_all_categories(&conn).await.unwrap();
        let first = category_in_db.first().unwrap();
        assert_eq!(result, first.id);
        assert!(first.created_by == "System");
        assert!(first.created_at >= beginning_test_timestamp);
    }
}
