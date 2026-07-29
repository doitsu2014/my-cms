//! Response envelope shared by every post endpoint — `ApiResponseWith<T>` for
//! success, `ApiResponseError` for failure, `ErrorCode`, and the `AxumResponse`
//! trait that turns them into an `axum::response::Response`.

pub use cms::presentation_models::api_response::{
    ApiResponseError, ApiResponseWith, AxumResponse, ErrorCode,
};