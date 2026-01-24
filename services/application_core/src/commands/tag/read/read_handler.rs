use std::{future::Future, sync::Arc};

use sea_orm::{sea_query::Expr, DatabaseConnection, DbErr, EntityTrait, QueryFilter};
use tracing::instrument;

use crate::{
    common::datetime_generator::generate_vietnam_now,
    entities::tags::{Column, Model},
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
        let mut new_tags: Vec<Model> = vec![];
        for name in names {
            let slug = name.to_slug();
            if tags.iter().all(|tag| tag.name != name) {
                let tag = Model {
                    id: Default::default(),
                    name: name.to_string(),
                    slug: slug.to_string(),
                    created_by: "".to_string(),
                    created_at: generate_vietnam_now(),
                    last_modified_by: None,
                    last_modified_at: None,
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

#[cfg(test)]
mod tests {
    use sea_orm::TransactionTrait;
    use seaography::itertools::Itertools;
    use std::sync::Arc;
    use test_helpers::{setup_test_space, ContainerAsyncPostgresEx};

    use crate::{
        commands::tag::{
            create::create_handler::{CreateTagsResponse, TagCreateHandler, TagCreateHandlerTrait},
            delete::delete_handler::{TagDeleteHandler, TagDeleteHandlerTrait},
        },
        common::app_error::AppError,
    };

    use super::{TagReadHandler, TagReadHandlerTrait};

    #[async_std::test]
    async fn handle_read_tags_test01() {
        let test_space = setup_test_space().await;
        let database = test_space.postgres.get_database_connection().await;

        let arc_conn = Arc::new(database);
        let tag_create_handler = TagCreateHandler {
            db: arc_conn.clone(),
        };

        let tag_delete_handler = TagDeleteHandler {
            db: arc_conn.clone(),
        };

        let tag_read_handler = TagReadHandler {
            db: arc_conn.clone(),
        };

        let create = vec!["a".to_string(), "b".to_string(), "c".to_string()];
        let result = arc_conn
            .as_ref()
            .transaction::<_, CreateTagsResponse, AppError>(|tx| {
                Box::pin(async move {
                    let x = tag_create_handler
                        .handle_create_tags_in_transaction(create, Some("System".to_string()), tx)
                        .await?;
                    Ok(x)
                })
            })
            .await
            .unwrap();

        let created_ids = result.new_tag_ids;

        let create = ["a".to_string(), "b".to_string(), "c".to_string()];
        let new_tags = ["e".to_string(), "ddd".to_string()];
        let combined = create
            .iter()
            .chain(new_tags.iter())
            .map(|x| x.to_string())
            .collect();

        let assertions = tag_read_handler
            .handle_get_and_classify_tags_by_names(combined)
            .await
            .unwrap();

        let assertion_new_tags = assertions.new_tags;
        let assertion_existing_tags = assertions.existing_tags;

        assert_eq!(assertion_new_tags.len(), 2);
        assert!(assertion_new_tags
            .iter()
            .all(|e| new_tags.contains(&e.name)));

        assert_eq!(assertion_existing_tags.len(), 3);
        assert!(assertion_existing_tags
            .iter()
            .all(|e| create.contains(&e.name)));

        arc_conn
            .as_ref()
            .transaction::<_, u64, AppError>(|tx| {
                Box::pin(async move {
                    let x = tag_delete_handler
                        .handle_delete_tags_in_transaction(
                            created_ids,
                            Some("System".to_string()),
                            tx,
                        )
                        .await?;
                    Ok(x)
                })
            })
            .await
            .unwrap();

        let create = ["a".to_string(), "b".to_string(), "c".to_string()];
        let new_tags = ["e".to_string(), "ddd".to_string()];
        let combined = create
            .iter()
            .chain(new_tags.iter())
            .map(|x| x.to_string())
            .collect_vec();

        let assertions = tag_read_handler
            .handle_get_and_classify_tags_by_names(combined.clone())
            .await
            .unwrap();

        let assertion_new_tags = assertions.new_tags;
        let assertion_existing_tags = assertions.existing_tags;

        assert_eq!(assertion_new_tags.len(), 5);
        assert!(assertion_new_tags
            .iter()
            .all(|e| combined.contains(&e.name)));

        assert_eq!(assertion_existing_tags.len(), 0);
    }
}
