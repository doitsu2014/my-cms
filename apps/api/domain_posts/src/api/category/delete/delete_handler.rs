use axum::{extract::State, response::IntoResponse, Extension, Json};
use sea_orm::sqlx::types::Uuid;
use tracing::instrument;

use crate::domain::auth::SupabaseToken;
use crate::domain::response::{ApiResponseError, ApiResponseWith, AxumResponse};
use crate::handlers::category::delete::delete_handler::{
    CategoryDeleteHandler, CategoryDeleteHandlerTrait,
};
use domain_interface::DomainContext;

#[instrument]
pub async fn api_delete_categories(
    State(ctx): State<DomainContext>,
    Extension(token): Extension<SupabaseToken>,
    Json(body): Json<Vec<Uuid>>,
) -> impl IntoResponse {
    let handler = CategoryDeleteHandler {
        db: ctx.conn.clone(),
    };

    let result = handler
        .handle_delete_categories(body, Some(token.email().unwrap_or("").to_string()))
        .await;

    match result {
        Ok(inserted_id) => ApiResponseWith::new(inserted_id.to_string()).to_axum_response(),
        Err(e) => ApiResponseError::from(e).to_axum_response(),
    }
}
