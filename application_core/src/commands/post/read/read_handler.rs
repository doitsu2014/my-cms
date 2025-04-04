use sea_orm::{
    prelude::DateTimeWithTimeZone, sea_query::Expr, ActiveEnum, ColumnTrait, DatabaseConnection, EntityTrait, JoinType, QueryFilter, QuerySelect, QueryTrait, RelationTrait
};
use serde::{Deserialize, Serialize};
use std::sync::Arc;
use std::collections::HashMap;
use tracing::instrument;
use uuid::Uuid;

use crate::{
    common::app_error::AppError,
    entities::{
        categories,
        posts::{self, Model},
        sea_orm_active_enums::CategoryType,
        tags,
        post_translations,
    },
    Posts, Tags,
};

#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct PostReadResponse {
    pub id: Uuid,
    pub title: String,
    pub preview_content: Option<String>,
    pub content: String,
    pub slug: String,
    pub thumbnail_paths: Vec<String>,
    pub published: bool,
    pub created_by: String,
    pub created_at: DateTimeWithTimeZone,
    pub last_modified_by: Option<String>,
    pub last_modified_at: Option<DateTimeWithTimeZone>,
    pub category_id: Uuid,
    pub row_version: i32,
    pub tags: Vec<tags::Model>,
    pub tag_names: Vec<String>,
    pub translations: Vec<post_translations::Model>,
}

impl PostReadResponse {
    fn new(
        post: Model,
        tags: Vec<tags::Model>,
        translations: Vec<post_translations::Model>,
    ) -> Self {
        let tag_names = tags
            .iter()
            .map(|tag| tag.name.to_owned())
            .collect::<Vec<String>>();

        PostReadResponse {
            id: post.id,
            title: post.title,
            preview_content: post.preview_content,
            content: post.content,
            slug: post.slug,
            published: post.published,
            created_by: post.created_by,
            created_at: post.created_at,
            last_modified_by: post.last_modified_by,
            last_modified_at: post.last_modified_at,
            category_id: post.category_id,
            row_version: post.row_version,
            thumbnail_paths: post.thumbnail_paths,
            tags,
            tag_names,
            translations,
        }
    }
}

pub trait PostReadHandlerTrait {
    fn handle_get_all_posts(
        &self,
    ) -> impl std::future::Future<Output = Result<Vec<PostReadResponse>, AppError>>;

    fn handle_get_posts_with_filtering(
        &self,
        category_type: Option<CategoryType>,
        published: Option<bool>,
    ) -> impl std::future::Future<Output = Result<Vec<PostReadResponse>, AppError>>;

    fn handle_get_post(
        &self,
        id: Uuid,
    ) -> impl std::future::Future<Output = Result<PostReadResponse, AppError>>;
}

#[derive(Debug)]
pub struct PostReadHandler {
    pub db: Arc<DatabaseConnection>,
}

impl PostReadHandlerTrait for PostReadHandler {
    #[instrument]
    async fn handle_get_all_posts(&self) -> Result<Vec<PostReadResponse>, AppError> {
        let db_result = Posts::find()
            .find_with_related(Tags)
            .all(self.db.as_ref())
            .await
            .map_err(|e| e.into())?;

        let response = db_result
            .iter()
            .map(|p_and_tags| {
                PostReadResponse::new(p_and_tags.0.to_owned(), p_and_tags.1.to_owned(), vec![])
            })
            .collect::<Vec<PostReadResponse>>();

        Ok(response)
    }

    #[instrument]
    async fn handle_get_posts_with_filtering(
        &self,
        category_type: Option<CategoryType>,
        published: Option<bool>,
    ) -> Result<Vec<PostReadResponse>, AppError> {
        // Get Posts with Categories and Tags
        let db_result = Posts::find()
            .join(JoinType::LeftJoin, posts::Relation::Categories.def())
            .apply_if(category_type, |query, v| {
                query.filter(Expr::col(categories::Column::CategoryType).eq(v.as_enum()))
            })
            .apply_if(published, |query, v| {
                query.filter(Expr::col(posts::Column::Published).eq(v))
            })
            .find_with_related(Tags)
            .all(self.db.as_ref())
            .await
            .map_err(|e| e.into())?;

        // Collect post IDs
        let post_ids: Vec<Uuid> = db_result.iter().map(|(post, _)| post.id).collect();

        // Fetch all translations for the collected post IDs
        let translations_map: HashMap<Uuid, Vec<post_translations::Model>> = post_translations::Entity::find()
            .filter(post_translations::Column::PostId.is_in(post_ids.clone()))
            .all(self.db.as_ref())
            .await
            .map_err(|e| e.into())?
            .into_iter()
            .fold(HashMap::new(), |mut acc, translation| {
                acc.entry(translation.post_id)
                    .or_insert_with(Vec::new)
                    .push(translation);
                acc
            });

        // Build the response
        let response = db_result
            .into_iter()
            .map(|(post, tags)| {
                let post_translations = translations_map.get(&post.id).cloned().unwrap_or_default();
                PostReadResponse::new(post, tags, post_translations)
            })
            .collect::<Vec<PostReadResponse>>();

        Ok(response)
    }

    #[instrument]
    async fn handle_get_post(&self, id: Uuid) -> Result<PostReadResponse, AppError> {
        let db_result = Posts::find_by_id(id)
            .find_with_related(Tags)
            .all(self.db.as_ref())
            .await
            .map_err(|e| e.into())?;

        if db_result.is_empty() {
            return Result::Err(AppError::NotFound);
        }

        let post = db_result.first().unwrap().0.to_owned();
        let tags = db_result.first().unwrap().1.to_owned();

        let translations = post_translations::Entity::find()
            .filter(post_translations::Column::PostId.eq(post.id))
            .all(self.db.as_ref())
            .await
            .map_err(|e| e.into())?;

        let response = PostReadResponse::new(post, tags, translations);

        Result::Ok(response)
    }
}

#[cfg(test)]
mod tests {
    use std::sync::Arc;
    use test_helpers::{setup_test_space, ContainerAsyncPostgresEx};

    use crate::{
        commands::{
            category::{
                create::create_handler::{CategoryCreateHandler, CategoryCreateHandlerTrait},
                test::fake_create_category_request_with_category_type,
            },
            post::{
                create::create_handler::{PostCreateHandler, PostCreateHandlerTrait},
                read::read_handler::{PostReadHandler, PostReadHandlerTrait},
                test::fake_create_post_request,
            },
        },
        entities::sea_orm_active_enums::CategoryType,
    };

    #[async_std::test]
    async fn handle_get_posts_with_filtering_test01() {
        let test_space = setup_test_space().await;
        let database = test_space.postgres.get_database_connection().await;

        let arc_conn = Arc::new(database);
        let category_create_handler = CategoryCreateHandler {
            db: arc_conn.clone(),
        };
        let post_create_handler = PostCreateHandler {
            db: arc_conn.clone(),
        };
        let post_read_handler = PostReadHandler {
            db: arc_conn.clone(),
        };

        let create_category_blog_request =
            fake_create_category_request_with_category_type(5, CategoryType::Blog);
        let create_category_other_request =
            fake_create_category_request_with_category_type(5, CategoryType::Other);
        let created_blog_category_id = category_create_handler
            .handle_create_category_with_tags(create_category_blog_request, None)
            .await
            .unwrap();
        let created_other_category_id = category_create_handler
            .handle_create_category_with_tags(create_category_other_request, None)
            .await
            .unwrap();

        let create_blog_post_request = fake_create_post_request(created_blog_category_id, 5);
        let blog_result = post_create_handler
            .handle_create_post(create_blog_post_request, None)
            .await
            .unwrap();
        let create_other_post_request = fake_create_post_request(created_other_category_id, 5);
        let other_result = post_create_handler
            .handle_create_post(create_other_post_request, None)
            .await
            .unwrap();

        let db_blog_posts = post_read_handler
            .handle_get_posts_with_filtering(Some(CategoryType::Blog), None)
            .await
            .unwrap();
        let db_other_posts = post_read_handler
            .handle_get_posts_with_filtering(Some(CategoryType::Other), None)
            .await
            .unwrap();
        let all_posts = post_read_handler.handle_get_all_posts().await.unwrap();
        let all_posts_from_no_filtering = post_read_handler
            .handle_get_posts_with_filtering(None, None)
            .await
            .unwrap();

        assert_eq!(db_blog_posts.len(), 1);
        assert_eq!(db_blog_posts.first().unwrap().id, blog_result);
        assert_eq!(db_other_posts.len(), 1);
        assert_eq!(db_other_posts.first().unwrap().id, other_result);
        assert_eq!(all_posts_from_no_filtering.len(), all_posts.len());
    }
}
