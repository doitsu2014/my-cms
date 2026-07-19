use crate::common::supabase_auth::SupabaseToken;
use crate::{ApiResponseError, ApiResponseWith, AppState, AxumResponse};
use application_core::commands::media::bucket::empty::empty_handler::{
    EmptyBucketHandler, EmptyBucketHandlerTrait,
};
use axum::{
    extract::{Path, State},
    response::IntoResponse,
    Extension,
};
use tower_cookies::Cookies;
use tracing::instrument;

#[instrument(skip(state))]
pub async fn api_empty_bucket(
    state: State<AppState>,
    _cookies: Cookies,
    Extension(_token): Extension<SupabaseToken>,
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
