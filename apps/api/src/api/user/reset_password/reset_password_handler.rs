use crate::common::supabase_auth::SupabaseToken;
use crate::{ApiResponseError, ApiResponseWith, AppState, AxumResponse};
use application_core::commands::user::reset_password::{
    ResetPasswordHandler, ResetPasswordHandlerTrait, ResetPasswordRequest,
};
use axum::{
    extract::{Path, State},
    response::IntoResponse,
    Extension, Json,
};
use sea_orm::sqlx::types::Uuid;
use tracing::instrument;

#[instrument]
pub async fn api_reset_password(
    state: State<AppState>,
    Path(user_id): Path<Uuid>,
    Extension(token): Extension<SupabaseToken>,
    Json(body): Json<ResetPasswordRequest>,
) -> impl IntoResponse {
    let handler = ResetPasswordHandler {
        supabase: state.supabase_admin_client.clone(),
    };

    let result = handler
        .handle_reset_password(user_id, body, token.user_id())
        .await;

    match result {
        Ok(response) => ApiResponseWith::new(response).to_axum_response(),
        Err(e) => ApiResponseError::from(e).to_axum_response(),
    }
}
