use crate::{ApiResponseError, ApiResponseWith, AppState, AxumResponse};
use axum::{extract::Path, response::IntoResponse, Extension};
use domain_interface::AuthenticatedActor;
use domain_media::handlers::bucket::get::get_handler::{GetBucketHandler, GetBucketHandlerTrait};
use tower_cookies::Cookies;
use tracing::instrument;

#[instrument(skip(state))]
pub async fn api_get_bucket(
    state: Extension<AppState>,
    _cookies: Cookies,
    Extension(_actor): Extension<AuthenticatedActor>,
    Path(name): Path<String>,
) -> impl IntoResponse {
    let handler = GetBucketHandler {
        media_config: state.media_config.clone(),
    };

    match handler.get_bucket(&name).await {
        Ok(bucket) => ApiResponseWith::new(bucket).to_axum_response(),
        Err(e) => ApiResponseError::from(e).to_axum_response(),
    }
}
