use axum::response::IntoResponse;
use serde_json::json;
use tracing_opentelemetry_instrumentation_sdk::find_current_trace_id;

#[tracing::instrument]
pub async fn handle() -> &'static str {
    "CMS is running successfully!"
}

#[tracing::instrument]
pub async fn check_health() -> impl IntoResponse {
    let trace_id = find_current_trace_id();
    dbg!(&trace_id);

    axum::Json(json!({
        "message": "CMS is running healthy",
        "trace_id": trace_id
    }))
}
