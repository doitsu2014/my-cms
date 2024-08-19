use sea_orm::{prelude::DateTimeWithTimeZone, DatabaseConnection, EntityTrait};
use serde::{Deserialize, Serialize};
use std::sync::Arc;
use tracing::instrument;
use uuid::Uuid;

use crate::{
    common::app_error::AppError,
    entities::{posts::Model, tags},
    Posts, Tags,
};

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct PostReadResponse {
    pub id: Uuid,
    pub title: String,
    pub preview_content: Option<String>,
    pub content: String,
    pub slug: String,
    pub published: bool,
    pub created_by: String,
    pub created_at: DateTimeWithTimeZone,
    pub last_modified_by: Option<String>,
    pub last_modified_at: Option<DateTimeWithTimeZone>,
    pub category_id: Uuid,
    pub row_version: i32,
    pub tags: Vec<tags::Model>,
    pub tag_names: Vec<String>,
    pub thumbnail_paths: Vec<String>,
}

impl PostReadResponse {
    fn new(post: Model, tags: Vec<tags::Model>) -> Self {
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
        }
    }
}

pub trait PostReadHandlerTrait {
    fn handle_get_all_posts(
        &self,
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
                PostReadResponse::new(p_and_tags.0.to_owned(), p_and_tags.1.to_owned())
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

        let response = db_result
            .iter()
            .map(|p_and_tags| {
                PostReadResponse::new(p_and_tags.0.to_owned(), p_and_tags.1.to_owned())
            })
            .collect::<Vec<PostReadResponse>>()
            .first()
            .unwrap()
            .to_owned();

        Result::Ok(response)
    }
}
