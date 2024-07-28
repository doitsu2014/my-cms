use crate::{ApiResponseError, ApiResponseWith, AppState, AxumResponse, ErrorCode};
use application_core::{entities::categories::Model, Categories};
use axum::{extract::State, response::IntoResponse};
use sea_orm::{DatabaseConnection, DbErr, EntityTrait};
use tracing::instrument;

#[instrument]
pub async fn handle_get_all_categories(conn: &DatabaseConnection) -> Result<Vec<Model>, DbErr> {
    let db_result = Categories::find().all(conn).await?;
    Result::Ok(db_result)
}

#[instrument]
pub async fn api_get_all_categories(state: State<AppState>) -> impl IntoResponse {
    let result = handle_get_all_categories(&state.conn).await;

    match result {
        Ok(categories) => ApiResponseWith::new(categories).to_axum_response(),
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
        commands::category::{
            create::create_request::CreateCategoryRequest,
            read::read_handler::handle_get_all_categories,
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

        // Create vec with 3 element integer

        for i in 0..3 {
            let _ = handle_create_category_with_tags(
                conn.clone(),
                CreateCategoryRequest {
                    display_name: format!("Category {}", i),
                    slug: format!("category-{}", i).to_string(),
                    category_type: CategoryType::Blog,
                    parent_id: None,
                    tags: None,
                },
                None,
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
