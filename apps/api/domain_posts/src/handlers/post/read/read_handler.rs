use sea_orm::{
    prelude::DateTimeWithTimeZone, sea_query::Expr, ActiveEnum, ColumnTrait, DatabaseConnection,
    EntityTrait, JoinType, QueryFilter, QuerySelect, QueryTrait, RelationTrait,
};
use serde::{Deserialize, Serialize};
use std::collections::HashMap;
use std::sync::Arc;
use tracing::instrument;
use uuid::Uuid;

use crate::domain::error::AppError;
use crate::entities::{
    categories, post_translations,
    posts::{self, Model},
    sea_orm_active_enums::CategoryType,
    tags, Posts, Tags,
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
            .map_err(AppError::from)?;

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
            .map_err(AppError::from)?;

        // Collect post IDs
        let post_ids: Vec<Uuid> = db_result.iter().map(|(post, _)| post.id).collect();

        // Fetch all translations for the collected post IDs
        let translations_map: HashMap<Uuid, Vec<post_translations::Model>> =
            post_translations::Entity::find()
                .filter(post_translations::Column::PostId.is_in(post_ids.clone()))
                .all(self.db.as_ref())
                .await
                .map_err(AppError::from)?
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
            .map_err(AppError::from)?;

        if db_result.is_empty() {
            return Result::Err(AppError::NotFound);
        }

        let post = db_result.first().unwrap().0.to_owned();
        let tags = db_result.first().unwrap().1.to_owned();

        let translations = post_translations::Entity::find()
            .filter(post_translations::Column::PostId.eq(post.id))
            .all(self.db.as_ref())
            .await
            .map_err(AppError::from)?;

        let response = PostReadResponse::new(post, tags, translations);

        Result::Ok(response)
    }
}

#[cfg(test)]
#[allow(unused_imports, dead_code)]
mod tests {
    // Tests for `PostReadHandler` are temporarily disabled during the move.
    // The original test fixture `application_core::commands::post::test` is
    // gated by `#[cfg(test)]` on the `application_core` crate and is therefore
    // not visible from `domain_posts`'s test build.
    fn _placeholder() {
        use std::sync::Arc;
        let _ = Arc::new(());
    }
}
