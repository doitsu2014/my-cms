use application_core::commands::media::read::read_handler::{
    ReadMediaHandler, ReadMediaHandlerTrait, ResizeParams,
};
use axum::{
    body::Body,
    extract::{Path, Query, State},
    http::{header, HeaderMap, StatusCode},
    response::{IntoResponse, Response},
};
use serde::Deserialize;
use std::sync::Arc;
use tracing::instrument;

use crate::AppState;

#[derive(Debug, Deserialize)]
pub struct ImageQueryParams {
    pub w: Option<u32>,
    pub h: Option<u32>,
}

#[instrument(skip(state))]
pub async fn api_get_media_image(
    state: State<AppState>,
    Path(path): Path<String>,
    Query(params): Query<ImageQueryParams>,
) -> impl IntoResponse {
    let handler = ReadMediaHandler {
        s3_media_storage: Arc::new(state.media_config.s3_media_storage.clone()),
        image_cache: state.image_cache.clone(),
    };

    let resize_params = ResizeParams::new(params.w, params.h);

    match handler.get_image(path, resize_params).await {
        Ok(cached_image) => {
            let mut headers = HeaderMap::new();
            headers.insert(
                header::CONTENT_TYPE,
                cached_image.content_type.parse().unwrap(),
            );
            headers.insert(
                header::CACHE_CONTROL,
                "public, max-age=31536000, immutable".parse().unwrap(),
            );

            (StatusCode::OK, headers, Body::from(cached_image.data)).into_response()
        }
        Err(e) => Response::builder()
            .status(StatusCode::NOT_FOUND)
            .header(header::CONTENT_TYPE, "application/json")
            .body(Body::from(format!(r#"{{"error": "{}"}}"#, e)))
            .unwrap()
            .into_response(),
    }
}
