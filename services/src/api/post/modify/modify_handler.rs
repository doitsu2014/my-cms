use crate::{
    ApiResponseError, ApiResponseWith, AppState,
    AxumResponse,
};
use application_core::commands::post::modify::{
    modify_handler::{PostModifyHandler, PostModifyHandlerTrait},
    modify_request::ModifyPostRequest,
};
use axum::{extract::State, response::IntoResponse, Extension, Json};
use crate::common::supabase_auth::SupabaseToken;
use tower_cookies::Cookies;
use tracing::instrument;

#[instrument]
pub async fn api_modify_post(
    state: State<AppState>,
    cookies: Cookies,
    Extension(token): Extension<SupabaseToken>,
    Json(body): Json<ModifyPostRequest>,
) -> impl IntoResponse {
    let handler = PostModifyHandler {
        db: state.conn.clone(),
    };

    let result = handler
        .handle_modify_post(body, Some(token.email().unwrap_or("").to_string()))
        .await;

    match result {
        Ok(inserted_id) => ApiResponseWith::new(inserted_id.to_string()).to_axum_response(),
        Err(e) => ApiResponseError::from(e).to_axum_response(),
    }
}
