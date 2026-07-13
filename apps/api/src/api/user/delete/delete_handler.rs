use crate::common::supabase_auth::SupabaseToken;
use crate::{ApiResponseError, AppState, AxumResponse};
use application_core::commands::user::delete::delete_handler::{
    DeleteUserHandler, DeleteUserHandlerTrait,
};
use axum::{
    extract::{Path, State},
    http::StatusCode,
    response::Response,
    Extension,
};
use sea_orm::sqlx::types::Uuid;
use tracing::instrument;

#[instrument]
pub async fn api_delete_user(
    state: State<AppState>,
    Path(user_id): Path<Uuid>,
    Extension(token): Extension<SupabaseToken>,
) -> Response<String> {
    let handler = DeleteUserHandler {
        supabase: state.supabase_admin_client.clone(),
    };

    let result = handler.handle_delete_user(user_id, token.user_id()).await;

    match result {
        Ok(()) => Response::builder()
            .status(StatusCode::NO_CONTENT)
            .body(String::new())
            .unwrap(),
        Err(e) => ApiResponseError::from(e).to_axum_response(),
    }
}
