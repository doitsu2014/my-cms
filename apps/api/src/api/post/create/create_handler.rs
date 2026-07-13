use crate::common::supabase_auth::SupabaseToken;
use application_core::commands::post::create::{
    create_handler::{PostCreateHandler, PostCreateHandlerTrait},
    create_request::CreatePostRequest,
};
use axum::{extract::State, response::IntoResponse, Extension, Json};
use tower_cookies::Cookies;
use tracing::instrument;

use crate::{ApiResponseError, ApiResponseWith, AppState, AxumResponse};

#[instrument]
pub async fn api_create_post(
    state: State<AppState>,
    cookies: Cookies,
    Extension(token): Extension<SupabaseToken>,
    Json(body): Json<CreatePostRequest>,
) -> impl IntoResponse {
    let handler = PostCreateHandler {
        db: state.conn.clone(),
    };

    let result = handler
        .handle_create_post(body, Some(token.email().unwrap_or("").to_string()))
        .await;

    match result {
        Ok(inserted_id) => ApiResponseWith::new(inserted_id.to_string()).to_axum_response(),
        Err(e) => ApiResponseError::from(e).to_axum_response(),
    }
}
