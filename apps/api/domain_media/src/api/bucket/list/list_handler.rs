//! `GET /media/buckets` — list all Supabase Storage buckets.

use axum::{extract::State, response::IntoResponse, Extension};
use domain_interface::AuthenticatedActor;
use tower_cookies::Cookies;
use tracing::instrument;

use crate::{
    api::state::MediaApiState,
    domain::{
        error::AppError,
        response::{ApiResponseError, ApiResponseWith, AxumResponse},
    },
    handlers::bucket::list::list_handler::{ListBucketsHandler, ListBucketsHandlerTrait},
};

#[instrument(skip(state, _cookies))]
pub async fn api_list_buckets(
    State(state): State<MediaApiState>,
    _cookies: Cookies,
    Extension(_actor): Extension<AuthenticatedActor>,
) -> impl IntoResponse {
    let handler = ListBucketsHandler {
        media_config: state.media_config.clone(),
    };

    match handler.list_buckets().await {
        Ok(buckets) => ApiResponseWith::new(buckets).to_axum_response(),
        Err(e) => {
            let app_error: AppError = e;
            ApiResponseError::from(app_error).to_axum_response()
        }
    }
}
