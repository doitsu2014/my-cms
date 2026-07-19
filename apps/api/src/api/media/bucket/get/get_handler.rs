use crate::common::supabase_auth::SupabaseToken;
use crate::{ApiResponseError, ApiResponseWith, AppState, AxumResponse};
use application_core::commands::media::bucket::get::get_handler::{
    GetBucketHandler, GetBucketHandlerTrait,
};
use axum::{
    extract::{Path, State},
    response::IntoResponse,
    Extension,
};
use tower_cookies::Cookies;
use tracing::instrument;

#[instrument(skip(state))]
pub async fn api_get_bucket(
    state: State<AppState>,
    _cookies: Cookies,
    Extension(_token): Extension<SupabaseToken>,
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
