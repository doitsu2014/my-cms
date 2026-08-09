//! `GET /media/buckets/{name}` — fetch a single bucket by name.

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
    handlers::bucket::get::get_handler::{GetBucketHandler, GetBucketHandlerTrait},
};

#[instrument(skip(state, _cookies))]
pub async fn api_get_bucket(
    State(state): State<MediaApiState>,
    _cookies: Cookies,
    Extension(_actor): Extension<AuthenticatedActor>,
    Path(name): Path<String>,
) -> impl IntoResponse {
    let handler = GetBucketHandler {
        media_config: state.media_config.clone(),
    };

    match handler.get_bucket(&name).await {
        Ok(bucket) => ApiResponseWith::new(bucket).to_axum_response(),
        Err(e) => {
            let app_error: AppError = e;
            ApiResponseError::from(app_error).to_axum_response()
        }
    }
}
