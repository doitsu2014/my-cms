use std::sync::Arc;

use sea_orm::{
    prelude::DateTimeWithTimeZone, sea_query::Expr, ActiveEnum, DatabaseConnection, EntityTrait,
    QueryFilter, QueryOrder,
};
use serde::{Deserialize, Serialize};
use uuid::Uuid;

use crate::{
    common::app_error::AppError,
    entities::{
        categories::{self, Column, Model},
        sea_orm_active_enums::CategoryType,
        tags,
    },
    Categories, Tags,
};

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct CategoryReadResponse {
    pub id: Uuid,
    pub display_name: String,
    pub slug: String,
    pub category_type: CategoryType,
    pub created_by: String,
    pub created_at: DateTimeWithTimeZone,
    pub last_modified_by: Option<String>,
    pub last_modified_at: Option<DateTimeWithTimeZone>,
    pub parent_id: Option<Uuid>,
    pub row_version: i32,
    pub tags: Vec<tags::Model>,
    pub tag_names: Vec<String>,
}

impl CategoryReadResponse {
    fn new(category: Model, tags: Vec<tags::Model>) -> Self {
        let tag_names = tags
            .iter()
            .map(|tag| tag.name.to_owned())
            .collect::<Vec<String>>();

        CategoryReadResponse {
            id: category.id,
            display_name: category.display_name,
            slug: category.slug,
            category_type: category.category_type,
            created_by: category.created_by,
            created_at: category.created_at,
            last_modified_by: category.last_modified_by,
            last_modified_at: category.last_modified_at,
            parent_id: category.parent_id,
            row_version: category.row_version,
            tags,
            tag_names,
        }
    }
}

pub trait CategoryReadHandlerTrait {
    fn handle_get_all_categories(
        &self,
    ) -> impl std::future::Future<Output = Result<Vec<CategoryReadResponse>, AppError>> + Send;

    fn handle_get_with_filtering(
        &self,
        category_type: Option<CategoryType>,
    ) -> impl std::future::Future<Output = Result<Vec<CategoryReadResponse>, AppError>> + Send;

    fn handle_get_category(
        &self,
        id: Uuid,
    ) -> impl std::future::Future<Output = Result<CategoryReadResponse, AppError>> + Send;
}

pub struct CategoryReadHandler {
    pub db: Arc<DatabaseConnection>,
}

impl CategoryReadHandlerTrait for CategoryReadHandler {
    async fn handle_get_all_categories(&self) -> Result<Vec<CategoryReadResponse>, AppError> {
        let db_result = Categories::find()
            .find_with_related(Tags)
            .all(self.db.as_ref())
            .await
            .map_err(|e| e.into())?;

        let response = db_result
            .iter()
            .map(|c_and_tags| {
                CategoryReadResponse::new(c_and_tags.0.to_owned(), c_and_tags.1.to_owned())
            })
            .collect::<Vec<CategoryReadResponse>>();

        // let category and tags
        Result::Ok(response)
    }

    async fn handle_get_category(&self, id: Uuid) -> Result<CategoryReadResponse, AppError> {
        let db_result = Categories::find_by_id(id)
            .find_with_related(Tags)
            .all(self.db.as_ref())
            .await
            .map_err(|e| e.into())?;

        if db_result.is_empty() {
            return Result::Err(AppError::NotFound);
        }

        let response = db_result
            .iter()
            .map(|c_and_tags| {
                CategoryReadResponse::new(c_and_tags.0.to_owned(), c_and_tags.1.to_owned())
            })
            .collect::<Vec<CategoryReadResponse>>()
            .first()
            .unwrap()
            .to_owned();

        // let category and tags
        Result::Ok(response)
    }

    async fn handle_get_with_filtering(
        &self,
        category_type: Option<CategoryType>,
    ) -> Result<Vec<CategoryReadResponse>, AppError> {
        let mut query = Categories::find();

        if category_type.is_some() {
            query =
                query.filter(Expr::col(Column::CategoryType).eq(category_type.unwrap().as_enum()));
        }

        let db_result = query
            .find_with_related(Tags)
            .order_by_asc(Column::DisplayName)
            .all(self.db.as_ref())
            .await
            .map_err(|e| e.into())?;

        let response = db_result
            .iter()
            .map(|c_and_tags| {
                CategoryReadResponse::new(c_and_tags.0.to_owned(), c_and_tags.1.to_owned())
            })
            .collect::<Vec<CategoryReadResponse>>();

        // let category and tags
        Result::Ok(response)
    }
}

#[cfg(test)]
mod tests {
    use std::sync::Arc;
    use test_helpers::{setup_test_space, ContainerAsyncPostgresEx};

    use crate::{
        commands::category::{
            create::create_handler::{CategoryCreateHandler, CategoryCreateHandlerTrait},
            read::category_read_handler::{CategoryReadHandler, CategoryReadHandlerTrait},
            test::fake_create_category_request_with_category_type,
        },
        entities::sea_orm_active_enums::CategoryType,
    };

    #[async_std::test]
    async fn handle_get_all_cartegory_testcase_01() {
        let test_space = setup_test_space().await;
        let conn = test_space.postgres.get_database_connection().await;
        let number_of_blogs = 5;
        let number_of_others = 5;

        let create_handler = CategoryCreateHandler {
            db: Arc::new(conn.clone()),
        };
        let read_handler = CategoryReadHandler { db: Arc::new(conn) };

        for i in 0..number_of_blogs {
            let _ = create_handler
                .handle_create_category_with_tags(
                    fake_create_category_request_with_category_type(i, CategoryType::Blog),
                    None,
                )
                .await;
        }

        for i in 0..number_of_others {
            let _ = create_handler
                .handle_create_category_with_tags(
                    fake_create_category_request_with_category_type(i, CategoryType::Other),
                    None,
                )
                .await;
        }

        let result = read_handler.handle_get_all_categories().await;
        match result {
            Ok(categories) => assert_eq!(categories.len(), number_of_blogs + number_of_others),
            _ => panic!("Failed to test"),
        }

        let blogs = read_handler
            .handle_get_with_filtering(Some(CategoryType::Blog))
            .await
            .unwrap();
        assert!(blogs.len() == number_of_blogs);

        let others = read_handler
            .handle_get_with_filtering(Some(CategoryType::Other))
            .await
            .unwrap();
        assert!(others.len() == number_of_others);
        // Clean up
    }
}
