//! Auth-layer factory for the Supabase auth layer.
//!
//! This is the single source of truth for constructing `SupabaseAuthLayer`
//! instances from the env-var surface. Both the gateway composition root
//! and the standalone `domain_posts` binary call this function.

use std::env;

use crate::{SupabaseAuthConfig, SupabaseAuthLayer};
use domain_interface::DomainConfigError;

/// Env var name for the public Supabase URL (also used for JWKS fallback).
const SUPABASE_URL: &str = "SUPABASE_URL";
/// Env var name for the internal Supabase URL (preferred inside the container network).
const SUPABASE_INTERNAL_URL: &str = "SUPABASE_INTERNAL_URL";
/// Env var name for the HS256 JWT secret used for local validation.
const SUPABASE_JWT_SECRET: &str = "SUPABASE_JWT_SECRET";

/// Build a `SupabaseAuthLayer` from the env-var surface.
///
/// Reads:
/// - `SUPABASE_URL` (required) — public Supabase URL used for JWKS fallback.
/// - `SUPABASE_INTERNAL_URL` (optional) — internal URL used as a fallback for
///   `SUPABASE_URL` (the JWT validator uses this for JWKS fetches inside the
///   container network).
/// - `SUPABASE_JWT_SECRET` (required) — HS256 secret for local JWT validation.
///
/// Returns `Err(DomainConfigError::MissingEnv(<var>))` when either required
/// env var is missing. Callers (the gateway and `domain_posts` binaries)
/// propagate the error to `ExitCode::FAILURE`.
///
/// The successful construction emits a `tracing::info!` event that records
/// `expected_audience`, `required_roles_count`, and the resolved `supabase_url`.
/// The JWT secret is never logged.
///
/// # Arguments
///
/// - `expected_audience` — the `aud` claim value the JWT must carry.
/// - `required_roles` — the role(s) a request must hold to pass through.
///   Empty vector allows any authenticated user.
#[tracing::instrument(
    name = "auth_layer_from_env",
    skip_all,
    fields(expected_audience = %expected_audience, required_roles_count = required_roles.len()),
)]
pub fn auth_layer_from_env(
    expected_audience: String,
    required_roles: Vec<String>,
) -> Result<SupabaseAuthLayer, DomainConfigError> {
    let supabase_url =
        env::var(SUPABASE_URL).map_err(|_| DomainConfigError::MissingEnv(SUPABASE_URL))?;
    let supabase_internal_url =
        env::var(SUPABASE_INTERNAL_URL).unwrap_or_else(|_| supabase_url.clone());
    let jwt_secret = env::var(SUPABASE_JWT_SECRET)
        .map_err(|_| DomainConfigError::MissingEnv(SUPABASE_JWT_SECRET))?;

    tracing::info!(
        supabase_url = %supabase_internal_url,
        "auth layer constructed"
    );

    Ok(SupabaseAuthLayer::new(SupabaseAuthConfig {
        supabase_url: supabase_internal_url,
        jwt_secret,
        expected_audience,
        required_roles,
    }))
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::test_lock::{with_env_var, ENV_LOCK};
    use std::sync::{Arc, Mutex};
    use tracing_subscriber::layer::{Context, SubscriberExt};
    use tracing_subscriber::registry::LookupSpan;

    /// JWT secret material that the tracing test puts in the env. The
    /// test asserts the captured event does NOT contain this string.
    const TRACING_TEST_SECRET: &str = "must-never-appear-in-tracing-output";

    /// Minimal `tracing::Subscriber` that records every `info!` event's
    /// visited fields and the fields on each span that contains the event.
    /// Designed for the factory's tracing assertion: it lets the test
    /// inspect the captured payload without the JWT secret leaking.
    #[derive(Default, Clone)]
    struct CapturedInfoEvents {
        events: Arc<Mutex<Vec<Vec<(String, String)>>>>,
    }

    impl CapturedInfoEvents {
        fn new() -> Self {
            Self::default()
        }

        fn snapshot_events(&self) -> Vec<Vec<(String, String)>> {
            self.events.lock().unwrap().clone()
        }

        fn contains_secret(&self, secret: &str) -> bool {
            let events = self.events.lock().unwrap();
            for fields in events.iter() {
                for (_k, v) in fields {
                    if v.contains(secret) {
                        return true;
                    }
                }
            }
            false
        }

        fn has_event_with_field(&self, key: &str, expected_value: &str) -> bool {
            let events = self.events.lock().unwrap();
            events
                .iter()
                .any(|fields| fields.iter().any(|(k, v)| k == key && v == expected_value))
        }

        fn has_event_with_message(&self, expected_message: &str) -> bool {
            let events = self.events.lock().unwrap();
            events.iter().any(|fields| {
                fields
                    .iter()
                    .any(|(k, v)| k == "message" && v == expected_message)
            })
        }
    }

    impl<S> tracing_subscriber::Layer<S> for CapturedInfoEvents
    where
        S: tracing::Subscriber + for<'a> LookupSpan<'a>,
    {
        fn on_event(&self, event: &tracing::Event<'_>, ctx: Context<'_, S>) {
            // Only record `info!` events — the factory only emits info-level.
            if *event.metadata().level() != tracing::Level::INFO {
                return;
            }

            // Collect the visited event fields via the field visitor.
            let mut visitor = FieldVisitor::default();
            event.record(&mut visitor);

            // Walk the current span stack and merge each span's recorded
            // fields into the captured payload (the instrument macro
            // attaches `expected_audience` and `required_roles_count` to
            // the function-call span, not to the inner event).
            let mut merged: Vec<(String, String)> = visitor.0;
            if let Some(scope) = ctx.event_scope(event) {
                for span in scope.from_root() {
                    let extensions = span.extensions();
                    if let Some(fields) = extensions.get::<RecordedSpanFields>() {
                        merged.extend(fields.0.iter().cloned());
                    }
                }
            }
            self.events.lock().unwrap().push(merged);
        }

        fn on_new_span(
            &self,
            attrs: &tracing::span::Attributes<'_>,
            id: &tracing::span::Id,
            ctx: Context<'_, S>,
        ) {
            let mut visitor = FieldVisitor::default();
            attrs.record(&mut visitor);
            let span = ctx.span(id).expect("span exists");
            let mut extensions = span.extensions_mut();
            extensions.insert(RecordedSpanFields(visitor.0));
        }

        fn on_record(
            &self,
            id: &tracing::span::Id,
            values: &tracing::span::Record<'_>,
            ctx: Context<'_, S>,
        ) {
            let mut visitor = FieldVisitor::default();
            values.record(&mut visitor);
            let span = ctx.span(id).expect("span exists");
            let mut extensions = span.extensions_mut();
            extensions
                .get_mut::<RecordedSpanFields>()
                .expect("span fields inserted")
                .0
                .extend(visitor.0);
        }
    }

    /// Span-scoped field storage — the `tracing_subscriber::Layer` needs a
    /// `Send + Sync` extension type to stash the visited fields per span.
    #[derive(Default)]
    struct RecordedSpanFields(Vec<(String, String)>);

    #[derive(Default)]
    struct FieldVisitor(Vec<(String, String)>);

    impl tracing::field::Visit for FieldVisitor {
        fn record_str(&mut self, field: &tracing::field::Field, value: &str) {
            self.0.push((field.name().to_string(), value.to_string()));
        }

        fn record_debug(&mut self, field: &tracing::field::Field, value: &dyn std::fmt::Debug) {
            self.0
                .push((field.name().to_string(), format!("{:?}", value)));
        }

        fn record_i64(&mut self, field: &tracing::field::Field, value: i64) {
            self.0.push((field.name().to_string(), value.to_string()));
        }

        fn record_u64(&mut self, field: &tracing::field::Field, value: u64) {
            self.0.push((field.name().to_string(), value.to_string()));
        }

        fn record_bool(&mut self, field: &tracing::field::Field, value: bool) {
            self.0.push((field.name().to_string(), value.to_string()));
        }
    }

    #[test]
    fn auth_layer_from_env_returns_ok_when_env_vars_are_set() {
        let _guard = ENV_LOCK.lock().unwrap_or_else(|e| e.into_inner());
        let result = with_env_var(SUPABASE_URL, Some("http://localhost:8001"), || {
            with_env_var(SUPABASE_INTERNAL_URL, None, || {
                with_env_var(SUPABASE_JWT_SECRET, Some("test-secret"), || {
                    auth_layer_from_env("authenticated".to_string(), vec!["writer".to_string()])
                })
            })
        });
        assert!(
            result.is_ok(),
            "expected auth_layer_from_env to succeed, got {:?}",
            result
        );
    }

    #[test]
    fn auth_layer_from_env_uses_supabase_internal_url_when_supabase_url_is_missing() {
        // The factory's required URL is `SUPABASE_URL`; the internal URL is
        // `SUPABASE_INTERNAL_URL`. When `SUPABASE_INTERNAL_URL` is set,
        // construction must succeed (the layer is built; the resolved URL
        // is asserted via the tracing test below).
        let _guard = ENV_LOCK.lock().unwrap_or_else(|e| e.into_inner());
        let result = with_env_var(SUPABASE_URL, Some("http://public-host:8000"), || {
            with_env_var(
                SUPABASE_INTERNAL_URL,
                Some("http://internal-host:8001"),
                || {
                    with_env_var(SUPABASE_JWT_SECRET, Some("test-secret"), || {
                        auth_layer_from_env("authenticated".to_string(), vec![])
                    })
                },
            )
        });
        assert!(
            result.is_ok(),
            "expected auth_layer_from_env to succeed with INTERNAL_URL fallback, got {:?}",
            result
        );
    }

    #[test]
    fn auth_layer_from_env_returns_missing_env_error_when_supabase_url_is_unset() {
        let _guard = ENV_LOCK.lock().unwrap_or_else(|e| e.into_inner());
        let result = with_env_var(SUPABASE_URL, None, || {
            with_env_var(SUPABASE_JWT_SECRET, Some("test-secret"), || {
                auth_layer_from_env("authenticated".to_string(), vec![])
            })
        });
        assert!(
            matches!(result, Err(DomainConfigError::MissingEnv(SUPABASE_URL))),
            "expected MissingEnv(SUPABASE_URL), got {:?}",
            result
        );
    }

    #[test]
    fn auth_layer_from_env_returns_missing_env_error_when_supabase_jwt_secret_is_unset() {
        let _guard = ENV_LOCK.lock().unwrap_or_else(|e| e.into_inner());
        let result = with_env_var(SUPABASE_URL, Some("http://localhost:8001"), || {
            with_env_var(SUPABASE_JWT_SECRET, None, || {
                auth_layer_from_env("authenticated".to_string(), vec![])
            })
        });
        assert!(
            matches!(
                result,
                Err(DomainConfigError::MissingEnv(SUPABASE_JWT_SECRET))
            ),
            "expected MissingEnv(SUPABASE_JWT_SECRET), got {:?}",
            result
        );
    }

    #[test]
    fn auth_layer_from_env_emits_info_event_with_resolved_url_and_no_jwt_secret() {
        let captured = CapturedInfoEvents::new();
        // The `tracing_subscriber::registry` defaults to a `LevelFilter::OFF`
        // when no filter is layered on top, so an explicit max-level filter
        // is required for our captured events to be visited.
        let subscriber = tracing_subscriber::registry()
            .with(tracing::level_filters::LevelFilter::INFO)
            .with(captured.clone());

        // Install the captured subscriber as the global default exactly
        // once for the test binary. The `Once` makes `set_global_default`
        // races benign — the first call wins, subsequent calls are
        // ignored. The captured subscriber is owned by `captured` which
        // outlives the test, so the global default stays valid and any
        // info event emitted by the factory during the test is captured
        // deterministically regardless of the test thread.
        static INIT: std::sync::Once = std::sync::Once::new();
        INIT.call_once(|| {
            let _ = tracing::subscriber::set_global_default(subscriber);
        });

        let _guard = ENV_LOCK.lock().unwrap_or_else(|e| e.into_inner());
        let result = with_env_var(SUPABASE_URL, Some("http://localhost:8001"), || {
            with_env_var(SUPABASE_INTERNAL_URL, None, || {
                with_env_var(SUPABASE_JWT_SECRET, Some(TRACING_TEST_SECRET), || {
                    auth_layer_from_env("authenticated".to_string(), vec!["writer".to_string()])
                })
            })
        });

        assert!(result.is_ok(), "expected Ok, got {:?}", result);

        assert!(result.is_ok(), "expected Ok, got {:?}", result);

        // The factory's success path emits one info event whose `message`
        // field is "auth layer constructed".
        assert!(
            captured.has_event_with_message("auth layer constructed"),
            "expected info event with message 'auth layer constructed', got events: {:?}",
            captured.snapshot_events()
        );

        // The captured event must include the resolved `supabase_url` and
        // the `required_roles_count` field. The factory uses `%` for the
        // `supabase_url` (Display), so its captured value is the unquoted
        // string. `required_roles_count` is a `usize` rendered via Display.
        assert!(
            captured.has_event_with_field("supabase_url", "http://localhost:8001"),
            "expected field supabase_url=http://localhost:8001, got {:?}",
            captured.snapshot_events()
        );
        assert!(
            captured.has_event_with_field("required_roles_count", "1"),
            "expected field required_roles_count=1, got {:?}",
            captured.snapshot_events()
        );

        // The JWT secret must not appear in any captured field.
        assert!(
            !captured.contains_secret(TRACING_TEST_SECRET),
            "tracing output leaked the JWT secret"
        );
    }
}
