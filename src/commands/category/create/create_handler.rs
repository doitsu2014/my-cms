use super::create_request::CreateRequest;
use crate::{ApiResponseError, ApiResponseWith, AppState, AxumResponse};
use axum::{extract::State, response::IntoResponse, Json};
use entity::category;
use sea_orm::{EntityTrait, IntoActiveModel};
use tower_cookies::Cookies;
use tracing::instrument;

#[instrument]
pub async fn handle(
    state: State<AppState>,
    cookies: Cookies,
    Json(body): Json<CreateRequest>,
) -> impl IntoResponse {
    let model = body.into_model();
    let active_model = category::ActiveModel {
        ..model.into_active_model()
    };
    let result = category::Entity::insert(active_model)
        .exec(&state.conn)
        .await;

    match result {
        Ok(_) => {
            ApiResponseWith::new(body)
                .with_message("System has already create Category successfully".to_string())
                .to_axum_response();
        }
        Err(e) => {
            ApiResponseError::new()
                .with_error_code(crate::ErrorCode::UnknownError)
                .add_error(e.to_string())
                .to_axum_response();
        }
    }
}
