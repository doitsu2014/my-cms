use crate::common::supabase_auth::SupabaseToken;
use crate::{ApiResponseError, ApiResponseWith, AppState, AxumResponse};
use application_core::commands::user::modify::{
    modify_handler::{ModifyUserHandler, ModifyUserHandlerTrait},
    modify_request::ModifyUserRequest,
};
use axum::{
    extract::{Path, State},
    response::IntoResponse,
    Extension, Json,
};
use sea_orm::sqlx::types::Uuid;
use tracing::instrument;

#[instrument]
pub async fn api_modify_user(
    state: State<AppState>,
    Path(user_id): Path<Uuid>,
    Extension(token): Extension<SupabaseToken>,
    Json(body): Json<ModifyUserRequest>,
) -> impl IntoResponse {
    let handler = ModifyUserHandler {
        supabase: state.supabase_admin_client.clone(),
    };

    let result = handler
        .handle_modify_user(user_id, body, token.user_id())
        .await;

    match result {
        Ok(user) => ApiResponseWith::new(user).to_axum_response(),
        Err(e) => ApiResponseError::from(e).to_axum_response(),
    }
}
