use axum::{http::StatusCode, response::Response};
use hyper::header::CONTENT_TYPE;
use serde::Serialize;

pub trait AxumResponse {
    fn to_axum_response(self) -> Response<String>;
}

#[derive(Serialize)]
pub struct ApiResponseWith<TData>
where
    TData: Serialize,
{
    message: String,
    data: TData,
}

impl<TData> ApiResponseWith<TData>
where
    TData: Serialize,
{
    pub fn new(data: TData) -> Self {
        Self {
            message: "".to_string(),
            data,
        }
    }

    pub fn with_message(self, message: String) -> Self {
        Self { message, ..self }
    }
}

impl<TData> AxumResponse for ApiResponseWith<TData>
where
    TData: Serialize,
{
    fn to_axum_response(self) -> Response<String> {
        let json_body = serde_json::to_string(&self).unwrap();
        Response::builder()
            .status(StatusCode::OK)
            .header(CONTENT_TYPE, "application/json")
            .body(json_body)
            .unwrap()
    }
}

#[derive(Serialize)]
pub struct ApiResponseError {
    error_code: ErrorCode,
    errors: Vec<String>,
}

#[derive(Debug, Serialize, Clone, Copy, PartialEq)]
pub enum ErrorCode {
    #[serde(rename = "0")]
    UnknownError,
    #[serde(rename = "401")]
    UnAuthorized,
    #[serde(rename = "401")]
    ForBidden,
    #[serde(rename = "404")]
    NotFound,
    #[serde(rename = "10000")]
    ValidationError,
    #[serde(rename = "10001")]
    ConnectionError,
}

impl AxumResponse for ApiResponseError {
    fn to_axum_response(self) -> Response<String> {
        let status_code = match self.error_code {
            ErrorCode::UnknownError => StatusCode::INTERNAL_SERVER_ERROR,
            ErrorCode::UnAuthorized => StatusCode::UNAUTHORIZED,
            ErrorCode::ForBidden => StatusCode::FORBIDDEN,
            ErrorCode::NotFound => StatusCode::NOT_FOUND,
            ErrorCode::ValidationError => StatusCode::BAD_REQUEST,
            ErrorCode::ConnectionError => StatusCode::INTERNAL_SERVER_ERROR,
        };

        let json_body = serde_json::to_string(&self).unwrap();
        Response::builder()
            .status(status_code)
            .header(CONTENT_TYPE, "application/json")
            .body(json_body)
            .unwrap()
    }
}

impl ApiResponseError {
    pub fn with_error_code(self, error_code: ErrorCode) -> Self {
        Self {
            error_code,
            errors: self.errors,
        }
    }

    pub fn add_error(self, error: String) -> Self {
        let mut errors = self.errors;
        errors.push(error);
        Self {
            error_code: self.error_code,
            errors,
        }
    }

    pub fn new() -> Self {
        Self {
            error_code: ErrorCode::UnknownError,
            errors: vec![],
        }
    }
}

impl Default for ApiResponseError {
    fn default() -> Self {
        Self::new()
    }
}

#[cfg(test)]
mod tests {
    use crate::presentation_models::api_response::*;

    #[test]
    fn test_case_one() {
        let response_message = ApiResponseError::new()
            .with_error_code(ErrorCode::UnAuthorized)
            .add_error("User is unauthorized".to_string());

        assert_eq!(ErrorCode::UnAuthorized, response_message.error_code);
        assert_eq!(1, response_message.errors.len());
    }
}
