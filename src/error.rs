use axum::{
    Json,
    http::StatusCode,
    response::{IntoResponse, Response},
};
use serde_json::json;

#[derive(Debug)]
pub enum ApiError {
    BadRequest(String),
    NotFound(String),
    Internal,
}

impl ApiError {
    pub fn bad_request(message: impl Into<String>) -> Self {
        Self::BadRequest(message.into())
    }

    pub fn not_found(message: impl Into<String>) -> Self {
        Self::NotFound(message.into())
    }
}

impl IntoResponse for ApiError {
    fn into_response(self) -> Response {
        let (status, status_label, message) = match self {
            Self::BadRequest(message) => (StatusCode::BAD_REQUEST, "fail", message),
            Self::NotFound(message) => (StatusCode::NOT_FOUND, "fail", message),
            Self::Internal => (
                StatusCode::INTERNAL_SERVER_ERROR,
                "error",
                "Internal server error".to_owned(),
            ),
        };

        (status, Json(json!({
            "status": status_label,
            "message": message,
        })))
            .into_response()
    }
}

pub type ApiResult<T> = Result<T, ApiError>;
