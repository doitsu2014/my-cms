use std::{future::Future, sync::Arc};

use sea_orm::{DatabaseConnection, DatabaseTransaction, EntityTrait, Set};
use tracing::instrument;
use uuid::Uuid;

use crate::{
    commands::tag::read::read_handler::{TagReadHandler, TagReadHandlerTrait},
    common::{app_error::AppError, datetime_generator::generate_vietnam_now},
    entities::tags,
    Tags,
};

#[derive(Debug, Clone, serde::Serialize, serde::Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct CreateTagsResponse {
    pub new_tag_ids: Vec<Uuid>,
    pub existing_tag_ids: Vec<Uuid>,
}

pub trait TagCreateHandlerTrait {
    fn handle_create_tags_in_transaction(
        &self,
        tags: Vec<String>,
        actor_email: Option<String>,
        transaction: &DatabaseTransaction,
    ) -> impl Future<Output = Result<CreateTagsResponse, AppError>> + Send;
}

#[derive(Debug)]
pub struct TagCreateHandler {
    pub db: Arc<DatabaseConnection>,
}

impl TagCreateHandlerTrait for TagCreateHandler {
    #[instrument]
    async fn handle_create_tags_in_transaction(
        &self,
        tags: Vec<String>,
        actor_email: Option<String>,
        transaction: &DatabaseTransaction,
    ) -> Result<CreateTagsResponse, AppError> {
        let tag_read_handler = TagReadHandler {
            db: self.db.clone(),
        };

        let classifed_tags = tag_read_handler
            .handle_get_and_classify_tags_by_names(tags)
            .await
            .map_err(|e| e.into())?;

        let mut new_tag_ids: Vec<Uuid> = vec![];
        if !classifed_tags.new_tags.is_empty() {
            let new_tags = classifed_tags
                .new_tags
                .iter()
                .map(|tag| {
                    let id = Uuid::new_v4();
                    new_tag_ids.push(id);
                    tags::ActiveModel {
                        id: Set(id),
                        name: Set(tag.name.to_owned()),
                        slug: Set(tag.slug.to_owned()),
                        created_by: Set(actor_email.clone().unwrap_or("System".to_string())),
                        created_at: Set(generate_vietnam_now()),
                        ..Default::default()
                    }
                })
                .collect::<Vec<tags::ActiveModel>>();

            Tags::insert_many(new_tags)
                .exec(transaction)
                .await
                .map_err(|e| e.into())?;
        }

        let existing_tag_ids = classifed_tags
            .existing_tags
            .iter()
            .map(|tag| tag.id)
            .collect::<Vec<Uuid>>();

        Ok(CreateTagsResponse {
            new_tag_ids,
            existing_tag_ids,
        })
    }
}

#[cfg(test)]
mod tests {
    use sea_orm::TransactionTrait;
    use std::sync::Arc;
    use test_helpers::{setup_test_space, ContainerAsyncPostgresEx};
    use uuid::Uuid;

    use crate::{
        commands::tag::create::create_handler::{
            CreateTagsResponse, TagCreateHandler, TagCreateHandlerTrait,
        },
        common::app_error::AppError,
    };

    #[async_std::test]
    async fn handle_create_tags_in_transaction_test01() {
        let test_space = setup_test_space().await;
        let database = test_space.postgres.get_database_connection().await;
        let arc_conn = Arc::new(database);

        let requests = [
            ["a".to_string(), "b".to_string(), "c".to_string()],
            ["b".to_string(), "c".to_string(), "d".to_string()],
            ["d".to_string(), "e".to_string(), "f".to_string()],
            ["g".to_string(), "h".to_string(), "i".to_string()],
        ];

        let mut results: Vec<(Vec<Uuid>, Vec<Uuid>)> = vec![];
        for r in requests.clone() {
            let tag_create_handler = TagCreateHandler {
                db: arc_conn.clone(),
            };
            let result = arc_conn
                .as_ref()
                .transaction::<_, CreateTagsResponse, AppError>(|tx| {
                    Box::pin(async move {
                        let x = tag_create_handler
                            .handle_create_tags_in_transaction(
                                r.to_vec(),
                                Some("System".to_string()),
                                tx,
                            )
                            .await?;
                        Ok(x)
                    })
                })
                .await
                .unwrap();

            results.push((result.new_tag_ids, result.existing_tag_ids));
        }

        let mut count = 0;
        for (new_tag_ids, existing_tag_ids) in results {
            count += 1;
            // switch case
            match count {
                1 => {
                    assert_eq!(new_tag_ids.len(), 3);
                    assert_eq!(existing_tag_ids.len(), 0);
                }
                2 => {
                    assert_eq!(new_tag_ids.len(), 1);
                    assert_eq!(existing_tag_ids.len(), 2);
                }
                3 => {
                    assert_eq!(new_tag_ids.len(), 2);
                    assert_eq!(existing_tag_ids.len(), 1);
                }
                4 => {
                    assert_eq!(new_tag_ids.len(), 3);
                    assert_eq!(existing_tag_ids.len(), 0);
                }
                _ => {}
            }
        }
    }
}
