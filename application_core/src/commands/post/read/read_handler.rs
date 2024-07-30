use sea_orm::{DatabaseConnection, DbErr, EntityTrait};
use std::sync::Arc;
use tracing::instrument;

use crate::{entities::posts::Model, Posts};

pub trait PostReadHandlerTrait {
    fn handle_get_all_posts(&self) -> impl std::future::Future<Output = Result<Vec<Model>, DbErr>>;
}

#[derive(Debug)]
pub struct PostReadHandler {
    pub db: Arc<DatabaseConnection>,
}

impl PostReadHandlerTrait for PostReadHandler {
    #[instrument]
    async fn handle_get_all_posts(&self) -> Result<Vec<Model>, DbErr> {
        let db_result = Posts::find().all(self.db.as_ref()).await?;
        Result::Ok(db_result)
    }
}
