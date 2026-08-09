//! `POST /media/buckets/{name}/empty` — empty a bucket of all its objects.

use axum::{
    extract::{Path, State},
    response::IntoResponse,
    Extension,
};
use domain_interface::AuthenticatedActor;
use tower_cookies::Cookies;
use tracing::instrument;

use crate::{
    api::state::MediaApiState,
    domain::{
        error::AppError,
        response::{ApiResponseError, ApiResponseWith, AxumResponse},
    },
    handlers::bucket::empty::empty_handler::{EmptyBucketHandler, EmptyBucketHandlerTrait},
};

#[instrument(skip(state, _cookies))]
pub async fn api_empty_bucket(
    State(state): State<MediaApiState>,
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
        Err(e) => {
            let app_error: AppError = e;
            ApiResponseError::from(app_error).to_axum_response()
        }
    }
}
