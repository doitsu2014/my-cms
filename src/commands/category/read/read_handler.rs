use crate::{ApiResponseError, ApiResponseWith, AppState, AxumResponse};
use axum::{extract::State, response::IntoResponse};
use entity::prelude::*;
use sea_orm::{DatabaseConnection, DbErr, EntityTrait};
use tracing::instrument;

#[instrument]
pub async fn handle_get_all_categories(
    conn: &DatabaseConnection,
) -> Result<Vec<CategoryModel>, DbErr> {
    let db_result = Category::find().all(conn).await?;
    Result::Ok(db_result)
}

#[instrument]
pub async fn handle_api_get_all_categories(state: State<AppState>) -> impl IntoResponse {
    let result = handle_get_all_categories(&state.conn).await;
    match result {
        Ok(categories) => {
            ApiResponseWith::new(categories).to_axum_response();
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
    async fn handle_get_all_cartegory_testcase_01() {
        let postgres = Postgres::default().start().await.unwrap();

        let connection_string: String = format!(
            "postgres://postgres:postgres@127.0.0.1:{}/postgres",
            postgres.get_host_port_ipv4(5432).await.unwrap()
        );
        let conn = Database::connect(&connection_string).await.unwrap();
        Migrator::refresh(&conn).await.unwrap();

        // Create vec with 3 element integer

        for i in 0..3 {
            let _ = handle_create_category(
                &conn,
                CreateCategoryRequest {
                    display_name: format!("Category {}", i),
                    category_type: CategoryTypeEnum::Blog,
                    parent_id: None,
                },
            )
            .await;
        }

        let result = handle_get_all_categories(&conn).await;

        match result {
            Ok(categories) => assert_eq!(categories.len(), 3),
            _ => panic!("Test failed"),
        }
    }
}
