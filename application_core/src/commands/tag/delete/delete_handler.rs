use std::sync::Arc;

use crate::{common::app_error::AppError, entities::tags, Tags};
use sea_orm::{ColumnTrait, DatabaseConnection, DatabaseTransaction, EntityTrait, QueryFilter};
use tracing::{info, instrument};
use uuid::Uuid;

pub trait TagDeleteHandlerTrait {
    fn handle_delete_tags(
        &self,
        ids: Vec<Uuid>,
        actor_email: Option<String>,
    ) -> impl std::future::Future<Output = Result<u64, AppError>>;

    fn handle_delete_tags_in_transaction(
        &self,
        ids: Vec<Uuid>,
        actor_email: Option<String>,
        transaction: &DatabaseTransaction,
    ) -> impl std::future::Future<Output = Result<u64, AppError>>;
}

#[derive(Debug)]
pub struct TagDeleteHandler {
    pub db: Arc<DatabaseConnection>,
}

impl TagDeleteHandlerTrait for TagDeleteHandler {
    #[instrument]
    async fn handle_delete_tags(
        &self,
        ids: Vec<Uuid>,
        actor_email: Option<String>,
    ) -> Result<u64, AppError> {
        let result = Tags::delete_many()
            .filter(tags::Column::Id.is_in(ids))
            .exec(self.db.as_ref())
            .await
            .map_err(|e| e.into())?;

        info!(
            "{} tags deleted by {}",
            result.rows_affected,
            actor_email.unwrap_or_default()
        );

        return Ok(result.rows_affected);
    }

    #[instrument]
    async fn handle_delete_tags_in_transaction(
        &self,
        ids: Vec<Uuid>,
        actor_email: Option<String>,
        transaction: &DatabaseTransaction,
    ) -> Result<u64, AppError> {
        let result = Tags::delete_many()
            .filter(tags::Column::Id.is_in(ids))
            .exec(transaction)
            .await
            .map_err(|e| e.into())?;

        info!(
            "{} tags deleted by {}",
            result.rows_affected,
            actor_email.unwrap_or_default()
        );

        return Ok(result.rows_affected);
    }
}
