use crate::{ApiResponseError, ApiResponseWith, AppState, AxumResponse};
use axum::{extract::Extension, response::IntoResponse};
use domain_interface::AuthenticatedActor;
use domain_media::handlers::bucket::list::list_handler::{
    ListBucketsHandler, ListBucketsHandlerTrait,
};
use tower_cookies::Cookies;
use tracing::instrument;

#[instrument(skip(state))]
pub async fn api_list_buckets(
    state: Extension<AppState>,
    _cookies: Cookies,
    Extension(_actor): Extension<AuthenticatedActor>,
) -> impl IntoResponse {
    let handler = ListBucketsHandler {
        media_config: state.media_config.clone(),
    };

    match handler.list_buckets().await {
        Ok(buckets) => ApiResponseWith::new(buckets).to_axum_response(),
        Err(e) => ApiResponseError::from(e).to_axum_response(),
    }
}
