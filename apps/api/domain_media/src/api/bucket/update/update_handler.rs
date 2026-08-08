//! `PUT /media/buckets/{name}` — update a bucket.

use axum::{
    extract::{Path, State},
    response::IntoResponse,
    Extension, Json,
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
    handlers::bucket::update::{
        update_handler::{UpdateBucketHandler, UpdateBucketHandlerTrait},
        update_request::UpdateBucketRequest,
    },
};

#[instrument(skip(state, _cookies, body))]
pub async fn api_update_bucket(
    State(state): State<MediaApiState>,
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
        Err(e) => {
            let app_error: AppError = e;
            ApiResponseError::from(app_error).to_axum_response()
        }
    }
}
