use crate::{
    request_create_post::RequestCreatePost, ApiResponseError, ApiResponseWith, AppState,
    AxumResponse,
};
use axum::{extract::State, response::IntoResponse, Json};
use entity::post;
use sea_orm::{ActiveValue, EntityTrait, IntoActiveModel};
use tower_cookies::Cookies;
use tracing::instrument;

#[instrument]
pub async fn handle(
    state: State<AppState>,
    cookies: Cookies,
    Json(body): Json<RequestCreatePost>,
) -> impl IntoResponse {
    let model = body.into_model();

    let active_model = post::ActiveModel {
        id: ActiveValue::NotSet,
        ..model.into_active_model()
    };
    let result = post::Entity::insert(active_model).exec(&state.conn).await;

    match result {
        Ok(_) => {
            ApiResponseWith::new(body)
                .with_message("System has already created Post successfully".to_string())
                .to_axum_response();
        }
        Err(e) => {
            ApiResponseError::new()
                .with_error_code(crate::ErrorCode::UnknownError)
                .add_error(e.to_string())
                .to_axum_response();
        }
    };
}
