//! OpenTelemetry + tracing initialisation for the post domain. Mirrors the
//! legacy `apps/api/src/bin/my-cms-api.rs` lines 70–89. When the post
//! domain runs standalone (its `bin` target) or is composed behind the
//! gateway, the same initialisation is used so behavior is identical.

use init_tracing_opentelemetry::{
    otlp::OtelGuard,
    tracing_subscriber_ext::{build_level_filter_layer, build_logger_text},
};
use tracing_subscriber::layer::SubscriberExt;

/// Initialise tracing + OpenTelemetry when `ENABLED_OTLP_EXPORTER=true`.
/// Returns `Some(OtelGuard)` so the caller can keep the guard alive for
/// the lifetime of the process. When OTLP export is disabled returns
/// `None` and the caller can install a plain text subscriber instead.
pub fn init() -> Option<OtelGuard> {
    let enabled = std::env::var("ENABLED_OTLP_EXPORTER")
        .unwrap_or_else(|_| "false".to_string())
        .parse::<bool>()
        .unwrap_or(false);
    if enabled {
        init_tracing_opentelemetry::tracing_subscriber_ext::init_subscribers().ok()
    } else {
        None
    }
}

/// Install a plain-text tracing subscriber. Used when
/// `ENABLED_OTLP_EXPORTER` is false.
pub fn init_text_logging() {
    let subscriber = tracing_subscriber::registry()
        .with(build_level_filter_layer("").unwrap_or_default())
        .with(build_logger_text());
    let _ = tracing::subscriber::set_global_default(subscriber);
}
