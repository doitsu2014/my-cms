use application_core::commands::media::read::read_handler::{
    ReadMediaHandler, ReadMediaHandlerTrait, ResizeParams,
};
use application_core::common::app_error::AppError;
use axum::{
    body::Body,
    extract::{Path, Query, State},
    http::{header, HeaderMap, StatusCode},
    response::{IntoResponse, Redirect, Response},
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

fn error_status(e: &AppError) -> StatusCode {
    match e {
        AppError::NotFound => StatusCode::NOT_FOUND,
        _ => StatusCode::BAD_GATEWAY,
    }
}

fn error_response(e: AppError) -> Response {
    (
        error_status(&e),
        [(header::CONTENT_TYPE, "application/json")],
        format!(r#"{{"error": "{}"}}"#, e),
    )
        .into_response()
}

#[instrument(skip(state))]
pub async fn api_get_media_image(
    state: State<AppState>,
    Path(path): Path<String>,
    Query(params): Query<ImageQueryParams>,
) -> Response {
    if params.w.is_some() || params.h.is_some() {
        let url = state
            .media_config
            .storage
            .render_image_url(&path, params.w, params.h);
        return Redirect::temporary(&url).into_response();
    }

    let handler = ReadMediaHandler::new(
        Arc::new(state.media_config.storage.clone()),
        state.media_cache.clone(),
    );

    let resize_params = ResizeParams::new(params.w, params.h);

    match handler.get_image(path, resize_params).await {
        Ok(cached_media) => {
            let mut headers = HeaderMap::new();
            headers.insert(
                header::CONTENT_TYPE,
                cached_media.content_type.parse().unwrap(),
            );
            headers.insert(
                header::CACHE_CONTROL,
                "public, max-age=31536000, immutable".parse().unwrap(),
            );

            (StatusCode::OK, headers, Body::from(cached_media.data)).into_response()
        }
        Err(e) => error_response(e),
    }
}

#[instrument(skip(state))]
pub async fn api_get_media(state: State<AppState>, Path(path): Path<String>) -> impl IntoResponse {
    let handler = ReadMediaHandler::new(
        Arc::new(state.media_config.storage.clone()),
        state.media_cache.clone(),
    );

    match handler.get_media(path).await {
        Ok(cached_media) => {
            let mut headers = HeaderMap::new();
            headers.insert(
                header::CONTENT_TYPE,
                cached_media.content_type.parse().unwrap(),
            );
            headers.insert(
                header::CACHE_CONTROL,
                "public, max-age=31536000, immutable".parse().unwrap(),
            );

            (StatusCode::OK, headers, Body::from(cached_media.data)).into_response()
        }
        Err(e) => error_response(e),
    }
}
