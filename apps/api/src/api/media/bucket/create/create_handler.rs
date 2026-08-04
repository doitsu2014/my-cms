use crate::{ApiResponseError, ApiResponseWith, AppState, AxumResponse};
use application_core::commands::media::bucket::create::{
    create_handler::{CreateBucketHandler, CreateBucketHandlerTrait},
    create_request::CreateBucketRequest,
};
use axum::{extract::Extension, response::IntoResponse, Json};
use domain_interface::AuthenticatedActor;
use tower_cookies::Cookies;
use tracing::instrument;

#[instrument(skip(state))]
pub async fn api_create_bucket(
    state: Extension<AppState>,
    _cookies: Cookies,
    Extension(_actor): Extension<AuthenticatedActor>,
    Json(body): Json<CreateBucketRequest>,
) -> impl IntoResponse {
    let handler = CreateBucketHandler {
        media_config: state.media_config.clone(),
    };

    match handler.create_bucket(body).await {
        Ok(bucket) => ApiResponseWith::new(bucket).to_axum_response(),
        Err(e) => ApiResponseError::from(e).to_axum_response(),
    }
}
