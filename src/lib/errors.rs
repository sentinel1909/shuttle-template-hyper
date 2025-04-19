// src/lib/errors.rs

// dependencies
use crate::utilities::json_response_msg;
use http_body_util::combinators::BoxBody;
use hyper::body::Bytes;
use hyper::{
    header::{HeaderValue, CONTENT_TYPE},
    Error, Response, StatusCode,
};
use serde::{Deserialize, Serialize};
use thiserror::Error;

// enum type to represent ApiError variants
#[derive(Debug, Error)]
pub enum ApiError {
    #[error("Unauthorized")]
    Unauthorized,

    #[error("Not found")]
    NotFound,

    #[error("Service unavailable")]
    ActorUnavailable,

    #[error("Actor failed to respond")]
    ActorFailed,

    #[error("Invalid input: {0}")]
    InvalidInput(String),

    #[error("Internal server error: {0}")]
    Internal(String),

    #[error("JSON error: {0}")]
    Json(#[from] serde_json::Error),

    #[error("Actor channel failed: {0}")]
    ActorRecv(#[from] tokio::sync::oneshot::error::RecvError),
}

// struct type to represent an error response
#[derive(Deserialize, Serialize)]
pub struct ErrorResponse {
    pub error: String,
}

// methods for the ErrorResponse type
impl ApiError {
    pub fn status_code(&self) -> StatusCode {
        match self {
            ApiError::Unauthorized => StatusCode::UNAUTHORIZED,
            ApiError::NotFound => StatusCode::NOT_FOUND,
            ApiError::ActorUnavailable => StatusCode::BAD_GATEWAY,
            ApiError::ActorFailed => StatusCode::INTERNAL_SERVER_ERROR,
            ApiError::InvalidInput(_) => StatusCode::BAD_REQUEST,
            ApiError::Internal(_) | ApiError::Json(_) | ApiError::ActorRecv(_) => {
                StatusCode::INTERNAL_SERVER_ERROR
            }
        }
    }

    pub fn to_response(&self) -> Response<BoxBody<Bytes, Error>> {
        let error_payload = ErrorResponse {
            error: self.to_string(),
        };

        let body = json_response_msg(error_payload);
        let mut response = Response::new(body);

        *response.status_mut() = self.status_code();
        response
            .headers_mut()
            .insert(CONTENT_TYPE, HeaderValue::from_static("application/json"));

        response
    }
}
