use sea_orm::{DatabaseConnection, DbErr, EntityTrait};

use crate::{entities::categories::Model, Categories};

pub trait CategoryReadHandlerTrait {
    /// .
    ///
    /// # Errors
    ///
    /// This function will return an error if .
    fn handle_get_all_categories(
        &self,
    ) -> impl std::future::Future<Output = Result<Vec<Model>, DbErr>> + Send;
}

pub struct CategoryReadHandler {
    db: DatabaseConnection,
}

impl CategoryReadHandlerTrait for CategoryReadHandler {
    async fn handle_get_all_categories(&self) -> Result<Vec<Model>, DbErr> {
        let db_result = Categories::find().all(&self.db).await?;
        Result::Ok(db_result)
    }
}
