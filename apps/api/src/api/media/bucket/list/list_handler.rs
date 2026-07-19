use crate::common::supabase_auth::SupabaseToken;
use crate::{ApiResponseError, ApiResponseWith, AppState, AxumResponse};
use application_core::commands::media::bucket::list::list_handler::{
    ListBucketsHandler, ListBucketsHandlerTrait,
};
use axum::{extract::State, response::IntoResponse, Extension};
use tower_cookies::Cookies;
use tracing::instrument;

#[instrument(skip(state))]
pub async fn api_list_buckets(
    state: State<AppState>,
    _cookies: Cookies,
    Extension(_token): Extension<SupabaseToken>,
) -> impl IntoResponse {
    let handler = ListBucketsHandler {
        media_config: state.media_config.clone(),
    };

    match handler.list_buckets().await {
        Ok(buckets) => ApiResponseWith::new(buckets).to_axum_response(),
        Err(e) => ApiResponseError::from(e).to_axum_response(),
    }
}
