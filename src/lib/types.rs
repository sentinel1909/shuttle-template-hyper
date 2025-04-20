// src/lib/types.rs

// dependencies
use crate::errors::ApiError;
use crate::state::AppState;
use http_body_util::combinators::BoxBody;
use hyper::{
    Error, Request, Response,
    body::{Bytes, Incoming},
};
use serde::{Deserialize, Serialize};
use std::{future::Future, pin::Pin};

// type aliases
pub(crate) type HandlerFn = fn(Request<Incoming>, AppState) -> HandlerResult;
pub(crate) type HandlerResult =
    Pin<Box<dyn Future<Output = Result<RouterResponse, ApiError>> + Send>>;
pub(crate) type SvcReq = Request<Incoming>;

pub(crate) type SvcBody = BoxBody<Bytes, Error>;
pub(crate) type SvcResp = Response<SvcBody>;
pub(crate) type RouterResponse = Response<BoxBody<Bytes, Error>>;

// struct type to represent a generic JSON response message
#[derive(Deserialize, Serialize)]
pub(crate) struct JsonResponse<T: Serialize> {
    pub(crate) msg: &'static str,
    pub(crate) content: T,
}

// struct type to represent an error response
#[derive(Deserialize, Serialize)]
pub(crate) struct ErrorResponse {
    pub(crate) msg: &'static str,
    pub(crate) error: String,
}
