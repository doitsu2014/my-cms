use crate::{
    ApiResponseError, ApiResponseWith, AppState,
    AxumResponse,
};
use application_core::commands::category::modify::{
    modify_handler::{CategoryModifyHandler, CategoryModifyHandlerTrait},
    modify_request::ModifyCategoryRequest,
};
use axum::{extract::State, response::IntoResponse, Extension, Json};
use crate::common::supabase_auth::SupabaseToken;
use tower_cookies::Cookies;
use tracing::instrument;

#[instrument]
pub async fn api_modify_category(
    state: State<AppState>,
    cookies: Cookies,
    Extension(token): Extension<SupabaseToken>,
    Json(body): Json<ModifyCategoryRequest>,
) -> impl IntoResponse {
    let handler = CategoryModifyHandler {
        db: state.conn.clone(),
    };

    let result = handler
        .handle_modify_category(body, Some(token.email().unwrap_or("").to_string()))
        .await;

    match result {
        Ok(inserted_id) => ApiResponseWith::new(inserted_id.to_string()).to_axum_response(),
        Err(e) => ApiResponseError::from(e).to_axum_response(),
    }
}
