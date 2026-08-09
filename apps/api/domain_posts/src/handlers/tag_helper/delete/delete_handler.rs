use std::sync::Arc;

use sea_orm::{ColumnTrait, DatabaseConnection, DatabaseTransaction, EntityTrait, QueryFilter};
use tracing::{info, instrument};
use uuid::Uuid;

use crate::domain::error::AppError;
use crate::entities::{tags, Tags};

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
            .await?;

        info!(
            "{} tags deleted by {}",
            result.rows_affected,
            actor_email.unwrap_or_default()
        );

        Ok(result.rows_affected)
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
            .await?;

        info!(
            "{} tags deleted by {}",
            result.rows_affected,
            actor_email.unwrap_or_default()
        );

        Ok(result.rows_affected)
    }
}

#[cfg(test)]
mod tests {
    // End-to-end behavior (create → read → delete → re-read) is covered
    // by the existing integration test in `tag_helper::read::read_handler::tests`
    // (which is the canonical test home for the post-domain tag CRUD lifecycle
    // and replaces the legacy `application_core::commands::tag::read::read_handler::tests`).
    // Direct mock-database unit tests can be added here in a follow-up
    // change without touching the legacy `commands::tag` module.
}
