use crate::{ApiResponseError, ApiResponseWith, AppState, AxumResponse};
use application_core::commands::media::bucket::empty::empty_handler::{
    EmptyBucketHandler, EmptyBucketHandlerTrait,
};
use axum::{extract::Path, response::IntoResponse, Extension};
use domain_interface::AuthenticatedActor;
use tower_cookies::Cookies;
use tracing::instrument;

#[instrument(skip(state))]
pub async fn api_empty_bucket(
    state: Extension<AppState>,
    _cookies: Cookies,
    Extension(_actor): Extension<AuthenticatedActor>,
    Path(name): Path<String>,
) -> impl IntoResponse {
    let handler = EmptyBucketHandler {
        media_config: state.media_config.clone(),
    };

    match handler.empty_bucket(&name).await {
        Ok(()) => ApiResponseWith::new(serde_json::json!({
            "message": format!("Bucket '{}' emptied", name)
        }))
        .to_axum_response(),
        Err(e) => ApiResponseError::from(e).to_axum_response(),
    }
}
