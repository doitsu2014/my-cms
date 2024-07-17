use axum::response::IntoResponse;
use tracing_opentelemetry_instrumentation_sdk::find_current_trace_id;

use crate::{ApiResponseWith, AxumResponse};

#[tracing::instrument]
pub async fn handle() -> &'static str {
    "CMS is running successfully!"
}

#[tracing::instrument]
pub async fn check_health() -> impl IntoResponse {
    let trace_id = find_current_trace_id();
    ApiResponseWith::new(trace_id.unwrap_or("".to_string()))
        .with_message("CMS is running successfully!".to_string())
        .to_axum_response()
}
