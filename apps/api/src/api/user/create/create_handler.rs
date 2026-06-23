use crate::common::supabase_auth::SupabaseToken;
use crate::{ApiResponseError, ApiResponseWith, AppState, AxumResponse};
use application_core::commands::user::create::{
    create_handler::{CreateUserHandler, CreateUserHandlerTrait},
    create_request::CreateUserRequest,
};
use axum::{extract::State, response::IntoResponse, Extension, Json};
use tracing::instrument;

#[instrument]
pub async fn api_create_user(
    state: State<AppState>,
    Extension(token): Extension<SupabaseToken>,
    Json(body): Json<CreateUserRequest>,
) -> impl IntoResponse {
    let handler = CreateUserHandler {
        supabase: state.supabase_admin_client.clone(),
    };

    let result = handler.handle_create_user(body, token.user_id()).await;

    match result {
        Ok(response) => ApiResponseWith::new(response).to_axum_response(),
        Err(e) => ApiResponseError::from(e).to_axum_response(),
    }
}
