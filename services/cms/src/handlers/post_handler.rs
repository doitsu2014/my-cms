use axum::{
    routing::{get, post},
    Router,
};

pub async fn handle_get_list() {}

pub trait RouterPostHandlerExt {
    fn build_post_routes(self) -> Self;
}

impl RouterPostHandlerExt for Router {
    fn build_post_routes(self) -> Self {
        self.route("/posts", get(handle_get_list))
    }
}
