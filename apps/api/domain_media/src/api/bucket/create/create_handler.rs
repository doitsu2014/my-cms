//! `POST /media/buckets` — create a new Supabase Storage bucket.
//!
//! Adapter-layer concern: Axum envelope mapping. The bucket policy,
//! validation, and storage call are delegated to `CreateBucketHandler`.

use axum::{extract::State, response::IntoResponse, Extension, Json};
use domain_interface::AuthenticatedActor;
use tower_cookies::Cookies;
use tracing::instrument;

use crate::{
    api::state::MediaApiState,
    domain::{
        error::AppError,
        response::{ApiResponseError, ApiResponseWith, AxumResponse},
    },
    handlers::bucket::create::{
        create_handler::{CreateBucketHandler, CreateBucketHandlerTrait},
        create_request::CreateBucketRequest,
    },
};

#[instrument(skip(state, _cookies, body))]
pub async fn api_create_bucket(
    State(state): State<MediaApiState>,
    _cookies: Cookies,
    Extension(_actor): Extension<AuthenticatedActor>,
    Json(body): Json<CreateBucketRequest>,
) -> impl IntoResponse {
    let handler = CreateBucketHandler {
        media_config: state.media_config.clone(),
    };

    match handler.create_bucket(body).await {
        Ok(bucket) => ApiResponseWith::new(bucket).to_axum_response(),
        Err(e) => {
            let app_error: AppError = e;
            ApiResponseError::from(app_error).to_axum_response()
        }
    }
}
