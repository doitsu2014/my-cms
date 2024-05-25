use axum::extract::State;
use migration::{Migrator, MigratorTrait};
use opentelemetry::{
    global,
    trace::{TraceContextExt, Tracer},
    KeyValue,
};

use crate::AppState;

pub async fn handle() -> &'static str {
    "CMS is running successfully!"
}

pub async fn check_health() -> &'static str {
    let tracer2 = global::tracer("tracing-jaeger");
    tracer2.in_span("main-operation", |cx| {
        let span = cx.span();
        span.set_attribute(KeyValue::new("my-attribute", "my-value"));
        span.add_event(
            "Main span event".to_string(),
            vec![KeyValue::new("foo", "1")],
        );
        tracer2.in_span("child-operation...", |cx| {
            let span = cx.span();
            span.add_event("Sub span event", vec![KeyValue::new("bar", "1")]);
        });
    });

    "CMS is running successfully!"
}

pub async fn admin_database_migration(state: State<AppState>) -> &'static str {
    Migrator::up(&state.conn, None).await.unwrap();
    "CMS is running successfully!"
}
