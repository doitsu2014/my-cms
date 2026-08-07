use axum::extract::{Extension, Path};
use axum::response::IntoResponse;
use domain_user::handlers::read_one::read_one_handler::{
    ReadOneUserHandler, ReadOneUserHandlerTrait,
};
use sea_orm::sqlx::types::Uuid;
use tracing::instrument;

use crate::{ApiResponseError, ApiResponseWith, AppState, AxumResponse};

#[instrument]
pub async fn api_get_user(
    state: Extension<AppState>,
    Path(user_id): Path<Uuid>,
) -> impl IntoResponse {
    let handler = ReadOneUserHandler {
        supabase: state.supabase_admin_client.clone(),
    };

    let result = handler.handle_get_user(user_id).await;

    match result {
        Ok(user) => ApiResponseWith::new(user).to_axum_response(),
        Err(e) => ApiResponseError::from(e).to_axum_response(),
    }
}
