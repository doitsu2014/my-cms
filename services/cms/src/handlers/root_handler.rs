use axum::{routing::get, Router};

pub async fn handle() -> &'static str {
    "CMS is running successfully!"
}

pub async fn check_health() -> &'static str {
    "CMS is running successfully!"
}

pub async fn admin_database_migration() -> &'static str {
    "CMS is running successfully!"
}

pub trait RouterRootHandlerExt {
    fn build_root_routes(self) -> Self;
}

impl RouterRootHandlerExt for Router {
    fn build_root_routes(self) -> Self {
        self.route("/", get(handle))
            .route("/health", get(check_health))
            .route("/healthz", get(check_health))
            .route("/admin/database/migration", get(admin_database_migration))
    }
}
