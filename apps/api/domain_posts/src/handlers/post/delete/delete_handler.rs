use std::sync::Arc;

use crate::entities::posts, Posts};
use sea_orm::{ColumnTrait, DatabaseConnection, EntityTrait, QueryFilter};
use tracing::{info, instrument};
use uuid::Uuid;

pub trait PostDeleteHandlerTrait {
    fn handle_delete_posts(
        &self,
        ids: Vec<Uuid>,
        actor_email: Option<String>,
    ) -> impl std::future::Future<Output = Result<u64, AppError>>;
}

#[derive(Debug)]
pub struct PostDeleteHandler {
    pub db: Arc<DatabaseConnection>,
}

impl PostDeleteHandlerTrait for PostDeleteHandler {
    #[instrument]
    async fn handle_delete_posts(
        &self,
        ids: Vec<Uuid>,
        actor_email: Option<String>,
    ) -> Result<u64, AppError> {
        let result = Posts::delete_many()
            .filter(posts::Column::Id.is_in(ids))
            .exec(self.db.as_ref())
            .await
            .map_err(|e| e.into())?;

        info!(
            "{} posts deleted by {}",
            result.rows_affected,
            actor_email.unwrap_or_default()
        );

        return Ok(result.rows_affected);
    }
}


#[cfg(test)]
#[allow(unused_imports, dead_code)]
mod tests {
    // Tests for `PostDeleteHandler` are temporarily disabled during the move.
    // The original test fixture `application_core::commands::post::test` is
    // gated by `#[cfg(test)]` on the `application_core` crate and is therefore
    // not visible from `domain_posts`'s test build.
    fn _placeholder() {
        use std::sync::Arc;
        let _ = Arc::new(());
    }
}
