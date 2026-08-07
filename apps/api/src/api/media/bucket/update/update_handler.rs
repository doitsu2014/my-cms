use crate::{ApiResponseError, ApiResponseWith, AppState, AxumResponse};
use axum::{extract::Path, response::IntoResponse, Extension, Json};
use domain_interface::AuthenticatedActor;
use domain_media::handlers::bucket::update::{
    update_handler::{UpdateBucketHandler, UpdateBucketHandlerTrait},
    update_request::UpdateBucketRequest,
};
use tower_cookies::Cookies;
use tracing::instrument;

#[instrument(skip(state))]
pub async fn api_update_bucket(
    state: Extension<AppState>,
    _cookies: Cookies,
    Extension(_actor): Extension<AuthenticatedActor>,
    Path(name): Path<String>,
    Json(body): Json<UpdateBucketRequest>,
) -> impl IntoResponse {
    let handler = UpdateBucketHandler {
        media_config: state.media_config.clone(),
    };

    match handler.update_bucket(&name, body).await {
        Ok(bucket) => ApiResponseWith::new(bucket).to_axum_response(),
        Err(e) => ApiResponseError::from(e).to_axum_response(),
    }
}
