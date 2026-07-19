use crate::common::supabase_auth::SupabaseToken;
use crate::{ApiResponseError, ApiResponseWith, AppState, AxumResponse};
use application_core::commands::media::bucket::delete::delete_handler::{
    DeleteBucketHandler, DeleteBucketHandlerTrait,
};
use axum::{
    extract::{Path, Query, State},
    response::IntoResponse,
    Extension,
};
use serde::Deserialize;
use tower_cookies::Cookies;
use tracing::instrument;

#[derive(Debug, Deserialize)]
pub struct DeleteBucketParams {
    pub purge: Option<bool>,
}

#[instrument(skip(state))]
pub async fn api_delete_bucket(
    state: State<AppState>,
    _cookies: Cookies,
    Extension(_token): Extension<SupabaseToken>,
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
        Err(e) => ApiResponseError::from(e).to_axum_response(),
    }
}
