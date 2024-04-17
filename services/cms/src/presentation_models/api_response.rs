use axum::{
    extract::State,
    http::StatusCode,
    response::{IntoResponse, Response},
    Json,
};
use serde::Serialize;

#[derive(Serialize)]
pub struct ApiResponseWith<TData> {
    message: String,
    data: TData,
}

#[derive(Serialize)]
pub struct ApiResponseError<TError>
where
    TError: Serialize,
{
    error_code: ErrorCode,
    error: TError,
}

#[derive(Debug, Serialize)]
pub enum ErrorCode {
    UnAuthorized = 401,
    ForBidden = 403,
    NotFound = 404,
    ValidationError = 1000,
    ConnectionError = 1001,
}

impl<TError> ApiResponseError<TError>
where
    TError: Serialize,
{
    fn get_status_code(&self) -> StatusCode {
        match self.error_code {
            ErrorCode::UnAuthorized => StatusCode::UNAUTHORIZED,
            ErrorCode::ForBidden => StatusCode::FORBIDDEN,
            ErrorCode::NotFound => StatusCode::NOT_FOUND,
            ErrorCode::ValidationError => StatusCode::BAD_REQUEST,
            ErrorCode::ConnectionError => StatusCode::INTERNAL_SERVER_ERROR,
        }
    }

    pub fn to_axum_response(self) -> impl IntoResponse {
        (self.get_status_code(), Json(self))
    }

    pub fn new(error_code: ErrorCode, error: TError) -> Self {
        Self { error_code, error }
    }
}

#[cfg(test)]
mod tests {
    use crate::presentation_models::api_response::*;

    #[test]
    fn test_case_one() {
        let response_message = ApiResponseError::<String> {
            error_code: ErrorCode::UnAuthorized,
            error: "User is unauthorized".to_string(),
        };

        let response_status_code = response_message.get_status_code();
        assert_eq!(StatusCode::UNAUTHORIZED, response_status_code);
    }
}
