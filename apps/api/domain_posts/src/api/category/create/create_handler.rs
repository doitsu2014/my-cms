use axum::{extract::State, response::IntoResponse, Extension, Json};
use tracing::instrument;

use crate::domain::response::{ApiResponseError, ApiResponseWith, AxumResponse};
use crate::handlers::category::create::{
    create_handler::{CategoryCreateHandler, CategoryCreateHandlerTrait},
    create_request::CreateCategoryRequest,
};
use domain_interface::{AuthenticatedActor, DomainContext};

#[instrument]
pub async fn api_create_category_with_tags(
    State(ctx): State<DomainContext>,
    Extension(actor): Extension<AuthenticatedActor>,
    Json(body): Json<CreateCategoryRequest>,
) -> impl IntoResponse {
    let handler = CategoryCreateHandler {
        db: ctx.conn.clone(),
    };

    let result = handler
        .handle_create_category_with_tags(body, actor.email.clone())
        .await;

    match result {
        Ok(inserted_id) => ApiResponseWith::new(inserted_id.to_string()).to_axum_response(),
        Err(e) => ApiResponseError::from(e).to_axum_response(),
    }
}
