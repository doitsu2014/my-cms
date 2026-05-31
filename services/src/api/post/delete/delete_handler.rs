use crate::{
    ApiResponseError, ApiResponseWith, AppState,
    AxumResponse,
};
use application_core::commands::post::delete::delete_handler::{
    PostDeleteHandler, PostDeleteHandlerTrait,
};
use axum::{extract::State, response::IntoResponse, Extension, Json};
use crate::common::supabase_auth::SupabaseToken;
use sea_orm::sqlx::types::Uuid;
use tower_cookies::Cookies;
use tracing::instrument;

#[instrument]
pub async fn api_delete_posts(
    state: State<AppState>,
    cookies: Cookies,
    Extension(token): Extension<SupabaseToken>,
    Json(body): Json<Vec<Uuid>>,
) -> impl IntoResponse {
    let handler = PostDeleteHandler {
        db: state.conn.clone(),
    };

    let result = handler
        .handle_delete_posts(body, Some(token.email().unwrap_or("").to_string()))
        .await;

    match result {
        Ok(inserted_id) => ApiResponseWith::new(inserted_id.to_string()).to_axum_response(),
        Err(e) => ApiResponseError::from(e).to_axum_response(),
    }
}
