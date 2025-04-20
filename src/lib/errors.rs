// src/lib/errors.rs

// dependencies
use crate::types::{ErrorResponse, SvcResp};
use crate::utilities::set_content_type_json;
use http_body_util::{BodyExt, Full};
use hyper::body::Bytes;
use hyper::{Response, StatusCode};
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

    pub fn to_response(&self) -> SvcResp {
        let error_payload = ErrorResponse {
            msg: "error",
            error: self.to_string(),
        };

        let json = serde_json::to_vec(&error_payload).unwrap_or_else(|e| {
            let fallback = format!(
                r#"{{"msg":"error","error":"Failed to serialize error: {}"}}"#,
                e
            );
            fallback.into_bytes()
        });

        let body = Full::new(Bytes::from(json))
            .map_err(|never| match never {})
            .boxed();

        let mut error_response = Response::new(body);
        *error_response.status_mut() = self.status_code();
        set_content_type_json(&mut error_response);

        error_response
    }
}
