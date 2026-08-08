//! `DELETE /media/buckets/{name}` — delete a bucket (optionally purging).

use axum::{
    extract::{Path, Query, State},
    response::IntoResponse,
    Extension,
};
use domain_interface::AuthenticatedActor;
use serde::Deserialize;
use tower_cookies::Cookies;
use tracing::instrument;

use crate::{
    api::state::MediaApiState,
    domain::{
        error::AppError,
        response::{ApiResponseError, ApiResponseWith, AxumResponse},
    },
    handlers::bucket::delete::delete_handler::{DeleteBucketHandler, DeleteBucketHandlerTrait},
};

#[derive(Debug, Deserialize)]
pub struct DeleteBucketParams {
    pub purge: Option<bool>,
}

#[instrument(skip(state, _cookies))]
pub async fn api_delete_bucket(
    State(state): State<MediaApiState>,
    _cookies: Cookies,
    Extension(_actor): Extension<AuthenticatedActor>,
    Path(name): Path<String>,
    Query(params): Query<DeleteBucketParams>,
) -> impl IntoResponse {
    let handler = DeleteBucketHandler {
        media_config: state.media_config.clone(),
    };

    let purge = params.purge.unwrap_or(false);

    match handler.delete_bucket(&name, purge).await {
        Ok(()) => ApiResponseWith::new(serde_json::json!({ "message": "Bucket deleted" }))
            .to_axum_response(),
        Err(e) => {
            let app_error: AppError = e;
            ApiResponseError::from(app_error).to_axum_response()
        }
    }
}
