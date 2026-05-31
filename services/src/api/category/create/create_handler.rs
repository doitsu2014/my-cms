use crate::{
    ApiResponseError, ApiResponseWith, AppState,
    AxumResponse,
};
use application_core::commands::category::create::{
    create_handler::{CategoryCreateHandler, CategoryCreateHandlerTrait},
    create_request::CreateCategoryRequest,
};
use axum::{extract::State, response::IntoResponse, Extension, Json};
use crate::common::supabase_auth::SupabaseToken;
use tower_cookies::Cookies;
use tracing::instrument;

#[instrument]
pub async fn api_create_category_with_tags(
    state: State<AppState>,
    cookies: Cookies,
    Extension(token): Extension<SupabaseToken>,
    Json(body): Json<CreateCategoryRequest>,
) -> impl IntoResponse {
    let handler = CategoryCreateHandler {
        db: state.conn.clone(),
    };

    let result = handler
        .handle_create_category_with_tags(body, Some(token.email().unwrap_or("").to_string()))
        .await;

    match result {
        Ok(inserted_id) => ApiResponseWith::new(inserted_id.to_string()).to_axum_response(),
        Err(e) => ApiResponseError::from(e).to_axum_response(),
    }
}
