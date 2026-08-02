use axum::{extract::State, response::IntoResponse, Extension, Json};
use tracing::instrument;

use crate::domain::auth::SupabaseToken;
use crate::domain::response::{ApiResponseError, ApiResponseWith, AxumResponse};
use crate::handlers::category::modify::{
    modify_handler::{CategoryModifyHandler, CategoryModifyHandlerTrait},
    modify_request::ModifyCategoryRequest,
};
use domain_interface::DomainContext;

#[instrument]
pub async fn api_modify_category(
    State(ctx): State<DomainContext>,
    Extension(token): Extension<SupabaseToken>,
    Json(body): Json<ModifyCategoryRequest>,
) -> impl IntoResponse {
    let handler = CategoryModifyHandler {
        db: ctx.conn.clone(),
    };

    let result = handler
        .handle_modify_category(body, Some(token.email().unwrap_or("").to_string()))
        .await;

    match result {
        Ok(inserted_id) => ApiResponseWith::new(inserted_id.to_string()).to_axum_response(),
        Err(e) => ApiResponseError::from(e).to_axum_response(),
    }
}
