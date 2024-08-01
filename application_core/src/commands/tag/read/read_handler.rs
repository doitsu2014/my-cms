use std::{future::Future, sync::Arc};

use sea_orm::{sea_query::Expr, DatabaseConnection, DbErr, EntityTrait, QueryFilter, Set};
use tracing::instrument;

use crate::{
    entities::tags::{ActiveModel, Column, Model},
    StringExtension, Tags,
};

use super::read_response::GetAndClassifyTagCommandResponse;

pub trait TagReadHandlerTrait {
    fn handle_get_tags_by_names(
        &self,
        names: Vec<String>,
    ) -> impl Future<Output = Result<Vec<Model>, DbErr>> + Send;
    fn handle_get_tags_by_slugs(
        &self,
        slugs: Vec<String>,
    ) -> impl Future<Output = Result<Vec<Model>, DbErr>> + Send;
    fn handle_get_and_classify_tags_by_names(
        &self,
        names: Vec<String>,
    ) -> impl Future<Output = Result<GetAndClassifyTagCommandResponse, DbErr>> + Send;
}

#[derive(Debug)]
pub struct TagReadHandler {
    pub db: Arc<DatabaseConnection>,
}

impl TagReadHandlerTrait for TagReadHandler {
    #[instrument]
    async fn handle_get_tags_by_names(&self, names: Vec<String>) -> Result<Vec<Model>, DbErr> {
        // Get all tags with names
        let tags = Tags::find()
            .filter(Expr::col(Column::Name).is_in(names))
            .all(self.db.as_ref())
            .await?;
        Ok(tags)
    }

    #[instrument]
    async fn handle_get_tags_by_slugs(&self, slugs: Vec<String>) -> Result<Vec<Model>, DbErr> {
        // Get all tags with names
        let tags = Tags::find()
            .filter(Expr::col(Column::Slug).is_in(slugs))
            .all(self.db.as_ref())
            .await?;
        Ok(tags)
    }

    #[instrument]
    async fn handle_get_and_classify_tags_by_names(
        &self,
        names: Vec<String>,
    ) -> Result<GetAndClassifyTagCommandResponse, DbErr> {
        if names.is_empty() {
            return Ok(GetAndClassifyTagCommandResponse {
                new_tags: vec![],
                existing_tags: vec![],
            });
        }

        // Get all tags with names
        let tags = self.handle_get_tags_by_names(names.clone()).await?;
        let mut new_tags: Vec<ActiveModel> = vec![];
        for name in names {
            let slug = name.to_slug();
            if tags.iter().all(|tag| tag.name != name) {
                let tag = ActiveModel {
                    name: Set(name.clone()),
                    slug: Set(slug),
                    ..Default::default()
                };
                new_tags.push(tag);
            }
        }
        Ok(GetAndClassifyTagCommandResponse {
            new_tags,
            existing_tags: tags,
        })
    }
}
