use crate::domain::error::AppError;
use axum::{http::StatusCode, response::Response};
use hyper::header::CONTENT_TYPE;
use serde::Serialize;

pub trait AxumResponse {
    fn to_axum_response(self) -> Response<String>;
}

#[derive(Debug, Serialize)]
#[serde(rename_all = "camelCase")]
pub struct ApiResponseWith<T: Serialize> {
    message: String,
    data: T,
}

impl<T: Serialize> ApiResponseWith<T> {
    pub fn new(data: T) -> Self {
        Self {
            message: String::new(),
            data,
        }
    }
    pub fn to_status(self, status: StatusCode) -> Response<String> {
        let body = serde_json::to_string(&self).unwrap_or_else(|_| "{}".to_string());
        Response::builder()
            .status(status)
            .header(CONTENT_TYPE, "application/json")
            .body(body)
            .unwrap_or_else(|_| Response::new("{}".to_string()))
    }
}
impl<T: Serialize> AxumResponse for ApiResponseWith<T> {
    fn to_axum_response(self) -> Response<String> {
        self.to_status(StatusCode::OK)
    }
}

#[derive(Debug, Serialize, Clone, Copy, PartialEq)]
#[serde(rename_all = "camelCase")]
pub enum ErrorCode {
    #[serde(rename = "0")]
    UnknownError,
    #[serde(rename = "404")]
    NotFound,
    #[serde(rename = "409")]
    Conflict,
    #[serde(rename = "10000")]
    ValidationError,
    #[serde(rename = "10001")]
    ConnectionError,
    #[serde(rename = "99999")]
    ConcurrencyOptimistic,
}

#[derive(Debug, Serialize)]
#[serde(rename_all = "camelCase")]
pub struct ApiResponseError {
    error_code: ErrorCode,
    errors: Vec<String>,
}

impl ApiResponseError {
    fn new(code: ErrorCode, error: impl Into<String>) -> Self {
        Self {
            error_code: code,
            errors: vec![error.into()],
        }
    }
    pub fn from_error(error: AppError) -> Self {
        match error {
            AppError::Db(error) => Self::new(ErrorCode::ConnectionError, error.to_string()),
            AppError::DbTx(error) => Self::new(ErrorCode::ConnectionError, error.to_string()),
            AppError::Validation(field, message) => {
                Self::new(ErrorCode::ValidationError, format!("{field}: {message}"))
            }
            AppError::Conflict(message) => Self::new(ErrorCode::Conflict, message),
            AppError::ConcurrencyOptimistic(message) => {
                Self::new(ErrorCode::ConcurrencyOptimistic, message)
            }
            AppError::NotFound => Self::new(ErrorCode::NotFound, "Not found"),
            AppError::Unknown => Self::new(ErrorCode::UnknownError, "Unknown error"),
        }
    }
}
impl AxumResponse for ApiResponseError {
    fn to_axum_response(self) -> Response<String> {
        let status = match self.error_code {
            ErrorCode::NotFound => StatusCode::NOT_FOUND,
            ErrorCode::Conflict | ErrorCode::ConcurrencyOptimistic => StatusCode::CONFLICT,
            ErrorCode::ValidationError => StatusCode::BAD_REQUEST,
            ErrorCode::ConnectionError | ErrorCode::UnknownError => {
                StatusCode::INTERNAL_SERVER_ERROR
            }
        };
        let body = serde_json::to_string(&self).unwrap_or_else(|_| "{}".to_string());
        Response::builder()
            .status(status)
            .header(CONTENT_TYPE, "application/json")
            .body(body)
            .unwrap_or_else(|_| Response::new("{}".to_string()))
    }
}
