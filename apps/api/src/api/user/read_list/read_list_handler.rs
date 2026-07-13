use application_core::commands::user::read_list::read_list_handler::{
    ReadListUserHandler, ReadListUserHandlerTrait,
};
use axum::extract::{Query, State};
use axum::response::IntoResponse;
use serde::Deserialize;
use tracing::instrument;

use crate::{ApiResponseError, ApiResponseWith, AppState, AxumResponse};

#[derive(Debug, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct QueryParams {
    pub page: Option<u32>,
    pub per_page: Option<u32>,
    pub role: Option<String>,
    pub email: Option<String>,
}

#[instrument]
pub async fn api_list_users(
    state: State<AppState>,
    query: Query<QueryParams>,
) -> impl IntoResponse {
    let handler = ReadListUserHandler {
        supabase: state.supabase_admin_client.clone(),
    };

    let page = query.page.unwrap_or(1);
    let per_page = query.per_page.unwrap_or(50);

    let result = handler
        .handle_list_users(page, per_page, query.role.clone(), query.email.clone())
        .await;

    match result {
        Ok(users) => ApiResponseWith::new(users).to_axum_response(),
        Err(e) => ApiResponseError::from(e).to_axum_response(),
    }
}
