use crate::{ApiResponseError, ApiResponseWith, AppState, AxumResponse};
use application_core::commands::user::create::{
    create_handler::{CreateUserHandler, CreateUserHandlerTrait},
    create_request::CreateUserRequest,
};
use axum::{extract::Extension, response::IntoResponse, Json};
use domain_interface::AuthenticatedActor;
use tracing::instrument;

#[instrument]
pub async fn api_create_user(
    state: Extension<AppState>,
    Extension(actor): Extension<AuthenticatedActor>,
    Json(body): Json<CreateUserRequest>,
) -> impl IntoResponse {
    let handler = CreateUserHandler {
        supabase: state.supabase_admin_client.clone(),
    };

    let result = handler
        .handle_create_user(body, actor.user_id.as_str())
        .await;

    match result {
        Ok(response) => ApiResponseWith::new(response).to_axum_response(),
        Err(e) => ApiResponseError::from(e).to_axum_response(),
    }
}
