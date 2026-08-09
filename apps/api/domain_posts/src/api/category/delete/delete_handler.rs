use axum::{extract::State, response::IntoResponse, Extension, Json};
use sea_orm::sqlx::types::Uuid;
use tracing::instrument;

use crate::domain::response::{ApiResponseError, ApiResponseWith, AxumResponse};
use crate::handlers::category::delete::delete_handler::{
    CategoryDeleteHandler, CategoryDeleteHandlerTrait,
};
use domain_interface::{AuthenticatedActor, DomainContext};

#[instrument]
pub async fn api_delete_categories(
    State(ctx): State<DomainContext>,
    Extension(actor): Extension<AuthenticatedActor>,
    Json(body): Json<Vec<Uuid>>,
) -> impl IntoResponse {
    let handler = CategoryDeleteHandler {
        db: ctx.conn.clone(),
    };

    let result = handler
        .handle_delete_categories(body, actor.email.clone())
        .await;

    match result {
        Ok(inserted_id) => ApiResponseWith::new(inserted_id.to_string()).to_axum_response(),
        Err(e) => ApiResponseError::from(e).to_axum_response(),
    }
}
