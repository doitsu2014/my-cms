use crate::common::supabase_auth::SupabaseToken;
use crate::{ApiResponseError, ApiResponseWith, AppState, AxumResponse};
use application_core::commands::media::bucket::update::{
    update_handler::{UpdateBucketHandler, UpdateBucketHandlerTrait},
    update_request::UpdateBucketRequest,
};
use axum::{
    extract::{Path, State},
    response::IntoResponse,
    Extension, Json,
};
use tower_cookies::Cookies;
use tracing::instrument;

#[instrument(skip(state))]
pub async fn api_update_bucket(
    state: State<AppState>,
    _cookies: Cookies,
    Extension(_token): Extension<SupabaseToken>,
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
