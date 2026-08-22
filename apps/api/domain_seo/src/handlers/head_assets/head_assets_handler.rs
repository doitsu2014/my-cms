use chrono::Utc;
use domain_interface::AuthenticatedActor;
use sea_orm::{
    sea_query::Expr, ActiveModelTrait, ColumnTrait, DatabaseConnection, EntityTrait, QueryFilter,
    QueryOrder, Set,
};
use serde::{Deserialize, Serialize};
use std::sync::Arc;
use tracing::{info, instrument};
use uuid::Uuid;

use crate::{
    domain::error::AppError,
    entities::seo_head_assets::{ActiveModel, Column, Entity, Model},
    handlers::head_assets::head_asset_validation::{validate_source, MAX_LABEL_CHARS},
};

#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct CreateHeadAssetRequest {
    pub label: String,
    pub html: String,
    pub enabled: bool,
    pub sort_order: i32,
}
#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct UpdateHeadAssetRequest {
    pub label: String,
    pub html: String,
    pub enabled: bool,
    pub sort_order: i32,
    pub row_version: i32,
}
#[derive(Debug, Clone, Serialize)]
#[serde(rename_all = "camelCase")]
pub struct HeadAsset {
    pub id: Uuid,
    pub label: String,
    pub html: String,
    pub enabled: bool,
    pub sort_order: i32,
    pub row_version: i32,
    pub created_at: chrono::DateTime<chrono::FixedOffset>,
    pub updated_at: chrono::DateTime<chrono::FixedOffset>,
    pub created_by: String,
    pub updated_by: String,
}
#[derive(Debug, Clone, Serialize)]
#[serde(rename_all = "camelCase")]
pub struct PublicHeadAsset {
    pub id: Uuid,
    pub label: String,
    pub html: String,
    pub sort_order: i32,
    pub updated_at: chrono::DateTime<chrono::FixedOffset>,
}
impl From<Model> for HeadAsset {
    fn from(m: Model) -> Self {
        Self {
            id: m.id,
            label: m.label,
            html: m.html,
            enabled: m.enabled,
            sort_order: m.sort_order,
            row_version: m.row_version,
            created_at: m.created_at,
            updated_at: m.updated_at,
            created_by: m.created_by,
            updated_by: m.updated_by,
        }
    }
}
impl From<Model> for PublicHeadAsset {
    fn from(m: Model) -> Self {
        Self {
            id: m.id,
            label: m.label,
            html: m.html,
            sort_order: m.sort_order,
            updated_at: m.updated_at,
        }
    }
}

#[derive(Debug, Clone)]
pub struct HeadAssetHandler {
    pub db: Arc<DatabaseConnection>,
}
impl HeadAssetHandler {
    fn validate_fields(label: &str, html: &str, sort_order: i32) -> Result<String, AppError> {
        let label = label.trim();
        if label.is_empty() || label.chars().count() > MAX_LABEL_CHARS {
            return Err(AppError::Validation(
                "label".into(),
                "label must be nonblank and at most 128 characters".into(),
            ));
        }
        if sort_order <= 0 {
            return Err(AppError::Validation(
                "sortOrder".into(),
                "sortOrder must be positive".into(),
            ));
        }
        validate_source(html).map_err(|message| AppError::Validation("html".into(), message))?;
        Ok(label.to_string())
    }
    #[instrument(skip(self, request), fields(asset_label = %request.label, html = tracing::field::Empty))]
    pub async fn create(
        &self,
        request: CreateHeadAssetRequest,
        actor: &AuthenticatedActor,
    ) -> Result<HeadAsset, AppError> {
        let label = Self::validate_fields(&request.label, &request.html, request.sort_order)?;
        let now = Utc::now().fixed_offset();
        let id = Uuid::new_v4();
        let model = ActiveModel {
            id: Set(id),
            label: Set(label),
            html: Set(request.html),
            enabled: Set(request.enabled),
            sort_order: Set(request.sort_order),
            row_version: Set(1),
            created_at: Set(now),
            updated_at: Set(now),
            created_by: Set(actor.user_id.clone()),
            updated_by: Set(actor.user_id.clone()),
        };
        let saved = model.insert(self.db.as_ref()).await.map_err(map_db_error)?;
        info!(action="create", asset_id=%id, actor_id=%actor.user_id, enabled=saved.enabled, sort_order=saved.sort_order, "seo head asset mutation");
        Ok(saved.into())
    }
    pub async fn list(&self) -> Result<Vec<HeadAsset>, AppError> {
        Ok(Entity::find()
            .order_by_asc(Column::SortOrder)
            .order_by_asc(Column::Id)
            .all(self.db.as_ref())
            .await?
            .into_iter()
            .map(Into::into)
            .collect())
    }
    pub async fn get(&self, id: Uuid) -> Result<HeadAsset, AppError> {
        Entity::find_by_id(id)
            .one(self.db.as_ref())
            .await?
            .map(Into::into)
            .ok_or(AppError::NotFound)
    }
    pub async fn public_list(&self) -> Result<Vec<PublicHeadAsset>, AppError> {
        Ok(Entity::find()
            .filter(Column::Enabled.eq(true))
            .order_by_asc(Column::SortOrder)
            .order_by_asc(Column::Id)
            .all(self.db.as_ref())
            .await?
            .into_iter()
            .map(Into::into)
            .collect())
    }
    #[instrument(skip(self, request), fields(asset_id=%id))]
    pub async fn update(
        &self,
        id: Uuid,
        request: UpdateHeadAssetRequest,
        actor: &AuthenticatedActor,
    ) -> Result<HeadAsset, AppError> {
        let label = Self::validate_fields(&request.label, &request.html, request.sort_order)?;
        let now = Utc::now().fixed_offset();
        let result = Entity::update_many()
            .col_expr(Column::Label, Expr::value(label))
            .col_expr(Column::Html, Expr::value(request.html))
            .col_expr(Column::Enabled, Expr::value(request.enabled))
            .col_expr(Column::SortOrder, Expr::value(request.sort_order))
            .col_expr(Column::UpdatedAt, Expr::value(now))
            .col_expr(Column::UpdatedBy, Expr::value(actor.user_id.clone()))
            .col_expr(Column::RowVersion, Expr::col(Column::RowVersion).add(1))
            .filter(Column::Id.eq(id))
            .filter(Column::RowVersion.eq(request.row_version))
            .exec(self.db.as_ref())
            .await
            .map_err(map_db_error)?;
        if result.rows_affected == 0 {
            if Entity::find_by_id(id)
                .one(self.db.as_ref())
                .await?
                .is_some()
            {
                return Err(AppError::ConcurrencyOptimistic(
                    "rowVersion is stale".into(),
                ));
            }
            return Err(AppError::NotFound);
        }
        let saved = self.get(id).await?;
        info!(action="update", asset_id=%id, actor_id=%actor.user_id, enabled=saved.enabled, sort_order=saved.sort_order, "seo head asset mutation");
        Ok(saved)
    }
    pub async fn delete(&self, id: Uuid, actor: &AuthenticatedActor) -> Result<(), AppError> {
        let result = Entity::delete_by_id(id).exec(self.db.as_ref()).await?;
        if result.rows_affected == 0 {
            return Err(AppError::NotFound);
        }
        info!(action="delete", asset_id=%id, actor_id=%actor.user_id, "seo head asset mutation");
        Ok(())
    }
}
fn map_db_error(error: sea_orm::DbErr) -> AppError {
    let text = error.to_string();
    if text.to_ascii_lowercase().contains("unique")
        || text.to_ascii_lowercase().contains("duplicate")
    {
        AppError::Conflict("label already exists".into())
    } else {
        AppError::Db(error)
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    fn handler() -> HeadAssetHandler {
        HeadAssetHandler {
            db: Arc::new(sea_orm::DatabaseConnection::Disconnected),
        }
    }
    fn actor() -> AuthenticatedActor {
        AuthenticatedActor {
            user_id: "administrator-id".into(),
            email: None,
            primary_role: "authenticated".into(),
            app_roles: vec!["my-headless-cms-administrator".into()],
        }
    }

    #[tokio::test]
    async fn create_rejects_invalid_fields_before_database_access() {
        let result = handler()
            .create(
                CreateHeadAssetRequest {
                    label: " ".into(),
                    html: "<script></script>".into(),
                    enabled: true,
                    sort_order: 1,
                },
                &actor(),
            )
            .await;
        assert!(matches!(result, Err(AppError::Validation(field, _)) if field == "label"));
    }

    #[test]
    fn public_model_omits_administration_fields() {
        let model = PublicHeadAsset {
            id: Uuid::nil(),
            label: "x".into(),
            html: "<meta name=\"x\" content=\"y\">".into(),
            sort_order: 1,
            updated_at: Utc::now().fixed_offset(),
        };
        let json = serde_json::to_value(model).expect("serialize");
        assert!(json.get("enabled").is_none());
        assert!(json.get("rowVersion").is_none());
        assert!(json.get("createdBy").is_none());
    }
}
