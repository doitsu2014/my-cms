use crate::{request_create_post::RequestCreatePost, ApiResponseWith, AppState, AxumResponse};
use axum::{extract::State, response::IntoResponse, Json};
use entity::post;
use sea_orm::{ActiveValue, EntityTrait, IntoActiveModel};
use tower_cookies::Cookies;

pub async fn handle_get_list(state: State<AppState>) -> impl IntoResponse {
    let all = post::Entity::find()
        .all(&state.conn)
        .await
        .expect("Failed to fetch all posts");

    ApiResponseWith::new(all).to_axum_response()
}

pub async fn handle_post(
    state: State<AppState>,
    mut cookies: Cookies,
    Json(body): Json<RequestCreatePost>,
) -> impl IntoResponse {
    let model = body.into_model();

    let active_model = post::ActiveModel {
        id: ActiveValue::NotSet,
        ..model.into_active_model()
    };
    post::Entity::insert(active_model).exec(&state.conn).await;

    ApiResponseWith::new(body)
        .with_message("Post created successfully".to_string())
        .to_axum_response()
}
